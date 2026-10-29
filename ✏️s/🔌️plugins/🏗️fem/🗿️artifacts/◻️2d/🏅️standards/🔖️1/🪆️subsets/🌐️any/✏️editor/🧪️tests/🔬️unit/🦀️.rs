use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use crate::context;


use super::*;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use crate::context::{fem2d_app, render};
use semio_framework_plugin::{ArtifactEditor, EditorApp};
use pack::snapshot::ArtifactDsl;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_display;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_camera;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_animation;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dRetainedCommandJobFactory;
#[cfg(test)] use semio_framework_plugin::ArtifactOwnedToolJobFactory;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_down;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_move;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_up;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::focus_entity;
#[cfg(test)] use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_material;
#[cfg(test)] use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results as results_window;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp;
#[cfg(test)] use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;

//#region 🔖️CommandSurface
/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Fem2dCommand> {
    vec![
        Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }),
        Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
        Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }),
        Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }),
        Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 }),
        Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tx, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ty] }),
        Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) }),
        Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None }),
        Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) }),
        Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None }),
        Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }),
        Fem2dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: vec![semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] }),
        Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true }),
        Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0), field: None, value: None, window_id: None }),
        Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["scope:1:4:node2:n1".into(), "scope:1:7:element2:e1".into()] }),
        Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
        Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 }),
        Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some(semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").expect("source")), mode: "modal".into(), mode_index: 0, field: None, value: None, window_id: None }),
        Fem2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 120.0, y: 80.0, width: 640.0, height: 480.0, button: 0, shift: false, ctrl: false, meta: false, alt: false }),
        Fem2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 121.0, y: 81.0, width: 640.0, height: 480.0, samples: vec![[120.5, 80.5], [121.0, 81.0]] }),
        Fem2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 121.0, y: 81.0, width: 640.0, height: 480.0, shift: false, ctrl: false, meta: false, alt: false, cancelled: false }),
        Fem2dCommand::PatchNode(patch_node::PatchNode { id: "n1".into(), field: "x".into(), value: "1.5".into() }),
        Fem2dCommand::PatchElement(patch_element::PatchElement { id: "e3".into(), field: "sectionId".into(), value: "post140".into() }),
        Fem2dCommand::PatchMaterial(patch_material::PatchMaterial { id: "steel".into(), field: "e".into(), value: "200000000000".into() }),
        Fem2dCommand::PatchSection(patch_section::PatchSection { id: "chs76".into(), field: "area".into(), value: "0.006".into() }),
        Fem2dCommand::PatchSupport(patch_support::PatchSupport { id: "s1".into(), field: "rz".into(), value: "true".into() }),
        Fem2dCommand::PatchRegion(patch_region::PatchRegion { id: "r1".into(), field: "thickness".into(), value: "0.25".into() }),
        Fem2dCommand::PatchLoad(patch_load::PatchLoad { id: "scope:1:4:load4:live2:l6".into(), field: "value".into(), value: "-8000".into() }),
        Fem2dCommand::PatchLoadCase(patch_load_case::PatchLoadCase { id: "dead".into(), field: "name".into(), value: "Dead".into() }),
        Fem2dCommand::PatchCombination(patch_combination::PatchCombination { id: "uls".into(), field: "term:live".into(), value: "1.5".into() }),
        Fem2dCommand::SetResultAnimation(set_result_animation::SetResultAnimation { phase: Some(0.25), playing: Some(true), speed: None, loop_mode: Some("pingPong".into()), waveform: None, field: None, value: None, window_id: None }),
        Fem2dCommand::ResultAnimationTick(result_animation_tick::ResultAnimationTick { window_id: "results-left".into() }),
        Fem2dCommand::FocusEntity(focus_entity::FocusEntity { id: "scope:1:4:node2:n1".into() }),
        Fem2dCommand::TranslateSelection(crate::editor::fem2d::commands::gumball::translate_selection::TranslateSelection { ids: vec!["scope:1:4:node2:n1".into()], dx: 0.1, dy: 0.0, dz: 0.0, phase: None, reason: None }),
        Fem2dCommand::RotateSelection(crate::editor::fem2d::commands::gumball::rotate_selection::RotateSelection { ids: vec!["scope:1:4:node2:n1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 0.1, phase: None, reason: None }),
        Fem2dCommand::ScaleSelection(crate::editor::fem2d::commands::gumball::scale_selection::ScaleSelection { ids: vec!["scope:1:4:node2:n1".into()], sx: 1.01, sy: 1.01, sz: 1.0, phase: None, reason: None }),
        Fem2dCommand::SetTransformGumballFlag(crate::editor::fem2d::commands::gumball::set_transform_gumball_flag::SetTransformGumballFlag { flag: "move".into(), pressed: None }),
    ]
}

/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
/// hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 37, "every Fem2dCommand row must be covered by every_command()");
    let definition = create_fem2d_app();
    for id in ids {
        assert!(definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).any(|action| action.id == id), "command_id {id} must be a declared action");
    }
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 📌️ Declared command wire bytes, including each optional shape and canonical interaction identities.
#[semio_framework_async_macros::async_test]
async fn every_command_matches_declared_wire_bytes() {
    use protocol::OpBinary;
    let rows: Vec<(&str, Fem2dCommand)> = vec![
        ("010000020005000000000000f03f01050000000000000040", Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 })),
        ("010104026d31026e31026e3202733104000601010602020600030603", Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() })),
        ("010204026d31026e31026e3202733104000601010602020600030603", Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() })),
        ("01030105737465656c020006000105000000da7c724842", Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "steel".into(), e: 210e9 })),
        ("01040106697065333030030006000105a4005130630a763f020509c577de9de7153f", Fem2dCommand::AddSection(add_section::AddSection { name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 })),
        ("010501026e31020006000116020002", Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tx, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ty] })),
        ("010602046c697665026e3104000601010a010205000000000088b3c0030600", Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) })),
        ("010601026e3103000600010a010205000000000088b3c0", Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ty, value: -5000.0, case_id: None })),
        ("010701026531030006000105000000000000000002050000000000407fc0", Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None })),
        ("0108020464656164027231030006010105000000000088b340020600", Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) })),
        (
            "01090105737465656c060005000000000000000001050000000000000000020500000000000010400305000000000000004004060005057b14ae47e17a943f",
            Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None }),
        ),
        ("010a01044c697665020006000101", Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })),
        (
            "010b0303554c530464656164046c69766502000600010c020d0200060101059a9999999999f53f0d020006020105000000000000f83f",
            Fem2dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: vec![semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] }),
        ),
        ("010c010464656164020006000102", Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true })),
        ("010d000200040502050000000000003e40", Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0), field: None, value: None, window_id: None })),
        ("010e021573636f70653a313a373a656c656d656e74323a65311273636f70653a313a343a6e6f6465323a6e3101000c0206010600", Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["scope:1:4:node2:n1".into(), "scope:1:7:element2:e1".into()] })),
        ("010f010764656661756c7401000600", Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() })),
        ("011000030005000000000000f03f010500000000000000400205000000000000f83f", Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 })),
        ("0111021873636f70653a313a383a6c6f616443617365343a64656164056d6f64616c03000600010601020400", Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some(semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").expect("source")), mode: "modal".into(), mode_index: 0, field: None, value: None, window_id: None })),
    ];
    for (expected, command) in rows {
        let bytes = command.encode_op().expect("encode");
        assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected, "wire bytes changed for {}", command.command_id());
    }
}
//#endregion 🔖️CommandSurface

