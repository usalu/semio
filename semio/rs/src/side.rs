use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::connection::ConnectionStoreWeak;
use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::piece::PieceStoreWeak;
use crate::port::PortStoreWeak;

pub type SideStoreRef = Arc<RwLock<SideStore>>;
pub type SideStoreWeak = Weak<RwLock<SideStore>>;

/// One end of a [`crate::connection::ConnectionStore`].
#[derive(Debug)]
pub struct SideStore {
    pub guid: Guid,
    pub piece: PieceStoreWeak,
    pub port: Option<PortStoreWeak>,
    /// Optional "design piece" for designs that include other designs.
    pub design_piece: Option<PieceStoreWeak>,
    pub parent_connection: Option<ConnectionStoreWeak>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SideIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SideMetadataDto {
    pub guid: Guid,
    pub piece: crate::piece::PieceIdDto,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<crate::port::PortIdDto>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "designPiece")]
    pub design_piece: Option<crate::piece::PieceIdDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SideShallowDto {
    pub guid: Guid,
    pub piece: crate::piece::PieceIdDto,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<crate::port::PortIdDto>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "designPiece")]
    pub design_piece: Option<crate::piece::PieceIdDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SideFullDto {
    pub guid: Guid,
    pub piece: crate::piece::PieceIdDto,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<crate::port::PortIdDto>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "designPiece")]
    pub design_piece: Option<crate::piece::PieceIdDto>,
}

impl SideStore {
    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
            piece: Weak::new(),
            port: None,
            design_piece: None,
            parent_connection: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Side, self.guid.clone())
    }

    pub(crate) fn apply_metadata_dto(&mut self, d: SideMetadataDto) {
        self.guid = d.guid;
        self.hash_cache.invalidate();
    }

    pub fn set_piece_weak(&mut self, piece: PieceStoreWeak) {
        self.piece = piece;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "piece",
        });
        self.bubble();
    }

    pub fn set_port_weak(&mut self, port: Option<PortStoreWeak>) {
        self.port = port;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "port",
        });
        self.bubble();
    }

    pub fn set_design_piece_weak(&mut self, design_piece: Option<PieceStoreWeak>) {
        self.design_piece = design_piece;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "designPiece",
        });
        self.bubble();
    }

    fn bubble(&mut self) {
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        if let Some(w) = &self.parent_connection {
            if let Some(c) = w.upgrade() {
                if let Ok(c) = c.read() {
                    c.notify_aggregate_change();
                }
            }
        }
    }

    pub fn to_id_dto(&self) -> SideIdDto {
        SideIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> SideMetadataDto {
        let piece_guid = self
            .piece
            .upgrade()
            .and_then(|p| p.read().ok().map(|p| p.guid.clone()))
            .unwrap_or_default();
        let port = self.port.as_ref().and_then(|p| p.upgrade()).and_then(|p| p.read().ok().map(|p| p.to_id_dto()));
        let design_piece = self
            .design_piece
            .as_ref()
            .and_then(|p| p.upgrade())
            .and_then(|p| p.read().ok().map(|p| p.to_id_dto()));
        SideMetadataDto {
            guid: self.guid.clone(),
            piece: crate::piece::PieceIdDto { guid: piece_guid },
            port,
            design_piece,
        }
    }

    pub fn to_shallow_dto(&self) -> SideShallowDto {
        let m = self.to_metadata_dto();
        SideShallowDto {
            guid: m.guid,
            piece: m.piece,
            port: m.port,
            design_piece: m.design_piece,
        }
    }

    pub fn to_full_dto(&self) -> SideFullDto {
        let m = self.to_metadata_dto();
        SideFullDto {
            guid: m.guid,
            piece: m.piece,
            port: m.port,
            design_piece: m.design_piece,
        }
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
    }

    pub fn hash(&self) -> String {
        self.hash_cache.get_or_init(|| {
            let mut w = HashWriter::new();
            self.hash_into(&mut w);
            w.finalize()
        })
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.str(self.guid.as_str());
        if let Some(p) = self.piece.upgrade() {
            if let Ok(p) = p.read() {
                w.str(p.guid.as_str());
            }
        }
        if let Some(p) = self.port.as_ref().and_then(|p| p.upgrade()) {
            if let Ok(p) = p.read() {
                w.str(p.guid.as_str());
            }
        }
        if let Some(p) = self.design_piece.as_ref().and_then(|p| p.upgrade()) {
            if let Ok(p) = p.read() {
                w.str(p.guid.as_str());
            }
        }
    }
}

impl Default for SideStore {
    fn default() -> Self {
        Self {
            guid: Guid::new_v7(),
            piece: Weak::new(),
            port: None,
            design_piece: None,
            parent_connection: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }
}
