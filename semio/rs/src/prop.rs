use serde::{Deserialize, Serialize};

use crate::hash::HashWriter;

/// A typed property value (distinct from free-form Attributes: props carry
/// meaning in the domain, attributes are auxiliary metadata).
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Prop {
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl Prop {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self { key: key.into(), value: value.into(), unit: None }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("prop").str(&self.key).str(&self.value).opt_str(self.unit.as_deref());
    }
}