//#region 🔖️RetainedRoutes
/// 🧵️ LAW: the retained route table, the bounded-first-step proof roster, the publication-lane
/// contracts and the manifest's `Migrated` classifications are FOUR views of one set — every
/// declared command, exactly once, with no `BatchOnlyPendingRewrite` survivor. A route missing from
/// any one of them is a dead action at runtime (`interactive-job.missing-owned-reducer` or
/// `interactive-job.publication-authority-missing`).
#[semio_framework_async_macros::async_test]
async fn retained_routes_cover_every_command_exactly_once() {
    use std::collections::BTreeSet;
    let commands: BTreeSet<&str> = every_command().iter().map(teaching_proctor_domain::projections::fixtures::command_id).collect();
    let routes: BTreeSet<&str> = FEM2D_RETAINED_TOOL_IDS.iter().copied().collect();
    assert_eq!(routes.len(), FEM2D_RETAINED_TOOL_IDS.len(), "no duplicate retained route ids");
    assert_eq!(commands, routes, "every declared command is a retained route and vice versa");
    assert_eq!(<Fem2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), FEM2D_RETAINED_TOOL_IDS.len());
    assert_eq!(Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), FEM2D_RETAINED_TOOL_IDS.len());
    let definition = create_fem2d_app();
    for tool_id in FEM2D_RETAINED_TOOL_IDS {
        let contract = Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == *tool_id).unwrap_or_else(|| panic!("publication contract for {tool_id}"));
        assert!(!contract.lanes.is_empty(), "{tool_id} publishes into at least one lane");
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} declared"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be Migrated to dispatch interactively");
    }
}

