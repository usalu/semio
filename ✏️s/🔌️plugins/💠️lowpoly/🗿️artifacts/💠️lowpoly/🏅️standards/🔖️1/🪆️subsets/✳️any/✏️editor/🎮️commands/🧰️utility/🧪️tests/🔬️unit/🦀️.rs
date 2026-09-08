
use crate::editor::lowpoly::LowpolyCommand;
use crate::editor::lowpoly::testkit::{app, dispatch};
use semio_framework_plugin::{PluginApp, testkit};

#[semio_framework_async_macros::async_test]
async fn active_utility_switch_emits_no_ops_and_no_history() {
    // 🧰️ Selecting a host-owned utility must never create an undoable edit.
    let mut a = app().await;
    let result = dispatch(&mut a, LowpolyCommand::SetActiveUtility(super::set_active_utility::SetActiveUtility { utility_id: "rotate".into() })).await;
    assert!(result.mutations.is_empty(), "utility switch must emit no operations");
    let before = a.snapshot().expect("projection");
    a.handle_action("undo", None, &testkit::meta("a")).await.unwrap();
    assert_eq!(a.snapshot().expect("projection"), before, "utility switch left nothing to undo");
}
