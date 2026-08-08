//! 🩹 Lowpoly mutation — `ObjectsPatch` payload + builder + apply.
use crate::artifacts::lowpoly::{LowpolyObjectPatch, LowpolyProjection};
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};
use protocol::{apply_collection_mutation, CollectionMutation};

//#region 🔖️Mutation
/// @emoji 🩹 `ObjectsPatch` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjectsPatch {
    pub id: String,
    #[dsl(block)]
    pub patch: LowpolyObjectPatch,
}

pub fn objects_patch(id: impl Into<String>, patch: LowpolyObjectPatch) -> LowpolyMutation {
    LowpolyMutation::ObjectsPatch { id: id.into(), patch }
}

pub fn apply(projection: &mut LowpolyProjection, id: &str, patch: &LowpolyObjectPatch) {
    apply_collection_mutation(&mut projection.objects, &CollectionMutation::Patch { id: id.to_string(), patch: patch.clone() });
}
//#endregion 🔖️Mutation
