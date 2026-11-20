use super::*;
use semio_framework_job::StepBudget;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_SOLVER_PAGE_COUNT;
use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::{result_request, Fem3dResultSource};

/// 🗑️ Test owner of a sealed snapshot: the last alias retires through the funded protocol instead of a bare drop.
struct Doc(Option<Fem3dSnapshot>);
impl Doc {
    fn new(snapshot: Fem3dSnapshot) -> Self { Self(Some(snapshot)) }
}
impl Default for Doc {
    fn default() -> Self { Self::new(Fem3dSnapshot::default()) }
}
impl Clone for Doc {
    fn clone(&self) -> Self { Self::new((**self).clone()) }
}
impl std::ops::Deref for Doc {
    type Target = Fem3dSnapshot;
    fn deref(&self) -> &Fem3dSnapshot { self.0.as_ref().expect("live test snapshot") }
}
impl std::ops::DerefMut for Doc {
    fn deref_mut(&mut self) -> &mut Fem3dSnapshot { self.0.as_mut().expect("live test snapshot") }
}
impl Drop for Doc {
    fn drop(&mut self) { if let Some(snapshot) = self.0.take() { semio_s_fem_receiving::testkit::retire(snapshot); } }
}

fn with_context<R>(grant: RetainedCloneGrant, run: impl FnOnce(&mut StepContext<'_>) -> R) -> R {
    let bank = RetainedCloneGrant { maximum_items: 1 << 12, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 1 << 12 };
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(17), Generation(1), bank, || Some(1000), 12345, 64 * 32);
    let mut output = None;
    caller.turn(grant, |cx| cx.with_original_retirement(|cx| { output = Some(run(cx)); Ok(()) })).expect("original fixture turn");
    caller.retire(bank);
    output.expect("the original fixture turn ran")
}

fn lifecycle_close(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    with_context(grant, |cx| close_step(app_instance_id, cx, grant)).expect("original close turn")
}

fn recover_one(registry: &mut Registry, app_instance_id: u32) -> Option<PluginLifecycleStep> {
    with_context(PAGE_GRANT, |cx| recover_abandoned_one(registry, app_instance_id, cx, PAGE_GRANT)).expect("original recovery turn")
}

fn close_child(child: &mut Fem3dNumericalChild, grant: RetainedCloneGrant) -> bool {
    with_context(grant, |cx| child.close_step(cx, grant)).expect("original close turn").0
}


fn numerical_bank() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1 << 36, maximum_copy_bytes: 1 << 40, maximum_capacity_bytes: 1 << 40, maximum_release_bytes: 1 << 40, maximum_depth: 1 << 12 }
}

fn close_in(caller: &mut semio_framework_job::OriginalJobFixtureCaller, child: &mut Fem3dNumericalChild, grant: RetainedCloneGrant) -> bool {
    caller.turn(grant, |cx| cx.with_original_retirement(|cx| child.close_step(cx, grant))).expect("original close turn").0
}

async fn fixture_windows(ids: &[&str]) -> semio_plugin::WindowConfigOwnerRegistry {
    let mut windows = semio_plugin::WindowConfigOwnerRegistry::new(protocol::ActorId("results-fixture".into()));
    windows.register::<semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::config::Fem3dResultsWindowConfigOwner>().unwrap();
    for id in ids { windows.fixture_partition_authority(semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS, id).await.unwrap(); }
    windows
}

fn fixture_consumer(windows: &semio_plugin::WindowConfigOwnerRegistry, id: &str) -> ConsumerKey {
    ConsumerKey::Window(windows.borrow(semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS, id).unwrap().authority)
}

fn fixture_prepare_result(render: AppRenderOperationContext, doc: &Fem3dSnapshot, windows: &semio_plugin::WindowConfigOwnerRegistry, id: &str, request: Fem3dResultRequest) {
    let original = windows.borrow(semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS, id).unwrap();
    let (spec, source) = physical_parts(doc, request).unwrap();
    let grant = render.mounted_policy.preparation;
    let version = semio_framework_value_registry::fixed_request_owners::RequestCandidateVersion { source: source_version(render), consumer_generation: original.generation, consumer_revision: original.revision };
    let progress = MOUNTED.with(|registry| prepare_result_demand(&mut registry.borrow_mut(), render, ConsumerKey::Window(original.authority), id, spec, source, version, grant));
    let mut sequence = 0; let mut receipt = RetainedCloneProgress::default();
    let mut cx = StepContext::new(semio_framework_job::allocate_operation_id(), render.generation, StepBudget::new(64, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    semio_plugin::MountedSnapshotReadPreparation { ready: false, progress }.record(grant, &mut cx).unwrap();
    assert_eq!(cx.retained_progress(), progress);
}


fn release_grant(maximum_release_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_release_bytes, ..PAGE_GRANT }
}

const PAGE_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, maximum_depth: 1 };

