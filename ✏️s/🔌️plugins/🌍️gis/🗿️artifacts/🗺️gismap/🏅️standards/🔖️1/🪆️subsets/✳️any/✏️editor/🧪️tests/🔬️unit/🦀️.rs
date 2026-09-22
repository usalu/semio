pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry_and_members};
    use semio_framework_plugin::{EditorApp, Effect, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    use semio_s_artifact_stdio_semio::SemioMembers;

    pub type Gis2dApp = VcsArtifactApp<EditorApp<Gis2dPlayApp>, SemioMembers>;

    /// 🧹️ Owning guard that drains the exact close ladder before `ArtifactStore` drop asserts.
    pub struct Gis2dTestApp(Gis2dApp);

    impl std::ops::Deref for Gis2dTestApp {
        type Target = Gis2dApp;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for Gis2dTestApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for Gis2dTestApp {
        fn drop(&mut self) {
            if std::thread::panicking() || self.0.close_terminal_is_empty() {
                return;
            }
            close_registered_fixture_app(&mut self.0);
            assert!(self.0.close_terminal_is_empty(), "GIS map test app must reach terminal-empty close");
        }
    }

    /// 🧬️ Builds the real registered fixture and binds the instance addressed by [`meta`].
    pub async fn app() -> Gis2dTestApp {
        let mut app = new_app_with_registry_and_members::<EditorApp<Gis2dPlayApp>, SemioMembers>(gis2d_app_manifest_for_tests).await;
        app.bind_instance_id(meta("local").instance_id).await;
        Gis2dTestApp(app)
    }

    /// ✏️ Adapts `create_gis2d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::assert_declared_actions_bridge_to_commands` still expects —
    /// framework test context gap, not modifiable here.
    pub fn gis2d_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_gis2d_app(), examples: Vec::new() }
    }

    /// 🪟️ Targets the real GIS Map window instance for render, config, and measure authority.
    pub fn main_window_view() -> ViewModel {
        ViewModel { window_id: Some("gis-map-main-test".into()), window_instances: vec![ViewWindowInstance { id: "gis-map-main-test".into(), window_kind_id: map::GIS2D_PLAY_WINDOW_MAIN.into() }], ..Default::default() }
    }

    /// 🧹️ Drives a GIS Map fixture to its exact terminal-empty ownership witness.
    pub fn close(app: &mut Gis2dApp) {
        close_registered_fixture_app(app);
    }

    /// 📬️ Host-visible publication collected after one GIS Map command reaches quiescence.
    pub struct Gis2dDispatchReceipt(semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt);

    impl Gis2dDispatchReceipt {
        pub fn artifact_publication_count(&self) -> usize {
            self.0.lanes.iter().filter(|lane| **lane == TypedOperationResultLane::Artifact).count()
        }

        pub fn effects(&self) -> &[Effect] {
            &self.0.effects
        }
    }

    /// 🎯️ Dispatches a typed GIS Map command and drives its retained publication to exact ACK.
    pub async fn dispatch(app: &mut Gis2dApp, command: Gis2dCommand) -> Gis2dDispatchReceipt {
        let mut action_meta = meta("local");
        action_meta.view_state = Some(main_window_view());
        app.dispatch_typed(command, &action_meta).await.expect("dispatch admission");
        Gis2dDispatchReceipt(semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, action_meta.instance_id).await.expect("dispatch publication"))
    }

    /// 🖼️ Renders and retires one GIS Map fixture component tree.
    pub async fn render(app: &mut Gis2dApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &main_window_view()).await.expect("render")).expect("render projection")
    }

    /// 🗺️ Decodes the tiled-map surface payload from one rendered fixture tree.
    pub async fn render_tiled_map_scene(app: &mut Gis2dApp, body_key: &str) -> semio_framework_ui_scene::TiledMapScene {
        semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&render(app, body_key).await).expect("decode tiled-map scene")
    }

    /// 📐️ Projects the main GIS Map window measures through the registered fixture.
    pub async fn main_window_measures(app: &mut Gis2dApp) -> Vec<WindowMeasure> {
        let view = main_window_view();
        app.window_measures(&view).await.remove(view.window_id.as_deref().unwrap_or_default()).unwrap_or_default()
    }

    /// 🤝️ Applies disjoint commands to two registered, bound instances and proves convergence.
    pub async fn assert_two_instances_converge<P>(channel: &str, command_a: Gis2dCommand, command_b: Gis2dCommand, probe: impl Fn(&Gis2dApp) -> P)
    where
        P: PartialEq + std::fmt::Debug,
    {
        let mut instance_a = app().await;
        let mut instance_b = app().await;
        assert_eq!(instance_a.artifact_generation_now(), instance_b.artifact_generation_now(), "hot peers start at the same generation");
        assert_eq!(instance_a.snapshot().expect("initial a"), instance_b.snapshot().expect("initial b"), "hot peers start from the same independently materialized document");
        let document_a = instance_a.document_text().await.expect("initial document a");
        let document_b = instance_b.document_text().await.expect("initial document b");
        assert_eq!(document_a.dsl, document_b.dsl, "hot peers start from the same serialized initial snapshot");
        assert_eq!(document_a.ops, document_b.ops, "hot peers start with the same document identity and history cursor");
        let (backbone_a, backbone_b) = store::MemoryBackbone::pair(channel, channel).await;
        instance_a.attach_hot_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach hot backbone a after equal initial state proof");
        instance_b.attach_hot_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach hot backbone b after equal initial state proof");
        dispatch(&mut instance_a, command_a).await;
        dispatch(&mut instance_b, command_b).await;
        for _ in 0..8 {
            instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
            instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");
            instance_a.tick_backbone().await.expect("tick backbone a");
            instance_b.tick_backbone().await.expect("tick backbone b");
        }
        assert_eq!(probe(&instance_a), probe(&instance_b), "both instances must converge on the same snapshot");
        instance_a.detach_backbone().await.expect("detach backbone a");
        instance_b.detach_backbone().await.expect("detach backbone b");
        close(&mut instance_a);
        close(&mut instance_b);
    }
}

