//! ↩️ `scale-transforms` inverse — every addressed scale operator restored to its BASE widget.

use crate::scale_transforms::ScaleTransforms;
use crate::generation3d_transform_inverse;
use crate::GENERATION3D_SCALE_KINDS;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Absolute `update-widget` rows of the base operators.
pub fn inverse<Op: From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>>(payload: &ScaleTransforms, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
    if [payload.sx, payload.sy, payload.sz] == [1.0; 3] {
        return Ok(Vec::new());
    }
    generation3d_transform_inverse(base, &payload.targets, &GENERATION3D_SCALE_KINDS)?

    })
}
