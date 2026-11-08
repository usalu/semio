//! 👁️ DIN V 18599 viewer — the `view` mode: the read-only model, checks and diagrams over the same scene the editor projects — ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "at least one window" for a viewer packet.

use semio_plugin::{ModeDefinition, WindowLayout};
use semio_s_plugin_norm_app_surface::{device_profiles::LayoutPrefs, viewport3d::config::DeviceClass};

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::create_din18599_viewer`.
pub fn definition() -> ModeDefinition {
    semio_s_plugin_norm_app_surface::view_mode_definition()
}

/// 🪟️ Model, checks and diagrams of the shared visual layout at the default preferences.
pub fn layout() -> WindowLayout {
    layout_for(DeviceClass::Desktop, &LayoutPrefs::default())
}

/// 📱️ The viewer's window layout of a device class under the user's persisted layout preferences.
pub fn layout_for(device: DeviceClass, prefs: &LayoutPrefs) -> WindowLayout {
    semio_s_plugin_norm_app_surface::view_layout_with("din18599", device, prefs)
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🪢️Mounts
#[path = "."]
pub mod windows {
    #[path = "🪟️windows/✅️checks/🦀️.rs"]
    pub mod checks;
    #[path = "🪟️windows/📈️diagrams/🦀️.rs"]
    pub mod diagrams;
    #[path = "🪟️windows/🧊️viewport/🦀️.rs"]
    pub mod viewport;
}
//#endregion 🪢️Mounts
