//! ✏️ `change-foundation-liq-n160` diff: patches `liq_n1_60` of the addressed foundation row.

use crate::change_foundation_liq_n160::ChangeFoundationLiqN160;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta, En1998FoundationPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeFoundationLiqN160, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_liq_n160.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.foundations.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.liq_n1_60 == payload.new_liq_n160 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The foundation already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::modification(&payload.id, En1998FoundationPatch { liq_n1_60: Some(payload.new_liq_n160), ..Default::default() }), ..Default::default() })
}
