use super::*;
use semio_framework_job::StepBudget;

async fn fixture_windows(ids: &[&str]) -> semio_framework_plugin::WindowConfigOwnerRegistry {
    let mut windows = semio_framework_plugin::WindowConfigOwnerRegistry::new(protocol::ActorId("results-fixture".into()));
    windows.register::<semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::config::Fem3dResultsWindowConfigOwner>().unwrap();
    for id in ids { windows.fixture_partition_authority(semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS, id).await.unwrap(); }
    windows
}

fn fixture_consumer(windows: &semio_framework_plugin::WindowConfigOwnerRegistry, id: &str) -> ConsumerKey {
    ConsumerKey::Window(windows.borrow(semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS, id).unwrap().authority)
}

fn fixture_prepare_result(render: AppRenderOperationContext, doc: &Fem3dSnapshot, windows: &semio_framework_plugin::WindowConfigOwnerRegistry, id: &str, request: Fem3dResultRequest) {
    let original = windows.borrow(semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS, id).unwrap();
    let (spec, source) = physical_parts(doc, request).unwrap();
    let grant = render.mounted_policy.preparation;
    let version = semio_framework_value::RequestCandidateVersion { source: source_version(render), consumer_generation: original.generation, consumer_revision: original.revision };
    let progress = MOUNTED.with(|registry| prepare_result_demand(&mut registry.borrow_mut(), render, ConsumerKey::Window(original.authority), id, spec, source, version, grant));
    let mut sequence = 0; let mut receipt = RetainedCloneProgress::default();
    let mut cx = StepContext::new(semio_framework_job::allocate_operation_id(), render.generation, StepBudget::new(64, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    semio_framework_plugin::MountedSnapshotReadPreparation { ready: false, progress }.record(grant, &mut cx).unwrap();
    assert_eq!(cx.retained_progress(), progress);
}


fn release_grant(maximum_release_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_release_bytes, ..PAGE_GRANT }
}

const PAGE_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, maximum_depth: 1 };

const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

#[test]
fn fem3d_results_retained_pages_route_only_matching_completed_sources() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let current = (row["currentReady"].as_bool().unwrap() && row["currentRevision"].as_bool().unwrap() && row["currentSource"].as_bool().unwrap()).then_some(2);
        let retained = row["retainedSource"].as_bool().unwrap().then_some(1);
        let actual = select_result_shell(current, retained);
        assert_eq!(serde_json::to_value(actual).unwrap(), row["expected"], "{}", row["id"]);
        eprintln!("[DEBUG] FEM3D retained result {} shell={actual:?}", row["id"]);
    }
    let mut old = Fem3dSnapshot::default();
    old.load_cases = vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase { id: "first".into(), name: "First".into(), loads: vec![], self_weight: false }, semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase { id: "second".into(), name: "Second".into(), loads: vec![], self_weight: false }];
    let mut reordered = old.clone();
    reordered.load_cases.reverse();
    let old_request = Fem3dResultRequest { source: Fem3dResultSource::Case(1), ..Default::default() };
    let new_request = Fem3dResultRequest { source: Fem3dResultSource::Case(0), ..Default::default() };
    assert!(same_result_source(&old, old_request, &reordered, new_request));
    assert!(!same_result_source(&old, old_request, &reordered, old_request));
    assert!(!same_result_source(&old, old_request, &reordered, Fem3dResultRequest { mode: Some(0), ..new_request }));
    assert!(!same_result_source(&old, old_request, &reordered, Fem3dResultRequest { buckling: Some(0), ..new_request }));
    reordered.combinations.push(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id: "second".into(), name: "Combined Second".into(), terms: Default::default() });
    assert!(!same_result_source(&old, old_request, &reordered, Fem3dResultRequest { source: Fem3dResultSource::Combination(0), ..Default::default() }));
    assert!(!same_result_source(&old, Fem3dResultRequest::default(), &reordered, Fem3dResultRequest::default()));
    let modal = Fem3dResultRequest { mode: Some(0), ..Default::default() };
    assert!(same_result_source(&Fem3dSnapshot::default(), modal, &Fem3dSnapshot::default(), modal));
}

#[test]
fn fem3d_retained_results_close_uses_original_retirement_and_preserves_foreign_apps() {
    let mut registry = Registry::new();
    let app_instance_id = 7;
    let identity = Identity { app_instance_id, base_revision: RevisionId(11), generation: Generation(19), canonical_base_revision: [23; 32], operation: OperationId(29), job: 29 };
    let original = Current { app_instance_id, shell: 3, identity };
    registry.app_slots.claim(app_instance_id).unwrap();
    let key = request_key(&Fem3dSnapshot::default(), Fem3dResultRequest::default()).unwrap();
    let request = registry.requests.admit(app_instance_id, ConsumerKey::Model, key).unwrap().owner;
    let slot = request.index();
    registry.retained_results[slot] = Some(original);
    assert!(registry.retire_retained_results(app_instance_id + ACTIVE_CAPACITY as u32, slot));
    assert_eq!(registry.retained_results[slot].unwrap().identity, identity);
    assert!(!registry.retiring[3]);
    assert!(registry.retire_retained_results(app_instance_id, slot));
    assert!(registry.retained_results[slot].is_none());
    assert!(registry.retiring[3]);
    assert_eq!(registry.retiring_count[slot], 1);
    assert_eq!(registry.retiring_app[3], app_instance_id);
    assert!(registry.retire_retained_results(app_instance_id, slot));
    assert_eq!(registry.retiring_count[slot], 1);
    eprintln!("[DEBUG] FEM3D original result shell=3 retired exactly once; colliding foreign app preserves its owner");
}

