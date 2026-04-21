use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Weak};

use crate::design::DesignStoreWeak;
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStoreWeak;

pub type StatStoreRef = std::sync::Arc<RwLock<StatStore>>;
pub type StatStoreWeak = Weak<RwLock<StatStore>>;

/// Computed/summary stat attached to a design or kit (e.g. piece count).
#[derive(Debug)]
pub struct StatStore {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub parent_kit: Option<KitStoreWeak>,
    pub parent_design: Option<DesignStoreWeak>,
    hash_cache: Cache<String>,
}

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
    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
            key: String::new(),
            value: String::new(),
            unit: None,
            description: None,
            parent_kit: None,
            parent_design: None,
            hash_cache: Cache::default(),
        }
    }

    pub(crate) fn apply_full_dto_fields(&mut self, d: StatFullDto) {
        self.guid = d.guid;
        self.key = d.key;
        self.value = d.value;
        self.unit = d.unit;
        self.description = d.description;
        self.hash_cache.invalidate();
    }

    pub(crate) fn from_full_dto(d: StatFullDto) -> Self {
        let mut s = Self::empty_shell(d.guid.clone());
        s.apply_full_dto_fields(d);
        s
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
        self.bubble();
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.bubble();
    }

    pub fn set_unit(&mut self, unit: Option<String>) {
        self.unit = unit;
        self.bubble();
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.bubble();
    }

    fn bubble(&mut self) {
        self.hash_cache.invalidate();
        if let Some(w) = &self.parent_kit {
            if let Some(k) = w.upgrade() {
                if let Ok(k) = k.read() {
                    k.invalidate_hash();
                    k.invalidate_validation();
                }
            }
        }
        if let Some(w) = &self.parent_design {
            if let Some(d) = w.upgrade() {
                if let Ok(d) = d.read() {
                    d.invalidate_hash();
                    d.invalidate_flatten();
                    d.invalidate_validation();
                }
            }
        }
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

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
    }

    pub fn hash(&self) -> String {
        self.hash_cache.get_or_init(|| {
            let mut w = HashWriter::new();
            self.hash_into(&mut w);
            w.finalize()
        })
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
