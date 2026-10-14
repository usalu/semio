use crate::change_storeys::ChangeStoreys;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(_payload: &ChangeStoreys, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storeys::ChangeStoreys> {
    Ok((|| {
    vec![Op::from(ChangeStoreys { new_storeys: base.storeys })]

    })())
}