const CLOSE_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_copy_bytes: SESSION_CLOSE_COPY_BYTES, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES, maximum_depth: SESSION_CLOSE_DEPTH, ..PAGE_GRANT };

const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

#[test]
fn fem3d_results_retained_pages_route_only_matching_completed_sources() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let current = (row["currentReady"].as_bool().unwrap() && row["currentRevision"].as_bool().unwrap() && row["currentSource"].as_bool().unwrap()).then_some(2);
        let retained = row["retainedSource"].as_bool().unwrap().then_some(1);
        let actual = select_result_shell(current, retained);
        assert_eq!(serde_json::to_value(actual).unwrap(), row["expected"], "{}", row["id"]);
    }
    let mut old = Doc::default();
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
    assert!(same_result_source(&Doc::default(), modal, &Doc::default(), modal));
}

#[test]
fn fem3d_retained_results_close_uses_original_retirement_and_preserves_foreign_apps() {
    let mut registry = Registry::new();
    let app_instance_id = 7;
    let identity = Identity { app_instance_id, base_revision: RevisionId(11), generation: Generation(19), canonical_base_revision: [23; 32], operation: OperationId(29), job: 29 };
    let original = Current { app_instance_id, shell: 3, identity };
    registry.app_slots.claim(app_instance_id).unwrap();
    let key = request_key(&Doc::default(), Fem3dResultRequest::default()).unwrap();
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
}

#[test]
fn fem3d_exact_app_leases_keep_colliding_jobs_and_close_isolated() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
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
        for _ in 0..8 { if MOUNTED.with(|registry| registry.borrow().retiring[shell as usize]) { break; } assert_eq!(lifecycle_close(app, grant), session_pending(1, 0)); }
        let borrow = child.borrow();
        MOUNTED.with(|registry| registry.borrow_mut().maintenance_cursor = shell as usize);
        assert!(matches!(lifecycle_close(app, grant), PluginLifecycleStep::Blocked { reason: "FEM3D retiring child is borrowed" }));
        assert!(!terminal_is_empty(app));
        drop(borrow);
        for _ in 0..200000 { if terminal_is_empty(app) { break; } let step = lifecycle_close(app, grant); assert!(!matches!(step, PluginLifecycleStep::Blocked { .. })); }
        assert!(terminal_is_empty(app));
        live.retain(|id| *id != app);
        for &foreign in &live { assert_eq!(current_identity(foreign), Some(recovery_identity(foreign, u64::from(foreign)))); assert!(!terminal_is_empty(foreign)); }
    }
}

