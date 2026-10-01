//! ↩️ `scale-transforms` inverse — every addressed scale operator restored to its BASE widget.

use crate::standards::v1::subsets::any::schema::mutations::scale_transforms::ScaleTransforms;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_transform_inverse, Generation3dMutation, GENERATION3D_SCALE_KINDS};
use crate::Generation3dSnapshot;

/// ↩️ Absolute `update-widget` rows of the base operators.
pub fn inverse(payload: &ScaleTransforms, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    if [payload.sx, payload.sy, payload.sz] == [1.0; 3] {
        return Vec::new();
    }
    generation3d_transform_inverse(base, &payload.targets, &GENERATION3D_SCALE_KINDS)
}
