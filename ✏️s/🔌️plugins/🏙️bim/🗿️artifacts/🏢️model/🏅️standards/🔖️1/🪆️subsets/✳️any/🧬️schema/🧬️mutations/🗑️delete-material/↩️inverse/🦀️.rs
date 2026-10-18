//! ↩️ Inverse of `DeleteMaterial`: the concrete `CreateMaterial` carrying the full removed record, none when the material was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial;
use crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteMaterial, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial>,
{
    match base.materials.get(&payload.id) {
        Some(material) => vec![Op::from(CreateMaterial { id: payload.id.clone(), material: material.clone() })],
        None => Vec::new(),
    }
}
