use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    // 🌱️ `WindowLayout` (`semio-framework-plugin`, framework-owned) has not itself gained
    // `ToValue` — `Debug` gives the same "does the layout mention X" substring check.
    let debug = format!("{:?}", layout());
    assert!(debug.contains(graph::MATH_PLAY_WINDOW_GRAPH) && debug.contains(geometry::MATH_PLAY_WINDOW_GEOMETRY), "layout must reference both window kinds: {debug}");
}
