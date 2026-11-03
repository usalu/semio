//! ↩️ Inverse for `change-assumed-rail-lm71`.
use crate::change_assumed_rail_lm71::ChangeAssumedRailLm71;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedRailLm71, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_rail_lm71::ChangeAssumedRailLm71> {
    Ok((|| {
    vec![Op::from(ChangeAssumedRailLm71 { new_assumed_rail_lm71: base.assumed_rail_lm71 })]

    })())
}
