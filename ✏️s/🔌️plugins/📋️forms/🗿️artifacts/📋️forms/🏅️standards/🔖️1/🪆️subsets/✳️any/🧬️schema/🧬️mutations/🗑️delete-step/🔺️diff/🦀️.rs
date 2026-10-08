//! 🔺️ `delete-step` — sparse diff construction.

use super::mutation::DeleteStep;
use crate::schema::diff::FormsStepsDelta;
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_delete_step(payload: &DeleteStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(index) = steps.iter().position(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(FormsDiff { steps: Some(FormsStepsDelta::removal(&steps, index)), ..Default::default() })
}
//#endregion 🔖️Diff
