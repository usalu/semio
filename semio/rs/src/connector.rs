use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::port::{PortIdDto, PortStoreWeak};
use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};

pub type ConnectorStoreRef = Arc<RwLock<ConnectorStore>>;
pub type ConnectorStoreWeak = Weak<RwLock<ConnectorStore>>;

/// A named socket on a [`crate::typ::TypeStore`] that references a concrete port.
#[derive(Debug)]
pub struct ConnectorStore {
    pub guid: Guid,
    pub code: String,
    pub description: Option<String>,
    pub port: Option<PortStoreWeak>,
    pub qualities: Vec<QualityStoreRef>,
    pub attributes: Vec<AttributeStore>,
    /// Back-reference to the owning type.
    pub parent_type: Weak<RwLock<crate::typ::TypeStore>>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorMetadataDto {
    pub guid: Guid,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortIdDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorShallowDto {
    pub guid: Guid,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortIdDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorFullDto {
    pub guid: Guid,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortIdDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeFullDto>,
}

impl ConnectorStore {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            code: code.into(),
            description: None,
            port: None,
            qualities: Vec::new(),
            attributes: Vec::new(),
            parent_type: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_id_dto(d: ConnectorIdDto) -> Self {
        Self {
            guid: d.guid,
            code: String::new(),
            description: None,
            port: None,
            qualities: Vec::new(),
            attributes: Vec::new(),
            parent_type: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_metadata_dto(d: ConnectorMetadataDto) -> Self {
        Self {
            guid: d.guid,
            code: d.code,
            description: d.description,
            port: None,
            qualities: Vec::new(),
            attributes: Vec::new(),
            parent_type: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_shallow_dto(d: ConnectorShallowDto) -> Self {
        let mut s = Self::from_metadata_dto(ConnectorMetadataDto {
            guid: d.guid,
            code: d.code,
            description: d.description,
            port: d.port,
        });
        s.qualities = d
            .qualities
            .into_iter()
            .map(|q| Arc::new(RwLock::new(QualityStore::from_shallow_dto(q))))
            .collect();
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_shallow_dto).collect();
        s
    }

    pub fn from_full_dto(d: ConnectorFullDto) -> Self {
        let mut s = Self::from_metadata_dto(ConnectorMetadataDto {
            guid: d.guid,
            code: d.code,
            description: d.description,
            port: d.port,
        });
        s.qualities = d
            .qualities
            .into_iter()
            .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
            .collect();
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_full_dto).collect();
        s
    }

    pub fn to_id_dto(&self) -> ConnectorIdDto {
        ConnectorIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> ConnectorMetadataDto {
        let port = self.port.as_ref().and_then(|p| p.upgrade()).and_then(|p| p.read().ok().map(|p| p.to_id_dto()));
        ConnectorMetadataDto {
            guid: self.guid.clone(),
            code: self.code.clone(),
            description: self.description.clone(),
            port,
        }
    }

    pub fn to_shallow_dto(&self) -> ConnectorShallowDto {
        let m = self.to_metadata_dto();
        ConnectorShallowDto {
            guid: m.guid,
            code: m.code,
            description: m.description,
            port: m.port,
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                .collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_shallow_dto).collect(),
        }
    }

    pub fn to_full_dto(&self) -> ConnectorFullDto {
        let m = self.to_metadata_dto();
        ConnectorFullDto {
            guid: m.guid,
            code: m.code,
            description: m.description,
            port: m.port,
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                .collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_full_dto).collect(),
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
        w.tag("connector").str(self.guid.as_str()).str(&self.code).opt_str(self.description.as_deref());
        if let Some(port) = self.port.as_ref().and_then(|p| p.upgrade()) {
            if let Ok(port) = port.read() {
                w.str(port.guid.as_str());
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
