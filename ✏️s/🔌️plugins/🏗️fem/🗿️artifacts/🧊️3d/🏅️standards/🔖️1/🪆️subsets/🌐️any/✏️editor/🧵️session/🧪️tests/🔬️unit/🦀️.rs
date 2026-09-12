use super::*;

#[test]
fn fem3d_numerical_child_retains_every_outcome_until_bounded_retirement() {
    use semio_framework_job::{Checkpoint, CommitCandidate, JobFault, JobPayloadStream, JOB_PAYLOAD_PAGE_BYTES};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🪟️window-config-contract/🧫️fixtures/🧒️child-outcomes/🔣️.json")).expect("neutral child outcomes");
    let operation = semio_framework_job::Operation::new(OperationId(991), RevisionId(997), Generation(1009), 0);
    let payload = |stream| {
        let mut sequence = 0;
        StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence)
            .payload_from_bytes(stream, b"retained-child-state").expect("admitted child payload")
    };
    for row in corpus["cases"].as_array().expect("cases") {
        let kind = row["kind"].as_str().expect("kind");
        let outcome = match kind {
            "preview" => StepOutcome::PreviewReady(payload(JobPayloadStream::Preview)),
            "checkpoint" => StepOutcome::CheckpointReady(Checkpoint { applied_progress: 1, state: payload(JobPayloadStream::CheckpointState) }),
            "complete" => StepOutcome::Complete(CommitCandidate { state: payload(JobPayloadStream::CommitState), output: payload(JobPayloadStream::CommitOutput) }),
            "fault" => StepOutcome::Fault(JobFault { detail: payload(JobPayloadStream::Fault) }),
            _ => unreachable!(),
        };
        let mut child = Box::new(Fem3dNumericalChild::new());
        let result = child.observe_child_outcome(outcome);
        assert_eq!(result.is_err(), kind == "fault");
        if let Ok(complete) = result { assert_eq!(complete, kind == "complete"); }
        let pointer = child.child_outcome.as_ref().expect("exact retained outcome") as *const StepOutcome;
        let doc = Fem3dSnapshot::default();
        let mut solver = Fem3dSolverView::new(freshness(19), 0);
        let mut backing = Fem3dBackingCredit::new();
        let mut sequence = 0;
        for budget in [StepBudget::new(0, u64::MAX), StepBudget::new(1, 0)] {
            let mut context = StepContext::new(operation.operation, operation.generation, budget, semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            assert_eq!(child.step(&doc, &mut solver, &mut backing, freshness(19), operation, &mut context), Ok(false));
            assert_eq!(child.child_outcome.as_ref().expect("yield retains outcome") as *const StepOutcome, pointer);
            assert!(!child.child_outcome.as_ref().expect("payload still live").terminal_is_empty());
        }
        let cancelled = semio_framework_job::root_cancel_token();
        cancelled.cancel_now();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), cancelled, || Some(0), &mut sequence);
        assert!(child.step(&doc, &mut solver, &mut backing, freshness(19), operation, &mut context).is_err());
        assert_eq!(child.child_outcome.as_ref().expect("cancellation retains outcome") as *const StepOutcome, pointer);
        assert_eq!(child.close_child_outcome(0), Some((false, 0, 0)));
        assert_eq!(child.close_child_outcome(JOB_PAYLOAD_PAGE_BYTES - 1), Some((false, 0, 0)));
        assert_eq!(child.child_outcome.as_ref().expect("retained without grant") as *const StepOutcome, pointer);
        let mut released = 0;
        for _ in 0..8 {
            match child.close_child_outcome(JOB_PAYLOAD_PAGE_BYTES) {
                Some((false, items, bytes)) => {
                    assert!(items <= 1);
                    assert!(bytes <= JOB_PAYLOAD_PAGE_BYTES);
                    released += bytes / JOB_PAYLOAD_PAGE_BYTES;
                }
                None => break,
                other => panic!("unexpected child close {other:?}"),
            }
        }
        assert_eq!(released, row["pages"].as_u64().expect("pages") as usize);
        assert!(child.child_outcome.is_none());
        let mut complete = false;
        for _ in 0..256 {
            if child.close_step(JOB_PAYLOAD_PAGE_BYTES).0 { complete = true; break; }
        }
        assert!(complete, "all numerical owners close");
    }
    eprintln!("[DEBUG] FEM3D retained preview, checkpoint, commit state/output and fault pages through exact bounded close");
}