#[test]
fn fem3d_numerical_child_classifies_every_lent_outcome_without_retaining_it() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🧫️fixtures/🧒️child-outcomes/🔣️.json")).expect("neutral child outcomes");
    let operation = semio_framework_job::Operation::new(OperationId(991), RevisionId(997), Generation(1009), 0);
    for row in corpus["cases"].as_array().expect("cases") {
        let kind = row["kind"].as_str().expect("kind");
        let end = match kind {
            "preview" | "checkpoint" => ChildEnd::Lent,
            "complete" => ChildEnd::Complete,
            "fault" => ChildEnd::Fault,
            _ => unreachable!(),
        };
        let mut child = Box::new(Fem3dNumericalChild::new());
        let result = child.observe_child_end(end);
        assert_eq!(result.is_err(), kind == "fault");
        if let Ok(complete) = result {
            assert_eq!(complete, kind == "complete");
        }
        assert!(child.observe_child_end(ChildEnd::Cancelled).is_err());
        assert_eq!(child.observe_child_end(ChildEnd::Yield), Ok(false));
        let doc = Fem3dSnapshot::default();
        let mut solver = Fem3dSolverView::new(freshness(19), 0);
        let mut backing = Fem3dBackingCredit::new();
        let mut sequence = 0;
        let mut receipt = RetainedCloneProgress::default();
        for budget in [StepBudget::new(0, u64::MAX, TEST_GRANT), StepBudget::new(1, 0, TEST_GRANT)] {
            let mut context = StepContext::new(operation.operation, operation.generation, budget, semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
            assert_eq!(child.step(&doc, &mut solver, &mut backing, freshness(19), operation, &mut context), Ok(false));
        }
        let cancelled = semio_framework_job::root_cancel_token();
        cancelled.cancel_now();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), cancelled, || Some(0), &mut sequence, &mut receipt);
        assert!(child.step(&doc, &mut solver, &mut backing, freshness(19), operation, &mut context).is_err());
        let mut complete = false;
        for _ in 0..256 {
            if child.close_step(release_grant(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)).0 {
                complete = true;
                break;
            }
        }
        assert!(complete, "all numerical owners close");
    }
}

#[test]
fn fem3d_exact_app_leases_keep_colliding_jobs_and_close_isolated() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    let ids: Vec<u32> = corpus["appIds"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32).collect();
    for &app in &ids {
        let identity = recovery_identity(app, u64::from(app));
        MOUNTED.with(|registry| {
            let mut registry = registry.borrow_mut();
            let (shell, recovery) = reserve_recovery_state(&mut registry, identity);
            *registry.shells[shell as usize].borrow_mut() = Some(recoverable_state(identity, recovery));
        });
        assert_eq!(current_identity(app), Some(identity));
    }
    let mut live = ids.clone();
    for app in corpus["closeOrder"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32) {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_CLOSE_COPY_BYTES, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES, maximum_depth: SESSION_CLOSE_DEPTH, ..PAGE_GRANT };
        let (shell, child) = MOUNTED.with(|registry| { let registry = registry.borrow(); let shell = registry.current.iter().flatten().find(|current| current.app_instance_id == app).unwrap().shell; (shell, registry.shells[shell as usize].clone()) });
        for _ in 0..8 { if MOUNTED.with(|registry| registry.borrow().retiring[shell as usize]) { break; } assert_eq!(close_step(app, grant), session_pending(1, 0)); }
        let borrow = child.borrow();
        MOUNTED.with(|registry| registry.borrow_mut().maintenance_cursor = shell as usize);
        assert!(matches!(close_step(app, grant), PluginLifecycleStep::Blocked { reason: "FEM3D retiring child is borrowed" }));
        assert!(!terminal_is_empty(app));
        drop(borrow);
        for _ in 0..200000 { if terminal_is_empty(app) { break; } let step = close_step(app, grant); assert!(!matches!(step, PluginLifecycleStep::Blocked { .. })); }
        assert!(terminal_is_empty(app));
        live.retain(|id| *id != app);
        for &foreign in &live { assert_eq!(current_identity(foreign), Some(recovery_identity(foreign, u64::from(foreign)))); assert!(!terminal_is_empty(foreign)); }
        eprintln!("[DEBUG] FEM3D exact app lease closed={app} foreign original jobs={live:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn fem3d_request_families_share_identical_panes_and_preserve_physical_sources() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    let mut doc = Fem3dSnapshot::default();
    doc.load_cases = ["a", "b"].into_iter().map(|id| semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase { id: id.into(), name: id.into(), loads: vec![], self_weight: false }).collect();
    doc.combinations = vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id: "a".into(), name: "a".into(), terms: Default::default() }];
    let identity = recovery_identity(8123, 39);
    let render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.base_revision, generation: identity.generation, canonical_base_revision: identity.canonical_base_revision, mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: TEST_GRANT, maintenance: TEST_GRANT, close: TEST_GRANT } };
    let ids: Vec<&str> = corpus["requestWindows"].as_array().unwrap().iter().map(|row| row["window"].as_str().unwrap()).collect();
    let window_owners = fixture_windows(&ids).await;
    for row in corpus["requestWindows"].as_array().unwrap() {
        let family = row["family"].as_u64().unwrap(); let index = row["index"].as_u64().unwrap() as usize;
        let source = if row["namespace"] == 1 { Fem3dResultSource::Combination(0) } else { Fem3dResultSource::Case(usize::from(row["source"] == "b")) };
        let request = Fem3dResultRequest { source, mode: (family == 1).then_some(index), buckling: (family == 2).then_some(index) };
        for _ in 0..128 { fixture_prepare_result(render, &doc, &window_owners, row["window"].as_str().unwrap(), request); }
    }
    MOUNTED.with(|registry| { let registry = registry.borrow(); assert_eq!((0..ACTIVE_CAPACITY).filter(|index| registry.requests.at(*index).is_some_and(|(_, scope, _)| scope == identity.app_instance_id)).count(), 6); let a = registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-a")).unwrap(); assert_eq!(registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-a-shared")), Some(a)); assert_eq!(registry.requests.references(a), 2); });
    doc.load_cases.reverse(); MOUNTED.with(|registry| { let registry = registry.borrow(); let owner = registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-b")).unwrap(); assert_eq!(resolve_request(&registry, &doc, owner).unwrap().source, Fem3dResultSource::Case(0)); });
    let windows: Vec<semio_framework_plugin::ViewWindowInstance> = corpus["requestWindows"].as_array().unwrap().iter().filter(|row| row["window"] != "case-a").map(|row| semio_framework_plugin::ViewWindowInstance { id: row["window"].as_str().unwrap().into(), window_kind_id: semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS.into() }).collect();
    let departed = fixture_consumer(&window_owners, "case-a");
    MOUNTED.with(|registry| { let mut registry = registry.borrow_mut(); let step = registry.requests.remove_consumer_controlled(render.app_instance_id, departed, TEST_GRANT).unwrap(); assert!(step.progress.fits(TEST_GRANT)); });
    MOUNTED.with(|registry| { let registry = registry.borrow(); let owner = registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-a-shared")).unwrap(); assert_eq!(registry.requests.references(owner), 1); assert!(registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-a")).is_none()); });
    for _ in 0..512 { let step = prepare_snapshot_admission(render, &doc, TEST_GRANT); assert!(step.progress.fits(TEST_GRANT)); }
    MOUNTED.with(|registry| { let registry = registry.borrow(); assert_eq!(registry.pending.iter().flatten().filter(|pending| pending.render.app_instance_id == identity.app_instance_id && pending.preflight.stage == SnapshotPreflightStage::Complete).count(), 6); });
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_CLOSE_COPY_BYTES, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES, maximum_depth: SESSION_CLOSE_DEPTH, ..PAGE_GRANT };
    for _ in 0..512 { if terminal_is_empty(identity.app_instance_id) { break; } close_step(identity.app_instance_id, grant); }
    assert!(terminal_is_empty(identity.app_instance_id));
    eprintln!("[DEBUG] FEM3D six independent physical request families, shared pane reference, reordered source, fair pending preflight and final exact request/app closure");
}

