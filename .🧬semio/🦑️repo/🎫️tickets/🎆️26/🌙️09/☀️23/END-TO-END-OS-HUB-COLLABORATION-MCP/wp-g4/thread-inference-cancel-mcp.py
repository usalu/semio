import sys
base = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
def edit(rel, pairs):
    p = base + rel
    s = open(p, encoding="utf-8").read()
    for old, new in pairs:
        n = s.count(old)
        if n != 1:
            sys.exit(f"{rel}: anchor count {n}: {old[:100]!r}")
        s = s.replace(old, new)
    open(p, "w", encoding="utf-8").write(s)

edit("🔀️dispatch/🦀️.rs", [
('''    pub artifact_id: String,
    pub artifact_document: Option<ArtifactDocumentBinding>,
}
''', '''    pub artifact_id: String,
    pub artifact_document: Option<ArtifactDocumentBinding>,
    pub cancel: InferenceCancel,
}

/// 🛑️ The caller's live cancellation for one inference run. `inference_run` binds it to its job, so
/// `job_cancel` and `notifications/cancelled` stop the in-flight guest job through the plugin host's
/// cold relay instead of waiting for the solve to finish. Two handles compare equal when they are in
/// the same state — the token itself has no identity a frame log could print.
#[derive(Clone)]
pub struct InferenceCancel(pub semio_framework_async::CancelToken);

impl Default for InferenceCancel {
    fn default() -> Self {
        Self(semio_framework_async::CancelToken::root_now())
    }
}

impl std::fmt::Debug for InferenceCancel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "InferenceCancel(cancelled: {})", self.0.is_cancelled_now())
    }
}

impl PartialEq for InferenceCancel {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_cancelled_now() == other.0.is_cancelled_now()
    }
}
'''),
])

edit("🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs", [
('''        artifact_id: String::new(),
        artifact_document: None,
    };
    let outcome = adapter.run_inference(3, command.clone())''', '''        artifact_id: String::new(),
        artifact_document: None,
        cancel: crate::actions::InferenceCancel::default(),
    };
    let outcome = adapter.run_inference(3, command.clone())'''),
])

edit("🏠️workspace/🦀️.rs", [
('''semio_framework_async::block_on(route.router.infer(&request_bytes))''', '''semio_framework_async::block_on(route.router.infer(&request_bytes, &command.cancel.0))'''),
])

edit("💡️inference/🦀️.rs", [
('''        artifact_id: requested_artifact.unwrap_or_default().to_string(),
        artifact_document,
    };
    jobs.report_progress(&job_id, 0.25,''', '''        artifact_id: requested_artifact.unwrap_or_default().to_string(),
        artifact_document,
        cancel: crate::actions::InferenceCancel::default(),
    };
    let cancel = command.cancel.0.clone();
    jobs.bind_cancel(&job_id, move || cancel.cancel_now());
    jobs.report_progress(&job_id, 0.25,'''),
('''    // 🛑️ Cooperative cancellation, at the two points this handler genuinely owns: before the guest
    // is ever dispatched, and again once it returns. The SAME `cancellationId` also travels on the
    // wire, where the guest's own `semio.infer` loop polls it — so a cancel issued mid-run is
    // observed by the guest, not silently ignored, even though this synchronous call cannot itself
    // be interrupted.
''', ''),
('''    match context.actions.run_inference(INFERENCE_ROUTED_INSTANCE, command) {
        Ok(outcome) => {''', '''    match context.actions.run_inference(INFERENCE_ROUTED_INSTANCE, command) {
        Err(_) if jobs.is_cancel_requested(&job_id) => {
            jobs.mark_cancelled(&job_id);
            CallToolResult::ok(vec![ContentBlock::Text { text: format!("inference job {job_id} was cancelled") }], Some(merge_inference_run_fields(base, serde_json::json!({ "status": "CANCELLED", "complete": false }))))
        }
        Ok(outcome) => {'''),
])

edit("🖥️ui/🦀️.rs", [
('''    error: Option<GatewayError>,
    cancel_requested: bool,
}

/// 📸️''', '''    error: Option<GatewayError>,
    cancel_requested: bool,
    on_cancel: Option<Box<dyn FnOnce() + Send>>,
}

/// 📸️'''),
('''        let record = JobRecord { kind: kind.to_string(), status: JobStatus::Pending, progress: None, message: None, result: None, error: None, cancel_requested: false };''',
 '''        let record = JobRecord { kind: kind.to_string(), status: JobStatus::Pending, progress: None, message: None, result: None, error: None, cancel_requested: false, on_cancel: None };'''),
('''            Some(record) if !record.status.is_terminal() => {
                record.status = status;
                record.result = result;''', '''            Some(record) if !record.status.is_terminal() => {
                record.status = status;
                record.on_cancel = None;
                record.result = result;'''),
('''    /// 🛑️ `job_cancel`'s real effect: flips the cooperative flag; a still-`Pending` job (nothing
    /// running yet to interrupt) finishes as `Cancelled` immediately, a `Running` one waits for its
    /// producer to call [`JobRegistry::mark_cancelled`]. `NOT_FOUND`/`PRECONDITION_FAILED` for an
    /// unknown or already-terminal id — never a silent no-op.
    pub fn request_cancel(&self, job_id: &str) -> Result<JobSnapshot, GatewayError> {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let record = jobs.get_mut(job_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such job: {job_id}")))?;
        if record.status.is_terminal() {
            return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("job {job_id} already finished as {:?} — nothing to cancel", record.status)));
        }
        record.cancel_requested = true;
        if record.status == JobStatus::Pending {
            record.status = JobStatus::Cancelled;
        }
        Ok(record_snapshot(job_id, record))
    }''', '''    /// 🔗️ Binds the producer's own interrupt to `job_id`: `request_cancel` fires it once, outside the
    /// registry lock, so a `job_cancel` or `notifications/cancelled` reaches work that is blocked in a
    /// guest instead of waiting for it to poll. A cancel that already landed fires it immediately.
    pub fn bind_cancel(&self, job_id: &str, hook: impl FnOnce() + Send + 'static) {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let Some(record) = jobs.get_mut(job_id) else { return };
        if record.status.is_terminal() {
            return;
        }
        if !record.cancel_requested {
            record.on_cancel = Some(Box::new(hook));
            return;
        }
        drop(jobs);
        hook();
    }

    /// 🛑️ `job_cancel`'s real effect: flips the cooperative flag and fires the producer's bound
    /// interrupt; a still-`Pending` job (nothing running yet to interrupt) finishes as `Cancelled`
    /// immediately, a `Running` one waits for its producer to call [`JobRegistry::mark_cancelled`].
    /// `NOT_FOUND`/`PRECONDITION_FAILED` for an unknown or already-terminal id — never a silent no-op.
    pub fn request_cancel(&self, job_id: &str) -> Result<JobSnapshot, GatewayError> {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let record = jobs.get_mut(job_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such job: {job_id}")))?;
        if record.status.is_terminal() {
            return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("job {job_id} already finished as {:?} — nothing to cancel", record.status)));
        }
        record.cancel_requested = true;
        if record.status == JobStatus::Pending {
            record.status = JobStatus::Cancelled;
        }
        let hook = record.on_cancel.take();
        let snapshot = record_snapshot(job_id, record);
        drop(jobs);
        if let Some(hook) = hook {
            hook();
        }
        Ok(snapshot)
    }'''),
])
print("ok")
