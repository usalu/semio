use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app, app_with_registry, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::widget_id;

/// ❌️ `DeleteSelection` carries NO ids at all — its only input is the framework-owned `graph`
/// selection, so a dispatch after a real `interactionSelect` must remove exactly that widget and the
/// synapses hanging off it.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_framework_owned_graph_selection_from_the_flow_graph() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    {
        let before = context::snapshot(&app);
        assert!(before.fixture.widgets.iter().any(|widget| widget_id(widget) == "extrude"), "the default fixture owns the extrude widget");
        assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" || synapse.to == "extrude"), "the extrude widget is wired, so its removal must prune synapses too");
    }
    context::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::DeleteSelection(DeleteSelection {})).await;
    {
        let after = context::snapshot(&app);
        assert!(!after.fixture.widgets.iter().any(|widget| widget_id(widget) == "extrude"), "the selected widget survived its own delete");
        assert!(!after.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" || synapse.to == "extrude"), "a dangling synapse survived the delete");
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🧯️ With an EMPTY graph selection the delete is a no-op — never a whole-document wipe. This is the
/// destructive-row law: a menu entry that fires with nothing selected must cost nothing.
#[semio_framework_async_macros::async_test]
async fn delete_selection_with_an_empty_graph_selection_removes_nothing() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app).fixture.widgets.len();
    assert!(before > 0, "the default fixture must not be empty or the law is vacuous");
    dispatch(&mut app, Generation3dCommand::DeleteSelection(DeleteSelection {})).await;
    assert_eq!(context::snapshot(&app).fixture.widgets.len(), before);
}
