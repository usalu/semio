//! 📤️ EN 1999 play app command — replace the whole compliance document.
//!
//! 📌️ The payload's `#[dsl(keyword)]` MUST equal the `app_commands!` row's `as` literal: a single-field
//! tuple variant delegates its whole `RecordSpec` to the inner type, whose keyword otherwise defaults to
//! `None` and would print with no leading keyword at all.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into one `change-<field>` mutation per persistent field via
//! `En1999Mutation::from_snapshot`, bundled into a single atomic edit.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::En1999Mutation;
use crate::En1999Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: En1999Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ReplaceSnapshot, _doc: &ArtifactView<'_, En1999Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1999Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(En1999Mutation::from_snapshot(&payload.snapshot), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
