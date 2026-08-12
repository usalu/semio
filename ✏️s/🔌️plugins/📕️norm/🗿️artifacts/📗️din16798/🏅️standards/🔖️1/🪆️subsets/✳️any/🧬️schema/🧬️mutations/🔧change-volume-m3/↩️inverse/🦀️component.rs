//! ↩️ `change-volume-m3` inverse — restores the pre-change `volume_m3` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_volume_m3::mutation::ChangeVolumeM3;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeVolumeM3, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeVolumeM3(ChangeVolumeM3 { new_volume_m3: base.volume_m3.clone() })]
}
//#endregion 🔖️Inverse
