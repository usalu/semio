
use super::*;
use crate::editor::fem2d::testkit::{fem2d_app, render};
use semio_framework_plugin::{ArtifactEditor, EditorApp, PluginApp};
use store::ArtifactDsl;

//#region 🔖️CommandSurface
/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Fem2dCommand> {
    vec![
        Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }),
        Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
        Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }),
        Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }),
        Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 }),
        Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![crate::FemDof::Tx, crate::FemDof::Ty] }),
        Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) }),
        Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None }),
        Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) }),
        Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None }),
        Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }),
        Fem2dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: vec![crate::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] }),
        Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true }),
        Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) }),
        Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] }),
        Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
        Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 }),
        Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }),
        Fem2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
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
    assert_eq!(ids.len(), 19, "every Fem2dCommand row must be covered by every_command()");
    // 🏷️ Unlike flow's setLocale/flowEvalTick, every one of fem2d's 19 commands (including
    // setLocale) has a real manifest action declaration — see `create_fem2d_app`'s `.view_action`
    // calls.
    let definition = create_fem2d_app();
    for id in ids {
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == id), "command_id {id} must be a declared action");
    }
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 📌️ LAW: the pre-migration command wire format, row for row, INCLUDING both `Option` shapes of
/// every row whose `None`/`Some` cases encode differently. Every hex string was dumped from the old
/// `📡️protocol` crate's `Fem2dCommand` before `app_commands!` rebuilt the enum (ticket
/// `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
/// `🧪️wire-baseline-before-2d.txt`). Row order is the binary variant ordinal, so a reordering — which
/// no round-trip law can catch — shows up here as a leading-byte mismatch.
#[semio_framework_async_macros::async_test]
async fn every_command_keeps_its_pre_migration_bytes() {
    use protocol::OpBinary;
    let rows: Vec<(&str, Fem2dCommand)> = vec![
        ("010000020005000000000000f03f01050000000000000040", Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 })),
        ("010104026d31026e31026e3202733104000601010602020600030603", Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() })),
        ("010204026d31026e31026e3202733104000601010602020600030603", Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() })),
        ("01030105737465656c020006000105000000da7c724842", Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "steel".into(), e: 210e9 })),
        ("01040106697065333030030006000105a4005130630a763f020509c577de9de7153f", Fem2dCommand::AddSection(add_section::AddSection { name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 })),
        ("010501026e31020006000116020002", Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![crate::FemDof::Tx, crate::FemDof::Ty] })),
        ("010602046c697665026e3104000601010a010205000000000088b3c0030600", Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) })),
        ("010601026e3103000600010a010205000000000088b3c0", Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::FemDof::Ty, value: -5000.0, case_id: None })),
        ("010701026531030006000105000000000000000002050000000000407fc0", Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None })),
        ("0108020464656164027231030006010105000000000088b340020600", Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) })),
        (
            "01090105737465656c060005000000000000000001050000000000000000020500000000000010400305000000000000004004060005057b14ae47e17a943f",
            Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None }),
        ),
        ("010a01044c697665020006000101", Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })),
        (
            "010b0303554c530464656164046c69766502000600010c020d0200060101059a9999999999f53f0d020006020105000000000000f83f",
            Fem2dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: vec![crate::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] }),
        ),
        ("010c010464656164020006000102", Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true })),
        ("010d000200040502050000000000003e40", Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) })),
        ("010e02026531026e3101000c0206010600", Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] })),
        ("010f010764656661756c7401000600", Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() })),
        ("011000030005000000000000f03f010500000000000000400205000000000000f83f", Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 })),
        ("0111020464656164056d6f64616c03000600010601020400", Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 })),
        ("0112010564652d444501000600", Fem2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })),
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
    let routes: BTreeSet<&str> = FEM2D_RETAINED_TOOL_IDS.iter().copied().collect();
    assert_eq!(routes.len(), FEM2D_RETAINED_TOOL_IDS.len(), "no duplicate retained route ids");
    assert_eq!(commands, routes, "every declared command is a retained route and vice versa");
    assert_eq!(<Fem2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), FEM2D_RETAINED_TOOL_IDS.len());
    assert_eq!(Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), FEM2D_RETAINED_TOOL_IDS.len());
    let definition = create_fem2d_app();
    for tool_id in FEM2D_RETAINED_TOOL_IDS {
        let contract = Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == *tool_id).unwrap_or_else(|| panic!("publication contract for {tool_id}"));
        assert!(!contract.lanes.is_empty(), "{tool_id} publishes into at least one lane");
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} declared"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be Migrated to dispatch interactively");
    }
}

/// 🛣️ LAW: a route's declared lanes cover exactly what its own handler emits — the framework
/// refuses a completion whose emitted lane is absent from the contract.
#[semio_framework_async_macros::async_test]
async fn every_route_declares_the_lane_its_handler_emits() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = Fem2dConfig::default();
    let cfg = ConfigView { snapshot: &config };
    for command in every_command() {
        let tool_id = command.command_id();
        let emit = command.dispatch(&doc, &cfg).unwrap_or_else(|error| panic!("{tool_id} dispatches: {error:?}"));
        let lanes = Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == tool_id).expect("publication contract").lanes;
        assert!(emit.artifact_mutations.is_empty() || lanes.contains(&ArtifactToolPublicationLane::Artifact), "{tool_id} emits document mutations without the Artifact lane");
        assert!(emit.config_mutations.is_empty() || lanes.contains(&ArtifactToolPublicationLane::Config), "{tool_id} emits config mutations without the Config lane");
    }
}

