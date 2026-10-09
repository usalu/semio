//! 🎚️ Persisted local state for one exact BIM schedule window: the schedule it shows (empty shows the list of schedules) and whether it shows the table or the definition editor.

use crate::editor::bim::kit::window_config;

window_config! {
    window: super::WINDOW_KIND_ID,
    schema: "bim.schedule-window.config",
    envelope: "s.bim.model.schedule-window.config",
    extension: "bimschedulewindowcfg",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧮️schedule/🎚️config",
    display: "Set BIM Schedule Window Configuration",
    type BimScheduleWindowConfig, BimScheduleWindowConfigDiff, BimScheduleWindowConfigMutation, BimScheduleWindowConfigOwner;
    schedule: String = String::new();
    editing: bool = false;
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
