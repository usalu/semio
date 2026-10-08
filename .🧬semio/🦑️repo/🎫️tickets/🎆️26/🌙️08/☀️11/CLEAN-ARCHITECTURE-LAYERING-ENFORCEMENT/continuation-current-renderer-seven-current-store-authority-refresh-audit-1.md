# Current Renderer7 Store Authority Refresh

Source15 actual capture refused the Renderer6 bound Store before admission and produced no sealed plans. That negative and all Renderer6 controls remain intact. This successor conserves the exact three-descriptor fixture fullpair, all other current bindings and all45+5 source laws; it replaces complete current Store after body/recipe/hash with the actual current full source. Store physical contribution and current native sizes remain unaccepted. No guard is relaxed, no prior native acceptance reused.

Current Store hash `0b3370095f265e6a4345a0b8d11c50e7e5a6aa0eac66b62a11ac0fa8d26cee03`.

```diff
--- captured11Store
+++ currentStore
@@ -348,6 +348,8 @@
     }
 
     fn terminal_is_empty(&self) -> bool { self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() }
+    fn next_close_byte_demand(&self) -> usize { self.active.as_ref().map_or(1, |active| active.next_close_byte_demand()) }
+
 }
 
 impl<P, Mu: self::Mutation<P>> Drop for ArtifactHistoryReadRetirement<P, Mu> {
@@ -1533,6 +1535,29 @@
     Complete,
 }
 
+/// 📏️ Exact next physical release owned by an erased retirement, including its terminal cursor allocation.
+fn artifact_retirement_box_byte_demand(owner: &Box<dyn ErasedSnapshotRetirement>) -> usize {
+    if owner.terminal_is_empty() { std::mem::size_of_val(owner.as_ref()) } else { owner.next_close_byte_demand() }
+}
+
+/// 📦️ Retires a nested payload and then its cursor box under separate, unchanged caller grants.
+fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
+    let Some(owner) = slot.as_mut() else { return Ok(SnapshotRetirementStep::Complete); };
+    if owner.terminal_is_empty() {
+        let extent = std::mem::size_of_val(owner.as_ref());
+        if maximum_items == 0 || maximum_bytes < extent { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+        drop(slot.take());
+        return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: extent });
+    }
+    match owner.close_step(maximum_items, maximum_bytes)? {
+        SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
+        SnapshotRetirementStep::Pending { .. } => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "nested retirement exceeded its exact caller grant")),
+        SnapshotRetirementStep::Complete if owner.terminal_is_empty() => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
+        SnapshotRetirementStep::Complete => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "nested retirement completed with a live payload")),
+        SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
+    }
+}
+
 struct ArtifactStoreVcsRetirement<P, Mutation> {
     vcs: std::mem::ManuallyDrop<Option<ArtifactVcs<P, Mutation>>>,
     active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
@@ -1722,20 +1747,7 @@
         if maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if let Some(active) = self.active.as_mut() {
-            return match active.close_step(maximum_items, maximum_bytes)? {
-                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
-                SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS retirement nested owner exceeded its exact close grant")),
-                SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
-                SnapshotRetirementStep::Complete => {
-                    if !active.terminal_is_empty() {
-                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS retirement nested owner reported Complete without its terminal witness"));
-                    }
-                    drop(self.active.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-            };
-        }
+        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, maximum_items, maximum_bytes); }
         let Some(vcs) = self.vcs.as_mut() else {
             self.phase = ArtifactStoreVcsRetirementPhase::Complete;
             return Ok(SnapshotRetirementStep::Complete);
@@ -1806,6 +1818,10 @@
 
     fn terminal_is_empty(&self) -> bool {
         self.vcs.is_none() && self.active.is_none()
+    }
+
+    fn next_close_byte_demand(&self) -> usize {
+        self.active.as_ref().map_or(usize::from(self.vcs.is_some()), artifact_retirement_box_byte_demand)
     }
 }
 
@@ -9646,6 +9662,10 @@
     }
 
     fn record_release_fault(&mut self, diagnostic: OwnedSchemaDecodeDiagnostic) {
+        #[cfg(debug_assertions)] {
+            static FAULTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
+            if FAULTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 8 { eprintln!("[DEBUG] envelope release fault: code={} admitted-demand={} retained-limit={} released={}", diagnostic.code, self.maximum_field_close_byte_demand, self.maximum_retained_field_close_bytes, self.released_field_bytes); }
+        }
         if !matches!(self.state, ArtifactEnvelopeDecodeState::ReleaseFault(_)) {
             self.state = ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic);
         }
@@ -9678,6 +9698,11 @@
     /// live for `close_step` below, so the two must not overlap.
     fn release_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Option<semio_framework_job::StepOutcome> {
         cx.set_stage("artifact-envelope-decode-close");
+        #[cfg(debug_assertions)] {
+            static TURNS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
+            let turn = TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
+            if turn <= 8 || turn.is_power_of_two() { eprintln!("[DEBUG] envelope release phase turn={turn} state={:?} fields={} returned={} reclaimed={} record={} ledger={}", self.state, self.fields.is_some(), self.field_returned, self.field_registry.ticket_reclaimed(self.field_ticket), self.record.is_some(), self.released_field_bytes); }
+        }
         if self.fields.is_some() {
             let demand = self.fields.as_mut().expect("release step holds its field decoder lease").with_owner(|owner| if owner.terminal_is_empty() { Ok(0) } else { owner.next_close_byte_demand() });
             let demanded = match demand {
@@ -9714,6 +9739,11 @@
                 }
             };
             cx.consume_fuel(1);
+            #[cfg(debug_assertions)] {
+                static CLOSES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
+                let turn = CLOSES.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
+                if turn <= 8 || turn.is_power_of_two() { eprintln!("[DEBUG] envelope field release turn={turn} actual-demand={demanded} grant={maximum_bytes} step={step:?}"); }
+            }
             match step {
                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => {
                     if let Err(diagnostic) = self.record_released_field_bytes(released_bytes) {
@@ -10863,17 +10893,12 @@
             && self.retirement.is_none()
     }
 
+    fn close_retained_retirement(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
+        artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("artifact-envelope.fresh-vcs-retirement-fault"))
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
@@ -10891,16 +10916,7 @@
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
@@ -11011,14 +11027,17 @@
                 return Ok(demand);
             }
         }
-        if self.retirement.is_some() || self.active.as_ref().is_some_and(|active| !matches!(active, ArtifactEnvelopeFreshVcsActive::Snapshot { .. })) {
-            return Ok(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
-        }
+        if let Some(retirement) = self.retirement.as_ref() {
+            let demand = artifact_retirement_box_byte_demand(retirement);
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
@@ -11028,6 +11047,11 @@
     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
         if maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
+        }
+        #[cfg(debug_assertions)] {
+            static TURNS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
+            let turn = TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
+            if turn <= 8 || turn.is_power_of_two() { eprintln!("[DEBUG] Fresh VCS close phase turn={turn} grant={maximum_bytes} snapshot={} snapshot-terminal={} retirement-demand={:?} genesis={} value={} active={} ledgers={}/{}/{}/{}", self.snapshot.is_some(), self.snapshot.as_ref().is_none_or(|owner| owner.terminal_is_empty()), self.retirement.as_ref().map(artifact_retirement_box_byte_demand), self.genesis_pack.is_some(), self.value.is_some(), self.active.is_some(), self.edits.is_some(), self.changes.is_some(), self.checkpoints.is_some(), self.alternatives.is_some()); }
         }
         self.pending = None;
         if let Some(snapshot) = self.snapshot.as_mut() {
@@ -11389,9 +11413,12 @@
                 ArtifactEnvelopeFreshRecordActive::String { .. } | ArtifactEnvelopeFreshRecordActive::Empty { .. } => Ok(0),
             };
         }
-        if self.pending_completed.is_some() || self.active_retirement.is_some() {
-            return Ok(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
-        }
+        if let Some(retirement) = self.active_retirement.as_ref() {
+            let demand = artifact_retirement_box_byte_demand(retirement);
+            if demand > self.maximum_close_byte_demand() { return Err(Self::diagnostic("vcs", "artifact-envelope.target-close-demand-over-admitted-maximum")); }
+            return Ok(demand);
+        }
+        if self.pending_completed.is_some() { return Ok(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES); }
         if self.target.vcs.is_some() {
             return Ok(0);
         }
@@ -11454,17 +11481,7 @@
             }
             return Ok(step);
         }
-        if let Some(retirement) = self.active_retirement.as_mut() {
-            let step = retirement.close_step(maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("vcs", "artifact-envelope.target-retirement"))?;
-            if matches!(step, SnapshotRetirementStep::Complete) {
-                if !retirement.terminal_is_empty() {
-                    return Err(Self::diagnostic("vcs", "artifact-envelope.target-retirement-false-terminal"));
-                }
-                drop(self.active_retirement.take());
-                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
-            }
-            return Ok(step);
-        }
+        if self.active_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.active_retirement, maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("vcs", "artifact-envelope.target-retirement")); }
         if !self.close_target_vcs()? {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
@@ -16158,6 +16175,11 @@
 
     pub fn share_current(&self) -> Arc<P> { Arc::clone(self.current.as_ref().expect("initialization retains its current projection")) }
 
+    /// 📏️ Publish the next retained allocation, including its terminal erased cursor box.
+    pub fn next_close_byte_demand(&self) -> usize {
+        self.close_active.as_ref().map_or(usize::from(!self.taken), |active| if active.terminal_is_empty() { std::mem::size_of_val(active.as_ref()) } else { active.next_close_byte_demand() })
+    }
+
     fn replace_current_alias(&mut self, current: Arc<P>) -> Arc<P> { self.current.replace(current).expect("initialization retains its current projection") }
 
     pub fn adopt_current_owned(&mut self, current: P, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Result<(), P>
@@ -16173,8 +16195,14 @@
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
@@ -16349,17 +16377,8 @@
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
@@ -20589,6 +20608,20 @@
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
@@ -20806,6 +20839,7 @@
         }
         if publication.stage.is_none() {
             let mut staged = Box::new(Self::empty_batch_stage(&authority));
+            staged.edit.inverse = semio_framework_value::list::PagedList::with_payload_page_bytes(grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
             staged.mutation_retirement = (*self.mutation_retirement_factory).clone();
             staged.snapshot_retirement = (*self.snapshot_retirement_factory).clone();
             if staged.mutation_retirement.is_none() || staged.snapshot_retirement.is_none() {
@@ -20817,11 +20851,15 @@
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
@@ -22245,11 +22283,7 @@
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
@@ -25035,6 +25069,7 @@
     from: usize,
     prefix_until: usize,
     prefix_work_progress: bool,
+    operation_progress: bool,
     cursor: usize,
     operation: usize,
     schema: String,
@@ -25099,6 +25134,7 @@
             from,
             prefix_until: from,
             prefix_work_progress: false,
+            operation_progress: false,
             cursor: from,
             operation: 0,
             schema: schema.to_string(),
@@ -25224,13 +25260,28 @@
 
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
@@ -25260,8 +25311,9 @@
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
@@ -25276,8 +25328,9 @@
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
@@ -25289,21 +25342,23 @@
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
