use super::*;
fn physical_close_grant(release:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:release,maximum_depth:64}}
#[test]
fn retained_writer_physical_close_preserves_staged_and_rejected_backing() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🧪️tests/🧫️fixtures/📦️physical-close/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let operation = OperationId(90_020);
        let generation = Generation(21);
        let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
        let mut sequence = 0;
        let mut actual_retained_progress=RetainedCloneProgress::default();let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::Preview);
        writer.begin_staged_page(&mut context).unwrap();
        writer.write_staged(&vec![42; row["logicalBytes"].as_u64().unwrap() as usize]).unwrap();
        assert!(matches!(writer.admit_page(&mut context), Err(JobPayloadAdmissionFault::OpportunityExhausted)));
        let staged_pointer = writer.staged.as_ref().unwrap().1.backing_identity();
        let rejected_pointer = writer.rejected.as_ref().unwrap().backing_identity();
        let mut observations = Vec::new();
        for (staged, pointer) in [(true, staged_pointer), (false, rejected_pointer)] {
            assert_eq!(writer.next_close_byte_demand(), JOB_PAYLOAD_PAGE_BYTES);
            for _ in 0..4 {
                assert_eq!(writer.close_step(physical_close_grant(row["insufficientGrant"].as_u64().unwrap() as usize)).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..Default::default() }));
            }
            let refused = writer.close_step(physical_close_grant(row["insufficientGrant"].as_u64().unwrap() as usize)).unwrap();
            let retained_pointer = if staged {
                writer.staged.as_ref().map(|(_, source, _)| source.backing_identity()) == Some(pointer)
            } else {
                writer.rejected.as_ref().map(JobPayloadPageSource::backing_identity) == Some(pointer)
            };
            let released = writer.close_step(physical_close_grant(writer.next_close_byte_demand())).unwrap();
            observations.push((staged, refused, retained_pointer, released));
        }
        while !writer.terminal_is_empty() { writer.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap(); }
        assert!(ledger.terminal_is_empty());
        for (staged, refused, retained_pointer, released) in observations {
            assert_eq!(refused, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..Default::default() }), "staged={staged} {}", row["name"]);
            assert!(retained_pointer, "staged={staged} {}", row["name"]);
            assert_eq!(released, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }), "staged={staged} {}", row["name"]);
        }
    }
}

#[test]
fn payload_ledger_identity_must_match_the_exact_step_context() {
    let ledger = Arc::new(JobPayloadOperationLedger::new(OperationId(90_000), Generation(6)));
    let operation_mismatch = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut sequence = 0;
        let mut actual_retained_progress=RetainedCloneProgress::default();let _ = StepContext::with_payload_ledger(OperationId(90_001), Generation(6), StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    }));
    assert!(operation_mismatch.is_err());
    let generation_mismatch = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut sequence = 0;
        let mut actual_retained_progress=RetainedCloneProgress::default();let _ = StepContext::with_payload_ledger(OperationId(90_000), Generation(7), StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    }));
    assert!(generation_mismatch.is_err());
}

#[test]
fn retained_state_and_output_have_separate_credits_and_close_one_page_per_grant() {
    let operation = OperationId(90_002);
    let generation = Generation(8);
    let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
    let mut state_writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitState);
    let mut output_writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
    let mut sequence = 0;
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut state_context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    let mut state_page = state_context.admit_payload_page(&mut state_writer, JobPayloadPageSource::new()).expect("state page");
    state_page.write(b"state").expect("state bytes");
    state_page.commit();
    let rejected = state_context.payload_from_bytes(JobPayloadStream::CommitOutput, b"output").expect_err("a second stream cannot bypass the one-page opportunity");
    assert_eq!(rejected.fault, JobPayloadAdmissionFault::OpportunityExhausted);
    drop(rejected.into_source());
    assert_eq!(state_writer.page_count(), 1);
    drop(state_context);
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut output_context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    let mut output_page = output_context.admit_payload_page(&mut output_writer, JobPayloadPageSource::new()).expect("separate output page");
    output_page.write(b"output").expect("output bytes");
    output_page.commit();
    let mut terminal = StepOutcome::Complete(CommitCandidate { state: state_writer.finish().expect("state"), output: output_writer.finish().expect("output") });
    assert_eq!(terminal.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }));
    assert!(!terminal.terminal_is_empty());
    let ledger_step=terminal.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap();
    assert_eq!(ledger_step.progress().released_bytes, 0);
    assert!(!terminal.terminal_is_empty());
    assert_eq!(terminal.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }));
    let ledger_step=terminal.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap();
    assert_eq!(ledger_step.progress().released_bytes, 0);
    assert!(terminal.terminal_is_empty());
}

#[test]
fn retained_writer_and_reader_advance_exactly_one_page_per_opportunity() {
    let operation = OperationId(90_008);
    let generation = Generation(14);
    let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
    let bytes = vec![19u8; JOB_PAYLOAD_PAGE_BYTES + 1];
    let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
    let mut cursor = 0;
    let mut sequence = 0;
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut zero = StepContext::with_payload_ledger(operation, generation, StepBudget::new(0, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    assert_eq!(writer.write_slice_page(&mut zero, &bytes, &mut cursor), Ok(false));
    assert_eq!(cursor, 0);
    drop(zero);
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut first = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    assert_eq!(writer.write_slice_page(&mut first, &bytes, &mut cursor), Ok(false));
    assert_eq!(cursor, JOB_PAYLOAD_PAGE_BYTES);
    drop(first);
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut second = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    assert_eq!(writer.write_slice_page(&mut second, &bytes, &mut cursor), Ok(true));
    let mut payload = writer.finish().expect("two-page retained payload");
    let mut reader = payload.reader();
    assert_eq!(reader.read_page(0, JOB_PAYLOAD_PAGE_BYTES), None);
    assert_eq!(reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES).map(|page| page.len()), Some(JOB_PAYLOAD_PAGE_BYTES));
    assert_eq!(reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES).map(|page| page.len()), Some(1));
    assert!(reader.terminal_is_empty());
    assert_eq!(payload.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }));
    assert_eq!(payload.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }));
    let ledger_step=payload.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap();
    assert_eq!(ledger_step.progress().released_bytes, 0);
    assert_eq!(payload.close_step(physical_close_grant(JOB_PAYLOAD_PAGE_BYTES)).unwrap(), RetainedCloneStep::Complete(Default::default()));
    assert!(payload.terminal_is_empty());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