use super::*;
use crate::editor::gis2d::unit_tests::context::{app, close, gis2d_app_manifest_for_tests, render, Gis2dApp};
use semio_framework_plugin::{ContextMenuRequest, EditorApp, PluginApp, VcsArtifactApp};

#[test]
fn gis_map_durable_three_store_factory_builders_are_exact_role_ports() {
    let parent: std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<GisMapSnapshot, GisMapMutation>> = gis_map_parent_one_item_preparation_factory();
    let drawing: std::sync::Arc<
        dyn store::ArtifactStoreOneItemPreparationFactory<
            semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot,
            semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation,
        >,
    > = gis_map_drawing_one_item_preparation_factory();
    let value: std::sync::Arc<
        dyn store::ArtifactStoreOneItemPreparationFactory<
            semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot,
            semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::SemioValueMutation,
        >,
    > = gis_map_value_one_item_preparation_factory();
    assert_eq!([std::sync::Arc::strong_count(&parent), std::sync::Arc::strong_count(&drawing), std::sync::Arc::strong_count(&value)], [1, 1, 1]);
    let stamped = GisMapOneItemStampV1 { mutation_id: protocol::MutationId("11111111111111111111111111111111".into()), timestamp: protocol::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 } };
    let parent = gis_map_parent_stamped_one_item_preparation_factory(stamped.clone());
    let drawing = gis_map_drawing_stamped_one_item_preparation_factory(stamped.clone());
    let value = gis_map_value_stamped_one_item_preparation_factory(stamped);
    assert_eq!([std::sync::Arc::strong_count(&parent), std::sync::Arc::strong_count(&drawing), std::sync::Arc::strong_count(&value)], [1, 1, 1]);
}

