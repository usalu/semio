//! 📄 Writer mutation — `SetDocument` payload + builder + apply.
use crate::artifacts::writer::WriterProjection;
use crate::artifacts::writer::mutations::WriterMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 📄 `SetDocument` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetDocument {
    #[dsl(block)]
    pub document: WriterProjection,
}

pub fn set_document(document: WriterProjection) -> WriterMutation {
    WriterMutation::SetDocument { document }
}

pub fn apply(projection: &mut WriterProjection, document: &WriterProjection) {
    *projection = document.clone();
}
//#endregion 🔖️Mutation
