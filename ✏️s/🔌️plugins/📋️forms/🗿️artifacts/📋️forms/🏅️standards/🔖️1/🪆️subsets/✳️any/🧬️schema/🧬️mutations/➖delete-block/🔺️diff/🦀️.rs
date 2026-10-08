//! 🔺️ `delete-block` — sparse diff construction: one removed question id in the touched step.

use super::mutation::DeleteBlock;
use crate::schema::diff::{FormsQuestionsDelta, FormsStepPatch, FormsStepsDelta};
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_delete_block(payload: &DeleteBlock, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    let Some(index) = step.blocks.iter().position(|block| block.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist in step \"{}\".", payload.id, payload.step_id), [payload.step_id.clone(), payload.id.clone()]);
    };
    let blocks = FormsQuestionsDelta::removal(&step.blocks, index);
    protocol::MutationOutcome::new(FormsDiff { steps: Some(FormsStepsDelta::modification(payload.step_id.clone(), FormsStepPatch { blocks: Some(blocks), ..Default::default() })), ..Default::default() })
}
//#endregion 🔖️Diff
