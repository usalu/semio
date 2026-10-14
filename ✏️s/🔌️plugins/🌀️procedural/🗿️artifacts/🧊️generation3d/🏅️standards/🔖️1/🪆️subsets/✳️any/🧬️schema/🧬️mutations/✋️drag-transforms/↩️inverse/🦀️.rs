//! ↩️ `drag-transforms` inverse — every addressed translate operator restored to its BASE widget.

use crate::drag_transforms::DragTransforms;
use crate::generation3d_transform_inverse;
use crate::GENERATION3D_TRANSLATE_KINDS;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Absolute `update-widget` rows of the base operators.
pub fn inverse<Op: From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>>(payload: &DragTransforms, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
    if [payload.dx, payload.dy, payload.dz] == [0.0; 3] {
        return Ok(Vec::new());
    }
    generation3d_transform_inverse(base, &payload.targets, &GENERATION3D_TRANSLATE_KINDS)?

    })
}
