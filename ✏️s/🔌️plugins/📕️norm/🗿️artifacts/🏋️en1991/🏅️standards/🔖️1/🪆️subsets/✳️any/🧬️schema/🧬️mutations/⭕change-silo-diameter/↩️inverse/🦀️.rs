//! ↩️ Inverse for `change-silo-diameter`.
use crate::change_silo_diameter::ChangeSiloDiameter;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloDiameter, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_diameter::ChangeSiloDiameter> {
    Ok((|| {
    vec![Op::from(ChangeSiloDiameter { new_silo_diameter: base.silo_diameter })]

    })())
}
