# UI14 Current Provider Transition Audit

The current selected extern provider map binds19 actual units and576 compiled Root source paths; declared uncompiled packages remain explicitly distinct. Surviving11→14 compiled comparison changes9 Value bodies while direct452 UI bodies remain identical. Named DWARF type queries yielded no structure offset/size evidence, so no arithmetic +56 field attribution is made.

Current Root has further advanced in 2 compiled provider bodies below. Full captured14initial/current bodies are held in the UI2 proposal; each physicalLayoutAcceptedfalse. The166040 descriptor is an actual14 measured candidate only, requiring fresh current owning original whole after joint source admission. No current Root size equality or old native acceptance is inferred.

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

