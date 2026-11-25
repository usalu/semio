use crate::change_diaphragm_wall_height::ChangeDiaphragmWallHeight;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmWallHeight, base: &En1995Snapshot) -> Option<crate::change_diaphragm_wall_height::ChangeDiaphragmWallHeight> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmWallHeight { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.wall_height_m }),
        None => None,
    }
}
