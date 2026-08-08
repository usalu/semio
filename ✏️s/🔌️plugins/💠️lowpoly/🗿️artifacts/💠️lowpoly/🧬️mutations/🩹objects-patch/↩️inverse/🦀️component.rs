//! ↩️ Inverse for `ObjectsPatch`.
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use protocol::{inverse_collection_mutation, CollectionMutation};

use crate::artifacts::lowpoly::LowpolyObjectPatch;

//#region 🔖️Inverse
pub fn inverse(base: &LowpolySnapshot, id: &str, patch: &LowpolyObjectPatch) -> Vec<LowpolyMutation> {
    let inverted = inverse_collection_mutation(&base.objects, &CollectionMutation::Patch { id: id.to_string(), patch: patch.clone() });
    vec![crate::artifacts::lowpoly::mutations::objects_mutation_from_collection(inverted)]
}
//#endregion 🔖️Inverse
