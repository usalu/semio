//! 👁️ View model tests of the CAD editor unit tests: the gumball, camera, utility, context menu and sun measure behaviour. See [`super`].

use super::*;
#[cfg(test)] use semio_s_artifact_cad_cad_modes as edit;

//#region 🔖️ViewModel
#[semio_framework_async_macros::async_test]
async fn gumball_config_fields_present_regardless_of_dislocate_activation() {
    let selection = edit::world_selection_json(&view(default_document(), CadPlayRuntime::default()), CadPaneId::Shape, &[], Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(selection.contains("\"transformMode\":\"transform\""));
    assert!(selection.contains("\"moveAxes\":true"));
    assert!(selection.contains("\"rotate\":true"));
    assert!(selection.contains("\"scaleAxes\":false"));
    assert!(selection.contains("\"gumballActive\":false"), "no selection, no gumball");
    assert!(!selection.contains("\"gumballTarget\""));
}

/// 🕹️ The live `"cad"` domain reaches the world scene: selected/hovered ids ride the selection lane
/// and every instance carries its own flags, and the Dislocate gumball shows for a live selection.
#[semio_framework_async_macros::async_test]
async fn live_cad_selection_reaches_the_world_scene_and_arms_the_gumball() {
    let scene = forest_working_scene();
    let first = scene.building_objects[0].id.clone();
    let second = scene.building_objects[1].id.clone();
    let mut interaction = selecting(&[first.as_str()]);
    interaction.hovered_ids = vec![second.clone()];
    let view = view_with_interaction(forest_play_scene(), CadPlayRuntime::default(), interaction);
    let selection = edit::world_selection_json(&view, CadPaneId::Building, &scene.building_objects, Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(selection.contains(&format!("\"ids\":[\"{first}\"]")), "selection lane carries the domain's ids: {selection}");
    assert!(selection.contains(&format!("\"hoveredId\":\"{second}\"")), "selection lane carries the domain's hover: {selection}");
    assert!(selection.contains("\"gumballActive\":true"), "dislocate + selection arms the gumball: {selection}");
    let other_pane = edit::world_selection_json(&view, CadPaneId::Energy, &scene.energy_objects, Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(other_pane.contains("\"ids\":[]"), "a building selection never leaks into the energy pane's lane: {other_pane}");
    assert!(other_pane.contains("\"hoveredId\":null"), "a building hover never leaks into the energy pane's lane: {other_pane}");
    let instances: Vec<serde_json::Value> = serde_json::from_str(&edit::world_instances_json(&scene.building_objects, &view)).expect("instances json");
    assert_eq!(instances[0]["selected"], serde_json::Value::Bool(true));
    assert_eq!(instances[0]["hovered"], serde_json::Value::Bool(false));
    assert_eq!(instances[1]["selected"], serde_json::Value::Bool(false));
    assert_eq!(instances[1]["hovered"], serde_json::Value::Bool(true));
    let idle = edit::world_selection_json(&view, CadPaneId::Building, &scene.building_objects, None, CadDislocateOptions::default());
    assert!(idle.contains("\"gumballActive\":false"), "no dislocate utility, no gumball even with a selection");
}

/// 🎥️ Camera edits target the host-authenticated exact window and never app or document state.
#[semio_framework_async_macros::async_test]
async fn set_camera_writes_config_not_mutations() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let emit = drive_in_window(&app, &scene, "setCamera", Some(json!({ "surfaceId": "cad.play.scene3d/building", "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 2.0, "fov": 60.0 } })), &CadConfig::default(), "building-right", building::WINDOW_KIND_ID).expect("addressed camera command");
    assert!(emit.artifact_mutations.is_empty(), "setCamera must not emit a VCS operation");
    assert!(emit.config_mutations.is_empty(), "setCamera must not mutate app configuration");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "building-right");
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), building::WINDOW_KIND_ID);
}

/// 🖼️ The four panes' reference overlays share one reference id; only the pane whose model
/// definition the app-owned reference selection names paints it selected.
#[semio_framework_async_macros::async_test]
async fn reference_selection_paints_only_its_own_pane() {
    let runtime = CadPlayRuntime { selected_reference_model_definition_id: Some(CAD_MODEL_DEFINITION_ENERGY.into()), selected_reference_id: Some("ref-concrete-forest".into()), ..CadPlayRuntime::default() };
    let view = view(forest_play_scene(), runtime);
    let energy = edit::world_selection_json(&view, CadPaneId::Energy, &[], None, CadDislocateOptions::default());
    assert!(energy.contains("\"referenceSelectedId\":\"ref-concrete-forest\""), "{energy}");
    let building = edit::world_selection_json(&view, CadPaneId::Building, &[], None, CadDislocateOptions::default());
    assert!(!building.contains("referenceSelectedId"), "{building}");
}

#[semio_framework_async_macros::async_test]
async fn gumball_inactive_without_selection() {
    let selection = edit::world_selection_json(&view(default_document(), CadPlayRuntime::default()), CadPaneId::Shape, &[], Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(selection.contains("\"gumballActive\":false"));
    assert!(!selection.contains("\"gumballTarget\""));
}

#[semio_framework_async_macros::async_test]
async fn active_utility_flows_from_the_host_view_into_scene() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig::default();
    let view_state = ViewModel { active_utility_id: Some(CAD_DISLOCATE_UTILITY_ID.into()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let node = render_direct(&app, shape::BODY_KEY, &doc, &config, &view_state).expect("CAD UI assembly");
    let selection = scene_lane(&node, "selection");
    assert!(selection.contains(r#""transformMode":"transform""#), "render sources Dislocate from ViewModel.active_utility_id: {selection}");
}

#[semio_framework_async_macros::async_test]
async fn dislocate_utility_is_scoped_by_each_window_view_context() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig::default();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let shape_view = ViewModel { active_utility_id: Some(CAD_DISLOCATE_UTILITY_ID.into()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let building_view = ViewModel { active_utility_id: None, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let shape = scene_lane(&render_direct(&app, shape::BODY_KEY, &doc, &config, &shape_view).expect("CAD UI assembly"), "selection");
    let building = scene_lane(&render_direct(&app, building::BODY_KEY, &doc, &config, &building_view).expect("CAD UI assembly"), "selection");
    assert!(shape.contains(r#""gumballActive":false"#), "the interaction-less render path has no selection to arm: {shape}");
    assert!(shape.contains(r#""transformMode":"transform""#));
    assert!(building.contains(r#""gumballActive":false"#));
    assert!(!building.contains(r#""transformMode":"transform""#));
}

#[semio_framework_async_macros::async_test]
async fn context_menu_resolves_labels_from_the_registry() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let registry = AppActionRegistry::from_definition(&create_cad_app());
    let config = CadConfig::default();

    let view_state = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let items = context_menu_direct(&app, &doc, &config, &view_state, &registry);
    assert!(items.iter().any(|item| item.id == "translateSelection" && item.label.is_some()), "labels must resolve from the registry: {items:?}");
    assert!(items.iter().any(|item| item.id == "deleteObject" && item.destructive == Some(true)), "deleteObject must be marked destructive: {items:?}");
}

/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
/// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
/// and the destructive `deleteObject` action stays the trailing item.
#[semio_framework_async_macros::async_test]
async fn context_menu_is_grouped_and_keeps_delete_object_last() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let registry = AppActionRegistry::from_definition(&create_cad_app());
    let config = CadConfig::default();

    let view_state = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let items = context_menu_direct(&app, &doc, &config, &view_state, &registry);

    assert!(items.len() <= 9, "top-level context menu should stay progressively disclosed: {items:?}");
    assert_eq!(items.last().map(|item| item.id.as_str()), Some("deleteObject"), "deleteObject must stay the trailing item: {items:?}");
    assert_eq!(items.last().and_then(|item| item.destructive), Some(true), "trailing deleteObject must be marked destructive: {items:?}");
}

/// 🎛️ Dislocate preferences target an exact window instance.
#[semio_framework_async_macros::async_test]
async fn dislocate_move_and_rotate_options_are_per_pane() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let emit = drive_in_window(&app, &scene, "setDislocateOption", Some(json!({ "pane": "building", "option": "rotate", "pressed": false })), &CadConfig::default(), "building-left", building::WINDOW_KIND_ID).expect("addressed utility command");
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "building-left");
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), building::WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn engagement_hud_no_longer_carries_utility_switcher_options() {
    let mut app = new_app().await;
    let engagements = app.window_engagements(&four_pane_view()).await;
    assert_eq!(engagements.len(), CAD_PANE_WINDOW_KIND_IDS.len(), "every pane must project an engagement for this law to say anything");
    for engagement in engagements.values() {
        assert!(engagement.options.is_none(), "utility switching now lives in the framework utility bar, not the engagement HUD");
    }
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn utility_switch_has_no_plugin_command_or_config_lane() {
    assert!(<CadPlayApp as ArtifactEditor>::command_from_action(SET_ACTIVE_UTILITY_ACTION_ID, None, None).is_err());
    assert!(!CAD_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
    assert!(!every_command().iter().any(|command| command.command_id() == SET_ACTIVE_UTILITY_ACTION_ID));
}

#[semio_framework_async_macros::async_test]
async fn sun_measures_registered_for_all_four_panes_and_default_off() {
    let app = CadPlayApp::default();
    let base_config = CadConfig::default();
    assert!(!edit::windows::config::CadWorldWindowConfig::default().sun.enabled, "sun must be off by default");
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let view_state = ViewModel {
        window_id: Some("shape-sun".into()),
        active_window_kind_id: Some(shape::WINDOW_KIND_ID.into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "shape-sun".into(), window_kind_id: shape::WINDOW_KIND_ID.into() }],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let measures = window_measures_direct(&app, &doc, &base_config, &view_state);
    assert!(measures.contains_key("shape-sun"), "missing exact-window sun measures");
    let emit = drive_in_window(&app, &scene, "toggleSun", None, &base_config, "shape-sun", shape::WINDOW_KIND_ID).expect("addressed sun command");
    assert!(emit.config_mutations.is_empty());
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "shape-sun");
}

// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `worldPick`/`setHover`/`setSelection`
// and their round-trip tests are DELETED, not migrated — mesh object/vertex/edge/face
// selection AND hover are now the framework-owned `"cad"` interaction domain, dispatched
// through the auto-injected `interactionSelect`/`interactionHover` verbs (never app-declared)
// and tested once, centrally, by the framework's own `semio-framework-plugin` suite.
//#endregion 🔖️ViewModel
