//! ➕️ Lowpoly mutation — `ObjectsAdd` payload + builder + apply.
use crate::artifacts::lowpoly::{LowpolyObject, LowpolySnapshot};
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};
use protocol::{apply_collection_mutation, CollectionMutation};

//#region 🔖️Mutation
/// @emoji ➕️ `ObjectsAdd` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjectsAdd {
    pub index: usize,
    #[dsl(block)]
    pub item: LowpolyObject,
}

pub fn objects_add(index: usize, item: LowpolyObject) -> LowpolyMutation {
    LowpolyMutation::ObjectsAdd { index, item }
}

pub fn apply(projection: &mut LowpolySnapshot, index: usize, item: &LowpolyObject) {
    apply_collection_mutation(&mut projection.objects, &CollectionMutation::Add { index, item: item.clone() });
}
//#endregion 🔖️Mutation
