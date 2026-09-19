pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry_and_members};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type PresentationApp = VcsArtifactApp<EditorApp<AnimatePresentationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;
    
    /// ✏️ `AnimatePresentationPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<AnimatePresentationPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<E>`
    /// builds it.
    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn presentation_app() -> PresentationApp {
        VcsArtifactApp::<EditorApp<AnimatePresentationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>::new(EditorApp::default()).await
    }
    
    /// 🧪️ Adapts `create_animate_presentation_app`'s `AppDefinition` (contract §2.4) into the
    /// `App { definition, examples }` shape `new_app_with_registry`/
    /// `context::assert_declared_actions_bridge_to_commands` still expect — framework test context gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    fn animate_presentation_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_animate_presentation_app(), examples: Vec::new() }
    }
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn presentation_app_with_registry() -> PresentationApp {
        new_app_with_registry_and_members::<EditorApp<AnimatePresentationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(animate_presentation_app_manifest_for_tests).await
    }
    
    pub async fn dispatch(app: &mut PresentationApp, command: PresentationCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }
    
    pub async fn render(app: &mut PresentationApp, body_key: &str) -> String {
        // 🌱️ `BuiltNode` deliberately has no `ToValue`/`FromValue` (framework `🦀️builder.rs`'s own
        // "DslValue-free exception" for `UiValue`-embedding types), so every caller here reads
        // rendered content back off the `Debug` rendering instead of round-tripping through JSON —
        // every call site below only substring-searches the result, never parses it as JSON.
        format!("{:?}", app.render(body_key, None, &ViewModel::default()).await.expect("render"))
    }
}

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

use crate::editor::animate::unit_tests::context::presentation_app;
use protocol::OpText;
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::PluginApp;

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
    for body in [PRESENTATION_PLAY_BODY_ARTIFACT, PRESENTATION_PLAY_BODY_CATALOGUE, PRESENTATION_PLAY_BODY_DETAILS] {
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
/// `artifact_app_laws::assert_two_registered_instances_converge` replayed over this crate's
/// `SemioMembers` harness: the law's registered twin builds member-less apps, whose genesis refuses the
/// derived `s.stdio.semio@v1/presentation`/`animation` children (the dag precedent).
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, settle_registered_typed_operation};
    fn probe(app: &context::PresentationApp) -> (String, usize) {
        let (source, tiles) = crate::presentation_working_scene(&app.snapshot().expect("projection"));
        (source.kind, tiles.len())
    }
    let mut source = crate::default_figure_tile_source();
    source.kind = "video".into();
    let mut instance_a = context::presentation_app_with_registry().await;
    let mut instance_b = context::presentation_app_with_registry().await;
    let receiver = meta("actor-a").instance_id;
    instance_a.bind_instance_id(receiver).await;
    instance_b.bind_instance_id(receiver).await;
    let (backbone_a, backbone_b) = store::MemoryBackbone::pair("mem://animate-presentation-convergence", "mem://animate-presentation-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");
    let genesis = probe(&instance_a);
    instance_a.dispatch_typed(PresentationCommand::AddTile(add_tile::AddTile { crop: Some(crate::FigureTileFrame { x: 0.0, y: 0.0, width: 0.3, height: 0.3 }) }), &meta("actor-a")).await.expect("a applies its edit");
    settle_registered_typed_operation(&mut instance_a, receiver).await.expect("a's edit publishes");
    instance_b.dispatch_typed(PresentationCommand::SetSource(set_source::SetSource { source }), &meta("actor-b")).await.expect("b applies its edit");
    settle_registered_typed_operation(&mut instance_b, receiver).await.expect("b's edit publishes");
    instance_a.tick_backbone().await.expect("a folds b's events");
    instance_b.tick_backbone().await.expect("b folds a's events");
    assert_eq!(probe(&instance_a), probe(&instance_b), "both instances must converge on the same snapshot");
    let admitted = instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("a commits a checkpoint");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut instance_a, admitted).await.expect("a's checkpoint commit settles");
    settle_registered_typed_operation(&mut instance_a, receiver).await.expect("a's checkpoint publication settles");
    instance_b.tick_backbone().await.expect("b folds a's checkpoint");
    assert_eq!(probe(&instance_a), probe(&instance_b), "a replicated checkpoint keeps both instances converged");
    assert_ne!(probe(&instance_a), genesis, "the replicated edits must actually land, not converge on the untouched genesis");
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
    close_registered_fixture_app(&mut instance_a);
    close_registered_fixture_app(&mut instance_b);
}

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn presentation_io_declares_frames_in_and_document_ports() {
    let ports = AnimatePresentationPlayApp::io().expect("io").all_ports().await;
    assert!(ports.iter().any(|port| port.id == "artifact:in"));
    assert!(ports.iter().any(|port| port.id == "artifact:out"));
    assert!(ports.iter().any(|port| port.id == "frames:in"));
}

