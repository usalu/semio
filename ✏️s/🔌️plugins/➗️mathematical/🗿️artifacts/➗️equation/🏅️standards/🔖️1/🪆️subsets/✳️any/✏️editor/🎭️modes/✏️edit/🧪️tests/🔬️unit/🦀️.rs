use crate::{editor::equation::modes::edit::windows::geometry, editor::equation::modes::edit::windows::graph};
use crate::editor::equation::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    let debug = format!("{:?}", layout());
    assert!(debug.contains(graph::MATH_PLAY_WINDOW_GRAPH) && debug.contains(geometry::MATH_PLAY_WINDOW_GEOMETRY), "layout must reference both window kinds: {debug}");
}
