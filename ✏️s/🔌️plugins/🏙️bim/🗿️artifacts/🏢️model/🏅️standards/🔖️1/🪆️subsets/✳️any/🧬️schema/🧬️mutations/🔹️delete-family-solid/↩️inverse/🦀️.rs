//! ↩️ Inverse of `DeleteFamilySolid`: the concrete `CreateFamilySolid` carrying the full removed record, none when the solid was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid;
use crate::standards::v1::subsets::any::schema::mutations::delete_family_solid::DeleteFamilySolid;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteFamilySolid, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid>,
{
    match base.family_solids.get(&payload.id) {
        Some(record) => vec![Op::from(CreateFamilySolid { id: payload.id.clone(), solid: record.clone() })],
        None => Vec::new(),
    }
}
