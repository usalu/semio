//! ↩️ `change-outdoor-co2` inverse.
use super::ChangeOutdoorCo2;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(_payload: &ChangeOutdoorCo2, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeOutdoorCo2(ChangeOutdoorCo2 { new_outdoor_co2_ppm: base.outdoor_co2_ppm })]
}
