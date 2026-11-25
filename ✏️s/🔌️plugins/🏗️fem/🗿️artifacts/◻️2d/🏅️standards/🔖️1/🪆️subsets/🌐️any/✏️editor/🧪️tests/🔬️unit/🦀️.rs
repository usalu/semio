use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use crate::context;
use crate::media_ports::editing_doc;


use semio_framework::InteractiveJobClassification;
use semio_plugin::{ArtifactToolPublicationLane, ArtifactView, ConfigView, NoConfig};
use semio_s_artifact_fem_2d_editor::editor::fem2d::{create_fem2d_app, fem2d_io, modes::edit};
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::{gumball, result_animation_tick};
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::{
    add_area_load, add_bar, add_beam, add_combination, add_load_case, add_member_udl,
    add_nodal_load, add_node, add_region, add_section, add_support, patch_combination,
    patch_element, patch_load, patch_load_case, patch_material, patch_node, patch_region,
    patch_section, patch_support, remove_selection, set_active_example, set_analysis_settings,
    set_self_weight,
};
use super::*;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::InteractionRef;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::MediaClass;
use semio_plugin::MediaForm;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::PluginAssemblyError;
use semio_plugin::UtilityCategory;
use semio_plugin::UtilityDefinition;
use semio_plugin::ViewModel;
use semio_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use crate::context::{fem2d_app, render};
use semio_plugin::{ArtifactEditor, EditorApp};
use pack::snapshot::ArtifactDsl;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_display;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_camera;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_animation;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dRetainedCommandJobFactory;
#[cfg(test)] use semio_plugin::ArtifactOwnedToolJobFactory;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_down;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_move;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_up;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::focus_entity;
#[cfg(test)] use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_material;
#[cfg(test)] use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results as results_window;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
#[cfg(test)] use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp;

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
        Fem2dCommand::TranslateSelection(gumball::translate_selection::TranslateSelection { ids: vec!["scope:1:4:node2:n1".into()], dx: 0.1, dy: 0.0, dz: 0.0, phase: None, reason: None }),
        Fem2dCommand::RotateSelection(gumball::rotate_selection::RotateSelection { ids: vec!["scope:1:4:node2:n1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 0.1, phase: None, reason: None }),
        Fem2dCommand::ScaleSelection(gumball::scale_selection::ScaleSelection { ids: vec!["scope:1:4:node2:n1".into()], sx: 1.01, sy: 1.01, sz: 1.0, phase: None, reason: None }),
        Fem2dCommand::SetTransformGumballFlag(gumball::set_transform_gumball_flag::SetTransformGumballFlag { flag: "move".into(), pressed: None }),
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
        semio_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
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
        ("010e021273636f70653a313a343a6e6f6465323a6e311573636f70653a313a373a656c656d656e74323a653101000c0206000601", Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["scope:1:4:node2:n1".into(), "scope:1:7:element2:e1".into()] })),
        ("010f010764656661756c7401000600", Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() })),
        ("011000030005000000000000f03f010500000000000000400205000000000000f83f", Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 })),
        ("011102056d6f64616c1873636f70653a313a383a6c6f616443617365343a6465616403000601010600020400", Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some(semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").expect("source")), mode: "modal".into(), mode_index: 0, field: None, value: None, window_id: None })),
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
    let commands: BTreeSet<&str> = every_command().iter().map(Fem2dCommand::command_id).collect();
    let routes: BTreeSet<&str> = Fem2dRetainedCommandJobFactory::TOOL_IDS.iter().copied().collect();
    assert_eq!(routes.len(), Fem2dRetainedCommandJobFactory::TOOL_IDS.len(), "no duplicate retained route ids");
    assert_eq!(commands, routes, "every declared command is a retained route and vice versa");
    assert_eq!(<Fem2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), Fem2dRetainedCommandJobFactory::TOOL_IDS.len());
    assert_eq!(Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), Fem2dRetainedCommandJobFactory::TOOL_IDS.len());
    let definition = create_fem2d_app();
    for tool_id in Fem2dRetainedCommandJobFactory::TOOL_IDS {
        let contract = Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == *tool_id).unwrap_or_else(|| panic!("publication contract for {tool_id}"));
        assert!(!contract.lanes.is_empty(), "{tool_id} publishes into at least one lane");
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} declared"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be Migrated to dispatch interactively");
    }
}

