//! ↩️ Inverse of `DeleteFamilySolid`: the concrete `CreateFamilySolid` carrying the full removed record, none when the solid was absent.

use super::super::create_family_solid::CreateFamilySolid;
use super::DeleteFamilySolid;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &DeleteFamilySolid, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.family_solids.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateFamilySolid(CreateFamilySolid { id: payload.id.clone(), solid: record.clone() })],
        None => Vec::new(),
    }
}
