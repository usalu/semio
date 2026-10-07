//! 🔺️ `scale-transforms` sparse diff — every addressed scale operator's factors become its BASE factors times the payload
//! factors (`compose_scale`), its scaling centre kept at the origin.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::scale_transforms::ScaleTransforms;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_param_vector,generation3d_transform_diff,generation3d_vector_literal,GENERATION3D_SCALE_KINDS};

use crate::standards::v1::subsets::any::schema::transforms::compose_scale;
use crate::Generation3dSnapshot;

/// 🏗️ A non-finite or non-positive factor is `mutation.invariant` (the schema's `exclusiveMinimum`).
pub fn diff(payload: &ScaleTransforms, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let factors = [payload.sx, payload.sy, payload.sz];
    if factors.iter().any(|value| !value.is_finite() || *value <= 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "scale factors must be finite and positive", payload.targets.clone());
    }
    generation3d_transform_diff(base, &payload.targets, &GENERATION3D_SCALE_KINDS, factors == [1.0; 3], |params| {
        let next = compose_scale(generation3d_param_vector(params, "factor", [1.0; 3]), factors).ok()?;
        Some(vec![("factor", generation3d_vector_literal("vector", next)), ("center", generation3d_vector_literal("point", [0.0; 3]))])
    })
}
