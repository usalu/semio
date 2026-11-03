use crate::change_truss_restraint_length::ChangeTrussRestraintLength;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTrussRestraintLength, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_truss_restraint_length::ChangeTrussRestraintLength> {
    Ok(match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => vec![Op::from(ChangeTrussRestraintLength { truss_id: payload.truss_id.clone(), new_value: item.restraint_length_m })],
        None => Vec::new(),
    })
}
