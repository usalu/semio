//! 🌉️ Genuine GIS component cold-pair, render-patch, and addressed mutation acceptance.

use semio_framework::manifest::{ActionAddress, ActionInvocation, ViewModel, ViewWindowInstance};
use semio_framework_actor::ActorId;
use semio_framework_plugin::AppInstanceId;
use semio_framework_os_kernel::os_spr::channel::{encode_app_command, AppCommand};
use semio_framework::kernel::{
    ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest, ActorUiPatchReceipt, Budget, ColdDocumentPairApplied, ColdDocumentPairFrontier, ColdDocumentPairHeader,
    ColdDocumentPairPage, ColdPairIngressStatus, CommandBatch, CommandBatchDriver, CommandBatchProgress, CommandEnvelope, CommandEnvelopeSet, Event, QuotaSchema, TurnResult, UiTurnPatchTransportLease, COLD_PAIR_PAGE_MAXIMUM_BYTES, COMMAND_PAGE_MAXIMUM_BYTES,
};
use semio_framework_plugin_host::{shard, GuestInstance, GuestRuntime, PackageHash, PackageId, PackageRef, SharedEngineConfig, WasmtimeRuntime};
use semio_framework_ui_contract::{Component, SurfaceKind, UiPatch, UiPatchOp};
use semio_framework_ui_scene::TiledMapScene;
use semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use semio_s_artifact_gis_gismap::{gis_map_snapshot_with_derived_children, GisMapSnapshot, MapFeature, GIS_MAP_SCHEMA};
use std::collections::BTreeMap;

const INSTANCE: u32 = 1;
const ACTIVATION_GENERATION: u64 = 41;
const OPEN_SEQUENCE: u64 = 7;
const TRANSFER_GENERATION: u64 = 51;
const COMMAND_GENERATION: u64 = 91;
const COMMAND_SEQUENCE: u64 = 11;
const SURFACE: &str = "1:gis2d-main";

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🔣️.json")).expect("schema-owned GIS component fixture")
}

fn budget() -> Budget {
    Budget { fuel: 200_000_000, deadline_ms: 30_000, max_effects: 64, max_patch_bytes: 4 * 1024 * 1024, max_frames: 64 }
}

fn component_bytes() -> Vec<u8> {
    let bytes = std::fs::read(std::env::var_os("SEMIO_GIS_COMPONENT_WASM").expect("receipt-bound GIS component path")).expect("read receipt-bound GIS component");
    assert_eq!(hex32(std::env::var("SEMIO_GIS_COMPONENT_SHA256").expect("receipt-bound component hash")), semio_framework_hash::Sha256::digest(&bytes));
    bytes
}

fn hex32(value: String) -> [u8; 32] {
    assert_eq!(value.len(), 64);
    let mut bytes = [0; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).expect("lowercase SHA-256");
    }
    bytes
}

async fn open_live(bytes: &[u8]) -> (WasmtimeRuntime, GuestInstance, ActorInstanceLifetime) {
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("shared Wasmtime engine");
    let package = PackageRef { package: PackageId("semio:gis".into()), hash: PackageHash(*semio_framework_hash::hash(bytes).as_bytes()) };
    let compiled = runtime.compile(&package, bytes).await.expect("genuine GIS component compiles");
    let mut instance = runtime.instantiate(&compiled, ActorId(1), &[], &budget()).await.expect("genuine GIS component instantiates");
    let open = Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: ACTIVATION_GENERATION, instance_id: INSTANCE, request_sequence: OPEN_SEQUENCE },
        app_id: AppInstanceId("s.gis.gismap@1/*#editor".into()),
        actor: "component-cold-map-patch".into(),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: QuotaSchema::default(),
    };
    let captured = runtime.execute_turn(&mut instance, &[open], budget()).await.expect("GIS component captures lifecycle").lifecycle_receipt.expect("captured lifecycle receipt");
    let lifetime = match captured {
        ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence } => {
            assert_eq!(request_sequence, OPEN_SEQUENCE);
            assert_eq!(lifetime.activation_generation, ACTIVATION_GENERATION);
            assert_eq!(lifetime.instance_id, INSTANCE);
            lifetime
        }
        _ => panic!("first lifecycle receipt must be Captured"),
    };
    let acknowledged = runtime.execute_turn(&mut instance, &[Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured })], budget()).await.expect("exact lifecycle acknowledgement");
    assert!(acknowledged.ui_patches.is_empty());
    assert_eq!(acknowledged.ui_patch_receipt, None);
    (runtime, instance, lifetime)
}

