use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStoreWeak;

pub type FileStoreRef = Arc<RwLock<FileStore>>;
pub type FileStoreWeak = Weak<RwLock<FileStore>>;

/// External resource referenced by a kit (3D model, texture, etc.).
#[derive(Debug)]
pub struct FileStore {
    pub guid: Guid,
    pub url: String,
    pub mime: Option<String>,
    pub size: Option<i64>,
    pub hash: Option<String>,
    pub description: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub parent_kit: Option<KitStoreWeak>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileMetadataDto {
    pub guid: Guid,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileShallowDto {
    pub guid: Guid,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileFullDto {
    pub guid: Guid,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl FileStore {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            url: url.into(),
            mime: None,
            size: None,
            hash: None,
            description: None,
            created: None,
            updated: None,
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
        EntityRef::new(EntityKind::File, self.guid.clone())
    }

    pub fn from_id_dto(d: FileIdDto) -> Self {
        Self {
            guid: d.guid,
            url: String::new(),
            mime: None,
            size: None,
            hash: None,
            description: None,
            created: None,
            updated: None,
            parent_kit: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_metadata_dto(d: FileMetadataDto) -> Self {
        Self {
            guid: d.guid,
            url: d.url,
            mime: d.mime,
            size: d.size,
            hash: d.hash,
            description: d.description,
            created: d.created,
            updated: d.updated,
            parent_kit: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_shallow_dto(d: FileShallowDto) -> Self {
        Self::from_metadata_dto(FileMetadataDto {
            guid: d.guid,
            url: d.url,
            mime: d.mime,
            size: d.size,
            hash: d.hash,
            description: d.description,
            created: d.created,
            updated: d.updated,
        })
    }

    pub fn from_full_dto(d: FileFullDto) -> Self {
        Self::from_metadata_dto(FileMetadataDto {
            guid: d.guid,
            url: d.url,
            mime: d.mime,
            size: d.size,
            hash: d.hash,
            description: d.description,
            created: d.created,
            updated: d.updated,
        })
    }

    pub fn to_id_dto(&self) -> FileIdDto {
        FileIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> FileMetadataDto {
        FileMetadataDto {
            guid: self.guid.clone(),
            url: self.url.clone(),
            mime: self.mime.clone(),
            size: self.size,
            hash: self.hash.clone(),
            description: self.description.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> FileShallowDto {
        let m = self.to_metadata_dto();
        FileShallowDto {
            guid: m.guid,
            url: m.url,
            mime: m.mime,
            size: m.size,
            hash: m.hash,
            description: m.description,
            created: m.created,
            updated: m.updated,
        }
    }

    pub fn to_full_dto(&self) -> FileFullDto {
        let m = self.to_metadata_dto();
        FileFullDto {
            guid: m.guid,
            url: m.url,
            mime: m.mime,
            size: m.size,
            hash: m.hash,
            description: m.description,
            created: m.created,
            updated: m.updated,
        }
    }

    pub fn set_url(&mut self, url: String) {
        if self.url == url {
            return;
        }
        self.url = url;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "url",
        });
        self.invalidate_hash();
    }

    pub fn set_mime(&mut self, v: Option<String>) {
        if self.mime == v {
            return;
        }
        self.mime = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "mime",
        });
        self.invalidate_hash();
    }

    pub fn set_size(&mut self, v: Option<i64>) {
        if self.size == v {
            return;
        }
        self.size = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "size",
        });
        self.invalidate_hash();
    }

    pub fn set_hash(&mut self, v: Option<String>) {
        if self.hash == v {
            return;
        }
        self.hash = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "hash",
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

    pub fn set_created(&mut self, v: Option<String>) {
        if self.created == v {
            return;
        }
        self.created = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "created",
        });
        self.invalidate_hash();
    }

    pub fn set_updated(&mut self, v: Option<String>) {
        if self.updated == v {
            return;
        }
        self.updated = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "updated",
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
        w.tag("file")
            .str(self.guid.as_str())
            .str(&self.url)
            .opt_str(self.mime.as_deref())
            .opt_str(self.hash.as_deref());
        if let Some(s) = self.size {
            w.f64(s as f64);
        }
    }
}
