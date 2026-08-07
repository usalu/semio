//! 🧮️ Forms play app — view state (`FormsConfig`) and its operation enum (`FormsConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.forms` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/wizard edits are VCS'd exactly like document
//! content. B1: absorbs every field that used to live on `forms_ui::FormsPlayApp`'s
//! `RefCell<FormsPlayRuntime>` (blueprint selection, the Try wizard's active step, its in-progress answer
//! values) plus `locale` (was read off `view_state.locale`) and `contributions_json` (was read off
//! `view_state.contributions_json` — the host-declared `Contribution::FormsQuestionKind` (legacy:
//! `PlaybookBlockKind`) list backing extension question kinds in the blueprint builder, try wizard, and
//! extension question rendering; the host now pushes contributions into config via
//! `SetContributions`, mirroring how it now pushes locale via `SetLocale`).

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `FormsPlayApp::Config` — the pure-trait `DocumentApp::Config` for the forms app.
/// `try_values_json`/`contributions_json` are both heterogeneous JSON (per-question-kind value shapes; an
/// arbitrary `Contribution` list) with no single concrete `dsl`-typed shape, so both stay JSON-blob
/// strings — the same idiom `layout_engine::LayoutConfig`'s port-recipe sibling
/// (`LayoutDocument::data_fields_json`) and `shooting_protocol::ShootingCommand::SetFixtureJson` use for
/// "opaque JSON payload, never a document/config field type of its own" data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "formscfg")]
#[dsl(layout = "lines")]
pub struct FormsConfig {
    /// 👁️ Selected blueprint step/question ids — was `FormsPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ The Try wizard's active step index — was `FormsPlayRuntime::current_step_index`.
    pub current_step_index: u32,
    /// 👁️ The Try wizard's in-progress answer overrides (JSON object text, question id -> value) — was
    /// `FormsPlayRuntime::try_values`.
    pub try_values_json: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🧩️ Host-declared plugin contributions (JSON array of `{pluginId, contribution}`, only
    /// `Contribution::FormsQuestionKind` entries matter; legacy `PlaybookBlockKind` still accepted) — was read off `view_state.contributions_json`.
    pub contributions_json: String,
}

impl Default for FormsConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), current_step_index: 0, try_values_json: "{}".into(), locale: "en-US".into(), contributions_json: "[]".into() }
    }
}

store::impl_whole_record_config!(FormsConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS Config recipe: [`FormsConfig`]'s operation enum — mirrors
/// `shooting_op::ShootingConfigOperation`'s shape exactly: one variant per settled interaction (was a
/// `FormsPlayRuntime` field write pre-B1), plus a generic `Snapshot` every variant's `backwards()` returns.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum FormsConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: FormsConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "step-index")]
    SetStepIndex { index: u32 },
    #[dsl(key = "try-values")]
    SetTryValues { json: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

impl Operation<FormsConfig> for FormsConfigOperation {
    type Diff = FormsConfig;

    fn diff(&self, base: &FormsConfig) -> FormsConfig {
        let mut next = base.clone();
        match self {
            FormsConfigOperation::Snapshot { config } => return config.clone(),
            FormsConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            FormsConfigOperation::SetStepIndex { index } => next.current_step_index = *index,
            FormsConfigOperation::SetTryValues { json } => next.try_values_json = json.clone(),
            FormsConfigOperation::SetLocale { value } => next.locale = value.clone(),
            FormsConfigOperation::SetContributions { json } => next.contributions_json = json.clone(),
        }
        next
    }

    fn backwards(&self, base: &FormsConfig) -> Vec<Self> {
        vec![FormsConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_config_default_matches_the_existing_runtime_defaults() {
        let config = FormsConfig::default();
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.current_step_index, 0);
        assert_eq!(config.try_values_json, "{}");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.contributions_json, "[]");
    }

    #[test]
    fn forms_config_dsl_and_pack_round_trip() {
        let config = FormsConfig { selected_ids: vec!["q1".into(), "q2".into()], current_step_index: 2, try_values_json: r#"{"name":"Ada"}"#.into(), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    fn config_round_trip(base: &FormsConfig, operation: &FormsConfigOperation) -> FormsConfig {
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
        let base = FormsConfig::default();
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetSelection { ids: vec!["q1".into()] }).selected_ids, vec!["q1".to_string()]);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetStepIndex { index: 2 }).current_step_index, 2);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetTryValues { json: r#"{"a":1}"#.into() }).try_values_json, r#"{"a":1}"#);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetContributions { json: "[]".into() }).contributions_json, "[]");
    }

    #[test]
    fn config_snapshot_op_text_round_trips() {
        let config = FormsConfig { selected_ids: vec!["q1".into(), "q2".into()], current_step_index: 1, try_values_json: r#"{"name":"Ada"}"#.into(), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetSelection { ids: vec!["a".into()] });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetStepIndex { index: 3 });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetLocale { value: "en-US".into() });
    }
}
//#endregion 🧪️Tests
