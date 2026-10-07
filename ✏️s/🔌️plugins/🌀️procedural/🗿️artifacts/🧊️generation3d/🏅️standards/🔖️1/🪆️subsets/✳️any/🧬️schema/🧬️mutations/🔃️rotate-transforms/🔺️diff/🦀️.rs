//! 🔺️ `rotate-transforms` sparse diff — every addressed rotate operator's axis-angle becomes its BASE rotation followed
//! by the payload rotation (`AxisAngle::then`).

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::rotate_transforms::RotateTransforms;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_number_literal,generation3d_param_number,generation3d_param_vector,generation3d_transform_diff,generation3d_vector_literal,GENERATION3D_ROTATE_KINDS};

use crate::standards::v1::subsets::any::schema::transforms::AxisAngle;
use crate::Generation3dSnapshot;

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
        Some(vec![("axis", generation3d_vector_literal("vector", next.axis)), ("angle", generation3d_number_literal(next.angle))])
    })
}
