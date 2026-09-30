    {
        let left_state = app.window_transient_snapshot(&left).expect("left preview transient snapshot").expect("left preview owner");
        let right_state = app.window_transient_snapshot(&right).expect("right preview transient snapshot").expect("right preview owner");
        assert!(left_state.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.as_deref()).is_some_and(|text| !text.is_empty()));
        assert!(right_state.get::<Generation3dPreviewWindowTransientOwner>().is_some_and(|state| state.preview_eval_text.is_none()));
    }
    let config_after = app.config_pack().await.expect("Generation3d app config after preview evaluation");
    assert_eq!((config_after.pack, config_after.spr), (config_before.pack, config_before.spr));
    drain_flow_eval_ticks_with_view(&mut app, &right).await;
    assert!(app.window_transient_snapshot(&right).expect("right evaluated snapshot").and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().cloned()).is_some_and(|state| state.preview_eval_text.is_some()));
    let document = app.document_pack().await.expect("Generation3d document before reload");
    app.load_document_pack(&document).await.expect("same document reload resets preview window transient");
    for view in [&left, &right] {
        assert!(app.window_transient_snapshot(view).expect("reset preview snapshot").and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().cloned()).is_some_and(|state| state.preview_eval_text.is_none()));
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut **app);
}
fn production_initial_snapshot(label: &str) -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.host_snapshot.schema = label.into();
    for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
        snapshot.host_snapshot.widgets.push(semio_framework_artifact_flow_flow::Widget::InputNote { id: id.into(), text: text.into() });
    }
    snapshot.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
    snapshot.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
    snapshot.host_snapshot.layout.insert("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 });
    snapshot.host_snapshot.layout.insert("clear-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 });
    for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
        snapshot.generation.cold_builder_mut().unwrap().generations.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() });
    }
    snapshot.generation.cold_builder_mut().unwrap().selected_generation_id = Some("rename-generation".into());
    snapshot
}

fn production_mutations() -> Vec<Generation3dMutation> {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::*;
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("production".into()))),
        ),
    );
    vec![
        Generation3dMutation::CreateWidget(create_widget::CreateWidget {
            index: 0,
            widget: semio_framework_artifact_flow_flow::Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true },
        }),
        Generation3dMutation::UpdateWidget(update_widget::UpdateWidget {
            widget: semio_framework_artifact_flow_flow::Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() },
        }),
        Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: "delete-target".into() }),
        Generation3dMutation::ConnectSynapse(connect_synapse::ConnectSynapse {
            index: 0,
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() },
        }),
        Generation3dMutation::UpdateSynapse(update_synapse::UpdateSynapse {
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() },
        }),
        Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: "disconnect-synapse".into() }),
        Generation3dMutation::MoveWidget(move_widget::MoveWidget { id: "move-target".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 31.0, y: -17.0 } }),
        Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: "clear-target".into() }),
        Generation3dMutation::UpdateCamera(update_camera::UpdateCamera { camera: semio_framework_artifact_flow_flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 } }),
        Generation3dMutation::ChangeSchema(change_schema::ChangeSchema { new_schema: "flow.host_snapshot.production-retained".into() }),
        Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation: semio_framework_artifact_playbook_playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: Default::default() } }),
        Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id: "delete-generation".into() }),
        Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id: "rename-generation".into(), new_name: "After Rename".into() }),
        Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue {
            id: "change-generation".into(),
            question_id: "deep-answer".into(),
            new_value: serde_json::json!({"object": {"array": [1.0, false, "retained"]}}).into(),
        }),
    ]
}

fn production_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::new();
    value.try_reserve_exact(bytes.len() * 2).expect("P3 production hex preflight");
    for byte in bytes {
        value.push(char::from(DIGITS[usize::from(byte >> 4)]));
        value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    value
}

fn production_semantic_digest(snapshot: &Generation3dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation3d.production-law.semantic");
    digest.observe(&semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::snapshot::binary::encode(snapshot));
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Generation3dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let mutations = production_mutations();
    assert_eq!(mutations.len(), 14, "production ingress carries every P3 mutation variant including delete-widget-position");
    let mut mutation_hex = Vec::new();
    mutation_hex.try_reserve_exact(mutations.len()).expect("P3 production mutation owner preflight");
    for mutation in &mutations {
        mutation_hex.push(production_hex(&semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::encode_op(mutation).expect("P3 production mutation encoding")));
    }
    let mut expected = production_initial_snapshot(label);
    semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::generation3d_apply_retained_mutations_for_test(&mut expected, &mutations);
    let expected_digest = production_semantic_digest(&expected);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": GENERATION_3D_SCHEMA,
        "id": "generation3d-production-mounted-law",
        "vcs": {
            "initialSnapshot": production_hex(&semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::snapshot::binary::encode_mounted(&snapshot)),
            "edits": [{
                "id": "generation3d-production-all14-edit",
                "actor": "generation3d-production-law",
                "forwards": mutation_hex,
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "1"
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first P3 production fixture envelope");
    // 🧹️ A `create-widget`/`update-widget` row owns a whole `Widget`, and the seed projection owns
    // an `OrderedMap` layout root — both fail-close on a bare drop, so this fixture retires what it
    // authored instead of letting the scope drop it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    for mutation in mutations {
        mutation.retire_cold();
    }
    snapshot.retire_cold();
    (wire, expected, expected_digest)
}

/// 🔐️ Owns the publication lease `admit_production_envelope` took and releases it even when the law
/// panics before its explicit release. The lease table is a PROCESS-GLOBAL 4-slot
/// `FixedOperationRegistry` (`🧬️schema/🧬️mutations/💾️binary/🦀️.rs:211`), so one leaked slot turns every
/// later law in the same binary into `generation3d-publication.saturated` — an order-dependent red
/// that has nothing to do with what those laws assert
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
struct Generation3dProductionLease {
    handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle,
    released: bool,
}

impl Generation3dProductionLease {
    fn release(&mut self) -> bool {
        if self.released {
            return false;
        }
        self.released = true;
        semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::generation3d_release_publication_authority(self.handle.operation, self.handle.generation)
    }
}

impl Drop for Generation3dProductionLease {
    fn drop(&mut self) {
        self.release();
    }
}

fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, wire: &[u8]) -> Generation3dProductionLease {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P3 production ingress credits");
    semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::generation3d_admit_publication_authority(handle.operation, handle.generation, handle.generation.0, handle.generation.0, handle.generation.0, semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationCredits { maximum_items: 8_192, maximum_output_pages: semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_CONTROL_CREDITS })
    .expect("P3 production publication authority");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P3 production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P3 production envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("P3 production envelope seal"));
    Generation3dProductionLease { handle, released: false }
}

