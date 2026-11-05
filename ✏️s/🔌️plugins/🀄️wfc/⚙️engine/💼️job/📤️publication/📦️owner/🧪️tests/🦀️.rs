//! 🧪️ Portable header frontiers measure actual box release against the same original receipt.
use super::*;
use crate::job::PublicationKind;
use semio_framework_job::RetainedCloneGrant;
use semio_s_plugin_wfc_engine_oracle::step_context::{TestJobDriver, TEST_TURN};

#[test]
fn original_publication_header_drop_is_separate_paid_and_denied_without_effects() {
    let law: serde_json::Value = serde_json::from_str(include_str!("frontiers.json")).unwrap();
    let header = std::mem::size_of::<Publication>();
    for case in law["cases"].as_array().unwrap() {
        let mut slot = Some(Publication::new(PublicationKind::Preview, Vec::new(), Vec::new()));
        assert!(slot.as_ref().unwrap().terminal_is_empty());
        assert_eq!(publication_retirement_demands(&slot).unwrap().release_bytes, header);
        let grant = RetainedCloneGrant { maximum_items: case["items"].as_u64().unwrap() as usize, maximum_release_bytes: if case["release"] == "short" { header - 1 } else { header }, ..TEST_TURN };
        let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1));
        let mut work = 0;
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| driver.turn(|cx| {
            let before = cx.original_work_units_spent();
            let step = cx.with_retained_loan(grant, |same| close_publication_step(&mut slot, same))?;
            work = cx.original_work_units_spent() - before;
            Ok(step)
        }).unwrap());
        let removed = case["removed"].as_bool().unwrap();
        assert_eq!(slot.is_none(), removed);
        assert_eq!(work, case["work"].as_u64().unwrap() as u128);
        assert!(!heap.overflowed);
        assert_eq!(heap.requested_bytes, 0);
        assert_eq!(heap.released_bytes, if removed { header } else { 0 });
        assert_eq!(step.progress().released_bytes, heap.released_bytes);
        assert_eq!(step.progress().copied_items, usize::from(removed));
        if slot.is_some() {
            driver.turn(|cx| close_publication_step(&mut slot, cx)).unwrap();
        }
        assert!(slot.is_none());
        let complete = driver.turn(|cx| close_publication_step(&mut slot, cx)).unwrap();
        assert!(matches!(complete, InteractiveJobCloseStep::Complete { .. }));
        assert_eq!(complete.progress(), RetainedCloneProgress::default());
        driver.retire_source();
    }
}
