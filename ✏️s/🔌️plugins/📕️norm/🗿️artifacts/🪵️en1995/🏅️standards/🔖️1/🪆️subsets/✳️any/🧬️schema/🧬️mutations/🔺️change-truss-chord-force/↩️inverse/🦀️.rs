use crate::change_truss_chord_force::ChangeTrussChordForce;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussChordForce, base: &En1995Snapshot) -> Option<crate::change_truss_chord_force::ChangeTrussChordForce> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussChordForce { truss_id: payload.truss_id.clone(), new_value: item.chord_force_n }),
        None => None,
    }
}
