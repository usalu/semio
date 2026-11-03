use crate::remove_crane_action::RemoveCraneAction;
use crate::insert_crane_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &RemoveCraneAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_crane_action::InsertCraneAction> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.get(payload.index)) {
        Some(element) => vec![Op::from(insert_crane_action::InsertCraneAction { id: payload.id.clone(), index: Some(payload.index), element: element.clone() })],
        None => Vec::new(),
    })
}
