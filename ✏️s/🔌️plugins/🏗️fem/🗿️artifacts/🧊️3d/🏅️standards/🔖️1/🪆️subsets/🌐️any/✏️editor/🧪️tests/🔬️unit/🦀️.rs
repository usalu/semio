
use super::*;

//#region 🧪️RetainedCommandEnvelope
#[test]
fn retained_command_fixture_matches_exact_routes_and_value_codec_boundaries() {
    use store::ArtifactStoreOneItemPreparationFactory as _;
    let fixture: dsl::DslValue = dsl::json::from_json_str(include_str!("../../🧫️fixtures/🚧️retained-command-limits/🔣️.json")).expect("language-neutral retained fixture");
    let migrated: Vec<&str> = fixture["routes"].as_array().expect("routes").iter().filter(|row| row["disposition"].as_str() == Some("Migrated")).map(|row| row["id"].as_str().expect("route id")).collect();
    assert_eq!(migrated, FEM3D_RETAINED_TOOL_IDS);
    assert_eq!(FEM3D_RETAINED_PUBLICATION_CONTRACTS.len(), migrated.len());
    for (row, contract) in fixture["routes"].as_array().expect("routes").iter().zip(FEM3D_RETAINED_PUBLICATION_CONTRACTS) {
        assert_eq!(row["id"].as_str(), Some(contract.tool_id));
        let declared: Vec<&str> = row["lanes"].as_array().expect("lanes").iter().map(|lane| lane.as_str().expect("lane id")).collect();
        let actual: Vec<&str> = contract
            .lanes
            .iter()
            .map(|lane| match lane {
                ArtifactToolPublicationLane::Artifact => "Artifact",
                ArtifactToolPublicationLane::Config => "Config",
                ArtifactToolPublicationLane::Draft => "Draft",
                ArtifactToolPublicationLane::Presence => "Presence",
                ArtifactToolPublicationLane::Transient => "Transient",
                ArtifactToolPublicationLane::Child => "Child",
                ArtifactToolPublicationLane::HostOnly => "HostOnly",
            })
            .collect();
        assert_eq!(declared, actual, "publication lanes drifted for {}", contract.tool_id);
    }
    assert_eq!(fixture["limits"]["rawBytes"].as_u64(), Some(FEM3D_RETAINED_RAW_BYTES as u64));
    assert_eq!(fixture["limits"]["commandStepBytes"].as_u64(), Some(FEM3D_RETAINED_OUTPUT_BYTES as u64));
    assert_eq!(fixture["limits"]["workItems"].as_u64(), Some(FEM3D_RETAINED_WORK_ITEMS as u64));
    assert_eq!(fixture["limits"]["decodedItems"].as_u64(), Some(FEM3D_RETAINED_DECODED_ITEMS as u64));
    assert_eq!(fixture["limits"]["artifactStoreBytes"].as_u64(), Some(FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES as u64));
    assert_eq!(fixture["limits"]["configValueBytes"].as_u64(), Some(FEM3D_CONFIG_VALUE_BYTES as u64));
    assert_eq!(fixture["limits"]["storeStepBytes"].as_u64(), Some(FEM3D_CONFIG_STEP_BYTES as u64));
    let factory = Fem3dConfigPreparationFactory;
    for case in fixture["boundaryCases"].as_array().expect("boundary cases") {
        let value = "x".repeat(case["bytes"].as_u64().expect("byte count") as usize);
        let mutation = Fem3dConfigMutation::SetCamera { camera: crate::FemCamera { json: value } };
        let encoded = dsl::json::to_json_string(&mutation);
        let decoded: Fem3dConfigMutation = dsl::json::from_json_str(&encoded).expect("first-party JSON decode");
        assert_eq!(decoded, mutation);
        assert_eq!(factory.preflight(&decoded, None, store::HistoryLane::Document).is_ok(), case["accepted"].as_bool().expect("admission oracle"));
    }
}