async fn canonical_pair(value: &serde_json::Value) -> (Vec<u8>, Vec<u8>) {
    use semio_framework_os_kernel::ArtifactPack;

    let snapshot = gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![MapFeature { id: value["id"].as_str().expect("position id").into(), data: semio_framework_os_kernel::DslValue::from(value) }], ..Default::default() });
    let envelope = semio_framework_os_kernel::create_document_envelope::<GisMapSnapshot, GisMapMutation>(GIS_MAP_SCHEMA, "shared-map", snapshot, None);
    let files = semio_framework_os_kernel::print_document_pack(&envelope).await.expect("canonical GIS pack pair");
    assert_eq!(files.pack, envelope.vcs.initial_snapshot.encode_pack());
    let mut retirement = semio_s_artifact_gis_gismap::spr::gis_map_envelope_decode_owner_bundle().retire_envelope(envelope);
    let mut retired = false;
    for _ in 0..100_000 {
        match retirement.close_step(1, semio_framework_os_kernel::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("GIS envelope retirement") {
            semio_framework_os_kernel::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                retired = true;
                break;
            }
            semio_framework_os_kernel::SnapshotRetirementStep::Pending { .. } => {}
            semio_framework_os_kernel::SnapshotRetirementStep::Blocked => panic!("unshared GIS fixture envelope retirement blocked"),
        }
    }
    assert!(retired, "GIS fixture envelope retires within the bounded grant");
    assert!(!files.pack.is_empty() && !files.spr.is_empty());
    (files.pack, files.spr)
}

fn cold_header(pack: &[u8], spr: &[u8], lifetime: ActorInstanceLifetime) -> ColdDocumentPairHeader {
    let mut aggregate = semio_framework_hash::Sha256::new();
    aggregate.update(pack);
    aggregate.update(spr);
    ColdDocumentPairHeader {
        lifetime,
        transfer_generation: TRANSFER_GENERATION,
        descriptor_sha256: hex32(std::env::var("SEMIO_GIS_DESCRIPTOR_SHA256").expect("receipt-bound descriptor hash")),
        baseline_frontier: ColdDocumentPairFrontier { document_id: "shared-map".into(), head_edit_ordinal: 7, head_edit_id: "edit-7".into(), last_commit_seq: 5, chain_sha256: [0x22; 32] },
        pack_sha256: semio_framework_hash::Sha256::digest(pack),
        spr_sha256: semio_framework_hash::Sha256::digest(spr),
        aggregate_sha256: aggregate.finalize(),
        pack_length: pack.len() as u64,
        spr_length: spr.len() as u64,
        page_count: (pack.len() + spr.len()).div_ceil(COLD_PAIR_PAGE_MAXIMUM_BYTES) as u32,
    }
}

fn cold_page(header: &ColdDocumentPairHeader, pack: &[u8], spr: &[u8], page_index: u32) -> ColdDocumentPairPage {
    let offset = page_index as usize * COLD_PAIR_PAGE_MAXIMUM_BYTES;
    let length = header.page_length(page_index).expect("admitted page");
    let mut bytes = Vec::with_capacity(length);
    if offset < pack.len() {
        let count = length.min(pack.len() - offset);
        bytes.extend_from_slice(&pack[offset..offset + count]);
    }
    if bytes.len() < length {
        let spr_offset = offset + bytes.len() - pack.len();
        bytes.extend_from_slice(&spr[spr_offset..spr_offset + length - bytes.len()]);
    }
    ColdDocumentPairPage { header: header.clone(), page_index, bytes }
}

