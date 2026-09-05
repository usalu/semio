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

use crate::artifacts::din18599::op::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;
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
pub fn handle(payload: &ReplaceSnapshot, _doc: &ArtifactView<'_, Din18599Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Din18599Mutation, NormConfigMutation>, Fault> {
    let text = crate::document::unescape_op_text_field(&payload.text);
    let target = <Din18599Snapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|error| Fault::from(format!("set-snapshot: invalid document text: {error}")))?;
    crate::app_surface::commit_snapshot_fields(Din18599Mutation::from_snapshot(&target), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::din18599::op::Din18599Mutation;
    use semio_framework_plugin::HistoryView;

    #[semio_framework_async_macros::async_test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = Din18599Snapshot::default();
        let config = NormConfig::default();
        let text = crate::document::escape_op_text_field(&<Din18599Snapshot as store::ArtifactDsl>::print_dsl(&Din18599Snapshot::default()));
        let emit = handle(&ReplaceSnapshot { text }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        assert_eq!(emit.artifact_mutations, Din18599Mutation::from_snapshot(&Din18599Snapshot::default()));
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }

    /// 🌡️ Regression guard: this command's whole reason for using `ArtifactDsl` text instead of
    /// `serde_json` — a value whose shortest round-trip representation needs its full 17
    /// significant digits must survive the command's own payload encoding exactly.
    #[semio_framework_async_macros::async_test]
    fn handle_preserves_full_f64_precision_through_the_payload() {
        let projection = Din18599Snapshot::default();
        let config = NormConfig::default();
        let text = crate::document::escape_op_text_field(&<Din18599Snapshot as store::ArtifactDsl>::print_dsl(&Din18599Snapshot::default()));
        let emit = handle(&ReplaceSnapshot { text }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        let restored = emit.artifact_mutations.iter().fold(Din18599Snapshot::default(), |snapshot, mutation| vcs::apply_mutation(&snapshot, mutation).expect("set-snapshot mutation applies").0);
        assert_eq!(restored.h_v, Din18599Snapshot::default().h_v, "h_v must survive the set-snapshot payload with full precision");
    }
}
//#endregion 🧪️Tests
