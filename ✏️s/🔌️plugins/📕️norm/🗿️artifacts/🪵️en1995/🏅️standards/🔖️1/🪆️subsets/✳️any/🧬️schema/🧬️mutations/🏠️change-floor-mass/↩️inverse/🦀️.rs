use crate::change_floor_mass::ChangeFloorMass;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorMass, base: &En1995Snapshot) -> Option<crate::change_floor_mass::ChangeFloorMass> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorMass { floor_id: payload.floor_id.clone(), new_value: item.mass_kg_per_m2 }),
        None => None,
    }
}
