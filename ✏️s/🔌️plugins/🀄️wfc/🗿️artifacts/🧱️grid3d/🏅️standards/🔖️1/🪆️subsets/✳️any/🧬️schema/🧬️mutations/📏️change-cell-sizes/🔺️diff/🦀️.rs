//! 🔺️ Sparse diff builder for `ChangeCellSizes` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::ChangeCellSizes, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let extent = payload.axis.extent(base) as usize;
    if payload.sizes.len() != extent {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Axis {} needs exactly {extent} cell sizes, got {}.", payload.axis.label(), payload.sizes.len()), [payload.axis.label().to_string()]);
    }
    if payload.sizes.iter().any(|size| !size.is_finite() || *size <= 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Axis {} cell sizes must all be finite and positive.", payload.axis.label()), [payload.axis.label().to_string()]);
    }
    if payload.axis.sizes(base) == &payload.sizes {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Axis {} already carries these cell sizes.", payload.axis.label()));
    }
    let sizes = Some(payload.sizes.clone());
    protocol::MutationOutcome::new(match payload.axis {
        Grid3dAxis::X => Grid3dDiff { cell_sizes_x: sizes, ..Default::default() },
        Grid3dAxis::Y => Grid3dDiff { cell_sizes_y: sizes, ..Default::default() },
        Grid3dAxis::Z => Grid3dDiff { cell_sizes_z: sizes, ..Default::default() },
    })
}
