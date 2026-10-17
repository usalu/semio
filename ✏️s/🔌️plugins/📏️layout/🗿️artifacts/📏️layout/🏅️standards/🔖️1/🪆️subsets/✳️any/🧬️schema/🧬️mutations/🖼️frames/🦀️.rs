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
#[path = "../🖊️change-frame-stroke/🦀️.rs"]
pub mod change_frame_stroke;
#[path = "../➕create-frame/🦀️.rs"]
pub mod create_frame;
#[path = "../📑create-layer/🦀️.rs"]
pub mod create_layer;
#[path = "../➖delete-frame/🦀️.rs"]
pub mod delete_frame;
#[path = "../🏷️set-drawing-text/🦀️.rs"]
pub mod set_drawing_text;
#[path = "../🧲set-frame-layer/🦀️.rs"]
pub mod set_frame_layer;
#[path = "../📐update-grid/🦀️.rs"]
pub mod update_grid;
#[path = "../🗂️update-layer/🦀️.rs"]
pub mod update_layer;
//#endregion 🪢️TaxonomyMounts

pub use semio_s_artifact_layout_layout_mutations_frames_2::{change_frame_fill, change_frame_wrap_mode, drag_frames, move_frame, reorder_frame, resize_frame, rotate_frame, rotate_frames, scale_frames, set_frame_flags, update_text_frame};
