//! ↩️ Inverse for `ObjectsRemove`.
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolyProjection;
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &LowpolyProjection, id: &str) -> Vec<LowpolyMutation> {
    let inverted = inverse_collection_mutation(&base.objects, &CollectionMutation::Remove { id: id.to_string() });
    vec![crate::artifacts::lowpoly::mutations::objects_mutation_from_collection(inverted)]
}
//#endregion 🔖️Inverse