#[test]
fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
    use std::io::Write as _;
    use store::ArtifactStoreOneItemPreparation as _;
    let value = "x".repeat(FEM3D_CONFIG_VALUE_BYTES);
    let mut preparation = Fem3dConfigPreparation {
        base: None,
        mutation: Some(Fem3dConfigMutation::SetCamera { camera: crate::FemCamera { json: value } }),
        description: None,
        authority: None,
        candidate: None,
        sealed_candidate: None,
        serialized_bytes: None,
        prepared: None,
        checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
        cancelled: false,
        closing: false,
    };
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 };
    preparation.cancel();
    assert!(matches!(preparation.advance(grant).expect("cancelled step"), store::ArtifactStoreOneItemPreparationStep::Blocked));
    preparation.begin_close();
    assert!(matches!(preparation.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).expect("undersized close"), store::SnapshotRetirementStep::Blocked));
    assert!(matches!(preparation.close_step(grant).expect("bounded close"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 4_096 }));
    assert!(matches!(preparation.close_step(grant).expect("terminal close"), store::SnapshotRetirementStep::Complete));
    assert!(preparation.terminal_is_empty());
    let mut counter = Fem3dConfigByteCounter { bytes: 0 };
    assert_eq!(counter.write(&[0; 4_096]).expect("maximum serialized envelope"), 4_096);
    assert!(counter.write(&[0]).is_err());
}

/// ⚖️ LAW: every one of the 18 declared actions is owned by `Fem3dRetainedCommandJobFactory`, is
/// classified `Migrated` in the manifest, and declares a nonempty publication lane contract.
/// `AppActionRegistry::tool_job_registration` enforces the same set equality at app construction
/// (`interactive-job.catalog-incomplete`), and `validate_ui_dispatch_classification` rejects anything
/// not `Migrated` at the very first gate of `handle_action` — this pins both, so a future row that
/// forgets its retained-tool-id, its classification or its lane contract fails here instead of going
/// silently dispatch-dead the way 16 of these 18 rows were.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::ToolExecutionShape;
    assert_eq!(FEM3D_RETAINED_TOOL_IDS.len(), 18);
    assert_eq!(<Fem3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 18);
    assert_eq!(FEM3D_RETAINED_PUBLICATION_CONTRACTS.len(), 18);
    // 🧷️ `validate_tool_job_rows` compares `registration.contract == row.contract` byte for byte, so
    // the proof rows and the registered factory MUST build from the one `fem3d_retained_contract()`.
    assert_eq!(fem3d_retained_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(ToolJobFactory::execution_contract(&Fem3dRetainedCommandJobFactory::new("s.fem.fem3d@1/*#editor")), fem3d_retained_contract());
    let mut sorted_ids = FEM3D_RETAINED_TOOL_IDS.to_vec();
    sorted_ids.sort_unstable();
    sorted_ids.dedup();
    assert_eq!(sorted_ids.len(), FEM3D_RETAINED_TOOL_IDS.len(), "duplicate retained tool ids in {FEM3D_RETAINED_TOOL_IDS:?}");
    assert_eq!(Fem3dRetainedCommandJobFactory::TOOL_IDS, FEM3D_RETAINED_TOOL_IDS);
    for command in every_command() {
        let tool_id = command.command_id();
        assert!(FEM3D_RETAINED_TOOL_IDS.contains(&tool_id), "command {tool_id} is not owned by Fem3dRetainedCommandJobFactory");
        let contract = FEM3D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == tool_id).unwrap_or_else(|| panic!("tool {tool_id} declares a publication contract"));
        assert!(!contract.lanes.is_empty(), "tool {tool_id} declares a nonempty publication lane set");
    }
    let definition = create_fem3d_app();
    let model_window = definition.window_kinds.iter().find(|window| window.id == window_model::FEM3D_WINDOW_MODEL).expect("model window declared");
    for tool_id in FEM3D_RETAINED_TOOL_IDS {
        let action = model_window.actions.iter().find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} is declared by the manifest"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "action {tool_id} must be UI-dispatchable");
    }
}

/// ⚖️ LAW: both lanes a fem3d tool can publish into have a real one-item preparation factory — an
/// Artifact-lane tool without `build_artifact_store_one_item_preparation_factory` is rejected at
/// dispatch with `interactive-job.publication-authority-missing`, which is exactly what would have
/// happened to all 15 document tools without `Fem3dArtifactPreparationFactory`.
#[semio_framework_async_macros::async_test]
async fn both_declared_publication_lanes_have_a_preparation_factory() {
    assert!(<Fem3dPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some());
    assert!(<Fem3dPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some());
}

