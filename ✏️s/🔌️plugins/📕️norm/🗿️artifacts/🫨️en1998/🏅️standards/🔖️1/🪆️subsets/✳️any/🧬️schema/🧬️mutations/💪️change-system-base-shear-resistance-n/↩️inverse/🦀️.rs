//! Inverse for `change-system-base-shear-resistance-n`.
use super::ChangeSystemBaseShearResistanceN;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeSystemBaseShearResistanceN, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.buildings.get(payload.building_index).and_then(|b| b.systems.get(payload.system_index)) {
        Some(sys) => vec![En1998Mutation::ChangeSystemBaseShearResistanceN(ChangeSystemBaseShearResistanceN { building_index: payload.building_index, system_index: payload.system_index, new_base_shear_resistance_n: sys.base_shear_resistance_n })],
        None => Vec::new(),
    }
}
