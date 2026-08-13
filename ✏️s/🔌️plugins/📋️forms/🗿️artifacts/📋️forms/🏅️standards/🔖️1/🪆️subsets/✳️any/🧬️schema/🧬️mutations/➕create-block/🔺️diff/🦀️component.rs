//! 🔺️ `create-block` — sparse diff construction: clones only the touched step's own `blocks` Vec
//! (bounded, single-step scope — never the whole document).

use super::mutation::CreateBlock;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::schema::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_create_block(payload: &CreateBlock, base: &FormsSnapshot) -> FormsDiff {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return FormsDiff::default();
    };
    if step.blocks.iter().any(|block| block.id == payload.block.id) {
        return FormsDiff::default();
    }
    let mut blocks = step.blocks.clone();
    let at = payload.index.unwrap_or(blocks.len()).min(blocks.len());
    blocks.insert(at, payload.block.clone());
    let patch = FormsStepPatch { blocks: Some(blocks), ..Default::default() };
    forms_diff_from_delta(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.step_id.clone(), patch }], ..Default::default() }, base)
}
//#endregion 🔖️Diff