#[semio_framework_async_macros::async_test]
async fn fem3d_wide_physical_request_and_window_text_use_shared_granted_custody() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    let source = "a".repeat(corpus["wideSourceLength"].as_u64().unwrap() as usize); let window = "w".repeat(corpus["wideWindowLength"].as_u64().unwrap() as usize);
    let mut doc = Fem3dSnapshot::default(); doc.load_cases = vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase { id: source.clone(), name: "Wide".into(), loads: vec![], self_weight: false }];
    let window_owners = fixture_windows(&[window.as_str()]).await;
    let identity = recovery_identity(9167, 43); let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 65536, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES.max(65536), maximum_depth: SESSION_CLOSE_DEPTH };
    let mut render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.base_revision, generation: identity.generation, canonical_base_revision: identity.canonical_base_revision, mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: RetainedCloneGrant { maximum_items: 0, ..grant }, maintenance: grant, close: grant } };
    let request = Fem3dResultRequest { source: Fem3dResultSource::Case(0), ..Default::default() };
    fixture_prepare_result(render, &doc, &window_owners, &window, request); MOUNTED.with(|registry| assert!(registry.borrow().app_slots.lookup(identity.app_instance_id).is_none()));
    render.mounted_policy.preparation = grant;
    for _ in 0..512 { fixture_prepare_result(render, &doc, &window_owners, &window, request); }
    MOUNTED.with(|registry| { let registry = registry.borrow(); let owner = registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, &window)).unwrap(); assert!(registry.requests.ready(owner)); assert_eq!(registry.requests.request_name(owner), Some(source.as_str())); assert_eq!(resolve_request(&registry, &doc, owner), Some(request)); let index = registry.requests.consumer_index(identity.app_instance_id, fixture_consumer(&window_owners, &window)).unwrap(); assert_eq!(registry.requests.consumer_name(index), Some(window.as_str())); });
    for _ in 0..128 { if terminal_is_empty(identity.app_instance_id) { break; } close_step(identity.app_instance_id, grant); }
    assert!(terminal_is_empty(identity.app_instance_id)); assert_eq!(doc.load_cases[0].id, source);
    eprintln!("[DEBUG] FEM3D shared granted source{} / window{} original text ownership, zero-item refusal and funded terminal closure", source.len(), window.len());
}

