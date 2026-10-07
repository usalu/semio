//! ↩️ `rotate-transforms` inverse — every addressed rotate operator restored to its BASE widget.

use crate::standards::v1::subsets::any::schema::mutations::rotate_transforms::RotateTransforms;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_transform_inverse,Generation3dMutation,GENERATION3D_ROTATE_KINDS};

use crate::Generation3dSnapshot;

/// ↩️ Absolute `update-widget` rows of the base operators.
pub fn inverse(payload: &RotateTransforms, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
    if payload.angle == 0.0 {
        return Ok(Vec::new());
    }
    generation3d_transform_inverse(base, &payload.targets, &GENERATION3D_ROTATE_KINDS)?

    })
}