fn freshness(generation: u64) -> Fem3dVisualFreshness {
    Fem3dVisualFreshness { app_instance_id: 7, model_revision: 11, document_generation: generation, operation: 13, numerical_preview_sequence: 17, surface_generation: generation, renderer_scene_generation: generation }
}

fn credit(doc: &Fem3dSnapshot) -> Fem3dPageCredit {
    let mut preflight = SnapshotPreflight::new();
    let mut turns = 0;
    while !preflight.step_one(doc).expect("bounded preflight") && turns < 1_024 {
        turns += 1;
    }
    preflight.credit().expect("exact credit")
}

fn solver(doc: &Fem3dSnapshot, scalar: Fem3dSolverScalar) -> Fem3dSolverView {
    let mut solver = Fem3dSolverView::new(freshness(19), doc.nodes.len());
    let mut backing = Fem3dBackingCredit::new();
    for page in 0..FEM3D_SOLVER_PAGE_COUNT {
        assert!(solver.admit_page(page, false, &mut backing));
        assert!(solver.admit_page(page, true, &mut backing));
    }
    for index in 0..doc.nodes.len() {
        solver.publish_scalar(freshness(19), index, scalar).expect("generation-qualified scalar");
    }
    assert!(solver.publish_progress(freshness(19), Fem3dVisualState::ValidatedFinal, 0.125, 1e-8, doc.nodes.len(), doc.nodes.len()));
    solver
}

fn build(doc: &Fem3dSnapshot, solver: &Fem3dSolverView) -> Fem3dPageVisualLease {
    let mut job = Fem3dPageVisualJob::new(freshness(19), credit(doc));
    let mut backing = Fem3dBackingCredit::new();
    let mut turns = 0;
    while !job.step_one(doc, solver, &mut backing, freshness(19)).expect("one production opportunity") && turns < 4_096 {
        turns += 1;
    }
    assert!(turns < 4_096);
    job.take_complete().expect("sealed page lease")
}

fn close(lease: &mut Fem3dPageVisualLease) -> usize {
    let mut released_pages = 0;
    let mut turns = 0;
    loop {
        let (terminal, _, bytes) = lease.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        released_pages += usize::from(bytes == WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        turns += 1;
        if terminal {
            break;
        }
        assert!(turns < 128);
    }
    released_pages
}

#[test]
fn fem3d_visual_maximum_plus_one_rejects_before_owner_transfer() {
    let mut doc = Fem3dSnapshot::default();
    doc.nodes.resize_with(MAXIMUM_NODES + 1, || crate::FemNode { id: "n".into(), x: 0.0, y: 0.0, z: 0.0 });
    let producer = doc.nodes.as_ptr();
    let credit = Fem3dPageCredit { item_count: 3, byte_count: 7, draw_count: 0, draw_bytes: 3 };
    let mut job = Fem3dPageVisualJob::new(freshness(19), credit);
    let solver = Fem3dSolverView::new(freshness(19), 0);
    let mut backing = Fem3dBackingCredit::new();
    assert!(job.step_one(&doc, &solver, &mut backing, freshness(19)).is_err());
    assert_eq!(producer, doc.nodes.as_ptr());
    assert_eq!(job.stage(), Fem3dVisualJobStage::ReserveSnapshot);
}

#[test]
fn fem3d_snapshot_preflight_page_maximum_plus_one_returns_exact_producer() {
    let mut doc = Fem3dSnapshot::default();
    doc.nodes.push(crate::FemNode { id: "n".repeat(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY + 1), x: 0.0, y: 0.0, z: 0.0 });
    let producer = doc.nodes.as_ptr();
    let mut preflight = SnapshotPreflight::new();
    let before = preflight;
    assert_eq!(preflight.step_one(&doc), Err(()));
    assert_eq!(producer, doc.nodes.as_ptr());
    assert_eq!(preflight.item_count, before.item_count);
    assert_eq!(preflight.byte_count, before.byte_count);
}

#[test]
fn fem3d_solver_nonzero_generation_corresponds_to_every_published_field_page() {
    let mut doc = Fem3dSnapshot::default();
    doc.nodes.push(crate::FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 });
    let scalar = Fem3dSolverScalar { displacement: [1.0, 2.0, 3.0], residual: [4.0, 5.0, 6.0], reaction: [7.0, 8.0, 9.0], contour: 13.0, mode_shape: [10.0, 11.0, 12.0], eigen_estimate: 17.0 };
    let solver = solver(&doc, scalar);
    assert_eq!(solver.scalar(0), Some(scalar));
    let mut lease = build(&doc, &solver);
    let displacement = world3d_snapshot_with_page(lease.snapshot(), 10, |page| page.item(0).expect("displacement").numbers).expect("displacement page");
    let residual = world3d_snapshot_with_page(lease.snapshot(), 12, |page| page.item(0).expect("residual").numbers).expect("residual page");
    let reaction = world3d_snapshot_with_page(lease.snapshot(), 14, |page| page.item(0).expect("reaction").numbers).expect("reaction page");
    let contour = world3d_snapshot_with_page(lease.snapshot(), 16, |page| page.item(0).expect("contour").numbers).expect("contour page");
    let mode = world3d_snapshot_with_page(lease.snapshot(), 18, |page| page.item(0).expect("mode").numbers).expect("mode page");
    assert_eq!(&displacement[..3], &scalar.displacement);
    assert_eq!(&residual[..3], &scalar.residual);
    assert_eq!(&reaction[..3], &scalar.reaction);
    assert_eq!(contour[3], scalar.contour);
    assert_eq!(&mode[..3], &scalar.mode_shape);
    assert_eq!(mode[3], scalar.eigen_estimate);
    assert_eq!(close(&mut lease), FEM3D_VISUAL_PAGES);
}

