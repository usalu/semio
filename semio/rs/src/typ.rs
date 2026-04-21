use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore, AttributeStoreRef};
use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::geom::Location;
use crate::author::{AuthorFullDto, AuthorShallowDto, AuthorStore, AuthorStoreRef};
use crate::concept::{ConceptFullDto, ConceptShallowDto, ConceptStore, ConceptStoreRef};
use crate::connector::{ConnectorFullDto, ConnectorShallowDto, ConnectorStore, ConnectorStoreRef};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::port::{PortFullDto, PortShallowDto, PortStore, PortStoreRef};
use crate::prop::{PropFullDto, PropShallowDto, PropStore, PropStoreRef};
use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
use crate::representation::{RepresentationFullDto, RepresentationShallowDto, RepresentationStore, RepresentationStoreRef};
use crate::tag::{TagFullDto, TagShallowDto, TagStore, TagStoreRef};

pub type TypeStoreRef = Arc<RwLock<TypeStore>>;
pub type TypeStoreWeak = Weak<RwLock<TypeStore>>;

/// Reusable component definition: a type.
#[derive(Debug)]
pub struct TypeStore {
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
    pub ports: Vec<PortStoreRef>,
    pub connectors: Vec<ConnectorStoreRef>,
    pub representations: Vec<RepresentationStoreRef>,
    pub authors: Vec<AuthorStoreRef>,
    pub concepts: Vec<ConceptStoreRef>,
    pub tags: Vec<TagStoreRef>,
    pub qualities: Vec<QualityStoreRef>,
    pub props: Vec<PropStoreRef>,
    pub attributes: Vec<AttributeStoreRef>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub parent_kit: Weak<RwLock<crate::kit::KitStore>>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TypeIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TypeMetadataDto {
    pub guid: Guid,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TypeShallowDto {
    pub guid: Guid,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<PortShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connectors: Vec<ConnectorShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representations: Vec<RepresentationShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<AuthorShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concepts: Vec<ConceptShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<PropShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TypeFullDto {
    pub guid: Guid,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<PortFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connectors: Vec<ConnectorFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representations: Vec<RepresentationFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<AuthorFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concepts: Vec<ConceptFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<PropFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeFullDto>,
}

impl TypeStore {
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
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Type, self.guid.clone())
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        if let Some(k) = self.parent_kit.upgrade() {
            if let Ok(kr) = k.read() {
                kr.invalidate_hash();
            }
        }
    }

    pub fn invalidate_validation(&self) {
        if let Some(k) = self.parent_kit.upgrade() {
            if let Ok(kr) = k.read() {
                kr.invalidate_validation();
            }
        }
    }

    pub fn set_name(&mut self, name: String) {
        if self.name == name {
            return;
        }
        self.name = name;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "name",
        });
        self.invalidate_hash();
        self.invalidate_validation();
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
        self.invalidate_validation();
    }

    pub fn set_icon(&mut self, v: Option<String>) {
        if self.icon == v {
            return;
        }
        self.icon = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "icon",
        });
        self.invalidate_hash();
        self.invalidate_validation();
    }

    pub fn set_image(&mut self, v: Option<String>) {
        if self.image == v {
            return;
        }
        self.image = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "image",
        });
        self.invalidate_hash();
        self.invalidate_validation();
    }

    pub fn set_variant(&mut self, v: Option<String>) {
        if self.variant == v {
            return;
        }
        self.variant = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "variant",
        });
        self.invalidate_hash();
        self.invalidate_validation();
    }

    pub fn set_stock(&mut self, v: Option<i64>) {
        if self.stock == v {
            return;
        }
        self.stock = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "stock",
        });
        self.invalidate_hash();
        self.invalidate_validation();
    }

    pub fn set_virtual(&mut self, v: Option<bool>) {
        if self.virtual_ == v {
            return;
        }
        self.virtual_ = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "virtual",
        });
        self.invalidate_hash();
        self.invalidate_validation();
    }

    pub fn set_unit(&mut self, v: Option<String>) {
        if self.unit == v {
            return;
        }
        self.unit = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "unit",
        });
        self.invalidate_hash();
        self.invalidate_validation();
    }

    pub fn set_location(&mut self, v: Option<Location>) {
        if self.location == v {
            return;
        }
        self.location = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "location",
        });
        self.invalidate_hash();
        self.invalidate_validation();
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
        self.invalidate_validation();
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
        self.invalidate_validation();
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
            if let Ok(p) = p.read() {
                p.hash_into(w);
            }
        }
        for c in &self.connectors {
            if let Ok(c) = c.read() {
                c.hash_into(w);
            }
        }
        for r in &self.representations {
            if let Ok(r) = r.read() {
                r.hash_into(w);
            }
        }
        for a in &self.authors {
            if let Ok(a) = a.read() {
                a.hash_into(w);
            }
        }
        for c in &self.concepts {
            if let Ok(c) = c.read() {
                c.hash_into(w);
            }
        }
        for t in &self.tags {
            if let Ok(t) = t.read() {
                t.hash_into(w);
            }
        }
        for q in &self.qualities {
            if let Ok(q) = q.read() {
                q.hash_into(w);
            }
        }
        for p in &self.props {
            if let Ok(p) = p.read() {
                p.hash_into(w);
            }
        }
        for a in &self.attributes {
            if let Ok(a) = a.read() {
                a.hash_into(w);
            }
        }
    }

    pub fn port(&self, guid: &str) -> Option<PortStoreRef> {
        self.ports
            .iter()
            .find(|p| p.read().map(|p| p.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn connector(&self, guid: &str) -> Option<ConnectorStoreRef> {
        self.connectors
            .iter()
            .find(|c| c.read().map(|c| c.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn connector_for_port_guid(&self, port_guid: &Guid) -> Option<ConnectorStoreRef> {
        self.connectors
            .iter()
            .find(|c| {
                c.read()
                    .ok()
                    .and_then(|cr| {
                        cr.port
                            .as_ref()
                            .and_then(|w| w.upgrade())
                            .and_then(|p| p.read().ok().map(|pr| pr.guid == *port_guid))
                    })
                    .unwrap_or(false)
            })
            .cloned()
    }

    pub fn representation(&self, guid: &str) -> Option<RepresentationStoreRef> {
        self.representations
            .iter()
            .find(|r| r.read().map(|r| r.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn from_id_dto(d: TypeIdDto) -> Self {
        Self {
            guid: d.guid,
            name: String::new(),
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
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_metadata_dto(d: TypeMetadataDto) -> Self {
        Self {
            guid: d.guid,
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
            authors: Vec::new(),
            concepts: Vec::new(),
            tags: Vec::new(),
            qualities: Vec::new(),
            props: Vec::new(),
            attributes: Vec::new(),
            created: d.created,
            updated: d.updated,
            parent_kit: Weak::new(),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    /// Hydrate type graph from full DTO (ports, connectors, representations, kit link).
    /// Only [`crate::kit::KitStore::from_full_dto`] should construct types in host code.
    pub(crate) fn hydrate_from_full_dto(
        d: TypeFullDto,
        kit: &Arc<RwLock<crate::kit::KitStore>>,
        file_refs: &[crate::file::FileStoreRef],
    ) -> TypeStoreRef {
        let TypeFullDto {
            guid,
            name,
            description,
            icon,
            image,
            variant,
            stock,
            virtual_,
            unit,
            location,
            created,
            updated,
            ports,
            connectors,
            representations,
            authors,
            concepts,
            tags,
            qualities,
            props,
            attributes,
        } = d;

        let t = Arc::new(RwLock::new(TypeStore {
            guid: guid.clone(),
            name: name.clone(),
            description: description.clone(),
            icon: icon.clone(),
            image: image.clone(),
            variant: variant.clone(),
            stock,
            virtual_,
            unit: unit.clone(),
            location,
            ports: Vec::new(),
            connectors: Vec::new(),
            representations: Vec::new(),
            authors: authors
                .into_iter()
                .map(|a| Arc::new(RwLock::new(AuthorStore::from_full_dto(a))))
                .collect(),
            concepts: concepts
                .into_iter()
                .map(|c| Arc::new(RwLock::new(ConceptStore::from_full_dto(c))))
                .collect(),
            tags: tags
                .into_iter()
                .map(|t| Arc::new(RwLock::new(TagStore::from_full_dto(t))))
                .collect(),
            qualities: qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
                .collect(),
            props: props
                .into_iter()
                .map(|p| Arc::new(RwLock::new(PropStore::from_full_dto(p))))
                .collect(),
            attributes: attributes
                .into_iter()
                .map(|a| Arc::new(RwLock::new(AttributeStore::from_full_dto(a))))
                .collect(),
            created: created.clone(),
            updated: updated.clone(),
            parent_kit: Arc::downgrade(kit),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }));

        let tw_pre = Arc::downgrade(&t);
        let port_refs: Vec<PortStoreRef> = ports
            .into_iter()
            .map(|p| {
                let mut port = PortStore::from_full_dto(p);
                port.parent_type = tw_pre.clone();
                Arc::new(RwLock::new(port))
            })
            .collect();

        let mut connector_refs: Vec<ConnectorStoreRef> = Vec::with_capacity(connectors.len());
        for cdto in connectors {
            let port_guid = cdto.port.as_ref().map(|p| p.guid.clone());
            let mut c = ConnectorStore::from_full_dto(cdto);
            c.parent_type = Arc::downgrade(&t);
            if let Some(pg) = port_guid {
                if let Some(pref) = port_refs.iter().find(|p| p.read().map(|p| p.guid == pg).unwrap_or(false)) {
                    c.port = Some(Arc::downgrade(pref));
                }
            }
            connector_refs.push(Arc::new(RwLock::new(c)));
        }

        let mut rep_refs: Vec<RepresentationStoreRef> = Vec::with_capacity(representations.len());
        for rdto in representations {
            let file_guid = rdto.file.as_ref().map(|f| f.guid.clone());
            let mut r = RepresentationStore::from_full_dto(rdto);
            r.parent_type = Arc::downgrade(&t);
            if let Some(fg) = file_guid {
                if let Some(fref) = file_refs.iter().find(|f| f.read().map(|f| f.guid == fg).unwrap_or(false)) {
                    r.file = Some(Arc::downgrade(fref));
                }
            }
            rep_refs.push(Arc::new(RwLock::new(r)));
        }

        if let Ok(mut t_mut) = t.write() {
            let tw = Arc::downgrade(&t);
            for a in &t_mut.authors {
                if let Ok(mut aw) = a.write() {
                    aw.parent_type = Some(tw.clone());
                }
            }
            for c in &t_mut.concepts {
                if let Ok(mut cw) = c.write() {
                    cw.parent_type = Some(tw.clone());
                }
            }
            for tag in &t_mut.tags {
                if let Ok(mut tw0) = tag.write() {
                    tw0.parent_type = Some(tw.clone());
                }
            }
            for q in &t_mut.qualities {
                if let Ok(mut qw) = q.write() {
                    qw.parent_type = Some(tw.clone());
                }
            }
            for p in &t_mut.props {
                if let Ok(mut pw) = p.write() {
                    pw.parent_type = Some(tw.clone());
                }
            }
            for a in &t_mut.attributes {
                if let Ok(mut aw) = a.write() {
                    aw.parent_type = Some(tw.clone());
                }
            }
            t_mut.ports = port_refs;
            t_mut.connectors = connector_refs;
            t_mut.representations = rep_refs;
        }
        t
    }

    pub fn to_id_dto(&self) -> TypeIdDto {
        TypeIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> TypeMetadataDto {
        TypeMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            icon: self.icon.clone(),
            image: self.image.clone(),
            variant: self.variant.clone(),
            stock: self.stock,
            virtual_: self.virtual_,
            unit: self.unit.clone(),
            location: self.location,
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> TypeShallowDto {
        let m = self.to_metadata_dto();
        TypeShallowDto {
            guid: m.guid,
            name: m.name,
            description: m.description,
            icon: m.icon,
            image: m.image,
            variant: m.variant,
            stock: m.stock,
            virtual_: m.virtual_,
            unit: m.unit,
            location: m.location,
            created: m.created,
            updated: m.updated,
            ports: self
                .ports
                .iter()
                .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                .collect(),
            connectors: self
                .connectors
                .iter()
                .filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto()))
                .collect(),
            representations: self
                .representations
                .iter()
                .filter_map(|r| r.read().ok().map(|r| r.to_shallow_dto()))
                .collect(),
            authors: self
                .authors
                .iter()
                .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                .collect(),
            concepts: self
                .concepts
                .iter()
                .filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto()))
                .collect(),
            tags: self.tags.iter().filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto())).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                .collect(),
            props: self.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto())).collect(),
            attributes: self
                .attributes
                .iter()
                .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                .collect(),
        }
    }

    pub fn to_full_dto(&self) -> TypeFullDto {
        let m = self.to_metadata_dto();
        TypeFullDto {
            guid: m.guid,
            name: m.name,
            description: m.description,
            icon: m.icon,
            image: m.image,
            variant: m.variant,
            stock: m.stock,
            virtual_: m.virtual_,
            unit: m.unit,
            location: m.location,
            created: m.created,
            updated: m.updated,
            ports: self
                .ports
                .iter()
                .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                .collect(),
            connectors: self
                .connectors
                .iter()
                .filter_map(|c| c.read().ok().map(|c| c.to_full_dto()))
                .collect(),
            representations: self
                .representations
                .iter()
                .filter_map(|r| r.read().ok().map(|r| r.to_full_dto()))
                .collect(),
            authors: self
                .authors
                .iter()
                .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                .collect(),
            concepts: self
                .concepts
                .iter()
                .filter_map(|c| c.read().ok().map(|c| c.to_full_dto()))
                .collect(),
            tags: self.tags.iter().filter_map(|t| t.read().ok().map(|t| t.to_full_dto())).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                .collect(),
            props: self.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_full_dto())).collect(),
            attributes: self
                .attributes
                .iter()
                .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                .collect(),
        }
    }
}
