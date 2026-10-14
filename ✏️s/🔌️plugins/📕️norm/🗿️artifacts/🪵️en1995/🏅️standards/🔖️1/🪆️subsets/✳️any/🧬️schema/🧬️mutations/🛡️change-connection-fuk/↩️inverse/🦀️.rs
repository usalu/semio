use crate::change_connection_fuk::ChangeConnectionFUK;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeConnectionFUK, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_connection_fuk::ChangeConnectionFUK> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    vec![Op::from(ChangeConnectionFUK { connection_id: payload.connection_id.clone(), new_value: item.f_u_k })]

    })())
}