#[test]
fn fem3d_visual_stale_cancel_fault_deadline_and_interrupted_device_close_preserve_last_valid() {
    let doc = Fem3dSnapshot::default();
    let solver = solver(&doc, Fem3dSolverScalar::default());
    let mut current = build(&doc, &solver);
    let current_snapshot = current.snapshot();
    let mut stale = Fem3dPageVisualJob::new(freshness(19), credit(&doc));
    let mut backing = Fem3dBackingCredit::new();
    let mut turns = 0;
    while stale.stage() != Fem3dVisualJobStage::ValidateFreshness && turns < 1_024 {
        stale.step_one(&doc, &solver, &mut backing, freshness(19)).expect("bounded step");
        turns += 1;
    }
    assert!(stale.step_one(&doc, &solver, &mut backing, freshness(23)).is_err());
    assert!(stale.take_complete().is_none());
    assert_eq!(current.snapshot(), current_snapshot);
    assert_eq!(stale.close_step(0), (false, 0, 0));
    while !stale.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).0 && turns < 2_048 {
        turns += 1;
    }
    assert!(stale.terminal_is_empty());
    assert_eq!(close(&mut current), FEM3D_VISUAL_PAGES);
}

#[test]
fn fem3d_visual_replay_accessibility_fixed_page_close_and_each_step_are_bounded() {
    let doc = Fem3dSnapshot::default();
    let solver = solver(&doc, Fem3dSolverScalar::default());
    let mut first = build(&doc, &solver);
    let mut second = build(&doc, &solver);
    let labels = |lease: &Fem3dPageVisualLease| {
        world3d_snapshot_with_page(lease.snapshot(), 20, |page| {
            let en = page.item(0).and_then(|item| item.strings[1]).and_then(|span| page.string(span)).map(str::to_owned);
            let de = page.item(1).and_then(|item| item.strings[1]).and_then(|span| page.string(span)).map(str::to_owned);
            (en, de)
        })
        .expect("label page")
    };
    let first_labels = labels(&first);
    let second_labels = labels(&second);
    assert_eq!(first_labels, second_labels);
    assert_eq!(first_labels.0.as_deref(), Some(FEM3D_VISUAL_LABEL_EN));
    assert_eq!(first_labels.1.as_deref(), Some(FEM3D_VISUAL_LABEL_DE));
    assert_eq!(close(&mut first), FEM3D_VISUAL_PAGES);
    assert_eq!(close(&mut second), FEM3D_VISUAL_PAGES);

    let mut job = Fem3dPageVisualJob::new(freshness(19), credit(&doc));
    let mut backing = Fem3dBackingCredit::new();
    let started = std::time::Instant::now();
    let _ = job.step_one(&doc, &solver, &mut backing, freshness(19));
    assert!(started.elapsed().as_micros() < 8_000);
    while !job.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).0 {}
    assert!(job.terminal_is_empty());
}