#[test]
fn fem3d_original_fault_and_cancel_terminal_receipts_prevent_implicit_resubmit() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    for (ordinal, kind) in corpus["terminalKinds"].as_array().unwrap().iter().enumerate() {
        let identity = recovery_identity(9239 + ordinal as u32, 47 + ordinal as u64);
        let (shell, recovery) = MOUNTED.with(|registry| { let mut registry = registry.borrow_mut(); let (shell, recovery) = reserve_recovery_state(&mut registry, identity); let owner = registry.shell_request[shell as usize].unwrap(); assert!(registry.requests.begin_execution(owner, identity_version(identity), identity.job)); (shell, recovery) });
        *MOUNTED.with(|registry| registry.borrow().shells[shell as usize].clone()).borrow_mut() = Some(recoverable_state(identity, recovery.clone()));
        if kind == "faulted" { MOUNTED.with(|registry| { let child = registry.borrow().shells[shell as usize].clone(); let step = child.borrow_mut().as_mut().unwrap().fail(b"original-fault".to_vec()); let JobStep::Failed(body) = step else { panic!("original fault outcome") }; assert_eq!(body, b"original-fault"); }); }
        let job = MountedJob { shell: MOUNTED.with(|registry| registry.borrow().shells[shell as usize].clone()), recovery, identity, completed: false, input: None, restored: None };
        if kind == "cancelled" { let mut job = job; job.cancel(); drop(job); } else { drop(job); }
        MOUNTED.with(|registry| drain_recovery_state(&mut registry.borrow_mut(), identity, shell));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_CLOSE_COPY_BYTES, maximum_capacity_bytes: 65536, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES.max(65536), maximum_depth: SESSION_CLOSE_DEPTH };
        let mut render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.base_revision, generation: identity.generation, canonical_base_revision: identity.canonical_base_revision, mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
        let doc = Fem3dSnapshot::default();
        for _ in 0..64 { assert!(!prepare_snapshot_admission(render, &doc, TEST_GRANT).ready); }
        assert_eq!(result_stage(Some(render), None), Some(if kind == "faulted" { "fem3d.failed" } else { "fem3d.cancelled" }));
        MOUNTED.with(|registry| assert!(registry.borrow().pending.iter().all(Option::is_none)));
        render.canonical_base_revision[31] ^= 1;
        let mut admitted = false; for _ in 0..128 { if prepare_snapshot_admission(render, &doc, TEST_GRANT).ready { admitted = true; break; } }
        assert!(admitted, "a new exact canonical source may reopen after terminal outcome");
        for _ in 0..512 { if terminal_is_empty(identity.app_instance_id) { break; } close_step(identity.app_instance_id, grant); }
        assert!(terminal_is_empty(identity.app_instance_id));
        eprintln!("[DEBUG] FEM3D original {kind} retained exact source terminal through worker retirement, no same-version resubmit, canonical change reopened and terminal custody returned");
    }
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
    let mut job = Fem3dPageVisualJob::new(solver.freshness, credit(doc));
    let mut backing = Fem3dBackingCredit::new();
    let mut turns = 0;
    while !job.step_one(doc, solver, &mut backing, solver.freshness).expect("one production opportunity") && turns < 4_096 {
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
    doc.nodes.resize_with(MAXIMUM_NODES + 1, || semio_s_fem_node::volumetric::FemNodeValue { id: "n".into(), x: 0.0, y: 0.0, z: 0.0 });
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
    doc.nodes.push(semio_s_fem_node::volumetric::FemNodeValue { id: "n".repeat(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY + 1), x: 0.0, y: 0.0, z: 0.0 });
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
    doc.nodes.push(semio_s_fem_node::volumetric::FemNodeValue { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 });
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
    assert_eq!(model.admit_node_one(MAXIMUM_FIELDS + 1, MOUNTED_ANALYSIS_BACKING_BYTES), Err(semio_s_fem_analyses_model::MountedAnalysisFault::Capacity { requested: MAXIMUM_FIELDS + 1, maximum: semio_s_fem_analyses_model::MOUNTED_ANALYSIS_NODE_SLOTS }));
    assert_eq!(model.nodes_len(), 0);
    let node = Node { id: "returned-node".into(), pos: [0.0; 3] };
    let node_pointer = node.id.as_ptr();
    let returned = model.push_node(node).expect_err("unadmitted node returns producer");
    assert_eq!(returned.id.as_ptr(), node_pointer);

    let mut domain = MountedPlanarDomain::new();
    assert_eq!(domain.admit_outer_one(MAXIMUM_FIELDS + 1), Err(semio_s_fem_mesh::MountedDomainFault::PointCapacity { requested: MAXIMUM_FIELDS + 1, maximum: semio_s_fem_mesh::MOUNTED_DOMAIN_POINT_SLOTS }));
    assert_eq!(domain.push_outer([1.0, 2.0]), Err([1.0, 2.0]));

    let mut scalars = MountedScalarSlots::new();
    assert_eq!(scalars.admit_one(MAXIMUM_FIELDS * 6 + 1), Err(semio_s_fem_sparse::MountedScalarFault::Capacity { requested: MAXIMUM_FIELDS * 6 + 1, maximum: semio_s_fem_sparse::MOUNTED_SCALAR_SLOTS }));
    assert_eq!(scalars.len(), 0);
    assert_eq!(scalars.push(17.0), Err(17.0));
    assert!(started.elapsed().as_micros() < 8_000);
}

