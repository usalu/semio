use crate::change_truss_kind::ChangeTrussKind;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussKind, base: &En1995Snapshot) -> Option<crate::change_truss_kind::ChangeTrussKind> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussKind { truss_id: payload.truss_id.clone(), new_value: item.kind }),
        None => None,
    }
}
