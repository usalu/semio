use super::*;
use crate::editor::generation3d::commands::move_media_node::MoveMediaNode;
use crate::editor::generation3d::testkit::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::editor::generation3d::testkit;

/// 📍️ The deliberately degenerate layout every assertion below starts from: every widget pinned to
/// one and the same canvas position, which is exactly the state `reorganize` exists to repair.
const OVERLAPPED: (f64, f64) = (-900.0, -900.0);

async fn overlap_every_widget(app: &mut crate::editor::generation3d::testkit::Generation3dApp) -> Vec<String> {
    let widget_ids: Vec<String> = testkit::snapshot(&app).fixture.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert!(widget_ids.len() >= 2, "the reorganize fixture needs at least two widgets to be meaningfully overlapped");
    for id in &widget_ids {
        dispatch(app, Generation3dCommand::MoveMediaNode(MoveMediaNode { node_id: id.clone(), x: OVERLAPPED.0, y: OVERLAPPED.1 })).await;
    }
    let layout = &testkit::snapshot(&app).fixture.layout;
    for id in &widget_ids {
        let position = layout.get(id).unwrap_or_else(|| panic!("widget {id} must have a pinned layout after moveMediaNode"));
        assert_eq!((position.x, position.y), OVERLAPPED, "widget {id} must start out overlapped");
    }
    widget_ids
}

/// 🗺️ `reorganize` must actually re-lay-out the graph: from a deliberately overlapped layout it has
/// to move at least one widget off the shared position, and it must not invent or drop widgets
/// while doing so.
#[semio_framework_async_macros::async_test]
async fn reorganize_moves_at_least_one_widget_off_a_deliberately_overlapped_layout() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let widget_ids = overlap_every_widget(&mut app).await;

    dispatch(&mut app, Generation3dCommand::Reorganize(Reorganize {})).await;

    let after = testkit::snapshot(&app);
    assert_eq!(after.fixture.widgets.len(), widget_ids.len(), "reorganize is a layout operation and must not add or remove widgets");
    let moved: Vec<&String> = widget_ids.iter().filter(|id| after.fixture.layout.get(id.as_str()).is_some_and(|position| (position.x, position.y) != OVERLAPPED)).collect();
    assert!(!moved.is_empty(), "reorganize must move at least one widget off the overlapped position, but every widget stayed at {OVERLAPPED:?}");
}

/// 🗺️ Repairing overlap means separating: after `reorganize` no two widgets may still share one
/// position, which is the property the overlapped baseline destroys.
#[semio_framework_async_macros::async_test]
async fn reorganize_leaves_no_two_widgets_sharing_one_position() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let widget_ids = overlap_every_widget(&mut app).await;

    dispatch(&mut app, Generation3dCommand::Reorganize(Reorganize {})).await;

    let after = testkit::snapshot(&app);
    let mut seen: std::collections::BTreeSet<(u64, u64)> = std::collections::BTreeSet::new();
    for id in &widget_ids {
        let Some(position) = after.fixture.layout.get(id.as_str()) else { continue };
        let key = (position.x.to_bits(), position.y.to_bits());
        assert!(seen.insert(key), "reorganize left widget {id} sharing a position with another widget");
    }
    assert!(!seen.is_empty(), "reorganize must leave a real layout behind");
}
