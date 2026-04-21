use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore, AttributeStoreRef};
use crate::author::{AuthorFullDto, AuthorShallowDto, AuthorStore, AuthorStoreRef};
use crate::concept::{ConceptFullDto, ConceptShallowDto, ConceptStore, ConceptStoreRef};
use crate::connection::{
    ConnectionFullDto, ConnectionMetadataDto, ConnectionShallowDto, ConnectionStore, ConnectionStoreRef,
};
use crate::connector::ConnectorStoreRef;
use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::geom::{Camera, Coord, Location, Plane};
use crate::group::{GroupFullDto, GroupShallowDto, GroupStore, GroupStoreRef};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStore;
use crate::layer::{LayerFullDto, LayerShallowDto, LayerStore, LayerStoreRef};
use crate::piece::{PieceFullDto, PieceShallowDto, PieceStore, PieceStoreRef};
use crate::prop::{PropFullDto, PropShallowDto, PropStore, PropStoreRef};
use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
use crate::side::{SideStore, SideStoreRef};
use crate::stat::{StatFullDto, StatShallowDto, StatStore, StatStoreRef};
use crate::tag::{TagFullDto, TagShallowDto, TagStore, TagStoreRef};
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
    pub authors: Vec<AuthorStoreRef>,
    pub concepts: Vec<ConceptStoreRef>,
    pub tags: Vec<TagStoreRef>,
    pub qualities: Vec<QualityStoreRef>,
    pub props: Vec<PropStoreRef>,
    pub attributes: Vec<AttributeStoreRef>,
    pub stats: Vec<StatStoreRef>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub parent_kit: Weak<RwLock<crate::kit::KitStore>>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
    flatten_cache: Cache<HashMap<Guid, (Plane, Coord)>>,
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

fn resolve_connector_for_side(side: &crate::side::SideStore, typ: &crate::typ::TypeStore) -> Option<ConnectorStoreRef> {
    if let Some(pw) = &side.port {
        if let Some(p) = pw.upgrade() {
            if let Ok(pr) = p.read() {
                return typ.connector_for_port_guid(&pr.guid);
            }
        }
    }
    typ.connectors.first().cloned()
}

fn connection_from_full_dto(
    cdto: ConnectionFullDto,
    piece_index: &HashMap<Guid, PieceStoreRef>,
    design_weak: DesignStoreWeak,
) -> ConnectionStoreRef {
    let s1 = Arc::new(RwLock::new(SideStore::empty_shell(cdto.connected.guid.clone())));
    let s2 = Arc::new(RwLock::new(SideStore::empty_shell(cdto.connecting.guid.clone())));
    wire_side_from_dto(&cdto.connected, &s1, piece_index);
    wire_side_from_dto(&cdto.connecting, &s2, piece_index);
    let conn = Arc::new(RwLock::new(ConnectionStore::empty_with_sides(
        cdto.guid.clone(),
        s1.clone(),
        s2.clone(),
    )));
    {
        let mut cw = conn.write().expect("connection write");
        cw.apply_metadata_fields(ConnectionMetadataDto {
            guid: cdto.guid.clone(),
            connected: cdto.connected.clone(),
            connecting: cdto.connecting.clone(),
            gap: cdto.gap,
            shift: cdto.shift,
            rise: cdto.rise,
            rotation: cdto.rotation,
            turn: cdto.turn,
            tilt: cdto.tilt,
            x: cdto.x,
            y: cdto.y,
            description: cdto.description.clone(),
        });
        cw.parent_design = design_weak.clone();
        cw.attributes = cdto
            .attributes
            .into_iter()
            .map(|a| Arc::new(RwLock::new(AttributeStore::from_full_dto(a))))
            .collect();
    }
    if let Ok(mut s1w) = s1.write() {
        s1w.parent_connection = Some(Arc::downgrade(&conn));
    }
    if let Ok(mut s2w) = s2.write() {
        s2w.parent_connection = Some(Arc::downgrade(&conn));
    }
    conn
}

