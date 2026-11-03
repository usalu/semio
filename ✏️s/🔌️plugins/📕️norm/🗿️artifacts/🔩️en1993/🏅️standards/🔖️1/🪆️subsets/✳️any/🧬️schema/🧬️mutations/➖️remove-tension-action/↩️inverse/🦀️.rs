use crate::remove_tension_action::RemoveTensionAction;
use crate::insert_tension_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &RemoveTensionAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tension_action::InsertTensionAction> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.get(payload.index)) {
        Some(element) => vec![Op::from(insert_tension_action::InsertTensionAction { id: payload.id.clone(), index: Some(payload.index), element: element.clone() })],
        None => Vec::new(),
    })
}
