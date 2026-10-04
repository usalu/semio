//! 🕸️ Mesh operations preserve independent source literals and the complete typed managed owner.

use super::DeleteMesh;
use crate::mutations::create_mesh;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMesh, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    match &object.mesh {
        Some(existing) => vec![LowpolyMutation::CreateMesh(create_mesh::CreateMesh { id: payload.id.clone(), child_id: existing.child_id.clone(), target: existing.target.clone(), mesh_workspace: object.mesh_content.clone(), mesh_state:object.mesh_state.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
