use super::*;

//#region 🔮️ThirdPartyOracle
#[derive(Debug, PartialEq)]
struct Generation3dSemanticResult {
    widget_count: usize,
    synapse_count: usize,
    layout_count: usize,
    moved_id: String,
    x_bits: u64,
    y_bits: u64,
    synapse_id: String,
    from_port: String,
    to_port: String,
}

/// 🧩️ Owned test boundary shielding production and exported APIs from an oracle library.
trait Generation3dSemanticOracle {
    fn evaluate(&self, source: &[u8]) -> Result<Generation3dSemanticResult, String>;
}

struct SerdeJsonMoveOracle;

impl Generation3dSemanticOracle for SerdeJsonMoveOracle {
    fn evaluate(&self, source: &[u8]) -> Result<Generation3dSemanticResult, String> {
        let root: serde_json::Value = serde_json::from_slice(source).map_err(|error| error.to_string())?;
        let input = root.get("input").ok_or("oracle.input")?;
        let widgets = input.get("widgets").and_then(serde_json::Value::as_array).ok_or("oracle.widgets")?;
        let synapses = input.get("synapses").and_then(serde_json::Value::as_array).ok_or("oracle.synapses")?;
        let layout = input.get("layout").and_then(serde_json::Value::as_object).ok_or("oracle.layout")?;
        let mutation = root.get("mutation").ok_or("oracle.mutation")?;
        if mutation.get("kind").and_then(serde_json::Value::as_str) != Some("move-widget") {
            return Err("oracle.mutation-kind".into());
        }
        let moved_id = mutation.get("id").and_then(serde_json::Value::as_str).ok_or("oracle.moved-id")?;
        if !widgets.iter().any(|widget| widget.get("id").and_then(serde_json::Value::as_str) == Some(moved_id)) || !layout.contains_key(moved_id) {
            return Err("oracle.moved-owner".into());
        }
        let synapse = synapses.first().ok_or("oracle.synapse")?;
        let from = synapse.get("from").and_then(serde_json::Value::as_str).ok_or("oracle.synapse-from")?;
        let to = synapse.get("to").and_then(serde_json::Value::as_str).ok_or("oracle.synapse-to")?;
        if !widgets.iter().any(|widget| widget.get("id").and_then(serde_json::Value::as_str) == Some(from)) || !widgets.iter().any(|widget| widget.get("id").and_then(serde_json::Value::as_str) == Some(to)) {
            return Err("oracle.synapse-owner".into());
        }
        let position = mutation.get("layout").ok_or("oracle.mutation-layout")?;
        Ok(Generation3dSemanticResult {
            widget_count: widgets.len(),
            synapse_count: synapses.len(),
            layout_count: layout.len(),
            moved_id: moved_id.into(),
            x_bits: position.get("x").and_then(serde_json::Value::as_f64).ok_or("oracle.x")?.to_bits(),
            y_bits: position.get("y").and_then(serde_json::Value::as_f64).ok_or("oracle.y")?.to_bits(),
            synapse_id: synapse.get("id").and_then(serde_json::Value::as_str).ok_or("oracle.synapse-id")?.into(),
            from_port: synapse.get("fromPort").and_then(serde_json::Value::as_str).ok_or("oracle.from-port")?.into(),
            to_port: synapse.get("toPort").and_then(serde_json::Value::as_str).ok_or("oracle.to-port")?.into(),
        })
    }
}

fn semantic_result(snapshot: &Generation3dSnapshot, moved_id: &str) -> Generation3dSemanticResult {
    let position = snapshot.host_document.layout.get(moved_id).expect("P3 small-feature moved layout");
    let synapse = snapshot.host_document.synapses.first().expect("P3 small-feature synapse");
    Generation3dSemanticResult {
        widget_count: snapshot.host_document.widgets.len(),
        synapse_count: snapshot.host_document.synapses.len(),
        layout_count: snapshot.host_document.layout.len(),
        moved_id: moved_id.into(),
        x_bits: position.x.to_bits(),
        y_bits: position.y.to_bits(),
        synapse_id: synapse.id.clone(),
        from_port: synapse.from_port.clone(),
        to_port: synapse.to_port.clone(),
    }
}

