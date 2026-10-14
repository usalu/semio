//! ↩️ `change-net-floor-area-m2` inverse.

use crate::change_net_floor_area_m2::ChangeNetFloorAreaM2;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeNetFloorAreaM2, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_net_floor_area_m2::ChangeNetFloorAreaM2> {
    Ok((|| {
    vec![Op::from(ChangeNetFloorAreaM2 { new_net_floor_area_m2: base.net_floor_area_m2 })]

    })())
}
