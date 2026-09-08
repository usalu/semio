
use super::*;

//#region 🧪️RetainedCommandEnvelope
#[test]
fn retained_command_fixture_matches_exact_routes_and_serde_json_boundaries() {
    use store::ArtifactStoreOneItemPreparationFactory as _;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json")).expect("language-neutral retained fixture");
    let migrated: Vec<&str> = fixture["routes"].as_array().expect("routes").iter().filter(|row| row["disposition"] == "Migrated").map(|row| row["id"].as_str().expect("route id")).collect();
    assert_eq!(migrated, ANIMATE_PRESENTATION_RETAINED_TOOL_IDS);
    assert_eq!(ANIMATE_PRESENTATION_RETAINED_PUBLICATION_CONTRACTS.len(), migrated.len());
    assert_eq!(fixture["limits"]["configValueBytes"].as_u64(), Some(ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES as u64));
    assert_eq!(fixture["limits"]["storeStepBytes"].as_u64(), Some(ANIMATE_PRESENTATION_CONFIG_STEP_BYTES as u64));
    let factory = AnimatePresentationConfigPreparationFactory;
    for case in fixture["boundaryCases"].as_array().expect("boundary cases") {
        let value = "x".repeat(case["bytes"].as_u64().expect("byte count") as usize);
        let mutation = PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value });
        let encoded = serde_json::to_vec(&mutation).expect("third-party JSON encode");
        let decoded: PresentationConfigMutation = serde_json::from_slice(&encoded).expect("third-party JSON decode");
        assert_eq!(decoded, mutation);
        assert_eq!(factory.preflight(&decoded, None, store::HistoryLane::Document).is_ok(), case["accepted"].as_bool().expect("admission oracle"));
    }
}

#[test]
fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
    use std::io::Write as _;
    use store::ArtifactStoreOneItemPreparation as _;
    let value = "x".repeat(ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES);
    let mut preparation = AnimatePresentationConfigPreparation {
        base: None,
        mutation: Some(PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value })),
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
    let mut counter = AnimatePresentationConfigByteCounter { bytes: 0 };
    assert_eq!(counter.write(&[0; 4_096]).expect("maximum serialized envelope"), 4_096);
    assert!(counter.write(&[0]).is_err());
}
//#endregion 🧪️RetainedCommandEnvelope

use crate::editor::animate::testkit::presentation_app;
use protocol::OpText;
use semio_framework_plugin::PluginApp;
use semio_framework_plugin::testkit::meta;

#[semio_framework_async_macros::async_test]
async fn deck_schema_is_animate_presentation() {
    assert_eq!(default_presentation_snapshot().schema, PRESENTATION_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = presentation_app().await;
    app.dispatch_typed(PresentationCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 2 }), &meta("local")).await.expect("seed grid");
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 4);
    app.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.is_empty());
    app.handle_action("redo", None, &meta("local")).await.expect("redo");
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn render_unknown_body_key_reports_it_by_name() {
    use semio_framework_plugin::ViewModel;
    let mut app = presentation_app().await;
    let node = app.render("some.unknown.body", None, &ViewModel::default()).await.expect("render unknown");
    // 🌱️ `BuiltNode` deliberately has no `ToValue`/`FromValue` (framework `🦀️builder.rs`'s own
    // "DslValue-free exception"), so this reads the message back off `Debug` instead of JSON.
    let debug_str = format!("{node:?}");
    assert!(debug_str.contains("Unknown body: some.unknown.body"));
}

#[semio_framework_async_macros::async_test]
async fn app_manifest_declares_expected_operations_and_shell_actions() {
    use semio_framework_plugin::ActionKind;
    let definition = create_animate_presentation_app();
    let operation_ids: Vec<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).filter(|action| matches!(action.kind, ActionKind::Mutation)).map(|action| action.id.as_str()).collect();
    for expected in ["seedGrid", "addTile", "deleteTile", "deleteSelection", "renameTiles", "patchTileCrops", "setSource", "setFrame", "setActiveExample", "clearTiles", "engagementSubmit"] {
        assert!(operation_ids.contains(&expected), "missing declared operation {expected}");
    }
    assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "exportVideoFromDeck" && matches!(action.kind, ActionKind::Shell)));
    assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "engagementInput" && matches!(action.kind, ActionKind::View)));
}

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    // 🌱️ `AppDefinition` is documented framework-side (ticket 26/09/01/RUNTIME-DEPENDENCY-
    // ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS) as still serde-only, blocked on
    // `WindowKindDefinition`/`UtilityDefinition` converting first — outside this plugin's write
    // scope, so this reads the manifest back off `Debug` instead of JSON.
    let json = format!("{:?}", create_animate_presentation_app());
    assert!(json.contains(tile_editor::PRESENTATION_PLAY_WINDOW_MAIN), "window kind missing from the manifest: {json}");
    assert!(json.contains(main::PRESENTATION_PLAY_MODE_MAIN), "mode missing from the manifest");
    for body in [PRESENTATION_PLAY_BODY_DOCUMENT, PRESENTATION_PLAY_BODY_CATALOGUE, PRESENTATION_PLAY_BODY_DETAILS] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains(PRESENTATION_DOCUMENT_SCHEMA), "artifact kind missing from the manifest");
}

