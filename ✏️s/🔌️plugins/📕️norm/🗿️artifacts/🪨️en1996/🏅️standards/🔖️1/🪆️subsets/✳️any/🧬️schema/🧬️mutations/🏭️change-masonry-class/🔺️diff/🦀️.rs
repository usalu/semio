use super::ChangeMasonryClass;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &ChangeMasonryClass, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = En1996Diff { masonry_class: Some(payload.new_masonry_class), ..Default::default() };
    protocol::MutationOutcome::new(diff)
}
