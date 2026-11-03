//! ↩️ Inverse for `change-rail-alpha`.
use crate::change_rail_alpha::ChangeRailAlpha;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeRailAlpha, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_alpha::ChangeRailAlpha> {
    Ok((|| {
    vec![Op::from(ChangeRailAlpha { new_rail_alpha: base.rail_alpha })]

    })())
}
