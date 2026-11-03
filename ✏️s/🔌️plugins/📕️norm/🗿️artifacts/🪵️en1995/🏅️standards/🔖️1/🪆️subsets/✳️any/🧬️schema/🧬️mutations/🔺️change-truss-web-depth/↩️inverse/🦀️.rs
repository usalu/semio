use crate::change_truss_web_depth::ChangeTrussWebDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTrussWebDepth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_truss_web_depth::ChangeTrussWebDepth> {
    Ok(match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => vec![Op::from(ChangeTrussWebDepth { truss_id: payload.truss_id.clone(), new_value: item.web_depth_m })],
        None => Vec::new(),
    })
}
