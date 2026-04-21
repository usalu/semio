use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::Attribute;
use crate::author::Author;
use crate::concept::Concept;
use crate::connector::{Connector, ConnectorDto, ConnectorRef};
use crate::geom::Location;
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::port::{Port, PortDto, PortRef};
use crate::prop::Prop;
use crate::quality::{Quality, QualityDto, QualityRef};
use crate::representation::{Representation, RepresentationDto, RepresentationRef};
use crate::tag::Tag;

pub type TypeRef = Arc<RwLock<Type>>;
pub type TypeWeak = Weak<RwLock<Type>>;

/// Reusable component definition: a type.
#[derive(Debug)]
pub struct Type {
    pub guid: Guid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub variant: Option<String>,
    pub stock: Option<i64>,
    pub virtual_: Option<bool>,
    pub unit: Option<String>,
    pub location: Option<Location>,
    pub ports: Vec<PortRef>,
    pub connectors: Vec<ConnectorRef>,
    pub representations: Vec<RepresentationRef>,
    pub authors: Vec<Author>,
    pub concepts: Vec<Concept>,
    pub tags: Vec<Tag>,
    pub qualities: Vec<QualityRef>,
    pub props: Vec<Prop>,
    pub attributes: Vec<Attribute>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub parent_kit: Weak<RwLock<crate::kit::Kit>>,
    hash_cache: OnceLock<String>,
}

impl Type {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            name: name.into(),
            description: None,
            icon: None,
            image: None,
            variant: None,
            stock: None,
            virtual_: None,
            unit: None,
            location: None,
            ports: Vec::new(),
            connectors: Vec::new(),
            representations: Vec::new(),
            authors: Vec::new(),
            concepts: Vec::new(),
            tags: Vec::new(),
            qualities: Vec::new(),
            props: Vec::new(),
            attributes: Vec::new(),
            created: None,
            updated: None,
            parent_kit: Weak::new(),
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
        w.tag("type")
            .str(self.guid.as_str())
            .str(&self.name)
            .opt_str(self.description.as_deref())
            .opt_str(self.variant.as_deref())
            .opt_str(self.unit.as_deref());
        for p in &self.ports {
            if let Ok(p) = p.read() { p.hash_into(w); }
        }
        for c in &self.connectors {
            if let Ok(c) = c.read() { c.hash_into(w); }
        }
        for r in &self.representations {
            if let Ok(r) = r.read() { r.hash_into(w); }
        }
        for a in &self.authors { a.hash_into(w); }
        for c in &self.concepts { c.hash_into(w); }
        for t in &self.tags { t.hash_into(w); }
        for q in &self.qualities {
            if let Ok(q) = q.read() { q.hash_into(w); }
        }
        for p in &self.props { p.hash_into(w); }
        for a in &self.attributes { a.hash_into(w); }
    }

    /// Find a port on this type by its guid.
    pub fn port(&self, guid: &str) -> Option<PortRef> {
        self.ports
            .iter()
            .find(|p| p.read().map(|p| p.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn connector(&self, guid: &str) -> Option<ConnectorRef> {
        self.connectors
            .iter()
            .find(|c| c.read().map(|c| c.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn representation(&self, guid: &str) -> Option<RepresentationRef> {
        self.representations
            .iter()
            .find(|r| r.read().map(|r| r.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TypeDto {
    #[serde(default)]
    pub guid: Option<Guid>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtual")]
    pub virtual_: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<PortDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connectors: Vec<ConnectorDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representations: Vec<RepresentationDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Author>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concepts: Vec<Concept>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Prop>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl From<&Type> for TypeDto {
    fn from(t: &Type) -> Self {
        TypeDto {
            guid: Some(t.guid.clone()),
            name: t.name.clone(),
            description: t.description.clone(),
            icon: t.icon.clone(),
            image: t.image.clone(),
            variant: t.variant.clone(),
            stock: t.stock,
            virtual_: t.virtual_,
            unit: t.unit.clone(),
            location: t.location,
            ports: t
                .ports
                .iter()
                .filter_map(|p| p.read().ok().map(|p| PortDto::from(&*p)))
                .collect(),
            connectors: t
                .connectors
                .iter()
                .filter_map(|c| c.read().ok().map(|c| ConnectorDto::from(&*c)))
                .collect(),
            representations: t
                .representations
                .iter()
                .filter_map(|r| r.read().ok().map(|r| RepresentationDto::from(&*r)))
                .collect(),
            authors: t.authors.clone(),
            concepts: t.concepts.clone(),
            tags: t.tags.clone(),
            qualities: t
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| QualityDto::from(&*q)))
                .collect(),
            props: t.props.clone(),
            attributes: t.attributes.clone(),
            created: t.created.clone(),
            updated: t.updated.clone(),
        }
    }
}

impl Type {
    /// Build a fully hydrated type from its DTO, wiring internal back-references.
    pub fn from_dto(d: TypeDto) -> TypeRef {
        let t = Arc::new(RwLock::new(Type {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            name: d.name,
            description: d.description,
            icon: d.icon,
            image: d.image,
            variant: d.variant,
            stock: d.stock,
            virtual_: d.virtual_,
            unit: d.unit,
            location: d.location,
            ports: Vec::new(),
            connectors: Vec::new(),
            representations: Vec::new(),
            authors: d.authors,
            concepts: d.concepts,
            tags: d.tags,
            qualities: d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(Quality::from(q))))
                .collect(),
            props: d.props,
            attributes: d.attributes,
            created: d.created,
            updated: d.updated,
            parent_kit: Weak::new(),
            hash_cache: OnceLock::new(),
        }));

        let port_refs: Vec<PortRef> = d
            .ports
            .into_iter()
            .map(|p| Arc::new(RwLock::new(Port::from_dto(p))))
            .collect();

        let mut connector_refs: Vec<ConnectorRef> = Vec::with_capacity(d.connectors.len());
        for cdto in d.connectors {
            let port_guid = cdto.port_guid.clone();
            let mut c = Connector::from_dto(cdto);
            c.parent_type = Arc::downgrade(&t);
            if let Some(pg) = port_guid {
                if let Some(pref) = port_refs.iter().find(|p| {
                    p.read().map(|p| p.guid == pg).unwrap_or(false)
                }) {
                    c.port = Some(Arc::downgrade(pref));
                }
            }
            connector_refs.push(Arc::new(RwLock::new(c)));
        }

        let mut rep_refs: Vec<RepresentationRef> = Vec::with_capacity(d.representations.len());
        for rdto in d.representations {
            let mut r = Representation::from_dto(rdto);
            r.parent_type = Arc::downgrade(&t);
            rep_refs.push(Arc::new(RwLock::new(r)));
        }

        if let Ok(mut t_mut) = t.write() {
            t_mut.ports = port_refs;
            t_mut.connectors = connector_refs;
            t_mut.representations = rep_refs;
        }
        t
    }
}
