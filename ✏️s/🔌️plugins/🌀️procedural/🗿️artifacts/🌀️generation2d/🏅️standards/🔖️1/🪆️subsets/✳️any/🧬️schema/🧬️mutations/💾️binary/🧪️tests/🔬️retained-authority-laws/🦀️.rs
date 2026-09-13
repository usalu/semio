use super::*;

fn close_session(session: &mut Generation2dMutationSession) {
    let admitted = session.retained_allocated_bytes();
    let mut released = 0;
    let mut refused_subexact = false;
    for _ in 0..GENERATION2D_MAXIMUM_DOMAIN_ITEMS {
        let maximum_bytes = session.next_retained_release_allocation_bytes().unwrap_or(0);
        if maximum_bytes > 0 && !refused_subexact {
            let before = session.retained_allocated_bytes();
            assert_eq!(
                session.close_step(1, maximum_bytes - 1).expect("P2 subexact retained mutation close"),
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 }
            );
            assert_eq!(session.retained_allocated_bytes(), before);
            refused_subexact = true;
        }
        match session.close_step(1, maximum_bytes).expect("P2 retained mutation close") {
            store::mounted_pack_rt::RetainedPackCloseStep::Complete => {
                assert!(session.terminal_is_empty());
                assert_eq!(released, admitted);
                assert!(refused_subexact || admitted == 0);
                return;
            }
            store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_bytes, .. } => released += released_bytes,
        }
    }
    panic!("P2 retained mutation session did not close");
}

#[test]
fn every_fourteen_variant_decodes_through_retained_structural_grants() {
    let mutations = generation2d_all_retained_mutation_fixtures_for_test();
    assert_eq!(mutations.len(), GENERATION2D_MUTATION_VARIANT_COUNT);
    for mutation in mutations {
        let bytes = encode_op(&mutation).expect("P2 retained mutation fixture encode");
        let mut session = Generation2dMutationSession::new(bytes.len(), GENERATION2D_MAXIMUM_DOMAIN_ITEMS).expect("P2 retained mutation preflight");
        let mut refused_subexact = false;
        for byte in bytes {
            assert!(session.ingress_ready());
            session.admit_byte(byte).expect("one retained mutation byte");
            for _ in 0..GENERATION2D_OWNER_BYTES {
                if let Some(exact) = session.next_retained_allocation_bytes().expect("P2 retained mutation allocation query") {
                    if exact > 0 && !refused_subexact {
                        let before = session.retained_allocated_bytes();
                        assert_eq!(session.reserve_retained_allocation(exact - 1).expect("P2 subexact retained mutation allocation"), (false, 0));
                        assert_eq!(session.retained_allocated_bytes(), before);
                        refused_subexact = true;
                    }
                    let (progressed, _) = session.reserve_retained_allocation(exact).expect("P2 retained mutation allocation");
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
            if let Some(exact) = session.next_retained_allocation_bytes().expect("P2 retained mutation allocation query") {
                let (progressed, _) = session.reserve_retained_allocation(exact).expect("P2 retained mutation allocation");
                assert!(progressed);
            } else if session.grant().expect("one retained semantic grant") {
                ready = true;
                break;
            }
        }
        assert!(ready, "retained P2 mutation owner must converge");
        let decoded = session.take().expect("typed P2 mutation handoff");
        let matches = decoded == mutation;
        let report = matches.then(String::new).unwrap_or_else(|| format!("decoded={decoded:?} fixture={mutation:?}"));
        generation2d_retire_mutation_cold(decoded);
        generation2d_retire_mutation_cold(mutation);
        close_session(&mut session);
        assert!(matches, "retained decode must recover the exact fixture mutation; {report}");
    }
}

#[test]
fn deterministic_all_field_ledger_includes_the_2d_only_variant() {
    let mutations = generation2d_all_retained_mutation_fixtures_for_test();
    let mut left = store::ArtifactStoreInitializationDigest::new(b"generation2d.all14");
    let mut right = store::ArtifactStoreInitializationDigest::new(b"generation2d.all14");
    for mutation in &mutations {
        generation2d_observe_mutation(&mut left, mutation);
        generation2d_observe_mutation(&mut right, mutation);
    }
    assert_eq!(left.finish(), right.finish());
    let carries_the_2d_only_variant = mutations.iter().any(|mutation| matches!(mutation, Generation2dMutation::ClearWidgetLayout(_)));
    generation2d_retire_mutations_cold(mutations);
    assert!(carries_the_2d_only_variant);
}

struct OuterPackField {
    authority: Option<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<Generation2dSnapshot, Generation2dMutation>>>,
    maximum_close_byte_demand: usize,
    maximum_retained_close_bytes: usize,
}

impl OuterPackField {
    fn diagnostic(code: &'static str) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT }
    }
}

impl store::ArtifactEnvelopeFieldDecoder<Generation2dSnapshot, Generation2dMutation> for OuterPackField {
    fn accept_field_token(
        &mut self,
        _field_id: u16,
        _token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(Self::diagnostic("generation2d-retained-pack.outer-field-not-readable"))
    }

    fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(Self::diagnostic("generation2d-retained-pack.outer-record-not-readable"))
    }

    fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.authority.as_ref().map_or(Ok(0), |authority| authority.next_close_byte_demand())
    }

    fn maximum_close_byte_demand(&self) -> usize {
        self.maximum_close_byte_demand
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        self.maximum_retained_close_bytes
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        let Some(authority) = self.authority.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        let step = authority.close_step(maximum_items, maximum_bytes)?;
        if step != store::SnapshotRetirementStep::Complete {
            return Ok(step);
        }
        if !authority.terminal_is_empty() {
            return Err(Self::diagnostic("generation2d-retained-pack.outer-vcs-false-terminal"));
        }
        drop(self.authority.take());
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.authority.is_none()
    }
}