/// 🛣️ LAW: a route's declared lanes cover exactly what its own handler emits — the framework
/// refuses a completion whose emitted lane is absent from the contract.
#[semio_framework_async_macros::async_test]
async fn every_route_declares_the_lane_its_handler_emits() {
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    for command in every_command() {
        let tool_id = command.command_id();
        let view = |id: &str, kind: &str| semio_framework_plugin::ViewModel { window_id: Some(id.into()), window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: id.into(), window_kind_id: kind.into() }], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        let emit = match &command {
            Fem2dCommand::SetCamera(payload) => set_camera::handle_window(payload, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::SetResultDisplay(payload) => set_result_display::handle_window(payload, &cfg, &view("results-law", results_window::WINDOW_KIND_ID)),
            Fem2dCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle_window(payload, &doc, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle_window(payload, &doc, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::CanvasPointerUp(payload) => canvas_pointer_up::handle_window(payload, &doc, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::FocusEntity(payload) => focus_entity::handle_window(payload, &doc, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::SetResultAnimation(payload) => set_result_animation::handle_window(payload, &doc, &cfg, &view("results-law", results_window::WINDOW_KIND_ID)),
            Fem2dCommand::ResultAnimationTick(payload) => semio_s_artifact_fem_2d_editor::editor::fem2d::commands::result_animation_tick::result_animation_tick_step::<semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_animation::Fem2dResultsPlayback>(payload, &cfg, &view("results-left", results_window::WINDOW_KIND_ID), None).map(|step| step.emit),
            Fem2dCommand::SetTransformGumballFlag(payload) => semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::set_transform_gumball_flag::handle_window(payload, &view("model-law", model_window::WINDOW_KIND_ID)),
            _ => command.dispatch(&doc, &cfg),
        }
        .unwrap_or_else(|error| panic!("{tool_id} dispatches: {error:?}"));
        let lanes = Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == tool_id).expect("publication contract").lanes;
        assert!(emit.artifact_mutations.is_empty() || lanes.contains(&ArtifactToolPublicationLane::Artifact), "{tool_id} emits document mutations without the Artifact lane");
        assert!(emit.config_mutations.is_empty() || lanes.contains(&ArtifactToolPublicationLane::Config), "{tool_id} emits config mutations without the Config lane");
        assert!(emit.window_config_mutations.is_empty() || lanes.contains(&ArtifactToolPublicationLane::WindowConfig), "{tool_id} emits window config mutations without the WindowConfig lane");
        assert!(
            !lanes.contains(&ArtifactToolPublicationLane::HostOnly) || (emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.window_config_mutations.is_empty()),
            "{tool_id} declares the host-only lane but publishes into a store"
        );
    }
}

/// 🎯️ LAW: the stringly `{action, args}` wire every shell speaks resolves to the typed command
/// with the same id — the trait's default rejects app actions outright, so this bridge is the only
/// path from a rendered button to `dispatch`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_resolves_every_declared_action() {
    let args = semio_framework_value::DslValue::Object(vec![("x".into(), semio_framework_value::DslValue::float(1.0)), ("y".into(), semio_framework_value::DslValue::float(2.0)), ("exampleId".into(), semio_framework_value::DslValue::String(semio_s_artifact_fem_2d_io::examples::demo::ID.into()))]);
    for tool_id in FEM2D_RETAINED_TOOL_IDS {
        let command = <Fem2dPlayApp as ArtifactEditor>::command_from_action(*tool_id, Some((args).clone()), None).unwrap_or_else(|error| panic!("action {tool_id} must resolve: {error:?}"));
        assert_eq!(command.command_id(), *tool_id);
    }
    assert!(<Fem2dPlayApp as ArtifactEditor>::command_from_action("nope", None, None).is_err());
    let node = <Fem2dPlayApp as ArtifactEditor>::command_from_action("addNode", Some((args).clone()), None).expect("addNode resolves");
    assert_eq!(node, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }));
}

