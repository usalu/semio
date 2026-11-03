use crate::change_fastener_spacing_par::ChangeFastenerSpacingPar;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerSpacingPar, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_spacing_par::ChangeFastenerSpacingPar> {
    Ok(match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => vec![Op::from(ChangeFastenerSpacingPar { fastener_id: payload.fastener_id.clone(), new_value: item.spacing_parallel_m })],
        None => Vec::new(),
    })
}
