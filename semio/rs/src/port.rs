use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
use crate::geom::{Coord, Vector};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};

pub type PortStoreRef = Arc<RwLock<PortStore>>;
pub type PortStoreWeak = std::sync::Weak<RwLock<PortStore>>;

/// Connection anchor on a [`crate::typ::TypeStore`].
#[derive(Debug)]
pub struct PortStore {
    pub guid: Guid,
    pub id: Option<String>,
    pub family: Option<String>,
    pub compatible_families: Vec<String>,
    pub mandatory: Option<bool>,
    pub t: Option<f64>,
    pub description: Option<String>,
    pub point: Option<Coord>,
    pub direction: Option<Vector>,
    pub qualities: Vec<QualityStoreRef>,
    pub attributes: Vec<AttributeStore>,
    hash_cache: OnceLock<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PortIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PortMetadataDto {
    pub guid: Guid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "compatibleFamilies")]
    pub compatible_families: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point: Option<Coord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Vector>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PortShallowDto {
    pub guid: Guid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "compatibleFamilies")]
    pub compatible_families: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point: Option<Coord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Vector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PortFullDto {
    pub guid: Guid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "compatibleFamilies")]
    pub compatible_families: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point: Option<Coord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Vector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeFullDto>,
}

impl PortStore {
    pub fn new() -> Self {
        Self {
            guid: Guid::new_v7(),
            id: None,
            family: None,
            compatible_families: Vec::new(),
            mandatory: None,
            t: None,
            description: None,
            point: None,
            direction: None,
            qualities: Vec::new(),
            attributes: Vec::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: PortIdDto) -> Self {
        Self {
            guid: d.guid,
            id: None,
            family: None,
            compatible_families: Vec::new(),
            mandatory: None,
            t: None,
            description: None,
            point: None,
            direction: None,
            qualities: Vec::new(),
            attributes: Vec::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: PortMetadataDto) -> Self {
        Self {
            guid: d.guid,
            id: d.id,
            family: d.family,
            compatible_families: d.compatible_families,
            mandatory: d.mandatory,
            t: d.t,
            description: d.description,
            point: d.point,
            direction: d.direction,
            qualities: Vec::new(),
            attributes: Vec::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: PortShallowDto) -> Self {
        let mut s = Self::from_metadata_dto(PortMetadataDto {
            guid: d.guid,
            id: d.id,
            family: d.family,
            compatible_families: d.compatible_families,
            mandatory: d.mandatory,
            t: d.t,
            description: d.description,
            point: d.point,
            direction: d.direction,
        });
        s.qualities = d
            .qualities
            .into_iter()
            .map(|q| Arc::new(RwLock::new(QualityStore::from_shallow_dto(q))))
            .collect();
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_shallow_dto).collect();
        s
    }

    pub fn from_full_dto(d: PortFullDto) -> Self {
        let mut s = Self::from_metadata_dto(PortMetadataDto {
            guid: d.guid,
            id: d.id,
            family: d.family,
            compatible_families: d.compatible_families,
            mandatory: d.mandatory,
            t: d.t,
            description: d.description,
            point: d.point,
            direction: d.direction,
        });
        s.qualities = d
            .qualities
            .into_iter()
            .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
            .collect();
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_full_dto).collect();
        s
    }

    pub fn to_id_dto(&self) -> PortIdDto {
        PortIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> PortMetadataDto {
        PortMetadataDto {
            guid: self.guid.clone(),
            id: self.id.clone(),
            family: self.family.clone(),
            compatible_families: self.compatible_families.clone(),
            mandatory: self.mandatory,
            t: self.t,
            description: self.description.clone(),
            point: self.point,
            direction: self.direction,
        }
    }

    pub fn to_shallow_dto(&self) -> PortShallowDto {
        let m = self.to_metadata_dto();
        PortShallowDto {
            guid: m.guid,
            id: m.id,
            family: m.family,
            compatible_families: m.compatible_families,
            mandatory: m.mandatory,
            t: m.t,
            description: m.description,
            point: m.point,
            direction: m.direction,
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                .collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_shallow_dto).collect(),
        }
    }

    pub fn to_full_dto(&self) -> PortFullDto {
        let m = self.to_metadata_dto();
        PortFullDto {
            guid: m.guid,
            id: m.id,
            family: m.family,
            compatible_families: m.compatible_families,
            mandatory: m.mandatory,
            t: m.t,
            description: m.description,
            point: m.point,
            direction: m.direction,
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
        w.tag("port")
            .str(self.guid.as_str())
            .opt_str(self.id.as_deref())
            .opt_str(self.family.as_deref());
        for f in &self.compatible_families {
            w.str(f);
        }
        w.opt_bool(self.mandatory).opt_f64(self.t);
        if let Some(p) = &self.point {
            p.hash_into(w);
        }
        if let Some(d) = &self.direction {
            d.hash_into(w);
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

impl Default for PortStore {
    fn default() -> Self {
        Self::new()
    }
}
