//! ↩️ `change-materials` inverse.

use crate::mutations::change_materials::ChangeMaterials;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeMaterials, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeMaterials(ChangeMaterials { materials: base.materials.clone() })]
}
