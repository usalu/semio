use crate::insert_cf_action::InsertCfAction;
use crate::remove_cf_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &InsertCfAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_cf_action::RemoveCfAction> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(remove_cf_action::RemoveCfAction { id: payload.id.clone(), index: payload.index.unwrap_or(usize::MAX).min(row.actions.len()) })],
        None => Vec::new(),
    })
}
