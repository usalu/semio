//! ↩️ `change-envelope-n50` inverse.
use crate::change_envelope_n50::ChangeEnvelopeN50;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(_payload: &ChangeEnvelopeN50, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_envelope_n50::ChangeEnvelopeN50> {
    Ok((|| {
    vec![Op::from(ChangeEnvelopeN50 { new_envelope_n50_h_inv: base.envelope_n50_h_inv })]

    })())
}