/// 🎯️ LAW: the stringly `{action, args}` wire every shell speaks resolves to the typed command
/// with the same id — the trait's default rejects app actions outright, so this bridge is the only
/// path from a rendered button to `dispatch`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_resolves_every_declared_action() {
    let args = dsl::DslValue::Object(vec![("x".into(), dsl::DslValue::float(1.0)), ("y".into(), dsl::DslValue::float(2.0)), ("exampleId".into(), dsl::DslValue::String(crate::examples::demo::ID.into()))]);
    for tool_id in FEM2D_RETAINED_TOOL_IDS {
        let command = <Fem2dPlayApp as ArtifactEditor>::command_from_action(*tool_id, Some(&args)).unwrap_or_else(|error| panic!("action {tool_id} must resolve: {error:?}"));
        assert_eq!(command.command_id(), *tool_id);
    }
    assert!(<Fem2dPlayApp as ArtifactEditor>::command_from_action("nope", None).is_err());
    let node = <Fem2dPlayApp as ArtifactEditor>::command_from_action("addNode", Some(&args)).expect("addNode resolves");
    assert_eq!(node, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }));
}
//#endregion 🔖️RetainedRoutes

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = dsl::json::to_json_string(&create_fem2d_app());
    for id in [model_window::WINDOW_KIND_ID, results_window::WINDOW_KIND_ID] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::MODE_ID), "mode {} missing from the manifest", edit::MODE_ID);
    assert!(json.contains("computation.fem2d"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn config_spec_declares_no_fields() {
    assert!(Fem2dPlayApp::config_spec().fields.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn manifest_declares_config_io_and_computation_artifact_kind() {
    let definition = create_fem2d_app();
    assert!(definition.config.fields.is_empty());
    assert_eq!(definition.io.document_schema, crate::FEM_2D_SCHEMA);
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
    assert_eq!(io.document_schema, crate::FEM_2D_SCHEMA);
    assert_eq!(io.document_media_type.class, MediaClass::TwoD);
    assert_eq!(io.document_media_type.form, MediaForm::Vector);
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

/// 🗣️ B1: the manifest itself (not a runtime `cfg.locale`-driven overlay) now carries every
/// locale's translation via `LocalizedLabel`.
#[semio_framework_async_macros::async_test]
async fn manifest_labels_resolve_german_locale_2d() {
    use semio_framework_plugin::{Locale, Terminology};
    let definition = create_fem2d_app();
    let window_model = definition.window_kinds.iter().find(|window| window.id == model_window::WINDOW_KIND_ID).expect("model window kind declared");
    assert_eq!(window_model.label.resolve(Terminology::Native, Locale::En), "Model");
    assert_eq!(window_model.label.resolve(Terminology::Native, Locale::De), "Modell");
    let add_node_action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "addNode").expect("addNode action declared");
    assert_eq!(add_node_action.label.resolve(Terminology::Native, Locale::En), "Add Node");
    assert_eq!(add_node_action.label.resolve(Terminology::Native, Locale::De), "Knoten hinzufügen");
    let set_locale_action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setLocale").expect("setLocale action declared");
    assert_eq!(set_locale_action.label.resolve(Terminology::Native, Locale::En), "Set Locale");
    assert_eq!(set_locale_action.label.resolve(Terminology::Native, Locale::De), "Sprache festlegen");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
/// 🌱️ LAW: a fresh editor opens on the bundled example, not on an empty canvas — the model window
/// must have something to paint at first frame.
#[semio_framework_async_macros::async_test]
async fn the_editor_boots_on_the_bundled_example_document() {
    let boot = <Fem2dPlayApp as ArtifactEditor>::initial_snapshot();
    assert!(!boot.nodes.is_empty(), "expected the bundled example fixture's nodes");
    assert!(!boot.elements.is_empty(), "expected the bundled example fixture's elements");
    assert_ne!(boot, crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot());
    assert_eq!(boot, Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL).expect("the bundled example parses"));
    let app = fem2d_app();
    assert!(!app.snapshot().expect("snapshot").nodes.is_empty(), "a booted app renders a non-empty document");
}

#[semio_framework_async_macros::async_test]
async fn undo_restores_document_after_add_node() {
    let mut app = fem2d_app();
    let before = app.snapshot().expect("snapshot").nodes.len();
    semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 1.0 }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1).await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = fem2d_app();
    assert!(render(&mut app, "fem2d.play.nope").contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_on_disjoint_edits() {
    let (mut instance_a, mut instance_b) = semio_framework_plugin::resolve_ready(semio_framework_plugin::testkit::paired_apps::<EditorApp<Fem2dPlayApp>>("mem://fem2d-convergence"));

    instance_a.dispatch_typed(Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }), &semio_framework_plugin::testkit::meta("actor-a")).await.expect("a adds a material");
    instance_b.dispatch_typed(Fem2dCommand::AddNode(add_node::AddNode { x: 5.0, y: 5.0 }), &semio_framework_plugin::testkit::meta("actor-b")).await.expect("b adds a node");

    // A neutral history action always dispatches through the store, which pumps inbound operations first.
    semio_framework_plugin::resolve_ready(instance_a.handle_action("commitCheckpoint", None, &semio_framework_plugin::testkit::meta("actor-a"))).expect("pump a");
    semio_framework_plugin::resolve_ready(instance_b.handle_action("commitCheckpoint", None, &semio_framework_plugin::testkit::meta("actor-b"))).expect("pump b");

    let projection_a = instance_a.snapshot().expect("snapshot a");
    let projection_b = instance_b.snapshot().expect("snapshot b");
    assert!(projection_a.materials.iter().any(|m| m.name == "Steel"), "A keeps its material");
    assert!(projection_a.nodes.iter().any(|n| n.x == 5.0), "A absorbs B's node");
    assert_eq!(projection_a.nodes.len(), projection_b.nodes.len(), "both instances converge to the same node set");
}
//#endregion 🔖️CrossCutting

//#region 🔖️ConfigIo
// (see `🔖️ManifestSanity` above for config/io declaration checks)
//#endregion 🔖️ConfigIo

//#region 🔖️ExportImportMedia
/// 🧬️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
/// see `📓️taxonomy.md`'s forbidden vocabulary), so `import_media("document:in")` now surfaces as a
/// `Effect::LoadDocument` carrying the replacement document's pack bytes, not an
/// `artifact_mutations` entry — asserted directly on `Emit` rather than through `app.snapshot()`.
#[semio_framework_async_macros::async_test]
async fn export_media_document_out_round_trips_via_import_media_document_in() {
    let _app = Fem2dPlayApp;
    let snapshot: Fem2dSnapshot = Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let media = Fem2dPlayApp::export_media("document:out", &doc).expect("document:out exports");
    assert_eq!(media.media_type.class, MediaClass::TwoD);
    assert_eq!(media.media_type.form, MediaForm::Vector);
    let empty_projection = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
    let empty_history = semio_framework_plugin::HistoryView::empty();
    let empty_doc = ArtifactView::new(&empty_projection, &empty_history);
    let emit = Fem2dPlayApp::import_media("document:in", &media, &empty_doc).expect("document:in imports");
    assert!(emit.artifact_mutations.is_empty(), "whole-document replace must not be an artifact_mutations entry");
    let semio_framework::kernel::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("document:in must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <Fem2dSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode loaded document pack");
    assert_eq!(loaded, snapshot);
}

#[semio_framework_async_macros::async_test]
async fn export_media_results_out_returns_json_with_every_case_and_combination() {
    let _app = Fem2dPlayApp;
    let snapshot: Fem2dSnapshot = Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let media = Fem2dPlayApp::export_media("results:out", &doc).expect("results:out exports");
    assert_eq!(media.media_type.class, MediaClass::Data);
    assert_eq!(media.media_type.form, MediaForm::Value);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "computation.fem2d");
            let value = dsl::json::parse(&json).expect("results:out payload is valid JSON");
            for case_id in ["dead", "live", "uls"] {
                let result = value.get(case_id).unwrap_or_else(|| panic!("missing {case_id} in results:out payload: {value}"));
                assert!(result.get("displacements").is_some());
                assert!(result.get("reactions").is_some());
                assert!(result.get("checks").is_some());
            }
        }
        MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
    }
}

