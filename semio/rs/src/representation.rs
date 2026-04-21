use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::file::{FileIdDto, FileStoreWeak};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
use crate::tag::{TagFullDto, TagShallowDto, TagStore};

pub type RepresentationStoreRef = Arc<RwLock<RepresentationStore>>;
pub type RepresentationStoreWeak = Weak<RwLock<RepresentationStore>>;

/// Rendering / geometric representation of a [`crate::typ::TypeStore`].
#[derive(Debug)]
pub struct RepresentationStore {
    pub guid: Guid,
    pub url: String,
    pub description: Option<String>,
    pub tags: Vec<TagStore>,
    pub file: Option<FileStoreWeak>,
    pub qualities: Vec<QualityStoreRef>,
    pub attributes: Vec<AttributeStore>,
    pub parent_type: Weak<RwLock<crate::typ::TypeStore>>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RepresentationIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RepresentationMetadataDto {
    pub guid: Guid,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<FileIdDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RepresentationShallowDto {
    pub guid: Guid,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<FileIdDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RepresentationFullDto {
    pub guid: Guid,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<FileIdDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeFullDto>,
}

impl RepresentationStore {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            url: url.into(),
            description: None,
            tags: Vec::new(),
            file: None,
            qualities: Vec::new(),
            attributes: Vec::new(),
            parent_type: Weak::new(),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Representation, self.guid.clone())
    }

    pub fn from_id_dto(d: RepresentationIdDto) -> Self {
        Self {
            guid: d.guid,
            url: String::new(),
            description: None,
            tags: Vec::new(),
            file: None,
            qualities: Vec::new(),
            attributes: Vec::new(),
            parent_type: Weak::new(),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_metadata_dto(d: RepresentationMetadataDto) -> Self {
        Self {
            guid: d.guid,
            url: d.url,
            description: d.description,
            tags: Vec::new(),
            file: None,
            qualities: Vec::new(),
            attributes: Vec::new(),
            parent_type: Weak::new(),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_shallow_dto(d: RepresentationShallowDto) -> Self {
        let mut s = Self::from_metadata_dto(RepresentationMetadataDto {
            guid: d.guid,
            url: d.url,
            description: d.description,
            file: d.file,
        });
        s.tags = d.tags.into_iter().map(TagStore::from_shallow_dto).collect();
        s.qualities = d
            .qualities
            .into_iter()
            .map(|q| Arc::new(RwLock::new(QualityStore::from_shallow_dto(q))))
            .collect();
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_shallow_dto).collect();
        s
    }

    pub fn from_full_dto(d: RepresentationFullDto) -> Self {
        let mut s = Self::from_metadata_dto(RepresentationMetadataDto {
            guid: d.guid,
            url: d.url,
            description: d.description,
            file: d.file,
        });
        s.tags = d.tags.into_iter().map(TagStore::from_full_dto).collect();
        s.qualities = d
            .qualities
            .into_iter()
            .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
            .collect();
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_full_dto).collect();
        s
    }

    pub fn to_id_dto(&self) -> RepresentationIdDto {
        RepresentationIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> RepresentationMetadataDto {
        let file = self.file.as_ref().and_then(|f| f.upgrade()).and_then(|f| f.read().ok().map(|f| f.to_id_dto()));
        RepresentationMetadataDto {
            guid: self.guid.clone(),
            url: self.url.clone(),
            description: self.description.clone(),
            file,
        }
    }

    pub fn to_shallow_dto(&self) -> RepresentationShallowDto {
        let m = self.to_metadata_dto();
        RepresentationShallowDto {
            guid: m.guid,
            url: m.url,
            description: m.description,
            file: m.file,
            tags: self.tags.iter().map(TagStore::to_shallow_dto).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                .collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_shallow_dto).collect(),
        }
    }

    pub fn to_full_dto(&self) -> RepresentationFullDto {
        let m = self.to_metadata_dto();
        RepresentationFullDto {
            guid: m.guid,
            url: m.url,
            description: m.description,
            file: m.file,
            tags: self.tags.iter().map(TagStore::to_full_dto).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                .collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_full_dto).collect(),
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
        if let Some(t) = self.parent_type.upgrade() {
            if let Ok(tr) = t.read() {
                tr.invalidate_hash();
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
        w.tag("representation")
            .str(self.guid.as_str())
            .str(&self.url)
            .opt_str(self.description.as_deref());
        for t in &self.tags {
            t.hash_into(w);
        }
        if let Some(file) = self.file.as_ref().and_then(|f| f.upgrade()) {
            if let Ok(file) = file.read() {
                w.str(file.guid.as_str());
            }
        }
        for q in &self.qualities {
            if let Ok(q) = q.read() {
                q.hash_into(w);
            }
        }
        for a in &self.attributes {
            a.hash_into(w);
        }
    }
}
