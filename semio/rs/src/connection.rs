use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::Attribute;
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::side::{Side, SideDto};

pub type ConnectionRef = Arc<RwLock<Connection>>;
pub type ConnectionWeak = Weak<RwLock<Connection>>;

/// Join between two [`crate::piece::Piece`] instances.
#[derive(Debug)]
pub struct Connection {
    pub guid: Guid,
    pub connected: Side,
    pub connecting: Side,
    pub gap: Option<f64>,
    pub shift: Option<f64>,
    pub rise: Option<f64>,
    pub rotation: Option<f64>,
    pub turn: Option<f64>,
    pub tilt: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub description: Option<String>,
    pub attributes: Vec<Attribute>,
    pub parent_design: Weak<RwLock<crate::design::Design>>,
    hash_cache: OnceLock<String>,
}

impl Connection {
    pub fn new() -> Self {
        Self {
            guid: Guid::new_v7(),
            connected: Side::default(),
            connecting: Side::default(),
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
        w.tag("connection").str(self.guid.as_str());
        self.connected.hash_into(w);
        self.connecting.hash_into(w);
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
            a.hash_into(w);
        }
    }
}

impl Default for Connection {
    fn default() -> Self { Self::new() }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ConnectionDto {
    #[serde(default)]
    pub guid: Option<Guid>,
    pub connected: SideDto,
    pub connecting: SideDto,
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
    pub attributes: Vec<Attribute>,
}

impl From<&Connection> for ConnectionDto {
    fn from(c: &Connection) -> Self {
        let side_to_dto = |s: &Side| -> SideDto {
            let piece_guid = s
                .piece
                .upgrade()
                .and_then(|p| p.read().ok().map(|p| p.guid.clone()))
                .unwrap_or_default();
            let port_guid = s
                .port
                .as_ref()
                .and_then(|p| p.upgrade())
                .and_then(|p| p.read().ok().map(|p| p.guid.clone()));
            let design_piece_guid = s
                .design_piece
                .as_ref()
                .and_then(|p| p.upgrade())
                .and_then(|p| p.read().ok().map(|p| p.guid.clone()));
            SideDto { piece_guid, port_guid, design_piece_guid }
        };
        ConnectionDto {
            guid: Some(c.guid.clone()),
            connected: side_to_dto(&c.connected),
            connecting: side_to_dto(&c.connecting),
            gap: c.gap,
            shift: c.shift,
            rise: c.rise,
            rotation: c.rotation,
            turn: c.turn,
            tilt: c.tilt,
            x: c.x,
            y: c.y,
            description: c.description.clone(),
            attributes: c.attributes.clone(),
        }
    }
}

impl Connection {
    pub fn from_dto(d: ConnectionDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            connected: Side::default(),
            connecting: Side::default(),
            gap: d.gap,
            shift: d.shift,
            rise: d.rise,
            rotation: d.rotation,
            turn: d.turn,
            tilt: d.tilt,
            x: d.x,
            y: d.y,
            description: d.description,
            attributes: d.attributes,
            parent_design: Weak::new(),
            hash_cache: OnceLock::new(),
        }
    }
}
