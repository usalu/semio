//! ↩️ Inverse of `CreateMaterial`: the concrete `DeleteMaterial` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial;
use crate::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateMaterial, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial>,
{
    if base.materials.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteMaterial { id: payload.id.clone() })]
}
