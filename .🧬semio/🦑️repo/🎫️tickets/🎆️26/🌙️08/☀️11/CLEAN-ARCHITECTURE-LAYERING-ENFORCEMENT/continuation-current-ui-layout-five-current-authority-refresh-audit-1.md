# Current UI5 Source Authority Refresh

UI4 actual source controls are retained but refused for current admission after Job advanced. This successor joins all currently changed selected compiled14 Root inputs as complete initial/current source transitions and binds current World8. The native measured166040/520 is qualified to captured14; current physical layout is unaccepted and fresh owning original whole is mandatory. Original fixture stacks/threshold/capacity/owner/assertions remain exact.

```diff
--- captured14/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs
+++ current/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs
@@ -1,6 +1,6 @@
 //! 🗺️ Fixed-page ordered owners with resumable native key comparison and insertion.
 
-use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement, admit_retained_clone_scaffold_retirement};
+use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_close, admit_retained_clone_progress, admit_retained_clone_retirement, admit_retained_clone_scaffold_retirement, close_retained_binding};
 use crate::{SnapshotRetirementStep, retirement::RetireOwned};
 use serde::{Serialize, Serializer, ser::SerializeMap};
 use std::{cmp::Ordering, mem::size_of, sync::Arc};
@@ -351,6 +351,30 @@
         }
         self.source = None;
         Ok(SnapshotRetirementStep::Complete)
+    }
+
+    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
+        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "ordered map must begin close before granted retirement")); }
+        if !self.key_cursor.terminal_is_empty() {
+            if self.key_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
+            let step = self.key_cursor.close_granted(grant)?;
+            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained ordered-map key close")?.progress()));
+        }
+        if !self.value_cursor.terminal_is_empty() {
+            if self.value_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
+            let step = self.value_cursor.close_granted(grant)?;
+            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.value_cursor.terminal_is_empty(), "retained ordered-map value close")?.progress()));
+        }
+        if !self.close.is_empty() { return self.close.step_granted(grant); }
+        if let Some(step) = self.close.begin_granted(&mut self.key, grant)? { return Ok(step); }
+        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
+        if let Some(step) = self.close.begin_granted(&mut self.page_output, grant)? { return Ok(step); }
+        if !self.pages.is_empty() || self.pages.capacity() != 0 {
+            if let Some(step) = self.close.begin_default_granted(&mut self.pages, grant)? { return Ok(step); }
+        }
+        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
+        close_retained_binding(&mut self.source, grant)
     }
 
     fn terminal_is_empty(&self) -> bool {
```
```diff
--- captured14/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
+++ current/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
@@ -979,9 +979,9 @@
                     return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                 }
                 let bytes = size_of::<T::Cursor>();
-                if bytes > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
+                if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                 self.child = None;
-                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }));
+                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }));
             }
             let capacity = size_of::<T>();
             let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
```
```diff
--- captured14/🧰️framework/🔨️modules/🧵️job/🦀️.rs
+++ current/🧰️framework/🔨️modules/🧵️job/🦀️.rs
@@ -404,37 +404,12 @@
 //#region 📄️RetainedPayload
 pub const JOB_PAYLOAD_PAGE_BYTES: usize = 16 * 1024;
 
-/// 💰️ Pays one bounded close turn's byte grant into the fixed physical page granule and reports
-/// whether the page is now fully paid for.
-///
-/// A payload page is one `Box<[MaybeUninit<u8>; JOB_PAYLOAD_PAGE_BYTES]>`: the backing is released
-/// whole or not at all, however few logical bytes it carries. The grant a closing caller offers is
-/// its own per-turn budget, and it is NOT always a whole page — the store's retirement ladders run
-/// on a 4 KiB granule, a reactor drains its cursors on whatever the host turn has left. Refusing
-/// every sub-page grant outright (the shape this replaces) turned those callers into an unbounded
-/// spin: each turn answered `Pending { 0, 0 }` — "progress, call again" — while nothing could ever
-/// move, so a mounted session's own pre-admitted terminal-fault page pinned its close cursor before
-/// the job ever saw `begin_close` (ticket 26/09/18, the mounted presence capture law).
-///
-/// So the grant is ACCRUED instead: each turn spends at most what it was offered, the backing stays
-/// put until the accrued charge covers the whole physical page, and the turn that completes the
-/// charge is the turn that frees it. A caller offering a full page is unchanged — it pays the page
-/// in one turn and sees `released_bytes == JOB_PAYLOAD_PAGE_BYTES` — and a caller offering less now
-/// finishes in a bounded number of turns instead of never.
-///
-/// `Ok(paid)` means the page is paid for and must now be released; `Err(paid)` means the turn spent
-/// `paid` bytes of its grant and the backing is retained.
-fn charge_payload_page(charged: &mut usize, maximum_items: usize, maximum_bytes: usize) -> Result<usize, usize> {
-    if maximum_items == 0 || maximum_bytes == 0 {
+/// 📏️ Admits one indivisible physical page release from the current turn's authority.
+fn charge_payload_page(maximum_items: usize, maximum_bytes: usize) -> Result<usize, usize> {
+    if maximum_items == 0 || maximum_bytes < JOB_PAYLOAD_PAGE_BYTES {
         return Err(0);
     }
-    let paid = maximum_bytes.min(JOB_PAYLOAD_PAGE_BYTES - *charged);
-    *charged += paid;
-    if *charged < JOB_PAYLOAD_PAGE_BYTES {
-        return Err(paid);
-    }
-    *charged = 0;
-    Ok(paid)
+    Ok(JOB_PAYLOAD_PAGE_BYTES)
 }
 
 pub const JOB_PAYLOAD_OPERATION_PAGES: usize = 256;
@@ -616,12 +591,11 @@
     page_count: usize,
     length: usize,
     ledger: Option<Arc<JobPayloadOperationLedger>>,
-    charged: usize,
 }
 
 impl RetainedJobPayload {
     pub fn empty(stream: JobPayloadStream) -> Self {
-        Self { stream, pages: ManuallyDrop::new(std::array::from_fn(|_| None)), page_count: 0, length: 0, ledger: None, charged: 0 }
+        Self { stream, pages: ManuallyDrop::new(std::array::from_fn(|_| None)), page_count: 0, length: 0, ledger: None }
     }
 
     pub fn len(&self) -> usize {
@@ -646,6 +620,11 @@
 
     pub fn reader(&self) -> RetainedJobPayloadReader<'_> {
         RetainedJobPayloadReader { payload: self, page: 0 }
+    }
+
+    /// 📏️ Queries the exact next physical release without spending authority.
+    pub fn next_close_byte_demand(&self) -> usize {
+        if self.page_count == 0 { 0 } else { JOB_PAYLOAD_PAGE_BYTES }
     }
 
     pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep {
@@ -654,7 +633,7 @@
             return JobPayloadCloseStep::Complete;
         }
         let index = self.pages.iter().position(Option::is_some).expect("retained payload page count matches occupied pages");
-        let released_bytes = match charge_payload_page(&mut self.charged, maximum_items, maximum_bytes) {
+        let released_bytes = match charge_payload_page(maximum_items, maximum_bytes) {
             Ok(paid) => paid,
             Err(paid) => return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: paid },
         };
@@ -733,7 +712,6 @@
     rejected: ManuallyDrop<Option<JobPayloadPageSource>>,
     staged: ManuallyDrop<Option<(Arc<JobPayloadOperationLedger>, JobPayloadPageSource, usize)>>,
     sealed: bool,
-    charged: usize,
 }
 
 impl std::fmt::Debug for RetainedJobPayloadWriter {
@@ -744,7 +722,7 @@
 
 impl RetainedJobPayloadWriter {
     pub fn new(stream: JobPayloadStream) -> Self {
-        Self { payload: ManuallyDrop::new(Some(RetainedJobPayload::empty(stream))), rejected: ManuallyDrop::new(None), staged: ManuallyDrop::new(None), sealed: false, charged: 0 }
+        Self { payload: ManuallyDrop::new(Some(RetainedJobPayload::empty(stream))), rejected: ManuallyDrop::new(None), staged: ManuallyDrop::new(None), sealed: false }
     }
 
     pub fn take_rejected_source(&mut self) -> Option<JobPayloadPageSource> {
@@ -783,10 +761,16 @@
         self.sealed = true;
     }
 
+    /// 📏️ Queries the staged, refused, or committed page currently owned by the close phase.
+    pub fn next_close_byte_demand(&self) -> usize {
+        if self.staged.is_some() || self.rejected.is_some() { JOB_PAYLOAD_PAGE_BYTES }
+        else { self.payload.as_ref().map_or(0, RetainedJobPayload::next_close_byte_demand) }
+    }
+
     pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep {
         self.sealed = true;
         if let Some((ledger, _, _)) = self.staged.as_ref() {
-            let released_bytes = match charge_payload_page(&mut self.charged, maximum_items, maximum_bytes) {
+            let released_bytes = match charge_payload_page(maximum_items, maximum_bytes) {
                 Ok(paid) => paid,
                 Err(paid) => return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: paid },
             };
@@ -797,7 +781,7 @@
             return JobPayloadCloseStep::Pending { released_items: 1, released_bytes };
         }
         if self.rejected.is_some() {
-            let released_bytes = match charge_payload_page(&mut self.charged, maximum_items, maximum_bytes) {
+            let released_bytes = match charge_payload_page(maximum_items, maximum_bytes) {
                 Ok(paid) => paid,
                 Err(paid) => return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: paid },
             };
@@ -1252,6 +1236,18 @@
         matches!(self, StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_))
     }
 
+    /// 📏️ Reports the next owned result page's physical release extent.
+    pub fn next_close_byte_demand(&self) -> usize {
+        match self {
+            StepOutcome::Yield | StepOutcome::Cancelled => 0,
+            StepOutcome::PreviewReady(payload) => payload.next_close_byte_demand(),
+            StepOutcome::CheckpointReady(checkpoint) => checkpoint.state.next_close_byte_demand(),
+            StepOutcome::Complete(candidate) if !candidate.state.terminal_is_empty() => candidate.state.next_close_byte_demand(),
+            StepOutcome::Complete(candidate) => candidate.output.next_close_byte_demand(),
+            StepOutcome::Fault(fault) => fault.detail.next_close_byte_demand(),
+        }
+    }
+
     pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep {
         match self {
             StepOutcome::Yield | StepOutcome::Cancelled => JobPayloadCloseStep::Complete,
@@ -1308,6 +1304,8 @@
     fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome;
     fn begin_close(&mut self);
     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep;
+    /// 📏️ Physical allocation required by the next close action; inquiry grants no release authority.
+    fn next_close_byte_demand(&self) -> usize { 0 }
     /// 🔔️ Registers the wake source that can advance a genuinely blocked close.
     fn register_close_wake(&self, _waker: &Waker) -> bool {
         false
@@ -2095,7 +2093,7 @@
     }
     let mut pages = std::array::from_fn(|_| None);
     pages[0] = Some(JobPayloadPage { source, length: bytes.len() });
-    Ok(RetainedJobPayload { stream, pages: ManuallyDrop::new(pages), page_count: 1, length: bytes.len(), ledger: Some(Arc::clone(ledger)), charged: 0 })
+    Ok(RetainedJobPayload { stream, pages: ManuallyDrop::new(pages), page_count: 1, length: bytes.len(), ledger: Some(Arc::clone(ledger)) })
 }
 
 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
@@ -2341,6 +2339,14 @@
         self.checked_out.as_mut().map(WorkerJobOutcome::job_mut)
     }
 
+    /// 🧮️ Inspect the exact retained owner without advancing or enlarging its close grant.
+    pub fn next_close_byte_demand(&self) -> Result<usize, WorkerJobContention> {
+        match self.checked_out.as_ref().and_then(|owner| owner.authority.as_ref()) {
+            Some(authority) => Ok(worker_job_authority_close_byte_demand(authority)),
+            None => self.session.next_close_byte_demand(),
+        }
+    }
+
     pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
         let owner = self.checked_out.take().ok_or_else(|| self.session.contention())?;
         owner.resume().map_err(|owner| {
@@ -2447,6 +2453,14 @@
         self.checked_out.as_mut()?.authority.as_mut()?.job.as_mut()
     }
 
+    /// 🧭️ Forward the exact owner demand while preserving checkout and worker ownership.
+    pub fn next_close_byte_demand(&self) -> Result<usize, WorkerJobContention> {
+        match self.checked_out.as_ref().and_then(|owner| owner.authority.as_ref()) {
+            Some(authority) => Ok(worker_job_authority_close_byte_demand(authority)),
+            None => self.session.next_close_byte_demand(),
+        }
+    }
+
     pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
         let owner = self.checked_out.take().ok_or_else(|| self.session.contention())?;
         owner.resume().map_err(|owner| {
@@ -2701,6 +2715,13 @@
 }
 
 impl<J: InteractiveJob> WorkerJobSessionAdmissionRejected<J> {
+    /// 🪙️ Publish the physical demand of the next retained rejection owner.
+    pub fn next_close_byte_demand(&self) -> usize {
+        if self.close_stage == 0 {
+            self.job.as_ref().map_or(0, InteractiveJob::next_close_byte_demand)
+        } else if self.fault_source.is_some() { JOB_PAYLOAD_PAGE_BYTES } else { 0 }
+    }
+
     pub fn begin_close(&mut self) {
         if self.closing {
             return;
@@ -2869,6 +2890,14 @@
     }
 }
 
+fn worker_job_authority_close_byte_demand<J: InteractiveJob>(authority: &WorkerJobAuthority<J>) -> usize {
+    if let Some(outcome) = authority.quarantined_outcome.as_ref().or(authority.outcome.as_ref()) { return outcome.next_close_byte_demand(); }
+    if authority.close_stage == 0 { return 0; }
+    if let Some(fault) = authority.preadmitted_fault.as_ref() { return fault.next_close_byte_demand(); }
+    if authority.close_stage == 1 { return authority.job.as_ref().map_or(0, InteractiveJob::next_close_byte_demand); }
+    0
+}
+
 fn worker_job_close_step<J: InteractiveJob>(inner: &WorkerJobSessionInner<J>, maximum_items: usize, maximum_bytes: usize) -> WorkerJobCloseStep {
     if inner.phase.compare_exchange(SESSION_CLOSE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
         return if inner.phase() == SESSION_EMPTY { WorkerJobCloseStep::Complete } else { WorkerJobCloseStep::Blocked };
@@ -3164,6 +3193,19 @@
 
     pub fn begin_close(&self) -> WorkerJobCloseStep {
         worker_job_begin_close(&self.inner)
+    }
+
+    /// 🔐️ Inspect under the same exclusive phase admission used by the close cursor.
+    pub fn next_close_byte_demand(&self) -> Result<usize, WorkerJobContention> {
+        let phase = self.inner.phase();
+        if phase == SESSION_EMPTY { return Ok(0); }
+        if matches!(phase, SESSION_SUBMITTED | SESSION_TRANSITION | SESSION_CHECKED_OUT) || self.inner.phase.compare_exchange(phase, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
+            return Err(WorkerJobContention::CheckedOut(self.inner.generation));
+        }
+        let authority = unsafe { self.inner.take_authority() };
+        let demand = worker_job_authority_close_byte_demand(&authority);
+        unsafe { self.inner.put_authority(authority, phase) };
+        Ok(demand)
     }
 
     pub fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> WorkerJobCloseStep {
@@ -3659,5 +3701,9 @@
 #[path = "🧪️tests/🔬️clock-stride/🦀️.rs"]
 mod clock_stride_tests;
 
+#[cfg(test)]
+#[path = "🧪️tests/📏️close-demand/🦀️.rs"]
+mod close_demand_tests;
+
 #[path = "🔎️reconcile/🧬️schema/🦀️.rs"]
 pub mod reconcile;
```