/// 🧵️ LAW (design L4 / §2 D): a legacy one-per-event `canvasPointerMove` wire (no `samples`) is
/// exactly one sample at `(x, y)`; a batched wire keeps every `[x, y]` pair oldest-first; a
/// `canvasPointerUp` without `cancelled` is a real release.
#[semio_framework_async_macros::async_test]
async fn canvas_pointer_wire_defaults_samples_and_cancelled() {
    let f = semio_framework_value::DslValue::float;
    let legacy = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(640.0)), ("height".into(), f(480.0))]);
    let Fem2dCommand::CanvasPointerMove(moved) = <Fem2dPlayApp as ArtifactEditor>::command_from_action("canvasPointerMove", Some((legacy).clone()), None).expect("legacy move") else { panic!("move") };
    assert_eq!(moved.samples, vec![[5.0, 6.0]], "an absent `samples` is the single (x, y)");
    assert_eq!(moved.samples_or_last(), vec![(5.0, 6.0)]);

    let empty = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("samples".into(), semio_framework_value::DslValue::Array(Vec::new()))]);
    let Fem2dCommand::CanvasPointerMove(moved) = <Fem2dPlayApp as ArtifactEditor>::command_from_action("canvasPointerMove", Some((empty).clone()), None).expect("empty move") else { panic!("move") };
    assert_eq!(moved.samples, vec![[5.0, 6.0]], "an empty `samples` is the single (x, y)");

    let pair = |x: f64, y: f64| semio_framework_value::DslValue::Array(vec![f(x), f(y)]);
    let batched = semio_framework_value::DslValue::Object(vec![
        ("x".into(), f(3.0)),
        ("y".into(), f(4.0)),
        ("width".into(), f(640.0)),
        ("height".into(), f(480.0)),
        ("samples".into(), semio_framework_value::DslValue::Array(vec![pair(1.0, 1.5), pair(2.0, 2.5), semio_framework_value::DslValue::String("junk".into()), pair(3.0, 4.0)])),
    ]);
    let Fem2dCommand::CanvasPointerMove(moved) = <Fem2dPlayApp as ArtifactEditor>::command_from_action("canvasPointerMove", Some((batched).clone()), None).expect("batched move") else { panic!("move") };
    assert_eq!(moved.samples, vec![[1.0, 1.5], [2.0, 2.5], [3.0, 4.0]], "every well-formed pair, in order; malformed entries are skipped");
    assert_eq!((moved.x, moved.y), (3.0, 4.0), "x/y stay the last sample");

    let Fem2dCommand::CanvasPointerUp(released) = <Fem2dPlayApp as ArtifactEditor>::command_from_action("canvasPointerUp", Some((legacy).clone()), None).expect("release") else { panic!("up") };
    assert!(!released.cancelled, "an absent `cancelled` is a real release");
    let cancelled = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("cancelled".into(), semio_framework_value::DslValue::Bool(true))]);
    let Fem2dCommand::CanvasPointerUp(released) = <Fem2dPlayApp as ArtifactEditor>::command_from_action("canvasPointerUp", Some((cancelled).clone()), None).expect("cancel") else { panic!("up") };
    assert!(released.cancelled);
}
//#endregion 🔖️RetainedRoutes

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = semio_framework_pack_json::to_json_string(&create_fem2d_app());
    for id in [model_window::WINDOW_KIND_ID, results_window::WINDOW_KIND_ID] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::MODE_ID), "mode {} missing from the manifest", edit::MODE_ID);
    assert!(json.contains("computation.fem2d"), "artifact kind missing from the manifest");
    assert!(json.contains("\"transform\""), "transform utility missing from the manifest");
}

