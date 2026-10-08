//! 🔺️ `move-block-to-step` — sparse diff construction: a question reorder inside one step, or
//! a question removal in the source step plus an addition in the destination step.

use super::mutation::MoveBlockToStep;
use crate::schema::diff::{FormsQuestionsDelta, FormsStepPatch, FormsStepsDelta};
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_move_block_to_step(payload: &MoveBlockToStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(source_step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    let Some(current_index) = source_step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist in step \"{}\".", payload.block_id, payload.step_id), [payload.step_id.clone(), payload.block_id.clone()]);
    };
    let block = source_step.blocks[current_index].clone();

    if payload.step_id == payload.to_step_id {
        let at = payload.index.min(source_step.blocks.len() - 1);
        if at == current_index {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" is already at index {at} in step \"{}\".", payload.block_id, payload.step_id));
        }
        let blocks = FormsQuestionsDelta::relocation(&source_step.blocks, current_index, at);
        return protocol::MutationOutcome::new(FormsDiff { steps: Some(FormsStepsDelta::modification(payload.step_id.clone(), FormsStepPatch { blocks: Some(blocks), ..Default::default() })), ..Default::default() });
    }

    let Some(dest_step) = steps.iter().find(|step| step.id == payload.to_step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.to_step_id), [payload.to_step_id.clone()]);
    };
    let leaving = FormsQuestionsDelta::removal(&source_step.blocks, current_index);
    let arriving = FormsQuestionsDelta::insertion(payload.index.min(dest_step.blocks.len()), block);
    let mut steps_delta = FormsStepsDelta::modification(payload.step_id.clone(), FormsStepPatch { blocks: Some(leaving), ..Default::default() });
    steps_delta.absorb(FormsStepsDelta::modification(payload.to_step_id.clone(), FormsStepPatch { blocks: Some(arriving), ..Default::default() }));
    protocol::MutationOutcome::new(FormsDiff { steps: Some(steps_delta), ..Default::default() })
}
//#endregion 🔖️Diff
