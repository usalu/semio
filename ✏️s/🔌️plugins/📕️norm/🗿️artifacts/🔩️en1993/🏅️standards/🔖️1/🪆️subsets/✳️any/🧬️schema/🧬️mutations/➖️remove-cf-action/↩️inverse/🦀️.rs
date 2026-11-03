use crate::remove_cf_action::RemoveCfAction;
use crate::insert_cf_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &RemoveCfAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_cf_action::InsertCfAction> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.get(payload.index)) {
        Some(element) => vec![Op::from(insert_cf_action::InsertCfAction { id: payload.id.clone(), index: Some(payload.index), element: element.clone() })],
        None => Vec::new(),
    })
}
