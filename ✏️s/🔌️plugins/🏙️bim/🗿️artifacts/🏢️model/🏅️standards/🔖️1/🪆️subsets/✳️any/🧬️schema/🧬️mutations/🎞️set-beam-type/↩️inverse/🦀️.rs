//! ↩️ Inverse of `SetBeamType`: an absolute `SetBeamType` restoring the base value of exactly the fields the forward really changes, none when the beam type is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_beam_type::SetBeamType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetBeamType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_beam_type::SetBeamType>,
{
    let Some(record) = base.beam_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetBeamType::from_patch(payload.id.clone(), restore))]
}
