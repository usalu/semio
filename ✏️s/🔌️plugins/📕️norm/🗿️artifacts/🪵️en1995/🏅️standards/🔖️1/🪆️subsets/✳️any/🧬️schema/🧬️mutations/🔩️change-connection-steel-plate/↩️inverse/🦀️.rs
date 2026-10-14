use crate::change_connection_steel_plate::ChangeConnectionSteelPlate;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeConnectionSteelPlate, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_connection_steel_plate::ChangeConnectionSteelPlate> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    vec![Op::from(ChangeConnectionSteelPlate { connection_id: payload.connection_id.clone(), new_value: item.steel_plate })]

    })())
}