#[test]
fn window_measures_include_transform_utility_options() {
    use semio_framework_plugin::ArtifactView;
    use semio_framework_plugin::ConfigView;
    use semio_framework_plugin::HistoryView;
    use semio_framework_plugin::NoConfig;
    use semio_framework_plugin::ViewModel;
    use semio_framework_plugin::WindowMeasure;
    let doc = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("demo");
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let view_state = ViewModel { active_utility_id: Some("transform".into()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let measures = semio_s_artifact_fem_2d_editor::editor::fem2d::window_measures(&view, &cfg, &view_state);
    let model_measures = measures.get(model_window::WINDOW_KIND_ID).expect("model measures");
    let group = model_measures.iter().find_map(|measure| match measure {
        WindowMeasure::Group { id, active_utility_id, .. } if id == &format!("{}-utility-options-transform", semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::FEM2D_PLAY_CONTROLLER_ID) => Some(active_utility_id.clone()),
        _ => None,
    });
    assert_eq!(group, Some(Some("transform".into())));
}

#[semio_framework_async_macros::async_test]
async fn config_spec_declares_no_fields() {
    assert!(Fem2dPlayApp::config_spec().fields.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn manifest_declares_config_io_and_computation_artifact_kind() {
    let definition = create_fem2d_app();
    assert!(definition.config.fields.is_empty());
    assert_eq!(definition.io.artifact_schema, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA);
    let computation_kind = definition.artifact_kinds.iter().find(|kind| kind.id == "computation.fem2d").expect("computation.fem2d artifact kind declared");
    assert_eq!(computation_kind.media_type.class, MediaClass::Computation);
    assert_eq!(computation_kind.media_type.form, MediaForm::Value);
}

#[semio_framework_async_macros::async_test]
async fn app_io_forwards_the_engine_declared_ports() {
    let io = Fem2dPlayApp::io().expect("io declared");
    assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
    assert!(io.ports.iter().any(|port| port.id == "results:out"));
}

/// 🔌️ Wave-1's `required: true` unwired-input enforcement (`validate_edge_kinds`) lives in the run
/// crate, not here — this test only proves the port DECLARATION is correct; the cross-crate
/// enforcement is exercised at the run-crate level.
#[semio_framework_async_macros::async_test]
async fn fem2d_io_declares_geometry_in_and_results_out_ports() {
    let io = fem2d_io();
    assert_eq!(io.artifact_schema, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA);
    assert_eq!(io.artifact_media_type.class, MediaClass::TwoD);
    assert_eq!(io.artifact_media_type.form, MediaForm::Vector);
    assert_eq!(io.artifact.id, "2d.fem");
    assert_eq!(io.artifact.component_kind, "fem2d");

    let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
    assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
    assert!(geometry_in.required, "geometry:in is a required input port");
    assert_eq!(geometry_in.media_type.class, MediaClass::TwoD);
    assert_eq!(geometry_in.media_type.form, MediaForm::Vector);
    assert_eq!(geometry_in.multiplicity, semio_framework::PortMultiplicity::One);

    let results_out = io.ports.iter().find(|port| port.id == "results:out").expect("results:out declared");
    assert_eq!(results_out.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert!(!results_out.required, "results:out is optional");
    assert_eq!(results_out.kind_id.as_deref(), Some("computation.fem2d"));
    assert_eq!(results_out.media_type.class, MediaClass::Data);
    assert_eq!(results_out.media_type.form, MediaForm::Value);
}

//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
/// 🌱️ LAW: a fresh editor opens on the bundled example, not on an empty canvas — the model window
/// must have something to paint at first frame.
#[semio_framework_async_macros::async_test]
async fn the_editor_boots_on_the_bundled_example_document() {
    let boot = <Fem2dPlayApp as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    assert!(!boot.nodes.is_empty(), "expected the bundled example fixture's nodes");
    assert!(!boot.elements.is_empty(), "expected the bundled example fixture's elements");
    assert_ne!(boot, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot());
    assert_eq!(boot, Fem2dSnapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("the bundled example parses"));
    let app = fem2d_app();
    assert!(!app.snapshot().expect("snapshot").nodes.is_empty(), "a booted app renders a non-empty document");
}

#[semio_framework_async_macros::async_test]
async fn undo_restores_document_after_add_node() {
    let mut app = fem2d_app();
    let before = app.snapshot().expect("snapshot").nodes.len();
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(&mut app.0, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 1.0 }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = fem2d_app();
    assert!(render(&mut app, "fem2d.play.nope").contains("Unknown body"));
}

/// 🧬️ The framework's REGISTERED convergence law: two instances on one `MemoryBackbone` apply disjoint
/// edits and pump; both must project the same document. fem2d publishes bounded tool proofs, so a
/// registry-less pair faults `interactive-job.catalog-authority` before any edit lands.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_on_disjoint_edits() {
    semio_framework_plugin::artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<Fem2dPlayApp>, _, _, _>(
        "mem://fem2d-convergence",
        || async { semio_framework_plugin::App { definition: create_fem2d_app(), examples: Vec::new() } },
        Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }),
        Fem2dCommand::AddNode(add_node::AddNode { x: 5.0, y: 5.0 }),
        |app| {
            let projection = app.snapshot().expect("snapshot");
            (projection.materials.iter().any(|material| material.name == "Steel"), projection.nodes.iter().any(|node| node.x == 5.0), projection.nodes.len())
        },
        semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),
    )
    .await;
}


/// 🔌️ Mounted actors continue local commands through a short disconnected interval and converge after reconnect.
#[semio_framework_async_macros::async_test]
async fn two_actors_reconnect_after_local_edits_without_freezing_or_duplicate_replay() {
    use semio_framework_plugin::artifact_app_laws::paired_registered_apps;
    use semio_framework_plugin::artifact_app_laws::meta;
    use semio_framework_plugin::artifact_app_laws::fixture_identity;
    use semio_framework_plugin::artifact_app_laws::fixture_mounted_policy;
    use semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation;
    use semio_framework_plugin::artifact_app_laws::close_registered_fixture_app;
    use semio_framework_plugin::PluginApp;
    let row: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("portable reconnect fixture");
    let policy = fixture_mounted_policy();
    let mut identity = fixture_identity();
    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let (mut a, mut b) = paired_registered_apps::<EditorApp<Fem2dPlayApp>, _, _>("mem://fem2d-reconnect-initial", || async { context::manifest() }, policy, &mut identity).await;
    let before = a.snapshot().expect("initial projection").nodes.len();
    a.detach_backbone().await.expect("short disconnect a");
    b.detach_backbone().await.expect("short disconnect b");
    for (app, actor, key) in [(&mut a, "actor-a", "a"), (&mut b, "actor-b", "b")] {
        let point = &row[key];
        let action = meta(actor);
        semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(app, &Fem2dCommand::AddNode(add_node::AddNode { x: point["x"].as_f64().expect("x"), y: point["y"].as_f64().expect("y") }), &action, policy, &mut identity, scope.cx()).await.expect("local command remains responsive");
        settle_registered_typed_operation(app, action.instance_id, policy, &mut identity, scope.cx()).await.expect("local persisted publication");
        assert_eq!(app.snapshot().expect("disconnected local projection").nodes.len(), before + 1);
    }
    let (near, far) = store::os_store::backbone::MemoryBackbone::pair("mem://fem2d-reconnect-resumed", "mem://fem2d-reconnect-resumed").await;
    a.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("reconnect a");
    b.attach_backbone(store::os_store::backbone::Backbones::Memory(far)).await.expect("reconnect b");
    for _ in 0..3 {
        a.tick_backbone(&mut identity, scope.cx()).await.expect("a folds queued events");
        b.tick_backbone(&mut identity, scope.cx()).await.expect("b folds queued events");
    }
    let left = a.snapshot().expect("a converged projection");
    let right = b.snapshot().expect("b converged projection");
    assert_eq!(left, right);
    assert_eq!(left.nodes.len(), before + row["expectedAddedNodes"].as_u64().expect("expected extent") as usize);
    for key in ["a", "b"] {
        assert!(left.nodes.iter().any(|node| node.x == row[key]["x"].as_f64().unwrap() && node.y == row[key]["y"].as_f64().unwrap()));
    }
    a.tick_backbone(&mut identity, scope.cx()).await.expect("duplicate pump a");
    b.tick_backbone(&mut identity, scope.cx()).await.expect("duplicate pump b");
    assert_eq!(a.snapshot().expect("idempotent a"), left);
    assert_eq!(b.snapshot().expect("idempotent b"), right);
    a.detach_backbone().await.expect("release a");
    b.detach_backbone().await.expect("release b");
    close_registered_fixture_app(&mut a, policy, scope.cx()).expect("original caller paid close a");
    close_registered_fixture_app(&mut b, policy, scope.cx()).expect("original caller paid close b");
}

//#endregion 🔖️CrossCutting

//#region 🔖️ConfigIo
// (see `🔖️ManifestSanity` above for config/io declaration checks)
//#endregion 🔖️ConfigIo

//#region 🔖️ExportImportMedia
/// 🧬️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
/// see `📓️taxonomy.md`'s forbidden vocabulary), so `import_media("artifact:in")` now surfaces as a
/// `Effect::LoadDocument` carrying the replacement document's pack bytes, not an
/// `artifact_mutations` entry — asserted directly on `Emit` rather than through `app.snapshot()`.
#[semio_framework_async_macros::async_test]
async fn export_media_document_out_round_trips_via_import_media_document_in() {
    let _app = Fem2dPlayApp;
    let snapshot: Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).unwrap();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let media = Fem2dPlayApp::export_media("artifact:out", &doc).expect("document:out exports");
    assert_eq!(media.media_type.class, MediaClass::TwoD);
    assert_eq!(media.media_type.form, MediaForm::Vector);
    let empty_projection = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot();
    let empty_history = semio_framework_plugin::HistoryView::empty();
    let empty_doc = ArtifactView::new(&empty_projection, &empty_history);
    let emit = Fem2dPlayApp::import_media("artifact:in", &media, &empty_doc).expect("document:in imports");
    assert!(emit.artifact_mutations.is_empty(), "whole-document replace must not be an artifact_mutations entry");
    let semio_framework::kernel::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("document:in must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <Fem2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack).expect("decode loaded document pack");
    assert_eq!(loaded, snapshot);
}

#[semio_framework_async_macros::async_test]
async fn export_media_results_out_returns_json_with_every_case_and_combination() {
    let _app = Fem2dPlayApp;
    let snapshot: Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).unwrap();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let media = Fem2dPlayApp::export_media("results:out", &doc).expect("results:out exports");
    assert_eq!(media.media_type.class, MediaClass::Data);
    assert_eq!(media.media_type.form, MediaForm::Value);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "computation.fem2d");
            let value = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("results:out payload is valid JSON");
            for case_id in ["dead", "live", "uls"] {
                let result = value.get(case_id).unwrap_or_else(|| panic!("missing {case_id} in results:out payload: {value}"));
                assert!(result.get("displacements").is_some());
                assert!(result.get("reactions").is_some());
                assert!(result.get("checks").is_some());
            }
        }
        MediaPayload::Binary { .. } | MediaPayload::Intrinsic { .. } => panic!("expected a Structured payload"),
    }
}

