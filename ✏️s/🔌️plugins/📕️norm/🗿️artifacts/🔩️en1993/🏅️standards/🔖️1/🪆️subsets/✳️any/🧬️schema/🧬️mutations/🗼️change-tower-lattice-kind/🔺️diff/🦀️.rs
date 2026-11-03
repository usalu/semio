//! 🔺️ `change-tower-lattice-kind` diff: patches `lattice_kind` of the addressed tower leg row.

use crate::change_tower_lattice_kind::ChangeTowerLatticeKind;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TowerLegDelta, En1993TowerLegPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeTowerLatticeKind, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tower_legs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tower", Vec::<String>::new());
    };
    if row.lattice_kind == payload.new_lattice_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tower leg already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { tower_legs: En1993TowerLegDelta::modification(payload.id.clone(), En1993TowerLegPatch { lattice_kind: Some(payload.new_lattice_kind), ..Default::default() }), ..Default::default() })
}