fn semantic_digest(result: &Generation3dSemanticResult) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for bytes in [
        (result.widget_count as u64).to_be_bytes().to_vec(),
        (result.synapse_count as u64).to_be_bytes().to_vec(),
        (result.layout_count as u64).to_be_bytes().to_vec(),
        result.moved_id.as_bytes().to_vec(),
        result.x_bits.to_be_bytes().to_vec(),
        result.y_bits.to_be_bytes().to_vec(),
        result.synapse_id.as_bytes().to_vec(),
        result.from_port.as_bytes().to_vec(),
        result.to_port.as_bytes().to_vec(),
    ] {
        digest ^= bytes.len() as u64;
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        for byte in bytes {
            digest ^= u64::from(byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}

#[test]
fn small_move_widget_feature_matches_the_test_only_third_party_oracle() {
    let source = include_bytes!("../../../../../🧫️fixtures/🔬️p8yz-b-third-party-oracle-laws.json");
    let oracle = SerdeJsonMoveOracle.evaluate(source).expect("third-party P3 semantic oracle");
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.host_document.widgets = vec![
        semio_framework_artifact_flow_flow::Widget::Neuron { id: "source".into(), neuron_kind: "law".into(), params: Default::default(), input_ports: vec!["in".into()], output_ports: vec!["solid".into()], preview: true },
        semio_framework_artifact_flow_flow::Widget::OutputPreview { id: "preview".into(), preview: Default::default(), expanded: Default::default() },
    ];
    snapshot.host_document.synapses = vec![semio_framework_artifact_flow_flow::SynapseSpec { id: "source-preview".into(), from: "source".into(), from_port: "solid".into(), to: "preview".into(), to_port: String::new() }];
    snapshot.host_document.layout = [("source".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 }), ("preview".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 8.0, y: 3.0 })].into_iter().collect();
    generation3d_apply_retained_mutations_for_test(&mut snapshot, &[Generation3dMutation::MoveWidget(MoveWidget { id: "source".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 12.5, y: -8.25 } })]);
    let owned = semantic_result(&snapshot, "source");
    assert_eq!(owned, oracle, "owned P3 move result must equal the independent serde_json projection");
    assert_eq!(semantic_digest(&owned), semantic_digest(&oracle), "owned and oracle semantic digests must match exactly");
    snapshot.retire_cold();
}
//#endregion 🔮️ThirdPartyOracle

//#region ⏱️BoundedInitializer
/// 🔐️ The lease table is process-global and four slots deep — every initializer law holds this for
/// its whole body (`crate::publication_authority`).
fn initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Generation3dStoreInitializationAuthority {
    generation3d_admit_publication_authority(operation, generation, generation.0, generation.0, generation.0, crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationCredits { maximum_items: GENERATION3D_MAXIMUM_DOMAIN_ITEMS, maximum_output_pages: GENERATION3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: GENERATION3D_MOUNTED_CONTROL_CREDITS })
        .expect("P3 initializer law publication authority");
    Generation3dStoreInitializationAuthority::new(store::create_document_envelope(crate::GENERATION_3D_SCHEMA, "generation3d-bounded-initializer", Generation3dSnapshot::default(), None), operation, generation)
}

fn close_initializer(authority: &mut Generation3dStoreInitializationAuthority) {
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;
    for _ in 0..100_000 {
        if matches!(authority.close_step(1, GENERATION3D_OWNER_BYTES).expect("P3 initializer bounded close"), semio_framework_plugin::PluginCloseStep::Complete) {
            assert!(authority.terminal_is_empty());
            return;
        }
    }
    panic!("P3 initializer did not reach terminal-empty close");
}

