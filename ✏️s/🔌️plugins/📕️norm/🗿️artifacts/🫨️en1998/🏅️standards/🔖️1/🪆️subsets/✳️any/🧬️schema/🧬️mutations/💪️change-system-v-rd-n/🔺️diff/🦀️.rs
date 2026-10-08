//! 💪️ `change-system-v-rd-n` diff — patches the one field of the nested row at the indexes; a missing building or system is a `mutation.target-missing`.

use super::ChangeSystemVRdN;
use crate::diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998SystemDelta, En1998SystemPatch};
use crate::En1998Snapshot;

pub fn diff(payload: &ChangeSystemVRdN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(building) = base.buildings.get(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building missing", Vec::<String>::new());
    };
    let Some(row) = building.systems.get(payload.system_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "system missing", Vec::<String>::new());
    };
    let nested = En1998SystemDelta::modification(&row.id, En1998SystemPatch { base_shear_resistance_n: Some(payload.new_base_shear_resistance_n), ..Default::default() });
    let patch = En1998BuildingPatch { systems: nested, ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&building.id, patch), ..Default::default() })
}
