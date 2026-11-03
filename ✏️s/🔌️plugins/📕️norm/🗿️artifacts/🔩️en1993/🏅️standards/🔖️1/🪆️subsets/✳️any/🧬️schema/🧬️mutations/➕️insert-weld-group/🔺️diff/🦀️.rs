//! ➕️ `insert-weld-group` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_weld_group::InsertWeldGroup;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993WeldGroupDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertWeldGroup, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.weld_groups.iter().any(|existing| existing.id == payload.weld_group.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Weld group id {} already exists.", payload.weld_group.id), [payload.weld_group.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.weld_groups.len());
    protocol::MutationOutcome::new(En1993Diff { weld_groups: En1993WeldGroupDelta::insertion(index, payload.weld_group.clone()), ..Default::default() })
}
