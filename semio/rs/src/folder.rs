use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};

pub type FolderStoreRef = Arc<RwLock<FolderStore>>;
pub type FolderStoreWeak = Weak<RwLock<FolderStore>>;

/// Logical folder grouping files inside a kit.
#[derive(Debug)]
pub struct FolderStore {
    pub guid: Guid,
    pub path: String,
    pub description: Option<String>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FolderIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FolderMetadataDto {
    pub guid: Guid,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FolderShallowDto {
    pub guid: Guid,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FolderFullDto {
    pub guid: Guid,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl FolderStore {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            path: path.into(),
            description: None,
            hash_cache: Cache::default(),
        }
    }

    pub fn from_id_dto(d: FolderIdDto) -> Self {
        Self {
            guid: d.guid,
            path: String::new(),
            description: None,
            hash_cache: Cache::default(),
        }
    }

    pub fn from_metadata_dto(d: FolderMetadataDto) -> Self {
        Self {
            guid: d.guid,
            path: d.path,
            description: d.description,
            hash_cache: Cache::default(),
        }
    }

    pub fn from_shallow_dto(d: FolderShallowDto) -> Self {
        Self::from_metadata_dto(FolderMetadataDto {
            guid: d.guid,
            path: d.path,
            description: d.description,
        })
    }

    pub fn from_full_dto(d: FolderFullDto) -> Self {
        Self::from_metadata_dto(FolderMetadataDto {
            guid: d.guid,
            path: d.path,
            description: d.description,
        })
    }

    pub fn to_id_dto(&self) -> FolderIdDto {
        FolderIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> FolderMetadataDto {
        FolderMetadataDto {
            guid: self.guid.clone(),
            path: self.path.clone(),
            description: self.description.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> FolderShallowDto {
        let m = self.to_metadata_dto();
        FolderShallowDto {
            guid: m.guid,
            path: m.path,
            description: m.description,
        }
    }

    pub fn to_full_dto(&self) -> FolderFullDto {
        let m = self.to_metadata_dto();
        FolderFullDto {
            guid: m.guid,
            path: m.path,
            description: m.description,
        }
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
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
        w.tag("folder").str(self.guid.as_str()).str(&self.path).opt_str(self.description.as_deref());
    }
}
