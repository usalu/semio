//! 🎮️ Layout editor commands: the canvas, frame, page, document, export and engagement commands with the edit mode's windows, the canvas and the terminology.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;



pub use semio_s_artifact_layout_layout_editor_commands_2::{LAYOUT_PLAY_APP_ID, layout_action, ui_label, ui_value_text, ui_value_map, LAYOUT_INTERACTION_ELEMENTS, LAYOUT_GRANULARITY_ELEMENT, LayoutInteractionSnapshot, layout_select_action_args, layout_hover_action_args, layout_select_effect, layout_hover_effect, layout_clear_selection_effect, NATIVE_PLACEMENTS, native_artifact_kind, NATIVE_OPEN, native_open};
//#region 🪢️TaxonomyMounts
#[path = "🖼️add-frame/🦀️.rs"]
pub mod add_frame;
#[path = "➕️add-page/🦀️.rs"]
pub mod add_page;
#[path = "../🖼️canvas/🦀️.rs"]
pub mod canvas;
#[path = "🛬️canvas-drag-over/🦀️.rs"]
pub mod canvas_drag_over;
#[path = "📥️canvas-drop/🦀️.rs"]
pub mod canvas_drop;
#[path = "👇️canvas-pointer-down/🦀️.rs"]
pub mod canvas_pointer_down;
#[path = "↔️canvas-pointer-move/🦀️.rs"]
pub mod canvas_pointer_move;
#[path = "🧭️gumball/🦀️.rs"]
pub mod gumball;
#[path = "🩹️patch-frame/🦀️.rs"]
pub mod patch_frame;
#[path = "🧵️patch-page/🦀️.rs"]
pub mod patch_page;
#[path = "🔄️rotate-selection/🦀️.rs"]
pub mod rotate_selection;
#[path = "📏scale-selection/🦀️.rs"]
pub mod scale_selection;
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod modes {
    #[path = "../🎭️modes/✏️edit/🦀️.rs"]
    pub mod edit;
}
//#endregion 🪢️TaxonomyMounts

pub use semio_s_artifact_layout_layout_editor_commands_2::{canvas_drag_leave, canvas_pointer_up, delete_selection, engagement_input, engagement_submit, export_package, export_pdf, export_step, export_idml, export_png, export_svg, patch_document, set_active_page, set_camera};