#[semio_framework_async_macros::async_test]
async fn fem3d_request_families_share_identical_panes_and_preserve_physical_sources() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    let mut doc = Doc::default();
    doc.load_cases = ["a", "b"].into_iter().map(|id| semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase { id: id.into(), name: id.into(), loads: vec![], self_weight: false }).collect();
    doc.combinations = vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id: "a".into(), name: "a".into(), terms: Default::default() }];
    let identity = recovery_identity(8123, 39);
    let render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.base_revision, generation: identity.generation, canonical_base_revision: identity.canonical_base_revision, mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: TEST_GRANT, maintenance: TEST_GRANT, close: TEST_GRANT } };
    let ids: Vec<&str> = corpus["requestWindows"].as_array().unwrap().iter().map(|row| row["window"].as_str().unwrap()).collect();
    let mut window_owners = fixture_windows(&ids).await;
    for row in corpus["requestWindows"].as_array().unwrap() {
        let family = row["family"].as_u64().unwrap(); let index = row["index"].as_u64().unwrap() as usize;
        let source = if row["namespace"] == 1 { Fem3dResultSource::Combination(0) } else { Fem3dResultSource::Case(usize::from(row["source"] == "b")) };
        let request = Fem3dResultRequest { source, mode: (family == 1).then_some(index), buckling: (family == 2).then_some(index) };
        for _ in 0..128 { fixture_prepare_result(render, &doc, &window_owners, row["window"].as_str().unwrap(), request); }
    }
    MOUNTED.with(|registry| { let registry = registry.borrow(); assert_eq!((0..ACTIVE_CAPACITY).filter(|index| registry.requests.at(*index).is_some_and(|(_, scope, _)| scope == identity.app_instance_id)).count(), 6); let a = registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-a")).unwrap(); assert_eq!(registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-a-shared")), Some(a)); assert_eq!(registry.requests.references(a), 2); });
    doc.load_cases.reverse(); MOUNTED.with(|registry| { let registry = registry.borrow(); let owner = registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-b")).unwrap(); assert_eq!(resolve_request(&registry, &doc, owner).unwrap().source, Fem3dResultSource::Case(0)); });
    let windows: Vec<semio_plugin::ViewWindowInstance> = corpus["requestWindows"].as_array().unwrap().iter().filter(|row| row["window"] != "case-a").map(|row| semio_plugin::ViewWindowInstance { id: row["window"].as_str().unwrap().into(), window_kind_id: semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS.into() }).collect();
    let departed = fixture_consumer(&window_owners, "case-a");
    MOUNTED.with(|registry| { let mut registry = registry.borrow_mut(); let step = registry.requests.remove_consumer_controlled(render.app_instance_id, departed, TEST_GRANT).unwrap(); assert!(step.progress.fits(TEST_GRANT)); });
    MOUNTED.with(|registry| { let registry = registry.borrow(); let owner = registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-a-shared")).unwrap(); assert_eq!(registry.requests.references(owner), 1); assert!(registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, "case-a")).is_none()); });
    for _ in 0..512 { let step = prepare_snapshot_admission(render, &doc, TEST_GRANT); assert!(step.progress.fits(TEST_GRANT)); }
    MOUNTED.with(|registry| { let registry = registry.borrow(); assert_eq!(registry.pending.iter().flatten().filter(|pending| pending.render.app_instance_id == identity.app_instance_id && pending.preflight.stage == SnapshotPreflightStage::Complete).count(), 6); });
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_CLOSE_COPY_BYTES, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES, maximum_depth: SESSION_CLOSE_DEPTH, ..PAGE_GRANT };
    for _ in 0..512 { if terminal_is_empty(identity.app_instance_id) { break; } lifecycle_close(identity.app_instance_id, grant); }
    assert!(terminal_is_empty(identity.app_instance_id));
    window_owners.fixture_close();
}

#[semio_framework_async_macros::async_test]
async fn fem3d_wide_physical_request_and_window_text_use_shared_granted_custody() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    let source = "a".repeat(corpus["wideSourceLength"].as_u64().unwrap() as usize); let window = "w".repeat(corpus["wideWindowLength"].as_u64().unwrap() as usize);
    let mut doc = Doc::default(); doc.load_cases = vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase { id: source.clone(), name: "Wide".into(), loads: vec![], self_weight: false }];
    let mut window_owners = fixture_windows(&[window.as_str()]).await;
    let identity = recovery_identity(9167, 43); let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 65536, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES.max(65536), maximum_depth: SESSION_CLOSE_DEPTH };
    let mut render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.base_revision, generation: identity.generation, canonical_base_revision: identity.canonical_base_revision, mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: RetainedCloneGrant { maximum_items: 0, ..grant }, maintenance: grant, close: grant } };
    let request = Fem3dResultRequest { source: Fem3dResultSource::Case(0), ..Default::default() };
    fixture_prepare_result(render, &doc, &window_owners, &window, request); MOUNTED.with(|registry| assert!(registry.borrow().app_slots.lookup(identity.app_instance_id).is_none()));
    render.mounted_policy.preparation = grant;
    for _ in 0..512 { fixture_prepare_result(render, &doc, &window_owners, &window, request); }
    MOUNTED.with(|registry| { let registry = registry.borrow(); let owner = registry.requests.binding(identity.app_instance_id, fixture_consumer(&window_owners, &window)).unwrap(); assert!(registry.requests.ready(owner)); assert_eq!(registry.requests.request_name(owner), Some(source.as_str())); assert_eq!(resolve_request(&registry, &doc, owner), Some(request)); let index = registry.requests.consumer_index(identity.app_instance_id, fixture_consumer(&window_owners, &window)).unwrap(); assert_eq!(registry.requests.consumer_name(index), Some(window.as_str())); });
    for _ in 0..128 { if terminal_is_empty(identity.app_instance_id) { break; } lifecycle_close(identity.app_instance_id, grant); }
    assert!(terminal_is_empty(identity.app_instance_id)); assert_eq!(doc.load_cases[0].id, source);
    window_owners.fixture_close();
}