/// 🖱️ LAW: both canvas windows open with Direct Select armed, so the first canvas click selects without picking a utility.
#[test]
fn canvas_windows_arm_direct_select_initially() {
    let definition = create_fem2d_app();
    for kind in [model_window::WINDOW_KIND_ID, results_window::WINDOW_KIND_ID] {
        let window = definition.window_kinds.iter().find(|window| window.id == kind).unwrap_or_else(|| panic!("window kind {kind}"));
        assert_eq!(window.initial_utility_id.as_ref().map(|id| id.as_str()), Some("selectDirect"), "{kind}");
    }
}

/// ⌨️ LAW: undo binds Ctrl+Z and redo binds both Ctrl+Shift+Z and Ctrl+Y, the two redo chords desktop users reach for.
#[test]
fn history_keys_bind_undo_and_both_redo_chords() {
    let definition = create_fem2d_app();
    let bound = |action: &str| definition.keybindings.iter().filter(|binding| binding.action.action == action).map(|binding| binding.keys.as_str()).collect::<Vec<_>>();
    assert_eq!(bound("undo"), ["mod+z"]);
    assert_eq!(bound("redo"), ["mod+shift+z", "mod+y"]);
}

/// 🛣️ LAW: a route's declared lanes cover exactly what its own handler emits — the framework
/// refuses a completion whose emitted lane is absent from the contract.
#[semio_framework_async_macros::async_test]
async fn every_route_declares_the_lane_its_handler_emits() {
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().expect("bundled demo loads"));
    let history = semio_plugin::HistoryView::empty();
    let doc = editing_doc(&snapshot, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    for command in every_command() {
        let tool_id = command.command_id();
        let view = |id: &str, kind: &str| semio_plugin::ViewModel { window_id: Some(id.into()), window_instances: vec![semio_plugin::ViewWindowInstance { id: id.into(), window_kind_id: kind.into() }], ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
        let emit = match &command {
            Fem2dCommand::SetCamera(payload) => set_camera::handle_window(payload, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::SetResultDisplay(payload) => set_result_display::handle_window(payload, &cfg, &view("results-law", results_window::WINDOW_KIND_ID)),
            Fem2dCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle_window(payload, &doc, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle_window(payload, &doc, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::CanvasPointerUp(payload) => canvas_pointer_up::handle_window(payload, &doc, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::FocusEntity(payload) => focus_entity::handle_window(payload, &doc, &cfg, &view("model-law", model_window::WINDOW_KIND_ID)),
            Fem2dCommand::SetResultAnimation(payload) => set_result_animation::handle_window(payload, &doc, &cfg, &view("results-law", results_window::WINDOW_KIND_ID)),
            Fem2dCommand::ResultAnimationTick(payload) => result_animation_tick::result_animation_tick_step::<semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_animation::Fem2dResultsPlayback>(payload, &cfg, &view("results-left", results_window::WINDOW_KIND_ID), None).map(|step| step.emit),
            Fem2dCommand::SetTransformGumballFlag(payload) => gumball::set_transform_gumball_flag::handle_window(payload, &view("model-law", model_window::WINDOW_KIND_ID)),
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

fn resolve_action(action: &str, mut args: Option<semio_framework_value::DslValue>) -> Result<Fem2dCommand, Fault> {
    <Fem2dPlayApp as ArtifactEditor>::command_from_action(action, &mut args, &mut None)
}

/// 🎯️ LAW: the stringly `{action, args}` wire every shell speaks resolves to the typed command
/// with the same id — the trait's default rejects app actions outright, so this bridge is the only
/// path from a rendered button to `dispatch`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_resolves_every_declared_action() {
    let args = semio_framework_value::DslValue::Object(vec![("x".into(), semio_framework_value::DslValue::float(1.0)), ("y".into(), semio_framework_value::DslValue::float(2.0)), ("exampleId".into(), semio_framework_value::DslValue::String(semio_s_artifact_fem_2d_io::examples::demo::ID.into()))]);
    for tool_id in Fem2dRetainedCommandJobFactory::TOOL_IDS {
        let command = resolve_action(*tool_id, Some((args).clone())).unwrap_or_else(|error| panic!("action {tool_id} must resolve: {error:?}"));
        assert_eq!(command.command_id(), *tool_id);
    }
    assert!(resolve_action("nope", None).is_err());
    let node = resolve_action("addNode", Some((args).clone())).expect("addNode resolves");
    assert_eq!(node, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }));
}

/// 🧵️ LAW (design L4 / §2 D): a legacy one-per-event `canvasPointerMove` wire (no `samples`) is
/// exactly one sample at `(x, y)`; a batched wire keeps every `[x, y]` pair oldest-first; a
/// `canvasPointerUp` without `cancelled` is a real release.
#[semio_framework_async_macros::async_test]
async fn canvas_pointer_wire_defaults_samples_and_cancelled() {
    let f = semio_framework_value::DslValue::float;
    let legacy = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(640.0)), ("height".into(), f(480.0))]);
    let Fem2dCommand::CanvasPointerMove(moved) = resolve_action("canvasPointerMove", Some((legacy).clone())).expect("legacy move") else { panic!("move") };
    assert_eq!(moved.samples, vec![[5.0, 6.0]], "an absent `samples` is the single (x, y)");
    assert_eq!(moved.samples_or_last(), vec![(5.0, 6.0)]);

    let empty = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("samples".into(), semio_framework_value::DslValue::Array(Vec::new()))]);
    let Fem2dCommand::CanvasPointerMove(moved) = resolve_action("canvasPointerMove", Some((empty).clone())).expect("empty move") else { panic!("move") };
    assert_eq!(moved.samples, vec![[5.0, 6.0]], "an empty `samples` is the single (x, y)");

    let pair = |x: f64, y: f64| semio_framework_value::DslValue::Array(vec![f(x), f(y)]);
    let batched = semio_framework_value::DslValue::Object(vec![
        ("x".into(), f(3.0)),
        ("y".into(), f(4.0)),
        ("width".into(), f(640.0)),
        ("height".into(), f(480.0)),
        ("samples".into(), semio_framework_value::DslValue::Array(vec![pair(1.0, 1.5), pair(2.0, 2.5), semio_framework_value::DslValue::String("junk".into()), pair(3.0, 4.0)])),
    ]);
    let Fem2dCommand::CanvasPointerMove(moved) = resolve_action("canvasPointerMove", Some((batched).clone())).expect("batched move") else { panic!("move") };
    assert_eq!(moved.samples, vec![[1.0, 1.5], [2.0, 2.5], [3.0, 4.0]], "every well-formed pair, in order; malformed entries are skipped");
    assert_eq!((moved.x, moved.y), (3.0, 4.0), "x/y stay the last sample");

    let Fem2dCommand::CanvasPointerUp(released) = resolve_action("canvasPointerUp", Some((legacy).clone())).expect("release") else { panic!("up") };
    assert!(!released.cancelled, "an absent `cancelled` is a real release");
    let cancelled = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("cancelled".into(), semio_framework_value::DslValue::Bool(true))]);
    let Fem2dCommand::CanvasPointerUp(released) = resolve_action("canvasPointerUp", Some((cancelled).clone())).expect("cancel") else { panic!("up") };
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
    use semio_plugin::ArtifactView;
    use semio_plugin::ConfigView;
    use semio_plugin::HistoryView;
    use semio_plugin::NoConfig;
    use semio_plugin::ViewModel;
    use semio_plugin::WindowMeasure;
    let doc = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring::new(Fem2dSnapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("demo"));
    let history = HistoryView::empty();
    let view = ArtifactView::new(&*doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let view_state = ViewModel { active_utility_id: Some("transform".into()), ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
    let measures = <Fem2dPlayApp as ArtifactEditor>::window_measures(&view, &cfg, &view_state);
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
    assert_eq!(geometry_in.direction, semio_plugin::MediaPortDirection::In);
    assert!(geometry_in.required, "geometry:in is a required input port");
    assert_eq!(geometry_in.media_type.class, MediaClass::TwoD);
    assert_eq!(geometry_in.media_type.form, MediaForm::Vector);
    assert_eq!(geometry_in.multiplicity, semio_framework::PortMultiplicity::One);

    let results_out = io.ports.iter().find(|port| port.id == "results:out").expect("results:out declared");
    assert_eq!(results_out.direction, semio_plugin::MediaPortDirection::Out);
    assert!(!results_out.required, "results:out is optional");
    assert_eq!(results_out.kind_id.as_deref(), Some("computation.fem2d"));
    assert_eq!(results_out.media_type.class, MediaClass::Data);
    assert_eq!(results_out.media_type.form, MediaForm::Value);
}

//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn undo_restores_document_after_add_node() {
    let mut app = fem2d_app();
    let before = app.snapshot().expect("snapshot").nodes.len();
    semio_plugin::artifact_app_laws::assert_undo_redo_round_trip(&mut app.0, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 1.0 }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1, semio_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
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
    semio_plugin::artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<Fem2dPlayApp>, _, _, _>(
        "mem://fem2d-convergence",
        || async { semio_plugin::App { definition: create_fem2d_app(), examples: Vec::new() } },
        Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }),
        Fem2dCommand::AddNode(add_node::AddNode { x: 5.0, y: 5.0 }),
        |app| {
            let projection = app.snapshot().expect("snapshot");
            (projection.materials.iter().any(|material| material.name.as_str() == "Steel"), projection.nodes.iter().any(|node| node.x == 5.0), projection.nodes.len())
        },
        semio_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin::artifact_app_laws::fixture_identity(),
    )
    .await;
}


