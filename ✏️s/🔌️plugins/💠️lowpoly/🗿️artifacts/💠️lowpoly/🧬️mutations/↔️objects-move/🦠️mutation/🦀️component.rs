//! ↔️ Lowpoly mutation — `ObjectsMove` payload + builder + apply.
use crate::artifacts::lowpoly::LowpolyProjection;
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};
use protocol::{apply_collection_mutation, CollectionMutation};

//#region 🔖️Mutation
/// @emoji ↔️ `ObjectsMove` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjectsMove {
    pub id: String,
    pub to_index: usize,
}

pub fn objects_move(id: impl Into<String>, to_index: usize) -> LowpolyMutation {
    LowpolyMutation::ObjectsMove { id: id.into(), to_index }
}

pub fn apply(projection: &mut LowpolyProjection, id: &str, to_index: usize) {
    apply_collection_mutation(&mut projection.objects, &CollectionMutation::Move { id: id.to_string(), to_index });
}
//#endregion 🔖️Mutation
