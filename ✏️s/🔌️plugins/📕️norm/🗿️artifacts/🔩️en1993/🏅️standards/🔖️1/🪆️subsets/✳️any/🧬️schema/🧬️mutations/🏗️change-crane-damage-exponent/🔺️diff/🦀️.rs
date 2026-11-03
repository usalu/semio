//! 🔺️ `change-crane-damage-exponent` diff: patches `damage_exponent` of the addressed crane runway row.

use crate::change_crane_damage_exponent::ChangeCraneDamageExponent;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993CraneRunwayDelta, En1993CraneRunwayPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeCraneDamageExponent, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_damage_exponent.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.crane_runways.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "crane", Vec::<String>::new());
    };
    if row.damage_exponent == payload.new_damage_exponent {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The crane runway already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::modification(payload.id.clone(), En1993CraneRunwayPatch { damage_exponent: Some(payload.new_damage_exponent), ..Default::default() }), ..Default::default() })
}