/// ⚖️ LAW: every document mutation the 15 Artifact-lane tools can emit fits the Artifact lane's
/// one-item envelope, measured on the boot document (the largest fixture this app ships).
#[semio_framework_async_macros::async_test]
async fn every_boot_document_mutation_is_admissible_on_the_artifact_lane() {
    use store::ArtifactStoreOneItemPreparationFactory as _;
    let boot = crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot();
    let factory = Fem3dArtifactPreparationFactory;
    for solid in &boot.solids {
        let mutation = Fem3dMutation::CreateSolid(crate::standards::v1::subsets::any::schema::mutations::create_solid::CreateSolid { solid: solid.clone() });
        assert!(factory.preflight(&mutation, None, store::HistoryLane::Document).is_ok(), "solid {} exceeds the artifact one-item envelope", solid.id);
    }
    for node in &boot.nodes {
        let mutation = Fem3dMutation::CreateNode(crate::standards::v1::subsets::any::schema::mutations::create_node::CreateNode { node: node.clone() });
        assert!(factory.preflight(&mutation, None, store::HistoryLane::Document).is_ok(), "node {} exceeds the artifact one-item envelope", node.id);
    }
    assert!(factory.preflight(&Fem3dMutation::CreateNode(crate::standards::v1::subsets::any::schema::mutations::create_node::CreateNode { node: crate::FemNode { id: "n0".into(), x: 0.0, y: 0.0, z: 0.0 } }), None, store::HistoryLane::Interaction).is_err());
}

/// 🚀️ LAW: the editor boots with real geometry, so the `World3d` Model window has something to mesh
/// on first paint instead of the pre-fix empty document.
#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_the_bundled_example_not_empty() {
    let snapshot = <Fem3dPlayApp as ArtifactEditor>::initial_snapshot();
    assert!(!snapshot.nodes.is_empty(), "expected the bundled default example's nodes");
    assert!(!snapshot.solids.is_empty(), "expected the bundled default example's solids");
}
//#endregion 🧪️RetainedCommandEnvelope

use crate::editor::fem3d::testkit::{Fem3dApp, dispatch, fem3d_app};
use semio_framework_plugin::testkit::assert_undo_redo_round_trip;

//#region 🔖️CommandSurface
/// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the exact
/// fixture values the pre-migration `fem3d_protocol` crate's own `Fem3dCommand` test used.
fn every_command() -> Vec<Fem3dCommand> {
    vec![
        Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }),
        Fem3dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
        Fem3dCommand::AddFrame(add_frame::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 }),
        Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.077e10 }),
        Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 }),
        Fem3dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: crate::FemDof::ALL.to_vec() }),
        Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::FemDof::Tz, value: -5000.0, case_id: Some("live".into()) }),
        Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -500.0, case_id: None }),
        Fem3dCommand::AddAreaLoad(add_area_load::AddAreaLoad { solid_id: "sol1".into(), pressure: 5000.0, case_id: Some("dead".into()) }),
        Fem3dCommand::AddSolid(add_solid::AddSolid { x: 0.0, y: 0.0, width: 4.0, depth: 2.0, height: 0.5, material_id: "concrete".into(), base_z: Some(0.0), layers: Some(2), mesh_size: None }),
        Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }),
        Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "[[\"dead\",1.35],[\"live\",1.5]]".into() }),
        Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true }),
        Fem3dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) }),
        Fem3dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] }),
        Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() }),
        Fem3dCommand::SetCamera(set_camera::SetCamera { json: "{\"x\":1}".into() }),
        Fem3dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }),
    ]
}

