//! 🔺️ `create-step` — sparse diff construction.

use super::mutation::CreateStep;
use crate::schema::diff::FormsStepsDelta;
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_create_step(payload: &CreateStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    if steps.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.step.id), [payload.step.id.clone()]);
    }
    let at = payload.index.map_or(steps.len(), |index| index.min(steps.len()));
    protocol::MutationOutcome::new(FormsDiff { steps: Some(FormsStepsDelta::insertion(at, payload.step.clone())), ..Default::default() })
}
//#endregion 🔖️Diff
