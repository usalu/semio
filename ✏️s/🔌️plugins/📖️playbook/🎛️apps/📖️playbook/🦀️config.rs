//! 🧮️ Playbook play app — view state (`PlaybookConfig`) and its operation enum (`PlaybookConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.playbook` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection edits are VCS'd exactly like document content.
//! B1: absorbs the former app-struct `RefCell<Vec<String>>` selection state, plus `locale` (was read off
//! `view_state.locale`) — mirrors `writer_engine::WriterConfig`/`forms::config::FormsConfig`'s B1 shape.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `PlaybookPlayApp::Config` — the pure-trait `DocumentApp::Config` for the playbook app.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "playbookcfg")]
#[dsl(layout = "lines")]
pub struct PlaybookConfig {
    /// 👁️ Selected step/block ids — was `PlaybookPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for PlaybookConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(PlaybookConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ B1: `PlaybookConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `PlaybookPlayApp::selected_ids` field write), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `writer_op::WriterConfigOperation`/`forms::config::FormsConfigOperation` exactly
/// (see either's doc comment for the whole-config-snapshot inverse rationale). Lives here, not in the
/// kernel `playbook` crate, since `PlaybookConfig` is this app's own config artifact, not shared domain
/// state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum PlaybookConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: PlaybookConfig,
    },
    #[dsl(key = "selected-ids")]
    SetSelectedIds { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<PlaybookConfig> for PlaybookConfigOperation {
    type Diff = PlaybookConfig;

    fn diff(&self, base: &PlaybookConfig) -> PlaybookConfig {
        let mut next = base.clone();
        match self {
            PlaybookConfigOperation::Snapshot { config } => return config.clone(),
            PlaybookConfigOperation::SetSelectedIds { ids } => next.selected_ids = ids.clone(),
            PlaybookConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &PlaybookConfig) -> Vec<Self> {
        vec![PlaybookConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playbook_config_default_matches_the_existing_runtime_defaults() {
        let config = PlaybookConfig::default();
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn playbook_config_dsl_round_trips_default_and_populated() {
        store::test_support::assert_config_round_trip(&PlaybookConfig::default());
        let populated = PlaybookConfig { selected_ids: vec!["step-1".into(), "block-1".into()], locale: "de-DE".into() };
        store::test_support::assert_config_round_trip(&populated);
    }

    #[test]
    fn playbook_config_pack_round_trips() {
        let config = PlaybookConfig { selected_ids: vec!["block-1".into()], locale: "de-DE".into() };
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <PlaybookConfig as store::DocumentPack>::decode_pack(&bytes).expect("decode playbook config pack");
        assert_eq!(decoded, config);
    }

    fn config_round_trip(base: &PlaybookConfig, operation: &PlaybookConfigOperation) -> PlaybookConfig {
        let forward = operation.diff(base);
        let backwards = operation.backwards(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_operations_apply_and_restore_every_field() {
        let base = PlaybookConfig::default();
        assert_eq!(config_round_trip(&base, &PlaybookConfigOperation::SetSelectedIds { ids: vec!["block-1".into()] }).selected_ids, vec!["block-1".to_string()]);
        assert_eq!(config_round_trip(&base, &PlaybookConfigOperation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
    }

    #[test]
    fn playbook_config_operation_binary_matches_text() {
        store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigOperation::Snapshot { config: PlaybookConfig::default() });
    }
}
//#endregion 🧪️Tests