/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 18, "every Fem3dCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 📌️ LAW: the pre-migration command wire format, row for row — the hex list is positionally aligned
/// to `every_command()`, which carries exactly the values the old `📡️protocol` crate's baseline dump
/// used (ticket `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
/// `🧪️wire-baseline-before-3d.txt`). Row order is the binary variant ordinal, so a reordering — which
/// no round-trip law can catch — shows up here as a leading-byte mismatch. `addNodalLoad`'s `None`
/// case is pinned separately below because `every_command()` only carries its `Some` shape.
#[semio_framework_async_macros::async_test]
async fn every_command_keeps_its_pre_migration_bytes() {
    use protocol::OpBinary;
    let expected = [
        "010000030005000000000000f03f0105000000000000004002050000000000000840",
        "010104026e31026e3203726f6405737465656c04000600010601020603030602",
        "01020406686561323030026e31026e3205737465656c050006010106020206030306000405000000000000e03f",
        "01030105537465656c030006000105000000da7c72484202050000806444ce3242",
        "0104010648454132303005000600010545f5d6c05609763f020554fc8458a258033f0305210ec81462e4eb3e040576830df4f521a43e",
        "010501026e310200060001160600020406080a",
        "010602046c697665026e3104000601010a020205000000000088b3c0030600",
        "01070102653104000600010500000000000000000205000000000000000003050000000000407fc0",
        "010802046465616404736f6c31030006010105000000000088b340020600",
        "01090108636f6e637265746508000500000000000000000105000000000000000002050000000000001040030500000000000000400405000000000000e03f05060006050000000000000000070402",
        "010a01044c697665020006000101",
        "010b0203554c531c5b5b2264656164222c312e33355d2c5b226c697665222c312e355d5d02000600010601",
        "010c010464656164020006000102",
        "010d000200040502050000000000003e40",
        "010e02026531026e3101000c0206010600",
        "010f010764656661756c7401000600",
        "011001077b2278223a317d01000600",
        "0111020464656164056d6f64616c03000600010601020400",
    ];
    let commands = every_command();
    assert_eq!(commands.len(), expected.len(), "the baseline hex list must cover every command row");
    for (command, expected) in commands.iter().zip(expected) {
        let bytes = command.encode_op().expect("encode");
        assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected, "wire bytes changed for {}", command.command_id());
    }
    let nodal_load_without_case = Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::FemDof::Tz, value: -5000.0, case_id: None });
    assert_eq!(nodal_load_without_case.encode_op().expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>(), "010601026e3103000600010a020205000000000088b3c0");
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword. Three rows
/// (`setActiveExample`/`setCamera`/`setResultDisplay`) prove the wire keyword is NOT simply the
/// kebab-cased command id — this is exactly what a missing `#[dsl(keyword = ..)]` on a payload struct
/// silently breaks (the record prints with no keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let expected_keys = [
        "add-node",
        "add-bar",
        "add-frame",
        "add-material",
        "add-section",
        "add-support",
        "add-nodal-load",
        "add-member-udl",
        "add-area-load",
        "add-solid",
        "add-load-case",
        "add-combination",
        "set-self-weight",
        "set-analysis-settings",
        "remove-selection",
        "active-example",
        "camera",
        "result-display",
    ];
    for (command, expected) in every_command().into_iter().zip(expected_keys) {
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {command:?}: {printed:?}");
    }
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = dsl::json::to_json_string(&create_fem3d_app());
    for id in [window_model::FEM3D_WINDOW_MODEL, window_results::FEM3D_WINDOW_RESULTS] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::MODE_ID), "mode {} missing from the manifest", edit::MODE_ID);
    assert!(json.contains("computation.fem3d"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn manifest_labels_resolve_german_3d() {
    use semio_framework_plugin::{Locale, Terminology};
    let definition = create_fem3d_app();
    let window = definition.window_kinds.iter().find(|w| w.id == window_model::FEM3D_WINDOW_MODEL).expect("model window declared");
    assert_eq!(window.label.resolve(Terminology::Native, Locale::De), "Modell");
    let action = window.actions.iter().find(|action| action.id == "addFrame").expect("addFrame declared");
    assert_eq!(action.label.resolve(Terminology::Native, Locale::De), "Rahmen hinzufügen");
    assert_eq!(action.label.resolve(Terminology::Native, Locale::En), "Add Frame");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn undo_restores_document_after_add_node() {
    let mut app = fem3d_app();
    let before = app.snapshot().expect("snapshot").nodes.len();
    assert_undo_redo_round_trip(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1).await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::fem3d::testkit::render;
    let mut app = fem3d_app();
    assert!(render(&mut app, "fem3d.play.nope").contains("Unknown body"));
}
//#endregion 🔖️CrossCutting

//#region 🔖️MediaPorts
/// 🎞️ `"results:out"` runs every load case fresh and returns a `Structured` JSON payload — build a
/// doc with the bundled example (which has load cases), export, assert the JSON round-trips through
/// the first-party value codec and names a case id.
#[semio_framework_async_macros::async_test]
async fn export_media_results_out_returns_solved_json_for_every_case_3d() {
    let mut app: Fem3dApp = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let media = Fem3dPlayApp::export_media("results:out", &doc).expect("results:out exports");
    assert_eq!(media.media_type.class, MediaClass::Data);
    assert_eq!(media.media_type.form, MediaForm::Value);
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a Structured payload") };
    assert_eq!(schema, "computation.fem3d");
    let value = dsl::json::parse(&json).expect("results:out payload is valid JSON");
    assert!(value.get("dead").is_some(), "expected the example fixture's dead case in the results map: {json}");
    assert!(value["dead"].get("displacements").is_some(), "expected a displacements array: {json}");
}

/// 🎞️ `"results:out"` on a document with no load cases errors rather than panicking or returning an
/// empty payload.
#[semio_framework_async_macros::async_test]
async fn export_media_results_out_errors_without_load_cases_3d() {
    let snapshot = crate::standards::v1::subsets::any::schema::empty_fem3d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let err = Fem3dPlayApp::export_media("results:out", &doc).expect_err("no load cases should error");
    assert!(matches!(err, MediaError::Payload(..)));
}

/// 🎞️ `"geometry:in"` decodes an extruded-footprint JSON contract into a new `FemSolid` operation.
#[semio_framework_async_macros::async_test]
async fn import_media_geometry_in_adds_a_new_solid_3d() {
    let mut app: Fem3dApp = testkit::fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Concrete".into(), e: 30e9, g: 12.5e9 })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let json = dsl::json!({
        "outline": [[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]],
        "holes": [],
        "baseZ": 0.5,
        "height": 3.0,
        "layers": 2,
    })
    .to_string();
    let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any }, payload: MediaPayload::Structured { schema: "geometry".into(), json } };
    let emit = Fem3dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
    assert_eq!(emit.artifact_mutations.len(), 1);
    match &emit.artifact_mutations[0] {
        Fem3dMutation::CreateSolid(crate::standards::v1::subsets::any::schema::mutations::create_solid::CreateSolid { solid }) => {
            assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
            assert_eq!(solid.base_z, 0.5);
            assert_eq!(solid.height, 3.0);
            assert_eq!(solid.layers, 2);
            assert_eq!(solid.material_id, "m0");
        }
        _ => panic!("expected CreateSolid"),
    }
}

