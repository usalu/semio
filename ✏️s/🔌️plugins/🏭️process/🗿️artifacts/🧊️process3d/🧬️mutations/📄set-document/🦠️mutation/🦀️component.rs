//! 📄 Process3d mutation — `SetDocument`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 📄 `SetDocument` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDocument {
    pub document: Process3dDocument,
}

pub fn set_document(document: Process3dDocument) -> Process3dMutation {
    Process3dMutation::SetDocument { document }
}

pub fn apply(doc: &mut Process3dDocument, document: &Process3dDocument) {
    *doc = document.clone();
}
//#endregion 🔖️Mutation
