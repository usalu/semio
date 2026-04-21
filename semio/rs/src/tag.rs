use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::guid::Guid;
use crate::hash::HashWriter;

/// Freely choosable label used for filtering/grouping in the UI.
#[derive(Debug)]
pub struct TagStore {
    pub guid: Guid,
    pub name: String,
    pub order: Option<i64>,
    hash_cache: OnceLock<String>,
}

pub type TagStoreRef = std::sync::Arc<std::sync::RwLock<TagStore>>;
pub type TagStoreWeak = std::sync::Weak<std::sync::RwLock<TagStore>>;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TagIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TagMetadataDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TagShallowDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TagFullDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

impl TagStore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            name: name.into(),
            order: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: TagIdDto) -> Self {
        Self {
            guid: d.guid,
            name: String::new(),
            order: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: TagMetadataDto) -> Self {
        Self {
            guid: d.guid,
            name: d.name,
            order: d.order,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: TagShallowDto) -> Self {
        Self::from_metadata_dto(TagMetadataDto {
            guid: d.guid,
            name: d.name,
            order: d.order,
        })
    }

    pub fn from_full_dto(d: TagFullDto) -> Self {
        Self::from_metadata_dto(TagMetadataDto {
            guid: d.guid,
            name: d.name,
            order: d.order,
        })
    }

    pub fn to_id_dto(&self) -> TagIdDto {
        TagIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> TagMetadataDto {
        TagMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            order: self.order,
        }
    }

    pub fn to_shallow_dto(&self) -> TagShallowDto {
        let m = self.to_metadata_dto();
        TagShallowDto {
            guid: m.guid,
            name: m.name,
            order: m.order,
        }
    }

    pub fn to_full_dto(&self) -> TagFullDto {
        let m = self.to_metadata_dto();
        TagFullDto {
            guid: m.guid,
            name: m.name,
            order: m.order,
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
        w.tag("tag").str(self.guid.as_str()).str(&self.name);
        if let Some(o) = self.order {
            w.f64(o as f64);
        }
    }
}
