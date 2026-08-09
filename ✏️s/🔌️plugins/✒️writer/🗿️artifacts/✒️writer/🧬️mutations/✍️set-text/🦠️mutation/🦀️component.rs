//! ✍️ Writer mutation — `SetText` payload + builder + apply.
use crate::artifacts::writer::WriterSnapshot;
use crate::artifacts::writer::mutations::WriterMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji ✍️ `SetText` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetText {
    pub text: String,
}

pub fn set_text(text: String) -> WriterMutation {
    WriterMutation::SetText { text }
}

pub fn apply(snapshot: &mut WriterSnapshot, text: &str) {
    snapshot.text = text.to_string();
}
//#endregion 🔖️Mutation
