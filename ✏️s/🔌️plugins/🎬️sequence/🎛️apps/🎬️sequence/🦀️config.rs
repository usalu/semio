//! 🧮️ Sequence play app — view state (`SequenceConfig`) and its operation enum
//! (`SequenceConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.sequence` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/camera/orientation edits are VCS'd exactly
//! like document content.

use crate::artifacts::sequence::SequenceCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: sequence's real `DocumentApp::Config` — absorbs every former `SequencePlayRuntime` field
/// (`selected_step_ids`/`last_run_json`/`orientation`) plus the node-graph viewport camera
/// (session-only, never a document field) and the locale the pre-B1 host-pushed `ViewState` used to
/// carry (see `crate::apps::sequence::terminology::sequence_play_labels`) — same "absorb every
/// runtime field" shape `shooting_engine::ShootingConfig` established for the pilot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "sequencecfg")]
#[dsl(layout = "lines")]
pub struct SequenceConfig {
    /// 👁️ Selected step ids — was `SequencePlayRuntime::selected_step_ids`.
    pub selected_step_ids: Vec<String>,
    /// 🏃️ Last `run` command's `RunResult` JSON, rendered under the compiled script — was
    /// `SequencePlayRuntime::last_run_json`.
    pub last_run_json: String,
    /// 🌳️ Layered-layout flow direction (`"leftRight"`/`"topBottom"`) `reorganize` reads — was
    /// `SequencePlayRuntime::orientation`. Kept as a string rather than `DagLayoutOrientation`
    /// directly: that enum is foreign to this crate and only derives `Serialize`/`Deserialize`, not
    /// `dsl::DslField` (see `crate::apps::sequence::commands::layout`'s conversion helper).
    pub orientation: String,
    /// 🎥️ The node-graph viewport pan/zoom — session-only, never a document field. Was
    /// `SequencePlayRuntime::camera`.
    #[dsl(block)]
    pub camera: SequenceCamera,
    /// 🗣️ BCP-47 locale tag — was read off the host-pushed `ViewState.locale`.
    pub locale: String,
}

impl Default for SequenceConfig {
    fn default() -> Self {
        Self { selected_step_ids: Vec::new(), last_run_json: String::new(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(SequenceConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ B1: `SequenceConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `SequencePlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — same "whole-config snapshot is the simplest correct inverse" shape as
/// `shooting_op::ShootingConfigOperation`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SequenceConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: SequenceConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { step_ids: Vec<String> },
    #[dsl(key = "last-run")]
    SetLastRun { json: String },
    #[dsl(key = "orientation")]
    SetOrientation { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: SequenceCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<SequenceConfig> for SequenceConfigOperation {
    type Diff = SequenceConfig;

    fn diff(&self, base: &SequenceConfig) -> SequenceConfig {
        let mut next = base.clone();
        match self {
            SequenceConfigOperation::Snapshot { config } => return config.clone(),
            SequenceConfigOperation::SetSelection { step_ids } => next.selected_step_ids = step_ids.clone(),
            SequenceConfigOperation::SetLastRun { json } => next.last_run_json = json.clone(),
            SequenceConfigOperation::SetOrientation { value } => next.orientation = value.clone(),
            SequenceConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            SequenceConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &SequenceConfig) -> Vec<Self> {
        vec![SequenceConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_config_default_matches_the_existing_runtime_defaults() {
        let config = SequenceConfig::default();
        assert!(config.selected_step_ids.is_empty());
        assert!(config.last_run_json.is_empty());
        assert_eq!(config.orientation, "leftRight");
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn sequence_config_dsl_round_trips() {
        let config = SequenceConfig { selected_step_ids: vec!["step-1".into()], last_run_json: "{}".into(), orientation: "topBottom".into(), camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 }, locale: "de-DE".into() };
        let text = store::DocumentDsl::print_dsl(&config);
        let parsed = <SequenceConfig as store::DocumentDsl>::parse_dsl(&text).expect("config dsl round trip");
        assert_eq!(parsed, config);
    }

    #[test]
    fn sequence_config_pack_round_trips() {
        let config = SequenceConfig { selected_step_ids: vec!["step-2".into()], last_run_json: "{\"ok\":true}".into(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() };
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <SequenceConfig as store::DocumentPack>::decode_pack(&bytes).expect("config pack round trip");
        assert_eq!(decoded, config);
    }

    //#region 🔖️ConfigOperationTests
    fn round_trip_config(config: &SequenceConfig, operation: &SequenceConfigOperation) -> SequenceConfig {
        let forward = operation.diff(config);
        let backwards = operation.backwards(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward);
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_selection_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigOperation::SetSelection { step_ids: vec!["step-1".into()] });
        assert_eq!(next.selected_step_ids, vec!["step-1".to_string()]);
    }

    #[test]
    fn config_set_last_run_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigOperation::SetLastRun { json: "{\"ok\":true}".into() });
        assert_eq!(next.last_run_json, "{\"ok\":true}");
    }

    #[test]
    fn config_set_orientation_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigOperation::SetOrientation { value: "topBottom".into() });
        assert_eq!(next.orientation, "topBottom");
    }

    #[test]
    fn config_set_camera_round_trips() {
        let config = SequenceConfig::default();
        let camera = SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let next = round_trip_config(&config, &SequenceConfigOperation::SetCamera { camera: camera.clone() });
        assert_eq!(next.camera, camera);
    }

    #[test]
    fn config_set_locale_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigOperation::SetLocale { value: "de-DE".into() });
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::Snapshot { config: SequenceConfig::default() });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetSelection { step_ids: vec!["step-1".into(), "step-2".into()] });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetLastRun { json: "{}".into() });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetOrientation { value: "leftRight".into() });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetCamera { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🔖️ConfigOperationTests
}
//#endregion 🧪️Tests
