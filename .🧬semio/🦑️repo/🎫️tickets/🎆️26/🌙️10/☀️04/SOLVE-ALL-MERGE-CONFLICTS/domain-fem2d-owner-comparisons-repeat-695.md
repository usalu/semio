# FEM2D Session Whole Owner Comparisons

## implMountedState::step

### B to S

```diff
--- B

+++ S

@@ -1,19 +1,19 @@

-    fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep {
+    fn step(&mut self, cx: &mut StepContext<'_>) -> Result<JobStep, semio_framework_job::OriginalTurnFailure> {
         if self.cancel.is_cancelled_now() {
             self.stage = MountedStage::Closing;
-            return self.fail(b"fem2d.session-cancelled".to_vec());
+            return Ok(self.fail(b"fem2d.session-cancelled".to_vec()));
         }
-        if budget.fuel == 0 || budget.deadline_ms == 0 {
-            return JobStep::Running(None);
+        if cx.should_yield() {
+            return Ok(JobStep::Running(None));
         }
         (|| {
             if cx.should_yield() {
-                return JobStep::Running(None);
+                return Ok(JobStep::Running(None));
             }
             if let Some(step) = self.drive_visual_one(cx) {
-                return step;
+                return Ok(step);
             }
-            match self.stage {
+            Ok(match self.stage {
                 MountedStage::PrepareGraph => {
                     const PLANS: [FemStagePlan; 8] = [
                         FemStagePlan { stage: FemJobStage::ValidateReferences, units: 1 },
@@ -27,7 +27,7 @@

                     ];
                     if self.graph_plans.capacity() == 0 {
                         if self.graph_plans.try_reserve_exact(PLANS.len()).is_err() || self.graph_plans.capacity() * size_of::<FemStagePlan>() > SESSION_OWNER_PAGE_BYTES {
-                            return self.fail(b"fem2d.graph-plan-allocation".to_vec());
+                            return Ok(self.fail(b"fem2d.graph-plan-allocation".to_vec()));
                         }
                     } else if let Some(plan) = PLANS.get(self.graph_plans.len()).cloned() {
                         self.graph_plans.push(plan);
@@ -38,7 +38,7 @@

                     cx.consume_fuel(1);
                     self.progress(b"fem2d.graph-admitted")
                 }
-                MountedStage::Graph => match child_end(self.graph.as_mut().expect("graph stage owns graph").step(cx)) {
+                MountedStage::Graph => match child_end(self.graph.as_mut().expect("graph stage owns graph").step(cx))? {
                     ChildEnd::Complete(output) => {
                         self.stage = MountedStage::PrepareDomain;
                         match output {
@@ -61,9 +61,9 @@

                     }
                     Ok(true) => {
                         let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
-                        if let (Some(region), Some(domain)) = (snapshot.regions.first(), self.domain.take()) {
+                        if let (Some(region), Some(domain)) = (snapshot.regions.get(self.region_cursor), self.domain.take()) {
                             if self.visual.region_quality.insert(region.id.clone(), RegionVisualQuality::Unmeshed).is_err() {
-                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
+                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                             }
                             self.visual.state = FemVisualState::Unmeshed;
                             self.visual_dirty = true;
@@ -77,16 +77,22 @@

                     }
                     Err(detail) => self.fail(detail.to_vec()),
                 },
-                MountedStage::Mesh => match child_end(self.mesh.as_mut().expect("mesh stage owns mesh").step(cx)) {
+                MountedStage::Mesh => match child_end(self.mesh.as_mut().expect("mesh stage owns mesh").step(cx))? {
                     ChildEnd::Complete(output) => {
-                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
+                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.get(self.region_cursor)) {
                             if !self.visual.region_quality.update(&region.id, RegionVisualQuality::Final) {
-                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
+                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                             }
                             self.visual.state = FemVisualState::Refined;
                             self.visual_dirty = true;
                         }
-                        self.model_build = Some(MountedModelBuild::new(self.mesh.as_mut().and_then(MeshJob::take_completed_mesh)));
+                        let completed_mesh = self.mesh.as_mut().and_then(MeshJob::take_completed_mesh);
+                        if let Some(builder) = self.model_build.as_mut() {
+                            let Some(mesh) = completed_mesh else { return Ok(self.fail(b"fem2d.mesh-terminal-owner".to_vec())); };
+                            builder.attach_region(self.region_cursor, mesh);
+                        } else {
+                            self.model_build = Some(MountedModelBuild::new(completed_mesh));
+                        }
                         self.stage = MountedStage::BuildModel;
                         match output {
                             Some(bytes) => JobStep::Running(Some(bytes)),
@@ -94,7 +100,7 @@

                         }
                     }
                     ChildEnd::Preview { bytes: payload, quality } => {
-                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
+                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.get(self.region_cursor)) {
                             let quality = match quality {
                                 Some(0) => RegionVisualQuality::Coarse,
                                 Some(1) => RegionVisualQuality::Refined,
@@ -102,7 +108,7 @@

                                 _ => RegionVisualQuality::Unmeshed,
                             };
                             if !self.visual.region_quality.update(&region.id, quality) {
-                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
+                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                             }
                             self.visual.state = match quality {
                                 RegionVisualQuality::Unmeshed => FemVisualState::Unmeshed,
@@ -129,7 +135,7 @@

                     if self.model_build.is_none() {
                         self.model_build = Some(MountedModelBuild::new(None));
                         cx.consume_fuel(1);
-                        return self.progress(b"fem2d.model-admitted");
+                        return Ok(self.progress(b"fem2d.model-admitted"));
                     }
                     match self.model_build.as_mut().expect("model builder admitted above").step_one(snapshot) {
                         Ok(false) => {
@@ -137,8 +143,14 @@

                             self.progress(b"fem2d.model-building")
                         }
                         Ok(true) => {
-                            let model = std::sync::Arc::new(self.model_build.as_mut().and_then(MountedModelBuild::take_complete).expect("complete model transfers exactly once"));
-                            self.assembly_build = Some(AssemblyJobConstruction::new_owned(model, self.identity.operation(), 1));
+                            if self.region_cursor + 1 < snapshot.regions.len() {
+                                self.model_build.as_mut().expect("completed region retains builder").close_lane = 7;
+                                self.stage = MountedStage::RetireRegion;
+                                cx.consume_fuel(1);
+                                return Ok(self.progress(b"fem2d.region-complete"));
+                            }
+                            let model = self.model_build.as_mut().and_then(MountedModelBuild::take_complete).expect("complete model transfers exactly once");
+                            self.assembly_build = Some(AssemblyJobConstruction::new_mounted(model, self.identity.operation(), 1));
                             self.stage = MountedStage::PrepareAssembly;
                             self.visual.state = FemVisualState::Assembling;
                             cx.consume_fuel(1);
@@ -147,6 +159,26 @@

                         Err(error) => self.fail(error),
                     }
                 }
+                MountedStage::RetireRegion => {
+                    cx.consume_fuel(1);
+                    let builder = self.model_build.as_mut().expect("region retirement retains model");
+                    if !builder.close_step(SESSION_OWNER_PAGE_BYTES).0 { return Ok(self.progress(b"fem2d.region-mesh-close")); }
+                    if let Some(mesh) = self.mesh.as_mut() {
+                        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_OWNER_PAGE_BYTES, maximum_capacity_bytes: SESSION_OWNER_PAGE_BYTES, maximum_release_bytes: SESSION_OWNER_PAGE_BYTES, maximum_depth: SESSION_CLOSE_DEPTH };
+                        match mesh.close_step(grant) {
+                            InteractiveJobCloseStep::Complete { .. } => {}
+                            InteractiveJobCloseStep::Pending { .. } => return Ok(self.progress(b"fem2d.region-job-close")),
+                            _ => return Ok(self.fail(b"fem2d.region-job-close-refused".to_vec())),
+                        }
+                    }
+                    self.mesh = None;
+                    self.region_cursor += 1;
+                    self.domain_outer_cursor = 0;
+                    self.domain_hole_cursor = 0;
+                    self.domain_hole_point_cursor = 0;
+                    self.stage = MountedStage::PrepareDomain;
+                    self.progress(b"fem2d.region-next")
+                }
                 MountedStage::PrepareAssembly => match self.assembly_build.as_mut().expect("assembly construction retained").step_one() {
                     Ok(false) => {
                         cx.consume_fuel(1);
@@ -155,7 +187,7 @@

                     Ok(true) => {
                         self.assembly = self.assembly_build.as_mut().and_then(AssemblyJobConstruction::take_complete);
                         if self.assembly.is_none() {
-                            return self.fail(b"fem2d.assembly-construction-false-terminal".to_vec());
+                            return Ok(self.fail(b"fem2d.assembly-construction-false-terminal".to_vec()));
                         }
                         self.stage = MountedStage::Assembly;
                         cx.consume_fuel(1);
@@ -163,18 +195,10 @@

                     }
                     Err(error) => self.fail(error.to_string().into_bytes()),
                 },
-                MountedStage::Assembly => match child_end(self.assembly.as_mut().expect("assembly stage owns assembly").step(cx)) {
+                MountedStage::Assembly => match child_end(self.assembly.as_mut().expect("assembly stage owns assembly").step(cx))? {
                     ChildEnd::Complete(output) => {
-                        let assembly = self.assembly.take().expect("assembly owner retained");
-                        let csr_build = match AssemblyCsrBuild::new(assembly) {
-                            Ok(builder) => builder,
-                            Err(assembly) => {
-                                self.assembly = Some(assembly);
-                                return self.fail(b"fem2d.assembly-false-terminal".to_vec());
-                            }
-                        };
-                        self.csr_build = Some(csr_build);
-                        self.stage = MountedStage::BuildCsr;
+                        self.load_build = Some(MountedLoadBuild::new(self.source));
+                        self.stage = MountedStage::BuildLoads;
                         match output {
                             Some(bytes) => JobStep::Running(Some(bytes)),
                             None => self.progress(b"fem2d.assembly-complete"),
@@ -188,14 +212,38 @@

                     ChildEnd::Cancelled => self.fail(b"fem2d.assembly-cancelled".to_vec()),
                     ChildEnd::Fault(detail) => self.fail(detail.unwrap_or_else(|| b"fem2d.assembly-fault-capacity".to_vec())),
                 },
-                MountedStage::BuildCsr => match self.csr_build.as_mut().expect("CSR builder retained").step_one() {
+                MountedStage::BuildLoads => {
+                    cx.consume_fuel(1);
+                    match self.load_build.as_mut().expect("load owner retained").step_one(self.snapshot.as_ref().expect("snapshot retained"), self.assembly.as_ref().expect("assembly retained")) {
+                        Ok(false) => self.progress(b"fem2d.load-building"),
+                        Ok(true) => {
+                            let assembly = self.assembly.take().expect("assembly owner retained");
+                            self.csr_build = match AssemblyCsrBuild::new_free(assembly) {
+                                Ok(builder) => Some(builder),
+                                Err(assembly) => { self.assembly = Some(assembly); return Ok(self.fail(b"fem2d.assembly-false-terminal".to_vec())); }
+                            };
+                            self.stage = MountedStage::BuildCsr;
+                            self.progress(b"fem2d.loads-complete")
+                        }
+                        Err(detail) => self.fail(detail),
+                    }
+                }
+                MountedStage::BuildCsr => match self.csr_build.as_mut().expect("CSR builder retained").step_one_funded(cx.retained_grant()).and_then(|(complete, progress)| {
+                    cx.consume_retained(progress).map_err(|_| b"fem2d.csr-grant" as &'static [u8])?;
+                    Ok(complete)
+                }) {
                     Ok(false) => {
                         cx.consume_fuel(1);
                         self.progress(b"fem2d.csr-building")
                     }
                     Ok(true) => {
-                        let matrix = self.csr_build.as_mut().and_then(AssemblyCsrBuild::take_complete).expect("complete CSR transfers exactly once");
-                        self.pcg_build = Some(PcgJobConstruction::new(self.identity.operation(), matrix));
+                        let Some((matrix, progress)) = self.csr_build.as_mut().and_then(|build| build.take_complete(cx.retained_grant())) else { return Ok(self.progress(b"fem2d.csr-transfer-pending")); };
+                        if cx.consume_retained(progress).is_err() { return Ok(self.fail(b"fem2d.csr-transfer-grant".to_vec())); }
+                        let rhs = std::mem::take(&mut self.load_build.as_mut().expect("assembled loads retained").free);
+                        self.pcg_build = match PcgJobConstruction::new_with_mounted_rhs(self.identity.operation(), matrix, rhs) {
+                            Ok(builder) => Some(builder),
+                            Err((matrix, rhs)) => { self.rejected_pcg_matrix = Some(matrix); self.load_build.as_mut().expect("load owner retained").free = rhs; return Ok(self.fail(b"fem2d.pcg-rhs-shape".to_vec())); }
+                        };
                         self.stage = MountedStage::PreparePcg;
                         cx.consume_fuel(1);
                         self.progress(b"fem2d.csr-complete")
@@ -210,7 +258,7 @@

                     Ok(true) => {
                         self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                         if self.pcg.is_none() {
-                            return self.fail(b"fem2d.pcg-false-terminal".to_vec());
+                            return Ok(self.fail(b"fem2d.pcg-false-terminal".to_vec()));
                         }
                         self.stage = MountedStage::Pcg;
                         cx.consume_fuel(1);
@@ -218,7 +266,7 @@

                     }
                     Err(detail) => self.fail(detail.to_vec()),
                 },
-                MountedStage::Pcg => match child_end(self.pcg.as_mut().expect("pcg stage owns pcg").step(cx)) {
+                MountedStage::Pcg => match child_end(self.pcg.as_mut().expect("pcg stage owns pcg").step(cx))? {
                     ChildEnd::Complete(output) => {
                         if let Some(job) = self.pcg.as_ref() {
                             let (completed, total, residual, tolerance, converged) = job.visual_progress();
@@ -231,7 +279,7 @@

                         }
                         self.visual_field_cursor = 0;
                         self.visual_pcg_complete = true;
-                        self.stage = MountedStage::SyncPcgVisual;
+                        self.stage = MountedStage::RecoverReactions;
                         match output {
                             Some(bytes) => JobStep::Running(Some(bytes)),
                             None => self.progress(b"fem2d.pcg-complete"),
@@ -259,24 +307,41 @@

                     ChildEnd::Cancelled => self.fail(b"fem2d.pcg-cancelled".to_vec()),
                     ChildEnd::Fault(detail) => self.fail(detail.unwrap_or_else(|| b"fem2d.pcg-fault-capacity".to_vec())),
                 },
+                MountedStage::RecoverReactions => {
+                    cx.consume_fuel(1);
+                    let loads = self.load_build.as_mut().expect("physical RHS retained for equilibrium");
+                    if !loads.reactions.admit_one(loads.full.len()).unwrap_or(false) { return Ok(self.progress(b"fem2d.reaction-admitting")); }
+                    if loads.reactions.len() < loads.full.len() {
+                        let value = -loads.full.get(loads.reactions.len()).expect("full RHS scalar retained");
+                        if loads.reactions.push(value).is_err() { return Ok(self.fail(b"fem2d.reaction-capacity".to_vec())); }
+                        return Ok(self.progress(b"fem2d.reaction-initializing"));
+                    }
+                    let csr = self.csr_build.as_ref().expect("full stiffness retained");
+                    if let Some((row, column, coefficient)) = csr.visual_full_entry(self.reaction_entry) {
+                        let displacement = csr.visual_compact_index(column).and_then(|index| self.pcg.as_ref()?.visual_scalar(index)).map_or(0.0, |scalar| scalar.displacement);
+                        if loads.reactions.add_at(row, coefficient * displacement).is_err() { return Ok(self.fail(b"fem2d.reaction-index".to_vec())); }
+                        self.reaction_entry += 1;
+                    } else { self.stage = MountedStage::SyncPcgVisual; }
+                    self.progress(b"fem2d.reaction-recovering")
+                }
                 MountedStage::SyncPcgVisual => {
-                    let Some(snapshot) = self.snapshot.as_ref() else { return self.fail(b"fem2d.visual-snapshot-owner".to_vec()) };
-                    let Some(node) = snapshot.nodes.get(self.visual_field_cursor) else {
+                    let Some(node) = self.csr_build.as_ref().and_then(|assembly| assembly.model_node(self.visual_field_cursor)) else {
                         self.visual_dirty = true;
                         self.stage = if self.visual_pcg_complete { MountedStage::CommitReady } else { MountedStage::Pcg };
                         cx.consume_fuel(1);
-                        return self.progress(b"fem2d.visual-field-page-complete");
+                        return Ok(self.progress(b"fem2d.visual-field-page-complete"));
                     };
-                    let base = self.visual_field_cursor * 3;
-                    let Some(tx) = self.pcg.as_ref().and_then(|job| job.visual_scalar(base)) else { return self.fail(b"fem2d.visual-tx-scalar".to_vec()) };
-                    let Some(ty) = self.pcg.as_ref().and_then(|job| job.visual_scalar(base + 1)) else { return self.fail(b"fem2d.visual-ty-scalar".to_vec()) };
+                    let scalar = |dof| self.csr_build.as_ref().and_then(|assembly| assembly.visual_equation_indices(&node.id, dof)).and_then(|(_, free)| free).and_then(|index| self.pcg.as_ref()?.visual_scalar(index));
+                    let tx = scalar(semio_s_fem_model::Dof::Tx);
+                    let ty = scalar(semio_s_fem_model::Dof::Ty);
+                    let reaction = |dof| self.csr_build.as_ref().and_then(|assembly| assembly.visual_equation_indices(&node.id, dof)).and_then(|(full, _)| self.load_build.as_ref()?.reactions.get(full)).unwrap_or(0.0);
                     let field = semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField {
                         node_id: node.id.clone(),
-                        displacement: [tx.displacement, ty.displacement],
-                        residual: [tx.residual, ty.residual],
-                        reaction: [tx.reaction, ty.reaction],
-                        contour: tx.contour.max(ty.contour),
-                        mode_shape: [tx.mode_estimate, ty.mode_estimate],
+                        displacement: [tx.map_or(0.0, |scalar| scalar.displacement), ty.map_or(0.0, |scalar| scalar.displacement)],
+                        residual: [tx.map_or(0.0, |scalar| scalar.residual), ty.map_or(0.0, |scalar| scalar.residual)],
+                        reaction: [reaction(semio_s_fem_model::Dof::Tx), reaction(semio_s_fem_model::Dof::Ty)],
+                        contour: tx.map_or(0.0, |scalar| scalar.contour).max(ty.map_or(0.0, |scalar| scalar.contour)),
+                        mode_shape: [0.0; 2],
                     };
                     if let Some(target) = self.visual.fields.get_mut(self.visual_field_cursor) {
                         *target = field;
@@ -288,12 +353,19 @@

                     self.progress(b"fem2d.visual-field-entry")
                 }
                 MountedStage::CommitReady => {
-                    let validation = current_identity(self.identity.app_instance_id);
+                    let validation = current_identity(self.identity);
                     let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                     if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
-                        return self.fail(b"fem2d.session-stale-commit".to_vec());
+                        return Ok(self.fail(b"fem2d.session-stale-commit".to_vec()));
                     }
                     self.visual.validated_final = self.visual.converged;
+                    if self.visual.validated_final {
+                        MOUNTED.with(|registry| {
+                            let mut registry = registry.borrow_mut();
+                            let Some(slot) = registry.current.iter().position(|current| current.is_some_and(|current| current.identity == self.identity)) else { return Ok()};
+                            if let Some(current) = registry.current[slot].as_mut().filter(|current| current.identity == self.identity) { current.previous_valid = None; }
+                        });
+                    }
                     self.visual.state = if self.visual.validated_final { FemVisualState::ValidatedFinal } else { FemVisualState::SolvingUnconverged };
                     self.visual_dirty = true;
                     self.stage = MountedStage::PublishFinalVisual;
@@ -301,10 +373,10 @@

                     self.progress(b"fem2d.final-visual-requested")
                 }
                 MountedStage::PublishFinalVisual => {
-                    let validation = current_identity(self.identity.app_instance_id);
+                    let validation = current_identity(self.identity);
                     let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                     if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
-                        return self.fail(b"fem2d.session-stale-final-visual".to_vec());
+                        return Ok(self.fail(b"fem2d.session-stale-final-visual".to_vec()));
                     }
                     self.stage = MountedStage::Complete;
                     let mut output = Vec::with_capacity(32);
@@ -318,6 +390,6 @@

                 MountedStage::Complete => JobStep::Done(Vec::new()),
                 MountedStage::Fault => JobStep::Failed(self.fault.clone().unwrap_or_else(|| b"fem2d.session-fault".to_vec())),
                 MountedStage::Closing | MountedStage::Empty => JobStep::Failed(b"fem2d.session-closed".to_vec()),
-            }
+            })
         })()
     }
```

