use crate::change_floor_mass::ChangeFloorMass;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorMass, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_mass::ChangeFloorMass> {
    Ok(match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => vec![Op::from(ChangeFloorMass { floor_id: payload.floor_id.clone(), new_value: item.mass_kg_per_m2 })],
        None => Vec::new(),
    })
}
