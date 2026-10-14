//! 🕸️ Mesh operations preserve independent source literals and the complete typed managed owner.

use super::CreateMesh;
use crate::delete_mesh;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_mesh::CreateMesh> + From<crate::delete_mesh::DeleteMesh>>(payload: &CreateMesh, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    match &object.mesh {
        Some(existing) => vec![Op::from(CreateMesh::holding(object, existing))],
        None => vec![Op::from(delete_mesh::DeleteMesh { id: payload.id.clone() })],
    }

    })())
}
//#endregion 🔖️Inverse
