//! 🔺️ `reorder-step` — sparse diff construction.

use super::mutation::ReorderStep;
use crate::artifacts::forms::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::schema::diff::FormsStepsDelta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub async fn diff_reorder_step(payload: &ReorderStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(current_index) = steps.iter().position(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let mut order: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    order.retain(|id| id != &payload.id);
    let at = payload.to_index.min(order.len());
    if at == current_index {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" is already at index {at}.", payload.id));
    }
    order.insert(at, payload.id.clone());
    protocol::MutationOutcome::new(forms_diff_from_delta(FormsStepsDelta { reordered: Some(order), ..Default::default() }, base))
}
//#endregion 🔖️Diff