#[test]
fn fem3d_numerical_fixed_owner_maximum_plus_one_refuses_unchanged_and_closes_one_slot() {
    let started = std::time::Instant::now();
    let mut ids = FixedSlots::<String, 1>::new();
    assert_eq!(ids.admit_one(2), Err(()));
    assert_eq!(ids.admitted, 0);
    assert_eq!(ids.admit_one(1), Ok(false));
    assert_eq!(ids.admit_one(1), Ok(true));
    assert_eq!(ids.push("kept".into()), Ok(()));
    let rejected = "returned".to_owned();
    let rejected_pointer = rejected.as_ptr();
    let returned = ids.push(rejected).expect_err("full fixed owner returns producer");
    assert_eq!(returned.as_ptr(), rejected_pointer);
    assert_eq!(ids.len(), 1);
    assert_eq!(ids.pop().as_deref(), Some("kept"));
    assert!(!ids.close_admission_one());
    assert!(ids.close_admission_one());

    let mut model = MountedAnalysisModel::new();
    assert_eq!(model.admit_node_one(MAXIMUM_FIELDS + 1), Err(crate::analyses::MountedAnalysisCapacityExceeded { requested: MAXIMUM_FIELDS + 1, maximum: crate::analyses::MOUNTED_ANALYSIS_NODE_SLOTS }));
    assert_eq!(model.nodes_len(), 0);
    let node = Node { id: "returned-node".into(), pos: [0.0; 3] };
    let node_pointer = node.id.as_ptr();
    let returned = model.push_node(node).expect_err("unadmitted node returns producer");
    assert_eq!(returned.id.as_ptr(), node_pointer);

    let mut domain = MountedPlanarDomain::new();
    assert_eq!(domain.admit_outer_one(MAXIMUM_FIELDS + 1), Err(crate::mesh::MountedDomainFault::PointCapacity { requested: MAXIMUM_FIELDS + 1, maximum: crate::mesh::MOUNTED_DOMAIN_POINT_SLOTS }));
    assert_eq!(domain.push_outer([1.0, 2.0]), Err([1.0, 2.0]));

    let mut scalars = MountedScalarSlots::new();
    assert_eq!(scalars.admit_one(MAXIMUM_FIELDS * 6 + 1), Err(crate::sparse::MountedScalarFault::Capacity { requested: MAXIMUM_FIELDS * 6 + 1, maximum: crate::sparse::MOUNTED_SCALAR_SLOTS }));
    assert_eq!(scalars.len(), 0);
    assert_eq!(scalars.push(17.0), Err(17.0));
    assert!(started.elapsed().as_micros() < 8_000);
}

