//! ✏️ `change-foundation-bearing-n-max-n` diff: patches `bearing_n_max_n` of the addressed foundation row.

use crate::change_foundation_bearing_n_max_n::ChangeFoundationBearingNMaxN;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta, En1998FoundationPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeFoundationBearingNMaxN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_bearing_n_max_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.foundations.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.bearing_n_max_n == payload.new_bearing_n_max_n {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The foundation already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::modification(&payload.id, En1998FoundationPatch { bearing_n_max_n: Some(payload.new_bearing_n_max_n), ..Default::default() }), ..Default::default() })
}
