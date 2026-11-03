use crate::change_truss_chord_force::ChangeTrussChordForce;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTrussChordForce, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_truss_chord_force::ChangeTrussChordForce> {
    Ok(match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => vec![Op::from(ChangeTrussChordForce { truss_id: payload.truss_id.clone(), new_value: item.chord_force_n })],
        None => Vec::new(),
    })
}
