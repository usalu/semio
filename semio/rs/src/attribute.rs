use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::guid::Guid;
use crate::hash::HashWriter;

/// A name/value pair attached to pretty much any domain entity.
#[derive(Debug)]
pub struct AttributeStore {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    pub definition: Option<String>,
    hash_cache: OnceLock<String>,
}

pub type AttributeStoreRef = std::sync::Arc<std::sync::RwLock<AttributeStore>>;
pub type AttributeStoreWeak = std::sync::Weak<std::sync::RwLock<AttributeStore>>;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AttributeIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AttributeMetadataDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AttributeShallowDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AttributeFullDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

impl AttributeStore {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            key: key.into(),
            value: value.into(),
            definition: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: AttributeIdDto) -> Self {
        Self {
            guid: d.guid,
            key: String::new(),
            value: String::new(),
            definition: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: AttributeMetadataDto) -> Self {
        Self {
            guid: d.guid,
            key: d.key,
            value: d.value,
            definition: d.definition,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: AttributeShallowDto) -> Self {
        Self::from_metadata_dto(AttributeMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            definition: d.definition,
        })
    }

    pub fn from_full_dto(d: AttributeFullDto) -> Self {
        Self::from_metadata_dto(AttributeMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            definition: d.definition,
        })
    }

    pub fn to_id_dto(&self) -> AttributeIdDto {
        AttributeIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> AttributeMetadataDto {
        AttributeMetadataDto {
            guid: self.guid.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            definition: self.definition.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> AttributeShallowDto {
        let m = self.to_metadata_dto();
        AttributeShallowDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            definition: m.definition,
        }
    }

    pub fn to_full_dto(&self) -> AttributeFullDto {
        let m = self.to_metadata_dto();
        AttributeFullDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            definition: m.definition,
        }
    }

    pub fn invalidate_hash(&mut self) {
        self.hash_cache = OnceLock::new();
    }

    pub fn hash(&self) -> String {
        self.hash_cache
            .get_or_init(|| {
                let mut w = HashWriter::new();
                self.hash_into(&mut w);
                w.finalize()
            })
            .clone()
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("attr")
            .str(self.guid.as_str())
            .str(&self.key)
            .str(&self.value)
            .opt_str(self.definition.as_deref());
    }
}
