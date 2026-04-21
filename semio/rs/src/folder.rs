use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStoreWeak;

pub type FolderStoreRef = Arc<RwLock<FolderStore>>;
pub type FolderStoreWeak = Weak<RwLock<FolderStore>>;

/// Logical folder grouping files inside a kit.
#[derive(Debug)]
pub struct FolderStore {
    pub guid: Guid,
    pub path: String,
    pub description: Option<String>,
    pub parent_kit: Option<KitStoreWeak>,
    pub(crate) event_bus: Weak<EventBus>,
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
            parent_kit: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Folder, self.guid.clone())
    }

    pub fn from_id_dto(d: FolderIdDto) -> Self {
        Self {
            guid: d.guid,
            path: String::new(),
            description: None,
            parent_kit: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_metadata_dto(d: FolderMetadataDto) -> Self {
        Self {
            guid: d.guid,
            path: d.path,
            description: d.description,
            parent_kit: None,
            event_bus: Weak::new(),
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

    pub fn set_path(&mut self, path: String) {
        if self.path == path {
            return;
        }
        self.path = path;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "path",
        });
        self.invalidate_hash();
    }

    pub fn set_description(&mut self, v: Option<String>) {
        if self.description == v {
            return;
        }
        self.description = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "description",
        });
        self.invalidate_hash();
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        if let Some(w) = &self.parent_kit {
            if let Some(k) = w.upgrade() {
                if let Ok(kr) = k.read() {
                    kr.invalidate_hash();
                    kr.invalidate_validation();
                }
            }
        }
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