#[test]
fn insufficient_fuel_and_expired_deadline_yield_before_initializer_progress() {
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;
    let _serial = crate::publication_authority::lock();
    let operation = semio_framework_job::OperationId(u64::MAX - 301);
    let generation = semio_framework_job::Generation(301);
    let mut authority = initializer(operation, generation);
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    let mut zero_fuel = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(0, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut sequence);
    assert!(matches!(authority.step(&mut zero_fuel), semio_framework_job::StepOutcome::Yield));
    assert!(matches!(authority.phase, Generation3dStoreInitializationPhase::ValidateEnvelope));
    let mut expired = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, 0), cancel, semio_framework_job::default_now_us, &mut sequence);
    assert!(matches!(authority.step(&mut expired), semio_framework_job::StepOutcome::Yield));
    assert!(matches!(authority.phase, Generation3dStoreInitializationPhase::ValidateEnvelope));
    close_initializer(&mut authority);
    assert!(generation3d_release_publication_authority(operation, generation));
}

#[test]
fn cancelled_and_stale_aba_initializers_retire_to_terminal_empty() {
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;
    let _serial = crate::publication_authority::lock();
    let cancelled_operation = semio_framework_job::OperationId(u64::MAX - 302);
    let cancelled_generation = semio_framework_job::Generation(302);
    let mut cancelled = initializer(cancelled_operation, cancelled_generation);
    cancelled.request_cancel();
    let mut cancelled_sequence = 0;
    let cancelled_token = semio_framework_job::CancelToken::root_now();
    let mut cancelled_outcome = None;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(cancelled_operation, cancelled_generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancelled_token.clone(), semio_framework_job::default_now_us, &mut cancelled_sequence);
        let outcome = cancelled.step(&mut context);
        if !matches!(outcome, semio_framework_job::StepOutcome::Yield) {
            cancelled_outcome = Some(outcome);
            break;
        }
    }
    let mut cancelled_outcome = cancelled_outcome.expect("cancelled P3 initializer must terminate within its bounded owner budget");
    assert!(matches!(cancelled_outcome, semio_framework_job::StepOutcome::Cancelled));
    close_outcome(&mut cancelled_outcome);
    assert!(cancelled.terminal_is_empty());
    assert!(generation3d_release_publication_authority(cancelled_operation, cancelled_generation));

    let stale_operation = semio_framework_job::OperationId(u64::MAX - 303);
    let stale_generation = semio_framework_job::Generation(303);
    let mut stale = initializer(stale_operation, stale_generation);
    let mut stale_sequence = 0;
    let stale_token = semio_framework_job::CancelToken::root_now();
    let mut stale_outcome = None;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(
            stale_operation,
            semio_framework_job::Generation(stale_generation.0 + 1),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            stale_token.clone(),
            semio_framework_job::default_now_us,
            &mut stale_sequence,
        );
        let outcome = stale.step(&mut context);
        if !matches!(outcome, semio_framework_job::StepOutcome::Yield) {
            stale_outcome = Some(outcome);
            break;
        }
    }
    let mut stale_outcome = stale_outcome.expect("stale P3 initializer must terminate within its bounded owner budget");
    assert!(matches!(stale_outcome, semio_framework_job::StepOutcome::Fault(_)));
    close_outcome(&mut stale_outcome);
    assert!(stale.terminal_is_empty());
    assert!(generation3d_release_publication_authority(stale_operation, stale_generation));
}
/// 🧹️ Drains a terminal [`semio_framework_job::StepOutcome`]'s retained payload pages. A
/// `Fault`/`PreviewReady`/`Complete` outcome carries a `RetainedJobPayload` whose `Drop` deliberately
/// preserves its page backing, so an owner that merely inspects the discriminant and lets the value
/// fall out of scope aborts the test process
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn close_outcome(outcome: &mut semio_framework_job::StepOutcome) {
    for _ in 0..GENERATION3D_MAXIMUM_DOMAIN_ITEMS {
        if matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {
            return;
        }
    }
    panic!("P3 terminal step outcome did not close");
}

//#endregion ⏱️BoundedInitializer

