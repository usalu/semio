use serde::{Deserialize, Serialize};
use std::sync::Weak;

use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::piece::PieceStoreWeak;
use crate::port::PortStoreWeak;

/// One end of a [`crate::connection::ConnectionStore`].
#[derive(Debug, Clone)]
pub struct SideStore {
    pub guid: Guid,
    pub piece: PieceStoreWeak,
    pub port: Option<PortStoreWeak>,
    /// Optional "design piece" for designs that include other designs.
    pub design_piece: Option<PieceStoreWeak>,
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
    #[serde(flatten)]
    pub meta: SideMetadataDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SideFullDto {
    #[serde(flatten)]
    pub meta: SideMetadataDto,
}

impl SideStore {
    pub fn new(piece: PieceStoreWeak) -> Self {
        Self {
            guid: Guid::new_v7(),
            piece,
            port: None,
            design_piece: None,
        }
    }

    pub fn with_port(piece: PieceStoreWeak, port: PortStoreWeak) -> Self {
        Self {
            guid: Guid::new_v7(),
            piece,
            port: Some(port),
            design_piece: None,
        }
    }

    pub fn from_id_dto(d: SideIdDto) -> Self {
        Self {
            guid: d.guid,
            piece: Weak::new(),
            port: None,
            design_piece: None,
        }
    }

    pub fn from_metadata_dto(d: SideMetadataDto) -> Self {
        Self {
            guid: d.guid,
            piece: Weak::new(),
            port: None,
            design_piece: None,
        }
    }

    pub fn from_shallow_dto(d: SideShallowDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn from_full_dto(d: SideFullDto) -> Self {
        Self::from_metadata_dto(d.meta)
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
        SideShallowDto { meta: self.to_metadata_dto() }
    }

    pub fn to_full_dto(&self) -> SideFullDto {
        SideFullDto { meta: self.to_metadata_dto() }
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
        }
    }
}
