/// 📏️ Surface budget and context menu laws of the generation2d editor routes: the resident surface bound, the disclosure budget and the framework-owned graph selection.

//#region 📏️SurfaceBudgetTests
/// 🗂️ Every authored body key, in the order `generation2d_render_body` matches them.
const GENERATION2D_BODY_KEYS: [&str; 8] = [
    flow_window::GENERATION2D_PLAY_BODY_MAIN,
    edit_preview::GENERATION2D_PLAY_BODY_PREVIEW,
    generations::GENERATION2D_PLAY_BODY_GENERATIONS,
    form::GENERATION2D_PLAY_BODY_GENERATE_FORM,
    generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW,
    document_panel::GENERATION2D_PLAY_BODY_ARTIFACT,
    catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE,
    inspection_panel::GENERATION2D_PLAY_BODY_INSPECTION,
];

/// 📏️ The generation2d twin of generation3d's surface-bound law. Every window and panel body must fit
/// the framework's ONE resident surface capacity (`ui_contract::UI_RESIDENT_SURFACE_BYTES`, which
/// `ui_runtime`'s `SURFACE_RECONCILE_SURFACE_BYTES` is defined as). The registered operator catalogue is
/// APP-STATIC and rides the reserved `framework.section.catalogue` surface, so no body may grow with the
/// installed operator set — the exact failure that made the node-graph window unrenderable
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
#[semio_framework_async_macros::async_test]
async fn every_window_and_panel_surface_fits_the_resident_surface_bound() {
    use crate::app_fixture::render;
    let bound = semio_framework_ui_contract::UI_RESIDENT_SURFACE_BYTES;
    let mut app = app_with_registry().await;
    for body_key in GENERATION2D_BODY_KEYS {
        let rendered = render(&mut app, body_key).await;
        println!("[STATS] surface body={body_key} bytes={} bound={bound}", rendered.len());
        assert!(!rendered.is_empty(), "{body_key} rendered empty");
        assert!(rendered.len() <= bound, "{body_key} is {} B, over the {bound} B resident surface bound", rendered.len());
    }
    close(app);
}
//#endregion 📏️SurfaceBudgetTests

//#region 🔖️ContextMenuTests
/// 🗂️ Grouped disclosure still stays within the row budget when nothing is selected.
#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_disclosure_budget() {
    let mut app = app_with_registry().await;
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let items = app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    close(app);
    assert!(items.len() <= 9, "top-level menu rows (leaves + groups + separator) must stay within disclosure budget, got {}", items.len());
    assert!(
        items.iter().any(|item| item.id == "delete-selection" && item.disabled == Some(true) && item.reason.as_deref() == Some("Nothing selected") && item.action.is_none() && item.args.is_none()),
        "an empty selection keeps delete-selection visible, disabled with its reason, and dispatching nothing: {items:?}"
    );
}

/// 🕹️ The runtime funnels every right-click through `context_menu_with_request_context`, so a node
/// selected through the framework-owned `graph` domain must unfold the destructive delete row that
/// dispatches `nodeGraphEdit` with a `delete {nodeIds, synapseIds}` row naming that selection.
#[semio_framework_async_macros::async_test]
async fn context_menu_reads_the_framework_owned_graph_selection() {
    let mut app = app_with_registry().await;
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let ids_of = |menu: &[semio_framework_plugin::ContextMenuItemSpec]| -> Vec<String> {
        menu.iter().flat_map(|item| std::iter::once(item.id.clone()).chain(item.children.iter().flatten().map(|child| child.id.clone()))).collect()
    };
    let unselected = ids_of(&app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await);
    assert!(!unselected.iter().any(|id| id.contains("delete")), "an empty selection must not offer delete: {unselected:?}");
    context::select_graph(&mut app, "node", &["slider"]).await;
    let selected = ids_of(&app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await);
    assert!(selected.iter().any(|id| id.contains("delete")), "a live graph selection must offer the destructive delete row: {selected:?}");
    let rows = app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.into_iter().find(|item| item.id == "delete-selection").and_then(|item| item.args).expect("the delete row carries its nodeGraphEdit arguments");
    let rows = semio_framework_tool_machine::node_graph_edit_rows(&rows).expect("the delete row speaks the shared row vocabulary");
    assert_eq!(rows, vec![semio_framework_tool_machine::NodeGraphEditRow::Delete { node_ids: vec!["slider".into()], synapse_ids: Vec::new() }], "the row names the live selection by id");
    close(app);
}
//#endregion 🔖️ContextMenuTests
