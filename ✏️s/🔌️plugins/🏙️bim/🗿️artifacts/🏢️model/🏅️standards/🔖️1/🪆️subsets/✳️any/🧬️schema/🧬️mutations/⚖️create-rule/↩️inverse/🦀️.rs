//! ↩️ Inverse of `CreateRule`: the concrete `DeleteRule` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule;
use crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateRule, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    if base.rules.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteRule { id: payload.id.clone() })]
}
