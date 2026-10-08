//! 🔺️ `delete-block` — sparse diff construction: one removed question id in the touched step.

use super::mutation::DeleteBlock;
use crate::schema::diff::forms_diff_from_delta;
use crate::schema::diff::{FormsQuestionsDelta, FormsStepPatch, FormsStepsDelta};
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_delete_block(payload: &DeleteBlock, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    if !step.blocks.iter().any(|block| block.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist in step \"{}\".", payload.id, payload.step_id), [payload.step_id.clone(), payload.id.clone()]);
    }
    let blocks = FormsQuestionsDelta { removed: vec![payload.id.clone()], ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(&FormsStepsDelta { patched: vec![FormsStepPatch { id: payload.step_id.clone(), blocks: Some(blocks), ..Default::default() }], ..Default::default() }, base))
}
//#endregion 🔖️Diff
