//! 🔺️ Diff for `change-assumed-silo-pressure`.
use crate::change_assumed_silo_pressure::ChangeAssumedSiloPressure;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedSiloPressure, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_silo_pressure == payload.new_assumed_silo_pressure {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_silo_pressure: Some(payload.new_assumed_silo_pressure), ..Default::default() })
}