#[test]
fn fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized() {
    use crate::{FemDof, FemLoadCase, FemMaterial, FemNode, FemSolid, FemSupport};

    let doc = Fem3dSnapshot {
        nodes: vec![FemNode { id: "n0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n1".into(), x: 1.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 1.0, y: 1.0, z: 0.0 }, FemNode { id: "n3".into(), x: 0.0, y: 1.0, z: 0.0 }],
        materials: vec![FemMaterial { id: "m".into(), name: "M".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
        solids: vec![FemSolid { id: "s".into(), name: "S".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.25, layers: 1, mesh_size: 2.0, material_id: "m".into() }],
        supports: (0..4).map(|index| FemSupport { id: format!("f{index}"), node_id: format!("n{index}"), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] }).collect(),
        load_cases: vec![FemLoadCase { id: "g".into(), name: "G".into(), loads: vec![], self_weight: true }],
        ..Default::default()
    };
    let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
    let mut child = Fem3dNumericalChild::new();
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview = 0;
    let mut terminal = false;
    let mut last_stage = "initial";
    for _ in 0..200_000 {
        let deadline = semio_framework_job::default_now_us().unwrap().checked_add(8_000).unwrap();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, deadline), cancel.clone(), semio_framework_job::default_now_us, &mut preview);
        let before = (child.stage, child.node_cursor, child.scalar_axis, child.reaction_entry, child.child_outcome.is_some());
        let started = std::time::Instant::now();
        terminal = child.step(&doc, &mut fields, &mut backing, freshness(19), operation, &mut context).unwrap_or_else(|fault| {
            panic!("[DEBUG] production numerical child at {:?}/{}: {}", child.stage, context.stage(), String::from_utf8_lossy(&fault))
        });
        let elapsed_us = started.elapsed().as_micros();
        let after = (child.stage, child.node_cursor, child.scalar_axis, child.reaction_entry, child.child_outcome.is_some());
        assert_eq!(context.fuel_remaining(), 0, "[DEBUG] numerical before={before:?}, after={after:?}, context {}, terminal={terminal}, expired={}, elapsed_us={elapsed_us}", context.stage(), context.deadline_exceeded());
        last_stage = context.stage();
        assert!(elapsed_us < 8_000, "[DEBUG] numerical before={before:?}, after={after:?}, context {}, terminal={terminal}, expired={}, elapsed_us={elapsed_us}", context.stage(), context.deadline_exceeded());
        if terminal {
            break;
        }
    }
    assert!(terminal, "[DEBUG] numerical stopped at {:?}/{last_stage}, PCG={:?}, subspace={:?}, pending outcome={}", child.stage, child.pcg.as_ref().map(PcgJob::visual_progress), child.subspace.as_ref().map(SubspaceIterationJob::visual_progress), child.child_outcome.is_some());
    assert!(fields.ready());
    assert!((0..fields.len).filter_map(|index| fields.scalar(index)).any(|scalar| scalar.displacement != [0.0; 3] || scalar.reaction != [0.0; 3]));
    assert!((0..fields.len).filter_map(|index| fields.scalar(index)).any(|scalar| scalar.eigen_estimate > 0.0 && scalar.mode_shape != [0.0; 3]));
    assert_eq!(child.close_step(0), (false, 0, 0));
    for _ in 0..200_000 {
        if child.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).0 {
            return;
        }
    }
    panic!("numerical child close did not reach exact terminal");
}

#[test]
fn fem3d_production_field_correspondence_rejects_sparse_and_zero_aliases() {
    let scalar = Fem3dSolverScalar { displacement: [1.0, -2.0, 3.0], residual: [4.0, -5.0, 6.0], reaction: [7.0, -8.0, 9.0], contour: 3.0, mode_shape: [0.25, -0.5, 0.75], eigen_estimate: 17.0 };
    let mut fields = Fem3dSolverView::new(freshness(19), 2);
    let mut backing = Fem3dBackingCredit::new();
    for page in 0..FEM3D_SOLVER_PAGE_COUNT {
        assert!(fields.admit_page(page, false, &mut backing));
        assert!(fields.admit_page(page, true, &mut backing));
    }
    assert!(fields.publish_scalar(freshness(19), 1, scalar).is_ok());
    assert!(!fields.publish_progress(freshness(19), Fem3dVisualState::ValidatedFinal, 0.0, 1e-8, 2, 2));
    assert!(!fields.ready());
    assert!(fields.publish_scalar(freshness(19), 0, scalar).is_ok());
    assert!(fields.publish_progress(freshness(19), Fem3dVisualState::ValidatedFinal, 0.0, 1e-8, 2, 2));
    assert!(fields.ready());
    assert_ne!(fields.scalar(0).expect("field").reaction, fields.scalar(0).expect("field").residual);
    assert_ne!(fields.scalar(0).expect("field").mode_shape, fields.scalar(0).expect("field").displacement);
    assert!(fields.publish_scalar(freshness(23), 0, scalar).is_err());
    assert!(!fields.set_len(freshness(19), MAXIMUM_FIELDS + 1));
}

