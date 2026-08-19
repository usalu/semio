//! 🔺️ `move-block-to-step` — sparse diff construction: clones only the touched step(s)' own
//! `blocks` Vecs (one patch entry if `step_id == to_step_id`, two otherwise).

use super::mutation::MoveBlockToStep;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub async fn diff_move_block_to_step(payload: &MoveBlockToStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(source_step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    let Some(current_index) = source_step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Block \"{}\" does not exist in step \"{}\".", payload.block_id, payload.step_id),
            [payload.step_id.clone(), payload.block_id.clone()],
        );
    };
    let block = source_step.blocks[current_index].clone();

    if payload.step_id == payload.to_step_id {
        let mut blocks: Vec<_> = source_step.blocks.iter().filter(|b| b.id != payload.block_id).cloned().collect();
        let at = payload.index.min(blocks.len());
        if at == current_index {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" is already at index {at} in step \"{}\".", payload.block_id, payload.step_id));
        }
        blocks.insert(at, block);
        let patch = FormsStepPatch { blocks: Some(blocks), ..Default::default() };
        return protocol::MutationOutcome::new(forms_diff_from_delta(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.step_id.clone(), patch }], ..Default::default() }, base));
    }

    let Some(dest_step) = steps.iter().find(|step| step.id == payload.to_step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.to_step_id), [payload.to_step_id.clone()]);
    };
    let source_blocks: Vec<_> = source_step.blocks.iter().filter(|b| b.id != payload.block_id).cloned().collect();
    let mut dest_blocks = dest_step.blocks.clone();
    let at = payload.index.min(dest_blocks.len());
    dest_blocks.insert(at, block);
    protocol::MutationOutcome::new(forms_diff_from_delta(
        FormsStepsDelta {
            patched: vec![
                FormsStepPatchEntry { id: payload.step_id.clone(), patch: FormsStepPatch { blocks: Some(source_blocks), ..Default::default() } },
                FormsStepPatchEntry { id: payload.to_step_id.clone(), patch: FormsStepPatch { blocks: Some(dest_blocks), ..Default::default() } },
            ],
            ..Default::default()
        },
        base,
    ))
}
//#endregion 🔖️Diff
