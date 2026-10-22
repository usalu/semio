//! 🧰️ Command fixtures and retained lanes of the CAD editor unit tests: the every-command fixture, the example admission, the retained factory proofs and the wire round trips. See [`super`].

use super::*;
use semio_framework_test_fixtures::fixture_path;

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
        CadCommand::SetReferenceHidden(set_reference_hidden::SetReferenceHidden { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), hidden: true }),
        CadCommand::SetReferenceLocked(set_reference_locked::SetReferenceLocked { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), locked: false }),
        CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: Some("shape".into()) }),
        CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: None }),
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
    let command = <CadPlayApp as ArtifactEditor>::command_from_action("setActiveExample", Some((semio_framework_pack_json::to_dsl_value(&json!({ "exampleId": CAD_EXAMPLE_FOREST_LEFT }))).clone()), None).expect("declared example action");
    assert!(matches!(command, CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id }) if example_id == CAD_EXAMPLE_FOREST_LEFT));
    let contributions = <CadPlayApp as ArtifactEditor>::command_from_action("setContributions", Some((semio_framework_pack_json::to_dsl_value(&json!({ "json": "[{\"id\":\"cad\"}]" }))).clone()), None).expect("declared host command");
    assert!(matches!(contributions, CadCommand::SetContributions(set_contributions::SetContributions { json }) if json == "[{\"id\":\"cad\"}]"));
    assert!(<CadPlayApp as ArtifactEditor>::command_from_action("notACadAction", None, None).is_err());
}

