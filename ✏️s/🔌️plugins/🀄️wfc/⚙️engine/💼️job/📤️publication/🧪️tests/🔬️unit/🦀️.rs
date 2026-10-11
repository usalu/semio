//! 🧪️ Neutral publication vectors exercised through the retained publication owners.

use super::*;
use semio_framework_job::{allocate_operation_id, root_cancel_token, Generation, Operation, RevisionId, StepBudget, JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_PAGE_BYTES};

/// 🪜️ One retained publication binds every operation page slot of each of its streams on its own turn before any byte moves.
const POLL_BOUND: usize = 2 * JOB_PAYLOAD_OPERATION_PAGES + 64;

const GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral publication vectors")
}

fn source(value: &serde_json::Value) -> Vec<u8> {
    match value["length"].as_u64() {
        Some(length) => vec![u8::try_from(value["byte"].as_u64().unwrap()).unwrap(); usize::try_from(length).unwrap()],
        None => Vec::new(),
    }
}

fn publication(value: &serde_json::Value) -> Box<Publication> {
    let kind = match value["kind"].as_str().unwrap() {
        "preview" => Kind::Preview,
        "checkpoint" => Kind::Checkpoint(value["applied_progress"].as_u64().unwrap()),
        "commit" => Kind::Commit,
        "fault" => Kind::Fault,
        kind => panic!("unknown neutral publication kind {kind}"),
    };
    Publication::new(kind, source(&value["primary"]), source(&value["secondary"]))
}

fn self_funded(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

fn close(publication: &mut Publication) {
    for _ in 0..1_000 {
        if publication.terminal_is_empty() {
            return;
        }
        let grant = self_funded(publication.retirement_demands().expect("a locally owned publication quotes its close demand"));
        let step = publication.close_step(grant);
        assert!(step.progress().fits(grant), "a publication close receipt fits its own quoted grant");
        assert!(!matches!(step, InteractiveJobCloseStep::Blocked | InteractiveJobCloseStep::Refused { .. }));
    }
    panic!("fixture publication retained an owner after bounded close");
}

fn received(payload: &RetainedJobPayload) -> (Vec<u8>, serde_json::Value) {
    let pages: Vec<&[u8]> = (0..payload.page_count()).map(|index| payload.page(index).unwrap()).collect();
    (pages.iter().flat_map(|page| page.iter().copied()).collect(), serde_json::to_value(pages.iter().map(|page| page.len()).collect::<Vec<_>>()).unwrap())
}

type Delivered = (&'static str, (Vec<u8>, serde_json::Value), Option<(Vec<u8>, serde_json::Value)>, Option<u64>);

fn publication_operation(publication: &Publication) -> semio_framework_job::OperationId {
    semio_framework_job::OperationId(0x5057_0000 + std::ptr::from_ref(publication) as usize as u64)
}

fn poll_once(publication: &mut Publication, fuel: u64, sequence: &mut u64) -> Option<Delivered> {
    let operation = Operation::new(publication_operation(publication), RevisionId(1), Generation(1), 1);
    let mut receipt = RetainedCloneProgress::default();
    let mut context = crate::job::test_step_context(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX, GRANT), root_cancel_token(), sequence, &mut receipt);
    match publication.poll(&mut context).expect("one publication step")? {
        JobOutcomeBorrow::PreviewReady { payload, .. } => Some(("preview", received(payload), None, None)),
        JobOutcomeBorrow::CheckpointReady { state, applied_progress, .. } => Some(("checkpoint", received(state), None, Some(applied_progress))),
        JobOutcomeBorrow::Complete { state, output, .. } => Some(("commit", received(state.expect("commit state")), Some(received(output.expect("commit output"))), None)),
        JobOutcomeBorrow::Fault { detail, .. } => Some(("fault", received(detail), None, None)),
        JobOutcomeBorrow::Yield { .. } | JobOutcomeBorrow::Cancelled { .. } => None,
    }
}

#[test]
fn retained_publication_matches_neutral_pages_and_preserves_both_commit_streams() {
    let fixture = fixture();
    assert_eq!(fixture["page_bytes"], JOB_PAYLOAD_PAGE_BYTES);
    for vector in fixture["vectors"].as_array().unwrap() {
        let mut publication = publication(vector);
        let mut sequence = 0;
        let mut result = None;
        for _ in 0..POLL_BOUND {
            result = poll_once(&mut publication, 1, &mut sequence);
            if result.is_some() {
                break;
            }
        }
        assert!(publication.is_delivered());
        close(&mut publication);
        let (kind, primary, secondary, progress) = result.expect("completed publication");
        assert_eq!(kind, vector["kind"].as_str().unwrap());
        assert_eq!(primary, (source(&vector["primary"]), vector["primary"]["pages"].clone()));
        assert_eq!(secondary, vector.get("secondary").map(|value| (source(value), value["pages"].clone())));
        assert_eq!(progress, vector["applied_progress"].as_u64());
    }
}

#[test]
fn retained_publication_honors_zero_fuel_and_still_delivers_the_exact_streams() {
    let vector = fixture()["vectors"][0].clone();
    let mut publication = publication(&vector);
    let mut sequence = 0;
    assert!(poll_once(&mut publication, 0, &mut sequence).is_none());
    assert!(!publication.is_delivered());
    assert_eq!(publication.state_cursor, 0);
    let mut actual = None;
    for _ in 0..POLL_BOUND {
        actual = poll_once(&mut publication, 1, &mut sequence);
        if actual.is_some() {
            break;
        }
    }
    close(&mut publication);
    let (_, primary, secondary, _) = actual.expect("delivered after the zero-fuel turn");
    assert_eq!(primary.0, source(&vector["primary"]));
    assert_eq!(secondary.map(|stream| stream.0), vector.get("secondary").map(source));
}

#[test]
fn retained_publication_cancellation_closes_finished_and_partial_streams_incrementally() {
    let fixture = fixture();
    let mut publication = publication(&fixture["vectors"][0]);
    let mut sequence = 0;
    for _ in 0..5 {
        let _ = poll_once(&mut publication, 1, &mut sequence);
    }
    assert!(!publication.terminal_is_empty(), "a started publication retains owners");
    let zero = publication.close_step(RetainedCloneGrant::default());
    let mut within_budget = true;
    let source_capacity = publication.primary.capacity().max(publication.secondary.capacity());
    for _ in 0..POLL_BOUND {
        if publication.terminal_is_empty() {
            break;
        }
        let grant = self_funded(publication.retirement_demands().expect("a locally owned publication quotes its close demand"));
        if let InteractiveJobCloseStep::Pending { progress } = publication.close_step(grant) {
            within_budget &= progress.copied_items <= 1 && progress.released_bytes <= JOB_PAYLOAD_PAGE_BYTES.max(source_capacity);
        }
    }
    assert!(publication.terminal_is_empty());
    assert!(within_budget);
    assert_eq!(zero, InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
}