#[semio_framework_async_macros::async_test]
async fn gis_map_window_ownership_one_item_preparation_transfers_its_candidate_once() {
    let snapshot = crate::schema::empty_gis_map_snapshot();
    let envelope = store::create_document_envelope(crate::GIS_MAP_SCHEMA, "gis-map-preparation-law", snapshot, None);
    let mut store = store::ArtifactStore::new(envelope)
        .await
        .expect("GIS Map preparation-law Store opens");
    store.install_document_store_owners_exact(crate::spr::gis_map_document_store_owners());
    let factory = gis_map_parent_one_item_preparation_factory();
    let mutation = GisMapMutation::CreatePosition(crate::mutations::create_position::CreatePosition {
        index: 0,
        item: crate::MapFeature { id: "preparation-law".into(), data: dsl::DslValue::Null },
    });
    let mut publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(991),
            store.generation_now(),
            store.content_revision_now(),
            "gis-map-preparation-law".into(),
            vec![mutation],
            Some("candidate transfer".into()),
            store::HistoryLane::Document,
            Some(&factory),
        )
        .expect("GIS Map one-item publication admits its exact factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES };
    let mut published = false;
    for _ in 0..64 {
        match store.advance_apply_batch(&mut publication, grant).expect("GIS Map candidate transfer turn") {
            store::ArtifactStoreOneItemAdvance::Published(_) => {
                published = true;
                break;
            }
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("GIS Map one-item preparation blocked after admitting its exact candidate"),
            store::ArtifactStoreOneItemAdvance::Progress(_) => {}
            store::ArtifactStoreOneItemAdvance::AwaitingAck(_) => panic!("GIS Map one-item preparation reached ACK without exposing its exact publication receipt"),
            store::ArtifactStoreOneItemAdvance::Complete => panic!("GIS Map one-item preparation completed before publishing its exact candidate"),
        }
    }
    assert!(published, "GIS Map one-item preparation must publish after one semantic-candidate turn and one exact transfer turn");
    assert!(publication.acknowledge());
    for _ in 0..64 {
        if matches!(publication.close_step(grant).expect("GIS Map publication close"), store::SnapshotRetirementStep::Complete) {
            break;
        }
    }
    assert!(publication.terminal_is_empty());
    drop(publication);
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<GisMapSnapshot, GisMapMutation>::new();
    for _ in 0..100_000 {
        if matches!(semio_framework_plugin::ArtifactOwnedDisposer::close_step(&mut disposer, &mut store, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("GIS Map preparation-law Store close"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(semio_framework_plugin::ArtifactOwnedDisposer::terminal_is_empty(&disposer, &store));
}

/// 🧾️ LAW: every parent snapshot this crate can hand a live document replacement projects into the
/// bounded child authority the replacement pump demands before it opens a single member. The pump
/// calls `ChildRestoreProjection::from_snapshot` on the CANDIDATE parent and reports only
/// `candidate parent child projection is invalid`, so the exact
/// `ChildRestoreProjectionError` has to be named here, on each snapshot the fixture below can
/// produce and on each codec round trip that reaches the candidate store.
#[test]
fn every_gis_map_parent_snapshot_projects_its_canonical_child_handles() {
    use store::ArtifactPack;

    let admit = |label: &str, snapshot: &GisMapSnapshot| {
        let projection = store::ChildRestoreProjection::from_snapshot(snapshot).unwrap_or_else(|error| panic!("{label} parent is not projectable: {error:?} (drawing {:?}/{:?}, image {:?}, value {:?}/{:?})", snapshot.drawing.child_id, snapshot.drawing.target, snapshot.image, snapshot.value.child_id, snapshot.value.target));
        assert_eq!(projection.len(), 2, "{label} parent declares exactly the drawing and value children");
        for index in 0..projection.len() {
            let (slot, fields) = projection.get(index).expect("admitted row");
            assert_eq!(fields.child_id, fields.artifact_id, "{label} parent slot {slot}: a composed child's id IS its target artifact id");
            assert_eq!(fields.artifact_kind, "s.stdio.semio", "{label} parent slot {slot} kind");
        }
    };

    for (label, snapshot) in [("empty", crate::schema::empty_gis_map_snapshot()), ("default-document", crate::schema::default_document())] {
        admit(label, &snapshot);
        admit(&format!("{label} pack round trip"), &GisMapSnapshot::decode_pack(&snapshot.encode_pack()).unwrap_or_else(|error| panic!("{label} parent pack round trip: {error}")));
        admit(&format!("{label} dsl round trip"), &<GisMapSnapshot as store::ArtifactDsl>::parse_dsl(&<GisMapSnapshot as store::ArtifactDsl>::print_dsl(&snapshot)).unwrap_or_else(|error| panic!("{label} parent dsl round trip: {error}")));
    }
}

fn gis_map_envelope_wire() -> Vec<u8> {
    use store::ArtifactPack;

    let snapshot = crate::schema::empty_gis_map_snapshot();
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": GIS_MAP_SCHEMA,
        "id": "gis-map-live-load",
        "vcs": {
            "initialSnapshot": snapshot_hex,
            "edits": [],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first GIS fixture envelope");
    let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis-map-live-load", snapshot, None);
    let mut retirement = crate::spr::gis_map_envelope_decode_owner_bundle().retire_envelope(envelope);
    for _ in 0..100_000 {
        match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("GIS fixture envelope retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return wire;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("unshared GIS fixture envelope retirement blocked"),
        }
    }
    panic!("GIS fixture envelope retirement did not reach terminal")
}

fn admit_gis_map_envelope(app: &mut Gis2dApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("GIS live envelope ingress credits");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded GIS live envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("GIS live envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("GIS live envelope seal/submit"));
    handle
}

async fn drive_gis_map_live_load(app: &mut Gis2dApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    let mut last = semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Pending;
    for _ in 0..100_000 {
        PluginApp::advance_typed_operation_publication(app).await.expect("GIS envelope load reactor turn");
        last = app.advance_artifact_envelope_load(handle).expect("GIS live load advancement");
        if !matches!(last, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Pending | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Progress) {
            return last;
        }
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one GIS live maintenance turn");
        let _ = semio_framework_job::pump_worker_job_retirements(8, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        last = app.advance_artifact_envelope_load(handle).expect("GIS live load advancement after maintenance");
        if !matches!(last, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Pending | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Progress) {
            return last;
        }
        std::thread::yield_now();
    }
    panic!(
        "GIS live envelope load did not reach terminal (last load poll {:?}, decode {:?}, replacement {:?})",
        last,
        app.poll_artifact_envelope_decode(handle),
        app.poll_artifact_store_replacement(handle)
    )
}

#[semio_framework_async_macros::async_test]
async fn gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
    let mut app = app().await;
    let base_generation = app.artifact_generation_now();
    let handle = admit_gis_map_envelope(&mut app, &gis_map_envelope_wire());
    assert_eq!(handle.generation, base_generation);
    assert_eq!(drive_gis_map_live_load(&mut app, handle).await, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact GIS load acknowledgement"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate GIS load acknowledgement is a no-op"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn gis_map_live_envelope_cancel_closes_retained_pages_without_publication() {
    let mut app = app().await;
    let base_generation = app.artifact_generation_now();
    let wire = gis_map_envelope_wire();
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled GIS ingress credits");
    let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled GIS first page");
    app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled GIS page admission failed: {fault:?}"));
    app.cancel_artifact_envelope_load(handle).expect("cancel exact GIS ingress");
    assert_eq!(drive_gis_map_live_load(&mut app, handle).await, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.artifact_generation_now(), base_generation);
    close(&mut app);
}

//#region 🔖️CommandSurface
/// 🎯️ One value per `app_commands!` row, in row order — the wire-law loop below and the id
/// uniqueness check both run off this list, so a new row that forgets to appear here fails the
/// coverage assertion.
fn every_command() -> Vec<Gis2dCommand> {
    vec![
        Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example::DEFAULT_EXAMPLE_ID.into() }),
        Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: r#"[{"id":"p1","lon":1.0,"lat":2.0}]"#.into() }),
        Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: vec!["r1".into(), "r2".into()], field: "label".into(), value: "Home".into() }),
        Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: "r1".into(), field: "label".into(), value: "Home".into() }),
        Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: "water".into() }),
        Gis2dCommand::FitWorld(fit_world::FitWorld {}),
        Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":0,"y":0,"zoom":1}"#.into() }),
        Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() }),
        Gis2dCommand::SetVectorStyle(set_vector_style::SetVectorStyle { value: "colored".into() }),
        Gis2dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "automatic".into() }),
        Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: "p1".into(), feature_kind: "position".into() }),
        Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "roads".into(), value: 1.5 }),
        Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "p1".into() }),
        Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {}),
    ]
}

