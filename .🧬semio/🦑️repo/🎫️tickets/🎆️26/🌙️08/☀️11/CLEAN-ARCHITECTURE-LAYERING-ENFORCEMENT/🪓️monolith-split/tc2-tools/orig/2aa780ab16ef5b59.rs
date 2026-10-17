//! ✉️ Drawing editor unit laws over the live envelope: wire framing, admission, load driving, recursive clone swap, cancellation and edit identifier bounds.

use super::*;

fn drawing_envelope_wire() -> Vec<u8> {
    use pack::snapshot::ArtifactPack;

    let mut snapshot = default_drawing_document("drawing-retained-load", None);
    let mut group = semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Nested")).to_string().into()).expect("nonempty authored identity"), "Nested");
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Path")).to_string().into()).expect("nonempty authored identity"), "Path", vec![semio_s_artifact_draw_drawing_model::PathSegment::Move { to: [1.0, 2.0] }, semio_s_artifact_draw_drawing_model::PathSegment::Line { to: [3.0, 4.0] }].into()));
    }
    let retained_target = match &group {
        DrawingLayerNode::Group(value) => layer_id(&value.children[0]).to_string(),
        _ => unreachable!("retained Drawing fixture group remains exact"),
    };
    snapshot.layers.push(group);
    snapshot.assets.insert(semio_framework_value::paged::PagedUtf8::from("image-a"), semio_s_artifact_draw_drawing_model::DrawingImageAsset { width: 1, height: 1, samples: vec![[0,0,0,0];1].into() });
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let mutation = DrawingMutation::RenameLayer(semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer { layer_id: retained_target.clone().into(), new_name: "Retained Path".into() });
    let mutation_hex = semio_s_artifact_draw_drawing_mutations::binary::encode_op(&mutation).expect("Drawing fixture mutation pack").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": DRAWING_DOCUMENT_SCHEMA,
        "id": "drawing-retained-load",
        "vcs": {
            "initialPack": snapshot_hex,
            "edits": [{
                "id": "drawing-retained-edit-final",
                "actor": "drawing-retained-actor",
                "line": null,
                "forwards": [mutation_hex],
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "2026-08-23T00:00:00.000Z"
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first Drawing fixture envelope");
    let envelope = store::os_store::document_materialize::create_document_envelope(DRAWING_DOCUMENT_SCHEMA, "drawing-retained-load", snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut retirement = semio_s_artifact_draw_drawing_host_arena::drawing_envelope_decode_owner_bundle().retire_envelope(envelope);
    for _ in 0..100_000 {
        let copy_bytes = retirement.next_copy_byte_demand().expect("Drawing fixture envelope copy demand");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy_bytes,
            maximum_capacity_bytes: retirement.next_capacity_byte_demand(copy_bytes).expect("Drawing fixture envelope capacity demand"),
            maximum_release_bytes: retirement.next_release_byte_demand().expect("Drawing fixture envelope release demand"),
            maximum_depth: retirement.next_depth_demand().expect("Drawing fixture envelope depth demand").max(1),
        };
        let step = retirement.close_step(grant).expect("Drawing fixture envelope retirement");
        assert!(step.progress().fits(grant));
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            assert!(retirement.terminal_is_empty());
            drop(retirement);
            return wire;
        }
    }
    panic!("Drawing fixture envelope retirement did not reach terminal")
}

pub(super) fn admit_drawing_envelope(app: &mut DrawingApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Drawing live envelope ingress credits");
    for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Drawing envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Drawing envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("Drawing envelope seal"));
    handle
}

pub(super) fn drive_drawing_load(app: &mut DrawingApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..100_000 {
        app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Drawing maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("Drawing load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Drawing retained envelope load did not reach terminal")
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed() {
    let mut app = drawing_app().await;
    let base_generation = app.artifact_generation_now();
    let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
    assert_eq!(handle.generation, base_generation);
    let poll = drive_drawing_load(&mut app, handle);
    assert_eq!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready, "valid retained Drawing load refusal: {:?}", app.artifact_store_replacement_refusal(handle));
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
    let projection = app.snapshot().expect("Drawing retained mutation publication");
    let renamed = semio_s_artifact_draw_drawing_model::find_drawing_layer(&projection, "Path").expect("retained Drawing target");
    assert_eq!(semio_s_artifact_draw_drawing_model::layer_base(renamed).name, "Retained Path");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("first Drawing acknowledgement"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Drawing acknowledgement"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_cancel_closes_retained_pages_without_publication() {
    let mut app = drawing_app().await;
    let base_generation = app.artifact_generation_now();
    let wire = drawing_envelope_wire();
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Drawing ingress credits");
    let first = &wire[..wire.len().min(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Drawing first page");
    app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Drawing page admission failed: {fault:?}"));
    app.cancel_artifact_envelope_load(handle).expect("cancel Drawing ingress");
    assert_eq!(drive_drawing_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.artifact_generation_now(), base_generation);
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle() {
    for turns in [0usize, 1, 2, 8] {
        let mut app = drawing_app().await;
        let base_generation = app.artifact_generation_now();
        let base_id = app.snapshot().expect("Drawing last-valid snapshot").id;
        let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
        for _ in 0..turns {
            app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("bounded Drawing staged maintenance");
        }
        let stale = semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
        assert!(app.advance_artifact_envelope_load(stale).is_err(), "stale staged handle cannot consume the exact operation owner");
        app.cancel_artifact_envelope_load(handle).expect("exact Drawing staged cancellation");
        assert!(matches!(drive_drawing_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled));
        assert_eq!(app.artifact_generation_now(), base_generation);
        assert_eq!(app.snapshot().expect("Drawing last-valid survives staged cancel").id, base_id);
    }

    let mut app = drawing_app().await;
    let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
    let poll = drive_drawing_load(&mut app, handle);
    assert_eq!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready, "valid staged Drawing load refusal: {:?}", app.artifact_store_replacement_refusal(handle));
    let stale = semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
    assert!(!app.acknowledge_artifact_store_replacement(stale).expect("stale Drawing ACK is refused without retiring the exact owner"));
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("exact staged Drawing ACK"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate staged Drawing ACK is idempotent"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate() {
    for final_edit in [false, true] {
        let mut value: serde_json::Value = serde_json::from_slice(&drawing_envelope_wire()).expect("Drawing retained fixture JSON");
        let edits = value.pointer_mut("/vcs/edits").and_then(serde_json::Value::as_array_mut).expect("Drawing retained edits");
        if final_edit {
            let mut first = edits[0].clone();
            first["id"] = serde_json::Value::String("drawing-retained-edit-first".into());
            edits.insert(0, first);
        }
        edits.last_mut().expect("Drawing final edit")["id"] = serde_json::Value::String("x".repeat(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1));
        let wire = serde_json::to_vec(&value).expect("hostile Drawing edit fixture");
        let mut app = drawing_app().await;
        let generation = app.artifact_generation_now();
        let handle = admit_drawing_envelope(&mut app, &wire);
        assert_eq!(drive_drawing_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(app.artifact_generation_now(), generation);
    }
}