/// 🚪️ The React shell announces the first example by dispatching `setActiveExample` through the
/// retained typed-operation lane. It used to be classified `BatchOnlyPendingRewrite`, which the host
/// refuses outright (`dispatch-failed … interactive-job classification BatchOnlyPendingRewrite`), and
/// the whole-document load it publishes then met the trait-default initializer refusal
/// (`artifact-store.persisted-initializer-refused`). Every offered example must be admitted, publish
/// ONE `LoadDocument`, settle `Ready` through the host's archive door and become the live document.
#[semio_framework_async_macros::async_test]
async fn every_example_load_is_admitted_and_settles_through_the_host_document_archive_door() {
    use semio_framework_plugin::app::TypedOperationResultLane;
    let definition = create_cad_app();
    let declared = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).filter(|action| action.id == "setActiveExample").map(|action| action.semantics.execution.interactive_job).chain(definition.commands.iter().filter(|command| command.id == "setActiveExample").map(|command| command.semantics.execution.interactive_job)).collect::<Vec<_>>();
    assert!(!declared.is_empty() && declared.iter().all(|classification| *classification == InteractiveJobClassification::Migrated), "setActiveExample must be a live interactive job: {declared:?}");
    const INSTANCE: u32 = 7;
    for (archive_id, example_id) in [(91_u64, CAD_EXAMPLE_FOREST_LEFT), (92, semio_s_artifact_cad_cad_examples::ID), (93, "")] {
        let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = semio_framework_plugin::artifact_app_laws::new_app_with_registry_and_members::<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(cad_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        app.bind_instance_id(INSTANCE).await;
        app.dispatch_typed(CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() }), &semio_framework_plugin::ActionMeta { actor: "fixture".into(), instance_id: INSTANCE, view_state: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.expect("setActiveExample is admitted");
        let mut loaded = None;
        let mut terminal = false;
        for _ in 0..100_000 {
            PluginApp::maintenance_step(&mut app, scope.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("maintenance step");
            app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.expect("publication");
            if let Some(page) = app.take_typed_operation_result_page(INSTANCE) {
                assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                terminal |= page.lane == TypedOperationResultLane::Terminal;
                assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge"));
            }
            if let Some(semio_framework::kernel::Effect::LoadDocument { pack, spr }) = app.take_typed_operation_effect() {
                assert!(loaded.is_none(), "one example switch is one whole-document load");
                loaded = Some((pack, spr));
            }
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            if !app.has_pending_typed_operations() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(terminal, "{example_id:?}: the retained operation reaches its terminal page");
        let (parent_pack, parent_spr) = loaded.expect("an example publishes one whole-document load");
        let expected = <CadSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&parent_pack).expect("the load carries a CAD document");
        PluginApp::begin_document_archive_load(&mut app, archive_id, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).expect("archive admission");
        let mut status = None;
        for _ in 0..1_000_000 {
            let polled = PluginApp::poll_document_archive_load(&mut app, archive_id, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.expect("archive status");
            if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
                status = Some(polled);
                break;
            }
            PluginApp::maintenance_step(&mut app, scope.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("archive maintenance step");
            std::thread::yield_now();
        }
        let status = status.expect("archive load reaches a terminal state");
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{example_id:?}: {}", String::from_utf8_lossy(&status.fault));
        PluginApp::acknowledge_document_archive_load(&mut app, archive_id).expect("archive acknowledgement");
        let live = app.snapshot().expect("loaded snapshot");
        assert_eq!(live, expected, "{example_id:?} became the live document");
        for pane in CadPaneId::all() {
            let Some(child) = semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&live, pane) else { continue };
            assert!(app.child_store(semio_s_artifact_cad_cad_model::snapshot::cad_pane_model_slot(pane), &child.child_id).await.is_some(), "{example_id:?}: the genesis-derived {pane:?} model member is live after the load");
            let genesis = semio_s_artifact_cad_cad_play::cad_bundled_pane_scene(&child.child_id).unwrap_or_else(|| panic!("{example_id:?}: the loaded {pane:?} pane names a genesis scene"));
            let objects = pane_objects(&app, pane).await;
            assert_eq!(objects, semio_s_artifact_cad_cad_model::snapshot::cad_scene_pane_objects(&genesis, pane), "{example_id:?}: the loaded {pane:?} pane's child holds its genesis objects exactly");
            assert_eq!(objects.is_empty(), example_id.is_empty(), "{example_id:?}: only the empty document loads empty panes ({pane:?})");
        }
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
    }
}

/// 🛂️ An example id the app does not ship is refused by name — never a silent no-op that leaves the
/// previous document standing while the shell's picker claims the switch happened.
#[test]
fn an_unknown_example_is_refused_by_name() {
    let Err(fault) = drive_result(&CadPlayApp, &forest_play_scene(), "setActiveExample", Some(json!({ "exampleId": "not-an-example" })), &CadConfig::default()) else { panic!("an unknown example must be refused") };
    assert!(fault.message.starts_with("cad.example.unknown"), "{fault:?}");
}

#[test]
fn host_contributions_resolve_to_the_event_sourced_config_lane() {
    let mutation = <CadPlayApp as ArtifactEditor>::host_configuration_mutation("setContributions", Some(&semio_framework_pack_json::to_dsl_value(&json!({ "json": "[{\"id\":\"cad\"}]" })))).expect("host configuration").expect("CAD contribution mutation");
    assert_eq!(mutation, CadConfigMutation::SetContributions(SetContributionsEdit { json: "[{\"id\":\"cad\"}]".into() }));
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
    let fixture: Value = semio_framework_pack_json::parse(std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🧫️fixtures/♻️retirement/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🧫️fixtures/♻️retirement/🔣️.json").leak(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let maximum_items = fixture["grant"]["maximumItems"].as_u64().unwrap() as usize;
    let maximum_bytes = fixture["grant"]["maximumBytes"].as_u64().unwrap() as usize;
    let envelope = store::os_store::document_materialize::create_document_envelope::<NoDraft, NoDraftMutation>("draft.empty", "cad-draft-close", NoDraft::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut draft = store::DraftStore::new(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
    let birth = <CadPlayApp as ArtifactEditor>::draft_store_owners_source_demands().expect("CAD draft owner source quote");
    let owners = store::os_store::document_store_owners::fund_document_store_owners(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth.max(1) }, |grant| <CadPlayApp as ArtifactEditor>::build_draft_store_owners(grant).expect("CAD publishes a draft owner catalog")).expect("CAD draft owners are funded by their own quote");
    draft.install_document_store_owners_exact(owners).unwrap_or_else(|(error, _)| panic!("CAD draft store refused its exact owner catalog: {error}"));
    let quoted = |demand: semio_framework_value::RetirementDemand, items: usize| semio_framework_value::RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: demand.copy_bytes.max(maximum_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
    let mut disposer = <CadPlayApp as ArtifactEditor>::build_draft_store_disposer().unwrap();
    for turn in 0..100_000 {
        let grant = quoted(disposer.retirement_demands(&draft, maximum_bytes).expect("CAD draft store close quote"), maximum_items);
        match disposer.close_step(&mut draft, grant).unwrap() {
            semio_framework_plugin::PluginLifecycleStep::Progress(progress) => assert!(progress.copied_items <= maximum_items && progress.released_bytes <= grant.maximum_release_bytes),
            semio_framework_plugin::PluginLifecycleStep::Blocked { reason } => panic!("empty CAD draft close blocked: {reason}"),
            semio_framework_plugin::PluginLifecycleStep::AwaitingInput { reason } => panic!("fresh CAD fixture unexpectedly awaits external input: {reason}"),
            semio_framework_plugin::PluginLifecycleStep::Complete(_) => break,
        }
        assert!(turn < 99_999);
    }
    assert!(disposer.terminal_is_empty(&draft));
    let mut transient = store::TransientStore::new(semio_framework_plugin::NoTransient::default());
    let mut disposer = <CadPlayApp as ArtifactEditor>::build_transient_store_disposer().unwrap();
    let demand = disposer.retirement_demands(&transient, maximum_bytes).expect("CAD transient store close quote");
    assert_eq!(disposer.close_step(&mut transient, quoted(demand, 0)).unwrap(), semio_framework_plugin::PluginLifecycleStep::Progress(Default::default()));
    for turn in 0..100_000 {
        let grant = quoted(disposer.retirement_demands(&transient, maximum_bytes).expect("CAD transient store close quote"), maximum_items);
        if matches!(disposer.close_step(&mut transient, grant).unwrap(), semio_framework_plugin::PluginLifecycleStep::Complete(_)) {
            break;
        }
        assert!(turn < 99_999);
    }
    assert!(disposer.terminal_is_empty(&transient));
}

#[semio_framework_async_macros::async_test]
async fn retained_factory_proofs_activate_the_real_cad_manifest_and_close_under_the_production_grant() {
    let fixture: Value = semio_framework_pack_json::parse(std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json").leak(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("CAD activation fixture");
    let activation = &fixture["activation"];
    let controller = activation["controller"].as_str().expect("controller");
    let bus = semio_framework::ActionBus::new();
    let definition = create_cad_app();
    let host_route = fixture["routes"].as_array().unwrap().iter().find(|route| route["id"] == "setContributions").unwrap();
    assert_eq!(host_route["disposition"], "migrated");
    let host_command = definition.commands.iter().find(|command| command.id == host_route["id"].as_str().unwrap()).expect("host command declaration");
    assert_eq!(host_command.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    let registry = AppActionRegistry::from_definition(&definition);
    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>::with_registry_on_bus(EditorApp::<CadPlayApp>::default(), registry, bus.clone(), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    semio_framework_plugin::artifact_app_laws::settle_store_constructor(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
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
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
    assert!(app.close_terminal_is_empty(), "the real mounted CAD owner must reach its empty terminal shell");
}

#[test]
fn retained_config_store_preparation_is_bounded_exact_and_reversible() {
    let base = CadConfig::default();
    let mut next = base.clone();
    next.selected_node_ids.push("node-retained".into());
    let mutation = CadConfigMutation::Set(SetEdit { config: Box::new(next.clone()) });
    let footprint = admit_cad_config_mutation(&mutation).expect("bounded CAD config mutation");
    let (post, inverse, forward) = prepare_cad_config(&base, mutation.clone()).expect("exact CAD config preparation");
    assert_eq!(post, next);
    assert_eq!(forward, mutation);
    assert_eq!(inverse, vec![CadConfigMutation::Set(SetEdit { config: Box::new(base.clone()) })]);
    assert_eq!(footprint.work_items, 1 + inverse.len(), "config footprint must cover forward + inverse rows");
    assert_eq!(footprint, store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(&mutation, footprint.retained_bytes));
    let oversized = CadConfigMutation::SetContributions(SetContributionsEdit { json: "x".repeat(CAD_CONFIG_STORE_MAXIMUM_BYTES + 1) });
    assert!(admit_cad_config_mutation(&oversized).is_err());
}

/// 🧺️ The one-invertible-item declaration (`admit_cad_artifact_mutation`) is only exact while every
/// `CadMutation` inverts to at most one row against the forest document.
#[test]
fn every_cad_mutation_inverse_fits_the_one_invertible_item_footprint() {
    let base = forest_play_scene();
    let history = empty_history();
    let doc = ArtifactView::new(&base, &history);
    let config = CadConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let io_commands = ["importCadFile", "saveSelected", "saveInPlay", "saveCurrent", "loadRawRequest", "setActiveExample"];
    for command in every_command().into_iter().filter(|command| !io_commands.contains(&command.command_id())) {
        let mut ctx = CadDispatchCtx::new(CadInteractionSnapshot::default(), None, CadWorldWindowTransient::default());
        let Ok(emit) = command.dispatch(&doc, &cfg, &mut ctx) else { continue };
        for mutation in &emit.artifact_mutations {
            let inverse = <CadMutation as protocol::ColdMutation<CadSnapshot>>::inverse(mutation, &base).expect("valid retained mutation inverse fixture");
            assert!(inverse.len() <= 1, "{} inverts to {} rows; the artifact lane declares one invertible item", command.command_id(), inverse.len());
        }
    }
}

#[test]
fn retained_artifact_store_preparation_is_bounded_exact_and_reversible() {
    let base = empty_cad_snapshot();
    let node = CadNode { id: "node-retained".into(), label: "Retained".into(), kind: "group".into() };
    let mutation = CadMutation::CreateNode(semio_s_artifact_cad_cad_mut_nodes::create_node::CreateNode { node: node.clone() , index: None });
    let footprint = admit_cad_artifact_mutation(&mutation).expect("bounded CAD Artifact mutation");
    let (post, inverse, forward) = prepare_cad_artifact(&base, mutation.clone()).expect("exact CAD Artifact preparation");
    assert_eq!(post.nodes, vec![node]);
    assert_eq!(forward, mutation);
    assert_eq!(footprint.work_items, 1 + inverse.len(), "artifact footprint must cover forward + inverse rows");
    assert_eq!(footprint, store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(&mutation, footprint.retained_bytes));
    let mut restored = post;
    for operation in inverse {
        let outcome = <CadMutation as protocol::ColdMutation<CadSnapshot>>::diff(&operation, &restored);
        restored = protocol::apply_diff_cold(outcome.diff(), &restored).expect("exact inverse");
    }
    assert_eq!(restored, base);
}

#[test]
fn retained_route_fixture_matches_the_exact_owner_manifest_and_laws() {
    let fixture: Value = semio_framework_pack_json::parse(std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json").leak(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("CAD retained route fixture");
    let routes = fixture.get("routes").and_then(Value::as_array).expect("route array");
    let route_ids = routes.iter().map(|route| route.get("id").and_then(Value::as_str).expect("route id")).collect::<std::collections::BTreeSet<_>>();
    let command_ids = every_command().iter().map(CadCommand::command_id).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(routes.len(), command_ids.len());
    assert_eq!(route_ids, command_ids);
    assert_eq!(fixture.get("admittedRoutes"), Some(&Value::Array(CAD_RETAINED_TOOL_IDS.iter().map(|id| Value::from(*id)).collect())));
    assert_eq!(fixture.pointer("/limits/closePageBytes").and_then(Value::as_u64), Some(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES as u64));
    assert_eq!(fixture.get("laws"), Some(&json!(["ownerLocal", "progress", "cancel", "freshness", "ackBeforeClose", "incrementalClose", "terminalEmpty"])));
    assert!(routes.iter().all(|route| {
        let id = route.get("id").and_then(Value::as_str);
        let disposition = route.get("disposition").and_then(Value::as_str);
        let blocker = route.get("blocker").and_then(Value::as_str);
        if id.is_some_and(|id| CAD_RETAINED_TOOL_IDS.contains(&id)) {
            disposition == Some("migrated") && blocker == Some("none")
        } else {
            disposition == Some("batchOnlyPendingRewrite") && blocker.is_some_and(|blocker| blocker != "none")
        }
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
            let actions = semio_framework::window_kind_actions(&manifest, window).into_iter().filter(|action| action.id == *tool_id).collect::<Vec<_>>();
            if actions.is_empty() {
                continue;
            }
            assert_eq!(actions.len(), 1, "{tool_id} requires exactly one declaration per window scope");
            assert_eq!(actions[0].semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id}");
            declarations += 1;
        }
        assert!(declarations > 0, "{tool_id} requires a manifest command or window declaration");
    }
    assert_eq!(manifest.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&manifest, window)).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).map(|action| action.semantics.execution.interactive_job), Some(InteractiveJobClassification::Migrated));
    assert!(manifest.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&manifest, window)).filter(|action| route_ids.contains(action.id.as_str())).all(|action| {
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
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<CadPlayApp>>(cad_app_manifest_for_tests).await;
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
//#endregion 🔖️Fixtures
