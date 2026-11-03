//! ➖️ `remove-weld-group` diff — removes the row at the index.

use crate::remove_weld_group::RemoveWeldGroup;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993WeldGroupDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveWeldGroup, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.weld_groups.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("weld-group index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { weld_groups: En1993WeldGroupDelta::removal(&base.weld_groups, payload.index), ..Default::default() })
}
