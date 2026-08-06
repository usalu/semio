//! 🧮️ Procedural2d play app — view state (`Procedural2dConfig`) and its operation enum
//! (`Procedural2dConfigOperation`).
//!
//! This is APP state, not document state: selection, camera, show-mode and the derived generation
//! preview live here rather than under `🗿️artifacts/`, since none of it survives into the `.procedural2d`
//! document. It still round-trips through a real `DocumentStore` (with a real `backwards`), so every
//! edit is VCS'd exactly like document content.

use flow_core::CameraJson;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `Procedural2dPlayApp::Config` — the pure-trait config artifact. Selection, the graph camera, the
/// show-mode display toggle, the derived generation selection/preview, and locale all round-trip
/// through the config `DocumentStore` exactly like document content, with a real `backwards` per
/// [`Procedural2dConfigOperation`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "procedural2dcfg")]
#[dsl(layout = "lines")]
pub struct Procedural2dConfig {
    /// 👁️ Selected widget ids.
    pub selected_ids: Vec<String>,
    /// 🗺️ The node-graph camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 👁️ Display mode (`"preview"`/`"generate"`/`"wire"`).
    pub show_mode: String,
    /// 👁️ Active generation selection.
    pub selected_generation_id: Option<String>,
    /// 👁️ Derived generation preview text.
    pub generation_preview_text: Option<String>,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

impl Default for Procedural2dConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, show_mode: default_show_mode(), selected_generation_id: None, generation_preview_text: None, locale: "en-US".into() }
    }
}

pub fn default_show_mode() -> String {
    "preview".into()
}

store::impl_whole_record_config!(Procedural2dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`Procedural2dConfig`]'s operation enum — one variant per settled config write, plus a generic
/// `Snapshot` every variant's `backwards()` returns (each config tick is its own distinct edit, so
/// "undo this tick" is "restore the whole-config snapshot from just before it").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Procedural2dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Procedural2dConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "show-mode")]
    SetShowMode { value: String },
    #[dsl(key = "generation")]
    SetGeneration { selected_generation_id: Option<String>, generation_preview_text: Option<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<Procedural2dConfig> for Procedural2dConfigOperation {
    type Diff = Procedural2dConfig;

    fn diff(&self, base: &Procedural2dConfig) -> Procedural2dConfig {
        let mut next = base.clone();
        match self {
            Procedural2dConfigOperation::Snapshot { config } => return config.clone(),
            Procedural2dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Procedural2dConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            Procedural2dConfigOperation::SetShowMode { value } => next.show_mode = value.clone(),
            Procedural2dConfigOperation::SetGeneration { selected_generation_id, generation_preview_text } => {
                next.selected_generation_id = selected_generation_id.clone();
                next.generation_preview_text = generation_preview_text.clone();
            }
            Procedural2dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &Procedural2dConfig) -> Vec<Self> {
        vec![Procedural2dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_set_selection_round_trips_and_restores() {
        let base = Procedural2dConfig::default();
        let operation = Procedural2dConfigOperation::SetSelection { ids: vec!["w1".into(), "w2".into()] };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_ids, vec!["w1".to_string(), "w2".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards[0].diff(&forward), base);
    }

    #[test]
    fn config_set_camera_round_trips_and_restores() {
        let base = Procedural2dConfig::default();
        let camera = CameraJson { x: 9.0, y: -3.0, zoom: 2.5 };
        let forward = Procedural2dConfigOperation::SetCamera { camera: camera.clone() }.diff(&base);
        assert_eq!(forward.camera, camera);
    }

    #[test]
    fn config_set_show_mode_round_trips_and_restores() {
        let base = Procedural2dConfig::default();
        let forward = Procedural2dConfigOperation::SetShowMode { value: "wire".into() }.diff(&base);
        assert_eq!(forward.show_mode, "wire");
    }

    #[test]
    fn config_set_locale_round_trips_and_restores() {
        let base = Procedural2dConfig::default();
        let forward = Procedural2dConfigOperation::SetLocale { value: "de-DE".into() }.diff(&base);
        assert_eq!(forward.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        let config = Procedural2dConfig { selected_ids: vec!["a".into()], locale: "de-DE".into(), ..Procedural2dConfig::default() };
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetShowMode { value: "generate".into() });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetGeneration { selected_generation_id: None, generation_preview_text: None });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetGeneration { selected_generation_id: Some("g1".into()), generation_preview_text: None });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetLocale { value: "en-US".into() });
    }
}
//#endregion 🧪️Tests