#[test]
fn fem3d_process_permits_precede_solver_order_allocation_and_drop_handoff_closes_one_backing() {
    let mut backing = Fem3dBackingCredit::new();
    let region = FixedOrder::<MAXIMUM_REGIONS>::new(&mut backing).unwrap();
    let region_pointer = region.slots.as_ptr();
    let element = FixedOrder::<MAXIMUM_ELEMENTS>::new(&mut backing).unwrap();
    let element_pointer = element.slots.as_ptr();
    let before_refusal = (backing.live_items, backing.live_bytes);
    assert!(!backing.claim(FEM3D_PROCESS_BACKING_BYTES));
    assert_eq!((backing.live_items, backing.live_bytes), before_refusal);
    assert_eq!(region_pointer, region.slots.as_ptr());
    assert_eq!(element_pointer, element.slots.as_ptr());
    drop(region);
    assert!(backing.release(1, FEM3D_REGION_ORDER_BYTES));
    drop(element);
    assert!(backing.release(1, FEM3D_ELEMENT_ORDER_BYTES));
    assert!(backing.terminal_is_empty());

    let mut solver = Fem3dSolverView::new(freshness(31), 1);
    assert!(solver.admit_page(0, false, &mut backing));
    assert!(solver.admit_page(0, true, &mut backing));
    let scalar_pointer = solver.scalars[0].as_deref().unwrap().as_ptr();
    let initialized_pointer = solver.initialized[0].as_deref().unwrap().as_ptr();
    drop(solver);
    assert_eq!(close_recovered_fem3d_backing(0), Some((0, 0)));
    assert_eq!(close_recovered_fem3d_backing(FEM3D_SOLVER_SCALAR_PAGE_BYTES.max(FEM3D_SOLVER_INITIALIZED_PAGE_BYTES)), Some((1, FEM3D_SOLVER_SCALAR_PAGE_BYTES)));
    assert_eq!(close_recovered_fem3d_backing(FEM3D_SOLVER_INITIALIZED_PAGE_BYTES), Some((1, FEM3D_SOLVER_INITIALIZED_PAGE_BYTES)));
    assert!(!scalar_pointer.is_null());
    assert!(!initialized_pointer.is_null());

    let doc = Fem3dSnapshot::default();
    let solver = Fem3dSolverView::new(freshness(19), 0);
    let mut candidate = Fem3dPageVisualJob::new(freshness(19), credit(&doc));
    assert!(!candidate.step_one(&doc, &solver, &mut backing, freshness(19)).unwrap());
    assert!(!candidate.step_one(&doc, &solver, &mut backing, freshness(19)).unwrap());
    assert!(!candidate.step_one(&doc, &solver, &mut backing, freshness(19)).unwrap());
    drop(candidate);
    assert_eq!(close_recovered_fem3d_backing(FEM3D_REGION_ORDER_BYTES.max(FEM3D_ELEMENT_ORDER_BYTES)), Some((1, FEM3D_REGION_ORDER_BYTES)));
    assert_eq!(close_recovered_fem3d_backing(FEM3D_ELEMENT_ORDER_BYTES), Some((1, FEM3D_ELEMENT_ORDER_BYTES)));
    assert_eq!(world3d_snapshot_recovery_close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY), Some((1, 0)));
}

fn recovery_identity(app_instance_id: u32, generation: u64) -> Identity {
    Identity { app_instance_id, base_revision: RevisionId(11), generation: Generation(generation), canonical_base_revision: [generation as u8; 32], operation: OperationId(JOB_TAG | generation), job: JOB_TAG | generation }
}

fn recoverable_state(identity: Identity, recovery: Rc<MountedRecoverySlot>) -> MountedState {
    MountedState {
        identity,
        snapshot: None,
        snapshot_return: None,
        cancel: semio_framework_job::root_cancel_token(),
        preview_sequence: 0,
        credit: Fem3dPageCredit { item_count: 0, byte_count: 0, draw_count: 0, draw_bytes: 0 },
        backing: Fem3dBackingCredit::new(),
        solver: Some(Fem3dSolverView::new(identity.freshness(0), 1)),
        numerical: None,
        numerical_done: false,
        candidate: None,
        current: None,
        displaced: None,
        close_lane: 0,
        done: false,
        recovery,
    }
}