#[semio_framework_async_macros::async_test]
async fn export_media_results_out_errors_when_no_load_cases_are_defined() {
    let _app = Fem2dPlayApp;
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let error = Fem2dPlayApp::export_media("results:out", &doc).expect_err("no load cases means no results to export");
    match error {
        MediaError::Payload(port, _) => assert_eq!(port, "results:out"),
        other => panic!("expected MediaError::Payload, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn export_media_unknown_port_is_not_implemented() {
    let _app = Fem2dPlayApp;
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    assert!(matches!(Fem2dPlayApp::export_media("bogus:out", &doc), Err(MediaError::NotImplemented)));
}

#[semio_framework_async_macros::async_test]
async fn import_media_geometry_in_builds_a_new_region_from_the_first_material() {
    let _app = Fem2dPlayApp;
    let mut snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot();
    snapshot.materials.push(semio_s_fem_material::planar::FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 2.1e11, nu: 0.3, rho: 7850.0 });
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let payload = semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({ "outline": [[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], "holes": [] }));
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
    let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
    assert_eq!(emit.artifact_mutations.len(), 1);
    match &emit.artifact_mutations[0] {
        Fem2dMutation::CreateRegion(semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region, .. }) => {
            assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
            assert!(region.holes.is_empty());
            assert_eq!(region.material_id, "steel");
        }
        _ => panic!("expected CreateRegion"),
    }
}

