use crate::change_cutout_face::ChangeCutoutFace;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeCutoutFace, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cutout_face::ChangeCutoutFace> {
    Ok(match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => vec![Op::from(ChangeCutoutFace { cutout_id: payload.cutout_id.clone(), new_value: item.face })],
        None => Vec::new(),
    })
}