#[semio_framework_async_macros::async_test]
async fn fem3d_io_matches_declared_artifact_identity_3d() {
    let io = Fem3dPlayApp::io().expect("fem3d declares typed media I/O");
    assert_eq!(io.artifact.id, "3d.fem");
    assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
    assert!(io.ports.iter().any(|port| port.id == "results:out"));
}

/// 🔌️ Wave-1's `required: true` unwired-input enforcement (`validate_edge_kinds`) lives in the run
/// crate, not here — this test only proves the port DECLARATION is correct; the cross-crate
/// enforcement is exercised at the run-crate level.
#[semio_framework_async_macros::async_test]
async fn fem3d_io_declares_geometry_in_and_results_out_ports() {
    let io = fem3d_io();
    assert_eq!(io.document_schema, crate::FEM_3D_SCHEMA);
    assert_eq!(io.document_media_type.class, MediaClass::ThreeD);
    assert_eq!(io.document_media_type.form, MediaForm::Any);
    assert_eq!(io.artifact.id, "3d.fem");
    assert_eq!(io.artifact.component_kind, "fem3d");

    let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
    assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
    assert!(geometry_in.required, "geometry:in is a required input port");
    assert_eq!(geometry_in.media_type.class, MediaClass::ThreeD);
    assert_eq!(geometry_in.media_type.form, MediaForm::Any);
    assert_eq!(geometry_in.multiplicity, semio_framework::PortMultiplicity::One);

    let results_out = io.ports.iter().find(|port| port.id == "results:out").expect("results:out declared");
    assert_eq!(results_out.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert!(!results_out.required, "results:out is optional");
    assert_eq!(results_out.kind_id.as_deref(), Some("computation.fem3d"));
    assert_eq!(results_out.media_type.class, MediaClass::Data);
    assert_eq!(results_out.media_type.form, MediaForm::Value);
}
//#endregion 🔖️MediaPorts

//#region 🎬️SceneRender
#[semio_framework_async_macros::async_test]
async fn quat_z_to_identity_for_parallel_direction() {
    assert_eq!(quat_z_to([0.0, 0.0, 1.0]), [0.0, 0.0, 0.0, 1.0]);
}

#[semio_framework_async_macros::async_test]
async fn quat_z_to_handles_antiparallel_direction() {
    assert_eq!(quat_z_to([0.0, 0.0, -1.0]), [1.0, 0.0, 0.0, 0.0]);
}

#[semio_framework_async_macros::async_test]
async fn fem3d_camera_json_falls_back_to_world3d_default_for_empty_object() {
    let camera = crate::FemCamera::default();
    assert_eq!(fem3d_camera_json(&camera), semio_framework_plugin::world3d_default_camera());
    let custom = crate::FemCamera { json: "{\"x\":1}".into() };
    assert_eq!(fem3d_camera_json(&custom), "{\"x\":1}");
}

#[semio_framework_async_macros::async_test]
async fn fem3d_scene_parts_include_solid_mesh_and_oriented_member_instances() {
    let doc: Fem3dSnapshot = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT).expect("example fixture parses");
    let (meshes_json, instances_json) = fem3d_scene_parts(&doc, None, doc.analysis.deformation_scale, None);
    assert!(meshes_json.contains("solid-sol1"), "expected a solid- mesh id for the example fixture's solid: {meshes_json}");
    assert!(instances_json.contains("el-e1"), "expected a single oriented box instance per member (no -{{i}} sphere chain): {instances_json}");
}
//#endregion 🎬️SceneRender
