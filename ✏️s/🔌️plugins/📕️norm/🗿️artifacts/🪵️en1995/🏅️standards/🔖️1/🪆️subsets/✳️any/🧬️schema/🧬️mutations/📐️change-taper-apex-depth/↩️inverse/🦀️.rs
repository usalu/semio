use crate::change_taper_apex_depth::ChangeTaperApexDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTaperApexDepth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_taper_apex_depth::ChangeTaperApexDepth> {
    Ok(match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => vec![Op::from(ChangeTaperApexDepth { taper_id: payload.taper_id.clone(), new_value: item.apex_depth_m })],
        None => Vec::new(),
    })
}