#[semio_framework_async_macros::async_test]
async fn export_media_results_out_errors_when_no_load_cases_are_defined() {
    let _app = Fem2dPlayApp;
    let snapshot = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
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
    let snapshot = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    assert!(matches!(Fem2dPlayApp::export_media("bogus:out", &doc), Err(MediaError::NotImplemented)));
}

#[semio_framework_async_macros::async_test]
async fn import_media_geometry_in_builds_a_new_region_from_the_first_material() {
    let _app = Fem2dPlayApp;
    let mut snapshot = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
    snapshot.materials.push(crate::FemMaterial { id: "steel".into(), name: "Steel".into(), e: 2.1e11, nu: 0.3, rho: 7850.0 });
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let payload = dsl::json::to_string(&dsl::json!({ "outline": [[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], "holes": [] }));
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
    let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
    assert_eq!(emit.artifact_mutations.len(), 1);
    match &emit.artifact_mutations[0] {
        Fem2dMutation::CreateRegion(crate::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region }) => {
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
    let snapshot = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let payload = dsl::json::to_string(&dsl::json!({ "outline": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]] }));
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
    let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
    match &emit.artifact_mutations[0] {
        Fem2dMutation::CreateRegion(crate::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region }) => assert_eq!(region.material_id, "unassigned"),
        _ => panic!("expected CreateRegion"),
    }
}
//#endregion 🔖️ExportImportMedia
