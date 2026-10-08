# Current Renderer Ten Authority Refresh

Renderer9 current binding advanced first to25e8… then currentobserved full Store `80f7073ba4d296d520cec76f4e0083253de9ea4d47bd9fa3aeba90fc9cfcbe23`. This successor retains complete captured11before/currentafter bodies/hash/fullreplacement recipe. Every original fieldclaim/all three descriptor fullfixturepair/owners/caps/stacks/threshold/lawassertions unchanged. No field size attribution or physical layout/native acceptance follows from current source.

```diff
--- Renderer9-currentStore
+++ Renderer10-currentStore
@@ -1685,20 +1685,7 @@
         if maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if let Some(active) = self.active.as_mut() {
-            return match active.close_step(maximum_items, maximum_bytes)? {
-                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
-                SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "decoded edit nested owner exceeded its exact close grant")),
-                SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
-                SnapshotRetirementStep::Complete => {
-                    if !active.terminal_is_empty() {
-                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "decoded edit nested owner completed without a terminal witness"));
-                    }
-                    drop(self.active.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-            };
-        }
+        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, maximum_items, maximum_bytes); }
         let Some(edit) = self.edit.as_mut() else {
             return Ok(SnapshotRetirementStep::Complete);
         };
@@ -1717,6 +1704,10 @@
 
     fn terminal_is_empty(&self) -> bool {
         self.edit.is_none() && self.active.is_none()
+    }
+
+    fn next_close_byte_demand(&self) -> usize {
+        self.active.as_ref().map_or(usize::from(self.edit.is_some()), artifact_retirement_box_byte_demand)
     }
 }
 
@@ -2566,7 +2557,7 @@
     }
 
     fn close_phase_witness(&self) -> String {
-        format!("{:?}/started={}/active={}", self.phase, self.started, self.active.is_some())
+        format!("{:?}/started={}/active={}/demand={}/terminal={}/cursorBytes={}", self.phase, self.started, self.active.is_some(), self.active.as_ref().map_or(0, |owner| owner.next_close_byte_demand()), self.active.as_ref().is_some_and(|owner| owner.terminal_is_empty()), self.active.as_ref().map_or(0, |owner| std::mem::size_of_val(owner.as_ref())))
     }
 }
 
@@ -17285,6 +17276,7 @@
     fn cancel(&mut self);
     fn begin_close(&mut self);
     fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError>;
+    fn next_close_byte_demand(&self) -> usize { 1 }
     fn terminal_is_empty(&self) -> bool;
 }
 
@@ -17316,7 +17308,7 @@
 /// lane admits: a typed mutation for the app's own document lane, an owned wire item for a member.
 /// The batch machine calls it exactly once per staged mutation and never inspects the input.
 trait ArtifactStoreBatchItemAuthority<P, Mutation>: Send + Sync {
-    type Input: Send;
+    type Input: Send + 'static;
 
     fn preflight(&self, input: &Self::Input, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String>;
     fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, Self::Input>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, ArtifactStoreOneItemPreparationRequest<P, Self::Input>>;
@@ -17330,7 +17322,7 @@
     }
 }
 
-impl<P: Send + Sync, Mutation: Send> ArtifactStoreBatchItemAuthority<P, Mutation> for Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>> {
+impl<P: Send + Sync, Mutation: Send + 'static> ArtifactStoreBatchItemAuthority<P, Mutation> for Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>> {
     type Input = Mutation;
 
     fn preflight(&self, input: &Mutation, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
@@ -17350,7 +17342,7 @@
     }
 }
 
-impl<P: Send + Sync, Mutation: Send> ArtifactStoreBatchItemAuthority<P, Mutation> for Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>> {
+impl<P: Send + Sync, Mutation: Send + 'static> ArtifactStoreBatchItemAuthority<P, Mutation> for Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>> {
     type Input = MemberStoreOneItemWire;
 
     fn preflight(&self, input: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
@@ -17377,45 +17369,43 @@
 /// machine reads; every owner leaves through `begin_next` or one bounded `close_step`.
 trait ArtifactStoreBatchSource<P, Mutation>: Send {
     fn remaining(&self) -> usize;
+    fn census_one(&mut self, lane: HistoryLane) -> Result<Option<ArtifactStoreOneItemFootprint>, String>;
     fn begin_next(&mut self, request: ArtifactStoreBatchItemRequest<P>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, String>;
-    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> SnapshotRetirementStep;
+    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError>;
+    fn next_close_byte_demand(&self) -> usize;
     fn terminal_is_empty(&self) -> bool;
 }
 
-/// 🧺️ The one batch source shape: an app-owned per-item authority plus the gesture's inputs in
-/// reverse authored order, so `pop` hands the next item back in the order the app emitted it.
 struct ArtifactStoreBatchSourceOf<P, Mutation, A: ArtifactStoreBatchItemAuthority<P, Mutation>> {
     authority: Option<A>,
-    inputs: Vec<A::Input>,
+    inputs: std::collections::VecDeque<A::Input>,
+    census: usize,
+    footprint: ArtifactStoreOneItemFootprint,
+    issuer: Option<member_owned_batch::SourceRetirementIssuer<A::Input>>,
+    retirement: Option<Box<dyn member_owned_batch::BatchRetirement>>,
     marker: PhantomData<fn() -> (P, Mutation)>,
 }
 
 impl<P, Mutation, A: ArtifactStoreBatchItemAuthority<P, Mutation>> ArtifactStoreBatchSourceOf<P, Mutation, A> {
-    /// 🧮 Declares the whole gesture in one footprint: work items sum across every staged forward
-    /// and inverse owner, retained bytes stay the widest single item because only one item's
-    /// owners are ever live at once.
-    fn footprint(&self, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
-        let authority = self.authority.as_ref().ok_or_else(|| "batch source lost its exact app-owned item authority".to_string())?;
-        let mut total = ArtifactStoreOneItemFootprint { work_items: 0, retained_bytes: 0 };
-        for input in &self.inputs {
-            let item = authority.preflight(input, lane)?;
-            if !item.is_admissible() {
-                return Err("one-item preparation footprint exceeds its fixed item or byte capacity".into());
-            }
-            total = total.merged(item);
-        }
-        Ok(total)
+    fn new(authority: A, inputs: Vec<A::Input>, issuer: Option<member_owned_batch::SourceRetirementIssuer<A::Input>>) -> Self {
+        Self { authority: Some(authority), inputs: inputs.into(), census: 0, footprint: ArtifactStoreOneItemFootprint { work_items: 0, retained_bytes: 0 }, issuer, retirement: None, marker: PhantomData }
     }
 }
 
 impl<P: Send, Mutation, A: ArtifactStoreBatchItemAuthority<P, Mutation>> ArtifactStoreBatchSource<P, Mutation> for ArtifactStoreBatchSourceOf<P, Mutation, A> {
-    fn remaining(&self) -> usize {
-        self.inputs.len()
-    }
-
+    fn remaining(&self) -> usize { self.inputs.len() }
+    fn census_one(&mut self, lane: HistoryLane) -> Result<Option<ArtifactStoreOneItemFootprint>, String> {
+        let Some(input) = self.inputs.get(self.census) else { return Ok(Some(self.footprint)); };
+        let item = self.authority.as_ref().ok_or_else(|| "batch source lost its exact app-owned item authority".to_string())?.preflight(input, lane)?;
+        let footprint = self.footprint.merged(item);
+        if !item.is_admissible() || !footprint.is_admissible() { return Err("one-item preparation footprint exceeds its fixed item or byte capacity".into()); }
+        self.footprint = footprint;
+        self.census += 1;
+        Ok(None)
+    }
     fn begin_next(&mut self, request: ArtifactStoreBatchItemRequest<P>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, String> {
         let authority = self.authority.as_ref().ok_or_else(|| "batch source lost its exact app-owned item authority".to_string())?;
-        let mutation = self.inputs.pop().ok_or_else(|| "batch source was advanced past its last admitted item".to_string())?;
+        let mutation = self.inputs.pop_front().ok_or_else(|| "batch source was advanced past its last admitted item".to_string())?;
         let ArtifactStoreBatchItemRequest { operation, generation, base_revision, lane, authority: live, base } = request;
         let item = ArtifactStoreOneItemPreparationRequest { operation, generation, base_revision, lane, authority: live, base, mutation };
         match authority.begin(item) {
@@ -17423,28 +17413,49 @@
             Err(item) => {
                 let (base, mutation) = item.into_owners();
                 let _ = base.return_to_registry();
-                self.inputs.push(mutation);
+                self.inputs.push_front(mutation);
                 Err("app-owned one-item preparation factory rejected its exact owner bundle".into())
             }
         }
     }
-
-    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> SnapshotRetirementStep {
-        if grant.maximum_items == 0 {
-            return SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
-        }
-        if self.inputs.pop().is_some() {
-            return SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 };
-        }
-        if self.authority.take().is_some() {
-            return SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 };
-        }
-        SnapshotRetirementStep::Complete
-    }
-
-    fn terminal_is_empty(&self) -> bool {
-        self.inputs.is_empty() && self.authority.is_none()
-    }
+    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
+        if grant.maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+        if let Some(retirement) = self.retirement.as_mut() {
+            if !retirement.terminal_is_empty() {
+                let step = retirement.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: grant.maximum_bytes, maximum_release_bytes: grant.maximum_bytes, maximum_depth: self.issuer.as_ref().expect("original controlled source issuer").maximum_depth })?;
+                let progress = step.progress();
+                return Ok(SnapshotRetirementStep::Pending { released_items: progress.copied_items, released_bytes: progress.released_bytes });
+            }
+            let bytes = std::mem::size_of_val(retirement.as_ref());
+            if bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+            self.retirement = None;
+            self.issuer = None;
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
+        }
+        if let Some(issuer) = self.issuer.as_ref() {
+            if issuer.birth_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+            self.retirement = Some((issuer.begin)(std::mem::take(&mut self.inputs)));
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
+        }
+        if self.inputs.pop_front().is_some() { return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
+        let backing = self.inputs.capacity().checked_mul(std::mem::size_of::<A::Input>()).expect("batch source backing layout");
+        if backing != 0 {
+            if backing > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+            self.inputs = std::collections::VecDeque::new();
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: backing });
+        }
+        if self.authority.take().is_some() { return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
+        Ok(SnapshotRetirementStep::Complete)
+    }
+    fn next_close_byte_demand(&self) -> usize {
+        if let Some(retirement) = self.retirement.as_ref() {
+            return if retirement.terminal_is_empty() { std::mem::size_of_val(retirement.as_ref()) } else { retirement.demands().map_or(usize::MAX, |(capacity, release)| capacity.max(release)) };
+        }
+        if let Some(issuer) = self.issuer.as_ref() { return issuer.birth_bytes; }
+        if !self.inputs.is_empty() { return 0; }
+        self.inputs.capacity().checked_mul(std::mem::size_of::<A::Input>()).expect("batch source backing layout")
+    }
+    fn terminal_is_empty(&self) -> bool { self.inputs.capacity() == 0 && self.inputs.is_empty() && self.authority.is_none() && self.retirement.is_none() && self.issuer.is_none() }
 }
 //#endregion 🧺️BatchSource
 
@@ -17482,6 +17493,7 @@
     fn phase(&self) -> ArtifactStoreOneItemPublicationPhase;
     fn progress(&self) -> ArtifactStoreOneItemCheckpoint;
     fn next_group_byte_demand(&self) -> usize;
+    fn next_close_byte_demand(&self) -> usize { 1 }
     fn fault(&self) -> Option<&str>;
     fn retry(&mut self) -> bool;
     fn acknowledge(&mut self) -> bool;
@@ -17511,6 +17523,10 @@
     }
 
     fn next_group_byte_demand(&self) -> usize { self.publication.next_group_byte_demand() }
+    fn next_close_byte_demand(&self) -> usize {
+        if self.group_history.is_some() || self.group_displaced.is_some() { return 0; }
+        self.publication.next_close_byte_demand()
+    }
 
     fn fault(&self) -> Option<&str> {
         self.publication.fault()
@@ -17703,6 +17719,8 @@
     expected_revision: [u8; 32],
     lane: HistoryLane,
     footprint: ArtifactStoreOneItemFootprint,
+    source_census_completed: u32,
+    source_census_finished: bool,
     admitted_items: usize,
     authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
     authority_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
@@ -17731,6 +17749,18 @@
     /// 🎟️ Current private group copy or allocation demand, never accumulated credit.
     pub fn next_group_byte_demand(&self) -> usize { self.group_preparation.as_ref().map_or(1, |group| group.next_byte_demand()) }
 
+    /// 📏️ Names the current complete close allocation before any owner is transferred.
+    pub fn next_close_byte_demand(&self) -> usize {
+        if let Some(owner) = self.preparation.as_ref() { return if owner.terminal_is_empty() { std::mem::size_of_val(owner.as_ref()) } else { owner.next_close_byte_demand() }; }
+        if let Some(source) = self.source.as_ref() { return if source.terminal_is_empty() { std::mem::size_of_val(source.as_ref()) } else { source.next_close_byte_demand() }; }
+        if let Some(stage) = self.stage.as_ref() { return if stage.terminal_is_empty() { std::mem::size_of::<ArtifactStoreBatchStage<P, Mutation>>() } else { stage.retiring.as_ref().map_or(1, |owner| artifact_retirement_box_byte_demand(owner)) }; }
+        if let Some(verb) = self.verb.as_ref() { return verb.capacity(); }
+        if self.receipt.is_some() { return 0; }
+        if let Some(owner) = self.authority_retirement.as_ref() { return artifact_retirement_box_byte_demand(owner); }
+        if self.authority.is_some() { return std::mem::size_of::<canonical_edit::ArtifactStoreOneItemAuthorityRetirement>(); }
+        self.fault.as_ref().map_or(0, String::capacity)
+    }
+
     pub fn operation(&self) -> semio_framework_job::OperationId {
         self.operation
     }
@@ -17795,7 +17825,7 @@
         let item = if self.item_closing { ArtifactStoreOneItemCheckpoint::default() } else { self.preparation.as_ref().map_or_else(ArtifactStoreOneItemCheckpoint::default, |owner| owner.checkpoint()) };
         ArtifactStoreOneItemCheckpoint {
             cursor: staged.cursor,
-            completed_items: staged.completed_items.saturating_add(item.completed_items).saturating_add(self.group_preparation.as_ref().map_or(0, |group| group.completed_items())),
+            completed_items: staged.completed_items.saturating_add(item.completed_items).saturating_add(self.source_census_completed).saturating_add(self.group_preparation.as_ref().map_or(0, |group| group.completed_items())),
             completed_bytes: staged.completed_bytes.saturating_add(item.completed_bytes),
             digest: if item.digest == [0; 32] { staged.digest } else { item.digest },
         }
@@ -17844,6 +17874,7 @@
     {
         self.begin_close();
         if self.group_preparation.is_some() { return Ok(SnapshotRetirementStep::Blocked); }
+        if grant.maximum_items == 0 || grant.maximum_bytes < self.next_close_byte_demand() { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
         if let Some(owner) = self.preparation.as_mut() {
             let step = owner.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
             if step != SnapshotRetirementStep::Complete {
@@ -17852,20 +17883,24 @@
             if !owner.terminal_is_empty() {
                 return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "one-item preparation reported complete without terminal emptiness"));
             }
+            let released_bytes = std::mem::size_of_val(owner.as_ref());
+            if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
             self.preparation = None;
             self.item_closing = false;
-            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
         }
         if let Some(source) = self.source.as_mut() {
-            let step = source.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes });
+            let step = source.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
             if step != SnapshotRetirementStep::Complete {
                 return Ok(step);
             }
             if !source.terminal_is_empty() {
                 return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "batch source reported complete without terminal emptiness"));
             }
+            let released_bytes = std::mem::size_of_val(source.as_ref());
+            if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
             self.source = None;
-            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
         }
         if let Some(stage) = self.stage.as_mut() {
             let step = stage.close_step(grant)?;
@@ -17875,14 +17910,18 @@
             if !stage.terminal_is_empty() {
                 return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "staged batch edit reported complete without terminal emptiness"));
             }
+            let released_bytes = std::mem::size_of_val(stage.as_ref());
+            if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
             self.stage = None;
-            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
         }
         if grant.maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if self.verb.take().is_some() {
-            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
+        if let Some(verb) = self.verb.take() {
+            let released_bytes = verb.capacity();
+            drop(verb);
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
         }
         if self.receipt.take().is_some() {
             return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
@@ -17895,22 +17934,20 @@
             if !owner.terminal_is_empty() {
                 return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "batch authority reported complete without terminal emptiness"));
             }
+            let released_bytes = std::mem::size_of_val(owner.as_ref());
+            if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
             self.authority_retirement = None;
-            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
         }
         if let Some(authority) = self.authority.take() {
             self.authority_retirement = Some(authority.retire());
             return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
         }
-        if let Some(fault) = self.fault.as_mut().filter(|fault| !fault.is_empty()) {
-            let scalar = fault.chars().next_back().expect("nonempty batch fault");
-            if scalar.len_utf8() > grant.maximum_bytes {
-                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
-            }
-            fault.pop();
-            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: scalar.len_utf8() });
-        }
-        self.fault = None;
+        if let Some(fault) = self.fault.take() {
+            let released_bytes = fault.capacity();
+            drop(fault);
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
+        }
         self.phase = ArtifactStoreOneItemPublicationPhase::Complete;
         Ok(SnapshotRetirementStep::Complete)
     }
@@ -20321,12 +20358,9 @@
         let Some(factory) = factory else {
             return Err(ArtifactStoreBatchAdmissionRejected { reason: "batched publication requires an explicit app-owned ArtifactStoreOneItemPreparationFactory".into(), mutations });
         };
-        let mut inputs = mutations;
-        inputs.reverse();
-        let source = ArtifactStoreBatchSourceOf { authority: Some(Arc::clone(factory)), inputs, marker: PhantomData };
+        let source = ArtifactStoreBatchSourceOf::new(Arc::clone(factory), mutations, None);
         self.begin_apply_batch_owned(operation, expected_generation, expected_revision, actor, lane, None, source, outbound, transaction).map_err(|rejected| {
-            let (reason, mut mutations) = rejected.into_owners();
-            mutations.reverse();
+            let (reason, mutations) = rejected.into_owners();
             ArtifactStoreBatchAdmissionRejected { reason, mutations }
         })
     }
@@ -20392,7 +20426,7 @@
         Mutation: 'static,
         P: 'static,
     {
-        let reject = |reason: String, source: ArtifactStoreBatchSourceOf<P, Mutation, A>| ArtifactStoreBatchAdmissionRejected { reason, mutations: source.inputs };
+        let reject = |reason: String, source: ArtifactStoreBatchSourceOf<P, Mutation, A>| ArtifactStoreBatchAdmissionRejected { reason, mutations: source.inputs.into() };
         if source.inputs.is_empty() {
             return Err(reject("batched publication requires at least one admitted mutation owner".into(), source));
         }
@@ -20411,11 +20445,6 @@
         if self.backbone.is_some() && !outbound {
             return Err(reject("batched publication has no retained outbound backbone encoder/sender authority".into(), source));
         }
-        let footprint = match source.footprint(lane) {
-            Ok(footprint) if footprint.is_admissible() => footprint,
-            Ok(_) => return Err(reject("batched preparation footprint exceeds its fixed item or byte capacity".into(), source)),
-            Err(reason) => return Err(reject(reason, source)),
-        };
         if actor.is_empty() || actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
             return Err(reject("batched publication actor exceeds its fixed identity capacity".into(), source));
         }
@@ -20459,15 +20488,14 @@
             group_id,
             stamped_edit_id,
         });
-        if admitted_items > footprint.work_items {
-            return Err(reject("batched publication item census disagrees with its declared fixed work envelope".into(), source));
-        }
         Ok(ArtifactStoreBatchPublication {
             operation,
             expected_generation,
             expected_revision,
             lane,
-            footprint,
+            footprint: ArtifactStoreOneItemFootprint { work_items: 0, retained_bytes: 0 },
+            source_census_completed: 0,
+            source_census_finished: false,
             admitted_items,
             authority: Some(authority),
             authority_retirement: None,
@@ -20549,6 +20577,13 @@
         let item_grant = ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes };
         match publication.phase {
             ArtifactStoreOneItemPublicationPhase::Preparing => {
+                if !publication.source_census_finished {
+                    match publication.source.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its retained census source".into()))?.census_one(publication.lane).map_err(VcsError::ValidationFailed)? {
+                        Some(footprint) => { publication.footprint = footprint; publication.source_census_finished = true; },
+                        None => { publication.source_census_completed = publication.source_census_completed.saturating_add(1); },
+                    }
+                    return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
+                }
                 if publication.item_closing {
                     let owner = publication.preparation.as_mut().ok_or_else(|| VcsError::ValidationFailed("staged batch item lost its retained preparation owner before retirement".into()))?;
                     let step = Self::spend_item_close_grant(owner.as_mut(), item_grant).map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
@@ -25453,7 +25488,7 @@
             return Ok(false);
         };
         if !settlement.is_finished() {
-            settlement.step(&mut self.edit_messages, &mut self.outcomes, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES).map_err(VcsError::InverseRefused)?;
+            settlement.step(&mut self.edit_messages, &mut self.outcomes, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES).map_err(VcsError::InverseRefused)?;
             return Ok(false);
         }
         let rejects = match self.mode {
@@ -26728,7 +26763,7 @@
         if request.wire.schema.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.wire.bytes.len() > ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
             return Err(ArtifactStoreBatchAdmissionRejected { reason: "member wire exceeds its fixed schema or byte admission".into(), mutations: vec![request.wire] });
         }
-        let source = ArtifactStoreBatchSourceOf { authority: Some(Arc::clone(factory)), inputs: vec![request.wire], marker: PhantomData };
+        let source = ArtifactStoreBatchSourceOf::new(Arc::clone(factory), vec![request.wire], None);
         let publication = self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor, HistoryLane::Document, request.group_id, source, false, None)?;
         Ok(Box::new(MemberStoreOneItemPublication { member: Some(Arc::clone(&self.snapshot_read_leases)), publication, group_history: None, group_displaced: None }))
     }
@@ -26740,14 +26775,13 @@
         if request.actor.is_empty() || request.actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
             return Err("member batch metadata exceeds its fixed identity capacity".into());
         }
-        let mut inputs = request.mutations.take_mutations::<Mutation>().ok_or_else(|| "member batch source is already transferred or closing".to_string())?;
-        inputs.reverse();
-        let source = ArtifactStoreBatchSourceOf { authority: Some(Arc::clone(factory)), inputs, marker: PhantomData };
+        let issuer = request.mutations.source_retirement_issuer::<Mutation>().ok_or_else(|| "member batch lost its exact admitted retirement issuer".to_string())?;
+        let inputs = request.mutations.take_mutations::<Mutation>().ok_or_else(|| "member batch source is already transferred or closing".to_string())?;
+        let source = ArtifactStoreBatchSourceOf::new(Arc::clone(factory), inputs, Some(issuer));
         match self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor.clone(), HistoryLane::Document, request.group_id.clone(), source, false, request.transaction.clone()) {
             Ok(publication) => Ok(Box::new(MemberStoreOneItemPublication { member: Some(Arc::clone(&self.snapshot_read_leases)), publication, group_history: None, group_displaced: None })),
             Err(rejected) => {
-                let (reason, mut inputs) = rejected.into_owners();
-                inputs.reverse();
+                let (reason, inputs) = rejected.into_owners();
                 request.mutations.restore_mutations(inputs);
                 Err(reason)
             }
```

Exact GUI257001 sourcecontrols and planned receipt 🗑️generated/current-engine-layout-10/green-1/admission.json. All prior negatives retained, no Rootwrite/compiler.
