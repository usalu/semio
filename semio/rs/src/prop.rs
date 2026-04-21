use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Weak};

use crate::design::DesignStoreWeak;
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStoreWeak;
use crate::piece::PieceStoreWeak;
use crate::typ::TypeStoreWeak;

pub type PropStoreRef = std::sync::Arc<RwLock<PropStore>>;
pub type PropStoreWeak = Weak<RwLock<PropStore>>;

/// A typed property value (distinct from free-form Attributes: props carry
/// meaning in the domain, attributes are auxiliary metadata).
#[derive(Debug)]
pub struct PropStore {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
    pub parent_kit: Option<KitStoreWeak>,
    pub parent_design: Option<DesignStoreWeak>,
    pub parent_type: Option<TypeStoreWeak>,
    pub parent_piece: Option<PieceStoreWeak>,
    hash_cache: Cache<String>,
}

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
    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
            key: String::new(),
            value: String::new(),
            unit: None,
            parent_kit: None,
            parent_design: None,
            parent_type: None,
            parent_piece: None,
            hash_cache: Cache::default(),
        }
    }

    pub(crate) fn apply_full_dto_fields(&mut self, d: PropFullDto) {
        self.guid = d.guid;
        self.key = d.key;
        self.value = d.value;
        self.unit = d.unit;
        self.hash_cache.invalidate();
    }

    pub(crate) fn from_full_dto(d: PropFullDto) -> Self {
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
        if let Some(w) = &self.parent_type {
            if let Some(t) = w.upgrade() {
                if let Ok(t) = t.read() {
                    t.invalidate_hash();
                }
            }
        }
        if let Some(w) = &self.parent_piece {
            if let Some(p) = w.upgrade() {
                if let Ok(p) = p.read() {
                    p.invalidate_hash();
                }
            }
        }
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
        w.tag("prop")
            .str(self.guid.as_str())
            .str(&self.key)
            .str(&self.value)
            .opt_str(self.unit.as_deref());
    }
}