### B to H

```diff
--- B

+++ H

@@ -1,4 +1,4 @@

-    fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep {
+    pub(super) fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep {
         if self.cancel.is_cancelled_now() {
             self.stage = MountedStage::Closing;
             return self.fail(b"fem2d.session-cancelled".to_vec());
```

### H to current ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🔁️mounted-state-step/🦀️.rs:7

```diff
--- H

+++ C

@@ -1,19 +1,19 @@

-    pub(super) fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep {
+    pub(super) fn step(&mut self, cx: &mut StepContext<'_>) -> Result<JobStep, semio_framework_job::OriginalTurnFailure> {
         if self.cancel.is_cancelled_now() {
             self.stage = MountedStage::Closing;
-            return self.fail(b"fem2d.session-cancelled".to_vec());
+            return Ok(self.fail(b"fem2d.session-cancelled".to_vec()));
         }
-        if budget.fuel == 0 || budget.deadline_ms == 0 {
-            return JobStep::Running(None);
+        if cx.should_yield() {
+            return Ok(JobStep::Running(None));
         }
         (|| {
             if cx.should_yield() {
-                return JobStep::Running(None);
+                return Ok(JobStep::Running(None));
             }
             if let Some(step) = self.drive_visual_one(cx) {
-                return step;
+                return Ok(step);
             }
-            match self.stage {
+            Ok(match self.stage {
                 MountedStage::PrepareGraph => {
                     const PLANS: [FemStagePlan; 8] = [
                         FemStagePlan { stage: FemJobStage::ValidateReferences, units: 1 },
@@ -27,7 +27,7 @@

                     ];
                     if self.graph_plans.capacity() == 0 {
                         if self.graph_plans.try_reserve_exact(PLANS.len()).is_err() || self.graph_plans.capacity() * size_of::<FemStagePlan>() > SESSION_OWNER_PAGE_BYTES {
-                            return self.fail(b"fem2d.graph-plan-allocation".to_vec());
+                            return Ok(self.fail(b"fem2d.graph-plan-allocation".to_vec()));
                         }
                     } else if let Some(plan) = PLANS.get(self.graph_plans.len()).cloned() {
                         self.graph_plans.push(plan);
@@ -38,7 +38,7 @@

                     cx.consume_fuel(1);
                     self.progress(b"fem2d.graph-admitted")
                 }
-                MountedStage::Graph => match child_end(self.graph.as_mut().expect("graph stage owns graph").step(cx)) {
+                MountedStage::Graph => match child_end(self.graph.as_mut().expect("graph stage owns graph").step(cx))? {
                     ChildEnd::Complete(output) => {
                         self.stage = MountedStage::PrepareDomain;
                         match output {
@@ -61,9 +61,9 @@

                     }
                     Ok(true) => {
                         let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
-                        if let (Some(region), Some(domain)) = (snapshot.regions.first(), self.domain.take()) {
+                        if let (Some(region), Some(domain)) = (snapshot.regions.get(self.region_cursor), self.domain.take()) {
                             if self.visual.region_quality.insert(region.id.clone(), RegionVisualQuality::Unmeshed).is_err() {
-                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
+                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                             }
                             self.visual.state = FemVisualState::Unmeshed;
                             self.visual_dirty = true;
@@ -77,16 +77,22 @@

                     }
                     Err(detail) => self.fail(detail.to_vec()),
                 },
-                MountedStage::Mesh => match child_end(self.mesh.as_mut().expect("mesh stage owns mesh").step(cx)) {
+                MountedStage::Mesh => match child_end(self.mesh.as_mut().expect("mesh stage owns mesh").step(cx))? {
                     ChildEnd::Complete(output) => {
-                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
+                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.get(self.region_cursor)) {
                             if !self.visual.region_quality.update(&region.id, RegionVisualQuality::Final) {
-                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
+                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                             }
                             self.visual.state = FemVisualState::Refined;
                             self.visual_dirty = true;
                         }
-                        self.model_build = Some(MountedModelBuild::new(self.mesh.as_mut().and_then(MeshJob::take_completed_mesh)));
+                        let completed_mesh = self.mesh.as_mut().and_then(MeshJob::take_completed_mesh);
+                        if let Some(builder) = self.model_build.as_mut() {
+                            let Some(mesh) = completed_mesh else { return Ok(self.fail(b"fem2d.mesh-terminal-owner".to_vec())); };
+                            builder.attach_region(self.region_cursor, mesh);
+                        } else {
+                            self.model_build = Some(MountedModelBuild::new(completed_mesh));
+                        }
                         self.stage = MountedStage::BuildModel;
                         match output {
                             Some(bytes) => JobStep::Running(Some(bytes)),
@@ -94,7 +100,7 @@

                         }
                     }
                     ChildEnd::Preview { bytes: payload, quality } => {
-                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
+                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.get(self.region_cursor)) {
                             let quality = match quality {
                                 Some(0) => RegionVisualQuality::Coarse,
                                 Some(1) => RegionVisualQuality::Refined,
@@ -102,7 +108,7 @@

                                 _ => RegionVisualQuality::Unmeshed,
                             };
                             if !self.visual.region_quality.update(&region.id, quality) {
-                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
+                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                             }
                             self.visual.state = match quality {
                                 RegionVisualQuality::Unmeshed => FemVisualState::Unmeshed,
@@ -129,7 +135,7 @@

                     if self.model_build.is_none() {
                         self.model_build = Some(MountedModelBuild::new(None));
                         cx.consume_fuel(1);
-                        return self.progress(b"fem2d.model-admitted");
+                        return Ok(self.progress(b"fem2d.model-admitted"));
                     }
                     match self.model_build.as_mut().expect("model builder admitted above").step_one(snapshot) {
                         Ok(false) => {
@@ -137,8 +143,14 @@

                             self.progress(b"fem2d.model-building")
                         }
                         Ok(true) => {
-                            let model = std::sync::Arc::new(self.model_build.as_mut().and_then(MountedModelBuild::take_complete).expect("complete model transfers exactly once"));
-                            self.assembly_build = Some(AssemblyJobConstruction::new_owned(model, self.identity.operation(), 1));
+                            if self.region_cursor + 1 < snapshot.regions.len() {
+                                self.model_build.as_mut().expect("completed region retains builder").close_lane = 7;
+                                self.stage = MountedStage::RetireRegion;
+                                cx.consume_fuel(1);
+                                return Ok(self.progress(b"fem2d.region-complete"));
+                            }
+                            let model = self.model_build.as_mut().and_then(MountedModelBuild::take_complete).expect("complete model transfers exactly once");
+                            self.assembly_build = Some(AssemblyJobConstruction::new_mounted(model, self.identity.operation(), 1));
                             self.stage = MountedStage::PrepareAssembly;
                             self.visual.state = FemVisualState::Assembling;
                             cx.consume_fuel(1);
@@ -147,6 +159,26 @@

                         Err(error) => self.fail(error),
                     }
                 }
+                MountedStage::RetireRegion => {
+                    cx.consume_fuel(1);
+                    let builder = self.model_build.as_mut().expect("region retirement retains model");
+                    if !builder.close_step(SESSION_OWNER_PAGE_BYTES).0 { return Ok(self.progress(b"fem2d.region-mesh-close")); }
+                    if let Some(mesh) = self.mesh.as_mut() {
+                        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_OWNER_PAGE_BYTES, maximum_capacity_bytes: SESSION_OWNER_PAGE_BYTES, maximum_release_bytes: SESSION_OWNER_PAGE_BYTES, maximum_depth: SESSION_CLOSE_DEPTH };
+                        match mesh.close_step(grant) {
+                            InteractiveJobCloseStep::Complete { .. } => {}
+                            InteractiveJobCloseStep::Pending { .. } => return Ok(self.progress(b"fem2d.region-job-close")),
+                            _ => return Ok(self.fail(b"fem2d.region-job-close-refused".to_vec())),
+                        }
+                    }
+                    self.mesh = None;
+                    self.region_cursor += 1;
+                    self.domain_outer_cursor = 0;
+                    self.domain_hole_cursor = 0;
+                    self.domain_hole_point_cursor = 0;
+                    self.stage = MountedStage::PrepareDomain;
+                    self.progress(b"fem2d.region-next")
+                }
                 MountedStage::PrepareAssembly => match self.assembly_build.as_mut().expect("assembly construction retained").step_one() {
                     Ok(false) => {
                         cx.consume_fuel(1);
@@ -155,7 +187,7 @@

                     Ok(true) => {
                         self.assembly = self.assembly_build.as_mut().and_then(AssemblyJobConstruction::take_complete);
                         if self.assembly.is_none() {
-                            return self.fail(b"fem2d.assembly-construction-false-terminal".to_vec());
+                            return Ok(self.fail(b"fem2d.assembly-construction-false-terminal".to_vec()));
                         }
                         self.stage = MountedStage::Assembly;
                         cx.consume_fuel(1);
@@ -163,18 +195,10 @@

                     }
                     Err(error) => self.fail(error.to_string().into_bytes()),
                 },
-                MountedStage::Assembly => match child_end(self.assembly.as_mut().expect("assembly stage owns assembly").step(cx)) {
+                MountedStage::Assembly => match child_end(self.assembly.as_mut().expect("assembly stage owns assembly").step(cx))? {
                     ChildEnd::Complete(output) => {
-                        let assembly = self.assembly.take().expect("assembly owner retained");
-                        let csr_build = match AssemblyCsrBuild::new(assembly) {
-                            Ok(builder) => builder,
-                            Err(assembly) => {
-                                self.assembly = Some(assembly);
-                                return self.fail(b"fem2d.assembly-false-terminal".to_vec());
-                            }
-                        };
-                        self.csr_build = Some(csr_build);
-                        self.stage = MountedStage::BuildCsr;
+                        self.load_build = Some(MountedLoadBuild::new(self.source));
+                        self.stage = MountedStage::BuildLoads;
                         match output {
                             Some(bytes) => JobStep::Running(Some(bytes)),
                             None => self.progress(b"fem2d.assembly-complete"),
@@ -188,14 +212,38 @@

                     ChildEnd::Cancelled => self.fail(b"fem2d.assembly-cancelled".to_vec()),
                     ChildEnd::Fault(detail) => self.fail(detail.unwrap_or_else(|| b"fem2d.assembly-fault-capacity".to_vec())),
                 },
-                MountedStage::BuildCsr => match self.csr_build.as_mut().expect("CSR builder retained").step_one() {
+                MountedStage::BuildLoads => {
+                    cx.consume_fuel(1);
+                    match self.load_build.as_mut().expect("load owner retained").step_one(self.snapshot.as_ref().expect("snapshot retained"), self.assembly.as_ref().expect("assembly retained")) {
+                        Ok(false) => self.progress(b"fem2d.load-building"),
+                        Ok(true) => {
+                            let assembly = self.assembly.take().expect("assembly owner retained");
+                            self.csr_build = match AssemblyCsrBuild::new_free(assembly) {
+                                Ok(builder) => Some(builder),
+                                Err(assembly) => { self.assembly = Some(assembly); return Ok(self.fail(b"fem2d.assembly-false-terminal".to_vec())); }
+                            };
+                            self.stage = MountedStage::BuildCsr;
+                            self.progress(b"fem2d.loads-complete")
+                        }
+                        Err(detail) => self.fail(detail),
+                    }
+                }
+                MountedStage::BuildCsr => match self.csr_build.as_mut().expect("CSR builder retained").step_one_funded(cx.retained_grant()).and_then(|(complete, progress)| {
+                    cx.consume_retained(progress).map_err(|_| b"fem2d.csr-grant" as &'static [u8])?;
+                    Ok(complete)
+                }) {
                     Ok(false) => {
                         cx.consume_fuel(1);
                         self.progress(b"fem2d.csr-building")
                     }
                     Ok(true) => {
-                        let matrix = self.csr_build.as_mut().and_then(AssemblyCsrBuild::take_complete).expect("complete CSR transfers exactly once");
-                        self.pcg_build = Some(PcgJobConstruction::new(self.identity.operation(), matrix));
+                        let Some((matrix, progress)) = self.csr_build.as_mut().and_then(|build| build.take_complete(cx.retained_grant())) else { return Ok(self.progress(b"fem2d.csr-transfer-pending")); };
+                        if cx.consume_retained(progress).is_err() { return Ok(self.fail(b"fem2d.csr-transfer-grant".to_vec())); }
+                        let rhs = std::mem::take(&mut self.load_build.as_mut().expect("assembled loads retained").free);
+                        self.pcg_build = match PcgJobConstruction::new_with_mounted_rhs(self.identity.operation(), matrix, rhs) {
+                            Ok(builder) => Some(builder),
+                            Err((matrix, rhs)) => { self.rejected_pcg_matrix = Some(matrix); self.load_build.as_mut().expect("load owner retained").free = rhs; return Ok(self.fail(b"fem2d.pcg-rhs-shape".to_vec())); }
+                        };
                         self.stage = MountedStage::PreparePcg;
                         cx.consume_fuel(1);
                         self.progress(b"fem2d.csr-complete")
@@ -210,7 +258,7 @@

                     Ok(true) => {
                         self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                         if self.pcg.is_none() {
-                            return self.fail(b"fem2d.pcg-false-terminal".to_vec());
+                            return Ok(self.fail(b"fem2d.pcg-false-terminal".to_vec()));
                         }
                         self.stage = MountedStage::Pcg;
                         cx.consume_fuel(1);
@@ -218,7 +266,7 @@

                     }
                     Err(detail) => self.fail(detail.to_vec()),
                 },
-                MountedStage::Pcg => match child_end(self.pcg.as_mut().expect("pcg stage owns pcg").step(cx)) {
+                MountedStage::Pcg => match child_end(self.pcg.as_mut().expect("pcg stage owns pcg").step(cx))? {
                     ChildEnd::Complete(output) => {
                         if let Some(job) = self.pcg.as_ref() {
                             let (completed, total, residual, tolerance, converged) = job.visual_progress();
@@ -231,7 +279,7 @@

                         }
                         self.visual_field_cursor = 0;
                         self.visual_pcg_complete = true;
-                        self.stage = MountedStage::SyncPcgVisual;
+                        self.stage = MountedStage::RecoverReactions;
                         match output {
                             Some(bytes) => JobStep::Running(Some(bytes)),
                             None => self.progress(b"fem2d.pcg-complete"),
@@ -259,24 +307,41 @@

                     ChildEnd::Cancelled => self.fail(b"fem2d.pcg-cancelled".to_vec()),
                     ChildEnd::Fault(detail) => self.fail(detail.unwrap_or_else(|| b"fem2d.pcg-fault-capacity".to_vec())),
                 },
+                MountedStage::RecoverReactions => {
+                    cx.consume_fuel(1);
+                    let loads = self.load_build.as_mut().expect("physical RHS retained for equilibrium");
+                    if !loads.reactions.admit_one(loads.full.len()).unwrap_or(false) { return Ok(self.progress(b"fem2d.reaction-admitting")); }
+                    if loads.reactions.len() < loads.full.len() {
+                        let value = -loads.full.get(loads.reactions.len()).expect("full RHS scalar retained");
+                        if loads.reactions.push(value).is_err() { return Ok(self.fail(b"fem2d.reaction-capacity".to_vec())); }
+                        return Ok(self.progress(b"fem2d.reaction-initializing"));
+                    }
+                    let csr = self.csr_build.as_ref().expect("full stiffness retained");
+                    if let Some((row, column, coefficient)) = csr.visual_full_entry(self.reaction_entry) {
+                        let displacement = csr.visual_compact_index(column).and_then(|index| self.pcg.as_ref()?.visual_scalar(index)).map_or(0.0, |scalar| scalar.displacement);
+                        if loads.reactions.add_at(row, coefficient * displacement).is_err() { return Ok(self.fail(b"fem2d.reaction-index".to_vec())); }
+                        self.reaction_entry += 1;
+                    } else { self.stage = MountedStage::SyncPcgVisual; }
+                    self.progress(b"fem2d.reaction-recovering")
+                }
                 MountedStage::SyncPcgVisual => {
-                    let Some(snapshot) = self.snapshot.as_ref() else { return self.fail(b"fem2d.visual-snapshot-owner".to_vec()) };
-                    let Some(node) = snapshot.nodes.get(self.visual_field_cursor) else {
+                    let Some(node) = self.csr_build.as_ref().and_then(|assembly| assembly.model_node(self.visual_field_cursor)) else {
                         self.visual_dirty = true;
                         self.stage = if self.visual_pcg_complete { MountedStage::CommitReady } else { MountedStage::Pcg };
                         cx.consume_fuel(1);
-                        return self.progress(b"fem2d.visual-field-page-complete");
+                        return Ok(self.progress(b"fem2d.visual-field-page-complete"));
                     };
-                    let base = self.visual_field_cursor * 3;
-                    let Some(tx) = self.pcg.as_ref().and_then(|job| job.visual_scalar(base)) else { return self.fail(b"fem2d.visual-tx-scalar".to_vec()) };
-                    let Some(ty) = self.pcg.as_ref().and_then(|job| job.visual_scalar(base + 1)) else { return self.fail(b"fem2d.visual-ty-scalar".to_vec()) };
+                    let scalar = |dof| self.csr_build.as_ref().and_then(|assembly| assembly.visual_equation_indices(&node.id, dof)).and_then(|(_, free)| free).and_then(|index| self.pcg.as_ref()?.visual_scalar(index));
+                    let tx = scalar(semio_s_fem_model::Dof::Tx);
+                    let ty = scalar(semio_s_fem_model::Dof::Ty);
+                    let reaction = |dof| self.csr_build.as_ref().and_then(|assembly| assembly.visual_equation_indices(&node.id, dof)).and_then(|(full, _)| self.load_build.as_ref()?.reactions.get(full)).unwrap_or(0.0);
                     let field = semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField {
                         node_id: node.id.clone(),
-                        displacement: [tx.displacement, ty.displacement],
-                        residual: [tx.residual, ty.residual],
-                        reaction: [tx.reaction, ty.reaction],
-                        contour: tx.contour.max(ty.contour),
-                        mode_shape: [tx.mode_estimate, ty.mode_estimate],
+                        displacement: [tx.map_or(0.0, |scalar| scalar.displacement), ty.map_or(0.0, |scalar| scalar.displacement)],
+                        residual: [tx.map_or(0.0, |scalar| scalar.residual), ty.map_or(0.0, |scalar| scalar.residual)],
+                        reaction: [reaction(semio_s_fem_model::Dof::Tx), reaction(semio_s_fem_model::Dof::Ty)],
+                        contour: tx.map_or(0.0, |scalar| scalar.contour).max(ty.map_or(0.0, |scalar| scalar.contour)),
+                        mode_shape: [0.0; 2],
                     };
                     if let Some(target) = self.visual.fields.get_mut(self.visual_field_cursor) {
                         *target = field;
@@ -288,12 +353,20 @@

                     self.progress(b"fem2d.visual-field-entry")
                 }
                 MountedStage::CommitReady => {
-                    let validation = current_identity(self.identity.app_instance_id);
+                    let validation = current_identity(self.identity);
                     let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                     if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
-                        return self.fail(b"fem2d.session-stale-commit".to_vec());
+                        return Ok(self.fail(b"fem2d.session-stale-commit".to_vec()));
                     }
                     self.visual.validated_final = self.visual.converged;
+                    if self.visual.validated_final {
+                        let mut progress=RetainedCloneProgress::default();
+    let ready =     MOUNTED.with(|registry| {
+                            let mut registry = registry.borrow_mut();
+                            let Some(slot) = registry.current.iter().position(|current| current.is_some_and(|current| current.identity == self.identity)) else { return Ok()};
+                            if let Some(current) = registry.current[slot].as_mut().filter(|current| current.identity == self.identity) { current.previous_valid = None; }
+                        });
+                    }
                     self.visual.state = if self.visual.validated_final { FemVisualState::ValidatedFinal } else { FemVisualState::SolvingUnconverged };
                     self.visual_dirty = true;
                     self.stage = MountedStage::PublishFinalVisual;
@@ -301,10 +374,10 @@

                     self.progress(b"fem2d.final-visual-requested")
                 }
                 MountedStage::PublishFinalVisual => {
-                    let validation = current_identity(self.identity.app_instance_id);
+                    let validation = current_identity(self.identity);
                     let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                     if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
-                        return self.fail(b"fem2d.session-stale-final-visual".to_vec());
+                        return Ok(self.fail(b"fem2d.session-stale-final-visual".to_vec()));
                     }
                     self.stage = MountedStage::Complete;
                     let mut output = Vec::with_capacity(32);
@@ -318,6 +391,6 @@

                 MountedStage::Complete => JobStep::Done(Vec::new()),
                 MountedStage::Fault => JobStep::Failed(self.fault.clone().unwrap_or_else(|| b"fem2d.session-fault".to_vec())),
                 MountedStage::Closing | MountedStage::Empty => JobStep::Failed(b"fem2d.session-closed".to_vec()),
-            }
+            })
         })()
     }
```

