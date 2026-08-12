//! 🔺️ `reorder-step` — sparse diff construction.

use super::mutation::ReorderStep;
use crate::artifacts::forms::schema::diff::FormsStepsDelta;
use crate::artifacts::forms::{FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_reorder_step(payload: &ReorderStep, base: &FormsSnapshot) -> FormsDiff {
    if !base.steps.iter().any(|step| step.id == payload.id) {
        return FormsDiff::default();
    }
    let mut order: Vec<String> = base.steps.iter().map(|step| step.id.clone()).collect();
    order.retain(|id| id != &payload.id);
    let at = payload.to_index.min(order.len());
    order.insert(at, payload.id.clone());
    FormsDiff { steps: Some(FormsStepsDelta { reordered: Some(order), ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
