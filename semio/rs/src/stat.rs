use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::guid::Guid;
use crate::hash::HashWriter;

/// Computed/summary stat attached to a design or kit (e.g. piece count).
#[derive(Debug)]
pub struct StatStore {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
    pub description: Option<String>,
    hash_cache: OnceLock<String>,
}

pub type StatStoreRef = std::sync::Arc<std::sync::RwLock<StatStore>>;
pub type StatStoreWeak = std::sync::Weak<std::sync::RwLock<StatStore>>;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct StatIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct StatMetadataDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct StatShallowDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct StatFullDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl StatStore {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            key: key.into(),
            value: value.into(),
            unit: None,
            description: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: StatIdDto) -> Self {
        Self {
            guid: d.guid,
            key: String::new(),
            value: String::new(),
            unit: None,
            description: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: StatMetadataDto) -> Self {
        Self {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
            description: d.description,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: StatShallowDto) -> Self {
        Self::from_metadata_dto(StatMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
            description: d.description,
        })
    }

    pub fn from_full_dto(d: StatFullDto) -> Self {
        Self::from_metadata_dto(StatMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
            description: d.description,
        })
    }

    pub fn to_id_dto(&self) -> StatIdDto {
        StatIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> StatMetadataDto {
        StatMetadataDto {
            guid: self.guid.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            unit: self.unit.clone(),
            description: self.description.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> StatShallowDto {
        let m = self.to_metadata_dto();
        StatShallowDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            unit: m.unit,
            description: m.description,
        }
    }

    pub fn to_full_dto(&self) -> StatFullDto {
        let m = self.to_metadata_dto();
        StatFullDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            unit: m.unit,
            description: m.description,
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
        w.tag("stat")
            .str(self.guid.as_str())
            .str(&self.key)
            .str(&self.value)
            .opt_str(self.unit.as_deref())
            .opt_str(self.description.as_deref());
    }
}
