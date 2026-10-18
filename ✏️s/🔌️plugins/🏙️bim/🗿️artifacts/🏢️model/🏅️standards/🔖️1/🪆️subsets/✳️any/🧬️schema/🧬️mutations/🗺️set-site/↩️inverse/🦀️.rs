//! ↩️ Inverse of `SetSite`: an absolute `SetSite` restoring the base value of exactly the fields the forward really changes, none when the site is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_site::SetSite;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetSite, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_site::SetSite>,
{
    let Some(record) = base.sites.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetSite::from_patch(payload.id.clone(), restore))]
}
