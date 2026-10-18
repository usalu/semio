//! ↩️ Inverse of `CreateComponent`: the concrete `DeleteComponent` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_component::DeleteComponent;
use crate::standards::v1::subsets::any::schema::mutations::create_component::CreateComponent;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateComponent, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_component::DeleteComponent>,
{
    if base.components.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteComponent { id: payload.id.clone() })]
}
