use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::side::SideStore;

pub type ConnectionStoreRef = Arc<RwLock<ConnectionStore>>;
pub type ConnectionStoreWeak = Weak<RwLock<ConnectionStore>>;

/// Join between two [`crate::piece::PieceStore`] instances.
#[derive(Debug)]
pub struct ConnectionStore {
    pub guid: Guid,
    pub connected: SideStore,
    pub connecting: SideStore,
    pub gap: Option<f64>,
    pub shift: Option<f64>,
    pub rise: Option<f64>,
    pub rotation: Option<f64>,
    pub turn: Option<f64>,
    pub tilt: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub description: Option<String>,
    pub attributes: Vec<AttributeStore>,
    pub parent_design: Weak<RwLock<crate::design::DesignStore>>,
    hash_cache: OnceLock<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionMetadataDto {
    pub guid: Guid,
    pub connected: crate::side::SideMetadataDto,
    pub connecting: crate::side::SideMetadataDto,
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
    pub connected: crate::side::SideMetadataDto,
    pub connecting: crate::side::SideMetadataDto,
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
    pub connected: crate::side::SideMetadataDto,
    pub connecting: crate::side::SideMetadataDto,
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

impl ConnectionStore {
    pub fn new() -> Self {
        Self {
            guid: Guid::new_v7(),
            connected: SideStore::default(),
            connecting: SideStore::default(),
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

    pub fn from_id_dto(d: ConnectionIdDto) -> Self {
        Self {
            guid: d.guid,
            connected: SideStore::default(),
            connecting: SideStore::default(),
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

    pub fn from_metadata_dto(d: ConnectionMetadataDto) -> Self {
        Self {
            guid: d.guid,
            connected: SideStore::from_metadata_dto(d.connected),
            connecting: SideStore::from_metadata_dto(d.connecting),
            gap: d.gap,
            shift: d.shift,
            rise: d.rise,
            rotation: d.rotation,
            turn: d.turn,
            tilt: d.tilt,
            x: d.x,
            y: d.y,
            description: d.description,
            attributes: Vec::new(),
            parent_design: Weak::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: ConnectionShallowDto) -> Self {
        let mut s = Self::from_metadata_dto(ConnectionMetadataDto {
            guid: d.guid,
            connected: d.connected,
            connecting: d.connecting,
            gap: d.gap,
            shift: d.shift,
            rise: d.rise,
            rotation: d.rotation,
            turn: d.turn,
            tilt: d.tilt,
            x: d.x,
            y: d.y,
            description: d.description,
        });
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_shallow_dto).collect();
        s
    }

    pub fn from_full_dto(d: ConnectionFullDto) -> Self {
        let mut s = Self::from_metadata_dto(ConnectionMetadataDto {
            guid: d.guid,
            connected: d.connected,
            connecting: d.connecting,
            gap: d.gap,
            shift: d.shift,
            rise: d.rise,
            rotation: d.rotation,
            turn: d.turn,
            tilt: d.tilt,
            x: d.x,
            y: d.y,
            description: d.description,
        });
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_full_dto).collect();
        s
    }

    pub fn to_id_dto(&self) -> ConnectionIdDto {
        ConnectionIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> ConnectionMetadataDto {
        ConnectionMetadataDto {
            guid: self.guid.clone(),
            connected: self.connected.to_metadata_dto(),
            connecting: self.connecting.to_metadata_dto(),
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
            attributes: self.attributes.iter().map(AttributeStore::to_shallow_dto).collect(),
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

impl Default for ConnectionStore {
    fn default() -> Self {
        Self::new()
    }
}
