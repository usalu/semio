//! ↩️ Inverse for `ObjectsMove`.
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &LowpolySnapshot, id: &str, to_index: usize) -> Vec<LowpolyMutation> {
    let inverted = inverse_collection_mutation(&base.objects, &CollectionMutation::Move { id: id.to_string(), to_index });
    vec![crate::artifacts::lowpoly::mutations::objects_mutation_from_collection(inverted)]
}
//#endregion 🔖️Inverse
