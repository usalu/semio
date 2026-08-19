//! 👁️ EN 1999 viewer — the `view` mode: a single full-pane Report window, the read-only
//! counterpart of the editor's inputs/results split — ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "at least one window"
//! for a viewer packet.

use crate::viewer::en1999::modes::view::windows::report;
use semio_framework_plugin::{ModeDefinition, WindowLayout};

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::en1999::create_en1999_viewer`.
pub async fn definition() -> ModeDefinition {
    crate::app_surface::view_mode_definition()
}

/// 🪟️ Single full-pane Report window — the read-only viewer has no inputs/results split to allocate.
pub async fn layout() -> WindowLayout {
    crate::app_surface::single_window_layout(report::WINDOW_KIND_ID, "Report")
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn the_mode_is_the_viewers_default() {
        assert_eq!(definition().id, crate::app_surface::MODE_VIEW);
    }
}
//#endregion 🧪️Tests
