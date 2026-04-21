use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::guid::Guid;
use crate::hash::HashWriter;

/// A typed property value (distinct from free-form Attributes: props carry
/// meaning in the domain, attributes are auxiliary metadata).
#[derive(Debug)]
pub struct PropStore {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
    hash_cache: OnceLock<String>,
}

pub type PropStoreRef = std::sync::Arc<std::sync::RwLock<PropStore>>;
pub type PropStoreWeak = std::sync::Weak<std::sync::RwLock<PropStore>>;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PropIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PropMetadataDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PropShallowDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PropFullDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl PropStore {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            key: key.into(),
            value: value.into(),
            unit: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: PropIdDto) -> Self {
        Self {
            guid: d.guid,
            key: String::new(),
            value: String::new(),
            unit: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: PropMetadataDto) -> Self {
        Self {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: PropShallowDto) -> Self {
        Self::from_metadata_dto(PropMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
        })
    }

    pub fn from_full_dto(d: PropFullDto) -> Self {
        Self::from_metadata_dto(PropMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
        })
    }

    pub fn to_id_dto(&self) -> PropIdDto {
        PropIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> PropMetadataDto {
        PropMetadataDto {
            guid: self.guid.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            unit: self.unit.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> PropShallowDto {
        let m = self.to_metadata_dto();
        PropShallowDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            unit: m.unit,
        }
    }

    pub fn to_full_dto(&self) -> PropFullDto {
        let m = self.to_metadata_dto();
        PropFullDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            unit: m.unit,
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
        w.tag("prop")
            .str(self.guid.as_str())
            .str(&self.key)
            .str(&self.value)
            .opt_str(self.unit.as_deref());
    }
}
