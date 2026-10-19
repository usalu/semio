//! 📮️ `BimDispatchCtx`: the per-dispatch state every command handler reads, built once per command by the editor. It sits below the command nodes so that no node reaches back into the editor root.

use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::{plan, section, sheet, world};
use semio_s_artifact_bim_model_tests::editor::bim::presence::{BimPresence, BimPresenceMutation};
use semio_s_artifact_bim_model_tests::editor::bim::terminology::{bim_labels, BimLabels};
use semio_framework_plugin::ViewModel;

//#region 🔖️Dispatch
/// 🕹️ Per-dispatch state that is neither document nor app config: the framework selection of both domains, the addressed window with its own config, the armed utility and the
/// local presence. The generated `dispatch` has no way to thread an `InteractionView`, so the editor builds this once per command and hands it down. Handlers push the presence
/// they want shared into `presence_out`.
#[derive(Default)]
pub struct BimDispatchCtx {
    pub selected: Vec<String>,
    pub library_selected: Vec<String>,
    pub view: Option<ViewModel>,
    pub window_kind: String,
    pub utility: String,
    pub plan: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::config::BimPlanWindowConfig,
    pub world: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::config::BimWorldWindowConfig,
    pub section: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::config::BimSectionWindowConfig,
    pub schedule: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::config::BimScheduleWindowConfig,
    pub sheet: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::config::BimSheetWindowConfig,
    pub presence: BimPresence,
    pub presence_out: Vec<BimPresenceMutation>,
    pub gestures: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub window_transient: crate::editor::bim::transient::BimWindowTransient,
    pub transient_out: Option<crate::editor::bim::transient::BimWindowTransient>,
}

impl BimDispatchCtx {
    /// 🕹️ The context of a command addressed at `view` with the config snapshot of its window.
    pub fn new(selected: Vec<String>, library_selected: Vec<String>, view: Option<&ViewModel>, window_config: Option<&semio_framework_plugin::WindowConfigSnapshot>, presence: Option<&BimPresence>) -> Self {
        Self {
            selected,
            library_selected,
            window_kind: view.and_then(crate::editor::bim::chrome::addressed_kind).unwrap_or_default().to_string(),
            utility: view.map_or(semio_s_artifact_bim_model_kit::editor::bim::utilities::DEFAULT_UTILITY, semio_s_artifact_bim_model_kit::editor::bim::utilities::active).to_string(),
            view: view.cloned(),
            plan: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::config::from_snapshot(window_config),
            world: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::config::from_snapshot(window_config),
            section: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::config::from_snapshot(window_config),
            schedule: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::config::from_snapshot(window_config),
            sheet: semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::config::from_snapshot(window_config),
            presence: presence.cloned().unwrap_or_default(),
            presence_out: Vec::new(),
            gestures: None,
            window_transient: Default::default(),
            transient_out: None,
        }
    }

    /// 🗣️ The labels of the addressed view; `None` when the command carries no view.
    pub fn labels(&self) -> Option<&'static BimLabels> {
        self.view.as_ref().map(bim_labels)
    }
}
//#endregion 🔖️Dispatch
