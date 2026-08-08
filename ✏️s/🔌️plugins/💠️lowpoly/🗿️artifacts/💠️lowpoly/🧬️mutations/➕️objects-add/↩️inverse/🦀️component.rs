//! ↩️ Inverse for `ObjectsAdd`.
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolyProjection;
use protocol::{inverse_collection_mutation, CollectionMutation};
use crate::artifacts::lowpoly::LowpolyObject;

//#region 🔖️Inverse
pub fn inverse(base: &LowpolyProjection, index: usize, item: &LowpolyObject) -> Vec<LowpolyMutation> {
    let inverted = inverse_collection_mutation(&base.objects, &CollectionMutation::Add { index, item: item.clone() });
    vec![crate::artifacts::lowpoly::mutations::objects_mutation_from_collection(inverted)]
}
//#endregion 🔖️Inverse
