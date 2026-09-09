//! 📤️ Din4108 play app command — replace the whole compliance document.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into the closed semantic vocabulary via `Din4108Mutation::from_snapshot`
//! (base + target, since `layers` is a real ordered collection needing full remove/re-insert),
//! bundled into a single atomic edit.

use crate::config::{NormConfig, NormConfigMutation};
use crate::op::Din4108Mutation;
use crate::Din4108Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: Din4108Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, Din4108Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Din4108Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(Din4108Mutation::from_snapshot(doc.snapshot, &payload.snapshot), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