fn reserve_recovery_state(registry: &mut Registry, identity: Identity) -> (u16, Rc<MountedRecoverySlot>) {
    let shell = registry.allocate().unwrap();
    assert!(registry.reserve_credit(shell));
    let recovery = registry.recoveries[shell as usize].clone();
    assert!(recovery.reserve(identity));
    registry.current[identity.app_instance_id as usize % ACTIVE_CAPACITY] = Some(Current { app_instance_id: identity.app_instance_id, shell, identity });
    (shell, recovery)
}

fn drain_recovery_state(registry: &mut Registry, identity: Identity, shell: u16) {
    for _ in 0..128 {
        registry.recovery_cursor = shell as usize;
        let step = recover_abandoned_one(registry, identity.app_instance_id, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).unwrap();
        match step {
            PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
            }
            PluginCloseStep::Complete => {}
            PluginCloseStep::AwaitingInput { reason } => panic!("unexpected recovery input: {reason}"),
            PluginCloseStep::Blocked { reason } => panic!("{reason}"),
        }
        if !registry.recoveries[shell as usize].contains(identity.app_instance_id) {
            assert!(registry.shells[shell as usize].borrow().is_none());
            assert_eq!(registry.credit_items[shell as usize], 0);
            assert_eq!(registry.credit_bytes[shell as usize], 0);
            return;
        }
    }
    panic!("mounted recovery did not reach terminal zero");
}

