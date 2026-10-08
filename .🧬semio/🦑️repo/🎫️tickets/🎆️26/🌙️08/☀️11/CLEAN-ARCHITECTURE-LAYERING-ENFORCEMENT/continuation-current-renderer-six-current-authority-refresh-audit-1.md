# Current Renderer6 Source Authority Refresh

The first red attempt refused preflight Store drift and produced no finite controls. This is preserved as refusal, not a TDD outcome. Current Root Store adds `EditReplay.operation_progress: bool`; its complete captured/current transition declares that field explicitly, replacing the now-invalid statement-only qualification. The current Store fullbody and exact declared field are required; no old physical size inference survives. The initial hunk assembly refused an ambiguous nonunique fragment without writing any proposal; full endpoint recipes bind every byte.

The sole Renderer fixture pair proposes Engine90224, Kernel25672 and World28440 measured by compiled14 diagnostic2. All stacks/threshold/capacities/owners and original assertion bodies are conserved. Current Root producer changes prevent physical layout acceptance; only fresh original common native whole can accept current sizes. World8 provides current coupling while World5/7 size comparisons remain qualified prior source observations.

```diff
--- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-11/product/initial-captured-input/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
+++ /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
@@ -348,6 +348,8 @@
     }
 
     fn terminal_is_empty(&self) -> bool { self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() }
+    fn next_close_byte_demand(&self) -> usize { self.active.as_ref().map_or(1, |active| active.next_close_byte_demand()) }
+
 }
 
 impl<P, Mu: self::Mutation<P>> Drop for ArtifactHistoryReadRetirement<P, Mu> {
@@ -9646,6 +9648,10 @@
     }
 
     fn record_release_fault(&mut self, diagnostic: OwnedSchemaDecodeDiagnostic) {
+        #[cfg(test)] {
+            static FAULTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
+            if FAULTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 8 { eprintln!("[DEBUG] envelope release fault: code={} admitted-demand={} retained-limit={} released={}", diagnostic.code, self.maximum_field_close_byte_demand, self.maximum_retained_field_close_bytes, self.released_field_bytes); }
+        }
         if !matches!(self.state, ArtifactEnvelopeDecodeState::ReleaseFault(_)) {
             self.state = ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic);
         }
@@ -10863,17 +10869,25 @@
             && self.retirement.is_none()
     }
 
+    fn close_retained_retirement(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
+        let retirement = self.retirement.as_mut().expect("fresh VCS retained retirement is present");
+        if retirement.terminal_is_empty() {
+            let extent = std::mem::size_of_val(retirement.as_ref());
+            if maximum_items == 0 || maximum_bytes < extent { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+            drop(self.retirement.take());
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: extent });
+        }
+        match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("artifact-envelope.fresh-vcs-retirement-fault"))? {
+            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
+            SnapshotRetirementStep::Pending { .. } => Err(Self::diagnostic("artifact-envelope.fresh-vcs-retirement-over-grant")),
+            SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
+            SnapshotRetirementStep::Complete => Err(Self::diagnostic("artifact-envelope.fresh-vcs-retirement-false-terminal")),
+            SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
+        }
+    }
+
     fn close_owned_snapshot(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
-        if let Some(retirement) = self.retirement.as_mut() {
-            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("artifact-envelope.fresh-vcs-retirement-fault"))? {
-                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
-                    drop(self.retirement.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-                SnapshotRetirementStep::Complete => Err(Self::diagnostic("artifact-envelope.fresh-vcs-retirement-false-terminal")),
-                step => Ok(step),
-            };
-        }
+        if self.retirement.is_some() { return self.close_retained_retirement(maximum_items, maximum_bytes); }
         if let Some(vcs) = self.value.take() {
             self.retirement = Some(Box::new(ArtifactStoreVcsRetirement::new(vcs, self.initial_snapshot_factory.clone(), self.mutation_factory.clone())));
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
@@ -10891,16 +10905,7 @@
     }
 
     fn close_pending_history(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
-        if let Some(retirement) = self.retirement.as_mut() {
-            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("artifact-envelope.history-retirement-fault"))? {
-                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
-                    drop(self.retirement.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-                SnapshotRetirementStep::Complete => Err(Self::diagnostic("artifact-envelope.history-retirement-false-terminal")),
-                step => Ok(step),
-            };
-        }
+        if self.retirement.is_some() { return self.close_retained_retirement(maximum_items, maximum_bytes); }
         if let Some(active) = self.active.as_mut() {
             let step = match active {
                 ArtifactEnvelopeFreshVcsActive::Snapshot { .. } => Ok(SnapshotRetirementStep::Complete),
@@ -11011,14 +11016,17 @@
                 return Ok(demand);
             }
         }
-        if self.retirement.is_some() || self.active.as_ref().is_some_and(|active| !matches!(active, ArtifactEnvelopeFreshVcsActive::Snapshot { .. })) {
-            return Ok(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
-        }
+        if let Some(retirement) = self.retirement.as_ref() {
+            let demand = if retirement.terminal_is_empty() { std::mem::size_of_val(retirement.as_ref()) } else { retirement.next_close_byte_demand() };
+            if demand > self.maximum_close_byte_demand() { return Err(Self::diagnostic("artifact-envelope.fresh-vcs-close-demand-over-admitted-maximum")); }
+            return Ok(demand);
+        }
+        if self.active.as_ref().is_some_and(|active| !matches!(active, ArtifactEnvelopeFreshVcsActive::Snapshot { .. })) { return Ok(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES); }
         Ok(0)
     }
 
     fn maximum_close_byte_demand(&self) -> usize {
-        self.maximum_snapshot_close_byte_demand.max(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)
+        self.maximum_snapshot_close_byte_demand.max(ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES)
     }
 
     fn maximum_retained_close_bytes(&self) -> usize {
@@ -16158,6 +16166,11 @@
 
     pub fn share_current(&self) -> Arc<P> { Arc::clone(self.current.as_ref().expect("initialization retains its current projection")) }
 
+    /// 📏️ Publish the next retained allocation, including its terminal erased cursor box.
+    pub fn next_close_byte_demand(&self) -> usize {
+        self.close_active.as_ref().map_or(usize::from(!self.taken), |active| if active.terminal_is_empty() { std::mem::size_of_val(active.as_ref()) } else { active.next_close_byte_demand() })
+    }
+
     fn replace_current_alias(&mut self, current: Arc<P>) -> Arc<P> { self.current.replace(current).expect("initialization retains its current projection") }
 
     pub fn adopt_current_owned(&mut self, current: P, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Result<(), P>
@@ -16173,8 +16186,14 @@
         if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
         let Some(active) = self.close_active.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
         match active.close_step(maximum_items.min(1), maximum_bytes)? {
-            SnapshotRetirementStep::Complete if active.terminal_is_empty() => { drop(self.close_active.take()); Ok(SnapshotRetirementStep::Complete) }
+            SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
+                let bytes = std::mem::size_of_val(active.as_ref());
+                if maximum_bytes < bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+                drop(self.close_active.take());
+                Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes })
+            }
             SnapshotRetirementStep::Complete => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "initialization displaced snapshot has no terminal-empty witness")),
+            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store initialization child exceeded its exact close grant")),
             step => Ok(step),
         }
     }
@@ -16349,17 +16368,8 @@
         if maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if let Some(active) = self.close_active.as_mut() {
-            return match active.close_step(maximum_items.min(1), maximum_bytes)? {
-                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
-                SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store initialization child exceeded its exact close grant")),
-                SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
-                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
-                    drop(self.close_active.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-                SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store initialization child reported Complete without terminal-empty ownership")),
-            };
+        if self.close_active.is_some() {
+            return self.settle_current_retirement_step(maximum_items, maximum_bytes);
         }
         match self.close_phase {
             0 => {
@@ -20589,6 +20599,20 @@
                 if stage.edit.id.is_empty() || stage.edit.id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || authority.base_applied_edit_count != self.applied_edit_ids.len() {
                     return Err(VcsError::ValidationFailed("domain-prepared batched candidate failed its live cursor authority".into()));
                 }
+                if let Some(edit_id) = self.batch_amend_target(publication.transaction.as_ref()) {
+                    let existing = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its open edit".into()))?;
+                    let capacity = existing.inverse.len().checked_add(stage.edit.inverse.len()).ok_or_else(|| VcsError::ValidationFailed("batched inverse capacity overflow".into()))?;
+                    if let Some(bytes) = existing.inverse.next_capacity_allocation_bytes(capacity).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
+                        if bytes > grant.maximum_bytes {
+                            return Err(VcsError::ValidationFailed("batched inverse backing cannot fit the admitted allocation grant".into()));
+                        }
+                        let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+                        if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
+                            return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
+                        }
+                        return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
+                    }
+                }
                 publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                 Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
             }
@@ -20806,6 +20830,7 @@
         }
         if publication.stage.is_none() {
             let mut staged = Box::new(Self::empty_batch_stage(&authority));
+            staged.edit.inverse = semio_framework_value::list::PagedList::with_payload_page_bytes(grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
             staged.mutation_retirement = (*self.mutation_retirement_factory).clone();
             staged.snapshot_retirement = (*self.snapshot_retirement_factory).clone();
             if staged.mutation_retirement.is_none() || staged.snapshot_retirement.is_none() {
@@ -20817,11 +20842,15 @@
             publication.stage = Some(staged);
         }
         let inverse = &mut publication.stage.as_mut().expect("validated staged batch edit").edit.inverse;
-        if let Some(bytes) = inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
+        let backing = if publication.transaction_open { inverse.next_capacity_allocation_bytes(publication.footprint.work_items) } else { inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items) };
+        if let Some(bytes) = backing.map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
             if bytes > grant.maximum_bytes {
-                return Ok(ArtifactStoreOneItemAdvance::Blocked);
-            }
-            inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+                return Err(VcsError::ValidationFailed("batched inverse backing cannot fit the admitted allocation grant".into()));
+            }
+            let step = if publication.transaction_open { inverse.reserve_capacity_one(publication.footprint.work_items, grant.maximum_bytes) } else { inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes) }.map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+            if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
+                return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
+            }
             return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
         }
         let owner = publication.preparation.as_mut().expect("validated batched fold preparation owner");
@@ -22245,11 +22274,7 @@
         let budget = pending.local.as_ref().and(self.local_replay_budget).or(self.replay_budget).unwrap_or(ReplayTurnBudget::operations(usize::MAX));
         let deadline = budget.turn_deadline(deadline_us);
         let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
-        let mut replayed = 0usize;
-        let step = replay.step(&self.envelope.vcs.edits, &mut || {
-            replayed += 1;
-            budget.turn_ends(replayed, deadline)
-        });
+        let step = replay.step_operations(&self.envelope.vcs.edits, budget.operations, &mut || budget.turn_ends(0, deadline));
         match step {
             Ok(ReplayStep::Pending(progress)) => {
                 self.pending_reprojection = Some(pending);
@@ -25035,6 +25060,7 @@
     from: usize,
     prefix_until: usize,
     prefix_work_progress: bool,
+    operation_progress: bool,
     cursor: usize,
     operation: usize,
     schema: String,
@@ -25099,6 +25125,7 @@
             from,
             prefix_until: from,
             prefix_work_progress: false,
+            operation_progress: false,
             cursor: from,
             operation: 0,
             schema: schema.to_string(),
@@ -25224,13 +25251,28 @@
 
     /// ⏭️ Replays operations until the replay finishes or `deadline`, asked after every operation, answers `true`.
     pub fn step(&mut self, edits: &impl ReplayEdits<Mutation>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
+        self.step_with_operation_cap(edits, usize::MAX, deadline)
+    }
+
+    /// 🔢️ Caps folded mutations while retaining every structural deadline and owner check.
+    pub fn step_operations(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
+        if !self.operation_progress {
+            if self.progress.done != 0 { return Err(VcsError::ValidationFailed("operation-counted replay authority must be selected before work begins".into())); }
+            self.progress.total = self.progress.total.saturating_sub(u32::try_from(self.order.len().saturating_sub(self.from)).unwrap_or(u32::MAX));
+            self.operation_progress = true;
+        }
+        self.step_with_operation_cap(edits, maximum_operations, deadline)
+    }
+
+    fn step_with_operation_cap(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
+        if maximum_operations == 0 && !self.finished { return Ok(ReplayStep::Pending(self.progress)); }
+        let mut completed_operations = 0usize;
         while !self.finished {
             if let Some(retirement) = self.replay_retirement.as_mut() {
                 let maximum_bytes = retirement.next_close_byte_demand().max(4096);
                 let step = retirement.close_step(1, maximum_bytes).map_err(VcsError::InverseRefused)?;
                 if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                     if released_items > 1 || released_bytes > maximum_bytes { return Err(VcsError::ValidationFailed("replay cleanup exceeded its admitted grant".into())); }
-                    if released_bytes > 4096 { eprintln!("[DEBUG] replay cleanup exact capacity release bytes={released_bytes} admitted={maximum_bytes}"); }
                 }
                 if step == SnapshotRetirementStep::Complete {
                     if !retirement.terminal_is_empty() { return Err(VcsError::ValidationFailed("replay cleanup reported terminal with live owners".into())); }
@@ -25260,8 +25302,9 @@
                     if self.operation_preparation_factory.is_some() {
                         if !self.step_prepared_operation(edit, true, deadline)? { deadline(); return Ok(ReplayStep::Pending(self.progress)); }
                         self.operation += 1;
+                    completed_operations = completed_operations.saturating_add(1);
                         if self.prefix_work_progress { self.progress.done = self.progress.done.saturating_add(1); }
-                        if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                        if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                         continue;
                     }
                     let index = self.operation;
@@ -25276,8 +25319,9 @@
                         if let (Some(next), _) = fold_operation::<P, Mutation>(state, operation, index as u32) { <Arc<P> as FoldProjection<P, Mutation>>::advance(state, next); }
                     }
                     self.operation += 1;
+                    completed_operations = completed_operations.saturating_add(1);
                     if self.prefix_work_progress { self.progress.done = self.progress.done.saturating_add(1); }
-                    if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                    if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                     continue;
                 }
                 self.cursor += 1;
@@ -25289,21 +25333,23 @@
                 if self.operation_preparation_factory.is_some() {
                     if !self.step_prepared_operation(edit, false, deadline)? { deadline(); return Ok(ReplayStep::Pending(self.progress)); }
                     self.operation += 1;
+                    completed_operations = completed_operations.saturating_add(1);
                     self.progress.done = self.progress.done.saturating_add(1);
-                    if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                    if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                     continue;
                 }
                 self.replay_operation(edit)?;
                 self.operation += 1;
+                    completed_operations = completed_operations.saturating_add(1);
                 self.progress.done = self.progress.done.saturating_add(1);
-                if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                 continue;
             }
             if !self.step_close_edit(&edit.id)? {
                 if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
                 continue;
             }
-            self.progress.done = self.progress.done.saturating_add(1);
+            if !self.operation_progress { self.progress.done = self.progress.done.saturating_add(1); }
             self.cursor += 1;
             self.operation = 0;
             self.record();
```
```diff
--- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-11/product/initial-captured-input/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
+++ /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
@@ -59,7 +59,7 @@
     /// 🔗️ Retains a projected native field through the actual immutable root lease.
     pub fn project_owned<U: ?Sized + Sync, F>(&self, discriminator: usize, project: F) -> RetainedOwnedProjection<U>
     where F: for<'source> FnOnce(&'source T) -> &'source U {
-        RetainedOwnedProjection::from_source(self.borrow().project(discriminator, project))
+        unsafe { RetainedOwnedProjection::from_source(self.borrow().project(discriminator, project)) }
     }
 }
 
@@ -153,21 +153,27 @@
 #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
 pub struct RetainedCloneGrant {
     pub maximum_items: usize,
-    /// 🧮️ Bounds payload copying and completed-child scaffold release within one clone turn.
+    /// 🧮️ Bounds payload copying within one clone turn.
     pub maximum_copy_bytes: usize,
     pub maximum_capacity_bytes: usize,
+    /// ♻️ Bounds physical owner and scaffold release independently of copying.
+    pub maximum_release_bytes: usize,
     pub maximum_depth: usize,
 }
 
 impl RetainedCloneGrant {
     /// 🎟️ Admits one structural allocation while reserving no payload-copy credit.
     pub fn one_capacity_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
-        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: maximum_bytes, maximum_depth }
+        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: maximum_bytes, maximum_release_bytes: 0, maximum_depth }
     }
 
     /// 🎟️ Admits one payload turn while reserving no allocation-capacity credit.
     pub fn one_payload_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
-        Self { maximum_items: 1, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: 0, maximum_depth }
+        Self { maximum_items: 1, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth }
+    }
+    /// ♻️ Admits one release turn without copy or allocation credit.
+    pub fn one_release_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
+        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: maximum_bytes, maximum_depth }
     }
 }
 
@@ -176,6 +182,7 @@
     pub copied_items: usize,
     pub copied_bytes: usize,
     pub retained_capacity_bytes: usize,
+    pub released_bytes: usize,
 }
 
 impl RetainedCloneProgress {
@@ -183,17 +190,18 @@
         Ok(Self {
             copied_items: self.copied_items.checked_add(other.copied_items).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone item progress overflow"))?,
             copied_bytes: self.copied_bytes.checked_add(other.copied_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone byte progress overflow"))?,
+            released_bytes: self.released_bytes.checked_add(other.released_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone release progress overflow"))?,
             retained_capacity_bytes: self.retained_capacity_bytes.checked_add(other.retained_capacity_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained clone capacity progress overflow"))?,
         })
     }
 
     pub fn fits(self, grant: RetainedCloneGrant) -> bool {
-        self.copied_items <= grant.maximum_items && self.copied_bytes <= grant.maximum_copy_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes
+        self.copied_items <= grant.maximum_items && self.copied_bytes <= grant.maximum_copy_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes && self.released_bytes <= grant.maximum_release_bytes
     }
 }
 
 pub fn admit_retained_clone_progress(grant: RetainedCloneGrant, progress: RetainedCloneProgress, scope: &str) -> Result<RetainedCloneProgress, crate::ValueError> {
-    if progress.fits(grant) { Ok(progress) } else { Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, format!("{scope} exceeded its retained clone item, copy, or capacity grant"))) }
+    if progress.fits(grant) { Ok(progress) } else { Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, format!("{scope} exceeded its retained clone item, copy, capacity, or release grant"))) }
 }
 
 /// 🛡️ Checks both granted closure work and the child's exact terminal ownership witness.
@@ -343,7 +351,7 @@
         let Some(retirement) = self.controlled.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
         if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         if retirement.terminal_is_empty() {
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: self.controlled_bytes, retained_capacity_bytes: 0 };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: self.controlled_bytes };
             if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
             self.controlled = None;
             self.controlled_bytes = 0;
@@ -365,7 +373,7 @@
 pub fn close_retained_binding(binding: &mut Option<RetainedCloneBinding>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
     match RetainedCloneBinding::close_one(binding, grant.maximum_items)? {
         SnapshotRetirementStep::Complete => Ok(RetainedCloneStep::Complete(Default::default())),
-        SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 })),
+        SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes })),
         SnapshotRetirementStep::Blocked => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained binding close blocked")),
     }
 }
@@ -395,12 +403,12 @@
         if self.spent {
             return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is spent"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         if self.value.is_some() {
             return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
         }
-        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0 };
+        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0, released_bytes: 0 };
         if !progress.fits(grant) {
             return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
         }
@@ -482,13 +490,13 @@
         if self.closing {
             return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         let source = source.get();
         match self.phase {
             0 => {
                 let planned = source.len();
-                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned };
+                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                 if !progress.fits(grant) {
                     return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                 }
@@ -525,7 +533,7 @@
                     return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                 }
                 output.push_str(&source[start..end]);
-                let progress = RetainedCloneProgress { copied_items: 0, copied_bytes: end - start, retained_capacity_bytes: 0 };
+                let progress = RetainedCloneProgress { copied_items: 0, copied_bytes: end - start, retained_capacity_bytes: 0, released_bytes: 0 };
                 Ok(RetainedCloneStep::Progress(progress))
             }
             2 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
@@ -604,12 +612,12 @@
         if self.closing {
             return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         let source_value = source.get();
         if self.phase == 0 {
             let planned = source_value.len().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone capacity overflow"))?;
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
             if !progress.fits(grant) {
                 return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
             }
@@ -633,8 +641,7 @@
                 maximum_items: grant.maximum_items.saturating_sub(used.copied_items),
                 maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(used.copied_bytes),
                 maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(used.retained_capacity_bytes),
-                maximum_depth: grant.maximum_depth,
-            };
+                maximum_depth: grant.maximum_depth, maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(used.released_bytes) };
             if let Some(child) = self.child.as_mut() {
                 if self.child_value.is_some() {
                     if !child.terminal_is_empty() {
@@ -669,7 +676,7 @@
                 used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                 return Ok(RetainedCloneStep::Complete(used));
             }
-            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 {
+            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 && remaining.maximum_release_bytes == 0 {
                 return Ok(RetainedCloneStep::Progress(used));
             }
             let child = self.child.get_or_insert_with(T::retained_clone_cursor);
@@ -811,7 +818,7 @@
             return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
         }
         let first = self.source.is_none();
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         let source_value = source.get();
         if first {
@@ -957,7 +964,7 @@
         if self.output.is_some() {
             return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         if self.value.is_some() {
             if let Some(child) = self.child.as_mut() {
@@ -972,12 +979,12 @@
                     return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                 }
                 let bytes = size_of::<T::Cursor>();
-                if bytes > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
+                if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                 self.child = None;
-                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0 }));
+                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }));
             }
             let capacity = size_of::<T>();
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
             if !progress.fits(grant) {
                 return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
             }
@@ -990,7 +997,7 @@
                 return Err(crate::ValueError::new(crate::ValueRefusalKind::DepthLimit, "retained box clone structural depth limit exceeded"));
             }
             let capacity = size_of::<T::Cursor>();
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
             if !progress.fits(grant) {
                 return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
             }
@@ -1070,7 +1077,7 @@
         if let Some(child) = self.child.as_mut() {
             if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
             if !child.terminal_is_empty() { let step = child.close_granted(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T::Cursor>(), retained_capacity_bytes: 0 };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: size_of::<T::Cursor>() };
             if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
             self.child = None;
             return Ok(RetainedCloneStep::Progress(progress));
@@ -1126,7 +1133,7 @@
                 if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is closing")); }
                 if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is spent")); }
                 if self.output.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
-                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
                 if self.draining {
                     match self.phase {
```
```diff
--- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-11/product/initial-captured-input/🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs
+++ /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs
@@ -36,7 +36,7 @@
             let demand = destination.chunks.next_allocation_bytes()?;
             if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
             let progress = destination.chunks.reserve_one(demand).map_err(|error| ValueError::from(error.refusal()))?;
-            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, copied_bytes: 0 }));
+            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, copied_bytes: 0, released_bytes: 0 }));
         }
         if self.pending.is_none() {
             let mut end = (self.position + PAGED_UTF8_CHUNK_BYTES).min(source.len());
@@ -50,7 +50,7 @@
             self.pending = Some(pending);
             if capacity > grant.maximum_capacity_bytes { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append allocator exceeded its admitted chunk")); }
             self.end = end;
-            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity }));
+            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 }));
         }
         if self.position < self.end {
             let mut end = self.position.saturating_add(grant.maximum_copy_bytes).min(self.end);
@@ -59,7 +59,7 @@
             let bytes = end - self.position;
             self.pending.as_mut().expect("native append chunk owner").push_str(&source[self.position..end]);
             self.position = end;
-            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0 }));
+            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }));
         }
         let bytes = std::mem::size_of::<String>();
         if bytes > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
@@ -68,7 +68,7 @@
         if let Err(pending) = destination.chunks.push_reserved(pending) { self.pending = Some(pending); return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append lost its admitted slot")); }
         destination.byte_len += length;
         self.destination = Some((destination_identity, destination.byte_len));
-        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0 }))
+        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
     }
     pub fn begin_close(&mut self) -> bool { if self.closing { return false; } self.closing = true; true }
     pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
```
```diff
--- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-11/product/initial-captured-input/🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs
+++ /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs
@@ -259,7 +259,7 @@
                 if self.closing { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct clone cursor is closing")); }
                 if self.spent { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct clone cursor is spent")); }
                 if self.output.is_some() { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())); }
-                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
+                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                 source.bind(&mut self.source)?;
                 if self.draining {
                     return match self.phase { #(#drain_arms,)* _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct drain state is invalid")) };
@@ -411,10 +411,10 @@
                     #field_count => {
                         if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                         let bytes = 0usize #(.saturating_add(#construct_bytes))*;
-                        if bytes > grant.maximum_copy_bytes { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
+                        if bytes > grant.maximum_release_bytes { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                         self.output = Some(#construct);
                         self.phase += 1;
-                        Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0 }))
+                        Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }))
                     }
                     _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone state is invalid")),
                 }
@@ -479,7 +479,7 @@
                 if self.closing { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone cursor is closing")); }
                 if self.spent { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone cursor is spent")); }
                 if self.output.is_some() { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())); }
-                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
+                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                 source.bind(&mut self.source)?;
                 if self.variant.is_none() {
                     if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
```
```diff
--- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-11/product/initial-captured-input/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs
+++ /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs
@@ -58,7 +58,7 @@
         if self.closing {
             return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet retained clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         match self.phase {
             0 => {
@@ -86,7 +86,7 @@
                     for index in start..start + count {
                         self.bytes.push_reserved(*source.get(index).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet clone source changed"))?).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet clone lost its reserved capacity"))?;
                     }
-                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, retained_capacity_bytes: 0 }));
+                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, retained_capacity_bytes: 0, released_bytes: 0 }));
                 }
                 if grant.maximum_items == 0 {
                     return Ok(RetainedCloneStep::Progress(Default::default()));
@@ -193,7 +193,7 @@
         if self.closing {
             return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         match self.phase {
             0 => match self.inner.advance(source.project(1, PagedUtf8::retained_chunks), grant)? {
@@ -226,7 +226,7 @@
                 }
                 self.output = Some(PagedUtf8::from_retained_chunks_cloned(self.chunks.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone lost its chunks"))?, source.get().len()));
                 self.phase = 3;
-                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), retained_capacity_bytes: 0 }))
+                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), retained_capacity_bytes: 0, released_bytes: 0 }))
             }
             3 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
             _ => Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone cursor is spent")),
@@ -421,7 +421,7 @@
         if self.closing {
             return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged map retained clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         match self.phase {
             0 => match self.inner.advance(source.project(1, PagedMap::retained_entries), grant)? {
```
```diff
--- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-14/ui/initial-captured-input/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs
+++ /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs
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
--- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-14/ui/initial-captured-input/🧰️framework/🔨️modules/🧵️job/🦀️.rs
+++ /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs
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

The red2 preflight exposed JavaScript replacement-string interpretation of literal `$` characters in the complete Store source. The fresh helper uses a callback returning the full literal after body, and full SHA256 equality for source comparisons. This was a helper assembly error, not a native/body acceptance. Its raw negative output and complete helper/proposal remain preserved.
