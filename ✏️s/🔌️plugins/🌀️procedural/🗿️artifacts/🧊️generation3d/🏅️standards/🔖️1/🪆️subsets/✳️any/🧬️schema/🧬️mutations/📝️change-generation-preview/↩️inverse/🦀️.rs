//! ↩️ `change-generation-preview` inverse — the preview text the BASE held, restored as an absolute row; a preview that
//! stays put has nothing to undo.

use crate::change_generation_preview::ChangeGenerationPreview;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn inverse<Op: From<ChangeGenerationPreview>>(payload: &ChangeGenerationPreview, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if base.generation.preview_text == payload.text {
        return Ok(Vec::new());
    }
    Ok(vec![Op::from(ChangeGenerationPreview { text: base.generation.preview_text.clone() })])
}