#[test]
fn fem3d_original_fault_and_cancel_terminal_receipts_prevent_implicit_resubmit() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎞️results/🔣️.json")).unwrap();
    for (ordinal, kind) in corpus["terminalKinds"].as_array().unwrap().iter().enumerate() {
        let identity = recovery_identity(9239 + ordinal as u32, 47 + ordinal as u64);
        let (shell, recovery) = MOUNTED.with(|registry| { let mut registry = registry.borrow_mut(); let (shell, recovery) = reserve_recovery_state(&mut registry, identity); let owner = registry.shell_request[shell as usize].unwrap(); assert!(registry.requests.begin_execution(owner, identity_version(identity), identity.job)); (shell, recovery) });
        *MOUNTED.with(|registry| registry.borrow().shells[shell as usize].clone()).borrow_mut() = Some(recoverable_state(identity, recovery.clone()));
        if kind == "faulted" { MOUNTED.with(|registry| { let child = registry.borrow().shells[shell as usize].clone(); let step = child.borrow_mut().as_mut().unwrap().fail(b"original-fault".to_vec()); let JobStep::Failed(body) = step else { panic!("original fault outcome") }; assert_eq!(body, b"original-fault"); }); }
        let job = MountedJob { shell: MOUNTED.with(|registry| registry.borrow().shells[shell as usize].clone()), recovery, identity, completed: false, input: None, restored: None };
        if kind == "cancelled" { let mut job = job; job.cancel(); drop(job); } else { drop(job); }
        MOUNTED.with(|registry| drain_recovery_state(&mut registry.borrow_mut(), identity, shell));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_CLOSE_COPY_BYTES, maximum_capacity_bytes: 65536, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES.max(65536), maximum_depth: SESSION_CLOSE_DEPTH };
        let mut render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.base_revision, generation: identity.generation, canonical_base_revision: identity.canonical_base_revision, mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
        let doc = Doc::default();
        let windows = semio_plugin::WindowConfigOwnerRegistry::new(protocol::ActorId("terminal-fixture".into()));
        for _ in 0..64 { assert!(!prepare_snapshot_read(render, &doc, &windows, None, TEST_GRANT).ready); }
        assert_eq!(result_stage(Some(render), None), Some(if kind == "faulted" { "fem3d.failed" } else { "fem3d.cancelled" }));
        MOUNTED.with(|registry| assert!(registry.borrow().pending.iter().all(Option::is_none)));
        render.canonical_base_revision[31] ^= 1;
        let mut admitted = false; for _ in 0..512 { if prepare_snapshot_read(render, &doc, &windows, None, TEST_GRANT).ready { admitted = true; break; } }
        assert!(admitted, "a new exact canonical source may reopen after terminal outcome");
        for _ in 0..512 { if terminal_is_empty(identity.app_instance_id) { break; } lifecycle_close(identity.app_instance_id, grant); }
        assert!(terminal_is_empty(identity.app_instance_id));
    }
}

fn freshness(generation: u64) -> Fem3dVisualFreshness {
    Fem3dVisualFreshness { app_instance_id: 7, model_revision: 11, document_generation: generation, operation: 13, numerical_preview_sequence: 17, surface_generation: generation, renderer_scene_generation: generation }
}

fn node_row(id: String) -> semio_s_fem_node::volumetric::FemNodeValue {
    semio_s_fem_node::volumetric::FemNodeValue { id, x: 0.0, y: 0.0, z: 0.0 }
}

fn snapshot_with_nodes(nodes: Vec<semio_s_fem_node::volumetric::FemNodeValue>) -> Doc {
    Doc::new(Fem3dSnapshot::from_reference(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue { nodes, ..Default::default() }).expect("sealed test snapshot"))
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
    build_at(doc, solver, freshness(19))
}

