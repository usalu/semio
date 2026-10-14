//! ↩️ `change-bridge-sls` inverse.

use crate::change_bridge_sls::ChangeBridgeSls;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeBridgeSls, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_sls::ChangeBridgeSls> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeBridgeSls { new_bridge_sls: base.bridge_sls.clone() })]

    })())
}