fn close_session(session: &mut Generation3dMutationSession) {
    let admitted = session.retained_allocated_bytes();
    let mut released = 0;
    let mut refused_subexact = false;
    for _ in 0..GENERATION3D_MAXIMUM_DOMAIN_ITEMS {
        let maximum_bytes = session.next_retained_release_allocation_bytes().unwrap_or(0);
        if maximum_bytes > 0 && !refused_subexact {
            let before = session.retained_allocated_bytes();
            assert_eq!(
                session.close_step(1, maximum_bytes - 1).expect("P3 subexact retained mutation close"),
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 }
            );
            assert_eq!(session.retained_allocated_bytes(), before);
            refused_subexact = true;
        }
        match session.close_step(1, maximum_bytes).expect("P3 retained mutation close") {
            store::mounted_pack_rt::RetainedPackCloseStep::Complete => {
                assert!(session.terminal_is_empty());
                assert_eq!(released, admitted);
                assert!(refused_subexact || admitted == 0);
                return;
            }
            store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_bytes, .. } => released += released_bytes,
        }
    }
    panic!("P3 retained mutation session did not close");
}

#[test]
fn every_fourteen_variant_decodes_through_retained_structural_grants() {
    let mutations = generation3d_all_retained_mutation_fixtures_for_test();
    assert_eq!(mutations.len(), GENERATION3D_MUTATION_VARIANT_COUNT);
    for mutation in mutations {
        let bytes = encode_op(&mutation).expect("P3 retained mutation fixture encode");
        let mut session = Generation3dMutationSession::new(bytes.len(), GENERATION3D_MAXIMUM_DOMAIN_ITEMS).expect("P3 retained mutation preflight");
        let mut refused_subexact = false;
        for byte in bytes {
            assert!(session.ingress_ready());
            session.admit_byte(byte).expect("one retained mutation byte");
            for _ in 0..GENERATION3D_OWNER_BYTES {
                if let Some(exact) = session.next_retained_allocation_bytes().expect("P3 retained mutation allocation query") {
                    if exact > 0 && !refused_subexact {
                        let before = session.retained_allocated_bytes();
                        assert_eq!(session.reserve_retained_allocation(exact - 1).expect("P3 subexact retained mutation allocation"), (false, 0));
                        assert_eq!(session.retained_allocated_bytes(), before);
                        refused_subexact = true;
                    }
                    let (progressed, _) = session.reserve_retained_allocation(exact).expect("P3 retained mutation allocation");
                    assert!(progressed);
                } else {
                    session.grant().expect("one retained mutation ingress grant");
                }
                if session.ingress_ready() {
                    break;
                }
            }
            assert!(session.ingress_ready(), "symbol expansion must hand input ownership back before the next byte");
        }
        assert!(refused_subexact, "every retained mutation must pre-admit real record-body backing");
        session.seal().expect("exact retained mutation seal");
        let mut ready = false;
        for _ in 0..100_000 {
            if let Some(exact) = session.next_retained_allocation_bytes().expect("P3 retained mutation allocation query") {
                let (progressed, _) = session.reserve_retained_allocation(exact).expect("P3 retained mutation allocation");
                assert!(progressed);
            } else if session.grant().expect("one retained semantic grant") {
                ready = true;
                break;
            }
        }
        assert!(ready, "retained P3 mutation owner must converge");
        let decoded = session.take().expect("typed P3 mutation handoff");
        assert_eq!(decoded, mutation, "the retained ingress route must recover the exact mutation it was handed");
        decoded.retire_cold();
        mutation.retire_cold();
        close_session(&mut session);
    }
}

#[test]
fn deterministic_all_field_ledger_includes_the_3d_only_variant() {
    let mutations = generation3d_all_retained_mutation_fixtures_for_test();
    let mut left = store::ArtifactStoreInitializationDigest::new(b"generation3d.all14");
    let mut right = store::ArtifactStoreInitializationDigest::new(b"generation3d.all14");
    for mutation in &mutations {
        generation3d_observe_mutation(&mut left, mutation);
        generation3d_observe_mutation(&mut right, mutation);
    }
    assert_eq!(left.finish(), right.finish());
    assert!(mutations.iter().any(|mutation| matches!(mutation, Generation3dMutation::DeleteWidgetPosition(_))));
    for mutation in mutations {
        mutation.retire_cold();
    }
}

