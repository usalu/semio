//! ↩️ Inverse of `CreateSite`: the concrete `DeleteSite` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::create_site::CreateSite;
use crate::standards::v1::subsets::any::schema::mutations::delete_site::DeleteSite;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSite, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_site::DeleteSite>,
{
    if base.sites.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSite { id: payload.id.clone() })]
}
