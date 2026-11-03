use crate::change_floor_permanent_load::ChangeFloorPermanentLoad;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorPermanentLoad, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_permanent_load::ChangeFloorPermanentLoad> {
    Ok(match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => vec![Op::from(ChangeFloorPermanentLoad { floor_id: payload.floor_id.clone(), new_value: item.permanent_load_pa })],
        None => Vec::new(),
    })
}