/// 🏷️ The wire keyword each row prints under — the kebab `as` literal, independent of the camelCase
/// manifest action id. Pinned so a reordered/renamed row is caught here, not in production.
const WIRE_KEYWORDS: &[&str] =
    &["active-example", "patch-positions", "patch-routes", "patch-route", "toggle-layer-visibility", "fit-world", "camera", "render-mode", "vector-style", "lod-mode", "focus-feature", "layer-stroke-scale", "open-source", "propose-bounds-region"];

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(Gis2dCommand::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 14, "every Gis2dCommand row must be covered by every_command()");
}

#[test]
fn retained_factory_owns_every_migrated_command_and_exact_publication_lane() {
    assert_eq!(<Gis2dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS, GIS2D_RETAINED_TOOL_IDS);
    assert_eq!(<Gis2dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, GIS2D_RETAINED_PUBLICATION_CONTRACTS);
    assert_eq!(GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "setActiveExample").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig][..]));
    assert_eq!(GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "setCamera").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::WindowConfig][..]));
    assert_eq!(GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "openSource").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::HostOnly][..]));
    assert_eq!(GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "proposeBoundsRegion").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::HostOnly][..]));
}

#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    assert_eq!(every_command().len(), WIRE_KEYWORDS.len());
    for (command, keyword) in every_command().iter().zip(WIRE_KEYWORDS) {
        store::os_store::test_support::assert_op_text_binary_equivalence(command);
        let printed = protocol::OpText::print_op(command);
        assert!(printed == *keyword || printed.starts_with(&format!("{keyword} ")), "row {} printed {printed:?}, expected the {keyword:?} wire keyword", command.command_id());
    }
}

