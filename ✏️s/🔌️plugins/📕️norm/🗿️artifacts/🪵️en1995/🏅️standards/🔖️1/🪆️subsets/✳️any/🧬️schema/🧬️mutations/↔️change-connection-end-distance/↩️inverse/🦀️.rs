use crate::change_connection_end_distance::ChangeConnectionEndDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeConnectionEndDistance, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_connection_end_distance::ChangeConnectionEndDistance> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    vec![Op::from(ChangeConnectionEndDistance { connection_id: payload.connection_id.clone(), new_value: item.end_distance_m })]

    })())
}