/// 🔌️ Mounted actors continue local commands through a short disconnected interval and converge after reconnect.
#[semio_framework_async_macros::async_test]
async fn two_actors_reconnect_after_local_edits_without_freezing_or_duplicate_replay() {
    use semio_plugin::artifact_app_laws::paired_registered_apps;
    use semio_plugin::artifact_app_laws::meta;
    use semio_plugin::artifact_app_laws::fixture_identity;
    use semio_plugin::artifact_app_laws::fixture_mounted_policy;
    use semio_plugin::artifact_app_laws::settle_registered_typed_operation;
    use semio_plugin::artifact_app_laws::close_registered_fixture_app;
    use semio_plugin::PluginApp;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring as Retiring;
    let row: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("portable reconnect fixture");
    let policy = fixture_mounted_policy();
    let mut identity = fixture_identity();
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let (mut a, mut b) = paired_registered_apps::<EditorApp<Fem2dPlayApp>, _, _>("mem://fem2d-reconnect-initial", || async { context::manifest() }, policy, &mut identity).await;
    let before = Retiring::new(a.snapshot().expect("initial projection")).nodes.len();
    a.detach_backbone().await.expect("short disconnect a");
    b.detach_backbone().await.expect("short disconnect b");
    for (app, actor, key) in [(&mut a, "actor-a", "a"), (&mut b, "actor-b", "b")] {
        let point = &row[key];
        let action = meta(actor);
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(app, &Fem2dCommand::AddNode(add_node::AddNode { x: point["x"].as_f64().expect("x"), y: point["y"].as_f64().expect("y") }), &action, policy, &mut identity, scope.cx()).await.expect("local command remains responsive");
        settle_registered_typed_operation(app, action.instance_id, policy, &mut identity, scope.cx()).await.expect("local persisted publication");
        assert_eq!(Retiring::new(app.snapshot().expect("disconnected local projection")).nodes.len(), before + 1);
    }
    let (near, far) = store::os_store::backbone::MemoryBackbone::pair("mem://fem2d-reconnect-resumed", "mem://fem2d-reconnect-resumed").await;
    a.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("reconnect a");
    b.attach_backbone(store::os_store::backbone::Backbones::Memory(far)).await.expect("reconnect b");
    for _ in 0..3 {
        a.tick_backbone(&mut identity, scope.cx()).await.expect("a folds queued events");
        b.tick_backbone(&mut identity, scope.cx()).await.expect("b folds queued events");
    }
    let left = Retiring::new(a.snapshot().expect("a converged projection"));
    let right = Retiring::new(b.snapshot().expect("b converged projection"));
    assert_eq!(*left, *right);
    assert_eq!(left.nodes.len(), before + row["expectedAddedNodes"].as_u64().expect("expected extent") as usize);
    for key in ["a", "b"] {
        assert!(left.nodes.iter().any(|node| node.x == row[key]["x"].as_f64().unwrap() && node.y == row[key]["y"].as_f64().unwrap()));
    }
    a.tick_backbone(&mut identity, scope.cx()).await.expect("duplicate pump a");
    b.tick_backbone(&mut identity, scope.cx()).await.expect("duplicate pump b");
    assert_eq!(*Retiring::new(a.snapshot().expect("idempotent a")), *left);
    assert_eq!(*Retiring::new(b.snapshot().expect("idempotent b")), *right);
    a.detach_backbone().await.expect("release a");
    b.detach_backbone().await.expect("release b");
    close_registered_fixture_app(&mut a, policy, scope.cx()).expect("original caller paid close a");
    close_registered_fixture_app(&mut b, policy, scope.cx()).expect("original caller paid close b");
}

