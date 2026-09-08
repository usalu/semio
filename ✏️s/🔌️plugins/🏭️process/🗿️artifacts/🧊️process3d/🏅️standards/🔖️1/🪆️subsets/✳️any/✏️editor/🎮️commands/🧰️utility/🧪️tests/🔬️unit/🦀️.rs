
use super::*;
use crate::editor::process3d::testkit;

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_operations() {
    let mut app = testkit::app();
    let result = testkit::dispatch(&mut app, crate::editor::process3d::Process3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "cut".into() }));
    assert!(result.mutations.is_empty(), "utility selection is host-owned config state and must never emit document operations or history");
}