/// 🕹️ The `tiles` domain is declared `HierarchyProvider::Flat`, non-transitive, broadcast, and
/// bound to the tile-editor window (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
#[semio_framework_async_macros::async_test]
async fn the_manifest_declares_the_tiles_interaction_domain() {
    let definition = create_animate_presentation_app();
    let domain = definition.interactions.iter().find(|interaction| interaction.id == PRESENTATION_INTERACTION_DOMAIN).expect("tiles interaction domain declared");
    assert!(matches!(domain.hierarchy, HierarchyProvider::Flat));
    assert!(domain.selection.broadcast);
    assert!(!domain.selection.transitive);
    let canvas_window = definition.window_kinds.iter().find(|window| window.id == tile_editor::PRESENTATION_PLAY_WINDOW_MAIN).expect("tile-editor window declared");
    assert!(canvas_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == PRESENTATION_INTERACTION_DOMAIN));
}
//#endregion 🔖️ManifestSanity

/// 🧬️ Two independent instances start empty, apply DISJOINT edits (A adds a tile, B sets the
/// source), and exchanging operations over a `MemoryBackbone` converges both sides to contain BOTH
/// edits — impossible with whole-document snapshots, which would clobber one another.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    use store::MemoryBackbone;
    let mut instance_a = presentation_app().await;
    let mut instance_b = presentation_app().await;
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://animate-presentation-convergence", "mem://animate-presentation-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    instance_a.dispatch_typed(PresentationCommand::AddTile(add_tile::AddTile { crop: Some(crate::FigureTileFrame { x: 0.0, y: 0.0, width: 0.3, height: 0.3 }) }), &meta("actor-a")).await.expect("a adds tile");
    let (mut source, _) = crate::presentation_working_scene(&instance_b.snapshot().expect("projection"));
    source.kind = "video".into();
    instance_b.dispatch_typed(PresentationCommand::SetSource(set_source::SetSource { source }), &meta("actor-b")).await.expect("b sets source kind");

    instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");

    let (source_a, tiles_a) = crate::presentation_working_scene(&instance_a.snapshot().expect("projection"));
    let (source_b, tiles_b) = crate::presentation_working_scene(&instance_b.snapshot().expect("projection"));
    assert_eq!(tiles_a.len(), 1, "instance A keeps its own tile");
    assert_eq!(tiles_b.len(), 1, "instance B converges on A's tile");
    assert_eq!(source_a.kind, "video", "instance A converges on B's source edit");
    assert_eq!(source_b.kind, "video", "instance B keeps its own source edit");
}

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn presentation_io_declares_frames_in_and_document_ports() {
    let ports = AnimatePresentationPlayApp::io().expect("io").all_ports().await;
    assert!(ports.iter().any(|port| port.id == "document:in"));
    assert!(ports.iter().any(|port| port.id == "document:out"));
    assert!(ports.iter().any(|port| port.id == "frames:in"));
}

#[semio_framework_async_macros::async_test]
async fn import_media_frames_in_inserts_a_new_tile() {
    use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
    let mut app = testkit::presentation_app_with_registry().await;
    let before = crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len();
    let frame_json = dsl::os_pack::json::to_string(&dsl::os_pack::json::object([("name".to_string(), dsl::os_pack::json::Value::from("hero-frame")), ("src".to_string(), dsl::os_pack::json::Value::from("/frames/hero.png"))]));
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: frame_json } };
    app.import_media("frames:in", media, &meta("local")).await.expect("import frames:in");
    let (_, after_tiles) = crate::presentation_working_scene(&app.snapshot().expect("projection"));
    assert_eq!(after_tiles.len(), before + 1);
    assert_eq!(after_tiles.last().expect("imported tile").name, "hero-frame");
}

#[semio_framework_async_macros::async_test]
async fn import_media_frames_in_places_repeated_imports_in_distinct_cells() {
    use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
    let mut app = testkit::presentation_app_with_registry().await;
    for _ in 0..2 {
        let frame_json = dsl::os_pack::json::to_string(&dsl::os_pack::json::object([("name".to_string(), dsl::os_pack::json::Value::from("frame"))]));
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: frame_json } };
        app.import_media("frames:in", media, &meta("local")).await.expect("import frames:in");
    }
    let (_, tiles) = crate::presentation_working_scene(&app.snapshot().expect("projection"));
    assert_eq!(tiles.len(), 2);
    assert_ne!(tiles[0].crop, tiles[1].crop, "repeated imports land in distinct cells");
}

