//! 📤️ En1990 play app command — replace the whole compliance document.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into the closed semantic vocabulary via `En1990Mutation::from_snapshot`
//! (base + target, since `q_k` is a real ordered collection needing full remove/re-insert), bundled
//! into a single atomic edit.
//!
//! 🔧️ `text` carries the document's own `.en1990` DSL text (escaped onto one physical line via
//! `crate::document::escape_op_text_field`, same convention `SetArtifactMutation<D>`'s `OpText`
//! impl already uses), NOT a nested `#[dsl(block)]` struct field — `En1990Snapshot` no longer
//! implements `dsl::DslField` now that `q_k` is a composed `ArtifactChild<S>` slot (ticket
//! 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2; no `DslField` impl reachable from this
//! crate, same gap `➗️mathematical`/`📐️cad`/`✒️writer`/din18599 hit for their own composed-child
//! snapshot types). The decomposition itself is unchanged: `En1990Mutation::from_snapshot` still
//! reads both sides through the `en1990_qk` working-scene accessor.
//!
//! ⚠️ Deliberately NOT `serde_json` (unlike `✒️writer`'s `set_snapshot::SetSnapshot`): this
//! workspace's `serde_json` does not round-trip every `f64` value losslessly (confirmed
//! empirically on `din18599`'s `h_v = 40.800000000000004`, which parses back as a different,
//! nearby `f64` — see `din18599`'s `🎮️commands/📤️set-snapshot/🦀️component.rs` for the full
//! writeup). `En1990Snapshot`'s own hand-rolled `ArtifactDsl` codec (Rust's own
//! `f64::to_string`/`str::parse`) round-trips correctly, so it is used here instead.

use crate::artifacts::en1990::op::En1990Mutation;
use crate::artifacts::en1990::En1990Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    pub text: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub async fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, En1990Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1990Mutation, NormConfigMutation>, Fault> {
    let text = crate::document::unescape_op_text_field(&payload.text);
    let target = <En1990Snapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|error| Fault::from(format!("set-snapshot: invalid document text: {error}")))?;
    crate::app_surface::commit_snapshot_fields(En1990Mutation::from_snapshot(doc.snapshot, &target), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1990::op::En1990Mutation;
    use semio_framework_plugin::HistoryView;

    #[semio_framework_async_macros::async_test]
    async fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = En1990Snapshot::default();
        let config = NormConfig::default();
        let text = crate::document::escape_op_text_field(&<En1990Snapshot as store::ArtifactDsl>::print_dsl(&En1990Snapshot::default()));
        let emit = handle(&ReplaceSnapshot { text }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        assert_eq!(emit.artifact_mutations, En1990Mutation::from_snapshot(&En1990Snapshot::default(), &En1990Snapshot::default()));
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
