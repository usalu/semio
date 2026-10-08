# Current UI Layout3 Authority Refresh

UI2 independent review refused a newly advanced selected ordered-map Root body; UI2 receipt remains immutable and is not substituted. UI3 first green similarly refused before admission after the source-text assembly halted on a binary dependency. The corrected authority refresh compares bytes before decoding only actual changed Rust source bodies; raw refusal/full proposal/helper are retained. The same actual14 measured descriptor166040 candidate and original capacities/owner/stacks/threshold/assertions remain conserved. All complete current authority transitions below are physicalLayoutAcceptedfalse; no current native size equality is inferred.

Current14initial→Root complete transition diffs (3):

## 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs

```diff
--- 
+++ 
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

## 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs

```diff
--- 
+++ 
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

## 🧰️framework/🔨️modules/🧵️job/🦀️.rs

```diff
--- 
+++ 
@@ -1308,6 +1308,8 @@
     fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome;
     fn begin_close(&mut self);
     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep;
+    /// 📏️ Physical allocation required by the next close action; inquiry grants no release authority.
+    fn next_close_byte_demand(&self) -> usize { 0 }
     /// 🔔️ Registers the wake source that can advance a genuinely blocked close.
     fn register_close_wake(&self, _waker: &Waker) -> bool {
         false
@@ -2341,6 +2343,14 @@
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
@@ -2447,6 +2457,14 @@
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
@@ -2701,6 +2719,13 @@
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
@@ -2869,6 +2894,14 @@
     }
 }
 
+fn worker_job_authority_close_byte_demand<J: InteractiveJob>(authority: &WorkerJobAuthority<J>) -> usize {
+    if authority.quarantined_outcome.as_ref().is_some_and(|outcome| !outcome.terminal_is_empty()) || authority.outcome.as_ref().is_some_and(|outcome| !outcome.terminal_is_empty()) { return JOB_PAYLOAD_PAGE_BYTES; }
+    if authority.close_stage == 0 { return 0; }
+    if authority.preadmitted_fault.as_ref().is_some_and(|fault| !fault.terminal_is_empty()) { return JOB_PAYLOAD_PAGE_BYTES; }
+    if authority.close_stage == 1 { return authority.job.as_ref().map_or(0, InteractiveJob::next_close_byte_demand); }
+    0
+}
+
 fn worker_job_close_step<J: InteractiveJob>(inner: &WorkerJobSessionInner<J>, maximum_items: usize, maximum_bytes: usize) -> WorkerJobCloseStep {
     if inner.phase.compare_exchange(SESSION_CLOSE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
         return if inner.phase() == SESSION_EMPTY { WorkerJobCloseStep::Complete } else { WorkerJobCloseStep::Blocked };
@@ -3164,6 +3197,19 @@
 
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
@@ -3659,5 +3705,9 @@
 #[path = "🧪️tests/🔬️clock-stride/🦀️.rs"]
 mod clock_stride_tests;
 
+#[cfg(test)]
+#[path = "🧪️tests/📏️close-demand/🦀️.rs"]
+mod close_demand_tests;
+
 #[path = "🔎️reconcile/🧬️schema/🦀️.rs"]
 pub mod reconcile;

```

