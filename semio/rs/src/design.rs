use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::Attribute;
use crate::author::Author;
use crate::concept::Concept;
use crate::connection::{Connection, ConnectionDto, ConnectionRef};
use crate::geom::{Camera, Location, Plane};
use crate::group::{Group, GroupDto, GroupRef};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::layer::{Layer, LayerDto, LayerRef};
use crate::piece::{FlattenedPiece, Piece, PieceDto, PieceRef};
use crate::prop::Prop;
use crate::quality::{Quality, QualityDto, QualityRef};
use crate::side::Side;
use crate::stat::Stat;
use crate::tag::Tag;
use crate::typ::TypeRef;

pub type DesignRef = Arc<RwLock<Design>>;
pub type DesignWeak = Weak<RwLock<Design>>;

/// A placed/composed design: a scene of pieces joined by connections.
#[derive(Debug)]
pub struct Design {
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
    pub pieces: Vec<PieceRef>,
    pub connections: Vec<ConnectionRef>,
    pub layers: Vec<LayerRef>,
    pub groups: Vec<GroupRef>,
    pub authors: Vec<Author>,
    pub concepts: Vec<Concept>,
    pub tags: Vec<Tag>,
    pub qualities: Vec<QualityRef>,
    pub props: Vec<Prop>,
    pub attributes: Vec<Attribute>,
    pub stats: Vec<Stat>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub parent_kit: Weak<RwLock<crate::kit::Kit>>,
    hash_cache: OnceLock<String>,
    flatten_cache: OnceLock<FlattenedDesign>,
}

/// Result of flattening a design: world-space poses for every piece.
#[derive(Clone, Debug, Default)]
pub struct FlattenedDesign {
    pub pieces: Vec<(Guid, FlattenedPiece)>,
}

