use serde::{Deserialize, Serialize};

use crate::hash::HashWriter;

/// A name/value pair attached to pretty much any domain entity.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Attribute {
    pub key: String,
    #[serde(default)]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

impl Attribute {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self { key: key.into(), value: value.into(), definition: None }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("attr").str(&self.key).str(&self.value).opt_str(self.definition.as_deref());
    }
}
