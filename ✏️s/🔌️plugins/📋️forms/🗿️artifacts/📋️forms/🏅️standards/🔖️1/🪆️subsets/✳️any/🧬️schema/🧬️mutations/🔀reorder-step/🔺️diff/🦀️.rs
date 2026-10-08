//! 🔺️ `reorder-step` — sparse diff construction.

use super::mutation::ReorderStep;
use crate::schema::diff::FormsStepsDelta;
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_reorder_step(payload: &ReorderStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(current_index) = steps.iter().position(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let at = payload.to_index.min(steps.len() - 1);
    if at == current_index {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Step \"{}\" is already at index {at}.", payload.id));
    }
    protocol::MutationOutcome::new(FormsDiff { steps: Some(FormsStepsDelta::relocation(&steps, current_index, at)), ..Default::default() })
}
//#endregion 🔖️Diff
