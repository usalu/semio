//! 🔺️ Sparse diff builder for `ResizeGrid` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::ResizeGrid, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if payload.width == 0 || payload.height == 0 || payload.depth == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A grid axis cannot be empty.".to_string(), ["grid".to_string()]);
    }
    if (base.width, base.height, base.depth) == (payload.width, payload.height, payload.depth) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The grid is already {}×{}×{}.", payload.width, payload.height, payload.depth));
    }
    if let Some(cell) = base.pinned.iter().find(|cell| cell.x >= payload.width || cell.y >= payload.height || cell.z >= payload.depth) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pinned cell {} would fall outside the resized grid.", cell_key(cell.x, cell.y, cell.z)), [cell_key(cell.x, cell.y, cell.z)]);
    }
    if let Some(cell) = base.masked.iter().find(|cell| cell.x >= payload.width || cell.y >= payload.height || cell.z >= payload.depth) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Masked cell {} would fall outside the resized grid.", cell_key(cell.x, cell.y, cell.z)), [cell_key(cell.x, cell.y, cell.z)]);
    }
    protocol::MutationOutcome::new(Grid3dDiff {
        width: Some(payload.width),
        height: Some(payload.height),
        depth: Some(payload.depth),
        cell_sizes_x: Some(resized_axis(&base.cell_sizes_x, payload.width)),
        cell_sizes_y: Some(resized_axis(&base.cell_sizes_y, payload.height)),
        cell_sizes_z: Some(resized_axis(&base.cell_sizes_z, payload.depth)),
        ..Default::default()
    })
}
