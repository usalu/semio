use crate::change_truss_chord_depth::ChangeTrussChordDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussChordDepth, base: &En1995Snapshot) -> Option<crate::change_truss_chord_depth::ChangeTrussChordDepth> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussChordDepth { truss_id: payload.truss_id.clone(), new_value: item.chord_depth_m }),
        None => None,
    }
}