//#region 🧹️FlowFrontierOwnership
/// 🚪️ The EXACT driver every framework close ladder is: one item, one 4 KiB page, and NO channel to
/// ask the owner for a bigger grant. `SnapshotReadReturnPump::drive`
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`) turns a `Blocked` answer into
/// `PluginCloseStep::Blocked { reason: "returned snapshot-read disposer is waiting on external
/// ownership" }`, and `close_registered_fixture_app` then yields and asks again with the same grant
/// until its deadline — so for a retirement this app owns, `Blocked` is not backpressure, it is a
/// permanent stall (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn drive_under_the_frameworks_fixed_page_grant(retirement: &mut dyn ErasedSnapshotRetirement, owner: &str) -> usize {
    let mut released = 0usize;
    for _ in 0..GENERATION3D_MAXIMUM_DOMAIN_ITEMS {
        match retirement.close_step(1, GENERATION3D_OWNER_BYTES).unwrap_or_else(|reason| panic!("{owner} retirement faulted: {reason}")) {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty(), "{owner} reported Complete without its exact terminal-empty witness");
                return released;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1, "{owner} released {released_items} items under a one-item grant");
                assert!(released_bytes <= GENERATION3D_OWNER_BYTES, "{owner} released {released_bytes} bytes under a {GENERATION3D_OWNER_BYTES}-byte grant");
                released += released_bytes;
            }
            store::SnapshotRetirementStep::Blocked => panic!(
                "{owner} answered Blocked under the framework's exact one-page close grant — nothing external owns this value, so paying the Flow frontier's own reserve-then-close demand is this retirement's business and the app close ladder spins here forever"
            ),
        }
    }
    panic!("{owner} did not reach its terminal-empty close witness")
}

/// 🧊️ Every widget, synapse, layout row and generation this app's document owns is retired through
/// a `semio_framework_artifact_flow_flow::retained::FlowRetirement`, which is a RESERVE-then-CLOSE
/// frontier: `close_step` answers `Blocked` — never an error — while `next_allocation_bytes` still
/// names a page the current owner's decomposition needs. Both of this codec's retirements therefore
/// have to pay that reservation themselves, because the erased `ErasedSnapshotRetirement` contract
/// they are driven through has no demand channel at all.
///
/// The oracle is the framework's OWN generic route to the same value — the `Arc` retirement
/// `store::SnapshotRetirementFactory` hands the document store — which must release byte-for-byte
/// what the owned route releases.
#[test]
fn every_document_retirement_pays_its_own_flow_frontier_under_the_fixed_page_grant() {
    let mut owned = generation3d_retire_owned_snapshot(Generation3dSnapshot::default());
    let owned_bytes = drive_under_the_frameworks_fixed_page_grant(owned.as_mut(), "owned document snapshot");
    let mut aliased = store::SnapshotRetirementFactory::retire(&Generation3dRetainedSnapshotRetirementFactory, std::sync::Arc::new(Generation3dSnapshot::default()));
    let aliased_bytes = drive_under_the_frameworks_fixed_page_grant(aliased.as_mut(), "aliased document snapshot");
    assert_eq!(owned_bytes, aliased_bytes, "the owned and Arc retirement routes must release the same exact document backing");
    assert!(owned_bytes > 0, "a populated document fixture owns real backing");
}

/// 🔁️ The same law for the replay displacement route: every `generation3d_apply_initialization_mutation`
/// that displaces a widget, a synapse or a string hands the store a `Generation3dReplayRetirement`,
/// which the store's displaced-retirement ladder drives under the identical fixed page grant.
#[test]
fn every_displaced_replay_owner_pays_its_own_flow_frontier_under_the_fixed_page_grant() {
    let mut snapshot = Generation3dSnapshot::default();
    let mutations = generation3d_all_retained_mutation_fixtures_for_test();
    let mut displaced = 0usize;
    for mutation in &mutations {
        let Ok(Some(mut retirement)) = generation3d_apply_initialization_mutation(&mut snapshot, mutation) else { continue };
        drive_under_the_frameworks_fixed_page_grant(retirement.as_mut(), "displaced replay owner");
        displaced += 1;
    }
    for mutation in mutations {
        mutation.retire_cold();
    }
    assert!(displaced > 0, "the retained mutation fixtures must displace at least one owner");
    drive_under_the_frameworks_fixed_page_grant(generation3d_retire_owned_snapshot(snapshot).as_mut(), "replayed document snapshot");
}
//#endregion 🧹️FlowFrontierOwnership
