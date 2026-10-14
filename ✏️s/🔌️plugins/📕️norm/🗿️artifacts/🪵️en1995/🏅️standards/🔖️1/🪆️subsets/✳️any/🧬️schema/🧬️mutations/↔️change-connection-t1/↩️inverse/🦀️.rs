use crate::change_connection_t1::ChangeConnectionT1;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeConnectionT1, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_connection_t1::ChangeConnectionT1> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    vec![Op::from(ChangeConnectionT1 { connection_id: payload.connection_id.clone(), new_value: item.t1_m })]

    })())
}
