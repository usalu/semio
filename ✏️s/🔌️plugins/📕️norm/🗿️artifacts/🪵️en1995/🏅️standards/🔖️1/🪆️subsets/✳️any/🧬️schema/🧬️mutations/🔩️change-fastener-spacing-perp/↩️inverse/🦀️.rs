use crate::change_fastener_spacing_perp::ChangeFastenerSpacingPerp;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerSpacingPerp, base: &En1995Snapshot) -> Option<crate::change_fastener_spacing_perp::ChangeFastenerSpacingPerp> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerSpacingPerp { fastener_id: payload.fastener_id.clone(), new_value: item.spacing_perpendicular_m }),
        None => None,
    }
}
