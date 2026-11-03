use crate::change_structure_id::ChangeStructureId;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(_payload: &ChangeStructureId, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_structure_id::ChangeStructureId> {
    Ok((|| {
    vec![Op::from(ChangeStructureId { new_structure_id: base.structure_id.clone() })]

    })())
}
