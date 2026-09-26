//! ↩️ `change-geometry-parameters` — undo restores BASE's parameters; missing id ⇒
//! `Vec::new()`.

use super::ChangeGeometryParameters;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeGeometryParameters, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let Some(geometry) = base.geometry.get(&payload.id) else {
        return Vec::new();
    };
    vec![Vdi3805Mutation::ChangeGeometryParameters(ChangeGeometryParameters { id: payload.id.clone(), new_parameters: geometry.parameters.clone() })]
}
//#endregion 🔖️Inverse