#[test]
fn fem3d_queued_running_and_state_drop_publish_exact_identity_and_drain_one_owner() {
    let mut registry = Registry::new();

    let queued_identity = recovery_identity(41, 1);
    let (queued_shell, queued_recovery) = reserve_recovery_state(&mut registry, queued_identity);
    *registry.shells[queued_shell as usize].borrow_mut() = Some(recoverable_state(queued_identity, queued_recovery.clone()));
    let queued_job = MountedJob { shell: registry.shells[queued_shell as usize].clone(), recovery: queued_recovery.clone(), identity: queued_identity, completed: false };
    let queued_borrow = registry.shells[queued_shell as usize].borrow_mut();
    drop(queued_job);
    assert_eq!(queued_recovery.publication.get(), Some((queued_identity, MountedRecoveryPublication::Recover)));
    assert_eq!(queued_borrow.as_ref().map(|state| state.identity), Some(queued_identity));
    drop(queued_borrow);
    registry.recovery_cursor = queued_shell as usize;
    let _ = recover_abandoned_one(&mut registry, queued_identity.app_instance_id, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
    assert_eq!(queued_recovery.owner.lock().unwrap().as_ref().map(|state| state.identity), Some(queued_identity));
    drain_recovery_state(&mut registry, queued_identity, queued_shell);

    let running_identity = recovery_identity(42, 2);
    let (running_shell, running_recovery) = reserve_recovery_state(&mut registry, running_identity);
    let mut running_state = recoverable_state(running_identity, running_recovery.clone());
    assert!(running_state.solver.as_mut().unwrap().admit_page(0, false, &mut running_state.backing));
    *registry.shells[running_shell as usize].borrow_mut() = Some(running_state);
    let running_job = MountedJob { shell: registry.shells[running_shell as usize].clone(), recovery: running_recovery.clone(), identity: running_identity, completed: false };
    drop(running_job);
    assert!(registry.shells[running_shell as usize].borrow().is_none());
    assert_eq!(running_recovery.owner.lock().unwrap().as_ref().map(|state| state.identity), Some(running_identity));
    drain_recovery_state(&mut registry, running_identity, running_shell);

    let state_identity = recovery_identity(43, 3);
    let (state_shell, state_recovery) = reserve_recovery_state(&mut registry, state_identity);
    drop(recoverable_state(state_identity, state_recovery.clone()));
    assert_eq!(state_recovery.publication.get(), Some((state_identity, MountedRecoveryPublication::Recover)));
    assert_eq!(state_recovery.owner.lock().unwrap().as_ref().map(|state| state.identity), Some(state_identity));
    drain_recovery_state(&mut registry, state_identity, state_shell);
}

#[test]
fn fem3d_window_config_mounted_close_preserves_foreign_instance_in_same_slot() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🪟️window-config-contract/🧫️fixtures/🧹️mounted-close/🔣️.json")).expect("mounted close neutral fixture");
    for row in fixture["cases"].as_array().expect("close cases") {
        let closing = row["closingInstanceId"].as_u64().expect("closing instance") as u32;
        let pending = row["before"]["pendingInstanceId"].as_u64().map(|id| id as u32);
        let slot = closing as usize % ACTIVE_CAPACITY;
        if let Some(pending) = pending {
            assert_eq!(pending as usize % ACTIVE_CAPACITY, slot);
        }
        MOUNTED.with(|registry| {
            let mut registry = registry.borrow_mut();
            assert!(registry.pending[slot].is_none());
            registry.pending[slot] = pending.map(|app_instance_id| PendingSnapshot {
                render: AppRenderOperationContext { app_instance_id, base_revision: RevisionId(1), generation: Generation(1), canonical_base_revision: [1; 32] },
                preflight: SnapshotPreflight::new(),
            });
        });
        let first = close_step(closing, 1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        if pending == Some(closing) {
            assert_eq!(first, PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            assert_eq!(close_step(closing, 1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY), PluginCloseStep::Complete);
        } else {
            assert_eq!(first, PluginCloseStep::Complete);
        }
        assert!(terminal_is_empty(closing));
        let remaining = MOUNTED.with(|registry| registry.borrow().pending[slot].as_ref().map(|pending| pending.render.app_instance_id));
        assert_eq!(serde_json::json!({ "pendingInstanceId": remaining }), row["expected"], "{}", row["id"]);
        if let Some(remaining) = remaining {
            assert!(!terminal_is_empty(remaining));
            assert_eq!(close_step(remaining, 1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            assert_eq!(close_step(remaining, 1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY), PluginCloseStep::Complete);
        }
    }
    eprintln!("[DEBUG] FEM 3D mounted close preserved exact app ownership for three neutral cases");
}

#[test]
fn fem3d_visual_close_grants_retain_exact_backing_until_admitted() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🪟️window-config-contract/🧫️fixtures/💳️visual-close-grants/🔣️.json")).expect("neutral close grants");
    for row in corpus["cases"].as_array().expect("cases") {
        let doc = Fem3dSnapshot::default();
        let mut candidate = Fem3dPageVisualJob::new(freshness(19), credit(&doc));
        let mut backing = Fem3dBackingCredit::new();
        match row["owner"].as_str().expect("owner") {
            "region" => candidate.region_order = Some(FixedOrder::new(&mut backing).expect("region credit")),
            "element" => candidate.element_order = Some(FixedOrder::new(&mut backing).expect("element credit")),
            owner => panic!("unknown owner {owner}"),
        }
        let pointer = |job: &Fem3dPageVisualJob| job.region_order.as_ref().map(|order| order.slots.as_ptr()).or_else(|| job.element_order.as_ref().map(|order| order.slots.as_ptr()));
        let before_pointer = pointer(&candidate);
        let owner_bytes = candidate.backing_usage().1;
        let maximum_bytes = usize::try_from(row["maximumBytes"].as_u64().expect("grant")).expect("native grant");
        let retained = row["expected"]["retained"].as_bool().expect("retained state");
        let step = candidate.close_step(maximum_bytes);
        assert_eq!(step, (false, usize::from(!retained), if retained { 0 } else { owner_bytes }), "{}", row["id"]);
        assert!(step.2 <= maximum_bytes);
        assert!(backing.release(step.1, step.2));
        assert_eq!(candidate.backing_usage().0, usize::from(retained));
        assert_eq!(pointer(&candidate), if retained { before_pointer } else { None });
        let cleanup = candidate.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        assert!(backing.release(cleanup.1, cleanup.2));
        assert!(candidate.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).0);
        assert!(candidate.terminal_is_empty() && backing.terminal_is_empty());
    }
    eprintln!("[DEBUG] FEM visual close grants preserve exact backing for all six neutral cases");
}
