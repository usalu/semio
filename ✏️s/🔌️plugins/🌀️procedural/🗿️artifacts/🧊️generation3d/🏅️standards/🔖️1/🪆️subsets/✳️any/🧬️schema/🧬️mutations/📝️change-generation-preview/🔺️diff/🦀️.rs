//! 🔺️ `change-generation-preview` sparse diff — only the play state's preview text moves; the text it already holds is
//! `mutation.no-op`.

use crate::standards::v1::subsets::any::schema::diff::{Generation3dDiff, Generation3dPreviewChange};
use crate::standards::v1::subsets::any::schema::mutations::change_generation_preview::ChangeGenerationPreview;
use crate::Generation3dSnapshot;

pub fn diff(payload: &ChangeGenerationPreview, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if base.generation.preview_text == payload.text {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", "The generation preview is already as requested.");
    }
    protocol::MutationOutcome::new(Generation3dDiff { preview_text: Some(Generation3dPreviewChange { text: payload.text.clone() }), ..Default::default() })
}
