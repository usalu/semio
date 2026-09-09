//! 🧪️ Neutral publication vectors exercised through the persistent session ledger.

use super::*;
use semio_framework_job::{allocate_operation_id, root_cancel_token, BatchDriveConfig, BatchJobParams, BatchJobSession, Generation, InteractiveJob, InteractiveStage, Operation, RevisionId, StepBudget};

struct Probe {
    publication: Option<Box<Publication>>,
    closing: bool,
}

impl InteractiveJob for Probe {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        Publication::poll(&mut self.publication, context)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if let Some(publication) = self.publication.as_mut() {
            let step = publication.close_step(maximum_items, maximum_bytes);
            if publication.terminal_is_empty() {
                self.publication = None;
            }
            return step;
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.publication.is_none()
    }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral publication vectors")
}

fn source(value: &serde_json::Value) -> Vec<u8> {
    match value["length"].as_u64() {
        Some(length) => vec![u8::try_from(value["byte"].as_u64().unwrap()).unwrap(); usize::try_from(length).unwrap()],
        None => Vec::new(),
    }
}

fn probe(value: &serde_json::Value) -> Probe {
    let kind = match value["kind"].as_str().unwrap() {
        "preview" => Kind::Preview,
        "checkpoint" => Kind::Checkpoint(value["applied_progress"].as_u64().unwrap()),
        "commit" => Kind::Commit,
        "fault" => Kind::Fault,
        kind => panic!("unknown neutral publication kind {kind}"),
    };
    Probe { publication: Some(Publication::new(kind, source(&value["primary"]), source(&value["secondary"]))), closing: false }
}

fn session(probe: Probe) -> BatchJobSession<Probe> {
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(1), 1);
    let params = BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: root_cancel_token(),
        config: BatchDriveConfig { site: "wfc.publication.fixture", stage: InteractiveStage::BackgroundStep, fuel_per_step: 1, step_budget_us: 4_000 },
        now_us: || Some(0),
    };
    match BatchJobSession::try_new(probe, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                rejected.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
            }
            panic!("fixture session admission rejected");
        }
    }
}

fn retire(outcome: &mut StepOutcome) {
    while !outcome.terminal_is_empty() {
        outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
}

fn close(session: &mut BatchJobSession<Probe>) {
    session.begin_close();
    for _ in 0..100 {
        if session.terminal_is_empty() {
            return;
        }
        session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    panic!("fixture session retained an owner after bounded close");
}

fn received(payload: &RetainedJobPayload) -> (Vec<u8>, serde_json::Value) {
    let pages: Vec<&[u8]> = (0..payload.page_count()).map(|index| payload.page(index).unwrap()).collect();
    (pages.iter().flat_map(|page| page.iter().copied()).collect(), serde_json::to_value(pages.iter().map(|page| page.len()).collect::<Vec<_>>()).unwrap())
}

#[test]
fn retained_publication_matches_neutral_pages_and_preserves_both_commit_streams() {
    let fixture = fixture();
    assert_eq!(fixture["page_bytes"], JOB_PAYLOAD_PAGE_BYTES);
    for vector in fixture["vectors"].as_array().unwrap() {
        let mut session = session(probe(vector));
        let mut result = None;
        let mut steps = 0;
        for _ in 0..16 {
            session.step().expect("one publication step");
            let mut outcome = session.take_outcome().expect("exact publication outcome");
            steps += 1;
            result = match &outcome {
                StepOutcome::PreviewReady(payload) => Some(("preview", received(payload), None, None)),
                StepOutcome::CheckpointReady(checkpoint) => Some(("checkpoint", received(&checkpoint.state), None, Some(checkpoint.applied_progress))),
                StepOutcome::Complete(candidate) => Some(("commit", received(&candidate.state), Some(received(&candidate.output)), None)),
                StepOutcome::Fault(fault) => Some(("fault", received(&fault.detail), None, None)),
                _ => None,
            };
            retire(&mut outcome);
            if result.is_some() {
                break;
            }
            session.resume().expect("resume retained publication");
        }
        close(&mut session);
        let (kind, primary, secondary, progress) = result.expect("completed publication");
        assert_eq!(kind, vector["kind"].as_str().unwrap());
        assert_eq!(steps, vector["steps"].as_u64().unwrap());
        assert_eq!(primary, (source(&vector["primary"]), vector["primary"]["pages"].clone()));
        assert_eq!(secondary, vector.get("secondary").map(|value| (source(value), value["pages"].clone())));
        assert_eq!(progress, vector["applied_progress"].as_u64());
    }
}

#[test]
fn retained_publication_retries_exact_rejected_source_and_honors_zero_fuel() {
    let vector = fixture()["vectors"][0].clone();
    let mut probe = probe(&vector);
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(1), 1);
    let mut sequence = 0;
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(0, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(probe.step(&mut context), StepOutcome::Yield));
    assert_eq!(probe.publication.as_ref().unwrap().primary.cursor, 0);
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
    let mut held = context.payload_from_bytes(JobPayloadStream::Preview, b"held").unwrap_or_else(|rejected| {
        let source = rejected.into_source();
        drop(source);
        panic!("first fixture grant rejected");
    });
    assert!(matches!(probe.step(&mut context), StepOutcome::Yield));
    assert_eq!(probe.publication.as_ref().unwrap().primary.cursor, 0);
    while !held.terminal_is_empty() {
        held.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    let mut session = session(probe);
    let mut actual = None;
    for _ in 0..8 {
        session.step().expect("retry publication");
        let mut outcome = session.take_outcome().unwrap();
        if let StepOutcome::Complete(candidate) = &outcome {
            actual = Some((received(&candidate.state).0, received(&candidate.output).0));
        }
        retire(&mut outcome);
        if actual.is_some() {
            break;
        }
        session.resume().unwrap();
    }
    close(&mut session);
    assert_eq!(actual, Some((source(&vector["primary"]), source(&vector["secondary"]))));
}

#[test]
fn retained_publication_cancellation_closes_finished_and_partial_streams_incrementally() {
    let fixture = fixture();
    let mut session = session(probe(&fixture["vectors"][0]));
    let steps = fixture["cancellation"]["steps"].as_u64().unwrap();
    for index in 0..steps {
        session.step().unwrap();
        let mut outcome = session.take_outcome().unwrap();
        retire(&mut outcome);
        if index + 1 != steps {
            session.resume().unwrap();
        }
    }
    let probe = session.checked_out_job_mut().unwrap();
    let publication = probe.publication.as_ref().unwrap();
    let retained_both = publication.primary.ready.is_some() && publication.secondary.as_ref().unwrap().writer.as_ref().unwrap().page_count() == 1;
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(1), 1);
    let mut sequence = 0;
    let cancel = root_cancel_token();
    cancel.cancel_now();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), cancel, || Some(0), &mut sequence);
    let cancelled = matches!(probe.step(&mut context), StepOutcome::Cancelled);
    probe.begin_close();
    let zero = probe.close_step(0, 0);
    let mut within_budget = true;
    for _ in 0..32 {
        if probe.terminal_is_empty() {
            break;
        }
        if let InteractiveJobCloseStep::Pending { released_items, released_bytes } = probe.close_step(1, JOB_PAYLOAD_PAGE_BYTES) {
            within_budget &= released_items <= 1 && released_bytes <= JOB_PAYLOAD_PAGE_BYTES;
        }
    }
    let emptied = probe.terminal_is_empty();
    close(&mut session);
    assert!(retained_both && cancelled && emptied && within_budget);
    assert_eq!(zero, InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
}
