
use crate::editor::lowpoly::LowpolyCommand;
use crate::editor::lowpoly::testkit::app;
use semio_framework_plugin::{PluginApp, testkit};

#[semio_framework_async_macros::async_test]
async fn gumball_drag_coalesces_to_one_committed_edit() {
    // 🧲️ THE COALESCING REGRESSION: a multi-tick gumball translate must emit ZERO operations mid-drag and
    // exactly ONE commit operation (base → final mesh) on drag end — never a full-mesh patch per tick.
    let mut a = app().await;
    let before_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
    a.dispatch_typed(LowpolyCommand::TransformBegin(super::transform_begin::TransformBegin {}), &testkit::meta("a")).await.unwrap();
    let tick_a = a.dispatch_typed(LowpolyCommand::TranslateSelection(super::translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 0.5, dy: 0.0, dz: 0.0 }), &testkit::meta("a")).await.unwrap();
    let tick_b = a.dispatch_typed(LowpolyCommand::TranslateSelection(super::translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 0.25, dy: 0.0, dz: 0.0 }), &testkit::meta("a")).await.unwrap();
    assert!(tick_a.mutations.is_empty() && tick_b.mutations.is_empty(), "mid-drag transform ticks emit no operations");
    assert_eq!(a.snapshot().expect("projection").objects[0].mesh, before_mesh, "no operation reached the document mid-drag");
    let end = a.dispatch_typed(LowpolyCommand::TransformEnd(super::transform_end::TransformEnd {}), &testkit::meta("a")).await.unwrap();
    assert_eq!(end.mutations.len(), 1, "the whole drag commits as exactly one operation");
    let after_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
    assert_ne!(after_mesh, before_mesh, "the drag moved the mesh");
    a.handle_action("undo", None, &testkit::meta("a")).await.unwrap();
    assert_eq!(a.snapshot().expect("projection").objects[0].mesh, before_mesh, "one undo reverts the whole coalesced gumball drag");
}
