//! 🔺️ `create-step` — sparse diff construction.

use super::mutation::CreateStep;
use crate::schema::diff::forms_diff_from_delta;
use crate::schema::diff::FormsStepsDelta;
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_create_step(payload: &CreateStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    if steps.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.step.id), [payload.step.id.clone()]);
    }
    let reordered = payload.index.filter(|index| *index < steps.len()).map(|index| {
        let mut order: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
        order.insert(index, payload.step.id.clone());
        order
    });
    protocol::MutationOutcome::new(forms_diff_from_delta(&FormsStepsDelta { added: vec![payload.step.clone()], reordered, ..Default::default() }, base))
}
//#endregion 🔖️Diff
