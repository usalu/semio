//! 🔺️ `change-pile-pile-type` diff: patches `pile_type` of the addressed pile row.

use crate::change_pile_pile_type::ChangePilePileType;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePilePileType, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.pile_type == payload.new_pile_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.id, En1997PilesPatch { pile_type: Some(payload.new_pile_type), ..Default::default() })), ..Default::default() })
}
