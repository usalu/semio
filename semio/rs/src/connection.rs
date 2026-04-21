use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStoreRef};
use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::connector::ConnectorStore;
use crate::flatten_math::{self, compute_child_center_uv};
use crate::geom::{Coord, Plane};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::side::{SideMetadataDto, SideStore, SideStoreRef};

pub type ConnectionStoreRef = Arc<RwLock<ConnectionStore>>;
pub type ConnectionStoreWeak = Weak<RwLock<ConnectionStore>>;

/// Join between two [`crate::piece::PieceStore`] instances.
#[derive(Debug)]
pub struct ConnectionStore {
    pub guid: Guid,
    pub connected: SideStoreRef,
    pub connecting: SideStoreRef,
    pub gap: Option<f64>,
    pub shift: Option<f64>,
    pub rise: Option<f64>,
    pub rotation: Option<f64>,
    pub turn: Option<f64>,
    pub tilt: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub description: Option<String>,
    pub attributes: Vec<AttributeStoreRef>,
    pub parent_design: Weak<RwLock<crate::design::DesignStore>>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
    child_plane_matrix: Cache<nalgebra::Matrix4<f64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionMetadataDto {
    pub guid: Guid,
    pub connected: SideMetadataDto,
    pub connecting: SideMetadataDto,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rise: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionShallowDto {
    pub guid: Guid,
    pub connected: SideMetadataDto,
    pub connecting: SideMetadataDto,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rise: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionFullDto {
    pub guid: Guid,
    pub connected: SideMetadataDto,
    pub connecting: SideMetadataDto,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rise: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeFullDto>,
}

/// Port-local anchor for a connector (type space).
pub fn connector_anchor_ports(c: &ConnectorStore) -> (Coord, Coord) {
    if let Some(w) = &c.port {
        if let Some(p) = w.upgrade() {
            if let Ok(p) = p.read() {
                let pt = p.point.unwrap_or(Coord::ZERO);
                let dir = p.direction.unwrap_or(Coord::new(0.0, 0.0, 1.0));
                let n = dir.length();
                let d = if n > 1e-10 {
                    Coord::new(dir.x / n, dir.y / n, dir.z / n)
                } else {
                    Coord::new(0.0, 0.0, 1.0)
                };
                return (pt, d);
            }
        }
    }
    (Coord::ZERO, Coord::new(0.0, 0.0, 1.0))
}

impl ConnectionStore {
    pub(crate) fn empty_with_sides(guid: Guid, connected: SideStoreRef, connecting: SideStoreRef) -> Self {
        Self {
            guid,
            connected,
            connecting,
            gap: None,
            shift: None,
            rise: None,
            rotation: None,
            turn: None,
            tilt: None,
            x: None,
            y: None,
            description: None,
            attributes: Vec::new(),
            parent_design: Weak::new(),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
            child_plane_matrix: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Connection, self.guid.clone())
    }

    /// Invalidate this connection and all design-level aggregates (flatten, validation).
    pub(crate) fn notify_aggregate_change(&self) {
        self.hash_cache.invalidate();
        self.child_plane_matrix.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        if let Some(d) = self.parent_design.upgrade() {
            if let Ok(dr) = d.read() {
                dr.invalidate_hash();
                dr.invalidate_flatten();
                dr.invalidate_validation();
            }
        }
    }

    pub(crate) fn apply_metadata_fields(&mut self, d: ConnectionMetadataDto) {
        self.guid = d.guid;
        self.gap = d.gap;
        self.shift = d.shift;
        self.rise = d.rise;
        self.rotation = d.rotation;
        self.turn = d.turn;
        self.tilt = d.tilt;
        self.x = d.x;
        self.y = d.y;
        self.description = d.description;
        self.hash_cache.invalidate();
        self.child_plane_matrix.invalidate();
    }

    pub fn set_gap(&mut self, v: Option<f64>) {
        if self.gap == v {
            return;
        }
        self.gap = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "gap",
        });
        self.bubble();
    }
    pub fn set_shift(&mut self, v: Option<f64>) {
        if self.shift == v {
            return;
        }
        self.shift = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "shift",
        });
        self.bubble();
    }
    pub fn set_rise(&mut self, v: Option<f64>) {
        if self.rise == v {
            return;
        }
        self.rise = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "rise",
        });
        self.bubble();
    }
    pub fn set_rotation(&mut self, v: Option<f64>) {
        if self.rotation == v {
            return;
        }
        self.rotation = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "rotation",
        });
        self.bubble();
    }
    pub fn set_turn(&mut self, v: Option<f64>) {
        if self.turn == v {
            return;
        }
        self.turn = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "turn",
        });
        self.bubble();
    }
    pub fn set_tilt(&mut self, v: Option<f64>) {
        if self.tilt == v {
            return;
        }
        self.tilt = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "tilt",
        });
        self.bubble();
    }
    pub fn set_x(&mut self, v: Option<f64>) {
        if self.x == v {
            return;
        }
        self.x = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "x",
        });
        self.bubble();
    }
    pub fn set_y(&mut self, v: Option<f64>) {
        if self.y == v {
            return;
        }
        self.y = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "y",
        });
        self.bubble();
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
        self.bubble();
    }

    fn bubble(&mut self) {
        self.notify_aggregate_change();
    }

    /// World-space child plane from parent plane and connector geometry (Python `computeChildPlaneDict`).
    pub fn compute_child_plane_for_flatten(
        &self,
        parent_plane: &Plane,
        parent_connector: &ConnectorStore,
        child_connector: &ConnectorStore,
    ) -> Plane {
        let (pp, pd) = connector_anchor_ports(parent_connector);
        let (cp, cd) = connector_anchor_ports(child_connector);
        flatten_math::compute_child_plane(
            parent_plane,
            pp,
            pd,
            cp,
            cd,
            self.gap.unwrap_or(0.0),
            self.shift.unwrap_or(0.0),
            self.rise.unwrap_or(0.0),
            self.rotation.unwrap_or(0.0),
            self.turn.unwrap_or(0.0),
            self.tilt.unwrap_or(0.0),
        )
    }

    /// UV-style center for child piece (Python BFS `child_center`).
    pub fn compute_child_center_for_flatten(
        &self,
        parent_center: Coord,
        parent_connector: &ConnectorStore,
    ) -> Coord {
        let (_, pd) = connector_anchor_ports(parent_connector);
        let connection_u = self.x.unwrap_or(0.0);
        let connection_v = self.y.unwrap_or(0.0);
        let t = match parent_connector.port.as_ref().and_then(|w| w.upgrade()) {
            Some(p) => p.read().ok().and_then(|g| g.t).unwrap_or(0.0),
            None => 0.0,
        };
        compute_child_center_uv(parent_center, connection_u, connection_v, pd.z, t)
    }

    pub fn to_id_dto(&self) -> ConnectionIdDto {
        ConnectionIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> ConnectionMetadataDto {
        ConnectionMetadataDto {
            guid: self.guid.clone(),
            connected: self.connected.read().map(|s| s.to_metadata_dto()).unwrap_or_default(),
            connecting: self.connecting.read().map(|s| s.to_metadata_dto()).unwrap_or_default(),
            gap: self.gap,
            shift: self.shift,
            rise: self.rise,
            rotation: self.rotation,
            turn: self.turn,
            tilt: self.tilt,
            x: self.x,
            y: self.y,
            description: self.description.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> ConnectionShallowDto {
        let m = self.to_metadata_dto();
        ConnectionShallowDto {
            guid: m.guid,
            connected: m.connected,
            connecting: m.connecting,
            gap: m.gap,
            shift: m.shift,
            rise: m.rise,
            rotation: m.rotation,
            turn: m.turn,
            tilt: m.tilt,
            x: m.x,
            y: m.y,
            description: m.description,
            attributes: self
                .attributes
                .iter()
                .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                .collect(),
        }
    }

    pub fn to_full_dto(&self) -> ConnectionFullDto {
        let m = self.to_metadata_dto();
        ConnectionFullDto {
            guid: m.guid,
            connected: m.connected,
            connecting: m.connecting,
            gap: m.gap,
            shift: m.shift,
            rise: m.rise,
            rotation: m.rotation,
            turn: m.turn,
            tilt: m.tilt,
            x: m.x,
            y: m.y,
            description: m.description,
            attributes: self
                .attributes
                .iter()
                .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                .collect(),
        }
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
        self.child_plane_matrix.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
    }

    pub fn hash(&self) -> String {
        self.hash_cache.get_or_init(|| {
            let mut w = HashWriter::new();
            self.hash_into(&mut w);
            w.finalize()
        })
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("connection").str(self.guid.as_str());
        if let Ok(s) = self.connected.read() {
            s.hash_into(w);
        }
        if let Ok(s) = self.connecting.read() {
            s.hash_into(w);
        }
        w.opt_f64(self.gap)
            .opt_f64(self.shift)
            .opt_f64(self.rise)
            .opt_f64(self.rotation)
            .opt_f64(self.turn)
            .opt_f64(self.tilt)
            .opt_f64(self.x)
            .opt_f64(self.y)
            .opt_str(self.description.as_deref());
        for a in &self.attributes {
            if let Ok(a) = a.read() {
                a.hash_into(w);
            }
        }
    }
}

impl Default for ConnectionStore {
    fn default() -> Self {
        let s1 = Arc::new(RwLock::new(SideStore::default()));
        let s2 = Arc::new(RwLock::new(SideStore::default()));
        Self::empty_with_sides(crate::guid::Guid::new_v7(), s1, s2)
    }
}
