//! ↩️ `change-heated-volume-m3` inverse.

use crate::mutations::change_heated_volume_m3::ChangeHeatedVolumeM3;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeHeatedVolumeM3, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeHeatedVolumeM3(ChangeHeatedVolumeM3 { new_heated_volume_m3: base.heated_volume_m3 })]
}
