//! ↩️ Inverse of `SetView`: an absolute `SetView` restoring the base value of exactly the fields the forward really changes, none when the view is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_view::SetView;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetView, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_view::SetView>,
{
    let Some(record) = base.views.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetView::from_patch(payload.id.clone(), restore))]
}
