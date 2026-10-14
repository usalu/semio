//! Inverse for `change-floor-assumed-qk`.
use crate::change_floor_assumed_qk::ChangeFloorAssumedQk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorAssumedQk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_assumed_qk::ChangeFloorAssumedQk> {
    Ok((|| {
    if payload.index >= base.floors.len() { return Vec::new(); }
    vec![Op::from(ChangeFloorAssumedQk { index: payload.index, new_assumed_qk: base.floors[payload.index].assumed_qk })]

    })())
}
