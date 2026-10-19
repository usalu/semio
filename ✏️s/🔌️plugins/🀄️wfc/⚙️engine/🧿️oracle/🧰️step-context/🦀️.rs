//! 🧰️ Step context for driving a wfc job from tests: one leaked payload authority per operation, as the worker keeps it. Lives beside the reference solver so every crate's tests share it through a dev-dependency instead of a `cfg(test)` item of the job crate.

/// 🧪️ Step context whose operation keeps one leaked payload authority across steps, as the worker does.
pub fn test_step_context<'a>(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, budget: semio_framework_job::StepBudget, cancel: semio_framework_job::CancelToken, sequence: &'a mut u64, receipt: &'a mut semio_framework_job::RetainedCloneProgress) -> semio_framework_job::StepContext<'a> {
    use std::cell::RefCell;
    use std::collections::HashMap;
    thread_local! {
        static AUTHORITIES: RefCell<HashMap<(u64, u64), &'static semio_framework_job_payload::JobPayloadAuthority>> = RefCell::new(HashMap::new());
    }
    let authority = AUTHORITIES.with(|map| {
        *map.borrow_mut().entry((operation.0, generation.0)).or_insert_with(|| Box::leak(Box::new(semio_s_plugin_wfc_engine_job::job::admit_payload_authority(semio_framework_job::Operation::new(operation, semio_framework_job::RevisionId(0), generation, 0)))))
    });
    let cancel: &'static semio_framework_job::CancelToken = Box::leak(Box::new(cancel));
    semio_framework_job::StepContext::with_payload_authority(operation, generation, budget, cancel, || Some(0), sequence, receipt, authority).expect("test context admits its payload authority")
}
