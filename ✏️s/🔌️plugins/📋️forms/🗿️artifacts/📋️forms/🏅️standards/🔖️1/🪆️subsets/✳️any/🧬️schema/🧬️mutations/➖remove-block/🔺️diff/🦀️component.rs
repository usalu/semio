//! 🔺️ `delete-block` — sparse diff construction: clones only the touched step's own `blocks` Vec.

use super::mutation::DeleteBlock;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::{FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_delete_block(payload: &DeleteBlock, base: &FormsSnapshot) -> FormsDiff {
    let Some(step) = base.steps.iter().find(|step| step.id == payload.step_id) else {
        return FormsDiff::default();
    };
    if !step.blocks.iter().any(|block| block.id == payload.id) {
        return FormsDiff::default();
    }
    let blocks: Vec<_> = step.blocks.iter().filter(|block| block.id != payload.id).cloned().collect();
    let patch = FormsStepPatch { blocks: Some(blocks), ..Default::default() };
    FormsDiff {
        steps: Some(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.step_id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
