//! 🔺️ `change-generation-preview` sparse diff — only the play state's preview text moves; the text it already holds is
//! `mutation.no-op`.

use crate::standards::v1::subsets::any::schema::diff::{diff_generation_with, Generation3dDiff};
use crate::standards::v1::subsets::any::schema::mutations::change_generation_preview::ChangeGenerationPreview;
use crate::Generation3dSnapshot;

pub fn diff(payload: &ChangeGenerationPreview, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if base.generation.preview_text == payload.text {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", "The generation preview is already as requested.");
    }
    protocol::MutationOutcome::new(diff_generation_with(base, |generation| generation.preview_text.clone_from(&payload.text)))
}
