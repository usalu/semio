//! 📤️ Vdi3805 play app command — replace the whole compliance document.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into the closed semantic vocabulary via `Vdi3805Mutation::from_snapshot`
//! (base + target, since `catalog.products`/`geometry`/`curves` are real id-keyed collections
//! needing full remove/re-insert), bundled into a single atomic edit.

use crate::config::{NormConfig, NormConfigMutation};
use crate::op::Vdi3805Mutation;
use crate::Vdi3805Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: Vdi3805Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, Vdi3805Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Vdi3805Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(Vdi3805Mutation::from_snapshot(doc.snapshot, &payload.snapshot), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
