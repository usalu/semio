//! ↩️ Inverse for `change-assumed-rail-lm71-udl`.
use crate::change_assumed_rail_lm71_udl::ChangeAssumedRailLm71Udl;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedRailLm71Udl, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_rail_lm71_udl::ChangeAssumedRailLm71Udl> {
    Ok((|| {
    vec![Op::from(ChangeAssumedRailLm71Udl { new_assumed_rail_lm71_udl: base.assumed_rail_lm71_udl })]

    })())
}
