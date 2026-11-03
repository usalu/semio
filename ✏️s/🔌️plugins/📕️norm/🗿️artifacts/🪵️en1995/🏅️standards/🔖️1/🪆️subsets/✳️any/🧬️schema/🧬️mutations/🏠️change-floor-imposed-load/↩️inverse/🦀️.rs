use crate::change_floor_imposed_load::ChangeFloorImposedLoad;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorImposedLoad, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_imposed_load::ChangeFloorImposedLoad> {
    Ok(match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => vec![Op::from(ChangeFloorImposedLoad { floor_id: payload.floor_id.clone(), new_value: item.imposed_load_pa })],
        None => Vec::new(),
    })
}
