//! 🧪️ Neutral publication vectors exercised through the retained publication owners.

use super::*;
use semio_framework_job::{allocate_operation_id, Generation, InteractiveJob};
use semio_s_plugin_wfc_engine_oracle::step_context::TestJobDriver;
use semio_framework_job_payload::{JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_PAGE_BYTES};

/// 🪜️ One retained publication binds every operation page slot of each of its streams on its own turn before any byte moves.
const POLL_BOUND: usize = 8 * JOB_PAYLOAD_OPERATION_PAGES + 128;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("neutral publication vectors")
}

fn source(value: &serde_json::Value) -> Vec<u8> {
    match value["length"].as_u64() {
        Some(length) => vec![u8::try_from(value["byte"].as_u64().unwrap()).unwrap(); usize::try_from(length).unwrap()],
        None => Vec::new(),
    }
}

struct PublicationFixture { publication: Option<Box<Publication>> }

impl semio_framework_job::InteractiveJob for PublicationFixture {
    fn step<'job>(&'job mut self, cx: &mut StepContext<'_>, _: &mut semio_framework_job::OriginalJobReturnClaim<'_>) -> semio_framework_job::InteractiveJobStep<'job> {
        semio_framework_job::InteractiveJobStep::from_callback((|| {
            if self.publication.as_ref().is_some_and(|publication| publication.needs_retirement()) {
                crate::job::close_publication_step(&mut self.publication, cx)?;
                return Ok(None);
            }
            if self.publication.is_none() { return Ok(JobOutcomeBorrow::admit_complete(cx, None, None)?); }
            self.publication.as_mut().expect("live publication").poll(cx)
        })())
    }
    fn borrow_outcome<'job>(&'job self, descriptor: &'job JobOutcomeDescriptor) -> Result<JobOutcomeView<'job>, ValueError> {
        self.publication.as_ref().expect("live publication").borrow_outcome(descriptor)
    }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        crate::job::close_publication_step(&mut self.publication, cx)
    }
    fn terminal_is_empty(&self) -> bool { self.publication.is_none() }
}

fn publication(value: &serde_json::Value) -> PublicationFixture {
    let kind = match value["kind"].as_str().unwrap() {
        "preview" => Kind::Preview,
        "checkpoint" => Kind::Checkpoint(value["applied_progress"].as_u64().unwrap()),
        "commit" => Kind::Commit,
        "fault" => Kind::Fault,
        kind => panic!("unknown neutral publication kind {kind}"),
    };
    PublicationFixture { publication: Some(Publication::new(kind, source(&value["primary"]), source(&value["secondary"]))) }
}

