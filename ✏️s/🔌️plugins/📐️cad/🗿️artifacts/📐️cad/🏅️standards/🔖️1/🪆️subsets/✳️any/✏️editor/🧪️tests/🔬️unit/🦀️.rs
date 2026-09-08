
use super::testkit::*;
use super::*;
use crate::standards::v1::subsets::any::io::scene_from_spatial_payload;
use crate::standards::v1::subsets::any::schema::inferences::{
    CAD_DEFAULT_TYPOLOGY_EXTENT, CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX, CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX, CAD_FOREST_REFERENCE_PLANE_Z, CAD_FOREST_REFERENCE_WIDTH_WORLD, CAD_FOREST_REFERENCE_Y_OFFSET_RATIO, align_mesh_to_fixture_centroid,
    default_document, object_mesh_data, run_derive_from_geometry,
};
use crate::{CAD_PLAY_DOCUMENT_SCHEMA, CadNode, empty_cad_snapshot};
use semio_framework_plugin::{ActionKind, AppActionRegistry, EditorApp, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
use store::{Backbone, BackboneMessage, MemoryBackbone};

//#region 🔖️Fixtures
/// ⚖️ One value per `app_commands!` row (plus a `None`-everywhere twin for every row with
/// `Option` fields) — the closed set the wire laws below iterate. Captured from the
/// pre-consolidation `CadCommand` enum, ticket
/// `26/08/05/CAD-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`.
pub(crate) fn every_command() -> Vec<CadCommand> {
    vec![
        CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }),
        CadCommand::AddObject(add_object::AddObject { typology: None }),
        CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: Some("1.5".into()), delta: Some(2.5) }),
        CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: None, delta: None }),
        CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: vec!["object-1".into(), "object-2".into()], field: "label".into(), value: Some("Renamed".into()), delta: Some(0.25) }),
        CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: Vec::new(), field: "label".into(), value: None, delta: None }),
        CadCommand::DeleteObject(delete_object::DeleteObject { object_id: "object-1".into() }),
        CadCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: "object-1".into() }),
        CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }),
        CadCommand::RenameNode(rename_node::RenameNode { node_id: "node-1".into(), value: "Renamed".into() }),
        CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec!["object-1".into()], dx: 1.0, dy: -2.0, dz: 3.5 }),
        CadCommand::RotateSelection(rotate_selection::RotateSelection { object_ids: vec!["object-1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.57 }),
        CadCommand::ScaleSelection(scale_selection::ScaleSelection { object_ids: vec!["object-1".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
        CadCommand::ApplyTransformation(apply_transformation::ApplyTransformation { qid: "spatial.shape.from_geometry".into() }),
        CadCommand::ImportCadFile(import_cad_file::ImportCadFile { name: "triangle.obj".into(), payload: "data:model/obj;base64,AAAA".into() }),
        CadCommand::PatchCadPlayReference(patch_cad_play_reference::PatchCadPlayReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), field: "widthWorld".into(), value: Some("8".into()), delta: Some(0.5) }),
        CadCommand::PatchCadPlayReference(patch_cad_play_reference::PatchCadPlayReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), field: "hidden".into(), value: None, delta: None }),
        CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: Some("shape".into()) }),
        CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: None }),
        CadCommand::FocusModelDefinition(focus_model_definition::FocusModelDefinition { model_definition_id: "aec.building".into() }),
        CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-cut-concrete-forest-left".into() }),
        CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: Some("shape".into()), surface_id: Some("cad.play.scene3d/shape".into()), x: Some(1.0), y: Some(2.0), z: Some(3.0) }),
        CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: None, surface_id: None, x: None, y: None, z: None }),
        CadCommand::SetCamera(set_camera::SetCamera { pane: Some("cad.play.scene3d/building".into()), camera: CadCamera::default() }),
        CadCommand::SetCamera(set_camera::SetCamera { pane: None, camera: CadCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], zoom: 2.0, fov: 60.0, ..CadCamera::default() } }),
        CadCommand::SetProjection(set_projection::SetProjection { pane: Some("cad.play.scene3d/shape".into()), field: Some("orthographicView".into()), value_str: Some("top".into()), value_num: Some(12.5), param: Some("fov".into()) }),
        CadCommand::SetProjection(set_projection::SetProjection { pane: None, field: None, value_str: None, value_num: None, param: None }),
        CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam { pane: Some("cad.play.scene3d/shape".into()), field: Some("fov".into()), value_str: Some("x".into()), value_num: Some(45.0), param: Some("fov".into()) }),
        CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam { pane: None, field: None, value_str: None, value_num: None, param: None }),
        CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: Some("building".into()), option: "rotate".into(), pressed: Some(false) }),
        CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: None, option: "move".into(), pressed: None }),
        CadCommand::SetNodeSelection(set_node_selection::SetNodeSelection { node_ids: vec!["node-1".into(), "node-2".into()] }),
        CadCommand::SetReferenceSelection(set_reference_selection::SetReferenceSelection { pane: Some("shape".into()), model_definition_id: Some("spatial.shape".into()), reference_id: Some("ref-1".into()) }),
        CadCommand::SetReferenceSelection(set_reference_selection::SetReferenceSelection { pane: None, model_definition_id: None, reference_id: None }),
        CadCommand::ReferenceHover(reference_hover::ReferenceHover { reference_id: Some("ref-1".into()) }),
        CadCommand::ReferenceHover(reference_hover::ReferenceHover { reference_id: None }),
        CadCommand::EngagementInput(engagement_input::EngagementInput { value: "SetHeight2.5".into(), pane: Some("shape".into()) }),
        CadCommand::EngagementInput(engagement_input::EngagementInput { value: String::new(), pane: None }),
        CadCommand::EngagementPossibleSelect(engagement_possible_select::EngagementPossibleSelect { pane: Some("shape".into()), possible_id: "primitive.box".into() }),
        CadCommand::EngagementPossibleSelect(engagement_possible_select::EngagementPossibleSelect { pane: None, possible_id: "primitive.box".into() }),
        CadCommand::EngagementRepeatLast(engagement_repeat_last::EngagementRepeatLast { pane: Some("shape".into()) }),
        CadCommand::EngagementRepeatLast(engagement_repeat_last::EngagementRepeatLast { pane: None }),
        CadCommand::EngagementAbort(engagement_abort::EngagementAbort {}),
        CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: Some(3.0), y: Some(4.0), z: Some(0.0) }),
        CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: None, y: None, z: None }),
        CadCommand::ToggleSun(toggle_sun::ToggleSun {}),
        CadCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
        CadCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
        CadCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 0.85 }),
        CadCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        CadCommand::SaveSelected(save_selected::SaveSelected {}),
        CadCommand::SaveInPlay(save_in_play::SaveInPlay {}),
        CadCommand::SaveCurrent(save_current::SaveCurrent { format: Some("step".into()) }),
        CadCommand::SaveCurrent(save_current::SaveCurrent { format: None }),
        CadCommand::LoadRawRequest(load_raw_request::LoadRawRequest {}),
    ]
}

/// 🧪️ The shell's example picker reaches the production bridge and produces the typed command
/// that replaces the CAD document instead of falling through to the framework-only action path.
#[semio_framework_async_macros::async_test]
async fn production_action_bridge_loads_the_declared_example() {
    let command = <CadPlayApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&json::to_dsl_value(&json!({ "exampleId": CAD_EXAMPLE_FOREST_LEFT })))).expect("declared example action");
    assert!(matches!(command, CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id }) if example_id == CAD_EXAMPLE_FOREST_LEFT));
    let contributions = <CadPlayApp as ArtifactEditor>::command_from_action("setContributions", Some(&json::to_dsl_value(&json!({ "json": "[{\"id\":\"cad\"}]" })))).expect("declared host command");
    assert!(matches!(contributions, CadCommand::SetContributions(set_contributions::SetContributions { json }) if json == "[{\"id\":\"cad\"}]"));
    assert!(<CadPlayApp as ArtifactEditor>::command_from_action("notACadAction", None).is_err());
}

#[test]
fn host_contributions_resolve_to_the_event_sourced_config_lane() {
    let mutation = <CadPlayApp as ArtifactEditor>::host_configuration_mutation("setContributions", Some(&json::to_dsl_value(&json!({ "json": "[{\"id\":\"cad\"}]" })))).expect("host configuration").expect("CAD contribution mutation");
    assert_eq!(mutation, CadConfigMutation::SetContributions { json: "[{\"id\":\"cad\"}]".into() });
    assert_eq!(<CadPlayApp as ArtifactEditor>::host_configuration_mutation("setActiveExample", None).expect("non-host action"), None);
    assert!(<CadPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some());
    assert!(<CadPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some());
    let factory = CadRetainedCommandJobFactory::new("s.cad.cad@1/*#editor");
    let expected_keys = CAD_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new("s.cad.cad@1/*#editor", *tool_id)).collect::<Vec<_>>();
    assert_eq!(ToolJobFactory::keys(&factory), expected_keys);
    assert_eq!(ToolJobFactory::payload_schema_id(&factory), CAD_RETAINED_COMMAND_SCHEMA);
    assert_eq!(ToolJobFactory::classification(&factory), InteractiveJobClassification::Migrated);
    assert_eq!(<CadRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, CAD_RETAINED_PUBLICATION_CONTRACTS);
    assert_eq!(<CadPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), CAD_RETAINED_TOOL_IDS.len());
}

