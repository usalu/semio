//! ↩️ Inverse for `change-bridge-span`.
use crate::change_bridge_span::ChangeBridgeSpan;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeBridgeSpan, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_span::ChangeBridgeSpan> {
    Ok((|| {
    vec![Op::from(ChangeBridgeSpan { new_bridge_span: base.bridge_span })]

    })())
}
