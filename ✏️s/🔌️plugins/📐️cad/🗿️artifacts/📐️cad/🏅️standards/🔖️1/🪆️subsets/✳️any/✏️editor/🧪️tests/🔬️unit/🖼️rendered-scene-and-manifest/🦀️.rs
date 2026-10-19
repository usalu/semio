//! 🖼️ Rendered scene and manifest tests of the CAD editor unit tests: the world scene per pane, the utility, the manifest stitching and the reference verbs. See [`super`].

use super::*;

//#region 🔖️Render
#[semio_framework_async_macros::async_test]
async fn renders_world_scene_for_each_pane() {
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let view_state = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for body_key in [shape::BODY_KEY, building::BODY_KEY, energy::BODY_KEY, structure_classic::BODY_KEY] {
        let node = render_direct(&app, body_key, &doc, &CadConfig::default(), &view_state).expect("CAD UI assembly");
        let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = &node.component else { panic!("body {body_key} should render a surface") };
        assert_eq!(props.doc_schema.as_str(), "world-3d@1", "body {body_key} should render a world-3d scene");
        let lanes: Vec<&str> = node.children.iter().map(|child| child.key.as_str()).collect();
        for lane in ["meshes", "instances", "selection", "references", "environment", "fit"] {
            assert!(lanes.iter().any(|key| key.ends_with(&format!(".{lane}"))), "body {body_key} must publish the {lane} lane beside the spine, got {lanes:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn world_scene_binds_the_cad_interaction_domain_at_object_granularity() {
    let scene = forest_play_scene();
    let view = view(scene, CadPlayRuntime::default());
    let node = edit::build_world_scene_for_pane(&view, CadPaneId::Shape, shape::SURFACE_ID, None, CadDislocateOptions::default()).expect("scene surface");
    assert_eq!(node.key.as_str(), shape::SURFACE_ID);
    let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = &node.component else { panic!("world-3d surface node") };
    let world: semio_framework_plugin::World3dScene = semio_framework_ui_scene::decode(props).expect("world-3d spine");
    assert_eq!(world.domain_id.as_deref(), Some(CAD_INTERACTION_DOMAIN));
    assert_eq!(world.domain_granularity_id.as_deref(), Some(edit::CAD_WORLD_PICK_GRANULARITY));
}

/// 🛡️ Anti-regression guard for the "four empty windows" defect: every pane of the forest document must compose real
/// objects — from its model child, or from the genesis scene its child id names while no child is live — and
/// `build_world_scene_for_pane` must publish them. Checks the exact `instances_json` string it feeds
/// `MeshWindowKit::render` — the built scene's world-3d payload the defect left permanently empty — for every pane.
#[semio_framework_async_macros::async_test]
async fn forest_example_world_scene_has_non_empty_instances_for_every_pane() {
    let envelope = forest_view();
    for pane in CadPaneId::all() {
        let composed = envelope.panes.pane(pane);
        let (objects, geometry) = (composed.objects.as_slice(), composed.geometry(pane));
        assert!(!objects.is_empty(), "pane {pane:?} must have real objects, not the empty-defect slice");
        let instances_json = edit::world_instances_json(objects, &envelope);
        assert_ne!(instances_json, "[]", "pane {pane:?} instances_json must not be empty");
        let meshes_json = edit::world_meshes_json(objects, geometry);
        assert!(meshes_json.contains("\"data\""), "pane {pane:?} must inline its solids' tessellation into meshesJson");
        assert!(!meshes_json.contains("\"kind\""), "pane {pane:?} must not degrade authored solids to placeholder kinds");
    }
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_one_window_scoped_dislocate_utility() {
    let definition = create_cad_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, vec![CAD_DISLOCATE_UTILITY_ID]);
    // 🧰️ The framework auto-injects `setActiveUtility` as a View action once utilities are declared —
    // cad must NOT also declare it as an Mutation.
    let set_active_utility = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility auto-injected");
    assert_eq!(set_active_utility.kind, ActionKind::View);
    // 🚦️ Transform utilities gate the action panel while active (the default) — cad declares no
    // passive `allows_actions_while_active` view utilities.
    assert!(definition.utilities.iter().all(|utility| !utility.allows_actions_while_active));
    // 🧭️ Every world-3d pane owns its own Dislocate utility activation.
    for window in &definition.window_kinds {
        let refs: Vec<&str> = window.utilities.iter().map(|utility_ref| utility_ref.as_str()).collect();
        assert_eq!(refs, vec![CAD_DISLOCATE_UTILITY_ID], "window {} utilities", window.id);
    }
}

/// 🧱️ The manifest stitch: every window kind / panel tab the taxonomy nodes export lands in the
/// built `AppDefinition` with the same id, body key, surface kind and (empty) manifest measures the
/// pre-consolidation scalar `.window_kind(..)`/`.panel_tab(..)` calls produced — measures stay
/// config-derived per frame via `ArtifactApp::window_measures`, never frozen into the manifest.
#[semio_framework_async_macros::async_test]
async fn manifest_stitches_every_taxonomy_node_with_its_pre_migration_shape() {
    let definition = create_cad_app();
    let windows: Vec<(&str, &str)> = definition.window_kinds.iter().map(|window| (window.id.as_str(), window.body_key.as_str())).collect();
    assert_eq!(windows, vec![(shape::WINDOW_KIND_ID, shape::BODY_KEY), (building::WINDOW_KIND_ID, building::BODY_KEY), (energy::WINDOW_KIND_ID, energy::BODY_KEY), (structure_classic::WINDOW_KIND_ID, structure_classic::BODY_KEY),]);
    for window in definition.window_kinds.iter() {
        assert_eq!(window.surface_kind, semio_framework_ui_declaration::ui::SurfaceKind::World3d, "window {} surface kind", window.id);
        assert!(window.options.measures.is_empty(), "window {} must not freeze measures into the manifest", window.id);
    }
    let modes: Vec<&str> = definition.modes.iter().map(|mode| mode.id.as_str()).collect();
    assert_eq!(modes, vec![edit::CAD_PLAY_MODE_EDIT]);
    assert_eq!(definition.default_mode_id, edit::CAD_PLAY_MODE_EDIT);
    // 🕰️ The framework appends its own history tab after the app-declared ones.
    let panels: Vec<(&str, Option<&str>)> = definition.panel_tabs.iter().map(|tab| (tab.id(), tab.body_key.as_deref())).take(3).collect();
    assert_eq!(
        panels,
        vec![
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID, Some(document::CAD_PLAY_BODY_ARTIFACT)),
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID, Some(catalogue::CAD_PLAY_BODY_CATALOGUE)),
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID, Some(inspection::CAD_PLAY_BODY_PROPERTIES)),
        ]
    );
    let layout_json = semio_framework_pack_json::to_json_string(&edit::layout());
    for window_kind_id in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
        assert!(layout_json.contains(window_kind_id), "default quad layout must place {window_kind_id}: {layout_json}");
    }
    assert_eq!(definition.artifact_kinds.iter().map(|kind| kind.id.as_str()).collect::<Vec<_>>(), vec!["3d.cad"]);
}

