//! ↩️ Inverse of `SetBeam`: an absolute `SetBeam` restoring the base value of exactly the fields the forward really changes, none when the beam is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_beam::SetBeam;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetBeam, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_beam::SetBeam>,
{
    let Some(record) = base.beams.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetBeam::from_patch(payload.id.clone(), restore))]
}
