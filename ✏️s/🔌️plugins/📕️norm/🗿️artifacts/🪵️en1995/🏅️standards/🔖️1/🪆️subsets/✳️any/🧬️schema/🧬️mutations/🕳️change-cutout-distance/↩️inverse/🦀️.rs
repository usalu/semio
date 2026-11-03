use crate::change_cutout_distance::ChangeCutoutDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeCutoutDistance, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cutout_distance::ChangeCutoutDistance> {
    Ok(match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => vec![Op::from(ChangeCutoutDistance { cutout_id: payload.cutout_id.clone(), new_value: item.distance_m })],
        None => Vec::new(),
    })
}
