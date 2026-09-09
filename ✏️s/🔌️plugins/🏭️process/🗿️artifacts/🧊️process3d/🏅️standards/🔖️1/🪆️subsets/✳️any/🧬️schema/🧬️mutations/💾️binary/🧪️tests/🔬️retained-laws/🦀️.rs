use super::*;

fn every_mutation() -> Vec<Process3dMutation> {
    process3d_all_retained_mutation_fixtures_for_test()
}

fn authority_fixture() -> Process3dPublicationLease {
    Process3dPublicationLease {
        operation: u64::MAX - 313,
        generation: 51,
        base_revision: 51,
        parent_revision: 51,
        live_revision: 51,
        maximum_items: PROCESS3D_MAXIMUM_DOMAIN_ITEMS,
        maximum_output_pages: PROCESS3D_MOUNTED_OUTPUT_CHANNELS,
        maximum_controls: PROCESS3D_MOUNTED_CONTROL_CREDITS,
        closing: false,
        terminal: false,
    }
}

fn close_store(mut store: store::ArtifactStore<Process3dSnapshot, Process3dMutation>) {
    use semio_framework_plugin::ArtifactOwnedDisposer;
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<Process3dSnapshot, Process3dMutation>::new();
    for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        if matches!(disposer.close_step(&mut store, 1, PROCESS3D_OWNER_BYTES), Ok(semio_framework_plugin::PluginCloseStep::Complete)) {
            break;
        }
    }
    assert!(disposer.terminal_is_empty(&store));
    drop(store);
}

fn owned_store(label: &str, operation_value: u64) -> store::ArtifactStore<Process3dSnapshot, Process3dMutation> {
    let operation = semio_framework_job::OperationId(operation_value);
    let generation = semio_framework_job::Generation(51);
    process3d_admit_publication_authority(operation, generation, generation.0, generation.0, generation.0, PROCESS3D_MAXIMUM_DOMAIN_ITEMS, PROCESS3D_MOUNTED_OUTPUT_CHANNELS, PROCESS3D_MOUNTED_CONTROL_CREDITS).expect("fixture publication authority");
    let mut snapshot = crate::empty_process3d_snapshot();
    snapshot.stock_label = label.into();
    let envelope = store::create_document_envelope(crate::PROCESS_3D_SCHEMA, label, snapshot, None);
    let mut authority = Process3dStoreInitializationAuthority::new(envelope, operation, generation);
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut complete = false;
    for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        match semio_framework_plugin::ArtifactStoreInitializationAuthority::step(&mut authority, &mut context) {
            semio_framework_job::StepOutcome::Complete(_) => {
                complete = true;
                break;
            }
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => {}
            semio_framework_job::StepOutcome::Cancelled => panic!("fixture initializer cancelled"),
            semio_framework_job::StepOutcome::Fault(_) => panic!("fixture initializer faulted"),
        }
    }
    assert!(complete, "fixture initializer must converge");
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("fixture candidate handoff");
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    assert!(process3d_release_publication_authority(operation, generation));
    candidate
}

#[test]
fn actual_atomic_publication_is_fail_closed_and_retires_stale_candidate() {
    let authority = authority_fixture();
    let operation = semio_framework_job::OperationId(authority.operation);
    let generation = semio_framework_job::Generation(authority.generation);
    assert_eq!(process3d_validate_atomic_lease(Process3dPublicationLease { operation: authority.operation + 1, ..authority }, operation, generation, generation), Err("process3d-publication.wrong-operation"));
    assert_eq!(process3d_validate_atomic_lease(Process3dPublicationLease { generation: authority.generation + 1, ..authority }, operation, generation, generation), Err("process3d-publication.wrong-generation"));
    assert_eq!(process3d_validate_atomic_lease(Process3dPublicationLease { base_revision: authority.base_revision - 1, ..authority }, operation, generation, generation), Err("process3d-publication.wrong-base"));
    assert_eq!(process3d_validate_atomic_lease(Process3dPublicationLease { parent_revision: authority.parent_revision - 1, ..authority }, operation, generation, generation), Err("process3d-publication.wrong-parent"));

    let mut live = owned_store("last-valid", u64::MAX - 316);
    let stale = owned_store("stale-candidate", u64::MAX - 315);
    let accepted = owned_store("accepted-candidate", u64::MAX - 314);

    let stale = match semio_framework_plugin::publish_document_store_candidate_if_authoritative(&mut live, stale, || {
        process3d_validate_atomic_lease(Process3dPublicationLease { parent_revision: authority.parent_revision - 1, ..authority }, operation, generation, generation)
            .map_err(|code| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), "hostile stale publication"))
    }) {
        Err((_fault, stale)) => stale,
        Ok(displaced) => {
            close_store(displaced);
            panic!("wrong parent swapped the stale candidate")
        }
    };
    assert_eq!(live.snapshot_root().stock_label, "last-valid");
    close_store(stale);

    let displaced = match semio_framework_plugin::publish_document_store_candidate_if_authoritative(&mut live, accepted, || {
        process3d_validate_atomic_lease(authority, operation, generation, generation).map_err(|code| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), "valid publication"))
    }) {
        Ok(displaced) => displaced,
        Err((_fault, rejected)) => {
            close_store(rejected);
            panic!("fresh authority rejected the accepted candidate")
        }
    };
    assert_eq!(live.snapshot_root().stock_label, "accepted-candidate");
    close_store(displaced);
    close_store(live);
}

