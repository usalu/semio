use serde::{Deserialize, Serialize};

use crate::hash::HashWriter;

/// Computed/summary stat attached to a design or kit (e.g. piece count).
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Stat {
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Stat {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self { key: key.into(), value: value.into(), unit: None, description: None }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("stat")
            .str(&self.key)
            .str(&self.value)
            .opt_str(self.unit.as_deref())
            .opt_str(self.description.as_deref());
    }
}
