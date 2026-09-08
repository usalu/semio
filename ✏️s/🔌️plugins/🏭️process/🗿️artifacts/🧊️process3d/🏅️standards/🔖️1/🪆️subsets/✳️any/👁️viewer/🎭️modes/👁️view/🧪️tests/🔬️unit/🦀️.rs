
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_references_the_workpiece_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(workpiece::PROCESS3D_VIEW_WINDOW_MAIN), "layout must reference the workpiece window kind: {json}");
}
