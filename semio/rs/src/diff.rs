use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::connection::ConnectionDto;
use crate::design::{Design, DesignDto, DesignRef};
use crate::guid::Guid;
use crate::piece::PieceDto;
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
    pub before: Option<DesignDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<DesignDto>,
}

/// Structural delta between two [`Design`] states, expressed in DTO shape.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DesignDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "addedPieces")]
    pub added_pieces: Vec<PieceDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "removedPieces")]
    pub removed_pieces: Vec<Guid>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "modifiedPieces")]
    pub modified_pieces: Vec<PieceDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "addedConnections")]
    pub added_connections: Vec<ConnectionDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "removedConnections")]
    pub removed_connections: Vec<Guid>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "modifiedConnections")]
    pub modified_connections: Vec<ConnectionDto>,
}

impl DesignChange {
    /// A change whose forward/backward are both empty. Convenient starting
    /// point before populating the affected pieces/connections.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Snapshot `design` into [`DesignChange::before`].
    pub fn with_before(mut self, design: &Design) -> Self {
        self.before = Some(DesignDto::from(design));
        self
    }

    /// Snapshot `design` into [`DesignChange::after`].
    pub fn with_after(mut self, design: &Design) -> Self {
        self.after = Some(DesignDto::from(design));
        self
    }
}

impl Design {
    /// Compute a change that removes the given pieces/connections, returning
    /// a report with before/after snapshots and the symmetric diffs.
    pub fn delete_change(
        &mut self,
        piece_guids: &[Guid],
        connection_guids: &[Guid],
    ) -> SemioReport<DesignChange> {
        let before = DesignDto::from(&*self);

        let mut removed_pieces: Vec<PieceDto> = Vec::new();
        let mut removed_connections: Vec<ConnectionDto> = Vec::new();
        for pg in piece_guids {
            if let Some(p) = self.piece(pg.as_str()) {
                if let Ok(p) = p.read() {
                    removed_pieces.push(PieceDto::from(&*p));
                }
            }
        }
        for cg in connection_guids {
            if let Some(c) = self.connection(cg.as_str()) {
                if let Ok(c) = c.read() {
                    removed_connections.push(ConnectionDto::from(&*c));
                }
            }
        }

        self.connections.retain(|c| {
            c.read()
                .map(|c| !connection_guids.iter().any(|g| *g == c.guid))
                .unwrap_or(true)
        });
        let _deleted = self.delete_pieces(piece_guids);

        let after = DesignDto::from(&*self);

        let backward = DesignDiff {
            added_pieces: removed_pieces.clone(),
            added_connections: removed_connections.clone(),
            ..DesignDiff::default()
        };
        let forward = DesignDiff {
            removed_pieces: removed_pieces.iter().filter_map(|p| p.guid.clone()).collect(),
            removed_connections: removed_connections.iter().filter_map(|c| c.guid.clone()).collect(),
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

    /// Compute the flatten-as-change: reports the current flattened poses as
    /// modifications to the baseline (no-op backward).
    pub fn flatten_change(&self) -> SemioReport<DesignChange> {
        let before = DesignDto::from(self);
        let flattened = self.flatten();
        let mut modified_pieces: Vec<PieceDto> = Vec::new();
        for (guid, fp) in &flattened.pieces {
            if let Some(piece) = self.piece(guid.as_str()) {
                if let Ok(p) = piece.read() {
                    let mut dto = PieceDto::from(&*p);
                    dto.plane = Some(fp.plane);
                    dto.center = Some(fp.center);
                    modified_pieces.push(dto);
                }
            }
        }
        let forward = DesignDiff { modified_pieces: modified_pieces.clone(), ..DesignDiff::default() };
        let backward_mod: Vec<PieceDto> = before
            .pieces
            .iter()
            .filter(|p| {
                modified_pieces.iter().any(|m| m.guid == p.guid)
            })
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

impl Design {
    /// Convenience for consumers: run a delete change on a [`DesignRef`]
    /// without exposing the internal locking.
    pub fn delete_pieces_and_connections_ref(
        design: &DesignRef,
        piece_guids: &[Guid],
        connection_guids: &[Guid],
    ) -> SemioReport<DesignChange> {
        match design.write() {
            Ok(mut d) => d.delete_change(piece_guids, connection_guids),
            Err(_) => SemioReport::err("design lock poisoned"),
        }
    }
}

/// Keep an unused import warning silenced on stable rustc.
#[allow(dead_code)]
fn _keep_arc(_: Arc<()>) {}
