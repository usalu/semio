use super::ChangeAnnex;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &ChangeAnnex, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = En1996Diff { annex: Some(payload.new_annex), ..Default::default() };
    protocol::MutationOutcome::new(diff)
}
