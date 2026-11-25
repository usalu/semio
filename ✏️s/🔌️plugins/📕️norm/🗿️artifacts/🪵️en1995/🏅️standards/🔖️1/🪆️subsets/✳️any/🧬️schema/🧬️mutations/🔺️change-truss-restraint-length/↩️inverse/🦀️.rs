use crate::change_truss_restraint_length::ChangeTrussRestraintLength;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussRestraintLength, base: &En1995Snapshot) -> Option<crate::change_truss_restraint_length::ChangeTrussRestraintLength> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussRestraintLength { truss_id: payload.truss_id.clone(), new_value: item.restraint_length_m }),
        None => None,
    }
}