/// 🖼️ A reference pick is app-owned runtime state, and it clears the framework `"cad"` object
/// selection through the sanctioned `InteractionWrite` lane so the inspection panel can show it.
#[semio_framework_async_macros::async_test]
async fn reference_pick_selects_the_reference_and_clears_the_cad_domain() {
    let scene = forest_play_scene();
    let selected = forest_working_scene().building_objects[0].id.clone();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let command = command_from_action("setReferenceSelection", Some(&json!({ "modelDefinitionId": CAD_MODEL_DEFINITION_ENERGY, "referenceId": "ref-concrete-forest" })));
    let mut ctx = CadDispatchCtx::new(selecting(&[selected.as_str()]), None, CadWorldWindowTransient::default());
    let emit = command.dispatch(&doc, &cfg, &mut ctx).expect("reference pick");
    assert!(emit.artifact_mutations.is_empty(), "a reference pick is never a document edit");
    let runtime = runtime_after(&emit, &config);
    assert_eq!(runtime.selected_reference_model_definition_id.as_deref(), Some(CAD_MODEL_DEFINITION_ENERGY));
    assert_eq!(runtime.selected_reference_id.as_deref(), Some("ref-concrete-forest"));
    assert_eq!(emit.interaction_writes.len(), 1);
    assert_eq!(emit.interaction_writes[0].domain, CAD_INTERACTION_DOMAIN);
    assert_eq!(emit.interaction_writes[0].merge, protocol::MergeMode::Subtractive, "an empty replace is a no-op; the live ids are subtracted");
    assert_eq!(emit.interaction_writes[0].targets.iter().map(|target| target.id.as_str()).collect::<Vec<_>>(), vec![selected.as_str()]);
    let mut idle = CadDispatchCtx::new(CadInteractionSnapshot::default(), None, CadWorldWindowTransient::default());
    assert!(command.dispatch(&doc, &cfg, &mut idle).expect("reference pick").interaction_writes.is_empty(), "nothing selected, nothing to subtract");
    let view = view(scene, runtime);
    let panel = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(inspection::build_properties_panel(&view, cad_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), None, &semio_framework_plugin::TreeWindows::unhosted()).expect("panel"))).expect("projection");
    assert!(panel.contains("cad-play-inspector.reference.width.grow"), "{panel}");
}

