//! 🔺️ `change-forming-gain-used-yield` diff: patches `used_yield` of the addressed forming_gain block row.

use crate::change_forming_gain_used_yield::ChangeFormingGainUsedYield;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FormingGainDelta, En1993FormingGainPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFormingGainUsedYield, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_used_yield.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.forming_gains.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "forming-gain", Vec::<String>::new());
    };
    if row.used_yield == payload.new_used_yield {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The forming_gain block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { forming_gains: En1993FormingGainDelta::modification(payload.id.clone(), En1993FormingGainPatch { used_yield: Some(payload.new_used_yield), ..Default::default() }), ..Default::default() })
}
