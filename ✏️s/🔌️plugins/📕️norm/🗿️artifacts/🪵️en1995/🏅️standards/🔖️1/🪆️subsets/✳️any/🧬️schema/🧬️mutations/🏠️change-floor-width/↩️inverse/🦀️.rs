use crate::change_floor_width::ChangeFloorWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorWidth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_width::ChangeFloorWidth> {
    Ok(match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => vec![Op::from(ChangeFloorWidth { floor_id: payload.floor_id.clone(), new_value: item.width_m })],
        None => Vec::new(),
    })
}
