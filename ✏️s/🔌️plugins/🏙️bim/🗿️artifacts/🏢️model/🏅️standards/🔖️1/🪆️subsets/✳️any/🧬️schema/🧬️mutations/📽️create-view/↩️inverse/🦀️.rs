//! ↩️ Inverse of `CreateView`: the concrete `DeleteView` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_view::DeleteView;
use crate::standards::v1::subsets::any::schema::mutations::create_view::CreateView;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateView, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_view::DeleteView>,
{
    if base.views.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteView { id: payload.id.clone() })]
}