fn build_at(doc: &Fem3dSnapshot, solver: &Fem3dSolverView, fresh: Fem3dVisualFreshness) -> Fem3dPageVisualLease {
    let mut job = Fem3dPageVisualJob::new(fresh, credit(doc));
    let mut backing = Fem3dBackingCredit::new();
    let mut turns = 0;
    while !job.step_one(doc, solver, &mut backing, fresh).expect("one production opportunity") && turns < 4_096 {
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
    let doc = snapshot_with_nodes((0..=MAXIMUM_NODES).map(|index| node_row(format!("n{index}"))).collect());
    let producer = doc.nodes.len();
    let credit = Fem3dPageCredit { item_count: 3, byte_count: 7, draw_count: 0, draw_bytes: 3 };
    let mut job = Fem3dPageVisualJob::new(freshness(19), credit);
    let solver = Fem3dSolverView::new(freshness(19), 0);
    let mut backing = Fem3dBackingCredit::new();
    assert!(job.step_one(&doc, &solver, &mut backing, freshness(19)).is_err());
    assert_eq!(producer, doc.nodes.len());
    assert_eq!(job.stage(), Fem3dVisualJobStage::ReserveSnapshot);
}

#[test]
fn fem3d_snapshot_preflight_page_maximum_plus_one_returns_exact_producer() {
    let doc = snapshot_with_nodes(vec![node_row("n".repeat(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY + 1))]);
    let producer = doc.nodes.len();
    let mut preflight = SnapshotPreflight::new();
    let before = preflight;
    assert_eq!(preflight.step_one(&doc), Err(()));
    assert_eq!(producer, doc.nodes.len());
    assert_eq!(preflight.item_count, before.item_count);
    assert_eq!(preflight.byte_count, before.byte_count);
}

#[test]
fn fem3d_solver_nonzero_generation_corresponds_to_every_published_field_page() {
    let doc = snapshot_with_nodes(vec![semio_s_fem_node::volumetric::FemNodeValue { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 }]);
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
    let doc = Doc::default();
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
    let doc = Doc::default();
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
fn fem3d_numerical_paged_owner_maximum_plus_one_refuses_unchanged() {
    let started = std::time::Instant::now();
    let mut ids = PagedSlots::<String, 1>::new();
    assert_eq!(ids.admit_one(2, MOUNTED_ANALYSIS_BACKING_BYTES), Err(()));
    assert!(ids.terminal_is_empty());
    let mut allocated = 0;
    loop {
        let (complete, bytes) = ids.admit_one(1, MOUNTED_ANALYSIS_BACKING_BYTES).unwrap();
        allocated += bytes;
        if complete { break; }
    }
    assert!(allocated > 0);
    assert_eq!(ids.push("kept".into()), Ok(()));
    let rejected = "returned".to_owned();
    let rejected_pointer = rejected.as_ptr();
    let returned = ids.push(rejected).expect_err("full paged owner returns producer");
    assert_eq!(returned.as_ptr(), rejected_pointer);
    assert_eq!(ids.len(), 1);
    assert_eq!(ids.pop().as_deref(), Some("kept"));
    let mut released = 0;
    loop {
        let (closed, _, bytes) = ids.close_step(MOUNTED_ANALYSIS_BACKING_BYTES);
        released += bytes;
        if closed { break; }
    }
    assert_eq!(released, allocated);

    let mut model = MountedAnalysisModel::new();
    assert_eq!(model.admit_node_one(MAXIMUM_FIELDS + 1, MOUNTED_ANALYSIS_BACKING_BYTES), Err(semio_s_fem_analyses_model::MountedAnalysisFault::Capacity { requested: MAXIMUM_FIELDS + 1, maximum: semio_s_fem_analyses_model::MOUNTED_ANALYSIS_NODE_SLOTS }));
    assert_eq!(model.nodes_len(), 0);
    let node = Node { id: "returned-node".into(), pos: [0.0; 3] };
    let node_pointer = node.id.as_ptr();
    let returned = model.push_node(node).expect_err("unadmitted node returns producer");
    assert_eq!(returned.id.as_ptr(), node_pointer);

    let mut domain = MountedPlanarDomain::new();
    let maximum = semio_s_fem_mesh::MOUNTED_DOMAIN_POINT_SLOTS;
    assert_eq!(domain.admit_outer_one(maximum + 1), Err(semio_s_fem_mesh::MountedDomainFault::PointCapacity { requested: maximum + 1, maximum }));
    assert_eq!(domain.push_outer([1.0, 2.0]), Err([1.0, 2.0]));

    let mut scalars = MountedScalarSlots::new();
    let maximum = semio_s_fem_sparse::MOUNTED_SCALAR_SLOTS;
    assert_eq!(scalars.admit_one(maximum + 1, MOUNTED_ANALYSIS_BACKING_BYTES), Err(semio_s_fem_sparse::MountedScalarFault::Capacity { requested: maximum + 1, maximum }));
    assert_eq!(scalars.len(), 0);
    assert_eq!(scalars.push(17.0), Err(17.0));
    assert!(started.elapsed().as_micros() < 8_000);
}


/// 🏠️ The shipped demo crosses physical order pages while numerical and visual publication remain progressive.
#[test]
fn fem3d_production_numerical_child_solves_the_demo_with_paged_owner_backing() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📚️paged-demo/🔣️.json")).expect("independent NumPy demo equilibrium");
    let doc = Doc::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("the demo parses"));
    assert!(!doc.solids.is_empty(), "the demo carries a meshed slab");
    let (analysis_nodes, _) = semio_s_artifact_fem_3d_engine::fem3d_engine::meshing::mesh_solids(&doc).expect("the demo meshes");
    assert!(analysis_nodes.len() * 6 * size_of::<(String, Dof)>() > 4_096, "the demo's dof order outgrows the mounted owner page — the premise of this law");
    let operation = semio_framework_job::Operation::new(OperationId(31), RevisionId(7), Generation(3), 5);
    let mut child = Fem3dNumericalChild::new();
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(3), doc.nodes.len());
    let bank = numerical_bank();
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation, operation.generation, bank, || Some(1000), 1 << 40, 1 << 40);
    let mut terminal = false;
    let mut failure = None;
    for _ in 0..16_000_000 {
        let mut outcome = None;
        caller.turn(TEST_GRANT, |cx| { outcome = Some(child.step(&doc, &mut fields, &mut backing, freshness(3), operation, cx)); Ok(()) }).expect("original numerical turn");
        match outcome.expect("the original numerical turn ran") {
            Ok(complete) => terminal = complete,
            Err(fault) => failure = Some((child.stage, format!("{fault:?}"))),
        }
        if terminal || failure.is_some() {
            break;
        }
    }
    let mounted_ids: Vec<String> = (0..child.analysis_node_ids.len()).filter_map(|index| child.analysis_node_ids.get(index).cloned()).collect();
    let stress: Vec<_> = (0..mounted_ids.len()).map(|index| child.stress(index)).collect();
    for _ in 0..200_000 {
        if close_in(&mut caller, &mut child, TEST_GRANT) {
            break;
        }
    }
    assert!(child.terminal_is_empty(), "the demo releases every numerical owner");
    assert!(failure.is_none(), "paged demo failed at {failure:?}");
    assert!(terminal && fields.ready(), "the mounted demo reaches a validated numerical publication");
    let results = semio_s_artifact_fem_3d_engine::fem3d_engine::fem3d_solve_all(&doc).expect("the synchronous path solves the same document");
    let expected = results.get(&semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").unwrap()).expect("demo first case");
    for (index, node) in doc.nodes.iter().enumerate() {
        let actual = fields.scalar(index).expect("published demo scalar");
        let displacement = expected.displacements.iter().find(|row| row.node_id == *node.id.get()).expect("reference displacement");
        for axis in 0..3 {
            assert!((actual.displacement[axis] - displacement.values[axis]).abs() <= 1e-7 * displacement.values[axis].abs().max(1.0), "demo displacement {}/{axis}: {} != {}", node.id.get(), actual.displacement[axis], displacement.values[axis]);
        }
    }
    for (index, node) in doc.nodes.iter().enumerate() {
        let actual = fields.scalar(index).unwrap();
        if node.id.ends_with("_l1") || node.id.ends_with("_l2") {
            let storey = usize::from(node.id.ends_with("_l2"));
            let expected = oracle["frame"]["storeyVerticalDisplacements"][storey].as_f64().unwrap();
            assert!((actual.displacement[2] - expected).abs() <= 1e-8, "independent gravity displacement {}: {} != {expected}", node.id.get(), actual.displacement[2]);
        }
        if node.id.as_str() == "n00_l1" {
            for axis in 0..2 {
                let expected = oracle["frame"]["firstStoreyCornerLateralDisplacement"][axis].as_f64().unwrap();
                assert!((actual.displacement[axis] - expected).abs() <= 1e-8, "independent lateral displacement {axis}: {} != {expected}", actual.displacement[axis]);
            }
        }
        if node.id.as_str() == "sc0" {
            let expected = oracle["slab"]["cornerVonMisesStress"].as_f64().unwrap();
            assert!((actual.contour - expected).abs() <= 1e-5 * expected.abs(), "independent slab corner stress: {} != {expected}", actual.contour);
        }
        if node.id.ends_with("_g") {
            let expected = oracle["frame"]["rootVerticalReaction"].as_f64().unwrap();
            assert!((actual.reaction[2] - expected).abs() <= 1e-3, "independent frame reaction {}", node.id.get());
        }
    }
    let slab_reaction: f64 = doc.nodes.iter().enumerate().filter(|(_, node)| node.id.starts_with("sc")).map(|(index, _)| fields.scalar(index).unwrap().reaction[2]).sum();
    assert!((slab_reaction - oracle["slab"]["verticalReactionSum"].as_f64().unwrap()).abs() <= 1e-3);
    let expected_stress = semio_s_artifact_fem_3d_engine::fem3d_engine::mesh_preview::fem3d_nodal_von_mises_of(&doc, expected).unwrap();
    for node in analysis_nodes.iter() {
        if let Some(expected) = expected_stress.get(&node.id) { let mounted = mounted_ids.iter().position(|id| *id == node.id).expect("mounted analysis node"); assert!((stress[mounted].unwrap() - expected).abs() < 1e-5 * expected.abs().max(1.0), "demo stress {}: {:?} != {expected}", node.id, stress[mounted]); }
    }
    let mut visual = build_at(&doc, &fields, freshness(3));
    assert_eq!(close(&mut visual), FEM3D_VISUAL_PAGES);
    caller.retire(bank);
}

