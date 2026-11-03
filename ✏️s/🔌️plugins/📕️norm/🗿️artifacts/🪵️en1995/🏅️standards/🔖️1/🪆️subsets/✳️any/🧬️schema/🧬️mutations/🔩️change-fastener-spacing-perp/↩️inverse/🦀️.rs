use crate::change_fastener_spacing_perp::ChangeFastenerSpacingPerp;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerSpacingPerp, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_spacing_perp::ChangeFastenerSpacingPerp> {
    Ok(match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => vec![Op::from(ChangeFastenerSpacingPerp { fastener_id: payload.fastener_id.clone(), new_value: item.spacing_perpendicular_m })],
        None => Vec::new(),
    })
}
