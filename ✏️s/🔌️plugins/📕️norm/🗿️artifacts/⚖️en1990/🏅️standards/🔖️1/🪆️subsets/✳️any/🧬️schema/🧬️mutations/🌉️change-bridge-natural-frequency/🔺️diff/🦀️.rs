//! 🌉 `change-bridge-natural-frequency` diff: patches `natural_frequency` of the addressed bridge serviceability row.

use crate::change_bridge_natural_frequency::ChangeBridgeNaturalFrequency;
use semio_s_artifact_norm_en1990_diff::{En1990BridgeSlsDelta, En1990BridgeSlsPatch, En1990Diff};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeBridgeNaturalFrequency, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_natural_frequency.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bridge_sls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bridge serviceability {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.natural_frequency == payload.new_natural_frequency {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge serviceability already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { bridge_sls: En1990BridgeSlsDelta::modification(&payload.id, En1990BridgeSlsPatch { natural_frequency: Some(payload.new_natural_frequency), ..Default::default() }), ..Default::default() })
}
