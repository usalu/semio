//! ↩️ `change-heated-volume-m3` inverse.

use crate::change_heated_volume_m3::ChangeHeatedVolumeM3;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeHeatedVolumeM3, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_heated_volume_m3::ChangeHeatedVolumeM3> {
    Ok((|| {
    vec![Op::from(ChangeHeatedVolumeM3 { new_heated_volume_m3: base.heated_volume_m3 })]

    })())
}
