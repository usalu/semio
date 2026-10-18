//! ↩️ Inverse of `SetTag`: an absolute `SetTag` restoring the base value of exactly the fields the forward really changes, none when the tag is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_tag::SetTag;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetTag, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_tag::SetTag>,
{
    let Some(record) = base.tags.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetTag::from_patch(payload.id.clone(), restore))]
}
