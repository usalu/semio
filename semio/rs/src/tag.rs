use serde::{Deserialize, Serialize};

use crate::hash::HashWriter;

/// Freely choosable label used for filtering/grouping in the UI.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Tag {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

impl Tag {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), order: None }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("tag").str(&self.name);
        if let Some(o) = self.order {
            w.f64(o as f64);
        }
    }
}
