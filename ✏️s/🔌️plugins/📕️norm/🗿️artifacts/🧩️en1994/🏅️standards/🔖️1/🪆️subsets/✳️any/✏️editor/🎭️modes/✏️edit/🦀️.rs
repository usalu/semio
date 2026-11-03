//! ✏️ EN 1994 play app — the `edit` mode: properties and outliner left, 3D model centre, checks and diagrams right.

use semio_framework_plugin::{ModeDefinition, WindowLayout};
use semio_s_plugin_norm_app_surface::{device_profiles::LayoutPrefs, viewport3d::config::DeviceClass};

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::create_en1994_app`. Every norm app declares
/// the same single `edit` mode, so its shape is constructed once in `crate::document::app`.
pub fn definition() -> ModeDefinition {
    semio_s_plugin_norm_app_surface::edit_mode_definition()
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the app-level `default_layout`: the desktop dock of the shared visual layout at the default preferences.
pub fn layout() -> WindowLayout {
    layout_for(DeviceClass::Desktop, &LayoutPrefs::default())
}

/// 📱️ The window layout of a device class under the user's persisted layout preferences; the same windows rearranged (desktop dock, mobile tab bar, tablet two panes).
pub fn layout_for(device: DeviceClass, prefs: &LayoutPrefs) -> WindowLayout {
    semio_s_plugin_norm_app_surface::visual_layout_with("en1994", device, prefs)
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
    #[path = "🪟️windows/🌲️outliner/🦀️.rs"]
    pub mod outliner;
    #[path = "🪟️windows/🧾️properties/🦀️.rs"]
    pub mod properties;
    #[path = "🪟️windows/🧊️viewport/🦀️.rs"]
    pub mod viewport;
}
//#endregion 🪢️Mounts
