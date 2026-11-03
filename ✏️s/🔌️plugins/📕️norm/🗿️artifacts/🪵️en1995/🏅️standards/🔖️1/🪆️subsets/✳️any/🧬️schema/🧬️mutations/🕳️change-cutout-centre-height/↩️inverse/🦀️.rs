use crate::change_cutout_centre_height::ChangeCutoutCentreHeight;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeCutoutCentreHeight, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cutout_centre_height::ChangeCutoutCentreHeight> {
    Ok(match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => vec![Op::from(ChangeCutoutCentreHeight { cutout_id: payload.cutout_id.clone(), new_value: item.centre_height_m })],
        None => Vec::new(),
    })
}