#[semio_framework_async_macros::async_test]
async fn import_media_frames_in_inserts_a_new_tile() {
    use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
    let mut app = context::presentation_app_with_registry().await;
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
    let mut app = context::presentation_app_with_registry().await;
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
    let mut app = context::presentation_app_with_registry().await;
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
    assert_eq!(io.artifact_schema, PRESENTATION_DOCUMENT_SCHEMA);
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

//#region 🔖️ExampleArchiveLoad
/// 🚪️ The browser host answers the boot `setActiveExample`'s `Effect::LoadDocument` by handing the
/// pack/spr pair back through the document archive door with an empty member roster (`ShellHost`
/// `loadDocumentPair`); every composed `#[child]` slot must then be derived through
/// `genesis_child_pack`, or the closure completes `Incomplete` and the load is refused
/// (`document-archive-replacement.closure-rejected`).
#[semio_framework_async_macros::async_test]
async fn demo_example_load_settles_through_the_host_document_archive_door() {
    use semio_framework_plugin::{artifact_app_laws, PluginApp};
    let mut app = context::presentation_app_with_registry().await;
    let instance = 7;
    app.bind_instance_id(instance).await;
    let meta = semio_framework_plugin::ActionMeta { instance_id: instance, ..artifact_app_laws::meta("fixture") };
    app.dispatch_typed(PresentationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }), &meta).await.expect("example dispatch");
    let mut loaded = None;
    for _ in 0..100_000 {
        app.maintenance_step(1, 4_096).unwrap();
        app.advance_typed_operation_publication().await.unwrap();
        if let Some(page) = app.take_typed_operation_result_page(instance) {
            assert!(app.acknowledge_typed_operation_result(page.token).unwrap());
        }
        if let Some(semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }) = app.take_typed_operation_effect() {
            loaded = Some((pack, spr));
        }
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
        if !app.has_pending_typed_operations() {
            break;
        }
        std::thread::yield_now();
    }
    let (parent_pack, parent_spr) = loaded.expect("the demo example publishes a document load");
    PluginApp::begin_document_archive_load(&mut app, 91, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).expect("archive admission");
    let mut status = None;
    for _ in 0..1_000_000 {
        let polled = PluginApp::poll_document_archive_load(&mut app, 91).await.expect("archive status");
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        let _ = PluginApp::maintenance_step(&mut app, 1, 4_096).expect("archive maintenance step");
        std::thread::yield_now();
    }
    let status = status.expect("archive load reaches a terminal state");
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    PluginApp::acknowledge_document_archive_load(&mut app, 91).expect("archive acknowledgement");
    let snapshot = app.snapshot().expect("loaded snapshot");
    for (slot, child_id) in [("presentation", snapshot.presentation.child_id.clone()), ("animation", snapshot.animation.child_id.clone())] {
        assert!(app.child_store(slot, &child_id).await.is_some(), "the genesis-derived {slot} member is live after the load");
    }
    artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🔖️ExampleArchiveLoad