#[test]
fn fem3d_numerical_child_absent_lanes_retain_later_close_owners() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️numerical-close/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["cases"].as_array().unwrap() {
        let lane = row["lane"].as_u64().unwrap() as u8;
        let mut child = Fem3dNumericalChild::new();
        child.close_lane = lane;
        if row["model"].as_bool().unwrap() {
            let model = child.model.as_mut().unwrap();
            while !model.admit_node_one(1, MOUNTED_ANALYSIS_BACKING_BYTES).unwrap().complete {}
            model.push_node(Node { id: "retained-model-node".into(), pos: [0.0; 3] }).unwrap();
        } else { child.model = None; }
        if row["nodeIds"].as_bool().unwrap() {
            assert_eq!(child.analysis_node_ids.admit_one(1), Ok(false));
            child.analysis_node_ids.push("retained-analysis-node".into()).unwrap();
        }
        let zero = child.close_step(release_grant(0));
        let held = child.close_lane == lane && child.model.is_some() == row["model"].as_bool().unwrap() && child.analysis_node_ids.len() == usize::from(row["nodeIds"].as_bool().unwrap());
        let first = child.close_step(release_grant(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES));
        let actual = serde_json::json!({ "lane": child.close_lane, "complete": first.0 });
        for _ in 0..1_024 {
            let step = child.close_step(release_grant(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES));
            if step.0 {
                if child.close_lane < 22 { child.close_lane += 1; }
                else { break; }
            }
        }
        let emptied = child.model.is_none() && child.analysis_node_ids.len == 0 && child.analysis_node_ids.admitted == 0;
        observations.push((lane, zero, held, actual, row["expected"].clone(), emptied));
    }
    for (lane, zero, held, actual, expected, emptied) in observations {
        assert!(emptied, "lane {lane} closes retained test owners");
        assert_eq!(zero, (false, 0, 0), "lane {lane} zero grant");
        assert!(held, "lane {lane} zero grant preserves owner");
        assert_eq!(actual, expected, "lane {lane} absence advances instead of completing");
    }
}

