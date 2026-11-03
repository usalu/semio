use crate::insert_pile_group::InsertPileGroup;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PileGroupsRows};

pub fn diff(payload: &InsertPileGroup, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.pile_groups.iter().any(|existing| existing.id == payload.pile_group.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("PileGroup id {} already exists.", payload.pile_group.id), [payload.pile_group.id.clone()]);
    }
    if payload.index > base.pile_groups.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.pile_groups.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { pile_groups: Some(En1997PileGroupsRows::insertion(payload.index, payload.pile_group.clone())), ..Default::default() })
}
