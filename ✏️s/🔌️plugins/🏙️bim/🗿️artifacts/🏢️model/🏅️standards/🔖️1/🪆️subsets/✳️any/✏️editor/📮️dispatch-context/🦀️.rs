//! 📮️ `BimDispatchCtx`: the per-dispatch state every command handler reads, built once per command by the editor. It sits below the command nodes so that no node reaches back into the editor root.

use crate::editor::bim::modes::edit::windows::{plan, schedule, section, sheet, world};
use crate::editor::bim::presence::{BimPresence, BimPresenceMutation};
use crate::editor::bim::terminology::{bim_labels, BimLabels};
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
    pub plan: plan::config::BimPlanWindowConfig,
    pub world: world::config::BimWorldWindowConfig,
    pub section: section::config::BimSectionWindowConfig,
    pub schedule: schedule::config::BimScheduleWindowConfig,
    pub sheet: sheet::config::BimSheetWindowConfig,
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
            utility: view.map_or(crate::editor::bim::utilities::DEFAULT_UTILITY, crate::editor::bim::utilities::active).to_string(),
            view: view.cloned(),
            plan: plan::config::from_snapshot(window_config),
            world: world::config::from_snapshot(window_config),
            section: section::config::from_snapshot(window_config),
            schedule: schedule::config::from_snapshot(window_config),
            sheet: sheet::config::from_snapshot(window_config),
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
