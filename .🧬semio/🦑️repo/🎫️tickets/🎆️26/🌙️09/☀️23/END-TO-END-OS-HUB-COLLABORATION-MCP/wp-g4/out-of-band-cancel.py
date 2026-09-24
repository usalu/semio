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

edit("📣️notify/🦀️.rs", [
("use std::collections::{BTreeMap, BTreeSet};", "use std::collections::{BTreeMap, BTreeSet, VecDeque};"),
('''#[derive(Clone)]
pub struct ProgressBinding {
    pub token: serde_json::Value,
    pub request_id: Option<serde_json::Value>,
    pub slot: NotificationSlot,
}

thread_local! {
    static ACTIVE_PROGRESS: RefCell<Option<ProgressBinding>> = const { RefCell::new(None) };
}
''', '''#[derive(Clone)]
pub struct ProgressBinding {
    pub token: serde_json::Value,
    pub slot: NotificationSlot,
}

thread_local! {
    static ACTIVE_PROGRESS: RefCell<Option<ProgressBinding>> = const { RefCell::new(None) };
    static ACTIVE_REQUEST: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// 🧷️ RAII guard naming the JSON-RPC request whose `tools/call` runs on this thread — with or
/// without a progress token — so every job it mints is cancellable by `notifications/cancelled`.
/// Dropping it forgets a cancel that named this request.
pub struct RequestScope {
    previous: Option<String>,
    key: Option<String>,
}

impl Drop for RequestScope {
    fn drop(&mut self) {
        let previous = self.previous.take();
        ACTIVE_REQUEST.with(|active| *active.borrow_mut() = previous);
        if let Some(key) = self.key.take() {
            progress_bindings().lock().expect("progress binding lock poisoned").cancelled_requests.retain(|cancelled| cancelled != &key);
        }
    }
}

/// 🆔️ Enters the request scope for one `tools/call`; `None` (a notification) binds nothing.
pub fn enter_request_scope(request_id: Option<&serde_json::Value>) -> RequestScope {
    let key = request_id.map(request_key);
    let previous = ACTIVE_REQUEST.with(|active| std::mem::replace(&mut *active.borrow_mut(), key.clone()));
    RequestScope { previous, key }
}
'''),
('''#[derive(Default)]
struct ProgressBindings {
    by_job: BTreeMap<String, ProgressBinding>,
    jobs_by_request: BTreeMap<String, Vec<String>>,
}
''', '''#[derive(Default)]
struct ProgressBindings {
    by_job: BTreeMap<String, ProgressBinding>,
    jobs_by_request: BTreeMap<String, Vec<String>>,
    cancelled_requests: VecDeque<String>,
}

/// 🛑️ How many cancels for requests that have not minted a job yet are remembered — the stdio reader
/// runs ahead of dispatch by at most the client's pipelined requests, and a cancel for a request that
/// already finished must not accumulate forever.
const CANCELLED_REQUEST_MEMORY: usize = 256;
'''),
('''pub fn bind_job_to_active_scope(job_id: &str) {
    let Some(binding) = active_progress_binding() else { return };
    let mut bindings = progress_bindings().lock().expect("progress binding lock poisoned");
    if let Some(request_id) = binding.request_id.as_ref() {
        bindings.jobs_by_request.entry(request_key(request_id)).or_default().push(job_id.to_string());
    }
    bindings.by_job.insert(job_id.to_string(), binding);
}''', '''pub fn bind_job_to_active_scope(job_id: &str) {
    let request = ACTIVE_REQUEST.with(|active| active.borrow().clone());
    let progress = active_progress_binding();
    let cancelled = {
        let mut bindings = progress_bindings().lock().expect("progress binding lock poisoned");
        if let Some(key) = request.as_ref() {
            bindings.jobs_by_request.entry(key.clone()).or_default().push(job_id.to_string());
        }
        if let Some(binding) = progress {
            bindings.by_job.insert(job_id.to_string(), binding);
        }
        request.as_ref().is_some_and(|key| bindings.cancelled_requests.contains(key))
    };
    if cancelled {
        let _ = crate::ui::job_registry().request_cancel(job_id);
    }
}

/// 🛑️ `notifications/cancelled`'s effect: every job minted under `request_id` is asked to stop
/// through the job registry (which fires each producer's bound interrupt), and a request that has
/// not minted its job yet is remembered so the job is cancelled the moment it is minted.
pub fn cancel_request(request_id: &serde_json::Value) {
    let key = request_key(request_id);
    let jobs = {
        let mut bindings = progress_bindings().lock().expect("progress binding lock poisoned");
        if !bindings.cancelled_requests.contains(&key) {
            if bindings.cancelled_requests.len() == CANCELLED_REQUEST_MEMORY {
                bindings.cancelled_requests.pop_front();
            }
            bindings.cancelled_requests.push_back(key.clone());
        }
        bindings.jobs_by_request.get(&key).cloned().unwrap_or_default()
    };
    for job_id in jobs {
        let _ = crate::ui::job_registry().request_cancel(&job_id);
    }
}'''),
])

edit("🧭️protocol/🦀️.rs", [
('''        let Some(request_id) = request.params.as_ref().and_then(|params| params.get("requestId")) else {
            return DispatchOutcome::NoResponse;
        };
        for job_id in crate::notify::jobs_for_request(request_id) {
            let _ = crate::ui::job_registry().request_cancel(&job_id);
        }
        DispatchOutcome::NoResponse''', '''        if let Some(request_id) = request.params.as_ref().and_then(|params| params.get("requestId")) {
            crate::notify::cancel_request(request_id);
        }
        DispatchOutcome::NoResponse'''),
('''        let slot = self.notifications.clone()?;
        let request_id = request.id.as_ref().map(|id| serde_json::to_value(id).unwrap_or(serde_json::Value::Null));
        Some(crate::notify::ProgressBinding { token, request_id, slot })''', '''        let slot = self.notifications.clone()?;
        Some(crate::notify::ProgressBinding { token, slot })'''),
('''        let _progress = crate::notify::enter_progress_scope(self.progress_binding(request, params));''', '''        let _request = crate::notify::enter_request_scope(request.id.as_ref().map(|id| serde_json::to_value(id).unwrap_or(serde_json::Value::Null)).as_ref());
        let _progress = crate::notify::enter_progress_scope(self.progress_binding(request, params));'''),
])
print("ok")