/// 🌲️ Concrete Forest publishes the dead-load case for its 20-node frame.
#[test]
fn fem3d_production_numerical_child_solves_concrete_forest() {
    assert_mounted_example("concreteForest", include_str!("../../../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio"));
}

/// 🏠️ The House solver preserves the authored solid resolution while publishing dead load.
#[test]
fn fem3d_production_numerical_child_solves_house() {
    assert_mounted_example("house", include_str!("../../../../🖼️assets/🏠️house/🗣️.dsl.semio"));
}

fn assert_mounted_example(name: &str, text: &str) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🏘️mounted-examples.json")).unwrap();
    let parsed = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).unwrap();
    let doc = Doc::new(parsed);
    let expected = &fixture[name];
    let operation = semio_framework_job::Operation::new(OperationId(41), RevisionId(7), Generation(3), 5);
    let mut child = Fem3dNumericalChild::new();
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(3), doc.nodes.len());
    let bank = numerical_bank();
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation, operation.generation, bank, || Some(1000), 1 << 40, 1 << 40);
    let mut outcome = None;
    let trace = std::env::var_os("SEMIO_FEM_MOUNT_TRACE").is_some();
    let mut previous_stage = child.stage;
    for turn in 0..16_000_000 {
        caller.turn(TEST_GRANT, |context| {
            match child.step(&doc, &mut fields, &mut backing, freshness(3), operation, context) {
                Ok(false) => {},
                Ok(true) => outcome = Some(Ok(())),
                Err(error) => outcome = Some(Err((child.stage, format!("{error:?}")))),
            }
            Ok(())
        }).unwrap();
        if trace && (turn % 100_000 == 0 || child.stage != previous_stage) {
            eprintln!("[DEBUG] {name} turn {turn} stage {:?}", child.stage);
            previous_stage = child.stage;
        }
        if outcome.is_some() { break; }
    }
    let converged = fields.ready();
    let node_count = fields.len;
    let vertical_reaction: f64 = doc.nodes.iter().enumerate()
        .filter_map(|(index, _)| fields.scalar(index).map(|row| row.reaction[2]))
        .sum();
    for _ in 0..4_000_000 {
        if close_in(&mut caller, &mut child, TEST_GRANT) { break; }
    }
    assert!(child.terminal_is_empty(), "{name} releases its numerical owner after {outcome:?}");
    assert!(matches!(outcome, Some(Ok(()))), "{name} ended at {:?} with {outcome:?}", child.stage);
    assert!(converged, "{name} has validated numerical output");
    assert_eq!(node_count, expected["analysisNodes"].as_u64().unwrap() as usize, "{name} analysis node count");
    let expected_reaction = expected["verticalReaction"].as_f64().unwrap();
    assert!((vertical_reaction - expected_reaction).abs() < 1e-6 * expected_reaction,
            "{name} reaction {vertical_reaction} != {expected_reaction}");
    loop {
        let (closed, items, bytes) = fields.close_step(MOUNTED_ANALYSIS_BACKING_BYTES);
        assert!(backing.release(items, bytes));
        if closed { break; }
    }
    assert!(backing.terminal_is_empty());
    caller.retire(bank);
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

    let doc = Doc::default();
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
    let request = registry.requests.admit(identity.app_instance_id, ConsumerKey::Model, request_key(&Doc::default(), Fem3dResultRequest::default()).unwrap()).unwrap().owner;
    let slot = request.index();
    registry.shell_request[shell as usize] = Some(request);
    registry.current[slot] = Some(Current { app_instance_id: identity.app_instance_id, shell, identity });
    (shell, recovery)
}

