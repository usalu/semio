use crate::change_truss_chord_width::ChangeTrussChordWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussChordWidth, base: &En1995Snapshot) -> Option<crate::change_truss_chord_width::ChangeTrussChordWidth> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussChordWidth { truss_id: payload.truss_id.clone(), new_value: item.chord_width_m }),
        None => None,
    }
}