#[semio_framework_async_macros::async_test]
async fn retained_cad_presence_close_empty_lanes_have_exact_owners() {
    let fixture: Value = json::parse(include_str!("../../👥️presence/🧪️retirement.json")).unwrap();
    let maximum_items = fixture["grant"]["maximumItems"].as_u64().unwrap() as usize;
    let maximum_bytes = fixture["grant"]["maximumBytes"].as_u64().unwrap() as usize;
    let envelope = store::create_document_envelope::<NoDraft, NoDraftMutation>("draft.empty", "cad-draft-close", NoDraft::default(), None);
    let mut draft = store::DraftStore::new(envelope).await.unwrap();
    draft.install_member_store_owners_exact(<CadPlayApp as ArtifactEditor>::build_draft_store_owners().unwrap());
    let mut disposer = <CadPlayApp as ArtifactEditor>::build_draft_store_disposer().unwrap();
    for turn in 0..100_000 {
        match disposer.close_step(&mut draft, maximum_items, maximum_bytes).unwrap() {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= maximum_items && released_bytes <= maximum_bytes),
            semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("empty CAD draft close blocked: {reason}"),
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("fresh CAD fixture unexpectedly awaits external input: {reason}"),
            semio_framework_plugin::PluginCloseStep::Complete => break,
        }
        assert!(turn < 99_999);
    }
    assert!(disposer.terminal_is_empty(&draft));
    let mut transient = store::TransientStore::new(semio_framework_plugin::NoTransient::default());
    let mut disposer = <CadPlayApp as ArtifactEditor>::build_transient_store_disposer().unwrap();
    assert_eq!(disposer.close_step(&mut transient, 0, maximum_bytes).unwrap(), semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(disposer.close_step(&mut transient, maximum_items, maximum_bytes).unwrap(), semio_framework_plugin::PluginCloseStep::Complete);
    assert!(disposer.terminal_is_empty(&transient));
    eprintln!("[DEBUG] CAD exact NoDraft and NoTransient owners completed under 1-item/4096-byte grants");
}

#[semio_framework_async_macros::async_test]
async fn retained_factory_proofs_activate_the_real_cad_manifest_and_close_under_the_production_grant() {
    let fixture: Value = json::parse(include_str!("../../../🗄️retained-jobs/🔣️.json")).expect("CAD activation fixture");
    let activation = &fixture["activation"];
    let controller = activation["controller"].as_str().expect("controller");
    let bus = semio_framework::ActionBus::new();
    let definition = create_cad_app();
    let host_route = fixture["routes"].as_array().unwrap().iter().find(|route| route["id"] == "setContributions").unwrap();
    assert_eq!(host_route["disposition"], "migrated");
    let host_command = definition.commands.iter().find(|command| command.id == host_route["id"].as_str().unwrap()).expect("host command declaration");
    assert_eq!(host_command.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    let registry = AppActionRegistry::from_definition(&definition);
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<CadPlayApp>>::with_registry_on_bus(EditorApp::<CadPlayApp>::default(), registry, bus.clone()).await;
    assert_eq!(app.app_id().await, controller);
    assert_eq!(<CadPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), activation["proofRows"].as_u64().expect("proof rows") as usize);
    let mut admitted = std::collections::BTreeSet::new();
    for tool_id in CAD_RETAINED_TOOL_IDS {
        let admission = bus.admit_exact_wire(controller, *tool_id, CAD_RETAINED_COMMAND_SCHEMA, &[]).expect("real CAD factory is live before proof validation");
        assert_eq!(admission.factory_type_id, std::any::TypeId::of::<CadRetainedCommandJobFactory>());
        assert_eq!(admission.factory_type_name, std::any::type_name::<CadRetainedCommandJobFactory>());
        assert!(admitted.insert(*tool_id));
    }
    assert!(admitted.contains(activation["injectedTool"].as_str().expect("injected tool")));
    let maximum_items = activation["closeItems"].as_u64().expect("close items") as usize;
    let maximum_bytes = activation["closeBytes"].as_u64().expect("close bytes") as usize;
    let mut complete = false;
    for _ in 0..100_000 {
        match app.close_step(maximum_items, maximum_bytes).expect("bounded CAD close") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= maximum_items && released_bytes <= maximum_bytes);
            }
            semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("CAD constructor close blocked: {reason}"),
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("fresh CAD fixture unexpectedly awaits external input: {reason}"),
            semio_framework_plugin::PluginCloseStep::Complete => {
                complete = true;
                break;
            }
        }
    }
    assert!(complete && app.close_terminal_is_empty(), "the real mounted CAD owner must reach its empty terminal shell");
    eprintln!("[DEBUG] CAD activation joined {} exact app factory rows and completed bounded close", admitted.len());
}

#[test]
fn retained_config_store_preparation_is_bounded_exact_and_reversible() {
    let base = CadConfig::default();
    let mut next = base.clone();
    next.selected_node_ids.push("node-retained".into());
    let mutation = CadConfigMutation::Snapshot { config: next.clone() };
    let footprint = admit_cad_config_mutation(&mutation).expect("bounded CAD config mutation");
    assert_eq!(footprint.work_items, 1);
    let (post, inverse, forward) = prepare_cad_config(&base, mutation.clone()).expect("exact CAD config preparation");
    assert_eq!(post, next);
    assert_eq!(forward, mutation);
    assert_eq!(inverse, vec![CadConfigMutation::Snapshot { config: base.clone() }]);
    let oversized = CadConfigMutation::SetContributions { json: "x".repeat(CAD_CONFIG_STORE_MAXIMUM_BYTES + 1) };
    assert!(admit_cad_config_mutation(&oversized).is_err());
}

#[test]
fn retained_artifact_store_preparation_is_bounded_exact_and_reversible() {
    let base = empty_cad_snapshot();
    let node = CadNode { id: "node-retained".into(), label: "Retained".into(), kind: "group".into() };
    let mutation = CadMutation::CreateNode(crate::mutations::create_node::CreateNode { node: node.clone() });
    let footprint = admit_cad_artifact_mutation(&mutation).expect("bounded CAD Artifact mutation");
    assert_eq!(footprint.work_items, 1);
    let (post, inverse, forward) = prepare_cad_artifact(&base, mutation.clone()).expect("exact CAD Artifact preparation");
    assert_eq!(post.nodes, vec![node]);
    assert_eq!(forward, mutation);
    let mut restored = post;
    for operation in inverse {
        let outcome = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&operation, &restored);
        restored = protocol::MutationDiff::apply(outcome.diff(), &restored).expect("exact inverse");
    }
    assert_eq!(restored, base);
}

#[test]
fn retained_route_fixture_matches_the_exact_owner_manifest_and_laws() {
    let fixture: Value = json::parse(include_str!("../../../🗄️retained-jobs/🔣️.json")).expect("CAD retained route fixture");
    let routes = fixture.get("routes").and_then(Value::as_array).expect("route array");
    let route_ids = routes.iter().map(|route| route.get("id").and_then(Value::as_str).expect("route id")).collect::<std::collections::BTreeSet<_>>();
    let command_ids = every_command().iter().map(CadCommand::command_id).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(routes.len(), command_ids.len());
    assert_eq!(route_ids, command_ids);
    assert_eq!(fixture.get("admittedRoutes"), Some(&Value::Array(CAD_RETAINED_TOOL_IDS.iter().map(|id| Value::from(*id)).collect())));
    assert_eq!(fixture.pointer("/limits/closePageBytes").and_then(Value::as_u64), Some(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES as u64));
    assert_eq!(fixture.get("laws"), Some(&json!(["ownerLocal", "progress", "cancel", "freshness", "ackBeforeClose", "incrementalClose", "terminalEmpty"])));
    assert!(routes.iter().all(|route| {
        let id = route.get("id").and_then(Value::as_str);
        let disposition = route.get("disposition").and_then(Value::as_str);
        let blocker = route.get("blocker").and_then(Value::as_str);
        if id.is_some_and(|id| CAD_RETAINED_TOOL_IDS.contains(&id)) { disposition == Some("migrated") && blocker == Some("none") } else { disposition == Some("batchOnlyPendingRewrite") && blocker.is_some_and(|blocker| blocker != "none") }
    }));
    let manifest = create_cad_app();
    for tool_id in CAD_RETAINED_TOOL_IDS {
        let mut declarations = 0;
        for commands in std::iter::once(&manifest.commands).chain(manifest.modes.iter().map(|mode| &mode.commands)) {
            let matches = commands.iter().filter(|command| command.id == *tool_id).collect::<Vec<_>>();
            if matches.is_empty() {
                continue;
            }
            assert_eq!(matches.len(), 1, "{tool_id} requires exactly one declaration per command scope");
            assert_eq!(matches[0].semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id}");
            declarations += 1;
        }
        for window in &manifest.window_kinds {
            let actions = window.actions.iter().filter(|action| action.id == *tool_id).collect::<Vec<_>>();
            if actions.is_empty() {
                continue;
            }
            assert_eq!(actions.len(), 1, "{tool_id} requires exactly one declaration per window scope");
            assert_eq!(actions[0].semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id}");
            declarations += 1;
        }
        assert!(declarations > 0, "{tool_id} requires a manifest command or window declaration");
    }
    assert_eq!(manifest.window_kinds.iter().flat_map(|window| &window.actions).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).map(|action| action.semantics.execution.interactive_job), Some(InteractiveJobClassification::Migrated));
    assert!(manifest.window_kinds.iter().flat_map(|window| &window.actions).filter(|action| route_ids.contains(action.id.as_str())).all(|action| {
        let expected = if CAD_RETAINED_TOOL_IDS.contains(&action.id.as_str()) { InteractiveJobClassification::Migrated } else { InteractiveJobClassification::BatchOnlyPendingRewrite };
        action.semantics.execution.interactive_job == expected
    }));
}

