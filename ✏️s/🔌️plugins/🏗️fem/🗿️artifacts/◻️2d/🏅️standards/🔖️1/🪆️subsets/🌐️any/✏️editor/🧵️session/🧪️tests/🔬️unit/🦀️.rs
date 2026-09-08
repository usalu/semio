
use super::*;

#[test]
fn input_authority_round_trips_and_rejects_hostile_lengths() {
    let job = FEM2D_JOB_TAG | 19;
    let identity = MountedIdentity { app_instance_id: 7, base_revision: semio_framework_job::RevisionId(11), generation: semio_framework_job::Generation(13), canonical_base_revision: [23; 32], operation: OperationId(job), job };
    assert_eq!(decode_input(job, &encode_input(3, identity)), Some((3, identity)));
    assert_eq!(decode_input(job, &encode_input(3, identity)[..INPUT_BYTES - 1]), None);
    assert_eq!(decode_input(job + 1, &encode_input(3, identity)), None, "a host job-id substitution must not inherit the retained owner");
}

#[test]
fn mounted_close_and_capacity_are_fixed_and_terminal_witnessed() {
    let mut registry = MountedRegistry::new();
    let mut admitted = Vec::new();
    for _ in 0..SESSION_SHELL_CAPACITY {
        admitted.push(registry.allocate().expect("exact fixed slot"));
    }
    assert!(registry.allocate().is_none(), "capacity plus one must fail without growing");
    let returned = admitted.pop().expect("slot");
    registry.release(returned);
    assert_eq!(registry.allocate(), Some(returned), "fixed free ring deterministically reuses the returned slot");
    let identity = MountedIdentity { app_instance_id: 1, base_revision: semio_framework_job::RevisionId(2), generation: semio_framework_job::Generation(3), canonical_base_revision: [4; 32], operation: OperationId(5), job: 5 };
    registry.current[1] = Some(CurrentSession { app_instance_id: 1, shell: returned, identity });
    assert!(registry.current[33 % SESSION_ACTIVE_CAPACITY].filter(|current| current.app_instance_id == 33).is_none(), "a modulo collision must not inherit another app's generation authority");
    assert_eq!(maintenance_step(u32::MAX, 1, 4_096), PluginCloseStep::Complete);
    assert!(terminal_is_empty(u32::MAX));
}

#[test]
fn snapshot_lease_is_preceded_by_a_fixed_pending_admission_and_idle_polling_reuses_it() {
    let render = AppRenderOperationContext { app_instance_id: 2_000_000_007, base_revision: semio_framework_job::RevisionId(17), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32] };
    let snapshot = Fem2dSnapshot::default();
    assert!((0..64).any(|_| prepare_snapshot_read(render, &snapshot)), "empty schema census completes incrementally");
    let first = MOUNTED.with(|registry| registry.borrow().pending[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY].expect("pending admission"));
    for _ in 0..1_025 {
        assert!(prepare_snapshot_read(render, &snapshot));
    }
    let retained = MOUNTED.with(|registry| registry.borrow().pending[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY].expect("same pending admission"));
    assert_eq!(first.shell, retained.shell);
    assert_eq!(first.identity, retained.identity);
    assert!(matches!(close_step(render.app_instance_id, 1, 4_096), PluginCloseStep::Pending { released_items: 1, .. }));
    assert!(terminal_is_empty(render.app_instance_id));
}