#[test]
fn owner_census_rejects_zero_depth_and_maximum_plus_one() {
    let mut zero = Process3dOwnerTotals::default();
    assert_eq!(zero.admit(0, 0, 0), Ok(()));

    let mut exact = Process3dOwnerTotals::default();
    assert_eq!(exact.admit(PROCESS3D_MAXIMUM_DOMAIN_ITEMS, PROCESS3D_MAXIMUM_DOMAIN_BYTES, PROCESS3D_RETAINED_STACK_CAPACITY - 1), Ok(()));

    let mut item_overflow = Process3dOwnerTotals::default();
    assert_eq!(item_overflow.admit(PROCESS3D_MAXIMUM_DOMAIN_ITEMS + 1, 0, 0), Err("process3d-owner.items-capacity"));
    let mut byte_overflow = Process3dOwnerTotals::default();
    assert_eq!(byte_overflow.admit(0, PROCESS3D_MAXIMUM_DOMAIN_BYTES + 1, 0), Err("process3d-owner.bytes-capacity"));
    let mut depth_overflow = Process3dOwnerTotals::default();
    assert_eq!(depth_overflow.admit(0, 0, PROCESS3D_RETAINED_STACK_CAPACITY), Err("process3d-owner.combined-depth"));
}

#[test]
fn interrupted_snapshot_close_reaches_terminal_empty() {
    let mut owner = Process3dOwnedRetirement::snapshot(crate::empty_process3d_snapshot());
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut owner, 0, 0), Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })));
    for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        if matches!(store::ErasedSnapshotRetirement::close_step(&mut owner, 1, PROCESS3D_OWNER_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
            break;
        }
    }
    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&owner));
}

#[test]
fn deterministic_ledger_digest_is_replay_stable() {
    let mutation = Process3dMutation::ChangeCursor(crate::mutations::change_cursor::ChangeCursor { new_resolved_up_to: Some(7) });
    let mut left = store::ArtifactStoreInitializationDigest::new(b"process3d.fixture");
    let mut right = store::ArtifactStoreInitializationDigest::new(b"process3d.fixture");
    process3d_observe_mutation(&mut left, &mutation);
    process3d_observe_mutation(&mut right, &mutation);
    assert_eq!(left.finish(), right.finish());
}

#[test]
fn mounted_mutation_region_has_zero_whole_string_reader_edges() {
    let source = include_str!("../../🦀️.rs");
    let retained = source.split_once("enum Process3dRetainedMutationPhase").expect("retained mutation region start").1.split_once("enum Process3dMutationDecodeState").expect("retained mutation region end").0;
    assert_eq!(retained.matches(concat!("read_str", "_lp")).count(), 0, "mounted mutation reader must have no whole-string edge");
}

#[test]
fn every_mutation_uses_retained_grants_and_incremental_terminal_retirement() {
    let operation = semio_framework_job::OperationId(u64::MAX - 91);
    let generation = semio_framework_job::Generation(23);
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mutations = every_mutation();
    assert_eq!(mutations.len(), PROCESS3D_MUTATION_VARIANT_COUNT);

    for mutation in mutations {
        let bytes = encode_op(&mutation).expect("mutation fixture encoding");
        let mut reader = Process3dRetainedMutationReader::new();
        let mut grants = 0;
        loop {
            grants += 1;
            let mut grant = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            if reader.step(&bytes, &mut grant).expect("retained semantic grant") {
                break;
            }
            assert!(grants <= bytes.len() + PROCESS3D_RETAINED_STACK_CAPACITY, "retained cursor stopped advancing");
        }
        assert!(grants > 2, "a mutation crossed the mounted route without field-level suspension");
        let decoded = reader.take().expect("exact mutation handoff");
        assert_eq!(decoded, mutation);
        assert!(reader.take().is_none());
        drop(reader);

        let mut retirement = Process3dOwnedRetirement::mutation(decoded);
        for _ in 0..128 {
            if matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, PROCESS3D_OWNER_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
                break;
            }
        }
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));

        for interruption in 1..grants {
            let mut interrupted = Process3dRetainedMutationReader::new();
            for _ in 0..interruption {
                let mut grant = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                assert!(!interrupted.step(&bytes, &mut grant).expect("interrupted retained semantic grant"), "pre-terminal interruption must remain resumable");
                assert_eq!(grant.fuel_remaining(), 0, "one retained mutation substate consumes one grant");
            }
            let partial = interrupted.take_rejected().expect("every interrupted mutation substate hands back its exact partial owner");
            assert!(interrupted.terminal_is_empty());
            drop(interrupted);
            let mut retirement = Process3dOwnedRetirement::mutation(partial);
            for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                if matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, PROCESS3D_OWNER_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
                    break;
                }
            }
            assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
            drop(retirement);
        }
    }
}
