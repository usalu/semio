use serde::{Deserialize, Serialize};

use crate::hash::HashWriter;

/// A human author attached to a design, type, or kit.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

impl Author {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self { name: name.into(), email: email.into(), role: None, rank: None }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("author")
            .str(&self.name)
            .str(&self.email)
            .opt_str(self.role.as_deref());
        if let Some(r) = self.rank {
            w.f64(r as f64);
        }
    }
}