#[test]
fn snapshot_census_advances_one_owner_and_rejects_exact_plus_one_without_partial_credit() {
    let render = AppRenderOperationContext { app_instance_id: 2_000_000_006, base_revision: semio_framework_job::RevisionId(29), generation: semio_framework_job::Generation(31), canonical_base_revision: [37; 32] };
    let snapshot = Fem2dSnapshot::default();
    assert!(!prepare_snapshot_read(render, &snapshot));
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let slot = render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        let cursor = registry.preflight[slot].expect("one retained census cursor").cursor;
        assert_eq!((cursor.lane, cursor.outer, cursor.inner, cursor.deep), (0, 0, 0, 0));
        assert!(cursor.owner_opened);
        assert!(registry.pending[slot].is_none(), "no lease shell is admitted during the first schema-owner opportunity");
    });

    let mut exact = SnapshotAdmissionCursor::new();
    exact.items = SESSION_MAXIMUM_INPUT_ITEMS - 1;
    exact.bytes = SESSION_MAXIMUM_INPUT_BYTES - 1;
    assert_eq!(exact.charge(1, 1), Ok(()));
    let before = (exact.items, exact.bytes);
    assert_eq!(exact.charge(1, 0), Err(b"fem2d.session-admission-exceeded" as &'static [u8]));
    assert_eq!((exact.items, exact.bytes), before, "plus one returns the exact unchanged census owner");
    assert!(matches!(close_step(render.app_instance_id, 1, 4_096), PluginCloseStep::Pending { released_items: 1, .. }));
    assert!(terminal_is_empty(render.app_instance_id));
}

#[test]
fn process_owner_inventory_admits_exact_maximum_and_returns_exact_credit() {
    let catalog = MountedProcessOwnerCatalog::fixed();
    let mut seen = [false; 30];
    for claim in catalog.claims {
        let index = claim.class as usize;
        assert!(!seen[index], "every simultaneous owner class is inventoried exactly once");
        assert!(claim.roots != 0, "every class retains at least one fixed backing root");
        seen[index] = true;
    }
    assert!(seen.into_iter().all(|present| present));
    let exact = catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES).expect("the enumerated working set exactly fits its process authority");
    assert_eq!(exact, (SESSION_MAXIMUM_ITEMS, SESSION_MAXIMUM_BYTES));
    assert_eq!(catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS + 1, SESSION_MAXIMUM_INPUT_BYTES), Err(b"fem2d.session-process-input-credit-exceeded" as &'static [u8]));
    assert_eq!(catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES + 1), Err(b"fem2d.session-process-input-credit-exceeded" as &'static [u8]));

    let mut overflow = catalog;
    overflow.claims[MountedOwnerClass::OutputPages as usize].roots += 1;
    assert_eq!(overflow.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES), Err(b"fem2d.session-process-admission-exceeded" as &'static [u8]));

    let mut registry = MountedRegistry::new();
    let shell = registry.allocate().expect("fixed process shell");
    assert!(registry.reserve_credit(shell, exact.0, exact.1));
    let rejected = registry.allocate().expect("distinct fixed shell retains a failed request");
    assert!(!registry.reserve_credit(rejected, exact.0 + 1, exact.1), "plus one cannot partially change the registry credit");
    assert_eq!((registry.credit_items[rejected as usize], registry.credit_bytes[rejected as usize]), (0, 0));
    registry.release(rejected);
    registry.release_credit(shell);
    registry.release(shell);
    assert_eq!((registry.reserved_items, registry.reserved_bytes), (0, 0));
    let returned: [u16; SESSION_SHELL_CAPACITY] = std::array::from_fn(|_| registry.allocate().expect("all fixed shells returned"));
    assert_eq!(&returned[SESSION_SHELL_CAPACITY - 2..], &[rejected, shell], "failed and completed admissions return their exact shells in FIFO order");
}

#[test]
fn interrupted_model_close_releases_one_retained_root_per_grant() {
    let mut build = MountedModelBuild::new(None);
    build.model.nodes.push(crate::model::Node { id: "deep-node".repeat(128), pos: [0.0; 3] });
    build.region_node_ids.push("deep-region-node".repeat(128));
    let first = build.close_step(4_096);
    assert!(!first.0);
    assert_eq!(build.model.nodes.len(), 1);
    assert_eq!(build.model.nodes[0].id.capacity(), 0, "the first grant releases only the nested string backing");
    assert_eq!(build.region_node_ids.len(), 1, "a distinct retained root survives the interrupted close");
    while !build.model.nodes.is_empty() {
        assert!(!build.close_step(4_096).0);
    }
    assert_eq!(build.region_node_ids.len(), 1, "later close lanes remain untouched while the model page retires");
    while !build.close_step(4_096).0 {}
}

#[test]
fn mounted_revision_restart_keeps_cancel_before_spawn() {
    let source = include_str!("../../🦀️.rs");
    let admission = &source[source.find("pub fn prepare_snapshot_read(").expect("admission")..source.find("pub fn reconcile(").expect("reconcile")];
    let reconcile = &source[source.find("pub fn reconcile(").expect("reconcile")..source.find("pub fn with_live_visual").expect("visual boundary")];
    assert!(reconcile.find("Effect::CancelJob").expect("cancel effect") < reconcile.find("Effect::SpawnJob").expect("spawn effect"));
    assert!(admission.contains("checked_add(1)"));
    assert!(admission.contains("reserve_credit(shell, process_items, process_bytes)"));
    assert!(reconcile.contains("take_snapshot_read()"));
    assert!(reconcile.contains("retiring.iter().any(Option::is_none)"));
}

#[test]
fn commit_identity_requires_base_revision_and_generation() {
    let operation = Operation::new(OperationId(1), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 4);
    assert_eq!(semio_framework_job::validate_commit(&operation, operation.base_revision, operation.generation), CommitValidation::Accepted);
    assert!(matches!(semio_framework_job::validate_commit(&operation, semio_framework_job::RevisionId(9), operation.generation), CommitValidation::Stale { .. }));
    assert!(matches!(semio_framework_job::validate_commit(&operation, operation.base_revision, semio_framework_job::Generation(9)), CommitValidation::Stale { .. }));
}

#[test]
fn source_contract_keeps_one_semantic_child_step_and_live_visual_consumer() {
    let source = include_str!("../../🦀️.rs");
    for needle in [
        "FemJobGraph::new",
        "MeshJob::new",
        "AssemblyJobConstruction::new_owned",
        "PcgJobConstruction::new",
        "commit_authority_matches",
        "Effect::SpawnJob",
        "Effect::CancelJob",
        "JobPlacement::Isolated",
        "with_live_visual",
        "take_snapshot_read",
        "SESSION_MAXIMUM_ITEMS",
        "SESSION_MAXIMUM_BYTES",
        "MountedProcessOwnerCatalog",
        "MountedOwnerClass::AssemblyDofStrings",
        "mounted_node_id",
        "reserve_exact_owner_page",
    ] {
        assert!(source.contains(needle), "missing mounted FEM contract {needle}");
    }
    for forbidden in ["crate::fem2d_engine::build_model(", "AssemblyJob::new_owned(", ".into_full_matrix(", "PcgJob::new(", "pending_node_ids", ".fixed.iter().map"] {
        assert!(!source.contains(forbidden), "mounted FEM route restored bulk constructor {forbidden}");
    }
}