/// 🚿️ Drives ONE production envelope load to its terminal poll.
///
/// 🔎️ It does NOT terminate today, and the turn budget is not why: measured at 300 000 turns with
/// `std::thread::yield_now`, and again with `advance_typed_operation_publication().await` plus a
/// cooperative yield per turn, `poll_artifact_envelope_decode` reads `Pending` on every single turn —
/// so the job exists (a missing one reads `Fault`) and stays in
/// `ActiveArtifactEnvelopeDecodeState::Active`, and no maintenance turn ever reports `Blocked`, so
/// nothing on the ladder names an authority it is waiting for. The stall is inside the decode's own
/// `WorkerJobSession`, not in this driver (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️remaining-suite-reds-2026-09-13.md` §3.3).
fn drive_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..300_000 {
        semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::generation3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0)
            .expect("P3 authority refresh immediately before production maintenance");
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P3 production maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("P3 production load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("P3 production envelope load did not reach terminal, last decode poll {:?}", app.poll_artifact_envelope_decode(handle));
}

/// 🔐️ LAW: non-empty P3D3 canonical ingress reaches the real VCS maintenance replacement,
/// and accepted, stale, ABA, and displaced stores remain owned until explicit terminal ACK/close.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed() {
    // 🧹️ Registry-backed, never `VcsArtifactApp::new` — this app publishes
    // `bounded_first_step_tool_proofs!`, so a registryless instance faults at construction with
    // `interactive-job.catalog-authority` and its unwind aborts the binary.
    let _serial = crate::publication_authority::lock();
    let mut accepted = app_with_registry().await;
    let base_generation = accepted.artifact_generation_now();
    let (wire, expected, expected_digest) = production_envelope_wire("accepted-production-swap");
    let mut lease = admit_production_envelope(&mut accepted, &wire);
    let handle = lease.handle;
    assert_eq!(drive_production_envelope(&mut accepted, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let snapshot = context::snapshot(&accepted);
    assert_eq!(&snapshot, &expected, "real maintenance must publish all P3 snapshot and all-14 replay fields");
    assert_eq!(production_semantic_digest(&snapshot), expected_digest);
    assert!(snapshot.host_snapshot.layout.contains_key("move-target"));
    assert!(!snapshot.host_snapshot.layout.contains_key("clear-target"), "3D-only delete-widget-position must survive retained replay");
    assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P3 terminal ACK"));
    assert!(lease.release());
    drop(snapshot);
    expected.retire_cold();

    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
    for (hostile, expected_code) in [
        (Missing, "generation3d-publication.authority-missing"),
        (WrongOperation, "generation3d-publication.wrong-operation"),
        (WrongGeneration, "generation3d-publication.wrong-generation"),
        (WrongBase, "generation3d-publication.wrong-base"),
        (WrongParent, "generation3d-publication.wrong-parent"),
    ] {
        let mut app = app_with_registry().await;
        let last_valid = context::snapshot(&app);
        let last_valid_digest = production_semantic_digest(&last_valid);
        let base_generation = app.artifact_generation_now();
        let (wire, candidate, _) = production_envelope_wire("rejected-production-candidate");
        candidate.retire_cold();
        let mut lease = admit_production_envelope(&mut app, &wire);
        let handle = lease.handle;
        semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::generation3d_arm_publication_hostile(handle.operation, hostile);
        assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::binary::generation3d_take_publication_hostile_observed(handle.operation), Some(expected_code));
        assert_eq!(app.artifact_generation_now(), base_generation);
        let retained = context::snapshot(&app);
        assert_eq!(production_semantic_digest(&retained), last_valid_digest);
        assert_eq!(retained, last_valid);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P3 terminal ACK after candidate retirement"));
        assert!(lease.release());
        drop(retained);
        drop(last_valid);
    }
}

//#region 🔖️CommandSurface