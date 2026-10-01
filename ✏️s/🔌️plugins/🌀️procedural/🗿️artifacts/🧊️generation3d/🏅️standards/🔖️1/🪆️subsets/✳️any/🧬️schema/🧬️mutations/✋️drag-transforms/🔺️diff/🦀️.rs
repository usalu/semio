//! 🔺️ `drag-transforms` sparse diff — every addressed translate operator's offset grows by the payload offset, read
//! off its BASE offset.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::drag_transforms::DragTransforms;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_param_vector, generation3d_transform_diff, generation3d_vector_literal, GENERATION3D_TRANSLATE_KINDS};
use crate::Generation3dSnapshot;

/// 🏗️ Composes `offset + (dx, dy, dz)` into each operator; a non-finite payload is `mutation.invariant`.
pub fn diff(payload: &DragTransforms, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let delta = [payload.dx, payload.dy, payload.dz];
    if delta.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    generation3d_transform_diff(base, &payload.targets, &GENERATION3D_TRANSLATE_KINDS, delta == [0.0; 3], |params| {
        let current = generation3d_param_vector(params, "offset", [0.0; 3]);
        let next: [f64; 3] = std::array::from_fn(|axis| current[axis] + delta[axis]);
        next.iter().all(|value| value.is_finite()).then(|| vec![("offset", generation3d_vector_literal("vector", next))])
    })
}