//#endregion 🔖️CrossCutting

//#region 🔖️ConfigIo
// (see `🔖️ManifestSanity` above for config/io declaration checks)
//#endregion 🔖️ConfigIo



/// 👥️ The mounted camera and declared interaction use shared presence and retire on departure.
#[semio_framework_async_macros::async_test]
async fn mounted_camera_and_interaction_presence_reach_peers_without_document_edits() {
    use semio_framework_value::{DslValue, FromValue};
    use semio_os_kernel::{ActorId, Viewport2d, LOCAL_ACTOR_ID};
    use semio_plugin::artifact_app_laws::{
        OriginalFixtureScope, capture_fixture_window_config, close_registered_fixture_app,
        dispatch_fixture_action, dispatch_original_fixture_command, fixture_identity,
        fixture_mounted_policy, meta, new_app_with_registry_boxed, publish_fixture_presence_roster,
        settle_registered_typed_operation,
    };
    use semio_plugin::{INTERACTION_SELECT_ACTION_ID, PluginApp};
    use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::{
        FEM2D_GRANULARITY_NODE, FEM2D_INTERACTION_DOMAIN, fem2d_entity_target_id,
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/📷️camera/🧪️tests/🧫️fixtures/🔣️.json")).expect("shared camera fixture");
    let expected = protocol::PresenceWindowView::from_value(DslValue::from(&fixture["admission"][0]["value"])).expect("canvas camera");
    let protocol::PresenceViewKind::Canvas { x, y, zoom } = expected.kind else { panic!("canvas camera") };
    let policy = fixture_mounted_policy();
    let mut identity = fixture_identity();
    let mut scope = OriginalFixtureScope::admit();
    let mut source = new_app_with_registry_boxed::<EditorApp<Fem2dPlayApp>>(context::manifest, ActorId(LOCAL_ACTOR_ID.into()), policy, &mut identity).await;
    let mut peer = new_app_with_registry_boxed::<EditorApp<Fem2dPlayApp>>(context::manifest, ActorId(LOCAL_ACTOR_ID.into()), policy, &mut identity).await;
    let mut action = meta(LOCAL_ACTOR_ID);
    source.bind_instance_id(action.instance_id).await;
    peer.bind_instance_id(action.instance_id + 1).await;
    let source_before = source.document_pack().await.expect("source document");
    let peer_before = peer.document_pack().await.expect("peer document");
    let selected = {
        let snapshot = source.snapshot_ref();
        let node = snapshot.nodes.iter().next().expect("bundled structural node");
        fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, node.id.as_str()).expect("canonical node")
    };
    let view = ViewModel {
        window_id: Some(expected.window_id.clone()),
        window_instances: vec![semio_plugin::ViewWindowInstance { id: expected.window_id.clone(), window_kind_id: model_window::WINDOW_KIND_ID.into() }],
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    action.view_state = Some(view.clone().into());
    let camera = Fem2dCommand::SetCamera(set_camera::SetCamera { x, y, zoom });
    dispatch_original_fixture_command(&mut *source, &camera, &action, policy, &mut identity, scope.cx()).await.expect("dispatch model camera");
    settle_registered_typed_operation(&mut *source, action.instance_id, policy, &mut identity, scope.cx()).await.expect("commit model camera");
    let config = capture_fixture_window_config::<model_window::config::Fem2dModelWindowConfigOwner, _, _>(&mut *source, &view).await.expect("capture model camera").expect("window config");
    assert_eq!(config.camera, Viewport2d { x, y, zoom });
    let selection = serde_json::json!([{ "granularity": FEM2D_GRANULARITY_NODE, "id": selected }]).to_string();
    let args = DslValue::object([
        ("domainId".into(), DslValue::String(FEM2D_INTERACTION_DOMAIN.into())),
        ("merge".into(), DslValue::String("replace".into())),
        ("method".into(), DslValue::String("pick".into())),
        ("targets".into(), DslValue::String(selection)),
    ]);
    let admitted = dispatch_fixture_action(&mut *source, INTERACTION_SELECT_ACTION_ID, Some(args), &action, &mut identity, scope.cx(), policy.maintenance).await.expect("select model node");
    semio_plugin::artifact_app_laws::settle_framework_reserved_admission(&mut *source, admitted, scope.cx()).await.expect("settle model node selection");
    let local = source.ephemeral_snapshot().await;
    let interaction = protocol::decode_presence_interaction(&local.interaction, &mut 0).await.expect("source interaction is published");
    assert!(interaction.domains.iter().any(|domain| domain.domain == FEM2D_INTERACTION_DOMAIN && domain.selected == [selected.clone()]));
    assert_eq!(source.document_pack().await.expect("source document after publication"), source_before);
    let remote = protocol::PresencePeer {
        actor: "fem2d-source".into(), connected_at_ms: 1, label: None, presence_pack: None,
        user_id: None, role: None, drag_ghost_json: None, interaction: Some(interaction.clone()),
        color: Some(1), surface: None, views: vec![expected.clone()], ui: None, tool_run: None,
        principal_kind: None, active_tool: None, history_edit: None, typing: Vec::new(),
    };
    let bytes = protocol::encode_presence_peer(&remote).await;
    let received = protocol::decode_presence_peer(&bytes).await.expect("shared peer wire");
    assert_eq!(received.views, [expected]);
    assert!(publish_fixture_presence_roster(&mut *peer, 1, Some(9), &[received], 1_000, scope.cx(), policy.close).await.is_none());
    assert!(peer.roster.peer_presence_root().iter().any(|(actor, presence)| actor == "fem2d-source" && presence.interaction.as_ref() == Some(&interaction)));
    assert_eq!(peer.document_pack().await.expect("peer document after delivery"), peer_before);
    assert!(publish_fixture_presence_roster(&mut *peer, 2, Some(9), &[], 2_000, scope.cx(), policy.close).await.is_none());
    assert!(peer.roster.peer_presence_root().is_empty());
    assert_eq!(peer.document_pack().await.expect("peer document after departure"), peer_before);
    close_registered_fixture_app(&mut *source, policy, scope.cx()).expect("retire source");
    close_registered_fixture_app(&mut *peer, policy, scope.cx()).expect("retire peer");
}
