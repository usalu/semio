use super::ChangeStoreys;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &ChangeStoreys, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = En1996Diff { storeys: Some(payload.new_storeys), ..Default::default() };
    protocol::MutationOutcome::new(diff)
}
