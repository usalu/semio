//! 🔺️ `move-block-to-step` — sparse diff construction: clones only the touched step(s)' own
//! `blocks` Vecs (one patch entry if `step_id == to_step_id`, two otherwise).

use super::mutation::MoveBlockToStep;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::{FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_move_block_to_step(payload: &MoveBlockToStep, base: &FormsSnapshot) -> FormsDiff {
    let Some(source_step) = base.steps.iter().find(|step| step.id == payload.step_id) else {
        return FormsDiff::default();
    };
    let Some(block) = source_step.blocks.iter().find(|block| block.id == payload.block_id).cloned() else {
        return FormsDiff::default();
    };

    if payload.step_id == payload.to_step_id {
        let mut blocks: Vec<_> = source_step.blocks.iter().filter(|b| b.id != payload.block_id).cloned().collect();
        let at = payload.index.min(blocks.len());
        blocks.insert(at, block);
        let patch = FormsStepPatch { blocks: Some(blocks), ..Default::default() };
        return FormsDiff {
            steps: Some(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.step_id.clone(), patch }], ..Default::default() }),
            ..Default::default()
        };
    }

    let Some(dest_step) = base.steps.iter().find(|step| step.id == payload.to_step_id) else {
        return FormsDiff::default();
    };
    let source_blocks: Vec<_> = source_step.blocks.iter().filter(|b| b.id != payload.block_id).cloned().collect();
    let mut dest_blocks = dest_step.blocks.clone();
    let at = payload.index.min(dest_blocks.len());
    dest_blocks.insert(at, block);
    FormsDiff {
        steps: Some(FormsStepsDelta {
            patched: vec![
                FormsStepPatchEntry { id: payload.step_id.clone(), patch: FormsStepPatch { blocks: Some(source_blocks), ..Default::default() } },
                FormsStepPatchEntry { id: payload.to_step_id.clone(), patch: FormsStepPatch { blocks: Some(dest_blocks), ..Default::default() } },
            ],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
