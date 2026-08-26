//! ↩️ `replace-geometry-parameters` — undo restores BASE's parameters; missing id ⇒
//! `Vec::new()`.

use super::mutation::ReplaceGeometryParameters;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceGeometryParameters, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let Some(geometry) = base.geometry.get(&payload.id) else {
        return Vec::new();
    };
    vec![Vdi3805Mutation::ReplaceGeometryParameters(ReplaceGeometryParameters { id: payload.id.clone(), new_parameters: geometry.parameters.clone() })]
}
//#endregion 🔖️Inverse
