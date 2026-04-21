use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::Attribute;
use crate::file::FileWeak;
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::quality::{Quality, QualityDto, QualityRef};
use crate::tag::Tag;

pub type RepresentationRef = Arc<RwLock<Representation>>;
pub type RepresentationWeak = Weak<RwLock<Representation>>;

/// Rendering / geometric representation of a [`crate::typ::Type`].
#[derive(Debug)]
pub struct Representation {
    pub guid: Guid,
    pub url: String,
    pub description: Option<String>,
    pub tags: Vec<Tag>,
    pub file: Option<FileWeak>,
    pub qualities: Vec<QualityRef>,
    pub attributes: Vec<Attribute>,
    pub parent_type: Weak<RwLock<crate::typ::Type>>,
    hash_cache: OnceLock<String>,
}

impl Representation {
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RepresentationDto {
    #[serde(default)]
    pub guid: Option<Guid>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileGuid")]
    pub file_guid: Option<Guid>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute>,
}

impl From<&Representation> for RepresentationDto {
    fn from(r: &Representation) -> Self {
        RepresentationDto {
            guid: Some(r.guid.clone()),
            url: r.url.clone(),
            description: r.description.clone(),
            tags: r.tags.clone(),
            file_guid: r
                .file
                .as_ref()
                .and_then(|f| f.upgrade())
                .and_then(|f| f.read().ok().map(|f| f.guid.clone())),
            qualities: r
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| QualityDto::from(&*q)))
                .collect(),
            attributes: r.attributes.clone(),
        }
    }
}

impl Representation {
    pub fn from_dto(d: RepresentationDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            url: d.url,
            description: d.description,
            tags: d.tags,
            file: None,
            qualities: d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(Quality::from(q))))
                .collect(),
            attributes: d.attributes,
            parent_type: Weak::new(),
            hash_cache: OnceLock::new(),
        }
    }
}
