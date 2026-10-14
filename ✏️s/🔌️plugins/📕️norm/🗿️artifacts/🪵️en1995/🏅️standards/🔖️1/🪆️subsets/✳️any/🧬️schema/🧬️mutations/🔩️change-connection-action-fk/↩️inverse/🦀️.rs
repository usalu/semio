use crate::change_connection_action_fk::ChangeConnectionActionFK;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeConnectionActionFK, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_connection_action_fk::ChangeConnectionActionFK> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return Vec::new(); };
    vec![Op::from(ChangeConnectionActionFK { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_value: item.f_k_n })]

    })())
}