impl Drop for OuterPackField {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.authority.is_none(), "outer retained Pack field reached Drop before its VCS owner was terminal-empty");
    }
}

fn outer_pack_record(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> store::OwnedSchemaRecordCursor {
    let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 2 }).expect("outer retained Pack record credits");
    pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(b"{}").expect("outer retained Pack record page")).unwrap_or_else(|_| panic!("outer retained Pack page admission"));
    pages.seal().expect("outer retained Pack record seal");
    store::artifact_envelope_decode_record(operation, generation, pages).unwrap_or_else(|_| panic!("outer retained Pack record cursor"))
}

#[test]
fn retained_pack_outer_cancellation_preserves_subexact_source_and_releases_exact_physical_allocation() {
    use semio_framework_job::InteractiveJob;

    let operation = semio_framework_job::OperationId(74);
    let generation = semio_framework_job::Generation(3);
    let mut session = crate::standards::v1::subsets::any::schema::snapshot::binary::Generation2dMountedPackSession::new(4 + store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES, 8).expect("outer retained Pack session credits");
    for byte in *b"P2D2" {
        session.admit_byte(byte).expect("outer retained Pack discriminator");
    }
    while let Some(exact) = session.next_retained_allocation_bytes().expect("outer retained Pack source allocation query") {
        assert!(session.reserve_retained_allocation(exact).expect("outer retained Pack source allocation").progressed);
    }
    for _ in 0..store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES {
        session.admit_byte(0).expect("outer retained Pack source byte");
    }
    let admitted_source_bytes = session.progress().expect("outer retained Pack source progress").allocated_bytes;
    assert!(admitted_source_bytes > store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);

    let mut snapshot = Generation2dPackSnapshotAuthority::new(operation, generation, store::OwnedSchemaPath::field("initialSnapshot").expect("outer retained Pack snapshot path"));
    *snapshot.session = Some(session);
    snapshot.state = Generation2dPackSnapshotState::Ingest;
    let vcs = store::ArtifactEnvelopeFreshVcsAuthority::try_new(
        Box::new(snapshot),
        std::sync::Arc::new(Generation2dRetainedSnapshotRetirementFactory),
        std::sync::Arc::new(Generation2dRetainedMutationRetirementFactory),
        store::ArtifactEnvelopeOwnedFieldCatalog::edit_history_decoder(&Generation2dEnvelopeOwnedFieldCatalog),
    )
    .unwrap_or_else(|_| panic!("outer retained Pack VCS admission"));
    let vcs: Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<Generation2dSnapshot, Generation2dMutation>> = Box::new(vcs);
    let maximum_close_byte_demand = vcs.maximum_close_byte_demand();
    let maximum_retained_close_bytes = vcs.maximum_retained_close_bytes();
    let registry = store::ArtifactEnvelopeFieldDecoderRegistry::new();
    let fields = Box::new(OuterPackField { authority: Some(vcs), maximum_close_byte_demand, maximum_retained_close_bytes });
    let mut job = store::ArtifactEnvelopeDecodeAuthority::try_new(outer_pack_record(operation, generation), &registry, fields).unwrap_or_else(|_| panic!("outer retained Pack decode admission"));
    InteractiveJob::begin_close(&mut job);

    let mut refused_subexact = false;
    let mut released_source_bytes = 0;
    let mut prior_released_source_bytes = 0;
    let mut terminal = false;
    for _ in 0..100_000 {
        if let Some(ticket) = registry.next_returned_ticket() {
            let mut returned = registry.take_returned_ticket(ticket).expect("outer retained Pack returned field detach");
            for _ in 0..4 {
                if store::ErasedSnapshotRetirement::close_step(&mut returned, 1, maximum_close_byte_demand).expect("outer retained Pack returned field close") == store::SnapshotRetirementStep::Complete {
                    break;
                }
            }
            assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&returned));
        }
        let demand = job.next_close_byte_demand().expect("outer retained Pack close demand");
        if demand > store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES && !refused_subexact {
            let before = job.released_field_bytes();
            assert_eq!(InteractiveJob::close_step(&mut job, 1, demand - 1), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!(job.next_close_byte_demand().expect("preserved outer retained Pack close demand"), demand);
            assert_eq!(job.released_field_bytes(), before);
            refused_subexact = true;
        }
        let step = InteractiveJob::close_step(&mut job, 1, demand);
        let released = job.released_field_bytes();
        released_source_bytes += released - prior_released_source_bytes;
        prior_released_source_bytes = released;
        if step == semio_framework_job::InteractiveJobCloseStep::Complete {
            terminal = true;
            break;
        }
    }
    assert!(refused_subexact, "outer cancellation must expose the physical source demand above the parser page size");
    assert!(terminal && InteractiveJob::terminal_is_empty(&job));
    assert!(registry.terminal_is_empty());
    assert_eq!(job.released_field_bytes(), admitted_source_bytes);
    assert_eq!(released_source_bytes, admitted_source_bytes);
    let mut sequence = 0;
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(InteractiveJob::step(&mut job, &mut context), semio_framework_job::StepOutcome::Cancelled));
}