fn drain_recovery_state(registry: &mut Registry, identity: Identity, shell: u16) {
    for _ in 0..128 {
        registry.recovery_cursor = shell as usize;
        let step = recover_one(registry, identity.app_instance_id).unwrap_or(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
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
    let _ = recover_one(&mut registry, queued_identity.app_instance_id);
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
            let owner = registry.requests.admit(app, ConsumerKey::Model, request_key(&Doc::default(), Fem3dResultRequest::default()).unwrap()).unwrap().owner;
            let slot = owner.index();
            assert!(registry.pending[slot].is_none());
            registry.pending[slot] = pending.map(|app_instance_id| PendingSnapshot {
                render: AppRenderOperationContext { app_instance_id, base_revision: RevisionId(1), generation: Generation(1), canonical_base_revision: [1; 32], mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, maintenance: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, close: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 } } },
                preflight: SnapshotPreflight::new(),
            });
            if pending.is_none() { registry.requests.remove_one_scoped_consumer(app); assert!(registry.requests.release(owner)); let lease = registry.app_slots.lookup(closing).unwrap(); assert!(registry.app_slots.release(closing, lease)); }
            slot
        });
        let first = lifecycle_close(closing, CLOSE_GRANT);
        if pending == Some(closing) {
            assert_eq!(first, session_pending(1, 0));
            for _ in 0..16 { if terminal_is_empty(closing) { break; } lifecycle_close(closing, CLOSE_GRANT); }
        } else {
            assert_eq!(first, PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
        }
        assert!(terminal_is_empty(closing));
        let remaining = MOUNTED.with(|registry| registry.borrow().pending[slot].as_ref().map(|pending| pending.render.app_instance_id));
        assert_eq!(serde_json::json!({ "pendingInstanceId": remaining }), row["expected"], "{}", row["id"]);
        if let Some(remaining) = remaining {
            assert!(!terminal_is_empty(remaining));
            for _ in 0..16 { if terminal_is_empty(remaining) { break; } lifecycle_close(remaining, CLOSE_GRANT); }
            assert_eq!(lifecycle_close(remaining, CLOSE_GRANT), PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
        }
    }
}

