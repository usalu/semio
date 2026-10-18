//! ↩️ Inverse of `CreateLeader`: the concrete `DeleteLeader` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_leader::DeleteLeader;
use crate::standards::v1::subsets::any::schema::mutations::create_leader::CreateLeader;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateLeader, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_leader::DeleteLeader>,
{
    if base.leaders.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteLeader { id: payload.id.clone() })]
}