## implMountedState::close_step

### B to S

```diff
--- B

+++ S

@@ -1,49 +1,51 @@

-    fn close_step(&mut self, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+    fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
         let (maximum_items, maximum_bytes) = (grant.maximum_items.min(1), grant.maximum_release_bytes);
         let child = RetainedCloneGrant { maximum_items, ..grant };
         if maximum_items == 0 {
-            return session_pending(0, 0);
+            return Ok(session_pending(0, 0));
         }
         self.cancel.cancel_now();
         self.stage = MountedStage::Closing;
+        if let Some(matrix) = self.rejected_pcg_matrix.as_mut() {
+            let (terminal, items, bytes) = matrix.close_step(maximum_bytes);
+            if terminal { self.rejected_pcg_matrix = None; }
+            return Ok(session_pending(items.max(usize::from(terminal)), bytes));
+        }
         loop {
             match self.close_cursor {
                 0 => {
+                    if self.source.take().is_some() { return Ok(session_pending(1, 0)); }
                     if self.graph_plans.pop().is_some() {
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     let plan_bytes = self.graph_plans.capacity() * size_of::<FemStagePlan>();
                     if plan_bytes != 0 {
                         if plan_bytes > maximum_bytes {
-                            return session_pending(0, 0);
+                            return Ok(session_pending(0, 0));
                         }
                         self.graph_plans = Vec::new();
-                        return session_pending(1, plan_bytes);
+                        return Ok(session_pending(1, plan_bytes));
                     }
                     if let Some(graph) = self.graph.as_mut() {
-                        let step = InteractiveJob::close_step(graph, child);
-                        if let Some(blocked) = session_child_failure(step) {
-                            return blocked;
-                        }
                         if !InteractiveJob::terminal_is_empty(graph) {
-                            return PluginLifecycleStep::Progress(step.progress());
-                        }
+                            let step = InteractiveJob::close_step(graph, cx)?;
+                            return Ok(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress())));
+                        }
+                        if maximum_bytes < size_of::<FemJobGraph>() { return Ok(session_pending(0, 0)); }
                         self.graph = None;
-                        return session_pending(1, size_of::<FemJobGraph>());
+                        return Ok(session_pending(1, size_of::<FemJobGraph>()));
                     }
                     self.close_cursor += 1;
                 }
                 1 => {
                     if let Some(mesh) = self.mesh.as_mut() {
-                        let step = InteractiveJob::close_step(mesh, child);
-                        if let Some(blocked) = session_child_failure(step) {
-                            return blocked;
-                        }
                         if !InteractiveJob::terminal_is_empty(mesh) {
-                            return PluginLifecycleStep::Progress(step.progress());
-                        }
+                            let step = InteractiveJob::close_step(mesh, cx)?;
+                            return Ok(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress())));
+                        }
+                        if maximum_bytes < size_of::<MeshJob>() { return Ok(session_pending(0, 0)); }
                         self.mesh = None;
-                        return session_pending(1, size_of::<MeshJob>());
+                        return Ok(session_pending(1, size_of::<MeshJob>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -51,10 +53,10 @@

                     if let Some(model_build) = self.model_build.as_mut() {
                         let (terminal, released_items, released_bytes) = model_build.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         self.model_build = None;
-                        return session_pending(1, size_of::<MountedModelBuild>());
+                        return Ok(session_pending(1, size_of::<MountedModelBuild>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -62,10 +64,10 @@

                     if let Some(assembly_build) = self.assembly_build.as_mut() {
                         let (terminal, released_items, released_bytes) = assembly_build.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         self.assembly_build = None;
-                        return session_pending(1, size_of::<AssemblyJobConstruction>());
+                        return Ok(session_pending(1, size_of::<AssemblyJobConstruction>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -73,10 +75,10 @@

                     if let Some(assembly) = self.assembly.as_mut() {
                         let (terminal, released_items, released_bytes) = assembly.close_retained_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         self.assembly = None;
-                        return session_pending(1, size_of::<AssemblyJob<'static>>());
+                        return Ok(session_pending(1, size_of::<AssemblyJob<'static>>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -84,66 +86,67 @@

                     if let Some(csr_build) = self.csr_build.as_mut() {
                         let (terminal, released_items, released_bytes) = csr_build.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         self.csr_build = None;
-                        return session_pending(1, size_of::<AssemblyCsrBuild>());
+                        return Ok(session_pending(1, size_of::<AssemblyCsrBuild>()));
                     }
                     self.close_cursor += 1;
                 }
                 6 => {
                     if let Some(pcg_build) = self.pcg_build.as_mut() {
-                        let step = pcg_build.close_step(child);
-                        if let Some(blocked) = session_child_failure(step) {
-                            return blocked;
-                        }
-                        if !matches!(step, InteractiveJobCloseStep::Complete { .. }) {
-                            return PluginLifecycleStep::Progress(step.progress());
-                        }
+                        if !pcg_build.terminal_is_empty() {
+                            let step = pcg_build.close_step(cx)?;
+                            return Ok(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress())));
+                        }
+                        if maximum_bytes < size_of::<PcgJobConstruction>() { return Ok(session_pending(0, 0)); }
                         self.pcg_build = None;
-                        return session_pending(1, size_of::<PcgJobConstruction>());
+                        return Ok(session_pending(1, size_of::<PcgJobConstruction>()));
                     }
                     self.close_cursor += 1;
                 }
                 7 => {
+                    if let Some(loads) = self.load_build.as_mut() {
+                        if !loads.close_step() { return Ok(session_pending(1, 0)); }
+                        self.load_build = None;
+                        return Ok(session_pending(1, size_of::<MountedLoadBuild>()));
+                    }
                     if let Some(pcg) = self.pcg.as_mut() {
-                        let step = InteractiveJob::close_step(pcg, child);
-                        if let Some(blocked) = session_child_failure(step) {
-                            return blocked;
-                        }
                         if !InteractiveJob::terminal_is_empty(pcg) {
-                            return PluginLifecycleStep::Progress(step.progress());
-                        }
+                            let step = InteractiveJob::close_step(pcg, cx)?;
+                            return Ok(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress())));
+                        }
+                        if maximum_bytes < size_of::<PcgJob>() { return Ok(session_pending(0, 0)); }
                         self.pcg = None;
-                        return session_pending(1, size_of::<PcgJob>());
+                        return Ok(session_pending(1, size_of::<PcgJob>()));
                     }
                     self.close_cursor += 1;
                 }
                 8 => {
                     if let Some(domain) = self.domain.as_mut() {
                         match self.domain_close_lane {
-                            0 if domain.outer.pop().is_some() => return session_pending(1, 0),
+                            0 if domain.outer.pop().is_some() => return Ok(session_pending(1, 0)),
                             0 => {
                                 let bytes = domain.outer.capacity() * size_of::<[f64; 2]>();
                                 if bytes > maximum_bytes {
-                                    return session_pending(0, 0);
+                                    return Ok(session_pending(0, 0));
                                 }
                                 domain.outer = Vec::new();
                                 self.domain_close_lane = 1;
-                                return session_pending(1, bytes);
+                                return Ok(session_pending(1, bytes));
                             }
                             1 => {
                                 if let Some(hole) = domain.holes.last_mut() {
                                     if hole.pop().is_some() {
-                                        return session_pending(1, 0);
+                                        return Ok(session_pending(1, 0));
                                     }
                                     let bytes = hole.capacity() * size_of::<[f64; 2]>();
                                     if bytes > maximum_bytes {
-                                        return session_pending(0, 0);
+                                        return Ok(session_pending(0, 0));
                                     }
                                     *hole = Vec::new();
                                     domain.holes.pop();
-                                    return session_pending(1, bytes);
+                                    return Ok(session_pending(1, bytes));
                                 }
                                 self.domain_close_lane = 2;
                                 continue;
@@ -151,15 +154,15 @@

                             2 => {
                                 let bytes = domain.holes.capacity() * size_of::<Vec<[f64; 2]>>();
                                 if bytes > maximum_bytes {
-                                    return session_pending(0, 0);
+                                    return Ok(session_pending(0, 0));
                                 }
                                 domain.holes = Vec::new();
                                 self.domain_close_lane = 3;
-                                return session_pending(1, bytes);
+                                return Ok(session_pending(1, bytes));
                             }
                             _ => {
                                 self.domain = None;
-                                return session_pending(1, 0);
+                                return Ok(session_pending(1, 0));
                             }
                         }
                     }
@@ -170,21 +173,21 @@

                         if field.node_id.capacity() != 0 {
                             let bytes = field.node_id.capacity();
                             if bytes > maximum_bytes {
-                                return session_pending(0, 0);
+                                return Ok(session_pending(0, 0));
                             }
                             field.node_id = String::new();
-                            return session_pending(1, bytes);
+                            return Ok(session_pending(1, bytes));
                         }
                         self.visual.fields.pop();
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     let bytes = self.visual.fields.capacity() * size_of::<semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField>();
                     if bytes != 0 {
                         if bytes > maximum_bytes {
-                            return session_pending(0, 0);
+                            return Ok(session_pending(0, 0));
                         }
                         self.visual.fields = Vec::new();
-                        return session_pending(1, bytes);
+                        return Ok(session_pending(1, bytes));
                     }
                     self.close_cursor += 1;
                 }
@@ -193,21 +196,21 @@

                         if owner.capacity() != 0 {
                             let bytes = owner.capacity();
                             if bytes > maximum_bytes {
-                                return session_pending(0, 0);
+                                return Ok(session_pending(0, 0));
                             }
                             *owner = String::new();
-                            return session_pending(1, bytes);
+                            return Ok(session_pending(1, bytes));
                         }
                         self.visual.assembling_element_ids.pop();
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     let bytes = self.visual.assembling_element_ids.capacity() * size_of::<String>();
                     if bytes != 0 {
                         if bytes > maximum_bytes {
-                            return session_pending(0, 0);
+                            return Ok(session_pending(0, 0));
                         }
                         self.visual.assembling_element_ids = Vec::new();
-                        return session_pending(1, bytes);
+                        return Ok(session_pending(1, bytes));
                     }
                     self.close_cursor += 1;
                 }
@@ -215,20 +218,20 @@

                     if self.visual_region_owner.is_none() {
                         if let Some(owner) = self.visual.region_quality.take_one() {
                             self.visual_region_owner = Some(owner);
-                            return session_pending(1, 0);
+                            return Ok(session_pending(1, 0));
                         }
                     }
                     if let Some((id, _)) = self.visual_region_owner.as_mut() {
                         if id.capacity() != 0 {
                             let bytes = id.capacity();
                             if bytes > maximum_bytes {
-                                return session_pending(0, 0);
+                                return Ok(session_pending(0, 0));
                             }
                             *id = String::new();
-                            return session_pending(1, bytes);
+                            return Ok(session_pending(1, bytes));
                         }
                         self.visual_region_owner = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     self.close_cursor += 1;
                 }
@@ -236,24 +239,24 @@

                     if let Some(candidate) = self.visual_rejected.as_mut() {
                         let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         if !candidate.terminal_is_empty() {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM rejected visual reported false terminal" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM rejected visual reported false terminal" });
                         }
                         self.visual_rejected = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     if let Some(candidate) = self.visual_job_candidate.as_mut() {
                         let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         if !candidate.terminal_is_empty() {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM live visual candidate reported false terminal" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM live visual candidate reported false terminal" });
                         }
                         self.visual_job_candidate = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     self.close_cursor += 1;
                 }
@@ -261,13 +264,13 @@

                     if let Some(displaced) = self.visual_displaced.as_mut() {
                         let (terminal, released_items, released_bytes) = displaced.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         if !displaced.terminal_is_empty() {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM displaced visual reported false terminal" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM displaced visual reported false terminal" });
                         }
                         self.visual_displaced = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     self.close_cursor += 1;
                 }
@@ -275,50 +278,50 @@

                     if let Some(current) = self.visual_current.as_mut() {
                         let (terminal, released_items, released_bytes) = current.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         if !current.terminal_is_empty() {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM current visual reported false terminal" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM current visual reported false terminal" });
                         }
                         self.visual_current = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     self.close_cursor += 1;
                 }
                 15 => {
                     if let Some(snapshot) = self.snapshot.take() {
                         let Some(witness) = snapshot.return_to_registry_witness() else {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM snapshot lease was already returned" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM snapshot lease was already returned" });
                         };
                         self.snapshot_return = Some(witness);
-                        return session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>());
+                        return Ok(session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>()));
                     }
                     if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
-                        return session_pending(0, 0);
+                        return Ok(session_pending(0, 0));
                     }
                     if self.snapshot_return.take().is_some() {
-                        return session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotReadReturn>());
+                        return Ok(session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotReadReturn>()));
                     }
                     self.close_cursor += 1;
                 }
                 16 => {
                     if let Some(fault) = self.fault.as_mut() {
                         if fault.pop().is_some() {
-                            return session_pending(1, 0);
+                            return Ok(session_pending(1, 0));
                         }
                         let bytes = fault.capacity();
                         if bytes > maximum_bytes {
-                            return session_pending(0, 0);
+                            return Ok(session_pending(0, 0));
                         }
                         *fault = Vec::new();
                         self.fault = None;
-                        return session_pending(1, bytes);
+                        return Ok(session_pending(1, bytes));
                     }
                     self.close_cursor += 1;
                 }
                 _ => {
                     self.stage = MountedStage::Empty;
-                    return PluginLifecycleStep::Complete(RetainedCloneProgress::default());
+                    return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
                 }
             }
         }
```

