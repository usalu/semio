//! ➖️ Lowpoly mutation — `ObjectsRemove` payload + builder + apply.
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};
use protocol::{apply_collection_mutation, CollectionMutation};

//#region 🔖️Mutation
/// @emoji ➖️ `ObjectsRemove` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjectsRemove {
    pub id: String,
}

pub fn objects_remove(id: impl Into<String>) -> LowpolyMutation {
    LowpolyMutation::ObjectsRemove { id: id.into() }
}

pub fn apply(projection: &mut LowpolySnapshot, id: &str) {
    apply_collection_mutation(&mut projection.objects, &CollectionMutation::Remove { id: id.to_string() });
}
//#endregion 🔖️Mutation
