use serde::{Deserialize, Serialize};
use std::sync::Weak;

use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::piece::PieceWeak;
use crate::port::PortWeak;

/// One end of a [`crate::connection::Connection`].
#[derive(Debug, Clone)]
pub struct Side {
    pub piece: PieceWeak,
    pub port: Option<PortWeak>,
    /// Optional "design piece" for designs that include other designs.
    pub design_piece: Option<PieceWeak>,
}

impl Side {
    pub fn new(piece: PieceWeak) -> Self {
        Self { piece, port: None, design_piece: None }
    }

    pub fn with_port(piece: PieceWeak, port: PortWeak) -> Self {
        Self { piece, port: Some(port), design_piece: None }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
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

impl Default for Side {
    fn default() -> Self {
        Self { piece: Weak::new(), port: None, design_piece: None }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SideDto {
    #[serde(rename = "pieceGuid")]
    pub piece_guid: Guid,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portGuid")]
    pub port_guid: Option<Guid>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "designPieceGuid")]
    pub design_piece_guid: Option<Guid>,
}
