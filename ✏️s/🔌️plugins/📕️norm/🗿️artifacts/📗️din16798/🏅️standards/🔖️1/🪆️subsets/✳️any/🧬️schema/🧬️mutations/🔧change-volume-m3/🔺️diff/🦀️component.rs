//! 🔺️ `change-volume-m3` sparse diff construction — writes only `Din16798Diff.volume_m3` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_volume_m3::mutation::ChangeVolumeM3;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeVolumeM3, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { volume_m3: Some(payload.new_volume_m3.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
