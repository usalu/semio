use crate::remove_pile_group::RemovePileGroup;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PileGroupsRows};

pub fn diff(payload: &RemovePileGroup, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.pile_groups.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile-group #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { pile_groups: Some(En1997PileGroupsRows::removal(&base.pile_groups, payload.index)), ..Default::default() })
}
