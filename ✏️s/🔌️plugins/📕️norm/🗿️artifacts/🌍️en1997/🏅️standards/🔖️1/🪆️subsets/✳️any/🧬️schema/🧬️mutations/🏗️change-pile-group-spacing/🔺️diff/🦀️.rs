//! 🔺️ `change-pile-group-spacing` diff: patches `spacing` of the addressed pile group row.

use crate::change_pile_group_spacing::ChangePileGroupSpacing;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PileGroupsPatch, En1997PileGroupsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePileGroupSpacing, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_spacing.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.pile_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile-group {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.spacing == payload.new_spacing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile group already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { pile_groups: Some(En1997PileGroupsRows::modification(&payload.id, En1997PileGroupsPatch { spacing: Some(payload.new_spacing), ..Default::default() })), ..Default::default() })
}
