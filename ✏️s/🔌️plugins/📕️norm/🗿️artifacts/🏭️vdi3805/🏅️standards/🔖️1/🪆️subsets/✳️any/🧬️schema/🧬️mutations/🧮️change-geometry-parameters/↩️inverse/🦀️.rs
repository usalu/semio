//! ↩️ `change-geometry-parameters` — undo restores BASE's parameters; missing id ⇒
//! `Vec::new()`.

use crate::change_geometry_parameters::ChangeGeometryParameters;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeGeometryParameters, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_geometry_parameters::ChangeGeometryParameters> {
    Ok((|| {
    let Some(geometry) = base.geometry.get(&payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ChangeGeometryParameters { id: payload.id.clone(), new_parameters: geometry.parameters.clone() })]

    })())
}
//#endregion 🔖️Inverse
