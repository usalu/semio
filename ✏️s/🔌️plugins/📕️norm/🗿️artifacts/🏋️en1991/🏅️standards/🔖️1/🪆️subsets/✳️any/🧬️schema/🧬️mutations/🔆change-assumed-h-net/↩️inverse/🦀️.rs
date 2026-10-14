//! Inverse for `change-assumed-h-net`.
use crate::change_assumed_h_net::ChangeAssumedHNet;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedHNet, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_h_net::ChangeAssumedHNet> {
    Ok((|| {
    vec![Op::from(ChangeAssumedHNet { new_assumed_h_net: base.assumed_h_net })]

    })())
}
