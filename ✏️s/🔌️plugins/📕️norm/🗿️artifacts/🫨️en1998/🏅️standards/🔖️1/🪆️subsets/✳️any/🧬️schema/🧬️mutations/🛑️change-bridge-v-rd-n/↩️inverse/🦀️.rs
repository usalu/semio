//! Inverse for `change-bridge-v-rd-n`.
use crate::change_bridge_v_rd_n::ChangeBridgeVRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeVRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_v_rd_n::ChangeBridgeVRdN> {
    Ok((|| {
    match base.bridges.get(payload.index) {
        Some(b) => vec![Op::from(ChangeBridgeVRdN { index: payload.index, new_v_rd_n: b.v_rd_n })],
        None => Vec::new(),
    }

    })())
}
