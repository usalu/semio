//! ↩️ Inverse of `CreateClassificationSystem`: the concrete `DeleteClassificationSystem` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::DeleteClassificationSystem;
use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::create_classification_system::CreateClassificationSystem;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateClassificationSystem, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::DeleteClassificationSystem>,
{
    if elements::taken(base, &payload.id).is_some() || payload.id == "entry" {
        return Vec::new();
    }
    vec![Op::from(DeleteClassificationSystem { id: payload.id.clone() })]
}
