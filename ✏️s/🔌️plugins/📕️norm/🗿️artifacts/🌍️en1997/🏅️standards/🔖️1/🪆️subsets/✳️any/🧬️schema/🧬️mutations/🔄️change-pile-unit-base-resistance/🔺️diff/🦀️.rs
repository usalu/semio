//! 🔺️ `change-pile-unit-base-resistance` diff: patches `unit_base_resistance` of the addressed pile row.

use crate::change_pile_unit_base_resistance::ChangePileUnitBaseResistance;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePileUnitBaseResistance, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_unit_base_resistance.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.unit_base_resistance == payload.new_unit_base_resistance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.id, En1997PilesPatch { unit_base_resistance: Some(payload.new_unit_base_resistance), ..Default::default() })), ..Default::default() })
}