### B to H

```diff
--- B

+++ H

@@ -1,4 +1,4 @@

-    fn close_step(&mut self, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+    pub(super) fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
         let (maximum_items, maximum_bytes) = (grant.maximum_items.min(1), grant.maximum_release_bytes);
         let child = RetainedCloneGrant { maximum_items, ..grant };
         if maximum_items == 0 {
@@ -21,10 +21,10 @@

                         return session_pending(1, plan_bytes);
                     }
                     if let Some(graph) = self.graph.as_mut() {
-                        let step = InteractiveJob::close_step(graph, child);
-                        if let Some(blocked) = session_child_failure(step) {
-                            return blocked;
-                        }
+                        let step = match session_child_step(cx.with_retained_loan(child, |same| InteractiveJob::close_step(graph, same))) {
+                            Ok(step) => step,
+                            Err(blocked) => return blocked,
+                        };
                         if !InteractiveJob::terminal_is_empty(graph) {
                             return PluginLifecycleStep::Progress(step.progress());
                         }
@@ -35,10 +35,10 @@

                 }
                 1 => {
                     if let Some(mesh) = self.mesh.as_mut() {
-                        let step = InteractiveJob::close_step(mesh, child);
-                        if let Some(blocked) = session_child_failure(step) {
-                            return blocked;
-                        }
+                        let step = match session_child_step(cx.with_retained_loan(child, |same| InteractiveJob::close_step(mesh, same))) {
+                            Ok(step) => step,
+                            Err(blocked) => return blocked,
+                        };
                         if !InteractiveJob::terminal_is_empty(mesh) {
                             return PluginLifecycleStep::Progress(step.progress());
                         }
@@ -93,10 +93,10 @@

                 }
                 6 => {
                     if let Some(pcg_build) = self.pcg_build.as_mut() {
-                        let step = pcg_build.close_step(child);
-                        if let Some(blocked) = session_child_failure(step) {
-                            return blocked;
-                        }
+                        let step = match session_child_step(pcg_build.close_step(child)) {
+                            Ok(step) => step,
+                            Err(blocked) => return blocked,
+                        };
                         if !matches!(step, InteractiveJobCloseStep::Complete { .. }) {
                             return PluginLifecycleStep::Progress(step.progress());
                         }
@@ -107,10 +107,10 @@

                 }
                 7 => {
                     if let Some(pcg) = self.pcg.as_mut() {
-                        let step = InteractiveJob::close_step(pcg, child);
-                        if let Some(blocked) = session_child_failure(step) {
-                            return blocked;
-                        }
+                        let step = match session_child_step(cx.with_retained_loan(child, |same| InteractiveJob::close_step(pcg, same))) {
+                            Ok(step) => step,
+                            Err(blocked) => return blocked,
+                        };
                         if !InteractiveJob::terminal_is_empty(pcg) {
                             return PluginLifecycleStep::Progress(step.progress());
                         }
```