async fn load_pair(runtime: &WasmtimeRuntime, instance: &mut GuestInstance, lifetime: ActorInstanceLifetime, pack: &[u8], spr: &[u8]) -> ColdDocumentPairApplied {
    let header = cold_header(pack, spr, lifetime);
    let mut applied = None;
    for page_index in 0..header.page_count {
        let result = runtime.execute_turn(instance, &[Event::ColdDocumentPairPage(cold_page(&header, pack, spr, page_index))], budget()).await.expect("genuine guest accepts cold page");
        match result.cold_pair_ingress {
            ColdPairIngressStatus::PageAccepted(cursor) => assert_eq!(cursor, header.cursor(page_index)),
            ColdPairIngressStatus::Applied(receipt) => {
                assert_eq!(page_index + 1, header.page_count);
                applied = Some(receipt);
            }
            other => panic!("unexpected cold ingress status: {other:?}"),
        }
    }
    let applied = applied.expect("terminal page applies canonical GIS pair");
    assert_eq!(applied.lifetime, lifetime);
    assert_eq!(applied.transfer_generation, TRANSFER_GENERATION);
    assert_eq!(applied.baseline_frontier, header.baseline_frontier);
    assert_eq!(applied.aggregate_sha256, header.aggregate_sha256);
    applied
}

fn scene_from_patch(patch: &UiPatch) -> Option<TiledMapScene> {
    patch.ops.iter().find_map(|operation| {
        let component = match operation {
            UiPatchOp::Upsert(record) => Some(&record.component),
            UiPatchOp::SetComponent { component, .. } => Some(component),
            _ => None,
        }?;
        let Component::Surface(props) = component else { return None };
        if props.kind != SurfaceKind::TiledMap || props.doc_schema.as_str() != "tiled-map@1" {
            return None;
        }
        semio_framework_ui_scene::decode(props).ok()
    })
}

async fn retain_patch(result: TurnResult, expected_lifetime: ActorInstanceLifetime, session: u64) -> (ActorUiPatchReceipt, String, u64, Option<TiledMapScene>) {
    let receipt = result.ui_patch_receipt.expect("one exact GIS patch receipt");
    assert_eq!(receipt.lifetime, expected_lifetime);
    assert_eq!(result.ui_patches.len(), 1);
    let patch = result.ui_patches.iter().next().expect("one GIS patch owner");
    assert_eq!(patch.surface.as_ref(), SURFACE);
    let scene = scene_from_patch(patch);
    let surface = patch.surface.as_ref().to_string();
    let revision = patch.revision.0;
    let bridged = shard::to_actor_turn_result(result, session, 0, 0).await.expect("GIS patch enters retained shard owner");
    let lease = UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).expect("one exact GIS patch transport token");
    assert!(UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).is_err());
    let mut owner = lease.take_owner().unwrap_or_else(|_| panic!("one GIS patch owner"));
    assert_eq!(owner.len(), 1);
    while !owner.close_step() {}
    (receipt, surface, revision, scene)
}

async fn render_until_scene(runtime: &WasmtimeRuntime, instance: &mut GuestInstance, lifetime: ActorInstanceLifetime, marker: &str, session: &mut u64) -> (TiledMapScene, u64) {
    use semio_framework_os_kernel::ToValue;
    let mut event = Event::SurfaceVisible { surface: SURFACE.into(), body_key: "gis2d.play.composite".into(), view_state: semio_framework_os_kernel::pack_rt::encode_wire_value(&gis_view().to_value()) };
    let mut matched = None;
    for _ in 0..256 {
        let result = runtime.execute_turn(instance, &[event], budget()).await.expect("genuine GIS render turn");
        event = Event::Wake;
        if result.ui_patches.is_empty() {
            if let Some(scene) = matched.take() {
                return scene;
            }
            continue;
        }
        let (receipt, surface, revision, scene) = retain_patch(result, lifetime, *session).await;
        *session += 1;
        event = Event::PatchAck { receipt, surface, revision };
        if let Some(scene) = scene {
            assert!(scene.map_fixture_json.contains(marker), "map scene omits {marker}");
            matched = Some((scene, revision));
        }
    }
    panic!("genuine GIS component did not render a tiled-map patch within 256 turns")
}

fn gis_view() -> ViewModel {
    ViewModel {
        active_mode_id: Some("edit".into()),
        active_window_kind_id: Some("gis2d-main".into()),
        window_id: Some("gis2d-main".into()),
        window_instances: vec![ViewWindowInstance { id: "gis2d-main".into(), window_kind_id: "gis2d-main".into() }],
        ..Default::default()
    }
}

