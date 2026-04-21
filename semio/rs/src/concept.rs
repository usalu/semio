use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::guid::Guid;
use crate::hash::HashWriter;

/// Conceptual / semantic label grouping types and designs.
#[derive(Debug)]
pub struct ConceptStore {
    pub guid: Guid,
    pub name: String,
    pub description: Option<String>,
    pub order: Option<i64>,
    hash_cache: OnceLock<String>,
}

pub type ConceptStoreRef = std::sync::Arc<std::sync::RwLock<ConceptStore>>;
pub type ConceptStoreWeak = std::sync::Weak<std::sync::RwLock<ConceptStore>>;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConceptIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConceptMetadataDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConceptShallowDto {
    #[serde(flatten)]
    pub meta: ConceptMetadataDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConceptFullDto {
    #[serde(flatten)]
    pub meta: ConceptMetadataDto,
}

impl ConceptStore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            name: name.into(),
            description: None,
            order: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: ConceptIdDto) -> Self {
        Self {
            guid: d.guid,
            name: String::new(),
            description: None,
            order: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: ConceptMetadataDto) -> Self {
        Self {
            guid: d.guid,
            name: d.name,
            description: d.description,
            order: d.order,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: ConceptShallowDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn from_full_dto(d: ConceptFullDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn to_id_dto(&self) -> ConceptIdDto {
        ConceptIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> ConceptMetadataDto {
        ConceptMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            order: self.order,
        }
    }

    pub fn to_shallow_dto(&self) -> ConceptShallowDto {
        ConceptShallowDto { meta: self.to_metadata_dto() }
    }

    pub fn to_full_dto(&self) -> ConceptFullDto {
        ConceptFullDto { meta: self.to_metadata_dto() }
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
        w.tag("concept").str(self.guid.as_str()).str(&self.name).opt_str(self.description.as_deref());
        if let Some(o) = self.order {
            w.f64(o as f64);
        }
    }
}