### H to current ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧹️mounted-state-close/🦀️.rs:7

```diff
--- H

+++ C

@@ -2,34 +2,38 @@

         let (maximum_items, maximum_bytes) = (grant.maximum_items.min(1), grant.maximum_release_bytes);
         let child = RetainedCloneGrant { maximum_items, ..grant };
         if maximum_items == 0 {
-            return session_pending(0, 0);
+            return Ok(session_pending(0, 0));
         }
         self.cancel.cancel_now();
         self.stage = MountedStage::Closing;
+        if let Some(matrix) = self.rejected_pcg_matrix.as_mut() {
+            let (terminal, items, bytes) = matrix.close_step(maximum_bytes);
+            if terminal { self.rejected_pcg_matrix = None; }
+            return Ok(session_pending(items.max(usize::from(terminal)), bytes));
+        }
         loop {
             match self.close_cursor {
                 0 => {
+                    if self.source.take().is_some() { return Ok(session_pending(1, 0)); }
                     if self.graph_plans.pop().is_some() {
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     let plan_bytes = self.graph_plans.capacity() * size_of::<FemStagePlan>();
                     if plan_bytes != 0 {
                         if plan_bytes > maximum_bytes {
-                            return session_pending(0, 0);
+                            return Ok(session_pending(0, 0));
                         }
                         self.graph_plans = Vec::new();
-                        return session_pending(1, plan_bytes);
+                        return Ok(session_pending(1, plan_bytes));
                     }
                     if let Some(graph) = self.graph.as_mut() {
                         let step = match session_child_step(cx.with_retained_loan(child, |same| InteractiveJob::close_step(graph, same))) {
                             Ok(step) => step,
                             Err(blocked) => return blocked,
                         };
-                        if !InteractiveJob::terminal_is_empty(graph) {
-                            return PluginLifecycleStep::Progress(step.progress());
-                        }
+                        if maximum_bytes < size_of::<FemJobGraph>() { return Ok(session_pending(0, 0)); }
                         self.graph = None;
-                        return session_pending(1, size_of::<FemJobGraph>());
+                        return Ok(session_pending(1, size_of::<FemJobGraph>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -39,11 +43,9 @@

                             Ok(step) => step,
                             Err(blocked) => return blocked,
                         };
-                        if !InteractiveJob::terminal_is_empty(mesh) {
-                            return PluginLifecycleStep::Progress(step.progress());
-                        }
+                        if maximum_bytes < size_of::<MeshJob>() { return Ok(session_pending(0, 0)); }
                         self.mesh = None;
-                        return session_pending(1, size_of::<MeshJob>());
+                        return Ok(session_pending(1, size_of::<MeshJob>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -51,10 +53,10 @@

                     if let Some(model_build) = self.model_build.as_mut() {
                         let (terminal, released_items, released_bytes) = model_build.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         self.model_build = None;
-                        return session_pending(1, size_of::<MountedModelBuild>());
+                        return Ok(session_pending(1, size_of::<MountedModelBuild>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -62,10 +64,10 @@

                     if let Some(assembly_build) = self.assembly_build.as_mut() {
                         let (terminal, released_items, released_bytes) = assembly_build.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         self.assembly_build = None;
-                        return session_pending(1, size_of::<AssemblyJobConstruction>());
+                        return Ok(session_pending(1, size_of::<AssemblyJobConstruction>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -73,10 +75,10 @@

                     if let Some(assembly) = self.assembly.as_mut() {
                         let (terminal, released_items, released_bytes) = assembly.close_retained_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         self.assembly = None;
-                        return session_pending(1, size_of::<AssemblyJob<'static>>());
+                        return Ok(session_pending(1, size_of::<AssemblyJob<'static>>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -84,10 +86,10 @@

                     if let Some(csr_build) = self.csr_build.as_mut() {
                         let (terminal, released_items, released_bytes) = csr_build.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         self.csr_build = None;
-                        return session_pending(1, size_of::<AssemblyCsrBuild>());
+                        return Ok(session_pending(1, size_of::<AssemblyCsrBuild>()));
                     }
                     self.close_cursor += 1;
                 }
@@ -97,53 +99,54 @@

                             Ok(step) => step,
                             Err(blocked) => return blocked,
                         };
-                        if !matches!(step, InteractiveJobCloseStep::Complete { .. }) {
-                            return PluginLifecycleStep::Progress(step.progress());
-                        }
+                        if maximum_bytes < size_of::<PcgJobConstruction>() { return Ok(session_pending(0, 0)); }
                         self.pcg_build = None;
-                        return session_pending(1, size_of::<PcgJobConstruction>());
+                        return Ok(session_pending(1, size_of::<PcgJobConstruction>()));
                     }
                     self.close_cursor += 1;
                 }
                 7 => {
+                    if let Some(loads) = self.load_build.as_mut() {
+                        if !loads.close_step() { return Ok(session_pending(1, 0)); }
+                        self.load_build = None;
+                        return Ok(session_pending(1, size_of::<MountedLoadBuild>()));
+                    }
                     if let Some(pcg) = self.pcg.as_mut() {
                         let step = match session_child_step(cx.with_retained_loan(child, |same| InteractiveJob::close_step(pcg, same))) {
                             Ok(step) => step,
                             Err(blocked) => return blocked,
                         };
-                        if !InteractiveJob::terminal_is_empty(pcg) {
-                            return PluginLifecycleStep::Progress(step.progress());
-                        }
+                        if maximum_bytes < size_of::<PcgJob>() { return Ok(session_pending(0, 0)); }
                         self.pcg = None;
-                        return session_pending(1, size_of::<PcgJob>());
+                        return Ok(session_pending(1, size_of::<PcgJob>()));
                     }
                     self.close_cursor += 1;
                 }
                 8 => {
                     if let Some(domain) = self.domain.as_mut() {
                         match self.domain_close_lane {
-                            0 if domain.outer.pop().is_some() => return session_pending(1, 0),
+                            0 if domain.outer.pop().is_some() => return Ok(session_pending(1, 0)),
                             0 => {
                                 let bytes = domain.outer.capacity() * size_of::<[f64; 2]>();
                                 if bytes > maximum_bytes {
-                                    return session_pending(0, 0);
+                                    return Ok(session_pending(0, 0));
                                 }
                                 domain.outer = Vec::new();
                                 self.domain_close_lane = 1;
-                                return session_pending(1, bytes);
+                                return Ok(session_pending(1, bytes));
                             }
                             1 => {
                                 if let Some(hole) = domain.holes.last_mut() {
                                     if hole.pop().is_some() {
-                                        return session_pending(1, 0);
+                                        return Ok(session_pending(1, 0));
                                     }
                                     let bytes = hole.capacity() * size_of::<[f64; 2]>();
                                     if bytes > maximum_bytes {
-                                        return session_pending(0, 0);
+                                        return Ok(session_pending(0, 0));
                                     }
                                     *hole = Vec::new();
                                     domain.holes.pop();
-                                    return session_pending(1, bytes);
+                                    return Ok(session_pending(1, bytes));
                                 }
                                 self.domain_close_lane = 2;
                                 continue;
@@ -151,15 +154,15 @@

                             2 => {
                                 let bytes = domain.holes.capacity() * size_of::<Vec<[f64; 2]>>();
                                 if bytes > maximum_bytes {
-                                    return session_pending(0, 0);
+                                    return Ok(session_pending(0, 0));
                                 }
                                 domain.holes = Vec::new();
                                 self.domain_close_lane = 3;
-                                return session_pending(1, bytes);
+                                return Ok(session_pending(1, bytes));
                             }
                             _ => {
                                 self.domain = None;
-                                return session_pending(1, 0);
+                                return Ok(session_pending(1, 0));
                             }
                         }
                     }
@@ -170,21 +173,21 @@

                         if field.node_id.capacity() != 0 {
                             let bytes = field.node_id.capacity();
                             if bytes > maximum_bytes {
-                                return session_pending(0, 0);
+                                return Ok(session_pending(0, 0));
                             }
                             field.node_id = String::new();
-                            return session_pending(1, bytes);
+                            return Ok(session_pending(1, bytes));
                         }
                         self.visual.fields.pop();
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     let bytes = self.visual.fields.capacity() * size_of::<semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField>();
                     if bytes != 0 {
                         if bytes > maximum_bytes {
-                            return session_pending(0, 0);
+                            return Ok(session_pending(0, 0));
                         }
                         self.visual.fields = Vec::new();
-                        return session_pending(1, bytes);
+                        return Ok(session_pending(1, bytes));
                     }
                     self.close_cursor += 1;
                 }
@@ -193,21 +196,21 @@

                         if owner.capacity() != 0 {
                             let bytes = owner.capacity();
                             if bytes > maximum_bytes {
-                                return session_pending(0, 0);
+                                return Ok(session_pending(0, 0));
                             }
                             *owner = String::new();
-                            return session_pending(1, bytes);
+                            return Ok(session_pending(1, bytes));
                         }
                         self.visual.assembling_element_ids.pop();
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     let bytes = self.visual.assembling_element_ids.capacity() * size_of::<String>();
                     if bytes != 0 {
                         if bytes > maximum_bytes {
-                            return session_pending(0, 0);
+                            return Ok(session_pending(0, 0));
                         }
                         self.visual.assembling_element_ids = Vec::new();
-                        return session_pending(1, bytes);
+                        return Ok(session_pending(1, bytes));
                     }
                     self.close_cursor += 1;
                 }
@@ -215,20 +218,20 @@

                     if self.visual_region_owner.is_none() {
                         if let Some(owner) = self.visual.region_quality.take_one() {
                             self.visual_region_owner = Some(owner);
-                            return session_pending(1, 0);
+                            return Ok(session_pending(1, 0));
                         }
                     }
                     if let Some((id, _)) = self.visual_region_owner.as_mut() {
                         if id.capacity() != 0 {
                             let bytes = id.capacity();
                             if bytes > maximum_bytes {
-                                return session_pending(0, 0);
+                                return Ok(session_pending(0, 0));
                             }
                             *id = String::new();
-                            return session_pending(1, bytes);
+                            return Ok(session_pending(1, bytes));
                         }
                         self.visual_region_owner = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     self.close_cursor += 1;
                 }
@@ -236,24 +239,24 @@

                     if let Some(candidate) = self.visual_rejected.as_mut() {
                         let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         if !candidate.terminal_is_empty() {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM rejected visual reported false terminal" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM rejected visual reported false terminal" });
                         }
                         self.visual_rejected = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     if let Some(candidate) = self.visual_job_candidate.as_mut() {
                         let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         if !candidate.terminal_is_empty() {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM live visual candidate reported false terminal" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM live visual candidate reported false terminal" });
                         }
                         self.visual_job_candidate = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     self.close_cursor += 1;
                 }
@@ -261,13 +264,13 @@

                     if let Some(displaced) = self.visual_displaced.as_mut() {
                         let (terminal, released_items, released_bytes) = displaced.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         if !displaced.terminal_is_empty() {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM displaced visual reported false terminal" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM displaced visual reported false terminal" });
                         }
                         self.visual_displaced = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     self.close_cursor += 1;
                 }
@@ -275,50 +278,50 @@

                     if let Some(current) = self.visual_current.as_mut() {
                         let (terminal, released_items, released_bytes) = current.close_step(maximum_bytes);
                         if !terminal {
-                            return session_pending(released_items, released_bytes);
+                            return Ok(session_pending(released_items, released_bytes));
                         }
                         if !current.terminal_is_empty() {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM current visual reported false terminal" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM current visual reported false terminal" });
                         }
                         self.visual_current = None;
-                        return session_pending(1, 0);
+                        return Ok(session_pending(1, 0));
                     }
                     self.close_cursor += 1;
                 }
                 15 => {
                     if let Some(snapshot) = self.snapshot.take() {
                         let Some(witness) = snapshot.return_to_registry_witness() else {
-                            return PluginLifecycleStep::Blocked { reason: "mounted FEM snapshot lease was already returned" };
+                            return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM snapshot lease was already returned" });
                         };
                         self.snapshot_return = Some(witness);
-                        return session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>());
+                        return Ok(session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>()));
                     }
                     if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
-                        return session_pending(0, 0);
+                        return Ok(session_pending(0, 0));
                     }
                     if self.snapshot_return.take().is_some() {
-                        return session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotReadReturn>());
+                        return Ok(session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotReadReturn>()));
                     }
                     self.close_cursor += 1;
                 }
                 16 => {
                     if let Some(fault) = self.fault.as_mut() {
                         if fault.pop().is_some() {
-                            return session_pending(1, 0);
+                            return Ok(session_pending(1, 0));
                         }
                         let bytes = fault.capacity();
                         if bytes > maximum_bytes {
-                            return session_pending(0, 0);
+                            return Ok(session_pending(0, 0));
                         }
                         *fault = Vec::new();
                         self.fault = None;
-                        return session_pending(1, bytes);
+                        return Ok(session_pending(1, bytes));
                     }
                     self.close_cursor += 1;
                 }
                 _ => {
                     self.stage = MountedStage::Empty;
-                    return PluginLifecycleStep::Complete(RetainedCloneProgress::default());
+                    return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
                 }
             }
         }
```

