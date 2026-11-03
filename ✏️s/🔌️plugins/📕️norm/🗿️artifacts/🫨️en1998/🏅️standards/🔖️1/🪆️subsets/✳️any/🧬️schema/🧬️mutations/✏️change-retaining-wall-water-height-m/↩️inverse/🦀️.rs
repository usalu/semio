//! ↩️ Inverse for `change-retaining-wall-water-height-m`.
use crate::change_retaining_wall_water_height_m::ChangeRetainingWallWaterHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeRetainingWallWaterHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_retaining_wall_water_height_m::ChangeRetainingWallWaterHeightM> {
    Ok(match base.retaining_walls.get(payload.index) {
        Some(row) => vec![Op::from(ChangeRetainingWallWaterHeightM { index: payload.index, new_water_height_m: row.water_height_m })],
        None => Vec::new(),
    })
}
