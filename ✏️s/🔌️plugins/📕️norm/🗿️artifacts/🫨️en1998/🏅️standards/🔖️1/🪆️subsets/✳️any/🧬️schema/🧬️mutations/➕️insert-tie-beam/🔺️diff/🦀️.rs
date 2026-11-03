//! ➕️ `insert-tie-beam` diff: inserts the tie_beam row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_tie_beam::InsertTieBeam;
use semio_s_artifact_norm_en1998_diff::{En1998TieBeamDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertTieBeam, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.tie_beams.iter().any(|existing| existing.id == payload.tie_beam.id) {
        let key = payload.tie_beam.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("tie_beam {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.tie_beams.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { tie_beams: En1998TieBeamDelta::insertion(index, payload.tie_beam.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the tie_beam list; inserted at {index}.", payload.index.unwrap_or(index)))
}