## free::retire_one

### B to S

```diff
--- B

+++ S

@@ -1,30 +1,31 @@

-fn retire_one(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
     MOUNTED.with(|registry| {
         let mut registry = registry.borrow_mut();
-        let Some(index) = registry.retiring.iter().position(|shell| shell.is_some_and(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity.app_instance_id == app_instance_id))))
+        let Some(index) = registry.retiring.iter().position(|entry| entry.is_some_and(|entry| entry.app_instance_id == app_instance_id && !registry.current.iter().flatten().any(|current| current.previous_valid == Some(entry.shell))))
         else {
-            return PluginLifecycleStep::Complete(RetainedCloneProgress::default());
+            return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
         };
-        let shell = registry.retiring[index].expect("matched retirement shell");
+        let shell = registry.retiring[index].expect("matched retirement shell").shell;
         let step = {
             let mut owner = match registry.shells[shell as usize].try_borrow_mut() {
                 Ok(owner) => owner,
-                Err(_) => return PluginLifecycleStep::Blocked { reason: "mounted FEM job owner is checked out by its worker turn" },
+                Err(_) => return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM job owner is checked out by its worker turn" }),
             };
-            let Some(state) = owner.as_mut() else { return PluginLifecycleStep::Complete(RetainedCloneProgress::default()) };
-            state.close_step(grant)
+            let Some(state) = owner.as_mut() else { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM retirement lost its original child owner" })};
+            if state.identity.app_instance_id != app_instance_id { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM retirement child app affinity changed" })}
+            state.close_step(cx, grant)?
         };
         if matches!(step, PluginLifecycleStep::Complete(_)) {
             let terminal = registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(MountedState::terminal_is_empty));
             if !terminal {
-                return PluginLifecycleStep::Blocked { reason: "mounted FEM job reported a false terminal shell" };
+                return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM job reported a false terminal shell" });
             }
             *registry.shells[shell as usize].borrow_mut() = None;
             registry.retiring[index] = None;
             registry.release_credit(shell);
             registry.release(shell);
-            return session_pending(1, 0);
+            return Ok(session_pending(1, 0));
         }
-        step
+        Ok(step)
     })
 }
```

