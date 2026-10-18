//! ↩️ Inverse of `CreateRamp`: the concrete `DeleteRamp` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_ramp::DeleteRamp;
use crate::standards::v1::subsets::any::schema::mutations::create_ramp::CreateRamp;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateRamp, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_ramp::DeleteRamp>,
{
    if base.ramps.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteRamp { id: payload.id.clone() })]
}
