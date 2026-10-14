//! 🔺️ `rotate-transforms` sparse diff — every addressed rotate operator's axis-angle becomes its BASE rotation followed
//! by the payload rotation (`AxisAngle::then`).

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use crate::rotate_transforms::RotateTransforms;
use crate::generation3d_number_value;
use crate::generation3d_param_number;
use crate::generation3d_param_vector;
use crate::generation3d_transform_diff;
use crate::generation3d_vector_value;
use crate::GENERATION3D_ROTATE_KINDS;

use semio_s_artifact_procedural_3d_model::transforms::AxisAngle;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ A non-finite payload or a zero axis is `mutation.invariant` (`axis-nonzero`).
pub fn diff(payload: &RotateTransforms, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let axis = [payload.ax, payload.ay, payload.az];
    if axis.iter().chain([&payload.angle]).any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a rotation must be finite", payload.targets.clone());
    }
    if axis == [0.0; 3] {
        return protocol::MutationOutcome::fatal("mutation.invariant", "axis-nonzero: the rotation axis is the zero vector", payload.targets.clone());
    }
    let delta = AxisAngle { axis, angle: payload.angle };
    generation3d_transform_diff(base, &payload.targets, &GENERATION3D_ROTATE_KINDS, payload.angle == 0.0, |params| {
        let current = AxisAngle { axis: generation3d_param_vector(params, "axis", [0.0, 0.0, 1.0]), angle: generation3d_param_number(params, "angle", 0.0) };
        let next = current.then(delta).ok()?;
        Some(vec![("axis", generation3d_vector_value("vector", next.axis)), ("angle", generation3d_number_value(next.angle))])
    })
}
