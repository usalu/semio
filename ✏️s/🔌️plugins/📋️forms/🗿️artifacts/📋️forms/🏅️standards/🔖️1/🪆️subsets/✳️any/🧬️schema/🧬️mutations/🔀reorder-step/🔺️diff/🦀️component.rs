//! 🔺️ `reorder-step` — sparse diff construction.

use super::mutation::ReorderStep;
use crate::artifacts::forms::schema::diff::FormsStepsDelta;
use crate::artifacts::forms::schema::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_reorder_step(payload: &ReorderStep, base: &FormsSnapshot) -> FormsDiff {
    let steps = forms_steps(base);
    if !steps.iter().any(|step| step.id == payload.id) {
        return FormsDiff::default();
    }
    let mut order: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    order.retain(|id| id != &payload.id);
    let at = payload.to_index.min(order.len());
    order.insert(at, payload.id.clone());
    forms_diff_from_delta(FormsStepsDelta { reordered: Some(order), ..Default::default() }, base)
}
//#endregion 🔖️Diff