#[semio_framework_async_macros::async_test]
async fn import_media_geometry_in_falls_back_to_unassigned_material_when_none_exists() {
    let _app = Fem2dPlayApp;
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let payload = semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({ "outline": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]] }));
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
    let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
    match &emit.artifact_mutations[0] {
        Fem2dMutation::CreateRegion(semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region, .. }) => assert_eq!(region.material_id, "unassigned"),
        _ => panic!("expected CreateRegion"),
    }
}
//#endregion 🔖️ExportImportMedia

/// 👥️ The mounted camera and declared interaction use the shared transport and retire on departure.
#[semio_framework_async_macros::async_test]
async fn mounted_camera_and_interaction_presence_reach_peers_without_document_edits() {
    use semio_framework_value::FromValue;
    use semio_framework_plugin::artifact_app_laws::assert_registered_presence_round_trip;
    use semio_framework_plugin::artifact_app_laws::fixture_identity;
    use semio_framework_plugin::artifact_app_laws::fixture_mounted_policy;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/📷️camera/🧪️tests/🧫️fixtures/🔣️.json")).expect("portable shared presence fixture");
    let expected = protocol::PresenceWindowView::from_value(semio_framework_value::DslValue::from(&fixture["admission"][0]["value"])).expect("admitted shared camera");
    let boot = <Fem2dPlayApp as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let node = boot.nodes.first().expect("bundled structural node");
    let selected = protocol::wire::encode_scoped_identity(&["node", &node.id]).expect("canonical node target");
    let window_kind = model_window::WINDOW_KIND_ID;
    let view = semio_framework_plugin::ViewModel { window_id: Some("model-presence".into()), window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "model-presence".into(), window_kind_id: window_kind.into() }], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let protocol::PresenceViewKind::Canvas { x, y, zoom } = expected.kind else { panic!("portable canvas pose") };
    let command = Fem2dCommand::SetCamera(Fem2dCommand::SetCamera { x, y, zoom });
    assert_registered_presence_round_trip::<EditorApp<Fem2dPlayApp>, _, _>(|| async { context::manifest() }, command, view, expected, ("fem2d", Some(("node", &selected))), fixture_mounted_policy(), &mut fixture_identity()).await;
}