#[test]
fn fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized() {

    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemDof;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
    use semio_s_fem_material::volumetric::FemMaterialValue;
    use semio_s_fem_node::volumetric::FemNodeValue;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSolid;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSupport;
    use semio_framework_trace::InteractiveStage;
    use semio_framework_trace::StepOverrunLedger;
    use semio_framework_trace::Watchdog;
    use semio_framework_trace::INTERACTIVE_STEP_CEILING_US;

    let doc = Fem3dSnapshot {
        nodes: vec![FemNodeValue { id: "n0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n1".into(), x: 1.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n2".into(), x: 1.0, y: 1.0, z: 0.0 }, FemNodeValue { id: "n3".into(), x: 0.0, y: 1.0, z: 0.0 }],
        materials: vec![FemMaterialValue { id: "m".into(), name: "M".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
        solids: vec![FemSolid { id: "s".into(), name: "S".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.25, layers: 1, mesh_size: 2.0, material_id: "m".into(), axis: crate::FemAxis::Z }],
        supports: (0..4).map(|index| FemSupport { id: format!("f{index}"), node_id: format!("n{index}"), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] }).collect(),
        load_cases: vec![FemLoadCase { id: "g".into(), name: "G".into(), loads: vec![], self_weight: true }],
        ..Default::default()
    };
    let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
    let mut child = Fem3dNumericalChild::new();
    child.request = Fem3dResultRequest { mode: Some(0), ..Fem3dResultRequest::default() };
    child.request.mode = Some(0);
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview = 0;
    let mut terminal = false;
    let mut last_stage = "initial";
    let mut overruns = StepOverrunLedger::new();
    let modal_columns = 9usize;
    let maximum_modal_order = 40usize;
    let jacobi_sweep_steps = modal_columns * modal_columns + 1 + modal_columns * (modal_columns - 1) / 2 * (12 * modal_columns + 16) + modal_columns;
    let maximum_turns = 200_000 + 30 * (100 * jacobi_sweep_steps + 32 * maximum_modal_order.pow(2) * modal_columns);
    let mut turns = 0;
    let mut failure = None;
    for _ in 0..maximum_turns {
        turns += 1;
        let deadline = semio_framework_job::default_now_us().unwrap().checked_add(INTERACTIVE_STEP_CEILING_US).unwrap();
        let mut preview_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, deadline, TEST_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview, &mut preview_receipt);
        let before = (child.stage, child.node_cursor, child.scalar_axis, child.reaction_entry);
        let watchdog = Watchdog::start("fem3d.production-numerical-child", operation.operation, operation.generation, InteractiveStage::UserVisibleSimStep);
        match child.step(&doc, &mut fields, &mut backing, freshness(19), operation, &mut context) {
            Ok(complete) => terminal = complete,
            Err(fault) => failure = Some(format!("production numerical child at {:?}/{}: {}", child.stage, context.stage(), String::from_utf8_lossy(&fault))),
        }
        let verdict = watchdog.finish();
        let admission = overruns.admit(&verdict);
        let elapsed_us = verdict.elapsed_us();
        let after = (child.stage, child.node_cursor, child.scalar_axis, child.reaction_entry);
        if context.fuel_remaining() != 0 && !(context.deadline_exceeded() && before == after && !terminal) {
            failure.get_or_insert_with(|| format!("numerical unconsumed opportunity before={before:?}, after={after:?}, context {}, terminal={terminal}, expired={}, elapsed_us={elapsed_us:?}", context.stage(), context.deadline_exceeded()));
        }
        last_stage = context.stage();
        if verdict.is_fault() {
            eprintln!("numerical before={before:?}, after={after:?}, context {}, terminal={terminal}, expired={}, elapsed_us={elapsed_us:?}, admission={admission:?}", context.stage(), context.deadline_exceeded());
        }
        if admission.is_terminal() {
            failure.get_or_insert_with(|| format!("numerical watchdog rejected before={before:?}, after={after:?}, admission={admission:?}"));
        }
        if terminal || failure.is_some() {
            break;
        }
    }
    let progress = (child.stage, child.pcg.as_ref().map(PcgJob::visual_progress), child.subspace.as_ref().map(SubspaceIterationJob::visual_progress));
    let ready = fields.ready();
    let has_static = (0..fields.len).filter_map(|index| fields.scalar(index)).any(|scalar| scalar.displacement != [0.0; 3] || scalar.reaction != [0.0; 3]);
    let has_modal = (0..fields.len).filter_map(|index| fields.scalar(index)).any(|scalar| scalar.eigen_estimate > 0.0 && scalar.mode_shape != [0.0; 3]);
    assert_eq!(child.close_step(release_grant(0)), (false, 0, 0));
    let mut closed = false;
    for _ in 0..200_000 {
        if child.close_step(release_grant(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES.max(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY))).0 {
            closed = true;
            break;
        }
    }
    eprintln!("FEM3D numerical terminal={terminal}, ready={ready}, static={has_static}, modal={has_modal}, closed={closed}, close_lane={}, close_started_jobs={}, turns={turns}/{maximum_turns}, progress={progress:?}, recorded overruns={}, longest run={}, worst elapsed_us={}", child.close_lane, child.close_started_jobs, overruns.total_overruns(), overruns.longest_overrun_run(), overruns.worst_elapsed_us());
    assert!(closed && child.terminal_is_empty(), "numerical child close did not reach exact terminal");
    assert!(failure.is_none(), "[TRACE] {failure:?}");
    assert!(terminal, "[TRACE] numerical stopped at {progress:?}/{last_stage}");
    assert!(ready);
    assert!(has_static);
    assert!(has_modal);
}

/// 🏠️ The shipped demo crosses physical order pages while numerical and visual publication remain progressive.
#[test]
fn fem3d_production_numerical_child_solves_the_demo_with_paged_owner_backing() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📚️paged-demo/🔣️.json")).expect("independent NumPy demo equilibrium");
    let doc = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot();
    assert!(!doc.solids.is_empty(), "the demo carries a meshed slab");
    let (analysis_nodes, _) = semio_s_artifact_fem_3d_engine::fem3d_engine::meshing::mesh_solids(&doc).expect("the demo meshes");
    assert!(analysis_nodes.len() * 6 * size_of::<(String, Dof)>() > 4_096, "the demo's dof order outgrows the mounted owner page — the premise of this law");
    let operation = semio_framework_job::Operation::new(OperationId(31), RevisionId(7), Generation(3), 5);
    let mut child = Fem3dNumericalChild::new();
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(3), doc.nodes.len());
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview = 0;
    let mut terminal = false;
    let mut failure = None;
    for _ in 0..16_000_000 {
        let mut preview_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview, &mut preview_receipt);
        match child.step(&doc, &mut fields, &mut backing, freshness(3), operation, &mut context) {
            Ok(complete) => terminal = complete,
            Err(fault) => failure = Some((child.stage, String::from_utf8_lossy(&fault).into_owned())),
        }
        if terminal || failure.is_some() {
            break;
        }
    }
    let stress: [Option<f64>; MAXIMUM_FIELDS] = std::array::from_fn(|index| (child.stress_counts[index] != 0).then(|| child.stress_sums[index] / child.stress_counts[index] as f64));
    for _ in 0..200_000 {
        if child.close_step(release_grant(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES.max(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY))).0 {
            break;
        }
    }
    eprintln!("[DEBUG] FEM3D paged demo terminal={terminal}, failure={failure:?}, ready={}, closed={}", fields.ready(), child.terminal_is_empty());
    assert!(child.terminal_is_empty(), "the demo releases every numerical owner");
    assert!(failure.is_none(), "paged demo failed at {failure:?}");
    assert!(terminal && fields.ready(), "the mounted demo reaches a validated numerical publication");
    let results = semio_s_artifact_fem_3d_engine::fem3d_engine::fem3d_solve_all(&doc).expect("the synchronous path solves the same document");
    let expected = results.get(&semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").unwrap()).expect("demo first case");
    for (index, node) in doc.nodes.iter().enumerate() {
        let actual = fields.scalar(index).expect("published demo scalar");
        let displacement = expected.displacements.iter().find(|row| row.node_id == node.id).expect("reference displacement");
        for axis in 0..3 {
            assert!((actual.displacement[axis] - displacement.values[axis]).abs() <= 1e-7 * displacement.values[axis].abs().max(1.0), "demo displacement {}/{axis}", node.id);
        }
    }
    for (index, node) in doc.nodes.iter().enumerate() {
        let actual = fields.scalar(index).unwrap();
        if node.id.ends_with("_l1") || node.id.ends_with("_l2") {
            let storey = usize::from(node.id.ends_with("_l2"));
            let expected = oracle["frame"]["storeyVerticalDisplacements"][storey].as_f64().unwrap();
            assert!((actual.displacement[2] - expected).abs() <= 1e-8, "independent gravity displacement {}: {} != {expected}", node.id, actual.displacement[2]);
        }
        if node.id.ends_with("_g") {
            let expected = oracle["frame"]["rootVerticalReaction"].as_f64().unwrap();
            assert!((actual.reaction[2] - expected).abs() <= 1e-3, "independent frame reaction {}", node.id);
        }
    }
    let slab_reaction: f64 = doc.nodes.iter().enumerate().filter(|(_, node)| node.id.starts_with("sc")).map(|(index, _)| fields.scalar(index).unwrap().reaction[2]).sum();
    assert!((slab_reaction - oracle["slab"]["verticalReactionSum"].as_f64().unwrap()).abs() <= 1e-3);
    eprintln!("[DEBUG] FEM3D paged demo matched independent NumPy frame gravity displacement/reaction and slab equilibrium={slab_reaction}");
    let expected_stress = semio_s_artifact_fem_3d_engine::fem3d_engine::mesh_preview::fem3d_nodal_von_mises_of(&doc, expected).unwrap();
    for (index, node) in analysis_nodes.iter().enumerate() {
        if let Some(expected) = expected_stress.get(&node.id) { assert!((stress[index].unwrap() - expected).abs() < 1e-5 * expected.abs().max(1.0), "demo stress {}", node.id); }
    }
    let mut visual = build(&doc, &fields);
    assert_eq!(close(&mut visual), FEM3D_VISUAL_PAGES);
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
    registry.app_slots.claim(identity.app_instance_id).unwrap();
    let request = registry.requests.admit(identity.app_instance_id, ConsumerKey::Model, request_key(&Fem3dSnapshot::default(), Fem3dResultRequest::default()).unwrap()).unwrap().owner;
    let slot = request.index();
    registry.shell_request[shell as usize] = Some(request);
    registry.current[slot] = Some(Current { app_instance_id: identity.app_instance_id, shell, identity });
    (shell, recovery)
}

