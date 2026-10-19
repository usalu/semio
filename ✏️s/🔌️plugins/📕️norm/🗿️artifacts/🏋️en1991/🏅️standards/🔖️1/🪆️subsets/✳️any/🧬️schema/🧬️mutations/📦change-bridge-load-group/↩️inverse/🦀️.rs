//! ↩️ Inverse for `change-bridge-load-group`.
use crate::change_bridge_load_group::ChangeBridgeLoadGroup;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeBridgeLoadGroup, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_load_group::ChangeBridgeLoadGroup> {
    Ok((|| {
    vec![Op::from(ChangeBridgeLoadGroup { new_bridge_load_group: base.bridge_load_group.clone() })]

    })())
}
