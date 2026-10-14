//! ↩️ `rotate-transforms` inverse — every addressed rotate operator restored to its BASE widget.

use crate::rotate_transforms::RotateTransforms;
use crate::generation3d_transform_inverse;
use crate::Generation3dMutation;
use crate::GENERATION3D_ROTATE_KINDS;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Absolute `update-widget` rows of the base operators.
pub fn inverse(payload: &RotateTransforms, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
    if payload.angle == 0.0 {
        return Ok(Vec::new());
    }
    generation3d_transform_inverse(base, &payload.targets, &GENERATION3D_ROTATE_KINDS)?

    })
}