async fn dispatch_patch_positions(runtime: &WasmtimeRuntime, instance: &mut GuestInstance, position: &serde_json::Value) {
    use semio_framework_os_kernel::ToValue;

    let mut arguments = BTreeMap::new();
    arguments.insert("positionsJson".into(), semio_framework_os_kernel::DslValue::String(serde_json::to_string(&[position]).expect("position JSON")));
    let invocation = ActionInvocation {
        address: ActionAddress { plugin_id: "gis".into(), app_id: "s.gis.gismap@1/*#editor".into(), mode_id: "edit".into(), window_kind_id: "gis2d-main".into(), window_instance_id: "gis2d-main".into(), action_id: "patchPositions".into() },
        arguments,
    };
    let view = gis_view();
    let command = AppCommand::Command { seq: COMMAND_SEQUENCE, command: semio_framework_os_kernel::pack_rt::encode_wire_value(&invocation.to_value()), view_state: semio_framework_os_kernel::pack_rt::encode_wire_value(&view.to_value()) };
    let command = encode_app_command(&command).await.expect("encode addressed GIS mutation");
    let mut envelopes = CommandEnvelopeSet::try_new().expect("fixed command owners");
    envelopes.try_push(CommandEnvelope { instance: INSTANCE, seq: COMMAND_SEQUENCE, command }).unwrap_or_else(|(fault, _)| panic!("admit GIS command: {fault:?}"));
    let batch = CommandBatch::try_new(COMMAND_GENERATION, envelopes).unwrap_or_else(|(fault, _)| panic!("admit GIS command batch: {fault:?}"));
    let mut driver = CommandBatchDriver::new(COMMAND_GENERATION, batch);
    for _ in 0..256 {
        let event = driver.next_page().expect("retained GIS command page").map(|(cursor, bytes)| Event::CommandIngressPage { cursor, bytes }).unwrap_or(Event::Wake);
        let result = runtime.execute_turn(instance, &[event], budget()).await.expect("genuine GIS command turn");
        assert!(result.ui_patches.is_empty(), "command mutation publishes only after an explicit surface render turn");
        match driver.observe(&result.command_ingress, COMMAND_PAGE_MAXIMUM_BYTES).expect("exact GIS command acknowledgement") {
            CommandBatchProgress::Complete => return,
            CommandBatchProgress::Faulted => panic!("genuine GIS patchPositions command faulted"),
            CommandBatchProgress::PageReady | CommandBatchProgress::Waiting => {}
        }
    }
    panic!("genuine GIS patchPositions command did not reach terminal acknowledgement")
}

#[semio_framework_async_macros::async_test]
async fn genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface() {
    let fixture = fixture();
    let component = component_bytes();
    let (runtime, mut instance, lifetime) = open_live(&component).await;
    let (pack, spr) = canonical_pair(&fixture["document"]["before"]).await;
    let applied = load_pair(&runtime, &mut instance, lifetime, &pack, &spr).await;
    assert_ne!(applied.aggregate_sha256, [0; 32]);
    let mut session = 940_051;
    let (before, before_revision) = render_until_scene(&runtime, &mut instance, lifetime, "cold-before", &mut session).await;
    assert!(!before.map_fixture_json.contains("patched-after"));
    dispatch_patch_positions(&runtime, &mut instance, &fixture["document"]["after"]).await;
    let (after, after_revision) = render_until_scene(&runtime, &mut instance, lifetime, "patched-after", &mut session).await;
    assert!(!after.map_fixture_json.contains("cold-before"));
    assert!(after_revision > before_revision);
}

#[semio_framework_async_macros::async_test]
async fn genuine_gis_component_rejects_stale_cold_authority_before_loading() {
    let fixture = fixture();
    let component = component_bytes();
    let (runtime, mut instance, lifetime) = open_live(&component).await;
    let (pack, spr) = canonical_pair(&fixture["document"]["before"]).await;
    let mut header = cold_header(&pack, &spr, lifetime);
    header.lifetime.guest_lifetime += 1;
    let result = runtime.execute_turn(&mut instance, &[Event::ColdDocumentPairPage(cold_page(&header, &pack, &spr, 0))], budget()).await.expect("stale cold page is a typed result");
    assert!(matches!(result.cold_pair_ingress, ColdPairIngressStatus::Fault { ref fault, .. } if fault == b"cold-pair.not-live"));
    assert!(result.ui_patches.is_empty());
}
