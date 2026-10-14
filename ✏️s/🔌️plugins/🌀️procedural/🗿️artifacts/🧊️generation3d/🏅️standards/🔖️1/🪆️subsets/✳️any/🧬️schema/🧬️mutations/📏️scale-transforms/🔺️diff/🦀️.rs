//! 🔺️ `scale-transforms` sparse diff — every addressed scale operator's factors become its BASE factors times the payload
//! factors (`compose_scale`), its scaling centre kept at the origin.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use crate::scale_transforms::ScaleTransforms;
use crate::generation3d_param_vector;
use crate::generation3d_transform_diff;
use crate::generation3d_vector_value;
use crate::GENERATION3D_SCALE_KINDS;

use semio_s_artifact_procedural_3d_model::transforms::compose_scale;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ A non-finite or non-positive factor is `mutation.invariant` (the schema's `exclusiveMinimum`).
pub fn diff(payload: &ScaleTransforms, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let factors = [payload.sx, payload.sy, payload.sz];
    if factors.iter().any(|value| !value.is_finite() || *value <= 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "scale factors must be finite and positive", payload.targets.clone());
    }
    generation3d_transform_diff(base, &payload.targets, &GENERATION3D_SCALE_KINDS, factors == [1.0; 3], |params| {
        let next = compose_scale(generation3d_param_vector(params, "factor", [1.0; 3]), factors).ok()?;
        Some(vec![("factor", generation3d_vector_value("vector", next)), ("center", generation3d_vector_value("point", [0.0; 3]))])
    })
}
