//! 🔺️ `change-masonry-class` diff — sets the document's masonry class; the same value is a `mutation.no-op`.

use super::ChangeMasonryClass;
use crate::{En1996Diff, En1996Snapshot};

pub fn diff(payload: &ChangeMasonryClass, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.masonry_class == payload.new_masonry_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The masonry class already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { masonry_class: Some(payload.new_masonry_class), ..Default::default() })
}
