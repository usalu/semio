use serde::{Deserialize, Serialize};

use crate::hash::HashWriter;

/// Conceptual / semantic label grouping types and designs.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Concept {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

impl Concept {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), description: None, order: None }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("concept").str(&self.name).opt_str(self.description.as_deref());
        if let Some(o) = self.order {
            w.f64(o as f64);
        }
    }
}
