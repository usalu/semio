//! 🔺️ Sparse diff builder for `ChangeCellSize` — two scalar lanes, refusing a non-positive extent.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::ChangeCellSize, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if !(payload.cell_width.is_finite() && payload.cell_width > 0.0) || !(payload.cell_height.is_finite() && payload.cell_height > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell size must be finite and positive, got {} × {}.", payload.cell_width, payload.cell_height), ["cell-size".to_string()]);
    }
    if base.cell_width == payload.cell_width && base.cell_height == payload.cell_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cell size is already {} × {}.", payload.cell_width, payload.cell_height));
    }
    protocol::MutationOutcome::new(Grid2dDiff { cell_width: Some(payload.cell_width), cell_height: Some(payload.cell_height), ..Default::default() })
}