fn drain_recovery_state(registry: &mut Registry, identity: Identity, shell: u16) {
    for _ in 0..128 {
        registry.recovery_cursor = shell as usize;
        let step = recover_abandoned_one(registry, identity.app_instance_id, PAGE_GRANT).unwrap();
        match step {
            PluginLifecycleStep::Progress(progress) => {
                assert!(progress.copied_items <= 1);
                assert!(progress.released_bytes <= WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
            }
            PluginLifecycleStep::Complete(_) => {}
            PluginLifecycleStep::AwaitingInput { reason } => panic!("unexpected recovery input: {reason}"),
            PluginLifecycleStep::Blocked { reason } => panic!("{reason}"),
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
    let queued_job = MountedJob { shell: registry.shells[queued_shell as usize].clone(), recovery: queued_recovery.clone(), identity: queued_identity, completed: false, input: None, restored: None };
    let queued_borrow = registry.shells[queued_shell as usize].borrow_mut();
    drop(queued_job);
    assert_eq!(queued_recovery.publication.get(), Some((queued_identity, MountedRecoveryPublication::Recover)));
    assert_eq!(queued_borrow.as_ref().map(|state| state.identity), Some(queued_identity));
    drop(queued_borrow);
    registry.recovery_cursor = queued_shell as usize;
    let _ = recover_abandoned_one(&mut registry, queued_identity.app_instance_id, PAGE_GRANT);
    assert_eq!(queued_recovery.owner.lock().unwrap().as_ref().map(|state| state.identity), Some(queued_identity));
    drain_recovery_state(&mut registry, queued_identity, queued_shell);

    let running_identity = recovery_identity(42, 2);
    let (running_shell, running_recovery) = reserve_recovery_state(&mut registry, running_identity);
    let mut running_state = recoverable_state(running_identity, running_recovery.clone());
    assert!(running_state.solver.as_mut().unwrap().admit_page(0, false, &mut running_state.backing));
    *registry.shells[running_shell as usize].borrow_mut() = Some(running_state);
    let running_job = MountedJob { shell: registry.shells[running_shell as usize].clone(), recovery: running_recovery.clone(), identity: running_identity, completed: false, input: None, restored: None };
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🧫️fixtures/🧹️mounted-close/🔣️.json")).expect("mounted close neutral fixture");
    for row in fixture["cases"].as_array().expect("close cases") {
        let closing = row["closingInstanceId"].as_u64().expect("closing instance") as u32;
        let pending = row["before"]["pendingInstanceId"].as_u64().map(|id| id as u32);
        let slot = MOUNTED.with(|registry| {
            let mut registry = registry.borrow_mut();
            let app = pending.unwrap_or(closing);
            registry.app_slots.claim(app).unwrap();
            let owner = registry.requests.admit(app, ConsumerKey::Model, request_key(&Fem3dSnapshot::default(), Fem3dResultRequest::default()).unwrap()).unwrap().owner;
            let slot = owner.index();
            assert!(registry.pending[slot].is_none());
            registry.pending[slot] = pending.map(|app_instance_id| PendingSnapshot {
                render: AppRenderOperationContext { app_instance_id, base_revision: RevisionId(1), generation: Generation(1), canonical_base_revision: [1; 32], mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, maintenance: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, close: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 } } },
                preflight: SnapshotPreflight::new(),
            });
            if pending.is_none() { registry.requests.remove_one_scoped_consumer(app); assert!(registry.requests.release(owner)); let lease = registry.app_slots.lookup(closing).unwrap(); assert!(registry.app_slots.release(closing, lease)); }
            slot
        });
        let first = close_step(closing, PAGE_GRANT);
        if pending == Some(closing) {
            assert_eq!(first, session_pending(1, 0));
            for _ in 0..16 { if terminal_is_empty(closing) { break; } close_step(closing, PAGE_GRANT); }
        } else {
            assert_eq!(first, PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
        }
        assert!(terminal_is_empty(closing));
        let remaining = MOUNTED.with(|registry| registry.borrow().pending[slot].as_ref().map(|pending| pending.render.app_instance_id));
        assert_eq!(serde_json::json!({ "pendingInstanceId": remaining }), row["expected"], "{}", row["id"]);
        if let Some(remaining) = remaining {
            assert!(!terminal_is_empty(remaining));
            for _ in 0..16 { if terminal_is_empty(remaining) { break; } close_step(remaining, PAGE_GRANT); }
            assert_eq!(close_step(remaining, PAGE_GRANT), PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
        }
    }
}

#[test]
fn fem3d_visual_close_grants_retain_exact_backing_until_admitted() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🧫️fixtures/💳️visual-close-grants/🔣️.json")).expect("neutral close grants");
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
}

#[test]
fn fem3d_mounted_selected_sources_match_pynite_and_case_order_is_irrelevant() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️sources/🔣️.json")).unwrap();
    let base: Fem3dSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../📈️analysis/🧪️tests/🧫️fixtures/🧮️solves-fem3d-1-benchmarks/📐️cantilever-tip-load-local-y.snapshot.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for order in corpus["orders"].as_array().unwrap() {
        let mut doc = base.clone();
        doc.load_cases = order.as_array().unwrap().iter().map(|id| {
            let mut case = base.load_cases[0].clone();
            case.id = id.as_str().unwrap().into();
            let factor = corpus["cases"].as_array().unwrap().iter().find(|row| row["kind"] == "loadCase" && row["id"] == *id).unwrap()["factor"].as_f64().unwrap();
            for load in &mut case.loads { if let FemLoad::Nodal { value, .. } = load { *value *= factor; } }
            case
        }).collect();
        doc.combinations = vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id: "tip".into(), name: "Combined".into(), terms: corpus["combinationTerms"].as_object().unwrap().iter().map(|(id, value)| (id.clone(), value.as_f64().unwrap())).collect() }];
        for row in corpus["cases"].as_array().unwrap() {
            let identity = semio_s_fem_analyses_model::encode_result_source_parts(row["kind"].as_str().unwrap(), row["id"].as_str().unwrap()).unwrap();
            let mut child = Fem3dNumericalChild::new();
            child.request = result_request(&doc, Some(&identity), None).unwrap();
            let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
            let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
            let mut backing = Fem3dBackingCredit::new();
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview = 0;
            let mut complete = false;
            let mut fault = None;
            for _ in 0..500_000 {
                let mut receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview, &mut receipt);
                match child.step(&doc, &mut fields, &mut backing, freshness(19), operation, &mut context) { Ok(done) => complete = done, Err(error) => fault = Some(String::from_utf8_lossy(&error).into_owned()) }
                assert_ne!(context.stage(), "initial", "the original numerical owner publishes its actual stage through shared job progress");
                if complete || fault.is_some() { break; }
            }
            let tip = fields.scalar(1).map(|scalar| scalar.displacement[1]);
            let reaction = fields.scalar(0).map(|scalar| scalar.reaction[1]);
            let moment = child.full_reactions[0][5];
            let rotation = child.equations[1][5].and_then(|equation| child.pcg.as_ref()?.visual_scalar(equation)).map(|scalar| scalar.displacement);
            for _ in 0..200_000 { if child.close_step(release_grant(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY)).0 { break; } }
            eprintln!("[DEBUG] FEM3D source {identity} order={order} complete={complete} tip={tip:?} reaction={reaction:?} fault={fault:?}");
            assert!(child.terminal_is_empty() && fault.is_none() && complete && fields.ready());
            let factor = row["factor"].as_f64().unwrap();
            assert!((tip.unwrap() - factor * corpus["expected"]["tipTy"].as_f64().unwrap()).abs() < 1e-9);
            assert!((reaction.unwrap() - factor * corpus["expected"]["rootTyReaction"].as_f64().unwrap()).abs() < 1e-5);
            assert!((moment - factor * corpus["expected"]["rootRzReaction"].as_f64().unwrap()).abs() < 1e-5);
            assert!((rotation.unwrap() - factor * corpus["expected"]["tipRz"].as_f64().unwrap()).abs() < 1e-9);
        }
    }
}
#[path = "../🪪️render-revision/🦀️.rs"]
mod render_revision_tests;