/// ⚖️ LAW: the one-action spot check above is not enough — this is the framework's own harness,
/// which walks EVERY action this app's window kinds render, stages each one's declared args the way
/// the host does, and skips the framework-injected ids. It is what catches the next
/// `setActiveExample`: chrome that declares an action no command row backs.
#[semio_framework_async_macros::async_test]
async fn every_rendered_action_bridges_through_the_framework_harness() {
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<CadPlayApp>>(cad_app_manifest_for_testkit).await;
}

/// ⚖️ Text and binary are two projections of the same command, and every printed line starts with
/// that row's wire keyword — the guard that a command decomposition cannot silently rename a row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_own_wire_keyword() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let printed = protocol::OpText::print_op(&command);
        let keyword = printed.split_whitespace().next().unwrap_or_default().to_string();
        assert!(!keyword.is_empty(), "a command must print a leading wire keyword: {printed:?}");
        let decoded = <CadCommand as protocol::OpText>::parse_op(&printed).expect("re-parse");
        assert_eq!(decoded, command, "printed line must re-parse to the same command: {printed:?}");
    }
}

/// 🔒️ Wire-format pin: the exact bytes of rows whose `Option` fields make `None`/`Some` distinct
/// wire cases, copied out of the pre-consolidation baseline dump.
#[test]

fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &CadCommand| -> String { protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect() };
    assert_eq!(hex(&CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) })), "0100011b7370617469616c2e73686170652e7072696d69746976652e626f7801000600");
    assert_eq!(hex(&CadCommand::AddObject(add_object::AddObject { typology: None })), "01000000");
    assert_eq!(
        hex(&CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: Some("1.5".into()), delta: Some(2.5) })),
        "01010303312e35086f626a6563742d31086f726967696e2e780400060101060202060003050000000000000440"
    );
    assert_eq!(hex(&CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: None, delta: None })), "010102086f626a6563742d31086f726967696e2e7802000600010601");
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `SetHover`/`WorldPick` and their
    // byte pins are DELETED (those commands no longer exist); every OTHER row's ordinal shifted
    // too (this enum's binary encoding is a plain row-position ordinal — greenfield, no
    // back-compat expected), so the tail-of-enum pins (`EngagementAbort`/`ToggleSun`/
    // `SaveSelected`/`LoadRawRequest`) are dropped rather than hand-recomputed; the exact-wire-key
    // guard for every row (including these) already lives in
    // `every_command_round_trips_text_and_binary_under_its_own_wire_keyword` above.
}

#[semio_framework_async_macros::async_test]
async fn forest_example_uses_per_object_brep_meshes() {
    let scene = forest_working_scene();
    let runtime = CadPlayRuntime::default();
    let json = edit::world_instances_json(&scene.building_objects, &runtime);
    assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
    let meshes = edit::world_meshes_json(&scene.building_objects, scene.building_geometry.as_ref());
    assert!(meshes.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
    assert!(!meshes.contains("🧊️hexagonal-cut-concrete-forest-left.glb"));
    assert!(scene.building_objects.len() > 5);
    assert!(scene.building_objects.iter().all(|object| object.solid_handle.is_some()));
}

#[semio_framework_async_macros::async_test]
async fn quad_panes_each_populate_distinct_objects() {
    let scene = forest_working_scene();
    assert!(!scene.objects.is_empty(), "shape pane");
    assert!(!scene.building_objects.is_empty(), "building pane");
    assert!(!scene.energy_objects.is_empty(), "energy pane");
    assert!(!scene.structure_classic_objects.is_empty(), "structure classic pane");
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_cut_concrete_forest_not_placeholder_box() {
    let scene = CadPlayApp::initial_snapshot();
    assert_eq!(scene.id, CAD_EXAMPLE_FOREST_LEFT);
    assert_eq!(scene.nodes.first().map(|node| node.label.as_str()), Some("Concrete Forest Left"), "must not be the placeholder 'Model' node");
    let working = forest_working_scene();
    assert_ne!(working.objects.first().map(|object| object.id.as_str()), Some("object-box-1"));
    assert!(!working.building_objects.is_empty(), "building pane must not be the empty default placeholder");
    assert!(!working.energy_objects.is_empty(), "energy pane must not be the empty default placeholder");
    assert!(!working.structure_classic_objects.is_empty(), "structure pane must not be the empty default placeholder");
    assert!(working.objects.iter().all(|object| object.solid_handle.is_some()));
}

#[semio_framework_async_macros::async_test]
async fn forest_energy_world_mesh_survives_scene_roundtrip() {
    let scene = forest_working_scene();
    let roundtrip: CadWorkingScene = json::from_json_str(&json::to_json_string(&scene)).expect("deserialize");
    let object = roundtrip.energy_objects.first().expect("energy object");
    let mesh = object_mesh_data(object, roundtrip.energy_geometry.as_ref());
    let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(min_z > 2.5, "energy world mesh min z {min_z}");
    let slab = roundtrip.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
    let slab_mesh = object_mesh_data(slab, roundtrip.structure_classic_geometry.as_ref());
    let slab_min_z = slab_mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(slab_min_z > 2.5, "structure world mesh min z {slab_min_z}");
}

#[semio_framework_async_macros::async_test]
async fn forest_references_use_xy_ground_plane_and_z_up() {
    let scene = forest_play_scene();
    let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference");
    assert_eq!(reference.origin[2], CAD_FOREST_REFERENCE_PLANE_Z, "reference must stay on the CAD ground datum");
    assert!((reference.origin[0] - (-9.7)).abs() < 1e-9, "reference x {} should be base + 50% width (right)", reference.origin[0]);
    let expected_y = -18.0 + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * (0.5 + CAD_FOREST_REFERENCE_Y_OFFSET_RATIO);
    assert!((reference.origin[1] - expected_y).abs() < 1e-9, "reference CAD y {} should be centered then moved +20% forward on the world plane", reference.origin[1]);
    let centered_y = -18.0 + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * 0.5;
    assert!(((reference.origin[1] - centered_y) - CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * 0.2).abs() < 1e-9, "the requested offset must affect CAD y only");
    assert_eq!(CAD_FOREST_REFERENCE_Y_OFFSET_RATIO, 0.2);
    assert!(reference.locked, "example references default locked like puzzle 3d");
    assert_eq!(reference.width_world, 28.6);
}

#[semio_framework_async_macros::async_test]
async fn align_mesh_to_fixture_centroid_corrects_drifted_surface() {
    let scene = forest_working_scene();
    let geometry = scene.energy_geometry.as_ref().expect("energy geometry");
    let object = scene.energy_objects.first().expect("energy object");
    let mut mesh = object_mesh_data(object, Some(geometry));
    for vertex in mesh.positions.as_chunks_mut::<3>().0 {
        vertex[2] = 0.0;
    }
    align_mesh_to_fixture_centroid(&mut mesh, geometry, &object.primitives);
    let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(min_z > 2.5, "aligned mesh min z {min_z}");
}

#[semio_framework_async_macros::async_test]
async fn forest_surface_meshes_fall_back_to_typology_extent_without_pane_geometry() {
    // ⚠️ CORRECTED (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
    // wave G4): this test used to assert the mesh stayed at its authored height even with no
    // `CadGeometry` in hand — that only worked because `cad_brep_kernel()` was a process-global
    // `BrepEngineHost` singleton, so `energy.solid_handle` (minted by an EARLIER, already-dropped
    // call to `forest_pane_bundle`) still resolved in whatever kernel `object_mesh_data` happened
    // to reach. `origin` on fixture-derived `CadObject`s is always `[0,0,0]`
    // (`objects_from_fixture_model`) — the authored height lived ONLY in the solid's own vertex
    // data, addressed by that handle. A `cad_brep_kernel()` is now a fresh, local `Brep::new()`
    // per call (doctrine tier-(d): never outlives the call that built it), so a handle from a
    // different call is honestly unresolvable, and — exactly like `mesh_from_glb`'s documented
    // gap elsewhere in this codebase — meshing falls back to the typology's default extent box
    // at the kernel's local origin instead of silently fabricating a placement it cannot know.
    let scene = forest_working_scene();
    let energy = scene.energy_objects.first().expect("energy object");
    let energy_mesh = object_mesh_data(energy, None);
    assert!(!energy_mesh.positions.is_empty(), "energy mesh must still be real geometry, just typology-shaped");
    let slab = scene.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
    let slab_mesh = object_mesh_data(slab, None);
    assert!(!slab_mesh.positions.is_empty(), "structure slab mesh must still be real geometry, just typology-shaped");
}

#[semio_framework_async_macros::async_test]
async fn cad_document_schema_matches_domain() {
    let scene = empty_cad_snapshot();
    assert_eq!(scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn default_example_and_forest_scene_parse_as_projections() {
    let default_json = json::to_json_string(&default_document());
    let default_scene: CadSnapshot = json::from_json_str(&default_json).unwrap();
    assert_eq!(default_scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
    let forest_json = json::to_json_string(&forest_play_scene());
    let forest_scene: CadSnapshot = json::from_json_str(&forest_json).unwrap();
    assert_eq!(forest_scene.id, CAD_EXAMPLE_FOREST_LEFT);
    assert!(!forest_working_scene().building_objects.is_empty());
}
//#endregion 🔖️Fixtures
//#region 🔖️Render
#[semio_framework_async_macros::async_test]
async fn renders_world_scene_for_each_pane() {
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let view_state = ViewModel::default();
    for body_key in [shape::BODY_KEY, building::BODY_KEY, energy::BODY_KEY, structure_classic::BODY_KEY] {
        let node = render_direct(&app, body_key, &doc, &CadConfig::default(), &view_state).expect("CAD UI assembly");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"), "body {body_key} should render a world-3d scene");
    }
}

/// 🛡️ Anti-regression guard for the "four empty windows" defect: `forest_play_document` must
/// actually populate `shape_model`/`building_model`/`energy_model`/`structure_classic_model`
/// (via `cad_document_pane_bundle`, carried as each handle's `ArtifactChild::local_owner`), and
/// `build_world_scene_for_pane`'s own pane resolver (`edit::cad_pane_working_scene`/
/// `edit::cad_pane_working_objects`) must read real objects back out of them instead of the
/// hardcoded empty slice the defect shipped with. Checks the exact `instances_json` string
/// `build_world_scene_for_pane` feeds `MeshWindowKit::render` — the built scene's world-3d
/// payload the defect left permanently empty — for every pane, not just via the lower-level
/// `world_instances_json(&scene.building_objects, ..)` shortcut `forest_example_uses_per_object_brep_meshes` uses.
#[semio_framework_async_macros::async_test]
async fn forest_example_world_scene_has_non_empty_instances_for_every_pane() {
    let document = forest_play_scene();
    for pane in CadPaneId::all() {
        let working_scene = edit::cad_pane_working_scene(&document, pane).unwrap_or_else(|| panic!("pane {pane:?} must resolve a local-owner working scene"));
        let (objects, _geometry) = edit::cad_pane_working_objects(&working_scene, pane);
        assert!(!objects.is_empty(), "pane {pane:?} must have real objects, not the empty-defect slice");
        let instances_json = edit::world_instances_json(objects, &CadPlayRuntime::default());
        assert_ne!(instances_json, "[]", "pane {pane:?} instances_json must not be empty");
        let meshes_json = edit::world_meshes_json(objects, _geometry);
        assert!(!meshes_json.contains(CAD_FALLBACK_MESH_KIND), "pane {pane:?} must render real brep meshes, not the universal fallback box");
    }
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_one_window_scoped_dislocate_utility() {
    let definition = create_cad_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, vec![CAD_DISLOCATE_UTILITY_ID]);
    // 🧰️ The framework auto-injects `setActiveUtility` as a View action once utilities are declared —
    // cad must NOT also declare it as an Mutation.
    let set_active_utility = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility auto-injected");
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
        assert_eq!(window.surface_kind, ui_wgpu::wgpu::SurfaceKind::World3d, "window {} surface kind", window.id);
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
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID, Some(document::CAD_PLAY_BODY_DOCUMENT)),
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID, Some(catalogue::CAD_PLAY_BODY_CATALOGUE)),
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID, Some(inspection::CAD_PLAY_BODY_PROPERTIES)),
        ]
    );
    let layout_json = json::to_json_string(&edit::layout());
    for window_kind_id in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
        assert!(layout_json.contains(window_kind_id), "default quad layout must place {window_kind_id}: {layout_json}");
    }
    assert_eq!(definition.artifact_kinds.iter().map(|kind| kind.id.as_str()).collect::<Vec<_>>(), vec!["3d.cad"]);
}

