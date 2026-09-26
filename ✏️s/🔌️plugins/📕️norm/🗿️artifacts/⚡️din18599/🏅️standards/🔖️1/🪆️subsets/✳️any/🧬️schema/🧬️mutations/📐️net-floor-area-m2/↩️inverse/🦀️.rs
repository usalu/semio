//! ↩️ `change-net-floor-area-m2` inverse.

use crate::mutations::change_net_floor_area_m2::ChangeNetFloorAreaM2;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeNetFloorAreaM2, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeNetFloorAreaM2(ChangeNetFloorAreaM2 { new_net_floor_area_m2: base.net_floor_area_m2 })]
}