#[test]
fn fem3d_source_preflight_rejects_ambiguous_missing_nonfinite_and_excess_sources() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️sources/🔣️.json")).unwrap();
    assert_eq!(corpus["maximumSources"].as_u64().unwrap() as usize, MAXIMUM_LOADS);
    let base: Fem3dSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../📈️analysis/🧪️tests/🧫️fixtures/🧮️solves-fem3d-1-benchmarks/📐️cantilever-tip-load-local-y.snapshot.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for refusal in corpus["refusals"].as_array().unwrap() {
        let mut doc = base.clone();
        doc.combinations.push(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id: "tip".into(), name: "Combined".into(), terms: [("tip".into(), 1.0)].into_iter().collect() });
        match refusal.as_str().unwrap() {
            "duplicate-case" => doc.load_cases.push(doc.load_cases[0].clone()),
            "duplicate-combination" => doc.combinations.push(doc.combinations[0].clone()),
            "missing-case" => { doc.combinations[0].terms.insert("missing".into(), 1.0); },
            "nonfinite-factor" => { doc.combinations[0].terms.insert("tip".into(), f64::INFINITY); },
            "case-maximum-plus-one" => doc.load_cases = (0..=MAXIMUM_LOADS).map(|index| semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase { id: format!("case{index}"), name: "Case".into(), loads: vec![], self_weight: false }).collect(),
            "combination-maximum-plus-one" => doc.combinations = (0..=MAXIMUM_LOADS).map(|index| semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id: format!("combination{index}"), name: "Combination".into(), terms: [ ("tip".into(), 1.0) ].into_iter().collect() }).collect(),
            "term-maximum-plus-one" => doc.combinations[0].terms = (0..=MAXIMUM_LOADS).map(|index| (format!("case{index}"), 1.0)).collect(),
            _ => unreachable!(),
        }
        let mut preflight = SnapshotPreflight::new();
        let mut rejected = false;
        for _ in 0..20_000 {
            match preflight.step_one(&doc) { Err(()) => { rejected = true; break; }, Ok(true) => break, Ok(false) => {} }
        }
        eprintln!("[DEBUG] FEM3D source admission {refusal}: rejected={rejected}");
        assert!(rejected && preflight.credit().is_err());
    }
}
