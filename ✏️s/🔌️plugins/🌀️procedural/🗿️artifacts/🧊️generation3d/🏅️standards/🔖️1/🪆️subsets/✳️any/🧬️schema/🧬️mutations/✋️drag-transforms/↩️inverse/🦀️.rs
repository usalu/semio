//! ↩️ `drag-transforms` inverse — every addressed translate operator restored to its BASE widget.

use crate::standards::v1::subsets::any::schema::mutations::drag_transforms::DragTransforms;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_transform_inverse, Generation3dMutation, GENERATION3D_TRANSLATE_KINDS};
use crate::Generation3dSnapshot;

/// ↩️ Absolute `update-widget` rows of the base operators.
pub fn inverse(payload: &DragTransforms, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    if [payload.dx, payload.dy, payload.dz] == [0.0; 3] {
        return Vec::new();
    }
    generation3d_transform_inverse(base, &payload.targets, &GENERATION3D_TRANSLATE_KINDS)
}
