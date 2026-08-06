//! 🧮️ Animate present app — view state (`PresentConfig`) and its operation enum
//! (`PresentConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.present` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/engagement/locale edits are VCS'd exactly
//! like document content.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: animate present's real `DocumentApp::Config` — absorbs every former
/// `AnimatePresentPlayRuntime` field (`selected_ids`/`engagement_input`) plus the locale the pre-B1
/// host-pushed `ViewModel` used to carry (see `crate::apps::present::terminology`) — same "absorb every
/// runtime field" shape `shooting_engine::ShootingConfig` established for the pilot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "presentcfg")]
#[dsl(layout = "lines")]
pub struct PresentConfig {
    /// 👁️ Selected tile ids — was `AnimatePresentPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// ⌨️ In-progress engagement-bar input draft — was `AnimatePresentPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 🗣️ BCP-47 locale tag — was read off the host-pushed `ViewModel.locale`.
    pub locale: String,
}

impl Default for PresentConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), engagement_input: String::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(PresentConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ B1: `PresentConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `AnimatePresentPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — same "whole-config snapshot is the simplest correct inverse" shape as
/// `shooting_op::ShootingConfigOperation`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum PresentConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: PresentConfig,
    },
    #[dsl(key = "selection")]
    SetSelectedIds { ids: Vec<String> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<PresentConfig> for PresentConfigOperation {
    type Diff = PresentConfig;

    fn diff(&self, base: &PresentConfig) -> PresentConfig {
        let mut next = base.clone();
        match self {
            PresentConfigOperation::Snapshot { config } => return config.clone(),
            PresentConfigOperation::SetSelectedIds { ids } => next.selected_ids = ids.clone(),
            PresentConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            PresentConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &PresentConfig) -> Vec<Self> {
        vec![PresentConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_config_default_matches_the_existing_runtime_defaults() {
        let config = PresentConfig::default();
        assert!(config.selected_ids.is_empty());
        assert!(config.engagement_input.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn present_config_dsl_round_trips() {
        let config = PresentConfig { selected_ids: vec!["t1".into()], engagement_input: "2x2".into(), locale: "de-DE".into() };
        let text = store::DocumentDsl::print_dsl(&config);
        let parsed = <PresentConfig as store::DocumentDsl>::parse_dsl(&text).expect("config dsl round trip");
        assert_eq!(parsed, config);
    }

    #[test]
    fn present_config_pack_round_trips() {
        let config = PresentConfig { selected_ids: vec!["t2".into()], engagement_input: "add".into(), locale: "en-US".into() };
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <PresentConfig as store::DocumentPack>::decode_pack(&bytes).expect("config pack round trip");
        assert_eq!(decoded, config);
    }

    //#region 🔖️ConfigOperationTests
    fn round_trip_config(config: &PresentConfig, operation: &PresentConfigOperation) -> PresentConfig {
        let forward = operation.diff(config);
        let backwards = operation.backwards(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward);
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_selected_ids_round_trips() {
        let config = PresentConfig::default();
        let next = round_trip_config(&config, &PresentConfigOperation::SetSelectedIds { ids: vec!["t1".into()] });
        assert_eq!(next.selected_ids, vec!["t1".to_string()]);
    }

    #[test]
    fn config_set_engagement_input_round_trips() {
        let config = PresentConfig::default();
        let next = round_trip_config(&config, &PresentConfigOperation::SetEngagementInput { value: "2x2".into() });
        assert_eq!(next.engagement_input, "2x2");
    }

    #[test]
    fn config_set_locale_round_trips() {
        let config = PresentConfig::default();
        let next = round_trip_config(&config, &PresentConfigOperation::SetLocale { value: "de-DE".into() });
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&PresentConfigOperation::Snapshot { config: PresentConfig::default() });
        store::test_support::assert_op_line_round_trip(&PresentConfigOperation::SetSelectedIds { ids: vec!["t1".into(), "t2".into()] });
        store::test_support::assert_op_line_round_trip(&PresentConfigOperation::SetEngagementInput { value: "add".into() });
        store::test_support::assert_op_line_round_trip(&PresentConfigOperation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🔖️ConfigOperationTests
}
//#endregion 🧪️Tests
