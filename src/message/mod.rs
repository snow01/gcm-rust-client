use serde::Serializer;
use serde_json::map::Map as JsonMap;
use serde_json::value::Value as JsonValue;

pub use message::response::*;
/*use notification::Notification;*/

pub mod async_sender;
pub mod gcm_util;
pub mod response;
pub mod sender;

#[derive(PartialEq, Debug, Serialize)]
pub enum Priority {
    Normal,
    High,
}

/// Represents a GCM message. Construct the GCM message
/// using various utility methods and finally send it.
/// # Examples:
/// ```rust
/// use gcm::Message;
///
/// let message = Message::new(vec!["<registration id>".to_string()]).dry_run(true);
/// ```
#[derive(Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    registration_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "priority_lowercase"
    )]
    priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay_while_idle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_to_live: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<JsonMap<String, JsonValue>>,
}

fn priority_lowercase<S>(
    priority_field: &Option<Priority>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // unwrapping cause we skip serializing if none
    let normal_priority = Priority::Normal;
    let priority = priority_field.as_ref().unwrap_or(&normal_priority);
    match *priority {
        Priority::Normal => serializer.serialize_str("normal"),
        Priority::High => serializer.serialize_str("high"),
    }
}

impl Message {
    /// Get a new instance of Message. You need to supply either
    /// a registration id, or a topic (/topic/...).
    pub fn new(registration_ids: Vec<String>) -> Message {
        Message {
            registration_ids: Some(registration_ids),
            collapse_key: None,
            priority: None,
            content_available: None,
            delay_while_idle: None,
            time_to_live: None,
            restricted_package_name: None,
            dry_run: None,
            data: None,
        }
    }

    /// Set various registration ids to which the message ought to be sent.
    /*    pub fn registration_ids(mut self, ids: Vec<&'a str>) -> Message<'a> {
        self.registration_ids = Some(ids.iter().map(|s| s.to_string()).collect());
        self
    }*/

    /// Set this parameter to identify groups of messages that can be collapsed.
    pub fn collapse_key(mut self, collapse_key: String) -> Self {
        self.collapse_key = Some(collapse_key);
        self
    }

    /// Set the priority of the message. You can set Normal or High priorities.
    /// # Examples:
    /// ```rust
    /// use gcm::{Message, Priority};
    ///
    /// let message = Message::new(vec!["<registration id>".to_string()])
    ///     .priority(Priority::High);
    /// ```
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// To set the `content-available` field on iOS
    pub fn content_available(mut self, content_available: bool) -> Self {
        self.content_available = Some(content_available);
        self
    }

    /// When set to `true`, sends the message only when the device is active.
    pub fn delay_while_idle(mut self, delay_while_idle: bool) -> Self {
        self.delay_while_idle = Some(delay_while_idle);
        self
    }

    /// How long (in seconds) to keep the message on GCM servers in case the device
    /// is offline. The maximum and default is 4 weeks.
    pub fn time_to_live(mut self, time_to_live: i32) -> Self {
        self.time_to_live = Some(time_to_live);
        self
    }

    /// Package name of the application where the registration tokens must match.
    pub fn restricted_package_name(mut self, restricted_package_name: String) -> Self {
        self.restricted_package_name = Some(restricted_package_name);
        self
    }

    /// When set to `true`, allows you to test GCM without actually sending the message.
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = Some(dry_run);
        self
    }

    /// Use this to add custom key-value pairs to the message. This data
    /// must be handled appropriately on the client end.
    /// # Examples:
    /// ```rust
    /// use gcm::Message;
    /// use std::collections::HashMap;
    ///
    /// let mut map = JsonMap::new();
    /// map.insert("message".to_string(), JsonValue::String("Howdy!".to_string()));
    ///
    /// let message = Message::new(vec!["<registration id>".to_string()]).data(&map);
    /// ```
    pub fn data(mut self, data: &JsonMap<String, JsonValue>) -> Self {
        let mut datamap: JsonMap<String, JsonValue> = JsonMap::new();
        for (key, val) in data.iter() {
            datamap.insert(key.clone(), val.clone());
        }

        self.data = Some(datamap);
        self
    }

    /*    /// Use this to set a `Notification` for the message.
    /// # Examples:
    /// ```rust
    /// use gcm::{Message, NotificationBuilder};
    ///
    /// let notification = NotificationBuilder::new("Hey!")
    ///     .body("Do you want to catch up later?")
    ///     .finalize();
    ///
    /// let message = Message::new(vec!["<registration id>"])
    ///     .notification(notification);
    /// ```
    pub fn notification(mut self, notification: Notification<'a>) -> Self {
        self.notification = Some(notification);
        self
    }*/

    pub fn build(self) -> Self {
        self
    }
}