### B to H

```diff
--- B

+++ H

@@ -1,4 +1,4 @@

-fn retire_one(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
     MOUNTED.with(|registry| {
         let mut registry = registry.borrow_mut();
         let Some(index) = registry.retiring.iter().position(|shell| shell.is_some_and(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity.app_instance_id == app_instance_id))))
@@ -12,7 +12,7 @@

                 Err(_) => return PluginLifecycleStep::Blocked { reason: "mounted FEM job owner is checked out by its worker turn" },
             };
             let Some(state) = owner.as_mut() else { return PluginLifecycleStep::Complete(RetainedCloneProgress::default()) };
-            state.close_step(grant)
+            state.close_step(cx, grant)
         };
         if matches!(step, PluginLifecycleStep::Complete(_)) {
             let terminal = registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(MountedState::terminal_is_empty));
```

### H to current ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/💼️job-bridge-and-live-mount/🦀️.rs:423

```diff
--- H

+++ C

@@ -1,15 +1,15 @@

-fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
     MOUNTED.with(|registry| {
         let mut registry = registry.borrow_mut();
-        let Some(index) = registry.retiring.iter().position(|shell| shell.is_some_and(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity.app_instance_id == app_instance_id))))
+        let Some(index) = registry.retiring.iter().position(|entry| entry.is_some_and(|entry| entry.app_instance_id == app_instance_id && !registry.current.iter().flatten().any(|current| current.previous_valid == Some(entry.shell))))
         else {
-            return PluginLifecycleStep::Complete(RetainedCloneProgress::default());
+            return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
         };
-        let shell = registry.retiring[index].expect("matched retirement shell");
+        let shell = registry.retiring[index].expect("matched retirement shell").shell;
         let step = {
             let mut owner = match registry.shells[shell as usize].try_borrow_mut() {
                 Ok(owner) => owner,
-                Err(_) => return PluginLifecycleStep::Blocked { reason: "mounted FEM job owner is checked out by its worker turn" },
+                Err(_) => return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM job owner is checked out by its worker turn" }),
             };
             let Some(state) = owner.as_mut() else { return PluginLifecycleStep::Complete(RetainedCloneProgress::default()) };
             state.close_step(cx, grant)
@@ -17,14 +17,14 @@

         if matches!(step, PluginLifecycleStep::Complete(_)) {
             let terminal = registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(MountedState::terminal_is_empty));
             if !terminal {
-                return PluginLifecycleStep::Blocked { reason: "mounted FEM job reported a false terminal shell" };
+                return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM job reported a false terminal shell" });
             }
             *registry.shells[shell as usize].borrow_mut() = None;
             registry.retiring[index] = None;
             registry.release_credit(shell);
             registry.release(shell);
-            return session_pending(1, 0);
+            return Ok(session_pending(1, 0));
         }
-        step
+        Ok(step)
     })
 }
```

## free::maintenance_step

### B to S

```diff
--- B

+++ S

@@ -1,3 +1,11 @@

-pub fn maintenance_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
-    retire_one(app_instance_id, grant)
+pub fn maintenance_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
+    semio_framework_plugin::app::original_lifecycle_step(cx, grant, close_demands(app_instance_id)?, |cx, grant| {
+        let step = if let Some(progress) = MOUNTED.with(|registry| registry.borrow_mut().requests.retire_consumer_step(app_instance_id, grant)) {
+            PluginLifecycleStep::Progress(progress)
+        } else {
+            let owner = MOUNTED.with(|registry| { let registry = registry.borrow(); (0..SESSION_ACTIVE_CAPACITY).find_map(|index| { let (owner, scope, _) = registry.requests.at(index)?; (scope == app_instance_id && registry.requests.references(owner) == 0).then_some(owner) }) });
+            if let Some(owner) = owner { close_request_step(app_instance_id, owner, cx, grant)? } else { retire_one(app_instance_id, cx, grant)? }
+        };
+        Ok((step, terminal_is_empty(app_instance_id)))
+    })
 }
```

### B to H

```diff
--- B

+++ H

@@ -1,3 +1,3 @@

-pub fn maintenance_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
-    retire_one(app_instance_id, grant)
+pub fn maintenance_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+    retire_one(app_instance_id, cx, grant)
 }
```

### H to current ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/💼️job-bridge-and-live-mount/🦀️.rs:454

```diff
```

## free::close_demands

### B to S

```diff
--- B

+++ S

@@ -1,7 +1,12 @@

-pub fn close_demands(app_instance_id: u32) -> RetirementDemand {
-    if terminal_is_empty(app_instance_id) {
-        RetirementDemand::default()
-    } else {
-        RetirementDemand { copy_bytes: SESSION_CLOSE_COPY_BYTES, release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() }
-    }
+pub fn close_demands(app_instance_id: u32) -> Result<RetirementDemand, ValueError> {
+    MOUNTED.with(|registry| {
+        let registry = registry.borrow();
+        let mut demand = RetirementDemand { copy_bytes: SESSION_CLOSE_COPY_BYTES, release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() };
+        for shell in &registry.shells {
+            if let Ok(owner) = shell.try_borrow() {
+                if let Some(state) = owner.as_ref().filter(|state| state.identity.app_instance_id == app_instance_id) { state.child_close_demands(&mut demand)?; }
+            }
+        }
+        Ok(demand)
+    })
 }
```

### B to H

```diff
```

## free::close_step

### B to S

```diff
--- B

+++ S

@@ -1,34 +1,6 @@

-pub fn close_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
-    MOUNTED.with(|registry| {
-        let mut registry = registry.borrow_mut();
-        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
-        if registry.preflight_fault[slot].is_some_and(|fault| fault.app_instance_id == app_instance_id) {
-            registry.preflight_fault[slot] = None;
-            return session_pending(1, size_of::<PendingSnapshotFault>());
-        }
-        if registry.preflight[slot].is_some_and(|preflight| preflight.app_instance_id == app_instance_id) {
-            registry.preflight[slot] = None;
-            return session_pending(1, size_of::<PendingSnapshotAdmission>());
-        }
-        if let Some(pending) = registry.pending[slot].filter(|pending| pending.app_instance_id == app_instance_id) {
-            registry.pending[slot] = None;
-            registry.release_credit(pending.shell);
-            registry.release(pending.shell);
-            return session_pending(1, INPUT_BYTES);
-        }
-        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
-            if !registry.retain_retirement(current.shell) {
-                return PluginLifecycleStep::Blocked { reason: "mounted FEM retirement arena is saturated" };
-            }
-            if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
-                if let Some(state) = owner.as_ref() {
-                    state.cancel.cancel_now();
-                }
-            }
-            registry.current[slot] = None;
-            return session_pending(1, 0);
-        }
-        drop(registry);
-        retire_one(app_instance_id, grant)
+pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
+    semio_framework_plugin::app::original_lifecycle_step(cx, grant, close_demands(app_instance_id)?, |cx, grant| {
+        let step = close_turn(app_instance_id, cx, grant)?;
+        Ok((step, terminal_is_empty(app_instance_id)))
     })
 }
```

