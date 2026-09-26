use super::ChangeDesignSituation;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &ChangeDesignSituation, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = En1996Diff { design_situation: Some(payload.new_design_situation), ..Default::default() };
    protocol::MutationOutcome::new(diff)
}