#[semio_framework_async_macros::async_test]
async fn internal_and_plumbing_actions_excluded_from_palette() {
    let definition = create_cad_app();
    let hidden_actions = [
        "patchCadPlayReference",
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
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("action {action_id} missing from manifest"));
        assert!(!action.in_palette, "internal action {action_id} must have in_palette: false");
    }

    let palette_user_actions = ["addObject", "deleteObject", "duplicateObject", "translateSelection", "rotateSelection", "scaleSelection"];
    for action_id in palette_user_actions {
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("user action {action_id} missing from manifest"));
        assert!(action.in_palette, "user action {action_id} must have in_palette: true");
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_input_and_possible_engagements_present() {
    let mut app = new_app().await;
    let engagements = app.window_engagements().await;
    let shape = engagements.get(shape::WINDOW_KIND_ID).expect("shape engagement");
    assert!(shape.input.is_some());
    assert!(shape.possible_engagements.as_ref().is_some_and(|rows| !rows.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn window_engagements_registered_for_all_four_panes() {
    let mut app = new_app().await;
    let engagements = app.window_engagements().await;
    for window_kind in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
        assert!(engagements.contains_key(window_kind), "missing engagement for {window_kind}");
    }
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
//#region 🔖️ViewModel
#[semio_framework_async_macros::async_test]
async fn gumball_config_fields_present_regardless_of_dislocate_activation() {
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh selection is
    // framework-owned now and `ArtifactApp::render` has no `InteractionView` (see
    // `gumball_active`'s own doc comment) — the gumball can never see a live selection at this
    // render boundary, so `gumballActive` stays `false` even with Dislocate active; the
    // transform-mode config fields still render regardless (client-side, harmless while inactive).
    let selection = edit::world_selection_json(&default_document(), &CadPlayRuntime::default(), Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(selection.contains("\"transformMode\":\"transform\""));
    assert!(selection.contains("\"moveAxes\":true"));
    assert!(selection.contains("\"rotate\":true"));
    assert!(selection.contains("\"scaleAxes\":false"));
    assert!(selection.contains("\"gumballActive\":false"));
    assert!(!selection.contains("\"gumballTarget\""));
}

/// 🎥️ `setCamera`/`setProjection`/`setProjectionParam` are `ActionKind::View` (see the `.view_action`
/// registrations below) — they must never emit a `CadMutation` (no VCS edit, no undo entry) and
/// instead write a coalesced `CadConfigMutation`, isolated per pane.
#[semio_framework_async_macros::async_test]
async fn set_camera_writes_config_not_mutations() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let emit = drive(&app, &scene, "setCamera", Some(json!({ "surfaceId": "cad.play.scene3d/building", "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 2.0, "fov": 60.0 } })));
    assert!(emit.artifact_mutations.is_empty(), "setCamera must not emit a VCS operation");
    assert!(!emit.config_mutations.is_empty(), "setCamera must write a config operation");
    let runtime = runtime_after(&emit, &CadConfig::default());
    assert_eq!(cad_pane_camera_runtime(&runtime, CadPaneId::Building).zoom, 2.0);
    assert_eq!(cad_pane_camera_runtime(&runtime, CadPaneId::Shape).zoom, 1.0, "panes stay isolated");
}

#[semio_framework_async_macros::async_test]
async fn gumball_inactive_without_selection() {
    let selection = edit::world_selection_json(&default_document(), &CadPlayRuntime::default(), Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
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
    let view_state = ViewModel { active_utility_id: Some(CAD_DISLOCATE_UTILITY_ID.into()), ..ViewModel::default() };
    let node = render_direct(&app, shape::BODY_KEY, &doc, &config, &view_state).expect("CAD UI assembly");
    let json = serde_json::to_string(&node).unwrap();
    // The world selection blob is embedded as an escaped JSON string inside the scene node.
    assert!(json.contains(r#"transformMode\":\"transform"#), "render sources Dislocate from ViewModel.active_utility_id");
}

#[semio_framework_async_macros::async_test]
async fn dislocate_utility_is_scoped_by_each_window_view_context() {
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh selection is
    // framework-owned now and `ArtifactApp::render` has no `InteractionView` (see
    // `edit::gumball_active`'s own doc comment) — the gumball can never be live-active at this
    // render boundary, in any pane; the transform-mode config fields still render regardless.
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig::default();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let shape_view = ViewModel { active_utility_id: Some(CAD_DISLOCATE_UTILITY_ID.into()), ..ViewModel::default() };
    let building_view = ViewModel { active_utility_id: None, ..ViewModel::default() };
    let shape = render_direct(&app, shape::BODY_KEY, &doc, &config, &shape_view).expect("CAD UI assembly");
    let building = render_direct(&app, building::BODY_KEY, &doc, &config, &building_view).expect("CAD UI assembly");
    let shape_json = serde_json::to_string(&shape).unwrap();
    let building_json = serde_json::to_string(&building).unwrap();
    assert!(shape_json.contains(r#"gumballActive\":false"#));
    assert!(shape_json.contains(r#"transformMode\":\"transform"#));
    assert!(building_json.contains(r#"gumballActive\":false"#));
    assert!(!building_json.contains(r#"transformMode\":\"transform"#));
}

#[semio_framework_async_macros::async_test]
async fn context_menu_resolves_labels_from_the_registry() {
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `context_menu` is no longer
    // selection-gated — `ArtifactApp::context_menu` has no `InteractionView` parameter, so it
    // can no longer tell whether anything is selected (see its own doc comment); it always
    // shows the transform/duplicate/delete section now.
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let registry = AppActionRegistry::from_definition(&create_cad_app());
    let config = CadConfig::default();

    let view_state = ViewModel::default();
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

    let view_state = ViewModel::default();
    let items = context_menu_direct(&app, &doc, &config, &view_state, &registry);

    assert!(items.len() <= 9, "top-level context menu should stay progressively disclosed: {items:?}");
    assert_eq!(items.last().map(|item| item.id.as_str()), Some("deleteObject"), "deleteObject must stay the trailing item: {items:?}");
    assert_eq!(items.last().and_then(|item| item.destructive), Some(true), "trailing deleteObject must be marked destructive: {items:?}");
}

/// @emoji 🎛️ Dislocate move/rotate options are now keyed by PANE (`CadConfig::dislocate_shape`/
/// `dislocate_building`/…), not by an arbitrary host-pushed window-instance id — the direct
/// replacement for the pre-B1 per-window-instance isolation test.
#[semio_framework_async_macros::async_test]
async fn dislocate_move_and_rotate_options_are_per_pane() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let emit = drive(&app, &scene, "setDislocateOption", Some(json!({ "pane": "building", "option": "rotate", "pressed": false })));
    let config = config_after(&emit, &CadConfig::default());
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let view_state = ViewModel::default();
    let measures = window_measures_direct(&app, &doc, &config, &view_state);
    let rotate_pressed = |window_id: &str| {
        measures.get(window_id).and_then(|items| {
            items.iter().find_map(|measure| match measure {
                WindowMeasure::Group { id, children, .. } if id == "cad-play-utility-options-dislocate" => children.iter().find_map(|child| match child {
                    WindowMeasure::Toggle { id, pressed, .. } if id == "cad-dislocate-rotate" => Some(*pressed),
                    _ => None,
                }),
                _ => None,
            })
        })
    };
    assert_eq!(rotate_pressed(shape::WINDOW_KIND_ID), Some(true));
    assert_eq!(rotate_pressed(building::WINDOW_KIND_ID), Some(false));
}

#[semio_framework_async_macros::async_test]
async fn engagement_hud_no_longer_carries_utility_switcher_options() {
    let mut app = new_app().await;
    let engagements = app.window_engagements().await;
    for engagement in engagements.values() {
        assert!(engagement.options.is_none(), "utility switching now lives in the framework utility bar, not the engagement HUD");
    }
}

#[semio_framework_async_macros::async_test]
async fn utility_switch_has_no_plugin_command_or_config_lane() {
    assert!(<CadPlayApp as ArtifactEditor>::command_from_action(SET_ACTIVE_UTILITY_ACTION_ID, None).is_err());
    assert!(!CAD_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
    assert!(!every_command().iter().any(|command| command.command_id() == SET_ACTIVE_UTILITY_ACTION_ID));
}

#[semio_framework_async_macros::async_test]
async fn sun_measures_registered_for_all_four_panes_and_default_off() {
    let app = CadPlayApp::default();
    let base_config = CadConfig::default();
    assert!(!base_config.sun.enabled, "sun must be off by default");
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let view_state = ViewModel::default();
    let measures = window_measures_direct(&app, &doc, &base_config, &view_state);
    for window_kind in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
        assert!(measures.contains_key(window_kind), "missing sun measures for {window_kind}");
    }
    let emit = drive(&app, &scene, "toggleSun", None);
    let runtime = runtime_after(&emit, &base_config);
    assert!(runtime.sun.enabled);
}

// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `worldPick`/`setHover`/`setSelection`
// and their round-trip tests are DELETED, not migrated — mesh object/vertex/edge/face
// selection AND hover are now the framework-owned `"cad"` interaction domain, dispatched
// through the auto-injected `interactionSelect`/`interactionHover` verbs (never app-declared)
// and tested once, centrally, by the framework's own `semio-framework-plugin` suite.

//#endregion 🔖️ViewModel
//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn add_object_action_is_a_documented_no_op() {
    // ⚠️ `addObject` is a documented no-op pending the child-dispatch seam (see
    // `commands/🧱️object/component.rs`'s module doc) — this locks in the honest current
    // behavior (zero artifact mutations) rather than the pre-migration "grows the object list"
    // claim, which no longer applies now `CadSnapshot` carries no inline objects. Selection is
    // out of scope here too (framework-owned now, unreachable from `handle()`).
    let app = CadPlayApp::default();
    let scene = default_document();
    let emit = drive(&app, &scene, "addObject", Some(json!({ "typology": "building.building.column" })));
    assert!(emit.artifact_mutations.is_empty(), "addObject is a documented no-op until the child-dispatch seam lands");
}

#[semio_framework_async_macros::async_test]
async fn add_object_through_wrapper_is_a_documented_no_op() {
    let mut app = new_app().await;
    let before = json::to_json_string(&app.snapshot().expect("snapshot"));
    app.dispatch_typed(CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }), &meta("local")).await.expect("add object dispatch");
    let after = json::to_json_string(&app.snapshot().expect("snapshot"));
    assert_eq!(before, after, "addObject is a documented no-op until the child-dispatch seam lands");
}

#[semio_framework_async_macros::async_test]
async fn focus_model_definition_emits_document_operation() {
    let mut app = new_app().await;
    app.dispatch_typed(CadCommand::FocusModelDefinition(focus_model_definition::FocusModelDefinition { model_definition_id: "aec.building".into() }), &meta("local")).await.expect("focus model definition");
    assert_eq!(app.snapshot().expect("snapshot").active_model_definition_id, "aec.building");
}

#[semio_framework_async_macros::async_test]
async fn derive_transformation_populates_energy_pane() {
    // ⚠️ `apply_transformation_mutations` is a documented no-op pending the child-dispatch seam
    // (see its own doc comment in this file) — this instead exercises the real derive algorithm
    // directly (`run_derive_from_geometry`), the pure function `applyTransformation` will call
    // once that seam exists. `make_object_for_typology` already built and dropped its own local
    // `cad_brep_kernel()` internally to mint the object's solid handle; `cad_brep_kernel()` is a
    // fresh `Brep::new()` per call (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
    // wave G4 — no shared lock, no reentrancy concern), so the kernel built HERE for
    // `run_derive_from_geometry` is its own independent instance.
    let object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let derived = run_derive_from_geometry(&mut kernel, &[object], "energy");
    assert!(!derived.is_empty());
    assert!(derived.iter().any(|object| object.typology.starts_with("energy.energy.")));
}

#[semio_framework_async_macros::async_test]
async fn forest_transformation_uses_live_shape_pane() {
    // ⚠️ CORRECTED (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
    // wave G4): this test used to derive from `forest_working_scene().objects` and compare
    // against a single live box, relying on the forest fixture's `solid_handle`s resolving into
    // whatever kernel `run_derive_from_geometry` reached — only true under the deleted
    // process-global `BrepEngineHost` singleton. `solid_for_object` (the derive's per-object
    // solid builder) has never applied `object.origin` — fixture objects carry `origin:
    // [0,0,0]` regardless (`objects_from_fixture_model`) — so once a handle stops resolving it
    // falls back to an extent+typology-only box built at the kernel's local origin, and two
    // fixture objects sharing that origin fuse into a materially different (and no longer
    // fixture-distinguishing) hull. The real, still-true property — output tracks LIVE INPUT,
    // not a memoized static result — is verified here directly against extent, the one field
    // that DOES still flow through `solid_for_object`'s fallback path honestly.
    let live_box = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let box_derived = run_derive_from_geometry(&mut kernel, &[live_box], "energy");
    assert!(!box_derived.is_empty(), "a live box must derive at least a hull");

    let live_wall = make_object_for_typology("building.building.wall", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let wall_derived = run_derive_from_geometry(&mut kernel, &[live_wall], "energy");
    assert!(!wall_derived.is_empty(), "a live wall panel must derive at least a hull");

    let wall_typologies: Vec<&str> = wall_derived.iter().map(|object| object.typology.as_str()).collect();
    let box_typologies: Vec<&str> = box_derived.iter().map(|object| object.typology.as_str()).collect();
    assert_ne!(
        wall_typologies, box_typologies,
        "a thin wall panel and a cube must classify their dominant faces differently, proving the derive tracks the LIVE input's real shape, not a memoized result:\n  box:  {box_typologies:?}\n  wall: {wall_typologies:?}"
    );
}

#[semio_framework_async_macros::async_test]
async fn save_selected_emits_download_effect() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig::default();
    let emit = drive_with_config(&app, &scene, "saveSelected", None, &config);
    assert!(emit.artifact_mutations.is_empty(), "export must not mutate the document");
    assert_eq!(emit.effects.len(), 1);
    match &emit.effects[0] {
        Effect::DownloadMediaExport { filename, data, .. } => {
            assert_eq!(filename, "cad.selected.spatial.dsl");
            assert!(data.contains("activeModelDefinitionId"));
        }
        other => panic!("expected DownloadMediaExport, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn load_raw_request_emits_file_open_effect() {
    let app = CadPlayApp::default();
    let emit = drive(&app, &default_document(), "loadRawRequest", None);
    match &emit.effects[0] {
        Effect::RequestFileOpen { import_action, read_as, .. } => {
            assert_eq!(import_action, "importCadFile");
            assert_eq!(read_as.as_deref(), Some("dataUrl"));
        }
        other => panic!("expected RequestFileOpen, got {other:?}"),
    }
}
//#endregion 🔖️Operations
//#region 🔖️Engagement
#[semio_framework_async_macros::async_test]
async fn engagement_starts_box_interaction_session() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    let runtime = runtime_after(&emit, &config);
    assert!(runtime.engagement_session.is_some());
}

#[semio_framework_async_macros::async_test]
async fn world_pointer_move_updates_live_preview_without_committing_or_emitting_mutations() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    let config = config_after(&emit, &config);

    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config);
    assert!(emit.artifact_mutations.is_empty(), "a pointer move must not emit any document operation");
    let runtime = runtime_after(&emit, &config);
    let session = runtime.engagement_session.as_ref().expect("session still active");
    assert_eq!(session.state, "first_corner", "pointer.move must not change state");
    assert_eq!(session.context.get("cursor"), Some(&json::to_dsl_value(&json!([3.0, 4.0, 0.0]))));
}

//#region 🔖️GesturePreview
fn preview_operation(app_instance_id: u32) -> CadPreviewOperationIdentity {
    CadPreviewOperationIdentity { app_instance_id, parent_document_id: "cad-preview-document".into(), operation_id: 41, operation_generation: 3, canonical_base_revision: "cd".repeat(32) }
}

fn persisted_preview_stamp(config: &CadConfig) -> CadPreviewStamp {
    CadPreviewStamp { operation: json::from_json_str(config.engagement_preview_operation_json.as_ref().expect("persisted operation identity")).expect("valid persisted operation identity"), generation: config.engagement_preview_generation }
}

fn spatial_scene_import_args() -> Value {
    let file_text = json!({
        "schema": "spatial.model",
        "revision": 1,
        "modelDefinitionId": "spatial.shape",
        "objects": [{
            "id": "object-preview-transition",
            "label": "Preview transition",
            "typology": "spatial.shape.primitive.box",
            "visible": true,
            "locked": false,
            "origin": [0.0, 0.0, 0.0],
            "primitives": []
        }]
    })
    .to_string();
    json!({ "payload": file_text, "name": "preview-transition.spatial.json" })
}

/// 🔬️ CW7 preview-law seam: `CadPlayApp::gesture_preview` reads `CadEngagementScratch` only, never
/// `CadSnapshot`/`CadMutation` — driven through the real `worldPointerMove` handler (the natural
/// per-tick gesture handler) via the existing `drive` helper, config threaded explicitly across
/// calls (the pure `CadPlayApp` no longer holds any of this state itself).
#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_none_without_a_live_engagement_session() {
    let app = CadPlayApp::default();
    assert!(app.gesture_preview(&CadConfig::default()).is_none(), "no live engagement session, nothing to preview");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_reflects_the_live_rubber_band_preview_and_clears_on_abort() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    let config = config_after(&emit, &config);

    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config);
    let config = config_after(&emit, &config);
    let first = app.gesture_preview(&config).expect("a live engagement session is previewable");
    let value: Value = json::parse_bytes(&first.payload).expect("payload is valid json");
    assert_eq!(value["context"]["cursor"], json!([3.0, 4.0, 0.0]));

    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [5.0, 6.0, 0.0] })), &config);
    let config = config_after(&emit, &config);
    let second = app.gesture_preview(&config).expect("still live mid-gesture");
    assert_eq!(second.stamp.operation, first.stamp.operation);
    assert_eq!(second.stamp.generation, first.stamp.generation + 1, "the persisted preview generation advances exactly once per changed checkpoint");
    assert!(second.is_fresher_than(&first.stamp));
    let value_after_second: Value = json::parse_bytes(&second.payload).expect("payload is valid json");
    assert_eq!(value_after_second["context"]["cursor"], json!([5.0, 6.0, 0.0]), "preview tracks the live cursor, not the gesture start");

    let emit = drive_with_config(&app, &scene, "engagementAbort", None, &config);
    let config = config_after(&emit, &config);
    assert!(app.gesture_preview(&config).is_none(), "the engagement session was aborted: nothing left to preview");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_a_pure_read_never_mutating_the_engagement_session() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    let config = config_after(&emit, &config);
    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &config);
    let config = config_after(&emit, &config);
    let session_before = config.engagement_session_json.clone();
    let first = app.gesture_preview(&config);
    let second = app.gesture_preview(&config);
    assert_eq!(first, second, "equal checkpoint reads keep the exact same freshness stamp");
    assert_eq!(config.engagement_session_json, session_before, "gesture_preview must never mutate the live engagement session it reads");
}

#[semio_framework_async_macros::async_test]
async fn production_transition_authority_routes_engagement_utility_and_import_without_noop_increment() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let operation = preview_operation(1);
    let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };

    let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("ordinary engagement transition");
    let started = config_after(&started_emit, &base);
    let started_stamp = persisted_preview_stamp(&started);
    assert!(started.engagement_session_json.is_some());
    assert_eq!(started_stamp.operation, operation);
    assert_eq!(started_stamp.generation, base.engagement_preview_generation + 1);

    let utility_emit = drive_with_operation(&app, &scene, "engagementAbort", None, &started, Some(operation.clone())).expect("engagement cancellation transition");
    let utility_cleared = config_after(&utility_emit, &started);
    let utility_stamp = persisted_preview_stamp(&utility_cleared);
    assert!(utility_cleared.engagement_session_json.is_none());
    assert!(utility_stamp.is_fresher_than(&started_stamp));
    assert_eq!(utility_stamp.generation, started_stamp.generation + 1);

    let noop_emit = drive_with_operation(&app, &scene, "engagementAbort", None, &utility_cleared, Some(operation.clone())).expect("same checkpoint cancellation");
    let noop = config_after(&noop_emit, &utility_cleared);
    assert_eq!(noop.engagement_session_json, utility_cleared.engagement_session_json);
    assert_eq!(persisted_preview_stamp(&noop), utility_stamp, "a non-session config change must not advance the preview generation");

    let input_emit = drive_with_operation(&app, &scene, "engagementInput", Some(json!({ "value": "b", "pane": "shape" })), &noop, Some(operation.clone())).expect("engagement input");
    let with_input = config_after(&input_emit, &noop);
    assert_eq!(persisted_preview_stamp(&with_input), utility_stamp, "input-only config must preserve the stamp");
    let restarted_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &with_input, Some(operation.clone())).expect("restart engagement");
    let restarted = config_after(&restarted_emit, &with_input);
    let restarted_stamp = persisted_preview_stamp(&restarted);
    assert!(restarted_stamp.is_fresher_than(&utility_stamp));

    let import_emit = drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &restarted, Some(operation)).expect("scene import clear transition");
    let import_cleared = config_after(&import_emit, &restarted);
    let import_stamp = persisted_preview_stamp(&import_cleared);
    assert!(import_cleared.engagement_session_json.is_none());
    assert!(import_stamp.is_fresher_than(&restarted_stamp));
    assert_eq!(import_stamp.generation, restarted_stamp.generation + 1);
    assert!(matches!(import_emit.effects.first(), Some(Effect::LoadDocument { .. })));

    let example_emit = drive_with_operation(&app, &scene, "setActiveExample", Some(json!({ "exampleId": "" })), &restarted, Some(preview_operation(1))).expect("active example clear transition");
    let example_cleared = config_after(&example_emit, &restarted);
    let example_stamp = persisted_preview_stamp(&example_cleared);
    assert!(example_cleared.engagement_session_json.is_none());
    assert!(example_stamp.is_fresher_than(&restarted_stamp));
    assert_eq!(example_stamp.generation, restarted_stamp.generation + 1);
    assert!(matches!(example_emit.effects.first(), Some(Effect::LoadDocument { .. })));
}

#[semio_framework_async_macros::async_test]
async fn production_transition_authority_isolates_two_app_aba_sequences() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let operations = [preview_operation(1), preview_operation(2)];
    let mut previews = Vec::new();

    for operation in operations {
        let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("start app-local engagement");
        let started = config_after(&started_emit, &base);
        let a_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &started, Some(operation.clone())).expect("A");
        let at_a = config_after(&a_emit, &started);
        let first_a = app.gesture_preview(&at_a).expect("first A preview");
        let b_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &at_a, Some(operation.clone())).expect("B");
        let at_b = config_after(&b_emit, &at_a);
        let a_again_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &at_b, Some(operation)).expect("A again");
        let at_a_again = config_after(&a_again_emit, &at_b);
        let second_a = app.gesture_preview(&at_a_again).expect("second A preview");
        assert_eq!(first_a.payload, second_a.payload);
        assert_eq!(second_a.stamp.generation, first_a.stamp.generation + 2);
        assert_ne!(first_a.stamp, second_a.stamp);
        previews.push(second_a);
    }

    assert_eq!(previews[0].payload, previews[1].payload);
    assert_eq!(previews[0].stamp.generation, previews[1].stamp.generation);
    assert_ne!(previews[0].stamp.operation, previews[1].stamp.operation);
    assert!(!previews[0].is_fresher_than(&previews[1].stamp));
    assert!(!previews[1].is_fresher_than(&previews[0].stamp));
}

