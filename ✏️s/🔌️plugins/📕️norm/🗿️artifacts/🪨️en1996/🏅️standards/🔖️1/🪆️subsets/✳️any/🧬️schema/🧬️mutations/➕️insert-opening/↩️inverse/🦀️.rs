use crate::insert_opening::InsertOpening;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use crate::remove_opening;
pub fn inverse<Op>(payload: &InsertOpening, _base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_opening::RemoveOpening> {
    Ok((|| {
    vec![Op::from(remove_opening::RemoveOpening { wall_index: payload.wall_index, index: payload.index })]

    })())
}
