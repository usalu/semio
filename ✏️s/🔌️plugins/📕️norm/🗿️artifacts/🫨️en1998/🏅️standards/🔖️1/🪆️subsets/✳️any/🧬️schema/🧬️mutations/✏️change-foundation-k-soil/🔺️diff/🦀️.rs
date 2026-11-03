//! ✏️ `change-foundation-k-soil` diff: patches `k_soil` of the addressed foundation row.

use crate::change_foundation_k_soil::ChangeFoundationKSoil;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta, En1998FoundationPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeFoundationKSoil, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_k_soil.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.foundations.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.k_soil == payload.new_k_soil {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The foundation already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::modification(&payload.id, En1998FoundationPatch { k_soil: Some(payload.new_k_soil), ..Default::default() }), ..Default::default() })
}
