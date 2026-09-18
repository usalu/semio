//! 🔺️ Sparse diff builder for `UnmaskCell` — one removal from `masked`.

use crate::diff::{cell_id, Grid2dDiff};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::UnmaskCell, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if !base.masked.iter().any(|cell| cell.x == payload.x && cell.y == payload.y) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Cell ({}, {}) is not masked.", payload.x, payload.y), [cell_id(payload.x, payload.y)]);
    }
    protocol::MutationOutcome::new(Grid2dDiff { masked_removed: vec![cell_id(payload.x, payload.y)], ..Default::default() })
}
