//! 🕸️ Mesh operations preserve independent source literals and the complete typed managed owner.

use super::DeleteMesh;
use crate::create_mesh;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_mesh::CreateMesh>>(payload: &DeleteMesh, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    match &object.mesh {
        Some(existing) => vec![Op::from(create_mesh::CreateMesh::holding(object, existing))],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
