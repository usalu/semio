use super::*;
use crate::editor::generation3d::testkit::{app, dispatch_with_view, preview_views};
use crate::editor::generation3d::Generation3dCommand;

/// ⏱️ `flowEvalTick` is a WINDOW-owned retained job: its `extent` resolves only against the
/// addressed preview window's transient owner, so it must be dispatched through that window's own
/// `ViewModel` — an unaddressed dispatch is refused with `retained command exceeds semantic work
/// capacity`, which is the contract, not a defect (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn flow_eval_tick_does_not_panic_with_nothing_pending() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let (view, _) = preview_views("procedural-preview-test", "procedural-preview-test-other");
    dispatch_with_view(&mut app, Generation3dCommand::FlowEvalTick(FlowEvalTick { window_id: view.window_id.clone().expect("preview window") }), view).await.expect("flowEvalTick");
}