fn frontier(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

fn close(publication: &mut PublicationFixture, driver: &mut TestJobDriver) {
    driver.close(publication);
}

fn received(payload: &RetainedJobPayload) -> (Vec<u8>, serde_json::Value) {
    let pages: Vec<&[u8]> = (0..payload.page_count()).map(|index| payload.page(index).unwrap()).collect();
    (pages.iter().flat_map(|page| page.iter().copied()).collect(), serde_json::to_value(pages.iter().map(|page| page.len()).collect::<Vec<_>>()).unwrap())
}

type Delivered = (&'static str, (Vec<u8>, serde_json::Value), Option<(Vec<u8>, serde_json::Value)>, Option<u64>);

fn poll_once(publication: &mut PublicationFixture, fuel: u64, driver: &mut TestJobDriver) -> Option<Delivered> {
    driver.step(publication, fuel, |outcome| match outcome {
        Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => Some(("preview", received(payload), None, None)),
        Some(JobOutcomeBorrow::CheckpointReady { state, applied_progress, .. }) => Some(("checkpoint", received(state), None, Some(*applied_progress))),
        Some(JobOutcomeBorrow::Complete { state, output, .. }) => Some(("commit", received(state.expect("commit state")), Some(received(output.expect("commit output"))), None)),
        Some(JobOutcomeBorrow::Fault { detail, .. }) => Some(("fault", received(detail), None, None)),
        _ => None,
    })
}

#[test]
fn retained_publication_matches_neutral_pages_and_preserves_both_commit_streams() {
    let fixture = fixture();
    assert_eq!(fixture["page_bytes"], JOB_PAYLOAD_PAGE_BYTES);
    for vector in fixture["vectors"].as_array().unwrap() {
        let mut publication = publication(vector);
        let mut driver = TestJobDriver::new(allocate_operation_id(), Generation(1));
        let mut result = None;
        for _ in 0..POLL_BOUND {
            result = poll_once(&mut publication, 1, &mut driver);
            if result.is_some() {
                break;
            }
        }
        assert!(publication.publication.as_ref().unwrap().needs_retirement());
        close(&mut publication, &mut driver);
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
    let mut driver = TestJobDriver::new(allocate_operation_id(), Generation(1));
    assert!(poll_once(&mut publication, 0, &mut driver).is_none());
    assert!(!publication.publication.as_ref().unwrap().needs_retirement());
    assert_eq!(publication.publication.as_ref().unwrap().state_cursor, 0);
    let mut actual = None;
    for _ in 0..POLL_BOUND {
        actual = poll_once(&mut publication, 1, &mut driver);
        if actual.is_some() {
            break;
        }
    }
    close(&mut publication, &mut driver);
    let (_, primary, secondary, _) = actual.expect("delivered after the zero-fuel turn");
    assert_eq!(primary.0, source(&vector["primary"]));
    assert_eq!(secondary.map(|stream| stream.0), vector.get("secondary").map(source));
}

#[test]
fn retained_publication_cancellation_closes_finished_and_partial_streams_incrementally() {
    let fixture = fixture();
    let mut publication = publication(&fixture["vectors"][0]);
    let mut driver = TestJobDriver::new(allocate_operation_id(), Generation(1));
    for _ in 0..POLL_BOUND {
        let _ = poll_once(&mut publication, 1, &mut driver);
        if publication.publication.as_ref().unwrap().state_cursor > 0 { break; }
    }
    assert!(publication.publication.as_ref().unwrap().state_cursor > 0, "the original stream must start before cancellation");
    assert!(!publication.terminal_is_empty(), "a started publication retains owners");
    let zero = driver.turn(|cx| cx.with_retained_loan(RetainedCloneGrant::default(), |same| publication.close_step(same))).expect("a zero grant is a pending close");
    let mut within_budget = true;
    let inner = publication.publication.as_ref().unwrap();
    let source_capacity = inner.primary.capacity().max(inner.secondary.capacity()).max(std::mem::size_of::<Publication>());
    for _ in 0..POLL_BOUND {
        if publication.terminal_is_empty() {
            break;
        }
        let grant = frontier(crate::job::publication_retirement_demands(&publication.publication).expect("original publication quotes its next frontier"));
        if let Ok(InteractiveJobCloseStep::Pending { progress }) = driver.turn(|cx| cx.with_retained_loan(grant, |same| publication.close_step(same))) {
            within_budget &= progress.copied_items <= 1 && progress.released_bytes <= JOB_PAYLOAD_PAGE_BYTES.max(source_capacity);
        }
    }
    assert!(publication.terminal_is_empty());
    assert!(within_budget);
    assert_eq!(zero, InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
    close(&mut publication, &mut driver);
}

#[test]
fn delivered_publication_keeps_its_retiring_phase_until_the_box_returns() {
    let vector = fixture()["vectors"][1].clone();
    let mut publication = publication(&vector);
    let mut driver = TestJobDriver::new(allocate_operation_id(), Generation(1));
    let mut previews = 0;
    let mut complete = false;
    let mut saw_empty_retiring_header = false;
    for _ in 0..POLL_BOUND * 2 {
        saw_empty_retiring_header |= publication.publication.as_ref().is_some_and(|inner| inner.terminal_is_empty() && inner.needs_retirement());
        let outcome = driver.step(&mut publication, 1, |outcome| match outcome {
            Some(JobOutcomeBorrow::PreviewReady { .. }) => 1,
            Some(JobOutcomeBorrow::Complete { .. }) => 2,
            _ => 0,
        });
        previews += usize::from(outcome == 1);
        if outcome == 2 { complete = true; break; }
    }
    driver.close(&mut publication);
    assert!(complete, "a retired publication cannot be polled as a fresh preview");
    assert_eq!(previews, 1);
    assert!(saw_empty_retiring_header, "inner retirement and box removal are separate admitted turns");
}
