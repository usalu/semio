//! 📤️ En1990 play app command — replace the whole compliance document via `from_snapshot`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::En1990Mutation;
use crate::En1990Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    pub text: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, En1990Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1990Mutation, NoConfigMutation>, Fault> {
    let text = crate::document::unescape_op_text_field(&payload.text);
    let target = <En1990Snapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|error| Fault::from(format!("set-snapshot: invalid document text: {error}")))?;
    crate::app_surface::commit_snapshot_fields(En1990Mutation::from_snapshot(doc.snapshot, &target), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
