use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
use crate::author::{AuthorFullDto, AuthorShallowDto, AuthorStore};
use crate::concept::{ConceptFullDto, ConceptShallowDto, ConceptStore};
use crate::connection::{ConnectionFullDto, ConnectionShallowDto, ConnectionStore, ConnectionStoreRef};
use crate::geom::{Camera, Location};
use crate::group::{GroupFullDto, GroupShallowDto, GroupStore, GroupStoreRef};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::layer::{LayerFullDto, LayerShallowDto, LayerStore, LayerStoreRef};
use crate::piece::{PieceFullDto, PieceShallowDto, PieceStore, PieceStoreRef};
use crate::prop::{PropFullDto, PropShallowDto, PropStore};
use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
use crate::stat::{StatFullDto, StatShallowDto, StatStore};
use crate::tag::{TagFullDto, TagShallowDto, TagStore};
use crate::typ::TypeStoreRef;

pub type DesignStoreRef = Arc<RwLock<DesignStore>>;
pub type DesignStoreWeak = Weak<RwLock<DesignStore>>;

/// A placed/composed design: a scene of pieces joined by connections.
#[derive(Debug)]
pub struct DesignStore {
    pub guid: Guid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub variant: Option<String>,
    pub view: Option<String>,
    pub location: Option<Location>,
    pub camera: Option<Camera>,
    pub unit: Option<String>,
    pub pieces: Vec<PieceStoreRef>,
    pub connections: Vec<ConnectionStoreRef>,
    pub layers: Vec<LayerStoreRef>,
    pub groups: Vec<GroupStoreRef>,
    pub authors: Vec<AuthorStore>,
    pub concepts: Vec<ConceptStore>,
    pub tags: Vec<TagStore>,
    pub qualities: Vec<QualityStoreRef>,
    pub props: Vec<PropStore>,
    pub attributes: Vec<AttributeStore>,
    pub stats: Vec<StatStore>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub parent_kit: Weak<RwLock<crate::kit::KitStore>>,
    hash_cache: OnceLock<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DesignIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DesignMetadataDto {
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
    pub view: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Camera>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kit: Option<crate::kit::KitIdDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DesignShallowDto {
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
    pub view: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Camera>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kit: Option<crate::kit::KitIdDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pieces: Vec<PieceShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<ConnectionShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<LayerShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<GroupShallowDto>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats: Vec<StatShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DesignFullDto {
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
    pub view: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Camera>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kit: Option<crate::kit::KitIdDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pieces: Vec<PieceFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<ConnectionFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<LayerFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<GroupFullDto>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats: Vec<StatFullDto>,
}

impl DesignStore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            name: name.into(),
            description: None,
            icon: None,
            image: None,
            variant: None,
            view: None,
            location: None,
            camera: None,
            unit: None,
            pieces: Vec::new(),
            connections: Vec::new(),
            layers: Vec::new(),
            groups: Vec::new(),
            authors: Vec::new(),
            concepts: Vec::new(),
            tags: Vec::new(),
            qualities: Vec::new(),
            props: Vec::new(),
            attributes: Vec::new(),
            stats: Vec::new(),
            created: None,
            updated: None,
            parent_kit: Weak::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn invalidate_hash(&mut self) {
        self.hash_cache = OnceLock::new();
    }

    /// Clear per-piece pose flatten caches (does not touch content hashes).
    pub fn invalidate_piece_pose_caches(&mut self) {
        for p in &self.pieces {
            if let Ok(mut pw) = p.write() {
                pw.invalidate_pose_caches();
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
        w.tag("design")
            .str(self.guid.as_str())
            .str(&self.name)
            .opt_str(self.description.as_deref())
            .opt_str(self.variant.as_deref())
            .opt_str(self.view.as_deref())
            .opt_str(self.unit.as_deref());
        for p in &self.pieces {
            if let Ok(p) = p.read() {
                p.hash_into(w);
            }
        }
        for c in &self.connections {
            if let Ok(c) = c.read() {
                c.hash_into(w);
            }
        }
        for l in &self.layers {
            if let Ok(l) = l.read() {
                l.hash_into(w);
            }
        }
        for g in &self.groups {
            if let Ok(g) = g.read() {
                g.hash_into(w);
            }
        }
        for a in &self.authors {
            a.hash_into(w);
        }
        for c in &self.concepts {
            c.hash_into(w);
        }
        for t in &self.tags {
            t.hash_into(w);
        }
        for q in &self.qualities {
            if let Ok(q) = q.read() {
                q.hash_into(w);
            }
        }
        for p in &self.props {
            p.hash_into(w);
        }
        for a in &self.attributes {
            a.hash_into(w);
        }
        for s in &self.stats {
            s.hash_into(w);
        }
    }

    pub fn piece(&self, guid: &str) -> Option<PieceStoreRef> {
        self.pieces
            .iter()
            .find(|p| p.read().map(|p| p.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn connection(&self, guid: &str) -> Option<ConnectionStoreRef> {
        self.connections
            .iter()
            .find(|c| c.read().map(|c| c.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn layer(&self, guid: &str) -> Option<LayerStoreRef> {
        self.layers
            .iter()
            .find(|l| l.read().map(|l| l.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn group(&self, guid: &str) -> Option<GroupStoreRef> {
        self.groups
            .iter()
            .find(|g| g.read().map(|g| g.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    /// Remove any pieces matching the given guids (and all connections touching
    /// them). Returns the number of pieces actually removed.
    pub fn delete_pieces(&mut self, piece_guids: &[Guid]) -> usize {
        let before = self.pieces.len();
        self.pieces.retain(|p| {
            p.read()
                .map(|p| !piece_guids.iter().any(|g| *g == p.guid))
                .unwrap_or(true)
        });
        self.connections.retain(|c| {
            if let Ok(c) = c.read() {
                let touches = |s: &crate::side::SideStore| -> bool {
                    s.piece
                        .upgrade()
                        .and_then(|p| p.read().ok().map(|p| p.guid.clone()))
                        .map(|g| piece_guids.contains(&g))
                        .unwrap_or(false)
                };
                !(touches(&c.connected) || touches(&c.connecting))
            } else {
                true
            }
        });
        self.invalidate_hash();
        before - self.pieces.len()
    }

    pub fn from_id_dto(d: DesignIdDto) -> Self {
        Self {
            guid: d.guid,
            name: String::new(),
            description: None,
            icon: None,
            image: None,
            variant: None,
            view: None,
            location: None,
            camera: None,
            unit: None,
            pieces: Vec::new(),
            connections: Vec::new(),
            layers: Vec::new(),
            groups: Vec::new(),
            authors: Vec::new(),
            concepts: Vec::new(),
            tags: Vec::new(),
            qualities: Vec::new(),
            props: Vec::new(),
            attributes: Vec::new(),
            stats: Vec::new(),
            created: None,
            updated: None,
            parent_kit: Weak::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: DesignMetadataDto) -> Self {
        Self {
            guid: d.guid,
            name: d.name,
            description: d.description,
            icon: d.icon,
            image: d.image,
            variant: d.variant,
            view: d.view,
            location: d.location,
            camera: d.camera,
            unit: d.unit,
            pieces: Vec::new(),
            connections: Vec::new(),
            layers: Vec::new(),
            groups: Vec::new(),
            authors: Vec::new(),
            concepts: Vec::new(),
            tags: Vec::new(),
            qualities: Vec::new(),
            props: Vec::new(),
            attributes: Vec::new(),
            stats: Vec::new(),
            created: d.created,
            updated: d.updated,
            parent_kit: Weak::new(),
            hash_cache: OnceLock::new(),
        }
    }

    /// Build a fully hydrated design from `DesignFullDto`, wiring graph pointers.
    pub fn hydrate_from_full_dto(d: DesignFullDto, type_index: &HashMap<Guid, TypeStoreRef>) -> DesignStoreRef {
        let DesignFullDto {
            guid,
            name,
            description,
            icon,
            image,
            variant,
            view,
            location,
            camera,
            unit,
            created,
            updated,
            kit: _,
            pieces,
            connections,
            layers,
            groups,
            authors,
            concepts,
            tags,
            qualities,
            props,
            attributes,
            stats,
        } = d;

        let design = Arc::new(RwLock::new(DesignStore {
            guid: guid.clone(),
            name: name.clone(),
            description: description.clone(),
            icon: icon.clone(),
            image: image.clone(),
            variant: variant.clone(),
            view: view.clone(),
            location,
            camera,
            unit: unit.clone(),
            pieces: Vec::new(),
            connections: Vec::new(),
            layers: Vec::new(),
            groups: Vec::new(),
            authors: authors.into_iter().map(AuthorStore::from_full_dto).collect(),
            concepts: concepts.into_iter().map(ConceptStore::from_full_dto).collect(),
            tags: tags.into_iter().map(TagStore::from_full_dto).collect(),
            qualities: qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
                .collect(),
            props: props.into_iter().map(PropStore::from_full_dto).collect(),
            attributes: attributes.into_iter().map(AttributeStore::from_full_dto).collect(),
            stats: stats.into_iter().map(StatStore::from_full_dto).collect(),
            created: created.clone(),
            updated: updated.clone(),
            parent_kit: Weak::new(),
            hash_cache: OnceLock::new(),
        }));

        let mut piece_index: HashMap<Guid, PieceStoreRef> = HashMap::new();
        let mut piece_refs: Vec<PieceStoreRef> = Vec::new();
        for pdto in pieces {
            let type_guid = pdto.r#type.as_ref().map(|t| t.guid.clone());
            let mut piece = PieceStore::from_full_dto(pdto);
            piece.parent_design = Arc::downgrade(&design);
            if let Some(tg) = type_guid {
                if let Some(tref) = type_index.get(&tg) {
                    piece.type_ref = Some(Arc::downgrade(tref));
                }
            }
            let guid = piece.guid.clone();
            let arc = Arc::new(RwLock::new(piece));
            piece_index.insert(guid, arc.clone());
            piece_refs.push(arc);
        }

        let mut connection_refs: Vec<ConnectionStoreRef> = Vec::with_capacity(connections.len());
        for cdto in connections {
            let connected_meta = cdto.connected.clone();
            let connecting_meta = cdto.connecting.clone();
            let mut c = ConnectionStore::from_full_dto(cdto);
            c.parent_design = Arc::downgrade(&design);
            if let Some(p) = piece_index.get(&connected_meta.piece.guid) {
                c.connected.piece = Arc::downgrade(p);
                if let Some(pg) = &connected_meta.port {
                    if let Ok(piece) = p.read() {
                        if let Some(t) = piece.type_ref.as_ref().and_then(|t| t.upgrade()) {
                            if let Ok(t) = t.read() {
                                if let Some(port) = t.port(pg.guid.as_str()) {
                                    c.connected.port = Some(Arc::downgrade(&port));
                                }
                            }
                        }
                    }
                }
                if let Some(dpg) = &connected_meta.design_piece {
                    if let Some(dp) = piece_index.get(&dpg.guid) {
                        c.connected.design_piece = Some(Arc::downgrade(dp));
                    }
                }
            }
            if let Some(p) = piece_index.get(&connecting_meta.piece.guid) {
                c.connecting.piece = Arc::downgrade(p);
                if let Some(pg) = &connecting_meta.port {
                    if let Ok(piece) = p.read() {
                        if let Some(t) = piece.type_ref.as_ref().and_then(|t| t.upgrade()) {
                            if let Ok(t) = t.read() {
                                if let Some(port) = t.port(pg.guid.as_str()) {
                                    c.connecting.port = Some(Arc::downgrade(&port));
                                }
                            }
                        }
                    }
                }
                if let Some(dpg) = &connecting_meta.design_piece {
                    if let Some(dp) = piece_index.get(&dpg.guid) {
                        c.connecting.design_piece = Some(Arc::downgrade(dp));
                    }
                }
            }
            connection_refs.push(Arc::new(RwLock::new(c)));
        }

        let layer_refs: Vec<LayerStoreRef> = layers
            .into_iter()
            .map(|l| {
                let mut layer = LayerStore::from_full_dto(l);
                layer.parent_design = Arc::downgrade(&design);
                Arc::new(RwLock::new(layer))
            })
            .collect();

        let group_refs: Vec<GroupStoreRef> = groups
            .into_iter()
            .map(|g| {
                let pids: Vec<Guid> = g.pieces.iter().map(|p| p.guid.clone()).collect();
                let mut group = GroupStore::from_full_dto(g);
                group.parent_design = Arc::downgrade(&design);
                for pid in pids {
                    if let Some(p) = piece_index.get(&pid) {
                        group.pieces.push(Arc::downgrade(p));
                    }
                }
                Arc::new(RwLock::new(group))
            })
            .collect();

        if let Ok(mut d_mut) = design.write() {
            d_mut.pieces = piece_refs;
            d_mut.connections = connection_refs;
            d_mut.layers = layer_refs;
            d_mut.groups = group_refs;
        }
        design
    }

    pub fn to_id_dto(&self) -> DesignIdDto {
        DesignIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> DesignMetadataDto {
        let kit = self
            .parent_kit
            .upgrade()
            .and_then(|k| k.read().ok().map(|k| crate::kit::KitIdDto { guid: k.guid.clone() }));
        DesignMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            icon: self.icon.clone(),
            image: self.image.clone(),
            variant: self.variant.clone(),
            view: self.view.clone(),
            location: self.location,
            camera: self.camera,
            unit: self.unit.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
            kit,
        }
    }

    pub fn to_shallow_dto(&self) -> DesignShallowDto {
        let m = self.to_metadata_dto();
        DesignShallowDto {
            guid: m.guid,
            name: m.name,
            description: m.description,
            icon: m.icon,
            image: m.image,
            variant: m.variant,
            view: m.view,
            location: m.location,
            camera: m.camera,
            unit: m.unit,
            created: m.created,
            updated: m.updated,
            kit: m.kit,
            pieces: self
                .pieces
                .iter()
                .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                .collect(),
            connections: self
                .connections
                .iter()
                .filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto()))
                .collect(),
            layers: self
                .layers
                .iter()
                .filter_map(|l| l.read().ok().map(|l| l.to_shallow_dto()))
                .collect(),
            groups: self
                .groups
                .iter()
                .filter_map(|g| g.read().ok().map(|g| g.to_shallow_dto()))
                .collect(),
            authors: self.authors.iter().map(AuthorStore::to_shallow_dto).collect(),
            concepts: self.concepts.iter().map(ConceptStore::to_shallow_dto).collect(),
            tags: self.tags.iter().map(TagStore::to_shallow_dto).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                .collect(),
            props: self.props.iter().map(PropStore::to_shallow_dto).collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_shallow_dto).collect(),
            stats: self.stats.iter().map(StatStore::to_shallow_dto).collect(),
        }
    }

    pub fn to_full_dto(&self) -> DesignFullDto {
        let m = self.to_metadata_dto();
        DesignFullDto {
            guid: m.guid,
            name: m.name,
            description: m.description,
            icon: m.icon,
            image: m.image,
            variant: m.variant,
            view: m.view,
            location: m.location,
            camera: m.camera,
            unit: m.unit,
            created: m.created,
            updated: m.updated,
            kit: m.kit,
            pieces: self
                .pieces
                .iter()
                .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                .collect(),
            connections: self
                .connections
                .iter()
                .filter_map(|c| c.read().ok().map(|c| c.to_full_dto()))
                .collect(),
            layers: self
                .layers
                .iter()
                .filter_map(|l| l.read().ok().map(|l| l.to_full_dto()))
                .collect(),
            groups: self
                .groups
                .iter()
                .filter_map(|g| g.read().ok().map(|g| g.to_full_dto()))
                .collect(),
            authors: self.authors.iter().map(AuthorStore::to_full_dto).collect(),
            concepts: self.concepts.iter().map(ConceptStore::to_full_dto).collect(),
            tags: self.tags.iter().map(TagStore::to_full_dto).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                .collect(),
            props: self.props.iter().map(PropStore::to_full_dto).collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_full_dto).collect(),
            stats: self.stats.iter().map(StatStore::to_full_dto).collect(),
        }
    }
}
