//! 📤️ En1990 play app command — replace the whole compliance document.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into the closed semantic vocabulary via `En1990Mutation::from_snapshot`
//! (base + target, since `q_k` is a real ordered collection needing full remove/re-insert), bundled
//! into a single atomic edit.
//!
//! 🔧️ `text` carries the document's own `.en1990` DSL text (escaped onto one physical line via
//! `crate::document::escape_op_text_field`), not a nested `#[dsl(block)]` struct field — `En1990Snapshot` no longer
//! implements `dsl::DslField` now that `q_k` is a composed `ArtifactChild<S>` slot (ticket
//! 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2; no `DslField` impl reachable from this
//! crate, same gap `➗️mathematical`/`📐️cad`/`✒️writer`/din18599 hit for their own composed-child
//! snapshot types). The decomposition itself is unchanged: `En1990Mutation::from_snapshot` still
//! reads both sides through the `en1990_qk` working-scene accessor.
//!
//! ⚠️ Deliberately NOT `serde_json` (unlike `✒️writer`'s `set_snapshot::SetSnapshot`): this
//! workspace's `serde_json` does not round-trip every `f64` value losslessly (confirmed
//! empirically on `din18599`'s `h_v = 40.800000000000004`, which parses back as a different,
//! nearby `f64` — see `din18599`'s `🎮️commands/📤️set-snapshot/🦀️.rs` for the full
//! writeup). `En1990Snapshot`'s own hand-rolled `ArtifactDsl` codec (Rust's own
//! `f64::to_string`/`str::parse`) round-trips correctly, so it is used here instead.

use crate::op::En1990Mutation;
use crate::En1990Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
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
pub fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, En1990Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1990Mutation, NormConfigMutation>, Fault> {
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