#[semio_framework_async_macros::async_test]
async fn internal_and_plumbing_actions_excluded_from_palette() {
    let definition = create_cad_app();
    let hidden_actions = [
        "patchCadPlayReference",
        "setReferenceHidden",
        "setReferenceLocked",
        "engagementSubmit",
        "setNodeSelection",
        "setReferenceSelection",
        "referenceHover",
        "engagementInput",
        "engagementPossibleSelect",
        "engagementRepeatLast",
        "engagementAbort",
        "worldPointerDown",
        "worldPointerMove",
        "setDislocateOption",
    ];
    for action_id in hidden_actions {
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("action {action_id} missing from manifest"));
        assert!(!action.in_palette, "internal action {action_id} must have in_palette: false");
    }

    let palette_user_actions = ["addObject", "deleteObject", "duplicateObject", "translateSelection", "rotateSelection", "scaleSelection"];
    for action_id in palette_user_actions {
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("user action {action_id} missing from manifest"));
        assert!(action.in_palette, "user action {action_id} must have in_palette: true");
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_input_and_possible_engagements_present() {
    let mut app = new_app().await;
    let engagements = app.window_engagements(&four_pane_view()).await;
    let shape = engagements.get(shape::WINDOW_KIND_ID).expect("shape engagement");
    assert!(shape.input.is_some());
    assert!(shape.possible_engagements.as_ref().is_some_and(|rows| !rows.is_empty()));
    close(&mut app);
}

/// 🕹️ The interaction-view threading law for window chrome: the HUD's `cad-status` row counts the
/// live `"cad"` domain selection `window_engagements_with_request_context` threads in, so it can never
/// disagree with what the world scene paints. Before this was threaded the row read `0` forever.
#[semio_framework_async_macros::async_test]
async fn the_engagement_hud_counts_the_threaded_cad_selection() {
    let labels = cad_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let count_of = |interaction: CadInteractionSnapshot| -> String {
        let view = view_with_interaction(forest_play_scene(), CadPlayRuntime::default(), interaction);
        let engagement = shape::engagement(&view, labels);
        engagement.status.expect("cad status rows").into_iter().find(|row| row.id == "cad-status").expect("cad-status row").text
    };
    assert!(count_of(CadInteractionSnapshot::default()).starts_with('0'), "an empty cad domain must report zero selected");
    let selected = CadInteractionSnapshot { granularity: "object".into(), ids: vec!["1".into(), "2".into()], anchor_id: None, hovered_ids: Vec::new() };
    assert!(count_of(selected).starts_with('2'), "the HUD must count the threaded cad selection");
}

#[semio_framework_async_macros::async_test]
async fn window_engagements_registered_for_all_four_panes() {
    let mut app = new_app().await;
    let engagements = app.window_engagements(&four_pane_view()).await;
    for window_kind in CAD_PANE_WINDOW_KIND_IDS {
        assert!(engagements.contains_key(window_kind), "missing engagement for {window_kind}");
    }
    close(&mut app);
}

