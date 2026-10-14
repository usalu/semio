//! 🔺️ `change-masonry-class` diff — sets the document's masonry class; the same value is a `mutation.no-op`.

use crate::change_masonry_class::ChangeMasonryClass;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeMasonryClass, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.masonry_class == payload.new_masonry_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The masonry class already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { masonry_class: Some(payload.new_masonry_class), ..Default::default() })
}
