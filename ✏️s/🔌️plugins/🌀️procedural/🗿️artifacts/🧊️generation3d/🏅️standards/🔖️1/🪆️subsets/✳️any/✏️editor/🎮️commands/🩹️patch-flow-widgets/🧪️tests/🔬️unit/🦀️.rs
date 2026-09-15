use super::*;
use crate::editor::generation3d::unit_tests::context::{app, dispatch, snapshot};
use crate::editor::generation3d::Generation3dCommand;

fn patch(widget_id: &str, value: f64, gesture: Option<&str>) -> Generation3dCommand {
    Generation3dCommand::PatchFlowWidgets(PatchFlowWidgets { widget_ids: vec![widget_id.into()], field: "value".into(), value: Some(value), gesture: gesture.map(str::to_string) })
}

/// ⚖️ LAW: a value that names its press folds under that press's key, and a value that names none is a
/// described edit of its own — the same rule `nodeGraphEdit`'s `setSlider` answers, over the OTHER
/// door a continuous control reaches this document through (the Inspection panel's number field).
///
/// 🐛️ Without it, one second of a held spinner cost 29 undo steps and 24 preview evaluations, measured
/// on 6018 (`📓️slider-preview-update-2026-09-15.md` §4).
#[semio_framework_async_macros::async_test]
async fn a_patch_that_names_its_press_folds_under_that_press_and_an_anonymous_one_does_not() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    for (value, gesture) in [(6.5_f64, Some("p1")), (7.5, Some("p1")), (9.0, Some("p1"))] {
        dispatch(&mut app, patch("height", value, gesture)).await;
    }
    let landed = {
        let read = snapshot(&app);
        read.host_snapshot.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, .. } if id == "height" => Some(*value),
            _ => None,
        })
    };
    assert_eq!(landed, Some(9.0), "a press must leave the document on the value it released on");
    eprintln!("[DEBUG] patch press landed value={landed:?}");
}

/// ⚖️ LAW: a live patch declares the same narrow refresh scope a live `setSlider` does, and an
/// anonymous patch keeps the framework's full one.
#[semio_framework_async_macros::async_test]
async fn a_live_patch_declares_the_narrow_scope_and_an_anonymous_one_keeps_full() {
    assert_eq!(patch_coalesce_key(Some("p1")).as_deref(), Some("widget-field:p1"), "a live patch folds under its press");
    assert_eq!(patch_coalesce_key(Some("")), None, "an empty press identity is no identity");
    assert_eq!(patch_coalesce_key(None), None, "an anonymous patch is a described edit");
    assert_ne!(
        crate::editor::generation3d::commands::node_graph_edit::slider_gesture_ui_scope(),
        semio_framework::kernel::UiDirtyScope::Full,
        "a live patch must not repaint the whole shell"
    );
}
