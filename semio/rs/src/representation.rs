use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
use crate::file::{FileIdDto, FileStoreWeak};
use crate::guid::Guid;
use crate::hash::HashWriter;
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
    hash_cache: OnceLock<String>,
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
    #[serde(flatten)]
    pub meta: RepresentationMetadataDto,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RepresentationFullDto {
    #[serde(flatten)]
    pub meta: RepresentationMetadataDto,
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
            hash_cache: OnceLock::new(),
        }
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
            hash_cache: OnceLock::new(),
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
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: RepresentationShallowDto) -> Self {
        let mut s = Self::from_metadata_dto(d.meta);
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
        let mut s = Self::from_metadata_dto(d.meta);
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
        RepresentationShallowDto {
            meta: self.to_metadata_dto(),
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
        RepresentationFullDto {
            meta: self.to_metadata_dto(),
            tags: self.tags.iter().map(TagStore::to_full_dto).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                .collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_full_dto).collect(),
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
