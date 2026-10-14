//! ✏️ Process 3d play app — the `edit` mode: the single-window workpiece authoring layout.


//#region 🧩️Modules
#[path = "."]
pub mod windows {
    #[path = "."]
    pub mod workpiece {
        #[path = "🪟️windows/🪚️workpiece/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod options {
            #[path = "🪟️windows/🪚️workpiece/☑️options/☀️sun/🦀️.rs"]
            pub mod sun;
        }
    }
}
//#endregion 🧩️Modules

use crate::windows::workpiece;
use semio_framework_plugin::create_default_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;

pub const PROCESS3D_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::process3d::create_process3d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PROCESS3D_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into()], "row", None, Some(&["Workpiece".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