#[semio_framework_async_macros::async_test]
async fn production_transition_exhaustion_and_missing_context_fail_before_checkpoint_persistence() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let operation = preview_operation(9);
    let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("start live engagement");
    let started = config_after(&started_emit, &base);
    let mut at_max = started.clone();
    at_max.engagement_preview_generation = CAD_PREVIEW_GENERATION_MAX;
    let checkpoint_before = at_max.engagement_session_json.clone();
    let operation_before = at_max.engagement_preview_operation_json.clone();

    assert!(drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &at_max, Some(operation.clone())).is_err());
    assert!(drive_with_operation(&app, &scene, "setActiveExample", Some(json!({ "exampleId": "" })), &at_max, Some(operation.clone())).is_err());
    assert!(drive_with_operation(&app, &scene, "engagementAbort", None, &at_max, Some(operation)).is_err());
    assert_eq!(at_max.engagement_session_json, checkpoint_before, "failed commands cannot persist their cleared checkpoint");
    assert_eq!(at_max.engagement_preview_generation, CAD_PREVIEW_GENERATION_MAX);
    assert_eq!(at_max.engagement_preview_operation_json, operation_before);

    assert!(drive_with_operation(&app, &scene, "engagementAbort", None, &started, None).is_err());
    assert!(drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &started, None).is_err());
    let mut bypass = cad_runtime_from_config(&started);
    bypass.engagement_session = None;
    assert!(snapshot_of(&bypass, &started).is_err(), "ordinary snapshots must reject session-transition bypasses");
    assert_eq!(started.engagement_session_json, checkpoint_before);
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_rejects_aba_collision_and_cross_app_stamps_and_survives_restart() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base);
    let started = config_after(&emit, &base);

    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &started);
    let at_a = config_after(&emit, &started);
    let preview_a = app.gesture_preview(&at_a).expect("A preview");
    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &at_a);
    let at_b = config_after(&emit, &at_a);
    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &at_b);
    let at_a_again = config_after(&emit, &at_b);
    let preview_a_again = app.gesture_preview(&at_a_again).expect("second A preview");
    assert_eq!(preview_a.payload, preview_a_again.payload, "fixture must exercise A → B → A");
    assert_eq!(preview_a_again.stamp.generation, preview_a.stamp.generation + 2);
    assert_ne!(preview_a.stamp, preview_a_again.stamp, "ABA payload equality cannot reproduce a freshness stamp");

    let restarted: CadConfig = json::from_json_str(&json::to_json_string(&at_a_again)).expect("cold reopen config");
    assert_eq!(app.gesture_preview(&restarted).expect("reopened preview").stamp, preview_a_again.stamp);

    let mut other_app = restarted.clone();
    other_app.engagement_preview_operation_json =
        Some(json::to_json_string(&CadPreviewOperationIdentity { app_instance_id: 2, parent_document_id: "cad-test-document".into(), operation_id: 1, operation_generation: 1, canonical_base_revision: "00".repeat(32) }));
    let collision = app.gesture_preview(&other_app).expect("other app preview");
    assert_eq!(collision.stamp.generation, preview_a_again.stamp.generation, "forced finite-generation collision fixture");
    assert_ne!(collision.stamp.operation, preview_a_again.stamp.operation);
    assert!(!collision.is_fresher_than(&preview_a_again.stamp), "freshness requires exact operation identity, not only a colliding counter");
}

