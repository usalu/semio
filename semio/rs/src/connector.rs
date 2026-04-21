use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::Attribute;
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::port::PortWeak;
use crate::quality::{Quality, QualityDto, QualityRef};

pub type ConnectorRef = Arc<RwLock<Connector>>;
pub type ConnectorWeak = Weak<RwLock<Connector>>;

/// A named socket on a [`crate::typ::Type`] that references a concrete port.
#[derive(Debug)]
pub struct Connector {
    pub guid: Guid,
    pub code: String,
    pub description: Option<String>,
    pub port: Option<PortWeak>,
    pub qualities: Vec<QualityRef>,
    pub attributes: Vec<Attribute>,
    /// Back-reference to the owning type.
    pub parent_type: Weak<RwLock<crate::typ::Type>>,
    hash_cache: OnceLock<String>,
}

impl Connector {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            code: code.into(),
            description: None,
            port: None,
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ConnectorDto {
    #[serde(default)]
    pub guid: Option<Guid>,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portGuid")]
    pub port_guid: Option<Guid>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute>,
}

impl From<&Connector> for ConnectorDto {
    fn from(c: &Connector) -> Self {
        ConnectorDto {
            guid: Some(c.guid.clone()),
            code: c.code.clone(),
            description: c.description.clone(),
            port_guid: c
                .port
                .as_ref()
                .and_then(|p| p.upgrade())
                .and_then(|p| p.read().ok().map(|p| p.guid.clone())),
            qualities: c
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| QualityDto::from(&*q)))
                .collect(),
            attributes: c.attributes.clone(),
        }
    }
}

impl Connector {
    pub fn from_dto(d: ConnectorDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            code: d.code,
            description: d.description,
            port: None,
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
