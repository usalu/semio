//! 💥 `change-accidental-ad` diff: patches `ad` of the addressed accidental action row.

use crate::change_accidental_ad::ChangeAccidentalAd;
use semio_s_artifact_norm_en1990_diff::{En1990AccidentalDelta, En1990AccidentalPatch, En1990Diff};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeAccidentalAd, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_ad.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.accidentals.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Accidental action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.ad == payload.new_ad {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The accidental action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { accidentals: En1990AccidentalDelta::modification(&payload.id, En1990AccidentalPatch { ad: Some(payload.new_ad), ..Default::default() }), ..Default::default() })
}
