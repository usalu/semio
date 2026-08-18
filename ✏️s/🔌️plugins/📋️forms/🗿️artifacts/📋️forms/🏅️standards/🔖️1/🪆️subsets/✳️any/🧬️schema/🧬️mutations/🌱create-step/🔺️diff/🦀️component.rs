//! 🔺️ `create-step` — sparse diff construction.

use super::mutation::CreateStep;
use crate::artifacts::forms::schema::diff::FormsStepsDelta;
use crate::artifacts::forms::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_create_step(payload: &CreateStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    if steps.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.step.id), [payload.step.id.clone()]);
    }
    let mut delta = FormsStepsDelta { added: vec![payload.step.clone()], ..Default::default() };
    if let Some(index) = payload.index {
        let mut order: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.step.id.clone());
        delta.reordered = Some(order);
    }
    protocol::MutationOutcome::new(forms_diff_from_delta(delta, base))
}
//#endregion 🔖️Diff
