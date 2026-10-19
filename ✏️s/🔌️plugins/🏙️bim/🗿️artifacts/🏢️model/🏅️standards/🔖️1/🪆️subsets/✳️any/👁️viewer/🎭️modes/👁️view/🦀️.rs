//! 👁️ BIM viewer: the `view` mode, a world | plan split. The read-only counterpart of the editor `edit` mode.

use crate::viewer::bim::modes::view::windows::{plan, world};
use semio_framework_plugin::{create_default_layout, ModeDefinition, WindowLayout};
use semio_s_artifact_bim_model_tests::viewer::bim::terminology::BimViewerLabels;

pub const BIM_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::bim::create_bim_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: BIM_VIEW_MODE_VIEW.into(), label: BimViewerLabels::localized(|labels| labels.mode_view), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The default layout: the world window on the left and the plan window on the right; the windows carry their own localized labels, so the tabs take them.
pub fn layout() -> WindowLayout {
    create_default_layout(&[world::WINDOW_KIND_ID.into(), plan::WINDOW_KIND_ID.into()], "row", Some(&[60.0, 40.0]), None)
}
//#endregion 🔖️Definition

//#region 🧩️Modules
#[path = "."]
pub mod windows {
    #[path = "."]
    pub mod world {
        #[path = "🪟️windows/🧊️world/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod plan {
        #[path = "🪟️windows/🗺️plan/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
//#endregion 🧩️Modules