/// 🧷️ `PatchRoutes`' empty-`Vec` shape round-trips identically to its non-empty shape — the one
/// `Vec`-carrying optional-field case left after the interaction-mechanism migration deleted every
/// other optional-field row (`setSelection`/`setFeatureSelection`/`clearSelection`/`selectAll`).
#[semio_framework_async_macros::async_test]
async fn patch_routes_empty_route_ids_round_trips_text_and_binary() {
    store::os_store::test_support::assert_op_text_binary_equivalence(&Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: Vec::new(), field: "label".into(), value: String::new() }));
}

/// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
/// `command_id`. Uses the framework's own harness, which stages each action's declared args and
/// knows the framework-injected ids to skip (`undo`/`copy`/`recordTutorial`/…).
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Gis2dPlayApp>>(gis2d_app_manifest_for_tests).await;
    assert!(Gis2dPlayApp::command_from_action("noSuchAction", None).is_err());
}
//#endregion 🔖️CommandSurface

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_gis2d_app();
    assert_eq!(definition.modes.len(), 1);
    assert_eq!(definition.window_kinds.len(), 1);
    // 🧷️ The framework injects its own panel tabs on top of the app's three, so assert the app's
    // own tabs are stitched in rather than pinning a total.
    for body_key in [document_panel::GIS2D_PLAY_BODY_ARTIFACT, catalogue_panel::GIS2D_PLAY_BODY_CATALOGUE, inspection_panel::GIS2D_PLAY_BODY_INSPECTION] {
        assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
    }
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == crate::GISMAP_DIALECT.artifact_kind));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = app().await;
    assert!(render(&mut app, "gis2d.play.nope").await.contains("Unknown body"));
    close(&mut app);
}
//#endregion 🔖️Manifest

