//! ✏️ `change-bridge-deck-ei-nm2` diff: patches `deck_ei_nm2` of the addressed bridge row.

use crate::change_bridge_deck_ei_nm2::ChangeBridgeDeckEiNm2;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBridgeDeckEiNm2, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_deck_ei_nm2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.deck_ei_nm2 == payload.new_deck_ei_nm2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.id, En1998BridgePatch { deck_ei_nm2: Some(payload.new_deck_ei_nm2), ..Default::default() }), ..Default::default() })
}
