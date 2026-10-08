//! 🕸️ Mesh operations preserve independent source literals and the complete typed managed owner.

use super::CreateMesh;
use crate::mutations::delete_mesh;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &CreateMesh, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    match &object.mesh {
        Some(existing) => vec![LowpolyMutation::CreateMesh(CreateMesh::holding(object, existing))],
        None => vec![LowpolyMutation::DeleteMesh(delete_mesh::DeleteMesh { id: payload.id.clone() })],
    }

    })())
}
//#endregion 🔖️Inverse