#[test]
fn fem3d_visual_close_grants_retain_exact_backing_until_admitted() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧪️tests/🧫️fixtures/💳️visual-close-grants/🔣️.json")).expect("neutral close grants");
    for row in corpus["cases"].as_array().expect("cases") {
        let doc = Doc::default();
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
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🏷️sources/🔣️.json")).unwrap();
    let base = Doc::new(semio_framework_pack_json::from_json_str(include_str!("../../../../../📈️analysis/🧪️tests/🧫️fixtures/🧮️solves-fem3d-1-benchmarks/📐️cantilever-tip-load-local-y.snapshot.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
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
            let bank = numerical_bank();
            let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation, operation.generation, bank, || Some(1000), 1 << 40, 1 << 40);
                    let mut complete = false;
            let mut fault = None;
            for _ in 0..500_000 {
                let mut outcome = None; let mut stage = "initial";
                caller.turn(TEST_GRANT, |cx| { outcome = Some(child.step(&doc, &mut fields, &mut backing, freshness(19), operation, cx)); stage = cx.stage(); Ok(()) }).expect("original numerical turn");
                match outcome.expect("the original numerical turn ran") { Ok(done) => complete = done, Err(error) => fault = Some(format!("{error:?}")) }
                assert_ne!(stage, "initial", "the original numerical owner publishes its actual stage through shared job progress");
                if complete || fault.is_some() { break; }
            }
            let tip = fields.scalar(1).map(|scalar| scalar.displacement[1]);
            let reaction = fields.scalar(0).map(|scalar| scalar.reaction[1]);
            let moment = child.reactions(0).expect("mounted support reaction")[5];
            let rotation = child.equation(1, 5).and_then(|equation| child.pcg.as_ref()?.visual_scalar(equation)).map(|scalar| scalar.displacement);
            for _ in 0..200_000 { if close_in(&mut caller, &mut child, TEST_GRANT) { break; } }
            assert!(child.terminal_is_empty() && fault.is_none() && complete && fields.ready());
            let factor = row["factor"].as_f64().unwrap();
            assert!((tip.unwrap() - factor * corpus["expected"]["tipTy"].as_f64().unwrap()).abs() < 1e-9);
            assert!((reaction.unwrap() - factor * corpus["expected"]["rootTyReaction"].as_f64().unwrap()).abs() < 1e-5);
            assert!((moment - factor * corpus["expected"]["rootRzReaction"].as_f64().unwrap()).abs() < 1e-5);
            assert!((rotation.unwrap() - factor * corpus["expected"]["tipRz"].as_f64().unwrap()).abs() < 1e-9);
            caller.retire(bank);
        }
    }
}
#[path = "../🪪️render-revision/🦀️.rs"]
mod render_revision_tests;

#[test]
fn fem3d_source_preflight_rejects_ambiguous_missing_nonfinite_and_excess_sources() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🏷️sources/🔣️.json")).unwrap();
    assert_eq!(corpus["maximumSources"].as_u64().unwrap() as usize, MAXIMUM_LOADS);
    let base = Doc::new(semio_framework_pack_json::from_json_str(include_str!("../../../../../📈️analysis/🧪️tests/🧫️fixtures/🧮️solves-fem3d-1-benchmarks/📐️cantilever-tip-load-local-y.snapshot.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    for refusal in corpus["refusals"].as_array().unwrap() {
        let mut doc = base.clone();
        doc.combinations.push(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id: "tip".into(), name: "Combined".into(), terms: [("tip".into(), 1.0)].into_iter().collect() });
        match refusal.as_str().unwrap() {
            "duplicate-case" => { let first = doc.load_cases[0].clone(); doc.load_cases.push(first) }
            "duplicate-combination" => { let first = doc.combinations[0].clone(); doc.combinations.push(first) }
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
        assert!(rejected && preflight.credit().is_err());
    }
}
