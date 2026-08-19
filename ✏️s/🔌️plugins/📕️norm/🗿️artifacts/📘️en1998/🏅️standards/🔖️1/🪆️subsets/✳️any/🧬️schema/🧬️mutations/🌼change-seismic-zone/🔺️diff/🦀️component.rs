//! 🔺️ `change-seismic-zone` sparse diff construction — writes only `En1998Diff.seismic_zone` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_seismic_zone::mutation::ChangeSeismicZone;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSeismicZone, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.seismic_zone == payload.new_seismic_zone {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seismic zone is already {}.", payload.new_seismic_zone));
    }
    protocol::MutationOutcome::new(En1998Diff { seismic_zone: Some(payload.new_seismic_zone.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
