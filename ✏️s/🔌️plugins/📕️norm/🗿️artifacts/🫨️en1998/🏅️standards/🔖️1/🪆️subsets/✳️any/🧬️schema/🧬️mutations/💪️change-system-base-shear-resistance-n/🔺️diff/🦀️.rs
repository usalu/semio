//! Diff for `change-system-base-shear-resistance-n`.
use super::ChangeSystemBaseShearResistanceN;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeSystemBaseShearResistanceN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut buildings = base.buildings.clone();
    let Some(b) = buildings.get_mut(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building missing", Vec::<String>::new());
    };
    let Some(sys) = b.systems.get_mut(payload.system_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "system missing", Vec::<String>::new());
    };
    sys.base_shear_resistance_n = payload.new_base_shear_resistance_n;
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