//#region 🔖️Media
#[semio_framework_async_macros::async_test]
async fn export_media_map_out_produces_a_2d_map_structured_payload() {
    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = Gis2dPlayApp::export_media("map:out", &doc).expect("map:out export");
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
    assert_eq!(schema, "2d.map");
    assert!(json.contains("positions"));
    drop(json);
    drop(document);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn import_media_features_in_adds_new_positions_as_operations() {
    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let incoming = serde_json::json!({ "positions": [{ "id": "imported-1", "lon": 1.0, "lat": 2.0 }] }).to_string();
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.map".into(), json: incoming } };
    let emit = Gis2dPlayApp::import_media("features:in", &media, &doc).expect("features:in import");
    assert!(emit.artifact_mutations.iter().any(|operation| matches!(operation, GisMapMutation::CreatePosition(payload) if payload.item.id == "imported-1")));
    drop(emit);
    drop(doc);
    drop(history);
    drop(document);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn media_ports_declare_features_in_and_map_out() {
    let ports = Gis2dPlayApp::media_ports().await;
    assert!(ports.iter().any(|port| port.id == "features:in"));
    assert!(ports.iter().any(|port| port.id == "map:out"));
}

/// 🧭️ Relocated from the artifact's `⚙️engine` tests (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `gis2d_io`/`gis2d_map_media`.
#[semio_framework_async_macros::async_test]
async fn gis2d_io_declares_the_features_in_and_map_out_ports() {
    let io = gis2d_io();
    assert_eq!(io.artifact_schema, GIS_MAP_SCHEMA);
    assert_eq!(io.artifact.id, crate::GISMAP_DIALECT.artifact_kind);
    let ports = io.all_ports().await;
    assert!(ports.iter().any(|port| port.id == "features:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
    let map_out = ports.iter().find(|port| port.id == "map:out").expect("map:out declared");
    assert_eq!(map_out.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(map_out.kind_id.as_deref(), Some(crate::GISMAP_DIALECT.artifact_kind));
}

#[semio_framework_async_macros::async_test]
async fn gis2d_map_media_exports_the_document_descriptor() {
    let document = crate::schema::default_document();
    let media = gis2d_map_media(&document);
    let MediaPayload::Structured { schema, json } = media.payload else {
        panic!("expected a structured map:out payload");
    };
    assert_eq!(schema, "2d.map");
    assert!(json.contains("positions"));
}
//#endregion 🔖️Media

//#region 🔖️ContextMenu
/// 🖱️ Grouped disclosure: the empty-canvas context menu (no feature under the pointer) stays
/// within the row budget and keeps the known destructive `clearSelection` last, matching the
/// canonical migration pattern.
#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_budget_and_keeps_clear_selection_destructive_last() {
    let mut app = app().await;
    let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "gis2dMap".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let menu = app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("empty-canvas context menu should not be empty");
    assert_eq!(last.id, "clearSelection", "known destructive clearSelection must be last: {menu:?}");
    assert_eq!(last.destructive, Some(true), "clearSelection must be marked destructive: {menu:?}");
    drop(menu);
    close(&mut app);
}
//#endregion 🔖️ContextMenu

//#region 🎬️StageableActionVocabulary
/// 🔎️ One declared action of the gis2d manifest, wherever its window kind declares it.
fn declared_action(definition: &semio_framework_plugin::AppDefinition, id: &str) -> ActionDefinition {
    definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == id).unwrap_or_else(|| panic!("{id} is declared")).clone()
}

/// 🗺️ Every per-feature editing verb is declared a document `Mutation`, is argument-staged so the
/// Actions rail can dispatch it, and every one of its arguments carries an effective default — the
/// palette must be able to run it without the operator typing anything.
#[semio_framework_async_macros::async_test]
async fn every_feature_editing_verb_is_a_fully_defaulted_stageable_mutation() {
    let definition = create_gis2d_app();
    for id in ["addFeature", "moveFeature", "renameFeature", "deleteFeature"] {
        let action = declared_action(&definition, id);
        assert_eq!(action.kind, ActionKind::Mutation, "{id} writes the document");
        assert!(action.in_palette, "{id} is reachable from the command palette");
        assert!(!action.args.is_empty(), "{id} declares staged arguments");
        for arg in &action.args {
            assert!(arg.default.is_some(), "{id} argument {} must carry a default so the rail can dispatch it unattended", arg.id);
        }
        assert!(action.args.iter().any(|arg| arg.id == "collection"), "{id} addresses a document collection");
    }
}

/// 🗂️ The layer verbs that were unreachable from the rail (no argument schema meant an empty
/// `layerId` and a silent no-op) now stage the same layer vocabulary the panels enumerate.
#[semio_framework_async_macros::async_test]
async fn layer_verbs_stage_every_declared_map_layer() {
    let definition = create_gis2d_app();
    assert_eq!(GIS_MAP_LAYER_IDS.len(), GIS_MAP_LAYER_ARG_NAMES.len(), "the manifest-time layer names mirror the layer stack one-to-one");
    let declared: Vec<&str> = GIS_MAP_LAYER_IDS.iter().map(|(layer_id, _, _)| *layer_id).collect();
    for id in ["toggleLayerVisibility", "setLayerStrokeScale"] {
        let action = declared_action(&definition, id);
        let layer = action.args.iter().find(|arg| arg.id == "layerId").unwrap_or_else(|| panic!("{id} stages a layer"));
        let semio_framework_plugin::ArgSchema::String { options, .. } = &layer.schema else {
            panic!("{id} stages its layer as a discrete choice");
        };
        let staged: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
        assert_eq!(staged, declared, "{id} stages exactly the declared layer stack, in order");
    }
}

/// 🧬️ The retained tool surface, the publication contracts and the bounded-first-step proof catalog
/// must agree on the tool set — a command missing from any one of the three is dead at runtime even
/// though `cargo check` is green (the `TOOL_JOB_IDS` + factory + proofs chain).
#[semio_framework_async_macros::async_test]
async fn the_retained_tool_surface_proofs_and_publication_contracts_cover_every_command() {
    use std::collections::BTreeSet;
    let retained: BTreeSet<String> = GIS2D_RETAINED_TOOL_IDS.iter().map(|id| (*id).to_string()).collect();
    let published: BTreeSet<String> = GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id.to_string()).collect();
    assert_eq!(retained, published, "every retained tool declares its publication lanes");
    let proofs: BTreeSet<String> = <Gis2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().into_iter().map(|proof| proof.tool_id().to_string()).collect();
    assert_eq!(proofs, retained, "every retained tool carries its owner-local bounded reducer proof");
}
//#endregion 🎬️StageableActionVocabulary