struct RefusedSnapshotOwner {
    maximum_close_byte_demand: usize,
    maximum_retained_close_bytes: usize,
    terminal: bool,
    closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl RefusedSnapshotOwner {
    fn diagnostic() -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code: "generation2d-retained-pack.refused-snapshot-not-readable", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT }
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<Generation2dSnapshot> for RefusedSnapshotOwner {
    fn accept_token(
        &mut self,
        _token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(Self::diagnostic())
    }

    fn publish_reserved(
        &mut self,
        _target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<Generation2dSnapshot>,
        _reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(Self::diagnostic())
    }

    fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        self.maximum_close_byte_demand
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        self.maximum_retained_close_bytes
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        self.closed.store(true, std::sync::atomic::Ordering::Release);
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl Drop for RefusedSnapshotOwner {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal, "refused snapshot owner reached Drop before exact close");
    }
}

#[test]
fn retained_pack_outer_oversized_nested_snapshot_is_returned_for_close_before_vcs_admission() {
    let limits = [
        (store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES + 1, store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES + 1),
        (0, store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES + 1),
    ];
    for (maximum_close_byte_demand, maximum_retained_close_bytes) in limits {
        let closed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let snapshot: Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<Generation2dSnapshot>> = Box::new(RefusedSnapshotOwner {
            maximum_close_byte_demand,
            maximum_retained_close_bytes,
            terminal: false,
            closed: std::sync::Arc::clone(&closed),
        });
        let mut returned = store::ArtifactEnvelopeFreshVcsAuthority::try_new(
            snapshot,
            std::sync::Arc::new(Generation2dRetainedSnapshotRetirementFactory),
            std::sync::Arc::new(Generation2dRetainedMutationRetirementFactory),
            store::ArtifactEnvelopeOwnedFieldCatalog::edit_history_decoder(&Generation2dEnvelopeOwnedFieldCatalog),
        )
        .err()
        .expect("oversized nested snapshot must be returned before VCS admission");
        assert_eq!(returned.close_step(1, 0).expect("returned oversized snapshot close"), store::SnapshotRetirementStep::Complete);
        assert!(returned.terminal_is_empty());
        drop(returned);
        assert!(closed.load(std::sync::atomic::Ordering::Acquire));
    }
}
