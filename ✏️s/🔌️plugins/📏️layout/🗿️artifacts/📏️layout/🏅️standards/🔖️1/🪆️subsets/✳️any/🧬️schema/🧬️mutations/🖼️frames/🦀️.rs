//! 🖼️ Layout frame mutations: create, delete, move, resize, rotate, scale, drag, reorder and restyle frames and layers.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🪢️TaxonomyMounts
#[path = "../🔢change-frame-columns/🦀️.rs"]
pub mod change_frame_columns;
#[path = "../🎨change-frame-fill/🦀️.rs"]
pub mod change_frame_fill;
#[path = "../🖊️change-frame-stroke/🦀️.rs"]
pub mod change_frame_stroke;
#[path = "../🔤change-frame-wrap-mode/🦀️.rs"]
pub mod change_frame_wrap_mode;
#[path = "../➕create-frame/🦀️.rs"]
pub mod create_frame;
#[path = "../📑create-layer/🦀️.rs"]
pub mod create_layer;
#[path = "../➖delete-frame/🦀️.rs"]
pub mod delete_frame;
#[path = "../✋️drag-frames/🦀️.rs"]
pub mod drag_frames;
#[path = "../🕹️move-frame/🦀️.rs"]
pub mod move_frame;
#[path = "../🪜reorder-frame/🦀️.rs"]
pub mod reorder_frame;
#[path = "../📏resize-frame/🦀️.rs"]
pub mod resize_frame;
#[path = "../🔄️rotate-frame/🦀️.rs"]
pub mod rotate_frame;
#[path = "../🔃️rotate-frames/🦀️.rs"]
pub mod rotate_frames;
#[path = "../🗜️scale-frames/🦀️.rs"]
pub mod scale_frames;
#[path = "../🏷️set-drawing-text/🦀️.rs"]
pub mod set_drawing_text;
#[path = "../🔒set-frame-flags/🦀️.rs"]
pub mod set_frame_flags;
#[path = "../🧲set-frame-layer/🦀️.rs"]
pub mod set_frame_layer;
#[path = "../📐update-grid/🦀️.rs"]
pub mod update_grid;
#[path = "../🗂️update-layer/🦀️.rs"]
pub mod update_layer;
#[path = "../📑️update-text-frame/🦀️.rs"]
pub mod update_text_frame;
//#endregion 🪢️TaxonomyMounts
