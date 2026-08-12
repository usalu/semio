//! ↩️ `change-seismic-zone` inverse — restores the pre-change `seismic_zone` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_seismic_zone::mutation::ChangeSeismicZone;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSeismicZone, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeSeismicZone(ChangeSeismicZone { new_seismic_zone: base.seismic_zone.clone() })]
}
//#endregion 🔖️Inverse
