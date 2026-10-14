//! ↩️ `change-envelope-volume` inverse.
use crate::change_envelope_volume::ChangeEnvelopeVolume;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(_payload: &ChangeEnvelopeVolume, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_envelope_volume::ChangeEnvelopeVolume> {
    Ok((|| {
    vec![Op::from(ChangeEnvelopeVolume { new_envelope_volume_m3: base.envelope_volume_m3 })]

    })())
}