### B to H

```diff
--- B

+++ H

@@ -1,4 +1,4 @@

-pub fn close_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
     MOUNTED.with(|registry| {
         let mut registry = registry.borrow_mut();
         let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
@@ -29,6 +29,6 @@

             return session_pending(1, 0);
         }
         drop(registry);
-        retire_one(app_instance_id, grant)
+        retire_one(app_instance_id, cx, grant)
     })
 }
```

### H to current ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/💼️job-bridge-and-live-mount/🦀️.rs:466

```diff
--- H

+++ C

@@ -1,24 +1,32 @@

 pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
     MOUNTED.with(|registry| {
         let mut registry = registry.borrow_mut();
-        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
+        if registry.requests.key(lease).is_none_or(|(scope, _)| scope != app_instance_id) || registry.requests.references(lease) != 0 { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM request is still referenced or its original lease changed" }); }
+        let slot = lease.index();
+        if registry.source_apps[slot] == Some(app_instance_id) {
+            let bytes = size_of::<Option<ResolvedResultSource>>();
+            if bytes > grant.maximum_release_bytes { return Ok(session_pending(0, 0)); }
+            registry.sources[slot] = None;
+            registry.source_apps[slot] = None;
+            return Ok(session_pending(1, bytes));
+        }
         if registry.preflight_fault[slot].is_some_and(|fault| fault.app_instance_id == app_instance_id) {
             registry.preflight_fault[slot] = None;
-            return session_pending(1, size_of::<PendingSnapshotFault>());
+            return Ok(session_pending(1, size_of::<PendingSnapshotFault>()));
         }
         if registry.preflight[slot].is_some_and(|preflight| preflight.app_instance_id == app_instance_id) {
             registry.preflight[slot] = None;
-            return session_pending(1, size_of::<PendingSnapshotAdmission>());
+            return Ok(session_pending(1, size_of::<PendingSnapshotAdmission>()));
         }
         if let Some(pending) = registry.pending[slot].filter(|pending| pending.app_instance_id == app_instance_id) {
             registry.pending[slot] = None;
             registry.release_credit(pending.shell);
             registry.release(pending.shell);
-            return session_pending(1, INPUT_BYTES);
+            return Ok(session_pending(1, INPUT_BYTES));
         }
         if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
-            if !registry.retain_retirement(current.shell) {
-                return PluginLifecycleStep::Blocked { reason: "mounted FEM retirement arena is saturated" };
+            if !registry.retain_retirement(current.shell, current.app_instance_id) {
+                return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM retirement arena is saturated" });
             }
             if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
                 if let Some(state) = owner.as_ref() {
@@ -26,9 +34,21 @@

                 }
             }
             registry.current[slot] = None;
-            return session_pending(1, 0);
+            return Ok(session_pending(1, 0));
         }
         drop(registry);
-        retire_one(app_instance_id, cx, grant)
+        let step = retire_one(app_instance_id, cx, grant)?;
+        if !matches!(step, PluginLifecycleStep::Complete(_)) { return Ok(step)}
+        let children_remain = MOUNTED.with(|registry| registry.borrow().shell_requests.iter().any(|owner| *owner == Some(lease)));
+        if children_remain { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM request still owns its original child shell" }); }
+        let copy_bytes = size_of::<RequestKey>() * 2 + size_of::<semio_framework_value::FixedKeySlot>() * 2 + size_of::<Option<u32>>() + size_of::<usize>() * 2;
+        if grant.maximum_copy_bytes < copy_bytes || grant.maximum_depth == 0 { return Ok(session_pending(0, 0))}
+        if let Some(progress)=MOUNTED.with(|registry|registry.borrow_mut().requests.retire_request_step(lease,grant)){return Ok(PluginLifecycleStep::Progress(progress));}
+        MOUNTED.with(|registry| {
+            let mut registry = registry.borrow_mut();
+            assert!(registry.requests.release(lease), "the original request claim remains live through child retirement");
+            for selected in &mut registry.snapshot_owners { if *selected == Some(lease) { *selected = None; } }
+        });
+        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() }))
     })
 }
```

### H to current ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/💼️job-bridge-and-live-mount/🦀️.rs:522

```diff
--- H

+++ C

@@ -1,34 +1,6 @@

-pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
-    MOUNTED.with(|registry| {
-        let mut registry = registry.borrow_mut();
-        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
-        if registry.preflight_fault[slot].is_some_and(|fault| fault.app_instance_id == app_instance_id) {
-            registry.preflight_fault[slot] = None;
-            return session_pending(1, size_of::<PendingSnapshotFault>());
-        }
-        if registry.preflight[slot].is_some_and(|preflight| preflight.app_instance_id == app_instance_id) {
-            registry.preflight[slot] = None;
-            return session_pending(1, size_of::<PendingSnapshotAdmission>());
-        }
-        if let Some(pending) = registry.pending[slot].filter(|pending| pending.app_instance_id == app_instance_id) {
-            registry.pending[slot] = None;
-            registry.release_credit(pending.shell);
-            registry.release(pending.shell);
-            return session_pending(1, INPUT_BYTES);
-        }
-        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
-            if !registry.retain_retirement(current.shell) {
-                return PluginLifecycleStep::Blocked { reason: "mounted FEM retirement arena is saturated" };
-            }
-            if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
-                if let Some(state) = owner.as_ref() {
-                    state.cancel.cancel_now();
-                }
-            }
-            registry.current[slot] = None;
-            return session_pending(1, 0);
-        }
-        drop(registry);
-        retire_one(app_instance_id, cx, grant)
+pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
+    semio_framework_plugin::app::original_lifecycle_step(cx, grant, close_demands(app_instance_id)?, |cx, grant| {
+        let step = close_turn(app_instance_id, cx, grant)?;
+        Ok((step, terminal_is_empty(app_instance_id)))
     })
 }
```

## free::session_child_failure

### B to S

```diff
--- B

+++ S

@@ -1,7 +1,6 @@

 fn session_child_failure(step: InteractiveJobCloseStep) -> Option<PluginLifecycleStep> {
     match step {
         InteractiveJobCloseStep::Blocked => Some(PluginLifecycleStep::Blocked { reason: "mounted FEM child close is blocked" }),
-        InteractiveJobCloseStep::Refused { .. } => Some(PluginLifecycleStep::Blocked { reason: "mounted FEM child close was refused" }),
         InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. } => None,
     }
 }
```

## free::close_request_step

### S new

```rust
fn close_request_step(app_instance_id: u32, lease: semio_framework_value::FixedKeySlot, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    if grant.maximum_items == 0 { return Ok(session_pending(0, 0)); }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        if registry.requests.key(lease).is_none_or(|(scope, _)| scope != app_instance_id) || registry.requests.references(lease) != 0 { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM request is still referenced or its original lease changed" }); }
        let slot = lease.index();
        if registry.source_apps[slot] == Some(app_instance_id) {
            let bytes = size_of::<Option<ResolvedResultSource>>();
            if bytes > grant.maximum_release_bytes { return Ok(session_pending(0, 0)); }
            registry.sources[slot] = None;
            registry.source_apps[slot] = None;
            return Ok(session_pending(1, bytes));
        }
        if registry.preflight_fault[slot].is_some_and(|fault| fault.app_instance_id == app_instance_id) {
            registry.preflight_fault[slot] = None;
            return Ok(session_pending(1, size_of::<PendingSnapshotFault>()));
        }
        if registry.preflight[slot].is_some_and(|preflight| preflight.app_instance_id == app_instance_id) {
            registry.preflight[slot] = None;
            return Ok(session_pending(1, size_of::<PendingSnapshotAdmission>()));
        }
        if let Some(pending) = registry.pending[slot].filter(|pending| pending.app_instance_id == app_instance_id) {
            registry.pending[slot] = None;
            registry.release_credit(pending.shell);
            registry.release(pending.shell);
            return Ok(session_pending(1, INPUT_BYTES));
        }
        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
            if !registry.retain_retirement(current.shell, current.app_instance_id) {
                return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM retirement arena is saturated" });
            }
            if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
                if let Some(state) = owner.as_ref() {
                    state.cancel.cancel_now();
                }
            }
            registry.current[slot] = None;
            return Ok(session_pending(1, 0));
        }
        drop(registry);
        let step = retire_one(app_instance_id, cx, grant)?;
        if !matches!(step, PluginLifecycleStep::Complete(_)) { return Ok(step)}
        let children_remain = MOUNTED.with(|registry| registry.borrow().shell_requests.iter().any(|owner| *owner == Some(lease)));
        if children_remain { return Ok(PluginLifecycleStep::Blocked { reason: "mounted FEM request still owns its original child shell" }); }
        let copy_bytes = size_of::<RequestKey>() * 2 + size_of::<semio_framework_value::FixedKeySlot>() * 2 + size_of::<Option<u32>>() + size_of::<usize>() * 2;
        if grant.maximum_copy_bytes < copy_bytes || grant.maximum_depth == 0 { return Ok(session_pending(0, 0))}
        if let Some(progress)=MOUNTED.with(|registry|registry.borrow_mut().requests.retire_request_step(lease,grant)){return Ok(PluginLifecycleStep::Progress(progress));}
        MOUNTED.with(|registry| {
            let mut registry = registry.borrow_mut();
            assert!(registry.requests.release(lease), "the original request claim remains live through child retirement");
            for selected in &mut registry.snapshot_owners { if *selected == Some(lease) { *selected = None; } }
        });
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() }))
    })
}
```

## free::close_turn

### S new

```rust
fn close_turn(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    if grant.maximum_items == 0 { return Ok(session_pending(0, 0)); }
    if let Some(step)=MOUNTED.with(|registry|registry.borrow_mut().requests.remove_one_scoped_consumer_controlled(app_instance_id,grant)){return Ok(PluginLifecycleStep::Progress(step.progress));}
    if let Some(progress)=MOUNTED.with(|registry|registry.borrow_mut().requests.retire_consumer_step(app_instance_id,grant)){return Ok(PluginLifecycleStep::Progress(progress));}
    let owner = MOUNTED.with(|registry| { let registry = registry.borrow(); (0..SESSION_ACTIVE_CAPACITY).find_map(|index| { let (owner, scope, _) = registry.requests.at(index)?; (scope == app_instance_id).then_some(owner) }) });
    if let Some(owner) = owner { return Ok(close_request_step(app_instance_id, owner, cx, grant)?); }
    let step = retire_one(app_instance_id, cx, grant)?;
    if !matches!(step, PluginLifecycleStep::Complete(_)) { return Ok(step); }
    let copy_bytes = size_of::<RequestKey>() + size_of::<semio_framework_value::FixedKeySlot>() * 2 + size_of::<Option<u32>>() + size_of::<usize>() * 2;
    if grant.maximum_copy_bytes < copy_bytes || grant.maximum_depth == 0 { return Ok(session_pending(0, 0)); }
    MOUNTED.with(|registry| { let mut registry = registry.borrow_mut(); let Some(app) = registry.app_slots.lookup(app_instance_id) else { return Ok(PluginLifecycleStep::Complete(Default::default())); }; registry.model_requests[app.index()] = None; registry.snapshot_owners[app.index()] = None; assert!(registry.app_slots.release(app_instance_id, app)); Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })) })
}
```
