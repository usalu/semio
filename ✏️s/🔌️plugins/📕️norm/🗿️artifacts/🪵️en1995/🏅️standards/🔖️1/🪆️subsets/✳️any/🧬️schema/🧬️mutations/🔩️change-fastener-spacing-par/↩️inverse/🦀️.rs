use crate::change_fastener_spacing_par::ChangeFastenerSpacingPar;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerSpacingPar, base: &En1995Snapshot) -> Option<crate::change_fastener_spacing_par::ChangeFastenerSpacingPar> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerSpacingPar { fastener_id: payload.fastener_id.clone(), new_value: item.spacing_parallel_m }),
        None => None,
    }
}
