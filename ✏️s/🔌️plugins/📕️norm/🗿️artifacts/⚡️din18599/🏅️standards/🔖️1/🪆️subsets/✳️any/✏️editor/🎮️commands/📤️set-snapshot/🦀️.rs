//! 📤️ DIN V 18599 play app command — replace the whole compliance document.
//!
//! 📌️ The payload's `#[dsl(keyword)]` MUST equal the `app_commands!` row's `as` literal: a single-field
//! tuple variant delegates its whole `RecordSpec` to the inner type, whose keyword otherwise defaults to
//! `None` and would print with no leading keyword at all.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into one `change-<field>` mutation per persistent field via
//! `Din18599Mutation::from_snapshot`, bundled into a single atomic edit.
//!
//! 🔧️ `text` carries the document's own `.din18599` DSL text (escaped onto one physical line via
//! `crate::document::escape_op_text_field`), not a nested `#[dsl(block)]` struct field — `Din18599Snapshot` no longer
//! implements `dsl::DslField` now that `climate` is a composed `ArtifactChild<S>` slot (ticket
//! 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2; no `DslField` impl reachable from this
//! crate, same gap `➗️mathematical`/`📐️cad`/`✒️writer`/en1990 hit for their own composed-child
//! snapshot types).
//!
//! ⚠️ Deliberately NOT `serde_json` (unlike `✒️writer`'s `set_snapshot::SetSnapshot`, which this
//! command otherwise mirrors): this workspace's `serde_json` does not round-trip every `f64` value
//! losslessly (confirmed empirically — `serde_json::from_str::<f64>("40.800000000000004")` parses
//! to a DIFFERENT, nearby `f64`, `40.8`; Rust's own `f64::to_string`/`str::parse` — what
//! `Din18599Snapshot`'s hand-rolled `ArtifactDsl` impl already uses — round-trips correctly). Using
//! the snapshot's own DSL codec here avoids that precision loss entirely.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::Din18599Mutation;
use crate::Din18599Snapshot;
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
pub fn handle(payload: &ReplaceSnapshot, _doc: &ArtifactView<'_, Din18599Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din18599Mutation, NoConfigMutation>, Fault> {
    let text = crate::document::unescape_op_text_field(&payload.text);
    let target = <Din18599Snapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|error| Fault::from(format!("set-snapshot: invalid document text: {error}")))?;
    crate::app_surface::commit_snapshot_fields(Din18599Mutation::from_snapshot(&target), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