#[semio_framework_async_macros::async_test]
async fn preview_generation_cross_surface_domain_round_trips_max_and_rejects_plus_one() {
    let app = CadPlayApp::default();
    let operation = CadPreviewOperationIdentity { app_instance_id: 7, parent_document_id: "cad-max-document".into(), operation_id: 11, operation_generation: 13, canonical_base_revision: "ab".repeat(32) };
    let at_max = CadConfig { engagement_session_json: Some("{}".into()), engagement_preview_operation_json: Some(json_string_of(&operation)), engagement_preview_generation: CAD_PREVIEW_GENERATION_MAX, ..CadConfig::default() };
    let mut encoded = protocol::ToValue::to_value(&at_max);
    let decoded = <CadConfig as protocol::FromValue>::from_value(encoded.clone()).expect("maximum generation deserializes exactly");
    assert_eq!(decoded.engagement_preview_generation, CAD_PREVIEW_GENERATION_MAX);
    assert_eq!(app.gesture_preview(&decoded).expect("maximum generation remains previewable").stamp.generation, CAD_PREVIEW_GENERATION_MAX);

    let plus_one = i64::from(CAD_PREVIEW_GENERATION_MAX) + 1;
    if let protocol::DslValue::Object(fields) = &mut encoded {
        for (key, value) in fields.iter_mut() {
            if key == "engagementPreviewGeneration" {
                *value = protocol::DslValue::int(plus_one);
            }
        }
    }
    assert!(<CadConfig as protocol::FromValue>::from_value(encoded).is_err(), "maximum + 1 must fail before entering persisted config");

    let mut runtime = cad_runtime_from_config(&decoded);
    runtime.engagement_session = None;
    let ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation: Some(operation), view_state: None };
    assert!(preview_transition_snapshot_of(&runtime, &decoded, &ctx).is_err(), "incrementing the maximum generation must fail closed");

    let json_schema: Value = json::parse(include_str!("../../🎚️config/🧬️schema/🔣️.json")).expect("CAD config JSON descriptor");
    let generation_schema = &json_schema["properties"]["engagementPreviewGeneration"];
    assert_eq!(generation_schema["minimum"], json!(0));
    assert_eq!(generation_schema["maximum"], json!(CAD_PREVIEW_GENERATION_MAX));
    assert!(include_str!("../../🎚️config/🧬️schema/🛰️.proto").contains("int32 engagement_preview_generation = 32;"));
    assert!(include_str!("../../🎚️config/🧬️schema/🔗️.graphql").contains("engagementPreviewGeneration: Int!"));
    assert!(include_str!("../../🎚️config/🧬️schema/🟦️.ts").contains("engagementPreviewGeneration: number;"));
    assert!(include_str!("../../🎚️config/🧬️schema/🦀️.rs").contains("engagement_preview_generation: i32"));
}
//#endregion 🔖️GesturePreview

