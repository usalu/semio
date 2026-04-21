use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::connection::{ConnectionFullDto, ConnectionIdDto};
use crate::design::{DesignFullDto, DesignStore};
use crate::guid::Guid;
use crate::piece::{PieceFullDto, PieceIdDto};
use crate::report::SemioReport;

/// A symmetric description of a modification to a design: forward re-plays
/// the change, backward undoes it. `before`/`after` hold full snapshots for
/// hosts that prefer replacement over patching.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DesignChange {
    pub forward: DesignDiff,
    pub backward: DesignDiff,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<DesignFullDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<DesignFullDto>,
}

/// Structural delta between two [`DesignStore`] states, expressed in DTO shape.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DesignDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "addedPieces")]
    pub added_pieces: Vec<PieceFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "removedPieces")]
    pub removed_pieces: Vec<PieceIdDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "modifiedPieces")]
    pub modified_pieces: Vec<PieceFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "addedConnections")]
    pub added_connections: Vec<ConnectionFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "removedConnections")]
    pub removed_connections: Vec<ConnectionIdDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "modifiedConnections")]
    pub modified_connections: Vec<ConnectionFullDto>,
}

impl DesignDiff {
    /// Structural delta from `before` snapshot to `after` snapshot (DTO-level).
    pub fn between(before: &DesignFullDto, after: &DesignFullDto) -> Self {
        let mut diff = DesignDiff::default();

        let bp: HashMap<Guid, &PieceFullDto> = before.pieces.iter().map(|p| (p.guid.clone(), p)).collect();
        let ap: HashMap<Guid, &PieceFullDto> = after.pieces.iter().map(|p| (p.guid.clone(), p)).collect();
        let kb: HashSet<Guid> = bp.keys().cloned().collect();
        let ka: HashSet<Guid> = ap.keys().cloned().collect();
        for g in kb.difference(&ka) {
            diff.removed_pieces.push(PieceIdDto { guid: g.clone() });
        }
        for g in ka.difference(&kb) {
            diff.added_pieces.push((*ap[g]).clone());
        }
        for g in ka.intersection(&kb) {
            let b = bp[g];
            let a = ap[g];
            if *b != *a {
                diff.modified_pieces.push((*a).clone());
            }
        }

        let bc: HashMap<Guid, &ConnectionFullDto> = before.connections.iter().map(|c| (c.guid.clone(), c)).collect();
        let ac: HashMap<Guid, &ConnectionFullDto> = after.connections.iter().map(|c| (c.guid.clone(), c)).collect();
        let kbc: HashSet<Guid> = bc.keys().cloned().collect();
        let kac: HashSet<Guid> = ac.keys().cloned().collect();
        for g in kbc.difference(&kac) {
            diff.removed_connections.push(ConnectionIdDto { guid: g.clone() });
        }
        for g in kac.difference(&kbc) {
            diff.added_connections.push((*ac[g]).clone());
        }
        for g in kac.intersection(&kbc) {
            let b = bc[g];
            let a = ac[g];
            if *b != *a {
                diff.modified_connections.push((*a).clone());
            }
        }

        diff
    }
}

impl DesignChange {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_before(mut self, design: &DesignStore) -> Self {
        self.before = Some(design.to_full_dto());
        self
    }

    pub fn with_after(mut self, design: &DesignStore) -> Self {
        self.after = Some(design.to_full_dto());
        self
    }
}

impl DesignStore {
    pub fn delete_change(
        &mut self,
        piece_guids: &[Guid],
        connection_guids: &[Guid],
    ) -> SemioReport<DesignChange> {
        let before = self.to_full_dto();

        let mut removed_pieces: Vec<PieceFullDto> = Vec::new();
        let mut removed_connections: Vec<ConnectionFullDto> = Vec::new();
        for pg in piece_guids {
            if let Some(p) = self.piece(pg.as_str()) {
                if let Ok(p) = p.read() {
                    removed_pieces.push(p.to_full_dto());
                }
            }
        }
        for cg in connection_guids {
            if let Some(c) = self.connection(cg.as_str()) {
                if let Ok(c) = c.read() {
                    removed_connections.push(c.to_full_dto());
                }
            }
        }

        self.connections.retain(|c| {
            c.read()
                .map(|c| !connection_guids.iter().any(|g| *g == c.guid))
                .unwrap_or(true)
        });
        let _deleted = self.delete_pieces(piece_guids);

        let after = self.to_full_dto();

        let backward = DesignDiff {
            added_pieces: removed_pieces.clone(),
            added_connections: removed_connections.clone(),
            ..DesignDiff::default()
        };
        let forward = DesignDiff {
            removed_pieces: removed_pieces
                .iter()
                .map(|p| PieceIdDto { guid: p.guid.clone() })
                .collect(),
            removed_connections: removed_connections
                .iter()
                .map(|c| ConnectionIdDto { guid: c.guid.clone() })
                .collect(),
            ..DesignDiff::default()
        };

        let change = DesignChange {
            forward,
            backward,
            author: None,
            time: None,
            before: Some(before),
            after: Some(after),
        };
        SemioReport::ok(change)
    }

    pub fn flatten_change(&self) -> SemioReport<DesignChange> {
        let before = self.to_full_dto();
        let mut modified_pieces: Vec<PieceFullDto> = Vec::new();
        for piece in &self.pieces {
            if let Ok(p) = piece.read() {
                let mut dto = p.to_full_dto();
                dto.plane = Some(p.flat_plane());
                dto.center = Some(p.flat_center());
                modified_pieces.push(dto);
            }
        }
        let forward = DesignDiff { modified_pieces: modified_pieces.clone(), ..DesignDiff::default() };
        let backward_mod: Vec<PieceFullDto> = before
            .pieces
            .iter()
            .filter(|p| modified_pieces.iter().any(|m| m.guid == p.guid))
            .cloned()
            .collect();
        let backward = DesignDiff { modified_pieces: backward_mod, ..DesignDiff::default() };
        SemioReport::ok(DesignChange {
            forward,
            backward,
            author: None,
            time: None,
            before: Some(before.clone()),
            after: Some(before),
        })
    }
}
