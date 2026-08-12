//! ↩️ `replace-object-mesh` — undo restores the base-state mesh JSON; missing id ⇒ `Vec::new()`.

use super::mutation::ReplaceObjectMesh;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceObjectMesh, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![LowpolyMutation::ReplaceObjectMesh(ReplaceObjectMesh { id: payload.id.clone(), new_mesh_json: object.mesh_json.clone() })]
}
//#endregion 🔖️Inverse
