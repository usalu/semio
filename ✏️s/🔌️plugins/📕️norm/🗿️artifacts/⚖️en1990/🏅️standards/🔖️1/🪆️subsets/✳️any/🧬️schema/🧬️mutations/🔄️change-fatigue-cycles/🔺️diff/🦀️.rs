//! 🔄 `change-fatigue-cycles` diff: patches `cycles` of the addressed fatigue spectrum row.

use crate::change_fatigue_cycles::ChangeFatigueCycles;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990FatigueSpectrumDelta, En1990FatigueSpectrumPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeFatigueCycles, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_cycles.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.fatigue_spectra.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue spectrum {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.cycles == payload.new_cycles {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue spectrum already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { fatigue_spectra: En1990FatigueSpectrumDelta::modification(&payload.id, En1990FatigueSpectrumPatch { cycles: Some(payload.new_cycles), ..Default::default() }), ..Default::default() })
}