fn wire_side_from_dto(
    meta: &crate::side::SideMetadataDto,
    side_ref: &SideStoreRef,
    piece_index: &HashMap<Guid, PieceStoreRef>,
) {
    if let Ok(mut w) = side_ref.write() {
        w.apply_metadata_dto(meta.clone());
        if let Some(pref) = piece_index.get(&meta.piece.guid) {
            w.set_piece_weak(Arc::downgrade(pref));
            if let Some(port_id) = &meta.port {
                if let Ok(pc) = pref.read() {
                    if let Some(tw) = &pc.type_ref {
                        if let Some(t) = tw.upgrade() {
                            if let Ok(tr) = t.read() {
                                for pr in &tr.ports {
                                    if let Ok(prr) = pr.read() {
                                        if prr.guid == port_id.guid {
                                            w.set_port_weak(Some(Arc::downgrade(pr)));
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if let Some(dp) = &meta.design_piece {
                if let Some(dpref) = piece_index.get(&dp.guid) {
                    w.set_design_piece_weak(Some(Arc::downgrade(dpref)));
                }
            }
        }
    }
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
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
            flatten_cache: Cache::default(),
        }
    }

    pub(crate) fn empty_shell(guid: Guid, name: String) -> Self {
        Self {
            guid,
            name,
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
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
            flatten_cache: Cache::default(),
        }
    }

    #[inline]
    pub(crate) fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    pub(crate) fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Design, self.guid.clone())
    }

    pub(crate) fn apply_metadata_fields(&mut self, d: DesignMetadataDto) {
        self.guid = d.guid;
        self.name = d.name;
        self.description = d.description;
        self.icon = d.icon;
        self.image = d.image;
        self.variant = d.variant;
        self.view = d.view;
        self.location = d.location;
        self.camera = d.camera;
        self.unit = d.unit;
        self.created = d.created;
        self.updated = d.updated;
        self.hash_cache.invalidate();
        self.flatten_cache.invalidate();
    }

    pub fn invalidate_hash(&self) {
        self.invalidate_hash_local();
        if let Some(k) = self.parent_kit.upgrade() {
            if let Ok(kr) = k.read() {
                kr.invalidate_hash();
            }
        }
    }

    /// Like [`Self::invalidate_hash`] but does not bubble to the parent kit (avoids deadlock when
    /// the kit already holds its write lock, e.g. during [`KitStore::apply_design_diff`]).
    pub(crate) fn invalidate_hash_local(&self) {
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
    }

    /// Invalidate flatten caches and emit flatten + derived events for all pieces in this design.
    ///
    /// When a [`crate::piece::PieceStore`] mutator bubbles here, it **holds that piece's write
    /// lock**; pass its guid so we can still list it in `FlattenInvalidated` without blocking on
    /// `read()` (which would deadlock).
    pub fn invalidate_flatten(&self) {
        self.invalidate_flatten_with_locked_piece(None);
    }

    pub(crate) fn invalidate_flatten_with_locked_piece(&self, locked_piece: Option<Guid>) {
        let design_guid = self.guid.clone();
        let mut piece_guids: Vec<Guid> = self
            .pieces
            .iter()
            .filter_map(|p| match p.try_read() {
                Ok(pr) => Some(pr.guid.clone()),
                Err(_) => locked_piece.clone(),
            })
            .collect();
        piece_guids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
        piece_guids.dedup();
        self.flatten_cache.invalidate();
        self.emit_ev(KitEvent::FlattenInvalidated {
            design: design_guid,
            pieces: piece_guids.clone(),
        });
        for p in &self.pieces {
            if let Ok(pr) = p.try_read() {
                pr.invalidate_flat_pose();
            }
        }
        for gid in &piece_guids {
            self.emit_ev(KitEvent::DerivedChanged {
                entity: EntityRef::new(EntityKind::Piece, gid.clone()),
                field: "flat_plane",
            });
            self.emit_ev(KitEvent::DerivedChanged {
                entity: EntityRef::new(EntityKind::Piece, gid.clone()),
                field: "flat_center",
            });
        }
    }

    pub fn invalidate_validation(&self) {
        if let Some(k) = self.parent_kit.upgrade() {
            if let Ok(k) = k.read() {
                k.invalidate_validation();
            }
        }
    }

    fn bubble_to_kit(&self) {
        if let Some(k) = self.parent_kit.upgrade() {
            if let Ok(k) = k.read() {
                k.invalidate_hash();
                k.invalidate_validation();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
    }

    pub fn set_view(&mut self, v: Option<String>) {
        if self.view == v {
            return;
        }
        self.view = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "view",
        });
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
    }

    pub fn set_camera(&mut self, v: Option<Camera>) {
        if self.camera == v {
            return;
        }
        self.camera = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "camera",
        });
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
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
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        self.invalidate_flatten();
        self.bubble_to_kit();
    }

    /// Flattened world-space plane and center per piece guid (BFS, Python `flattenDesignDict`).
    pub fn flatten_map(&self) -> HashMap<Guid, (Plane, Coord)> {
        self.flatten_cache.get_or_init(|| {
            let Some(k) = self.parent_kit.upgrade() else {
                return self.flatten_identity_only();
            };
            let Ok(kr) = k.read() else {
                return self.flatten_identity_only();
            };
            self.compute_flatten_with_kit(&*kr)
        })
    }

    fn flatten_identity_only(&self) -> HashMap<Guid, (Plane, Coord)> {
        let mut m = HashMap::new();
        for p in &self.pieces {
            if let Ok(pr) = p.read() {
                let pl = pr.plane.unwrap_or_else(Plane::world_xy);
                let ce = pr.center.unwrap_or_default();
                m.insert(pr.guid.clone(), (pl, ce));
            }
        }
        m
    }

    fn compute_flatten_with_kit(&self, kit: &KitStore) -> HashMap<Guid, (Plane, Coord)> {
        let mut types_by_guid: HashMap<Guid, TypeStoreRef> = HashMap::new();
        for t in &kit.types {
            if let Ok(tr) = t.read() {
                types_by_guid.insert(tr.guid.clone(), t.clone());
            }
        }
        let mut piece_map: HashMap<Guid, PieceStoreRef> = HashMap::new();
        for p in &self.pieces {
            if let Ok(pr) = p.read() {
                piece_map.insert(pr.guid.clone(), p.clone());
            }
        }
        if piece_map.is_empty() {
            return HashMap::new();
        }
        let mut adj: HashMap<Guid, Vec<(Guid, ConnectionStoreRef)>> = HashMap::new();
        for c in &self.connections {
            let Ok(conn) = c.read() else { continue };
            let Ok(s0) = conn.connected.read() else { continue };
            let Ok(s1) = conn.connecting.read() else { continue };
            let g0 = s0.piece.upgrade().and_then(|p| p.read().ok().map(|x| x.guid.clone()));
            let g1 = s1.piece.upgrade().and_then(|p| p.read().ok().map(|x| x.guid.clone()));
            let (Some(src), Some(tgt)) = (g0, g1) else { continue };
            if !piece_map.contains_key(&src) || !piece_map.contains_key(&tgt) {
                continue;
            }
            adj.entry(src.clone()).or_default().push((tgt.clone(), c.clone()));
            adj.entry(tgt).or_default().push((src, c.clone()));
        }
        let mut piece_planes: HashMap<Guid, Plane> = HashMap::new();
        let mut centers: HashMap<Guid, Coord> = HashMap::new();
        let mut visited: HashSet<Guid> = HashSet::new();

        let roots: Vec<Guid> = self
            .pieces
            .iter()
            .filter_map(|p| p.read().ok().map(|pr| pr.guid.clone()))
            .collect();
        for root in roots {
            if visited.contains(&root) {
                continue;
            }
            let mut q = VecDeque::new();
            q.push_back(root.clone());
            visited.insert(root.clone());
            if let Some(p) = piece_map.get(&root) {
                if let Ok(pr) = p.read() {
                    let pl = if pr.plane.is_some() && pr.center.is_some() {
                        pr.plane.unwrap()
                    } else {
                        Plane::world_xy()
                    };
                    let ce = pr.center.unwrap_or_default();
                    piece_planes.insert(root.clone(), pl);
                    centers.insert(root, ce);
                }
            }
            while let Some(current) = q.pop_front() {
                let Some(current_plane) = piece_planes.get(&current).cloned() else { continue };
                let Some(current_piece_ref) = piece_map.get(&current) else { continue };
                let Ok(current_piece) = current_piece_ref.read() else { continue };
                for (nbr, conn_ref) in adj.get(&current).cloned().unwrap_or_default() {
                    if visited.contains(&nbr) {
                        continue;
                    }
                    let Ok(conn) = conn_ref.read() else { continue };
                    let (parent_side, child_side) = {
                        let Ok(s0) = conn.connected.read() else { continue };
                        let Ok(s1) = conn.connecting.read() else { continue };
                        let g0 = s0.piece.upgrade().and_then(|p| p.read().ok().map(|x| x.guid.clone()));
                        let g1 = s1.piece.upgrade().and_then(|p| p.read().ok().map(|x| x.guid.clone()));
                        let (Some(a), Some(b)) = (g0, g1) else { continue };
                        if a == current && b == nbr {
                            (conn.connected.clone(), conn.connecting.clone())
                        } else if b == current && a == nbr {
                            (conn.connecting.clone(), conn.connected.clone())
                        } else {
                            continue;
                        }
                    };
                    let child_id = nbr;
                    let Ok(ps) = parent_side.read() else { continue };
                    let Ok(cs) = child_side.read() else { continue };
                    let Some(child_pref) = piece_map.get(&child_id) else { continue };
                    let Ok(child_piece) = child_pref.read() else { continue };
                    let parent_type_guid = current_piece
                        .type_ref
                        .as_ref()
                        .and_then(|w| w.upgrade())
                        .and_then(|t| t.read().ok().map(|t| t.guid.clone()));
                    let child_type_guid = child_piece
                        .type_ref
                        .as_ref()
                        .and_then(|w| w.upgrade())
                        .and_then(|t| t.read().ok().map(|t| t.guid.clone()));
                    let (Some(ptg), Some(ctg)) = (parent_type_guid, child_type_guid) else { continue };
                    let Some(parent_type) = types_by_guid.get(&ptg) else { continue };
                    let Some(child_type) = types_by_guid.get(&ctg) else { continue };
                    let Ok(pt) = parent_type.read() else { continue };
                    let Ok(ct) = child_type.read() else { continue };
                    let Some(pc) = resolve_connector_for_side(&ps, &pt) else { continue };
                    let Some(cc) = resolve_connector_for_side(&cs, &ct) else { continue };
                    let Ok(pcr) = pc.read() else { continue };
                    let Ok(ccr) = cc.read() else { continue };
                    let child_plane = conn.compute_child_plane_for_flatten(&current_plane, &pcr, &ccr);
                    let parent_center = centers.get(&current).copied().unwrap_or_default();
                    let child_center = conn.compute_child_center_for_flatten(parent_center, &pcr);
                    piece_planes.insert(child_id.clone(), child_plane);
                    centers.insert(child_id.clone(), child_center);
                    visited.insert(child_id.clone());
                    q.push_back(child_id);
                }
            }
        }
        piece_planes
            .into_iter()
            .filter_map(|(g, pl)| centers.get(&g).map(|c| (g, (pl, *c))))
            .collect()
    }

    pub fn hash(&self) -> String {
        self.hash_cache.get_or_init(|| {
            let mut w = HashWriter::new();
            self.hash_into(&mut w);
            w.finalize()
        })
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
        for s in &self.stats {
            if let Ok(s) = s.read() {
                s.hash_into(w);
            }
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

    /// Remove pieces (and connections touching them). When `invalidate` is false, caller must
    /// finish with [`Self::invalidate_hash_local`], [`Self::invalidate_flatten`], and kit-level
    /// validation invalidation (see [`KitStore::apply_design_diff`]).
    pub fn delete_pieces(&mut self, piece_guids: &[Guid]) -> usize {
        self.delete_pieces_inner(piece_guids, true)
    }

    pub(crate) fn delete_pieces_inner(&mut self, piece_guids: &[Guid], invalidate: bool) -> usize {
        let parent = self.entity_ref();
        for g in piece_guids {
            self.emit_ev(KitEvent::ChildRemoved {
                parent: parent.clone(),
                child: EntityRef::new(EntityKind::Piece, g.clone()),
            });
        }
        let before = self.pieces.len();
        self.pieces.retain(|p| {
            p.read()
                .map(|p| !piece_guids.iter().any(|g| *g == p.guid))
                .unwrap_or(true)
        });
        self.connections.retain(|c| {
            if let Ok(c) = c.read() {
                let touches = |s: &SideStoreRef| -> bool {
                    s.read()
                        .ok()
                        .and_then(|side| {
                            side.piece
                                .upgrade()
                                .and_then(|p| p.read().ok().map(|p| piece_guids.contains(&p.guid)))
                        })
                        .unwrap_or(false)
                };
                !(touches(&c.connected) || touches(&c.connecting))
            } else {
                true
            }
        });
        if invalidate {
            self.invalidate_hash();
            self.invalidate_flatten();
            self.invalidate_validation();
        }
        before - self.pieces.len()
    }

    pub fn diff_from(&self, other: &DesignStore) -> crate::diff::DesignDiff {
        crate::diff::DesignDiff::between(&self.to_full_dto(), &other.to_full_dto())
    }

    pub fn apply_diff(
        &mut self,
        diff: &crate::diff::DesignDiff,
        type_index: &HashMap<Guid, TypeStoreRef>,
        design_weak: DesignStoreWeak,
    ) -> crate::error::Result<()> {
        let parent = self.entity_ref();
        for id in &diff.removed_connections {
            self.emit_ev(KitEvent::ChildRemoved {
                parent: parent.clone(),
                child: EntityRef::new(EntityKind::Connection, id.guid.clone()),
            });
        }
        for id in &diff.removed_connections {
            self.connections
                .retain(|c| c.read().map(|c| c.guid != id.guid).unwrap_or(true));
        }
        let removed_piece_guids: Vec<Guid> = diff.removed_pieces.iter().map(|p| p.guid.clone()).collect();
        if !removed_piece_guids.is_empty() {
            self.delete_pieces_inner(&removed_piece_guids, false);
        }
        for p in &diff.added_pieces {
            let pref = Arc::new(RwLock::new(PieceStore::empty_shell(p.guid.clone())));
            if let Ok(mut pw) = pref.write() {
                pw.apply_full_dto(p.clone(), design_weak.clone(), type_index);
            }
            self.emit_ev(KitEvent::ChildAdded {
                parent: parent.clone(),
                child: EntityRef::new(EntityKind::Piece, p.guid.clone()),
            });
            self.pieces.push(pref);
        }
        for p in &diff.modified_pieces {
            if let Some(pref) = self.piece(p.guid.as_str()) {
                if let Ok(mut pw) = pref.write() {
                    pw.apply_full_dto(p.clone(), design_weak.clone(), type_index);
                }
            }
        }
        let mut piece_index: HashMap<Guid, PieceStoreRef> = HashMap::new();
        for p in &self.pieces {
            if let Ok(pr) = p.read() {
                piece_index.insert(pr.guid.clone(), p.clone());
            }
        }
        for c in &diff.added_connections {
            self.connections
                .push(connection_from_full_dto(c.clone(), &piece_index, design_weak.clone()));
            self.emit_ev(KitEvent::ChildAdded {
                parent: parent.clone(),
                child: EntityRef::new(EntityKind::Connection, c.guid.clone()),
            });
        }
        for c in &diff.modified_connections {
            self.emit_ev(KitEvent::ChildRemoved {
                parent: parent.clone(),
                child: EntityRef::new(EntityKind::Connection, c.guid.clone()),
            });
            self.connections.retain(|x| x.read().map(|x| x.guid != c.guid).unwrap_or(true));
            self.connections
                .push(connection_from_full_dto(c.clone(), &piece_index, design_weak.clone()));
            self.emit_ev(KitEvent::ChildAdded {
                parent: parent.clone(),
                child: EntityRef::new(EntityKind::Connection, c.guid.clone()),
            });
        }
        // Do not bubble hash/validation to kit here: [`KitStore::apply_design_diff`] may hold the
        // kit write lock. Flatten events are design-local.
        self.invalidate_hash_local();
        self.invalidate_flatten();
        Ok(())
    }

    pub fn invert_change(change: &crate::diff::DesignChange) -> crate::diff::DesignChange {
        crate::diff::DesignChange {
            forward: change.backward.clone(),
            backward: change.forward.clone(),
            author: change.author.clone(),
            time: change.time.clone(),
            before: change.after.clone(),
            after: change.before.clone(),
        }
    }

    pub fn validate_change(&self, change: &crate::diff::DesignChange) -> crate::report::ValidationResult {
        let mut r = crate::report::ValidationResult::valid();
        if change.before.as_ref().map(|b| b.guid.clone()) != change.after.as_ref().map(|a| a.guid.clone()) {
            r.is_valid = false;
            r.errors
                .push("DesignChange before/after snapshots must refer to the same design guid".into());
        }
        r
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
            stats: self.stats.iter().filter_map(|s| s.read().ok().map(|s| s.to_shallow_dto())).collect(),
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
            stats: self.stats.iter().filter_map(|s| s.read().ok().map(|s| s.to_full_dto())).collect(),
        }
    }

    /// Rebuild design graph from DTO (pieces, connections with [`SideStore`] ends, nested leaves).
    /// Only [`crate::kit::KitStore::from_full_dto`] should construct designs in host code.
    pub(crate) fn hydrate_from_full_dto(d: DesignFullDto, type_index: &HashMap<Guid, TypeStoreRef>) -> DesignStoreRef {
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
            kit,
            pieces: piece_dtos,
            connections: connection_dtos,
            layers: layer_dtos,
            groups: group_dtos,
            authors: author_dtos,
            concepts: concept_dtos,
            tags: tag_dtos,
            qualities: quality_dtos,
            props: prop_dtos,
            attributes: attribute_dtos,
            stats: stat_dtos,
        } = d;

        let design = Arc::new(RwLock::new(DesignStore::empty_shell(guid.clone(), name.clone())));
        {
            let mut dw = design.write().expect("design write");
            dw.apply_metadata_fields(DesignMetadataDto {
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
                kit,
            });
        }

        let dw = Arc::downgrade(&design);

        let piece_guids: Vec<Guid> = piece_dtos.iter().map(|p| p.guid.clone()).collect();
        let mut piece_index: HashMap<Guid, PieceStoreRef> = HashMap::new();
        for pd in &piece_dtos {
            piece_index.insert(pd.guid.clone(), Arc::new(RwLock::new(PieceStore::empty_shell(pd.guid.clone()))));
        }
        for pdto in piece_dtos {
            if let Some(p) = piece_index.get(&pdto.guid) {
                if let Ok(mut pw) = p.write() {
                    pw.apply_full_dto(pdto, dw.clone(), type_index);
                }
            }
        }

        let pieces_ordered: Vec<PieceStoreRef> = piece_guids.into_iter().filter_map(|g| piece_index.remove(&g)).collect();

        let layers: Vec<LayerStoreRef> = layer_dtos
            .into_iter()
            .map(|ldto| {
                let mut layer = LayerStore::from_full_dto(ldto);
                layer.parent_design = dw.clone();
                Arc::new(RwLock::new(layer))
            })
            .collect();

        let groups: Vec<GroupStoreRef> = group_dtos
            .into_iter()
            .map(|gdto| {
                let mut g = GroupStore::from_full_dto(gdto);
                g.parent_design = dw.clone();
                Arc::new(RwLock::new(g))
            })
            .collect();

        let authors: Vec<AuthorStoreRef> = author_dtos
            .into_iter()
            .map(|a| {
                let mut s = AuthorStore::from_full_dto(a);
                s.parent_design = Some(dw.clone());
                Arc::new(RwLock::new(s))
            })
            .collect();

        let concepts: Vec<ConceptStoreRef> = concept_dtos
            .into_iter()
            .map(|c| {
                let mut s = ConceptStore::from_full_dto(c);
                s.parent_design = Some(dw.clone());
                Arc::new(RwLock::new(s))
            })
            .collect();

        let tags: Vec<TagStoreRef> = tag_dtos
            .into_iter()
            .map(|t| {
                let mut s = TagStore::from_full_dto(t);
                s.parent_design = Some(dw.clone());
                Arc::new(RwLock::new(s))
            })
            .collect();

        let qualities: Vec<QualityStoreRef> = quality_dtos
            .into_iter()
            .map(|q| {
                let mut s = QualityStore::from_full_dto(q);
                s.parent_design = Some(dw.clone());
                Arc::new(RwLock::new(s))
            })
            .collect();

        let props: Vec<PropStoreRef> = prop_dtos
            .into_iter()
            .map(|p| {
                let mut s = PropStore::from_full_dto(p);
                s.parent_design = Some(dw.clone());
                Arc::new(RwLock::new(s))
            })
            .collect();

        let attributes: Vec<AttributeStoreRef> = attribute_dtos
            .into_iter()
            .map(|a| {
                let mut s = AttributeStore::from_full_dto(a);
                s.parent_design = Some(dw.clone());
                Arc::new(RwLock::new(s))
            })
            .collect();

        let stats: Vec<StatStoreRef> = stat_dtos
            .into_iter()
            .map(|s| {
                let mut st = StatStore::from_full_dto(s);
                st.parent_design = Some(dw.clone());
                Arc::new(RwLock::new(st))
            })
            .collect();

        let mut piece_index_ordered: HashMap<Guid, PieceStoreRef> = HashMap::new();
        for p in &pieces_ordered {
            if let Ok(pr) = p.read() {
                piece_index_ordered.insert(pr.guid.clone(), p.clone());
            }
        }

        let connections: Vec<ConnectionStoreRef> = connection_dtos
            .into_iter()
            .map(|cdto| connection_from_full_dto(cdto, &piece_index_ordered, dw.clone()))
            .collect();

        {
            let mut dw = design.write().expect("design write");
            dw.pieces = pieces_ordered;
            dw.connections = connections;
            dw.layers = layers;
            dw.groups = groups;
            dw.authors = authors;
            dw.concepts = concepts;
            dw.tags = tags;
            dw.qualities = qualities;
            dw.props = props;
            dw.attributes = attributes;
            dw.stats = stats;
        }

        design
    }
}