impl Design {
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
            flatten_cache: OnceLock::new(),
        }
    }

    pub fn invalidate_hash(&mut self) {
        self.hash_cache = OnceLock::new();
        self.flatten_cache = OnceLock::new();
    }

    pub fn invalidate_flatten(&mut self) {
        self.flatten_cache = OnceLock::new();
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
            if let Ok(p) = p.read() { p.hash_into(w); }
        }
        for c in &self.connections {
            if let Ok(c) = c.read() { c.hash_into(w); }
        }
        for l in &self.layers {
            if let Ok(l) = l.read() { l.hash_into(w); }
        }
        for g in &self.groups {
            if let Ok(g) = g.read() { g.hash_into(w); }
        }
        for a in &self.authors { a.hash_into(w); }
        for c in &self.concepts { c.hash_into(w); }
        for t in &self.tags { t.hash_into(w); }
        for q in &self.qualities {
            if let Ok(q) = q.read() { q.hash_into(w); }
        }
        for p in &self.props { p.hash_into(w); }
        for a in &self.attributes { a.hash_into(w); }
        for s in &self.stats { s.hash_into(w); }
    }

    pub fn piece(&self, guid: &str) -> Option<PieceRef> {
        self.pieces
            .iter()
            .find(|p| p.read().map(|p| p.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn connection(&self, guid: &str) -> Option<ConnectionRef> {
        self.connections
            .iter()
            .find(|c| c.read().map(|c| c.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn layer(&self, guid: &str) -> Option<LayerRef> {
        self.layers
            .iter()
            .find(|l| l.read().map(|l| l.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn group(&self, guid: &str) -> Option<GroupRef> {
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
                let touches = |s: &Side| -> bool {
                    s.piece
                        .upgrade()
                        .and_then(|p| p.read().ok().map(|p| p.guid.clone()))
                        .map(|g| piece_guids.iter().any(|x| *x == g))
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

    /// Compute the flattened (world-space) pose of every piece in this design.
    /// Implements the identity/default pose layout; chained connection
    /// resolution is handled during explicit flatten operations elsewhere.
    pub fn flatten(&self) -> FlattenedDesign {
        self.flatten_cache
            .get_or_init(|| {
                let mut out = Vec::with_capacity(self.pieces.len());
                for p in &self.pieces {
                    if let Ok(p) = p.read() {
                        let plane = p.plane.unwrap_or_else(Plane::world_xy);
                        let center = p.center.unwrap_or_default();
                        out.push((p.guid.clone(), FlattenedPiece { plane, center }));
                    }
                }
                FlattenedDesign { pieces: out }
            })
            .clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DesignDto {
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
    pub view: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Camera>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pieces: Vec<PieceDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<ConnectionDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<LayerDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<GroupDto>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats: Vec<Stat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl From<&Design> for DesignDto {
    fn from(d: &Design) -> Self {
        DesignDto {
            guid: Some(d.guid.clone()),
            name: d.name.clone(),
            description: d.description.clone(),
            icon: d.icon.clone(),
            image: d.image.clone(),
            variant: d.variant.clone(),
            view: d.view.clone(),
            location: d.location,
            camera: d.camera,
            unit: d.unit.clone(),
            pieces: d
                .pieces
                .iter()
                .filter_map(|p| p.read().ok().map(|p| PieceDto::from(&*p)))
                .collect(),
            connections: d
                .connections
                .iter()
                .filter_map(|c| c.read().ok().map(|c| ConnectionDto::from(&*c)))
                .collect(),
            layers: d
                .layers
                .iter()
                .filter_map(|l| l.read().ok().map(|l| LayerDto::from(&*l)))
                .collect(),
            groups: d
                .groups
                .iter()
                .filter_map(|g| g.read().ok().map(|g| GroupDto::from(&*g)))
                .collect(),
            authors: d.authors.clone(),
            concepts: d.concepts.clone(),
            tags: d.tags.clone(),
            qualities: d
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| QualityDto::from(&*q)))
                .collect(),
            props: d.props.clone(),
            attributes: d.attributes.clone(),
            stats: d.stats.clone(),
            created: d.created.clone(),
            updated: d.updated.clone(),
        }
    }
}

impl Design {
    /// Build a hydrated design, wiring pieces' type references via `type_index`
    /// and connection sides to the freshly-built piece/port graph.
    pub fn from_dto(d: DesignDto, type_index: &HashMap<Guid, TypeRef>) -> DesignRef {
        let design = Arc::new(RwLock::new(Design {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
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
            stats: d.stats,
            created: d.created,
            updated: d.updated,
            parent_kit: Weak::new(),
            hash_cache: OnceLock::new(),
            flatten_cache: OnceLock::new(),
        }));

        let mut piece_index: HashMap<Guid, PieceRef> = HashMap::new();
        let mut piece_refs: Vec<PieceRef> = Vec::new();
        for pdto in d.pieces {
            let type_guid = pdto.type_guid.clone();
            let mut piece = Piece::from_dto(pdto);
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

        let mut connection_refs: Vec<ConnectionRef> = Vec::with_capacity(d.connections.len());
        for cdto in d.connections {
            let connected_dto = cdto.connected.clone();
            let connecting_dto = cdto.connecting.clone();
            let mut c = Connection::from_dto(cdto);
            c.parent_design = Arc::downgrade(&design);
            if let Some(p) = piece_index.get(&connected_dto.piece_guid) {
                c.connected.piece = Arc::downgrade(p);
                if let Some(pg) = &connected_dto.port_guid {
                    if let Ok(piece) = p.read() {
                        if let Some(t) = piece.type_ref.as_ref().and_then(|t| t.upgrade()) {
                            if let Ok(t) = t.read() {
                                if let Some(port) = t.port(pg.as_str()) {
                                    c.connected.port = Some(Arc::downgrade(&port));
                                }
                            }
                        }
                    }
                }
            }
            if let Some(p) = piece_index.get(&connecting_dto.piece_guid) {
                c.connecting.piece = Arc::downgrade(p);
                if let Some(pg) = &connecting_dto.port_guid {
                    if let Ok(piece) = p.read() {
                        if let Some(t) = piece.type_ref.as_ref().and_then(|t| t.upgrade()) {
                            if let Ok(t) = t.read() {
                                if let Some(port) = t.port(pg.as_str()) {
                                    c.connecting.port = Some(Arc::downgrade(&port));
                                }
                            }
                        }
                    }
                }
            }
            connection_refs.push(Arc::new(RwLock::new(c)));
        }

        let layer_refs: Vec<LayerRef> = d
            .layers
            .into_iter()
            .map(|l| {
                let mut layer = Layer::from(l);
                layer.parent_design = Arc::downgrade(&design);
                Arc::new(RwLock::new(layer))
            })
            .collect();

        let group_refs: Vec<GroupRef> = d
            .groups
            .into_iter()
            .map(|g| {
                let mut group = Group::from_dto(g.clone());
                group.parent_design = Arc::downgrade(&design);
                for pg in &g.piece_guids {
                    if let Some(p) = piece_index.get(pg) {
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
}
