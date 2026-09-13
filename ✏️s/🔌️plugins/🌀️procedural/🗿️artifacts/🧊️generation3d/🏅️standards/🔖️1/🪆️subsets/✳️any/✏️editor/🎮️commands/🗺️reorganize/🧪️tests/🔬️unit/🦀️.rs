use super::*;
use crate::editor::generation3d::commands::node_graph_edit::NodeGraphEdit;
use crate::editor::generation3d::unit_tests::context::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::editor::generation3d::unit_tests::context;

/// 📍️ The deliberately degenerate layout every assertion below starts from: every widget pinned to
/// one and the same canvas position, which is exactly the state `reorganize` exists to repair.
const OVERLAPPED: (f64, f64) = (-900.0, -900.0);

async fn overlap_every_widget(app: &mut crate::editor::generation3d::unit_tests::context::Generation3dApp) -> Vec<String> {
    let widget_ids: Vec<String> = context::snapshot(&app).fixture.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert!(widget_ids.len() >= 2, "the reorganize fixture needs at least two widgets to be meaningfully overlapped");
    for id in &widget_ids {
        dispatch(app, Generation3dCommand::NodeGraphEdit(NodeGraphEdit { operations_json: crate::editor::generation3d::unit_tests::node_move_operations_json(id, OVERLAPPED.0, OVERLAPPED.1) })).await;
    }
    let layout = &context::snapshot(&app).fixture.layout;
    for id in &widget_ids {
        let position = layout.get(id).unwrap_or_else(|| panic!("widget {id} must have a pinned layout after a nodeGraphEdit move"));
        assert_eq!((position.x, position.y), OVERLAPPED, "widget {id} must start out overlapped");
    }
    widget_ids
}

/// 🗺️ `reorganize` must actually re-lay-out the graph: from a deliberately overlapped layout it has
/// to move at least one widget off the shared position, and it must not invent or drop widgets
/// while doing so.
#[semio_framework_async_macros::async_test]
async fn reorganize_moves_at_least_one_widget_off_a_deliberately_overlapped_layout() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let widget_ids = overlap_every_widget(&mut app).await;

    dispatch(&mut app, Generation3dCommand::Reorganize(Reorganize {})).await;

    let after = context::snapshot(&app);
    assert_eq!(after.fixture.widgets.len(), widget_ids.len(), "reorganize is a layout operation and must not add or remove widgets");
    let moved: Vec<&String> = widget_ids.iter().filter(|id| after.fixture.layout.get(id.as_str()).is_some_and(|position| (position.x, position.y) != OVERLAPPED)).collect();
    assert!(!moved.is_empty(), "reorganize must move at least one widget off the overlapped position, but every widget stayed at {OVERLAPPED:?}");
}

/// 🗺️ Repairing overlap means separating: after `reorganize` no two widgets may still share one
/// position, which is the property the overlapped baseline destroys.
#[semio_framework_async_macros::async_test]
async fn reorganize_leaves_no_two_widgets_sharing_one_position() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let widget_ids = overlap_every_widget(&mut app).await;

    dispatch(&mut app, Generation3dCommand::Reorganize(Reorganize {})).await;

    let after = context::snapshot(&app);
    let mut seen: std::collections::BTreeSet<(u64, u64)> = std::collections::BTreeSet::new();
    for id in &widget_ids {
        let Some(position) = after.fixture.layout.get(id.as_str()) else { continue };
        let key = (position.x.to_bits(), position.y.to_bits());
        assert!(seen.insert(key), "reorganize left widget {id} sharing a position with another widget");
    }
    assert!(!seen.is_empty(), "reorganize must leave a real layout behind");
}

/// ⚖️ LAW: `reorganize` is reachable by BOTH hands. It is a top-level item of the flow canvas's
/// context menu (mouse) and it carries an app keybinding (keyboard), with an en+de label on the
/// action itself — no default language.
///
/// 🐛️ Before this law `reorganize` had exactly one trigger, the right-click menu, so a keyboard-only
/// user could not reach it at all and no report could cite a proof of it firing
/// (`📓️audit-user-journey-gaps-2026-09-13.md` gap #10).
#[test]
fn reorganize_is_reachable_by_menu_and_by_keyboard_in_both_languages() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let definition = crate::editor::generation3d::create_generation3d_app();
    let action = definition.window_kinds.iter().flat_map(|kind| kind.actions.iter()).find(|action| action.id == "reorganize").expect("reorganize action");
    let label = serde_json::to_string(&action.label).expect("reorganize label json");
    assert!(label.contains("Reorganize") && label.contains("Neu anordnen"), "reorganize needs both declared languages: {label}");
    let chords: Vec<&str> = definition.keybindings.iter().filter(|binding| binding.action.action == "reorganize").map(|binding| binding.keys.as_str()).collect();
    assert_eq!(chords, vec!["mod+alt+l"], "reorganize must carry exactly its declared keyboard chord");
    eprintln!("[DEBUG] reorganize chords={chords:?} label={label}");
}

/// ⚖️ LAW: the context menu still offers `reorganize` as its top-level layout verb — the mouse half of
/// the law above, asserted against the built menu rather than against the builder.
#[semio_framework_async_macros::async_test]
async fn reorganize_is_the_context_menus_top_level_layout_verb() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let menu = semio_framework_plugin::PluginApp::context_menu(&mut *app, &request, &semio_framework_plugin::ViewModel::default()).await;
    let ids: Vec<String> = menu.iter().map(|item| item.id.clone()).collect();
    assert!(ids.iter().any(|id| id == "reorganize"), "the flow canvas context menu must offer reorganize at top level: {ids:?}");
}