#[semio_framework_async_macros::async_test]
async fn engagement_repeat_last_restarts_the_last_finalized_interaction() {
    let app = CadPlayApp::default();
    let mut scene = default_document();
    let mut config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    config = config_after(&emit, &config);
    assert!(cad_runtime_from_config(&config).engagement_session.is_some());

    // 📦️box.json's default boxMode is "point" (length/width prompt); select diagonal mode (key
    // "d") to reach the classic two-corner-click flow.
    config.engagement_input = "d".into();
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    config = config_after(&emit, &config);

    for position in [json!([0.0, 0.0, 0.0]), json!([2.0, 3.0, 0.0])] {
        let emit = drive_with_config(&app, &scene, "worldPointerDown", Some(json!({ "pane": "shape", "position": position })), &config);
        scene = apply_mutations(&scene, &emit.artifact_mutations);
        config = config_after(&emit, &config);
    }

    config.engagement_input = "SetHeight2.5".into();
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    config = config_after(&emit, &config);

    // 📦️box.json's `set.height` only records the height (state stays first_corner_height); an
    // explicit `confirm` (Enter) is needed to reach `ready`, box's commit.fromStates.
    config.engagement_input = "Confirm".into();
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    scene = apply_mutations(&scene, &emit.artifact_mutations);
    config = config_after(&emit, &config);
    let runtime = cad_runtime_from_config(&config);
    assert!(runtime.engagement_session.is_none(), "box should have committed");
    assert_eq!(runtime.last_finalized_interaction_id.as_deref(), Some("primitive.box"));

    let emit = drive_with_config(&app, &scene, "engagementRepeatLast", Some(json!({ "pane": "shape" })), &config);
    let runtime = runtime_after(&emit, &config);
    let session = runtime.engagement_session.as_ref().expect("repeat-last should start a session");
    assert_eq!(session.interaction_id, "primitive.box");
}
//#endregion 🔖️Engagement
//#region 🔖️Import
#[semio_framework_async_macros::async_test]
async fn import_spatial_modelspace_round_trips() {
    let payload = json!({
        "schema": "spatial.modelspace",
        "revision": 1,
        "activeModelDefinitionId": "spatial.shape",
        "models": [{
            "id": "spatial.shape",
            "model": {
                "schema": "spatial.model",
                "revision": 1,
                "objects": [{
                    "id": "object-imported",
                    "label": "Imported",
                    "typology": "spatial.shape.primitive.box",
                    "visible": true,
                    "locked": false,
                    "origin": [1.0, 2.0, 3.0],
                    "primitives": []
                }]
            }
        }]
    });
    let scene = scene_from_spatial_payload(&json::to_dsl_value(&payload)).expect("scene");
    assert!(scene.shape_model.is_some(), "a real imported object must mint a shape-model child");
}

