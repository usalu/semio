//! ↩️ `change-generation-preview` inverse — the preview text the BASE held, restored as an absolute row; a preview that
//! stays put has nothing to undo.

use crate::standards::v1::subsets::any::schema::mutations::change_generation_preview::ChangeGenerationPreview;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

pub fn inverse(payload: &ChangeGenerationPreview, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    if base.generation.preview_text == payload.text {
        return Ok(Vec::new());
    }
    Ok(vec![Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text: base.generation.preview_text.clone() })])
}
