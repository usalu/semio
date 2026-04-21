use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::Attribute;
use crate::geom::{Coord, Vector};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::quality::{Quality, QualityDto, QualityRef};

pub type PortRef = Arc<RwLock<Port>>;
pub type PortWeak = Weak<RwLock<Port>>;

/// Connection anchor on a [`crate::typ::Type`].
#[derive(Debug)]
pub struct Port {
    pub guid: Guid,
    pub id: Option<String>,
    pub family: Option<String>,
    pub compatible_families: Vec<String>,
    pub mandatory: Option<bool>,
    pub t: Option<f64>,
    pub description: Option<String>,
    pub point: Option<Coord>,
    pub direction: Option<Vector>,
    pub qualities: Vec<QualityRef>,
    pub attributes: Vec<Attribute>,
    hash_cache: OnceLock<String>,
}

impl Port {
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

impl Default for Port {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PortDto {
    #[serde(default)]
    pub guid: Option<Guid>,
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
    pub qualities: Vec<QualityDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute>,
}

impl From<&Port> for PortDto {
    fn from(p: &Port) -> Self {
        PortDto {
            guid: Some(p.guid.clone()),
            id: p.id.clone(),
            family: p.family.clone(),
            compatible_families: p.compatible_families.clone(),
            mandatory: p.mandatory,
            t: p.t,
            description: p.description.clone(),
            point: p.point,
            direction: p.direction,
            qualities: p
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| QualityDto::from(&*q)))
                .collect(),
            attributes: p.attributes.clone(),
        }
    }
}

impl Port {
    pub fn from_dto(d: PortDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            id: d.id,
            family: d.family,
            compatible_families: d.compatible_families,
            mandatory: d.mandatory,
            t: d.t,
            description: d.description,
            point: d.point,
            direction: d.direction,
            qualities: d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(Quality::from(q))))
                .collect(),
            attributes: d.attributes,
            hash_cache: OnceLock::new(),
        }
    }
}
