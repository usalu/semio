//! 🪜️ Layout editor commands: the canvas, frame, page, document, export and engagement commands with the edit mode's windows, the canvas and the terminology (part 2).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

#[path = "🎯️actions/🦀️.rs"]
pub mod actions;
#[path = "🧲️interaction/🦀️.rs"]
pub mod interaction;
#[path = "🛍️native/🦀️.rs"]
pub mod native;
pub use actions::{LAYOUT_PLAY_APP_ID, layout_action, ui_label, ui_value_text, ui_value_map};
pub use interaction::{LAYOUT_INTERACTION_ELEMENTS, LAYOUT_GRANULARITY_ELEMENT, LayoutInteractionSnapshot, layout_select_action_args, layout_hover_action_args, layout_select_effect, layout_hover_effect, layout_clear_selection_effect};
pub use native::{NATIVE_PLACEMENTS, native_artifact_kind, NATIVE_OPEN, native_open};
#[path = "../🎮️commands/🚪️canvas-drag-leave/🦀️.rs"]
pub mod canvas_drag_leave;
#[path = "../🎮️commands/👆️canvas-pointer-up/🦀️.rs"]
pub mod canvas_pointer_up;
#[path = "../🎮️commands/🗑️delete-selection/🦀️.rs"]
pub mod delete_selection;
#[path = "../🎮️commands/🧭️engagement-input/🦀️.rs"]
pub mod engagement_input;
#[path = "../🎮️commands/📤️engagement-submit/🦀️.rs"]
pub mod engagement_submit;
#[path = "../🎮️commands/📦️export-package/🦀️.rs"]
pub mod export_package;
#[path = "../🎮️commands/📕️export-pdf/🦀️.rs"]
pub mod export_pdf;
#[path = "../🎮️commands/🌄️export-png/🦀️.rs"]
pub mod export_png;
#[path = "../🎮️commands/🎨️export-svg/🦀️.rs"]
pub mod export_svg;
#[path = "../🎮️commands/📄️patch-document/🦀️.rs"]
pub mod patch_document;
#[path = "../🎮️commands/📑️set-active-page/🦀️.rs"]
pub mod set_active_page;
#[path = "../🎮️commands/📷️set-camera/🦀️.rs"]
pub mod set_camera;