#[semio_framework_async_macros::async_test]
async fn import_cad_file_action_accepts_spatial_json_text_string_payload() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let file_text = json!({
        "schema": "spatial.model",
        "revision": 1,
        "modelDefinitionId": "spatial.shape",
        "objects": [{
            "id": "object-loaded",
            "label": "Loaded",
            "typology": "spatial.shape.primitive.box",
            "visible": true,
            "locked": false,
            "origin": [1.0, 2.0, 3.0],
            "primitives": []
        }]
    })
    .to_string();
    let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": file_text, "name": "cad.spatial.json" })));
    // 🌱️ Whole-document replace is not an in-history mutation (SEMANTIC-MUTATIONS-OVERHAUL
    // retired `SetSnapshot`) — a spatial JSON string payload now surfaces as a
    // `Effect::LoadDocument` carrying the replacement document's pack bytes.
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("importCadFile must emit a LoadDocument effect for a spatial JSON string payload") else {
        panic!("expected a LoadDocument effect");
    };
    let next = <CadSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert!(next.shape_model.is_some(), "a real imported object must mint a shape-model child");
}

#[semio_framework_async_macros::async_test]
async fn import_cad_file_action_imports_obj_by_extension() {
    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `import_cad_object_by_extension`
    // now returns a `SemioModelElement` — composing it into the document needs the same
    // child-dispatch seam as `commands/🧱️object/component.rs` (see `import_cad_file::handle`'s
    // own doc comment). Documented no-op. 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
    // (26/08/14): auto-selecting the imported object is no longer reachable from `handle()`
    // either (selection is framework-owned) — this now only asserts the document-write gap.
    let app = CadPlayApp::default();
    let scene = default_document();
    let obj_text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
    let obj_data_url = format!("data:model/obj;base64,{}", base64_codec::base64_standard_encode(obj_text));
    let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": obj_data_url, "name": "triangle.obj" })));
    assert!(emit.artifact_mutations.is_empty(), "importCadFile's document write is a documented no-op until the child-dispatch seam lands");
    assert!(emit.config_mutations.is_empty(), "importCadFile no longer touches config once selection moved to the framework");
}
//#endregion 🔖️Import
//#region 🔖️History
#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_added_node_through_generic_helper() {
    // ⚠️ `AddObject` is a documented no-op pending the child-dispatch seam (see
    // `commands/🧱️object/component.rs`'s module doc) — this exercises the generic
    // `assert_undo_redo_round_trip` testkit helper (distinct from `undo_redo_round_trips_added_node_through_wrapper`
    // below, which drives the manual add/undo/redo dance) against the real `AddNode` command.
    let mut app = new_app().await;
    let before = app.snapshot().expect("snapshot").nodes.len();
    semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1).await;
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_added_node_through_wrapper() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("snapshot").nodes.len();
    app.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).await.expect("add node");
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before + 1);
    let undo = app.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before);
    app.handle_action("redo", None, &meta("local")).await.expect("redo");
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn coalesced_translate_drag_is_a_single_undo_step() {
    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `translateSelection`
    // (and the `addObject` that used to seed the dragged object) are documented no-ops pending
    // the child-dispatch seam — object placement now lives inside composed
    // `s.stdio.semio.model` CHILD documents (see `commands/🔄️transform/component.rs`'s own doc
    // comment). This locks in the honest current behavior — a coalesced multi-tick drag emits
    // nothing to undo — rather than letting it silently drift.
    let mut app = new_app().await;
    let before = json::to_json_string(&app.snapshot().expect("snapshot"));
    for _ in 0..3 {
        app.dispatch_typed(CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec!["object-box-1".into()], dx: 1.0, dy: 0.0, dz: 0.0 }), &meta("local")).await.expect("translate tick");
    }
    let after = json::to_json_string(&app.snapshot().expect("snapshot"));
    assert_eq!(before, after, "translateSelection is a documented no-op until the child-dispatch seam lands");
}
//#endregion 🔖️History
//#region 🔖️Convergence
/// 🧪️ The definitional merge proof: two instances start from the SAME base projection, apply
/// DISJOINT edits (A translates object A, B patches object B's label), and after exchanging operations
/// over a `MemoryBackbone` both converge to contain BOTH edits — impossible under whole-document
/// `setDocument` snapshots.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `PatchObject` is a
    // documented no-op pending the child-dispatch seam (object fields now live inside composed
    // `s.stdio.semio.model` CHILD documents — see `commands/🧱️object/component.rs`'s module
    // doc), so it can no longer stand in as the disjoint edit this law needs. `RenameNode` is a
    // real, unaffected parent-document mutation (node data was never part of the deleted inline
    // object list) — proves the identical convergence property.
    let mut base = default_document();
    base.nodes = vec![CadNode { id: "node-a".into(), label: "A".into(), kind: "solid".into() }, CadNode { id: "node-b".into(), label: "B".into(), kind: "solid".into() }];
    let node_a = base.nodes[0].id.clone();
    let node_b = base.nodes[1].id.clone();
    let base_envelope = store::create_document_envelope::<CadSnapshot, CadMutation>(CAD_DOCUMENT_SCHEMA, "cad-play", base, None);
    let base_files = store::print_document_pack(&base_envelope).await.expect("print document pack");

    let mut instance_a = new_app().await;
    let mut instance_b = new_app().await;
    instance_a.load_document_pack(&base_files).await.expect("load a");
    instance_b.load_document_pack(&base_files).await.expect("load b");
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://cad-convergence", "mem://cad-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    // A renames node A.
    instance_a.dispatch_typed(CadCommand::RenameNode(rename_node::RenameNode { node_id: node_a.clone(), value: "Renamed By A".into() }), &meta("actor-a")).await.expect("a renames node a");

    // B renames node B — a disjoint edit that must survive alongside A's.
    instance_b.dispatch_typed(CadCommand::RenameNode(rename_node::RenameNode { node_id: node_b.clone(), value: "Renamed By B".into() }), &meta("actor-b")).await.expect("b renames node b");

    // A neutral history command always pumps inbound operations before doing its own work.
    instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");

    let scene_a = instance_a.snapshot().expect("projection a");
    let scene_b = instance_b.snapshot().expect("projection b");

    let label_a_in_a = scene_a.nodes.iter().find(|node| node.id == node_a).unwrap().label.clone();
    let label_a_in_b = scene_b.nodes.iter().find(|node| node.id == node_a).unwrap().label.clone();
    let label_b_in_a = scene_a.nodes.iter().find(|node| node.id == node_b).unwrap().label.clone();
    let label_b_in_b = scene_b.nodes.iter().find(|node| node.id == node_b).unwrap().label.clone();

    assert_eq!(label_a_in_a, "Renamed By A", "instance A keeps its own edit");
    assert_eq!(label_a_in_b, "Renamed By A", "instance B converges on A's edit");
    assert_eq!(label_b_in_a, "Renamed By B", "instance A converges on B's edit");
    assert_eq!(label_b_in_b, "Renamed By B", "instance B keeps its own edit");
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_cad() {
    let mut sender = new_app().await;
    let (near, mut far) = MemoryBackbone::pair("mem://cad-doc", "mem://cad-doc").await;
    sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
    sender.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).await.expect("add node");

    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(operations);
        }
    }
    assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
    let operations = envelopes;

    let mut receiver = new_app().await;
    let nodes_before = receiver.snapshot().expect("snapshot").nodes.len();
    receiver.ingest_operations(&operations).await.expect("ingest once");
    receiver.ingest_operations(&operations).await.expect("ingest twice");
    assert_eq!(receiver.snapshot().expect("snapshot").nodes.len(), nodes_before + 1, "feeding the same operation twice must not double-apply");
}
//#endregion 🔖️Convergence
