//! ↩️ Inverse for `change-steel-fy-pa`.
use crate::change_steel_fy_pa::ChangeSteelFYPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(_payload: &ChangeSteelFYPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_steel_fy_pa::ChangeSteelFYPa> {
    Ok((|| {
    vec![Op::from(ChangeSteelFYPa { new_steel_f_y_pa: base.steel_f_y_pa })]

    })())
}
