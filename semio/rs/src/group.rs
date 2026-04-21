use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::piece::{PieceIdDto, PieceStoreWeak};

pub type GroupStoreRef = Arc<RwLock<GroupStore>>;
pub type GroupStoreWeak = Weak<RwLock<GroupStore>>;

/// User-defined group of pieces inside a [`crate::design::DesignStore`].
#[derive(Debug)]
pub struct GroupStore {
    pub guid: Guid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub pieces: Vec<PieceStoreWeak>,
    pub parent_design: Weak<RwLock<crate::design::DesignStore>>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct GroupIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct GroupMetadataDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pieces: Vec<PieceIdDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct GroupShallowDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pieces: Vec<PieceIdDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct GroupFullDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pieces: Vec<PieceIdDto>,
}

impl GroupStore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            name: name.into(),
            description: None,
            color: None,
            icon: None,
            pieces: Vec::new(),
            parent_design: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_id_dto(d: GroupIdDto) -> Self {
        Self {
            guid: d.guid,
            name: String::new(),
            description: None,
            color: None,
            icon: None,
            pieces: Vec::new(),
            parent_design: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_metadata_dto(d: GroupMetadataDto) -> Self {
        Self {
            guid: d.guid,
            name: d.name,
            description: d.description,
            color: d.color,
            icon: d.icon,
            pieces: Vec::new(),
            parent_design: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_shallow_dto(d: GroupShallowDto) -> Self {
        Self::from_metadata_dto(GroupMetadataDto {
            guid: d.guid,
            name: d.name,
            description: d.description,
            color: d.color,
            icon: d.icon,
            pieces: d.pieces,
        })
    }

    pub fn from_full_dto(d: GroupFullDto) -> Self {
        Self::from_metadata_dto(GroupMetadataDto {
            guid: d.guid,
            name: d.name,
            description: d.description,
            color: d.color,
            icon: d.icon,
            pieces: d.pieces,
        })
    }

    pub fn to_id_dto(&self) -> GroupIdDto {
        GroupIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> GroupMetadataDto {
        GroupMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            color: self.color.clone(),
            icon: self.icon.clone(),
            pieces: self
                .pieces
                .iter()
                .filter_map(|p| p.upgrade())
                .filter_map(|p| p.read().ok().map(|p| p.to_id_dto()))
                .collect(),
        }
    }

    pub fn to_shallow_dto(&self) -> GroupShallowDto {
        let m = self.to_metadata_dto();
        GroupShallowDto {
            guid: m.guid,
            name: m.name,
            description: m.description,
            color: m.color,
            icon: m.icon,
            pieces: m.pieces,
        }
    }

    pub fn to_full_dto(&self) -> GroupFullDto {
        let m = self.to_metadata_dto();
        GroupFullDto {
            guid: m.guid,
            name: m.name,
            description: m.description,
            color: m.color,
            icon: m.icon,
            pieces: m.pieces,
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
        w.tag("group")
            .str(self.guid.as_str())
            .str(&self.name)
            .opt_str(self.description.as_deref())
            .opt_str(self.color.as_deref())
            .opt_str(self.icon.as_deref());
        for p in &self.pieces {
            if let Some(p) = p.upgrade() {
                if let Ok(p) = p.read() {
                    w.str(p.guid.as_str());
                }
            }
        }
    }
}