/// 🙈️ LAW: a reference row's set-verbs are idempotent by value — `setReferenceHidden`/`setReferenceLocked` with the value
/// the row's target asks for emit exactly one reference edit, and replaying them against the edited document emits none,
/// so a stale second click leaves one history row's effect; a set-verb without its value is refused at the command
/// boundary, never defaulted into a flip.
#[test]
fn reference_set_verbs_are_idempotent_by_value_and_refuse_a_missing_value() {
    let scene = forest_play_scene();
    let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference").clone();
    let history = empty_history();
    let config = CadConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let identity = json!({ "modelDefinitionId": CAD_MODEL_DEFINITION_ENERGY, "referenceId": reference.id.as_str() });
    for (verb, args, state) in [
        ("setReferenceHidden", json!({ "hidden": !reference.hidden, "modelDefinitionId": CAD_MODEL_DEFINITION_ENERGY, "referenceId": reference.id.as_str() }), (|reference: &CadReference| reference.hidden) as fn(&CadReference) -> bool),
        ("setReferenceLocked", json!({ "locked": !reference.locked, "modelDefinitionId": CAD_MODEL_DEFINITION_ENERGY, "referenceId": reference.id.as_str() }), (|reference: &CadReference| reference.locked) as fn(&CadReference) -> bool),
    ] {
        assert!(<CadPlayApp as ArtifactEditor>::command_from_action(verb, Some((semio_framework_pack_json::to_dsl_value(&identity)).clone()), None).is_err(), "{verb} without its value must be refused");
        let command = command_from_action(verb, Some(&args));
        let mut ctx = CadDispatchCtx::new(CadInteractionSnapshot::default(), None, CadWorldWindowTransient::default());
        let first = command.dispatch(&ArtifactView::new(&scene, &history), &cfg, &mut ctx).unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        assert_eq!(first.artifact_mutations.len(), 1, "{verb} with the inverse value is exactly one reference edit");
        let outcome = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&first.artifact_mutations[0], &scene);
        let edited = protocol::apply_diff(outcome.diff(), &scene).expect("the reference edit applies");
        let flagged = edited.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.iter().find(|entry| entry.id == reference.id)).expect("the reference survives its flag edit");
        assert_eq!(state(flagged), !state(&reference), "{verb} reaches the value the row asked for");
        let replay = command.dispatch(&ArtifactView::new(&edited, &history), &cfg, &mut ctx).unwrap_or_else(|fault| panic!("{verb} replay: {fault:?}"));
        assert!(replay.artifact_mutations.is_empty(), "replaying {verb} with the same value emits no second edit");
    }
}

/// 🎯️ LAW: a reference row carries ONE target — the reference, the inverse of each flag, and `setReferenceSelection` as its
/// activation — and its two toggles name only their set-verbs, so the row and its toggles dispatch against the same target.
#[test]
fn a_reference_row_names_its_set_verbs_over_one_target_that_asks_for_the_inverse() {
    let scene = forest_play_scene();
    let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference").clone();
    let row = document::reference_tree_item(CAD_MODEL_DEFINITION_ENERGY, &reference, cad_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))).expect("the reference row is admitted");
    let semio_framework_plugin::Component::TreeItem(props) = &row.component else { panic!("a reference row is a tree item") };
    let verbs: Vec<&str> = props.row_actions.iter().map(|action| action.verb.as_str()).collect();
    assert_eq!(verbs, ["setReferenceHidden", "setReferenceLocked"]);
    let target = props.target.as_ref().expect("the reference row carries one target");
    assert_eq!(target.activation.as_ref().map(|activation| activation.as_str()), Some("setReferenceSelection"));
    let asked = |flag: &str| target.args.as_ref().and_then(|args| args.iter().find(|(key, _)| key.as_str() == flag)).and_then(|(_, value)| match value {
        semio_framework_plugin::UiValue::Bool(value) => Some(value),
        _ => None,
    });
    assert_eq!((asked("hidden"), asked("locked")), (Some(!reference.hidden), Some(!reference.locked)), "the row asks for the inverse of hidden={} locked={}", reference.hidden, reference.locked);
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(row)).expect("the reference row retires");
}

#[semio_framework_async_macros::async_test]
async fn forest_example_includes_reference_overlay() {
    let scene = forest_play_scene();
    let references = edit::world_references_json(&scene, CadPaneId::Shape).expect("references");
    assert!(references.contains("ref-concrete-forest"));
}

#[semio_framework_async_macros::async_test]
async fn typology_extent_derives_from_authored_geometry() {
    let scene = forest_working_scene();
    let column = scene.building_objects.iter().find(|object| object.typology == "building.building.column").expect("column object");
    let extent = column.extent.expect("column extent derived from geometry");
    assert!(extent[2] > 0.05, "authored column height should be measurable");
    assert_ne!(extent, CAD_DEFAULT_TYPOLOGY_EXTENT, "should differ from the universal fallback");
}
//#endregion 🔖️Render
