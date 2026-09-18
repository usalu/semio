//! 🔺️ Sparse diff builder for `ResizeGrid` — the two extent lanes PLUS the cascade: every pinned
//! and masked cell that falls outside the new extent is removed in the same atomic delta.

use crate::diff::{cell_id, Grid2dDiff};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::ResizeGrid, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if payload.width == 0 || payload.height == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Grid extent must be positive, got {}×{}.", payload.width, payload.height), ["grid".to_string()]);
    }
    if base.width == payload.width && base.height == payload.height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Grid is already {}×{}.", payload.width, payload.height));
    }
    let outside = |x: u32, y: u32| x >= payload.width || y >= payload.height;
    let pinned_removed: Vec<String> = base.pinned.iter().filter(|cell| outside(cell.x, cell.y)).map(|cell| cell_id(cell.x, cell.y)).collect();
    let masked_removed: Vec<String> = base.masked.iter().filter(|cell| outside(cell.x, cell.y)).map(|cell| cell_id(cell.x, cell.y)).collect();
    let dropped = pinned_removed.len() + masked_removed.len();
    let outcome = protocol::MutationOutcome::new(Grid2dDiff { width: Some(payload.width), height: Some(payload.height), pinned_removed, masked_removed, ..Default::default() });
    if dropped == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Resizing to {}×{} also dropped {dropped} cell state(s) outside the new extent.", payload.width, payload.height))
    }
}