#[semio_framework_async_macros::async_test]
async fn import_media_rejects_unknown_port() {
    use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
    let mut app = testkit::presentation_app_with_registry().await;
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: "{}".into() } };
    assert!(app.import_media("not-a-port", media, &meta("local")).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn empty_presentation_snapshot_has_no_tiles() {
    let snapshot = crate::standards::v1::subsets::any::schema::empty_presentation_snapshot();
    assert!(crate::presentation_working_scene(&snapshot).1.is_empty());
}

/// 🌱️ Relocated from the former artifact-tree `⚙️engine`'s own tests (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `presentation_io`'s relocation to
/// this file's `🔖️Io` region.
#[semio_framework_async_macros::async_test]
async fn presentation_io_declares_the_frames_in_port() {
    let io = presentation_io();
    assert_eq!(io.document_schema, PRESENTATION_DOCUMENT_SCHEMA);
    assert_eq!(io.ports.len(), 1);
    let port = &io.ports[0];
    assert_eq!(port.id, "frames:in");
    assert_eq!(port.kind_id.as_deref(), Some("2d.image"));
    assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::In);
    assert_eq!(port.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    assert!(!port.required);
}

#[semio_framework_async_macros::async_test]
async fn frame_import_placement_is_deterministic_and_non_overlapping() {
    let first = next_frame_tile_crop(0);
    let second = next_frame_tile_crop(1);
    assert_ne!(first, second);
    assert_eq!(next_frame_tile_id(0), "frame-1");
    assert_eq!(next_frame_tile_id(1), "frame-2");
    // 🧮️ Pure function of the count, not a mutating counter.
    assert_eq!(next_frame_tile_crop(0), first);
}
//#endregion 🔖️PortTests

//#region 🔖️CommandSurfaceTests
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
/// hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 18, "every PresentationCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword. This is what
/// a missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
/// keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let expected_keywords: [(&str, &str); 17] = [
        ("seedGrid", "seed-grid"),
        ("addTile", "add-tile"),
        ("deleteTile", "delete-tile"),
        ("deleteSelection", "delete-selection"),
        ("renameTiles", "rename-tiles"),
        ("patchTileCrops", "patch-tile-crops"),
        ("setSource", "set-source"),
        ("setFrame", "set-frame"),
        ("setActiveExample", "set-active-example"),
        ("clearTiles", "clear-tiles"),
        ("engagementSubmit", "engagement-submit"),
        ("resetGrid", "reset-grid"),
        ("engagementInput", "engagement-input"),
        ("canvasPointerDown", "canvas-pointer-down"),
        ("noMutation", "no-op"),
        ("copyPrompt", "copy-prompt"),
        ("exportVideoFromDeck", "export-video-from-deck"),
    ];
    for (command, (id, keyword)) in every_command().into_iter().zip(expected_keywords) {
        assert_eq!(command.command_id(), id);
        let printed = OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), keyword, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact
/// bytes captured from the pre-merge `presentation_protocol` crate. A regression here is a real format
/// break, not a test-fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let with_crop = PresentationCommand::AddTile(add_tile::AddTile { crop: Some(crate::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 }) });
    let without_crop = PresentationCommand::AddTile(add_tile::AddTile { crop: None });
    assert!(OpText::print_op(&with_crop).starts_with("add-tile"));
    assert!(OpText::print_op(&without_crop).starts_with("add-tile"));
    store::os_store::test_support::assert_op_text_binary_equivalence(&with_crop);
    store::os_store::test_support::assert_op_text_binary_equivalence(&without_crop);

    let with_layer = PresentationCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: Some("t1".into()) });
    let without_layer = PresentationCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: None });
    store::os_store::test_support::assert_op_text_binary_equivalence(&with_layer);
    store::os_store::test_support::assert_op_text_binary_equivalence(&without_layer);
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<PresentationCommand> {
    vec![
        PresentationCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 3 }),
        PresentationCommand::AddTile(add_tile::AddTile { crop: Some(crate::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 }) }),
        PresentationCommand::DeleteTile(delete_tile::DeleteTile { id: "t1".into() }),
        PresentationCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        PresentationCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec!["t1".into(), "t2".into()], value: "Hero".into() }),
        PresentationCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: vec!["t1".into()], field: "width".into(), value: 0.4 }),
        PresentationCommand::SetSource(set_source::SetSource { source: crate::default_figure_tile_source() }),
        PresentationCommand::SetFrame(set_frame::SetFrame { frame: crate::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }),
        PresentationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }),
        PresentationCommand::ClearTiles(clear_tiles::ClearTiles {}),
        PresentationCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "2x2".into() }),
        PresentationCommand::ResetGrid(reset_grid::ResetGrid {}),
        PresentationCommand::EngagementInput(engagement_input::EngagementInput { value: "add".into() }),
        PresentationCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: Some("t1".into()) }),
        PresentationCommand::NoOperation(no_operation::NoOperation {}),
        PresentationCommand::CopyPrompt(copy_prompt::CopyPrompt {}),
        PresentationCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck { output_dir: "output/x".into(), scene_json: "{}".into() }),
    ]
}
//#endregion 🔖️CommandSurfaceTests
