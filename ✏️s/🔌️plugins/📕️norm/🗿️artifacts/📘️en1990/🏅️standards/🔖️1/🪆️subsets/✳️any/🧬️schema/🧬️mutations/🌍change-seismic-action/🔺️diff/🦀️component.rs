//! 🔺️ `change-seismic-action` — sparse diff construction; writes only
//! `En1990Diff.seismic_a_ed_kn`.

use super::mutation::ChangeSeismicAction;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSeismicAction, _base: &En1990Snapshot) -> En1990Diff {
    En1990Diff { seismic_a_ed_kn: Some(payload.new_seismic_a_ed_kn), ..Default::default() }
}
//#endregion 🔖️Diff
