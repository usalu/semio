//! 🔺️ `change-fire-design-temperature` diff: patches `design_temperature` of the addressed fire exposure row.

use crate::change_fire_design_temperature::ChangeFireDesignTemperature;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FireExposureDelta, En1993FireExposurePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFireDesignTemperature, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_design_temperature.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.fire_exposures.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fire-exposure", Vec::<String>::new());
    };
    if row.design_temperature == payload.new_design_temperature {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fire exposure already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: En1993FireExposureDelta::modification(payload.id.clone(), En1993FireExposurePatch { design_temperature: Some(payload.new_design_temperature), ..Default::default() }), ..Default::default() })
}
