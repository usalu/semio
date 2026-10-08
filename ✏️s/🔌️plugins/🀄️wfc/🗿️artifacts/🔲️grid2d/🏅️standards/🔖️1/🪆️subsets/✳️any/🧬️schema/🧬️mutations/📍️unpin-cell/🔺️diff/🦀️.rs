//! 🔺️ Sparse diff builder for `UnpinCell` — one removal from `pinned`.

use crate::diff::{cell_id, Grid2dDiff, Grid2dPinnedDelta};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::UnpinCell, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if !base.pinned.iter().any(|cell| cell.x == payload.x && cell.y == payload.y) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Cell ({}, {}) is not pinned.", payload.x, payload.y), [cell_id(payload.x, payload.y)]);
    }
    protocol::MutationOutcome::new(Grid2dDiff { pinned: Grid2dPinnedDelta::removal(&base.pinned, base.pinned.iter().position(|row| protocol::list_delta::Keyed::key(row) == cell_id(payload.x, payload.y)).unwrap_or(usize::MAX)), ..Default::default() })
}
