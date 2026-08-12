//! 🔺️ `change-seismic-zone` sparse diff construction — writes only `En1998Diff.seismic_zone` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_seismic_zone::mutation::ChangeSeismicZone;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSeismicZone, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { seismic_zone: Some(payload.new_seismic_zone.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
