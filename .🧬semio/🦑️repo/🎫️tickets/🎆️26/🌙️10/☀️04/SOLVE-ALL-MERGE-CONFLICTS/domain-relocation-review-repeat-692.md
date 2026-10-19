# Domain Split Owner Delta Review Repeat 692

## 8 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🦀️.rs

### 0 

```diff
+//! publication — what one mounted session runs per document revision.

```

### 2 

```diff
-//! publication — what one mounted session runs per document revision.

```

### 4 

```diff
-use semio_s_fem_analyses_assembly::{AssemblyCsrBuild, AssemblyJob, AssemblyJobConstruction};
-use semio_s_fem_analyses_model::{MountedAnalysisModel, MountedAnalysisSupport, MOUNTED_ANALYSIS_BACKING_BYTES};
+use semio_s_fem_analyses_assembly::AssemblyCsrBuild;
+
+use semio_s_fem_analyses_assembly::AssemblyJob;
+
+use semio_s_fem_analyses_assembly::AssemblyJobConstruction;
+use semio_s_fem_analyses_model::MountedAnalysisModel;
+use semio_s_fem_analyses_model::MountedAnalysisSupport;
+use semio_s_fem_analyses_model::MOUNTED_ANALYSIS_BACKING_BYTES;

```

### 6 

```diff
-use semio_s_fem_mesh::{MeshJob, MeshOpts, MountedPlanarDomain};
-use semio_s_fem_elements3d::{Bar3, Frame3};
-use semio_s_fem_model::{Dof, Node};
+use semio_s_fem_mesh::MeshJob;
+use semio_s_fem_mesh::MeshOpts;
+use semio_s_fem_mesh::MountedPlanarDomain;
+use semio_s_fem_elements3d::Bar3;
+use semio_s_fem_elements3d::Frame3;
+use semio_s_fem_model::Dof;
+use semio_s_fem_model::Node;

```

### 8 

```diff
-use semio_s_fem_sparse::{Csr, MountedScalarSlots};
+use semio_s_fem_sparse::Csr;
+use semio_s_fem_sparse::MountedScalarSlots;

```

### 10 

```diff
-use semio_s_fem_sparse_pcg::{PcgJob, PcgJobConstruction};
-use semio_s_fem_sparse_subspace::{ModalInputConstruction, SubspaceIterationJob};
+use semio_s_fem_sparse_pcg::PcgJob;
+use semio_s_fem_sparse_pcg::PcgJobConstruction;
+use semio_s_fem_sparse::SymmetricPencilConstruction;
+use semio_s_fem_sparse_subspace::SubspaceIterationJob;

```

### 12 

```diff
-use semio_framework_job::{InteractiveJob, JobOutcomeBorrow, StepContext};
+use semio_framework_job::InteractiveJob;
+use semio_framework_job::JobOutcomeBorrow;
+use semio_framework_job::StepContext;

```

### 14 

```diff
-use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
-use semio_framework_ui_scene::{world3d_snapshot_abort_write, world3d_snapshot_abort_write_step, world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_close_step, world3d_snapshot_recover_lease, world3d_snapshot_recover_page, world3d_snapshot_recover_write, world3d_snapshot_seal, world3d_snapshot_terminal_is_empty, world3d_snapshot_write_terminal_is_empty, World3dSnapshotDescriptor, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPage, World3dSnapshotPageKind, World3dSnapshotWriteToken, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY};
+use semio_framework_value::RetainedCloneGrant;
+use semio_framework_value::RetainedCloneProgress;
+use semio_framework_value::ValueError;
+use semio_framework_ui_scene::world3d_snapshot_abort_write;
+use semio_framework_ui_scene::world3d_snapshot_abort_write_step;
+use semio_framework_ui_scene::world3d_snapshot_admit_page;
+use semio_framework_ui_scene::world3d_snapshot_begin;
+use semio_framework_ui_scene::world3d_snapshot_begin_close;
+use semio_framework_ui_scene::world3d_snapshot_close_step;
+use semio_framework_ui_scene::world3d_snapshot_recover_lease;
+use semio_framework_ui_scene::world3d_snapshot_recover_page;
+use semio_framework_ui_scene::world3d_snapshot_recover_write;
+use semio_framework_ui_scene::world3d_snapshot_seal;
+use semio_framework_ui_scene::world3d_snapshot_terminal_is_empty;
+use semio_framework_ui_scene::world3d_snapshot_write_terminal_is_empty;
+use semio_framework_ui_scene::World3dSnapshotDescriptor;
+use semio_framework_ui_scene::World3dSnapshotItem;
+use semio_framework_ui_scene::World3dSnapshotLease;
+use semio_framework_ui_scene::World3dSnapshotPage;
+use semio_framework_ui_scene::World3dSnapshotPageKind;
+use semio_framework_ui_scene::World3dSnapshotWriteToken;
+use semio_framework_ui_scene::WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY;
+use semio_framework_ui_scene::WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY;

```

### 16 drop

```diff
+    ResolveSourceCase,

```

### 18 drop

```diff
+    RecoverStressElement,
+    PublishStressScalar,

```

### 20 terminal_is_empty

```diff
-fn child_end(result: Result<Option<JobOutcomeBorrow<'_>>, ValueError>) -> ChildEnd {
-    match result {
-        Ok(None | Some(JobOutcomeBorrow::Yield { .. })) => ChildEnd::Yield,
-        Ok(Some(JobOutcomeBorrow::PreviewReady { .. } | JobOutcomeBorrow::CheckpointReady { .. })) => ChildEnd::Lent,
-        Ok(Some(JobOutcomeBorrow::Complete { .. })) => ChildEnd::Complete,
-        Ok(Some(JobOutcomeBorrow::Fault { .. })) | Err(_) => ChildEnd::Fault,
-        Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => ChildEnd::Cancelled,
-    }
+pub enum Fem3dNumericalFailure {
+    Diagnostic(Vec<u8>),
+    Original(semio_framework_job::OriginalTurnFailure),
+}
+impl From<Vec<u8>> for Fem3dNumericalFailure { fn from(value: Vec<u8>) -> Self { Self::Diagnostic(value) } }
+impl From<semio_framework_job::OriginalTurnFailure> for Fem3dNumericalFailure { fn from(value: semio_framework_job::OriginalTurnFailure) -> Self { Self::Original(value) } }
+
+fn child_end(result: Result<Option<JobOutcomeBorrow<'_>>, semio_framework_job::OriginalTurnFailure>) -> Result<ChildEnd, semio_framework_job::OriginalTurnFailure> {
+    Ok(match result? {
+        None | Some(JobOutcomeBorrow::Yield { .. }) => ChildEnd::Yield,
+        Some(JobOutcomeBorrow::PreviewReady { .. } | JobOutcomeBorrow::CheckpointReady { .. }) => ChildEnd::Lent,
+        Some(JobOutcomeBorrow::Complete { .. }) => ChildEnd::Complete,
+        Some(JobOutcomeBorrow::Fault { .. }) => ChildEnd::Fault,
+        Some(JobOutcomeBorrow::Cancelled { .. }) => ChildEnd::Cancelled,
+    })

```

### 22 largest

```diff
-    pub model: Option<MountedAnalysisModel>,
-    pub analysis_node_ids: FixedSlots<String, MAXIMUM_FIELDS>,
+    model: Option<MountedAnalysisModel>,
+    analysis_node_ids: FixedSlots<String, MAXIMUM_FIELDS>,

```

### 24 largest

```diff
-    pub node_cursor: usize,
+    node_cursor: usize,

```

### 26 largest

```diff
-    pub pcg: Option<PcgJob>,
-    modal_build: Option<ModalInputConstruction>,
+    pcg: Option<PcgJob>,
+    modal_build: Option<SymmetricPencilConstruction>,

```

### 28 largest

```diff
-    pub subspace: Option<SubspaceIterationJob>,
+    subspace: Option<SubspaceIterationJob>,

```

### 30 largest

```diff
-    pub scalar_axis: usize,
-    pub scalar: Fem3dSolverScalar,
+    scalar_axis: usize,
+    scalar: Fem3dSolverScalar,

```

### 32 largest

```diff
+    full_reactions: [[f64; 6]; MAXIMUM_FIELDS],
+    stress_sums: [f64; MAXIMUM_FIELDS],
+    stress_counts: [usize; MAXIMUM_FIELDS],

```

### 34 largest

```diff
+    request: Fem3dResultRequest,
+    source_case: usize,
+    source_factor: f64,

```

### 36 largest

```diff
-    pub reaction_entry: usize,
+    reaction_entry: usize,

```

### 38 largest

```diff
-    pub close_lane: u8,
-    pub close_started_jobs: u8,
-    pub operation: Option<semio_framework_job::Operation>,
+    close_lane: u8,
+    close_started_jobs: u8,
+    operation: Option<semio_framework_job::Operation>,

```

### 40 largest

```diff
+    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
+        let mut demand = RetirementDemand { release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() };
+        let mut merge = |child: RetirementDemand| {
+            demand.copy_bytes = demand.copy_bytes.max(child.copy_bytes);
+            demand.capacity_bytes = demand.capacity_bytes.max(child.capacity_bytes);
+            demand.release_bytes = demand.release_bytes.max(child.release_bytes);
+            demand.depth = demand.depth.max(child.depth);
+        };
+        if let Some(child) = self.mesh.as_ref() { merge(semio_s_fem_sparse::numerical_close_demand(child)?); }
+        if let Some(child) = self.pcg.as_ref() { merge(semio_s_fem_sparse::numerical_close_demand(child)?); }
+        if let Some(child) = self.ldlt.as_ref() { merge(semio_s_fem_sparse::numerical_close_demand(child)?); }
+        if let Some(child) = self.subspace.as_ref() { merge(semio_s_fem_sparse::numerical_close_demand(child)?); }
+        if let Some(child) = self.pcg_build.as_ref() { merge(child.close_demand()?); }
+        Ok(demand)
+    }
+

```

### 42 new

```diff
+            full_reactions: [[0.0; 6]; MAXIMUM_FIELDS],
+            stress_sums: [0.0; MAXIMUM_FIELDS],
+            stress_counts: [0; MAXIMUM_FIELDS],

```

### 44 new

```diff
+            request: Fem3dResultRequest::default(),
+            source_case: 0,
+            source_factor: 1.0,

```

### 46 apply_rhs_value

```diff
+        let value = value * self.source_factor;

```

### 48 apply_rhs_value

```diff
-    pub fn observe_child_end(&mut self, end: ChildEnd) -> Result<bool, Vec<u8>> {
+    fn observe_child_end(&mut self, end: ChildEnd) -> Result<bool, Vec<u8>> {

```

### 50 step_model

```diff
-    pub fn step(&mut self, doc: &Fem3dSnapshot, solver: &mut Fem3dSolverView, backing: &mut Fem3dBackingCredit, freshness: Fem3dVisualFreshness, operation: semio_framework_job::Operation, context: &mut StepContext<'_>) -> Result<bool, Vec<u8>> {
+    pub fn step(&mut self, doc: &Fem3dSnapshot, solver: &mut Fem3dSolverView, backing: &mut Fem3dBackingCredit, freshness: Fem3dVisualFreshness, operation: semio_framework_job::Operation, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
+        context.set_stage(self.stage.progress_stage());

```

### 52 step

```diff
-                return Err(b"fem3d.numerical-cancelled".to_vec());
+                return Err((b"fem3d.numerical-cancelled".to_vec()).into());

```

### 54 step

```diff
-                return Err(b"fem3d.numerical-cancelled".to_vec());
+                return Err((b"fem3d.numerical-cancelled".to_vec()).into());

```

### 56 step

```diff
-                    return Err(b"fem3d.numerical-solver-page-admission".to_vec());
+                    return Err((b"fem3d.numerical-solver-page-admission".to_vec()).into());

```

### 58 step

```diff
-                return Err(b"fem3d.numerical-solid-admission".to_vec());
+                return Err((b"fem3d.numerical-solid-admission".to_vec()).into());

```

### 60 step

```diff
-            let end = child_end(self.mesh.as_mut().ok_or_else(|| b"fem3d.numerical-solid-mesh-child".to_vec())?.step(context));
+            let end = child_end(self.mesh.as_mut().ok_or_else(|| b"fem3d.numerical-solid-mesh-child".to_vec())?.step(context))?;

```

### 62 step

```diff
-            let step = InteractiveJob::close_step(mesh, context.retained_grant());
-            context.consume_retained(step.progress()).map_err(|_| b"fem3d.numerical-solid-mesh-retire-grant".to_vec())?;
-            if matches!(step, semio_framework_job::InteractiveJobCloseStep::Refused { .. }) {
-                return Err(b"fem3d.numerical-solid-mesh-retire-refused".to_vec());
+            if !InteractiveJob::terminal_is_empty(mesh) {
+                InteractiveJob::close_step(mesh, context)?;
+                return Ok(false);

```

### 64 step

```diff
-            if InteractiveJob::terminal_is_empty(mesh) {
+            semio_framework_job::close_original_retained_step(context, RetirementDemand { depth: 1, ..Default::default() }, |_| {

```

### 66 step

```diff
-            }
+                Ok(semio_framework_job::RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
+            })?;

```

### 68 step

```diff
-                    return Err(b"fem3d.numerical-field-count".to_vec());
+                    return Err((b"fem3d.numerical-field-count".to_vec()).into());

```

### 70 step

```diff
-            Fem3dNumericalStage::PrepareAssembly => match self.assembly_build.as_mut().ok_or_else(|| b"fem3d.numerical-assembly-build".to_vec())?.step_one() {
+            Fem3dNumericalStage::PrepareAssembly => match self.assembly_build.as_mut().ok_or_else(|| b"fem3d.numerical-assembly-build".to_vec())?.step_one_funded(context.retained_grant()).and_then(|(complete, progress)| {
+                context.consume_retained(progress).map_err(|_| crate::model::FemError::ResourceAdmission)?;
+                Ok(complete)
+            }) {

```

### 72 step

```diff
-                        return Err(b"fem3d.numerical-assembly-false-terminal".to_vec());
+                        return Err((b"fem3d.numerical-assembly-false-terminal".to_vec()).into());

```

### 74 step

```diff
-                Err(error) => return Err(error.to_string().into_bytes()),
+                Err(error) => return Err((error.to_string().into_bytes()).into()),

```

### 76 step

```diff
-                let end = child_end(self.assembly.as_mut().ok_or_else(|| b"fem3d.numerical-assembly".to_vec())?.step(context));
+                let end = child_end(self.assembly.as_mut().ok_or_else(|| b"fem3d.numerical-assembly".to_vec())?.step(context))?;

```

### 78 step

```diff
-                    self.stage = Fem3dNumericalStage::ApplyLoad;
+                    self.stage = Fem3dNumericalStage::ResolveSourceCase;
+                }
+            }
+            Fem3dNumericalStage::ResolveSourceCase => {
+                if self.source_case >= doc.load_cases.len() {
+                    let assembly = self.assembly.take().ok_or_else(|| b"fem3d.numerical-assembly-owner".to_vec())?;
+                    self.csr_build = Some(AssemblyCsrBuild::new_free(assembly).map_err(|_| b"fem3d.numerical-assembly-terminal".to_vec())?);
+                    self.stage = Fem3dNumericalStage::BuildCsr;
+                } else {
+                    self.source_factor = match self.request.source {
+                        Fem3dResultSource::FirstCase => if self.source_case == 0 { 1.0 } else { 0.0 },
+                        Fem3dResultSource::Case(index) => if self.source_case == index { 1.0 } else { 0.0 },
+                        Fem3dResultSource::Combination(index) => *doc.combinations.get(index).ok_or_else(|| b"fem3d.numerical-combination-missing".to_vec())?.terms.get(&doc.load_cases[self.source_case].id).unwrap_or(&0.0),
+                    };
+                    if !self.source_factor.is_finite() { return Err((b"fem3d.numerical-source-factor".to_vec()).into()); }
+                    if self.source_factor == 0.0 {
+                        self.source_case += 1;
+                    } else {
+                        self.load_cursor = 0;
+                        self.stage = Fem3dNumericalStage::ApplyLoad;
+                    }

```

### 80 step

```diff
-                let Some(case) = doc.load_cases.first() else { return Err(b"fem3d.numerical-load-case".to_vec()) };
+                let Some(case) = doc.load_cases.get(self.source_case) else { return Err((b"fem3d.numerical-load-case".to_vec()).into()) };

```

### 82 step

```diff
-                    let assembly = self.assembly.take().ok_or_else(|| b"fem3d.numerical-assembly-owner".to_vec())?;
-                    self.csr_build = Some(AssemblyCsrBuild::new_free(assembly).map_err(|_| b"fem3d.numerical-assembly-terminal".to_vec())?);
-                    self.stage = Fem3dNumericalStage::BuildCsr;
+                    self.source_case += 1;
+                    self.stage = Fem3dNumericalStage::ResolveSourceCase;

```

### 84 step

```diff
-                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
+                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;

```

### 86 step

```diff
-                    return Err(b"fem3d.numerical-load-kind".to_vec());
+                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());

```

### 88 step

```diff
-                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-load-node".to_vec()) };
+                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-load-node".to_vec()).into()) };

```

### 90 step

```diff
-                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
+                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;

```

### 92 step

```diff
-                    return Err(b"fem3d.numerical-load-kind".to_vec());
+                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());

```

### 94 step

```diff
-                let Some(element) = doc.elements.get(self.load_element_cursor) else { return Err(b"fem3d.numerical-member-load-element".to_vec()) };
+                let Some(element) = doc.elements.get(self.load_element_cursor) else { return Err((b"fem3d.numerical-member-load-element".to_vec()).into()) };

```

### 96 step

```diff
-                let Some(node) = doc.nodes.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-member-load-node".to_vec()) };
+                let Some(node) = doc.nodes.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-member-load-node".to_vec()).into()) };

```

### 98 step

```diff
-                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
+                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;

```

### 100 step

```diff
-                    return Err(b"fem3d.numerical-load-kind".to_vec());
+                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());

```

### 102 step

```diff
-                        return Err(b"fem3d.numerical-member-load-length".to_vec());
+                        return Err((b"fem3d.numerical-member-load-length".to_vec()).into());

```

### 104 step

```diff
-                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
+                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;

```

### 106 step

```diff
-                    return Err(b"fem3d.numerical-load-kind".to_vec());
+                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());

```

### 108 step

```diff
-                let Some(solid) = self.meshed_solids.get(self.load_solid_cursor) else { return Err(b"fem3d.numerical-area-load-solid".to_vec()) };
+                let Some(solid) = self.meshed_solids.get(self.load_solid_cursor) else { return Err((b"fem3d.numerical-area-load-solid".to_vec()).into()) };

```

### 110 step

```diff
-                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
+                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;

```

### 112 step

```diff
-                    return Err(b"fem3d.numerical-load-kind".to_vec());
+                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());

```

### 114 step

```diff
+                let source = doc.solids.get(solid.solid_index).ok_or_else(|| b"fem3d.numerical-area-load-source".to_vec())?;
+                let [p0, p1, p2] = [p0, p1, p2].map(|point| source.axis.to_world(point[0], point[1], source.base_z + source.height));

```

### 116 step

```diff
-                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-area-load-node".to_vec()) };
+                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-area-load-node".to_vec()).into()) };

```

### 118 step

```diff
-                let Some(node) = doc.nodes.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-self-weight-node".to_vec()) };
+                let Some(node) = doc.nodes.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-self-weight-node".to_vec()).into()) };

```

### 120 step

```diff
-                    self.apply_rhs_value(self.load_node_indices[self.load_vertex_cursor], Dof::Tz.index(), -material.rho * section.area * length * 9.80665 * 0.5)?;
+                    self.apply_rhs_value(self.load_node_indices[self.load_vertex_cursor], Dof::Tz.index(), -material.rho * section.area * length * 9.81 * 0.5)?;

```

### 122 step

```diff
-                    let assembly = self.assembly.take().ok_or_else(|| b"fem3d.numerical-assembly-owner".to_vec())?;
-                    self.csr_build = Some(AssemblyCsrBuild::new_free(assembly).map_err(|_| b"fem3d.numerical-assembly-terminal".to_vec())?);
-                    self.stage = Fem3dNumericalStage::BuildCsr;
+                    self.source_case += 1;
+                    self.stage = Fem3dNumericalStage::ResolveSourceCase;

```

### 124 step

```diff
-                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-self-weight-solid-node".to_vec()) };
+                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-self-weight-solid-node".to_vec()).into()) };

```

### 126 step

```diff
-                    self.apply_rhs_value(self.load_node_cursor, Dof::Tz.index(), -material.rho * volume * 9.80665 / 4.0)?;
+                    self.apply_rhs_value(self.load_node_cursor, Dof::Tz.index(), -material.rho * volume * 9.81 / 4.0)?;

```

### 128 step

```diff
-            Fem3dNumericalStage::BuildCsr => match self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr".to_vec())?.step_one() {
+            Fem3dNumericalStage::BuildCsr => match self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr".to_vec())?.step_one_funded(context.retained_grant()).and_then(|(complete, progress)| {
+                context.consume_retained(progress).map_err(|_| b"fem3d.numerical-csr-grant" as &'static [u8])?;
+                Ok(complete)
+            }) {

```

### 130 step

```diff
-                Err(detail) => return Err(detail.to_vec()),
+                Err(detail) => return Err((detail.to_vec()).into()),

```

### 132 step

```diff
-                let matrix = self.csr_build.as_mut().and_then(AssemblyCsrBuild::take_complete).ok_or_else(|| b"fem3d.numerical-csr-terminal".to_vec())?;
+                let build = self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr-terminal".to_vec())?;
+                if build.take_demands().is_none() { return Err((b"fem3d.numerical-csr-terminal".to_vec()).into()); }
+                let Some((matrix, progress)) = build.take_complete(context.retained_grant()) else { return Ok(false); };
+                context.consume_retained(progress).map_err(|_| b"fem3d.numerical-csr-transfer-grant".to_vec())?;

```

### 134 step

```diff
-                        return Err(b"fem3d.numerical-pcg-rhs".to_vec());
+                        return Err((b"fem3d.numerical-pcg-rhs".to_vec()).into());

```

### 136 step

```diff
-                        return Err(b"fem3d.numerical-pcg-false-terminal".to_vec());
+                        return Err((b"fem3d.numerical-pcg-false-terminal".to_vec()).into());

```

### 138 step

```diff
-                Err(detail) => return Err(detail.to_vec()),
+                Err(detail) => return Err((detail.to_vec()).into()),

```

### 140 step

```diff
-                let end = child_end(self.pcg.as_mut().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.step(context));
+                let end = child_end(self.pcg.as_mut().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.step(context))?;

```

### 142 step

```diff
-                    self.stage = Fem3dNumericalStage::BeginModal;
+                    self.element_cursor = 0;
+                    self.stage = Fem3dNumericalStage::RecoverStressElement;

```

### 144 step

```diff
-                        self.scalar.displacement[self.scalar_axis] = value.displacement;
-                        self.scalar.residual[self.scalar_axis] = value.residual;
-                        self.scalar.contour = self.scalar.contour.max(value.contour);
+                        if self.scalar_axis < 3 {
+                            self.scalar.displacement[self.scalar_axis] = value.displacement;
+                            self.scalar.residual[self.scalar_axis] = value.residual;
+                        }

```

### 146 step

```diff
-                    self.stage = if self.scalar_axis == 3 { Fem3dNumericalStage::PublishNodeScalar } else { Fem3dNumericalStage::ReadNodeScalar };
+                    self.stage = if self.scalar_axis == 6 { Fem3dNumericalStage::PublishNodeScalar } else { Fem3dNumericalStage::ReadNodeScalar };

```

### 148 step

```diff
-                    self.scalar.reaction[self.scalar_axis] = self.reaction_accumulator - self.full_rhs[row];
+                    let reaction = self.reaction_accumulator - self.full_rhs[row];
+                    self.full_reactions[self.node_cursor][self.scalar_axis] = reaction;
+                    if self.scalar_axis < 3 { self.scalar.reaction[self.scalar_axis] = reaction; }

```

### 150 step

```diff
-                    self.stage = if self.scalar_axis == 3 { Fem3dNumericalStage::PublishNodeScalar } else { Fem3dNumericalStage::ReadNodeScalar };
+                    self.stage = if self.scalar_axis == 6 { Fem3dNumericalStage::PublishNodeScalar } else { Fem3dNumericalStage::ReadNodeScalar };

```

### 152 step

```diff
+            Fem3dNumericalStage::RecoverStressElement => {
+                let build = self.csr_build.as_ref().ok_or_else(|| b"fem3d.numerical-stress-assembly".to_vec())?;
+                let Some(element) = build.model_element(self.element_cursor) else {
+                    self.node_cursor = 0;
+                    self.stage = Fem3dNumericalStage::PublishStressScalar;
+                    return Ok(false);
+                };
+                self.element_cursor += 1;
+                let Elements::Tet4(tetrahedron) = element else { return Ok(false) };
+                let mut indices = [0usize; 4];
+                let mut positions = [[0.0; 3]; 4];
+                let mut displacements = [0.0; 12];
+                for local in 0..4 {
+                    let id = element.mounted_node_id(local).ok_or_else(|| b"fem3d.numerical-stress-node".to_vec())?;
+                    let index = (0..self.analysis_node_ids.len()).find(|index| self.analysis_node_ids.get(*index).is_some_and(|candidate| candidate == id)).ok_or_else(|| b"fem3d.numerical-stress-node".to_vec())?;
+                    indices[local] = index;
+                    positions[local] = build.model_node(index).ok_or_else(|| b"fem3d.numerical-stress-position".to_vec())?.pos;
+                    for axis in 0..3 { displacements[local * 3 + axis] = self.equations[index][axis].and_then(|equation| self.pcg.as_ref()?.visual_scalar(equation)).map_or(0.0, |value| value.displacement); }
+                }
+                let stress = tetrahedron.mounted_recover_stress(&positions, &displacements).ok_or_else(|| b"fem3d.numerical-stress-geometry".to_vec())?.von_mises;
+                if !stress.is_finite() { return Err((b"fem3d.numerical-stress-value".to_vec()).into()); }
+                for index in indices { self.stress_sums[index] += stress; self.stress_counts[index] += 1; }
+            }
+            Fem3dNumericalStage::PublishStressScalar => {
+                if self.node_cursor == self.analysis_node_ids.len() {
+                    if self.request.buckling.is_some() { return Err((b"fem3d.numerical-buckling-owner-missing".to_vec()).into()); }
+                    self.stage = if self.request.mode.is_some() { Fem3dNumericalStage::BeginModal } else { Fem3dNumericalStage::PublishProgress };
+                } else {
+                    let mut scalar = solver.scalar(self.node_cursor).ok_or_else(|| b"fem3d.numerical-stress-scalar".to_vec())?;
+                    scalar.contour = self.stress_sums[self.node_cursor] / self.stress_counts[self.node_cursor].max(1) as f64;
+                    solver.publish_scalar(freshness, self.node_cursor, scalar).map_err(|_| b"fem3d.numerical-stress-publication".to_vec())?;
+                    self.node_cursor += 1;
+                }
+            }

```

### 154 step

```diff
-                self.modal_build = Some(ModalInputConstruction::new_mounted(matrix, std::mem::take(&mut self.modal_free_mass)));
+                self.modal_build = Some(SymmetricPencilConstruction::new_mounted(matrix, std::mem::take(&mut self.modal_free_mass)));

```

### 156 step

```diff
-            Fem3dNumericalStage::PrepareModal => match self.modal_build.as_mut().ok_or_else(|| b"fem3d.numerical-modal-build".to_vec())?.step_one() {
+            Fem3dNumericalStage::PrepareModal => match self.modal_build.as_mut().ok_or_else(|| b"fem3d.numerical-modal-build".to_vec())?.step_one(context) {

```

### 158 step

```diff
-                Err(detail) => return Err(detail.to_vec()),
+                Err(detail) => return Err((detail.to_vec()).into()),

```

### 160 step

```diff
-                let (stiffness, mass) = self.modal_build.as_mut().and_then(ModalInputConstruction::take_complete).ok_or_else(|| b"fem3d.numerical-modal-terminal".to_vec())?;
-                if stiffness.n == 0 || stiffness.n > 40 {
-                    return Err(b"fem3d.numerical-modal-order".to_vec());
+                let order = self.modal_build.as_ref().and_then(SymmetricPencilConstruction::completed_order).ok_or_else(|| b"fem3d.numerical-modal-terminal".to_vec())?;
+                let modes = self.request.mode.unwrap_or(0).checked_add(1).ok_or_else(|| b"fem3d.numerical-modal-order".to_vec())?;
+                if !SubspaceIterationJob::admits_shape(order, modes) {
+                    return Err((b"fem3d.numerical-modal-order".to_vec()).into());

```

### 162 step

```diff
+                let Some(((stiffness, mass), progress)) = self.modal_build.as_mut().and_then(|owner| owner.take_complete(context.retained_grant())) else { return Ok(false); };
+                context.consume_retained(progress).map_err(|_| b"fem3d.numerical-modal-transfer".to_vec())?;

```

### 164 step

```diff
-                let end = child_end(self.ldlt.as_mut().ok_or_else(|| b"fem3d.numerical-ldlt".to_vec())?.step(context));
+                let end = child_end(self.ldlt.as_mut().ok_or_else(|| b"fem3d.numerical-ldlt".to_vec())?.step(context))?;

```

### 166 step

```diff
-                let factor = self.ldlt.as_mut().and_then(LdltJob::take_factor).ok_or_else(|| b"fem3d.numerical-ldlt-factor".to_vec())?;
+                let Some((factor, progress)) = self.ldlt.as_mut().and_then(|owner| owner.take_factor(context.retained_grant())) else { return Ok(false); };
+                context.consume_retained(progress).map_err(|_| b"fem3d.numerical-factor-transfer".to_vec())?;

```

### 168 step

```diff
-                self.subspace = Some(SubspaceIterationJob::new(operation, factor, mass, order, 1, 30));
+                self.subspace = Some(SubspaceIterationJob::new(operation, factor, mass, order, self.request.mode.unwrap_or(0) + 1, 30));

```

### 170 step

```diff
-                let end = child_end(self.subspace.as_mut().ok_or_else(|| b"fem3d.numerical-subspace".to_vec())?.step(context));
+                let end = child_end(self.subspace.as_mut().ok_or_else(|| b"fem3d.numerical-subspace".to_vec())?.step(context))?;

```

### 172 step

```diff
-                        let (component, eigenvalue) = self.subspace.as_ref().and_then(|subspace| subspace.visual_mode_scalar(0, equation)).ok_or_else(|| b"fem3d.numerical-mode-scalar".to_vec())?;
+                        let (component, eigenvalue) = self.subspace.as_ref().and_then(|subspace| subspace.visual_mode_scalar(self.request.mode.unwrap_or(0), equation)).ok_or_else(|| b"fem3d.numerical-mode-scalar".to_vec())?;

```

### 174 step

```diff
-                let (_, _, mode_residual, modes_converged) = self.subspace.as_ref().ok_or_else(|| b"fem3d.numerical-subspace".to_vec())?.visual_progress();
+                let (_, _, mode_residual, modes_converged) = self.subspace.as_ref().map_or((0, 0, 0.0, true), SubspaceIterationJob::visual_progress);

```

### 176 step

```diff
-                    return Err(b"fem3d.numerical-progress-publication".to_vec());
+                    return Err((b"fem3d.numerical-progress-publication".to_vec()).into());

```

### 178 step

```diff
-    fn close_interactive(step: semio_framework_job::InteractiveJobCloseStep) -> (bool, usize, usize) {
-        match step {
-            semio_framework_job::InteractiveJobCloseStep::Pending { progress } => (false, progress.copied_items, progress.released_bytes),
-            semio_framework_job::InteractiveJobCloseStep::Blocked | semio_framework_job::InteractiveJobCloseStep::Refused { .. } => (false, 0, 0),
-            semio_framework_job::InteractiveJobCloseStep::Complete { progress } => (true, progress.copied_items, progress.released_bytes),
-        }
-    }
+

```

### 180 terminal_is_empty

```diff
-    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> (bool, usize, usize) {
+    fn close_rung((terminal, copied_items, released_bytes): (bool, usize, usize)) -> (bool, RetainedCloneProgress) {
+        (terminal, RetainedCloneProgress { copied_items, released_bytes, ..Default::default() })
+    }
+
+    pub fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), semio_framework_job::OriginalTurnFailure> {

```

### 182 close_step

```diff
-        if self.terminal_is_empty() { return (true, 0, 0); }
-        if maximum_bytes == 0 { return (false, 0, 0); }
+        if self.terminal_is_empty() { return Ok(Self::close_rung((true, 0, 0))); }
+        if maximum_bytes == 0 { return Ok(Self::close_rung((false, 0, 0))); }

```

### 184 close_step

```diff
-            return (false, step.1, step.2);
+            return Ok(Self::close_rung((false, step.1, step.2)));

```

### 186 close_step

```diff
-            return (false, 1, 0);
+            return Ok(Self::close_rung((false, 1, 0)));

```

### 188 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 190 close_step

```diff
-                let step = Self::close_interactive(InteractiveJob::close_step(job, child));
-                if step.0 {
-                    self.mesh = None;
+                if !job.terminal_is_empty() {
+                    let step = InteractiveJob::close_step(job, cx)?;
+                    return Ok((false, step.progress()));

```

### 192 close_step

```diff
+                self.mesh = None;
+                let step = (true, 1, 0);

```

### 194 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 196 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 198 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 200 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 202 close_step

```diff
-                let step = Self::close_interactive(job.close_step(child));
-                if step.0 {
-                    self.pcg_build = None;
+                if !job.terminal_is_empty() {
+                    let step = job.close_step(cx)?;
+                    return Ok((false, step.progress()));

```

### 204 close_step

```diff
+                self.pcg_build = None;
+                let step = (true, 1, 0);

```

### 206 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 208 close_step

```diff
-                let step = Self::close_interactive(InteractiveJob::close_step(job, child));
-                if step.0 {
-                    self.pcg = None;
+                if !job.terminal_is_empty() {
+                    let step = InteractiveJob::close_step(job, cx)?;
+                    return Ok((false, step.progress()));

```

### 210 close_step

```diff
+                self.pcg = None;
+                let step = (true, 1, 0);

```

### 212 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 214 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 216 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 218 close_step

```diff
-                let step = Self::close_interactive(InteractiveJob::close_step(job, child));
-                if step.0 {
-                    self.ldlt = None;
+                if !job.terminal_is_empty() {
+                    let step = InteractiveJob::close_step(job, cx)?;
+                    return Ok((false, step.progress()));

```

### 220 close_step

```diff
+                self.ldlt = None;
+                let step = (true, 1, 0);

```

### 222 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 224 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 226 close_step

```diff
-                let step = Self::close_interactive(InteractiveJob::close_step(job, child));
-                if step.0 {
-                    self.subspace = None;
+                if !job.terminal_is_empty() {
+                    let step = InteractiveJob::close_step(job, cx)?;
+                    return Ok((false, step.progress()));

```

### 228 close_step

```diff
+                self.subspace = None;
+                let step = (true, 1, 0);

```

### 230 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 232 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 234 close_step

```diff
-                                return (false, 0, 0);
+                                return Ok(Self::close_rung((false, 0, 0)));

```

### 236 close_step

```diff
-                            return (false, 1, bytes);
+                            return Ok(Self::close_rung((false, 1, bytes)));

```

### 238 close_step

```diff
-                        return (false, 1, 0);
+                        return Ok(Self::close_rung((false, 1, 0)));

```

### 240 close_step

```diff
-                        return (false, 1, 0);
+                        return Ok(Self::close_rung((false, 1, 0)));

```

### 242 close_step

```diff
-                        return (false, 1, 0);
+                        return Ok(Self::close_rung((false, 1, 0)));

```

### 244 close_step

```diff
-                        return (false, 1, 0);
+                        return Ok(Self::close_rung((false, 1, 0)));

```

### 246 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 248 close_step

```diff
-                            return (false, 0, 0);
+                            return Ok(Self::close_rung((false, 0, 0)));

```

### 250 close_step

```diff
-                        return (false, 1, bytes);
+                        return Ok(Self::close_rung((false, 1, bytes)));

```

### 252 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 254 close_step

```diff
-                            return (false, 0, 0);
+                            return Ok(Self::close_rung((false, 0, 0)));

```

### 256 close_step

```diff
-                        return (false, 1, bytes);
+                        return Ok(Self::close_rung((false, 1, bytes)));

```

### 258 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 260 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 262 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 264 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 266 close_step

```diff
-                            return (false, 0, 0);
+                            return Ok(Self::close_rung((false, 0, 0)));

```

### 268 close_step

```diff
-                        return (false, 1, bytes);
+                        return Ok(Self::close_rung((false, 1, bytes)));

```

### 270 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 272 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 274 close_step

```diff
-                    return (false, 1, 0);
+                    return Ok(Self::close_rung((false, 1, 0)));

```

### 276 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 278 close_step

```diff
-                        return (false, 0, 0);
+                        return Ok(Self::close_rung((false, 0, 0)));

```

### 280 close_step

```diff
-                    return (false, 1, released);
+                    return Ok(Self::close_rung((false, 1, released)));

```

### 282 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 284 close_step

```diff
-                        return (false, 0, 0);
+                        return Ok(Self::close_rung((false, 0, 0)));

```

### 286 close_step

```diff
-                    return (false, 1, released);
+                    return Ok(Self::close_rung((false, 1, released)));

```

### 288 close_step

```diff
-                    return (false, 0, 0);
+                    return Ok(Self::close_rung((false, 0, 0)));

```

### 290 close_step

```diff
-            _ => return (self.terminal_is_empty(), 0, 0),
+            _ => return Ok(Self::close_rung((self.terminal_is_empty(), 0, 0))),

```

### 292 close_step

```diff
-            return (false, step.1, step.2);
+            return Ok(Self::close_rung((false, step.1, step.2)));

```

### 294 close_step

```diff
-        step
+        Ok(Self::close_rung(step))

```

### 296 drop

```diff
-    pub fn backing_usage(&self) -> (usize, usize) {
+    fn backing_usage(&self) -> (usize, usize) {

```

### 298 page_kind

```diff
-    pub fn new(freshness: Fem3dVisualFreshness, credit: Fem3dPageCredit) -> Self {
+    fn new(freshness: Fem3dVisualFreshness, credit: Fem3dPageCredit) -> Self {

```

### 300 field_page

```diff
-    pub fn step_one(&mut self, doc: &Fem3dSnapshot, solver: &Fem3dSolverView, backing: &mut Fem3dBackingCredit, freshness: Fem3dVisualFreshness) -> Result<bool, Vec<u8>> {
+    fn step_one(&mut self, doc: &Fem3dSnapshot, solver: &Fem3dSolverView, backing: &mut Fem3dBackingCredit, freshness: Fem3dVisualFreshness) -> Result<bool, Vec<u8>> {

```

### 302 step_one

```diff
-                self.push_item(1, [Some(&solid.id), Some(if kind == 1 { "tetrahedron" } else { "hexahedron" }), None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers(solid.axis.to_world(self.solid_sum[0] / divisor, self.solid_sum[1] / divisor, solid.base_z + solid.height * 0.5), [1.0, 1.0, solid.height]), number_len: 14, indexes: [kind, 0, 0, 0, 0, 0, 0, 0], index_len: 1, flags: 1 })?;
+                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("solid", &solid.id).map_err(|_| b"fem3d.visual-identity".to_vec())?;
+                self.push_item(1, [Some(&identity), Some(if kind == 1 { "tetrahedron" } else { "hexahedron" }), None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers(solid.axis.to_world(self.solid_sum[0] / divisor, self.solid_sum[1] / divisor, solid.base_z + solid.height * 0.5), [1.0, 1.0, solid.height]), number_len: 14, indexes: [kind, 0, 0, 0, 0, 0, 0, 0], index_len: 1, flags: 1 })?;

```

### 304 step_one

```diff
-                        self.push_item(2 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(&node.id), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([node.x, node.y, node.z], [0.05; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 2 })?;
+                        let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("node", &node.id).map_err(|_| b"fem3d.visual-identity".to_vec())?;
+                        self.push_item(2 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([node.x, node.y, node.z], [0.05; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 2 })?;

```

### 306 step_one

```diff
-                self.push_item(4 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(element_id(element)), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers(midpoint, [0.05, 0.05, length]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 3 })?;
+                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("element", element_id(element)).map_err(|_| b"fem3d.visual-identity".to_vec())?;
+                self.push_item(4 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers(midpoint, [0.05, 0.05, length]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 3 })?;

```

### 308 step_one

```diff
-                self.push_item(6 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(element_id(&doc.elements[index])), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0; 3], [0.03; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 4 })?;
+                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("element", element_id(&doc.elements[index])).map_err(|_| b"fem3d.visual-identity".to_vec())?;
+                self.push_item(6 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0; 3], [0.03; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 4 })?;

```

### 310 step_one

```diff
-                self.push_item(8, [Some(load_id(load)), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0, 0.0, 1.0], [0.03, 0.03, 0.3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 5 })?;
+                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_load_target(&case.id, load_id(load)).map_err(|_| b"fem3d.visual-identity".to_vec())?;
+                self.push_item(8, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0, 0.0, 1.0], [0.03, 0.03, 0.3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 5 })?;

```

### 312 step_one

```diff
-                self.push_item(9, [Some(&support.id), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0; 3], [0.1; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 6 })?;
+                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("support", &support.id).map_err(|_| b"fem3d.visual-identity".to_vec())?;
+                self.push_item(9, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0; 3], [0.1; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 6 })?;

```

### 314 step_one

```diff
-                self.push_item(Self::field_page(self.stage, self.cursor), [Some(&node.id), None, None, None], Fem3dVisualItemPayload { numbers: Self::field_numbers(field, [node.x, node.y, node.z], vector, scalar), number_len: 14, indexes: [0; 8], index_len: 0, flags: flag })?;
+                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("node", &node.id).map_err(|_| b"fem3d.visual-identity".to_vec())?;
+                self.push_item(Self::field_page(self.stage, self.cursor), [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::field_numbers(field, [node.x, node.y, node.z], vector, scalar), number_len: 14, indexes: [0; 8], index_len: 0, flags: flag })?;

```

### 316 step_one

```diff
-    pub fn take_complete(&mut self) -> Option<Fem3dPageVisualLease> {
+    fn take_complete(&mut self) -> Option<Fem3dPageVisualLease> {

```

### 318 take_complete

```diff
-    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
+    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {

```

### 320 close_step

```diff
-    pub fn terminal_is_empty(&self) -> bool {
+    fn terminal_is_empty(&self) -> bool {

```

### 322 drop

```diff
+
+impl Fem3dNumericalStage {
+    pub fn progress_stage(self) -> &'static str {
+        match self {
+            Self::SolidMeshBegin | Self::SolidMesh => "fem3d.meshing",
+            Self::MountAssembly | Self::PrepareAssembly | Self::Assembly | Self::BuildCsr => "fem3d.assembly",
+            Self::BeginPcg | Self::PreparePcg | Self::Pcg => "fem3d.statics",
+            Self::BeginModal | Self::PrepareModal | Self::BeginLdlt | Self::Ldlt | Self::BeginSubspace | Self::Subspace | Self::ReadModeScalar | Self::PublishModeScalar => "fem3d.modes",
+            Self::ReadNodeScalar | Self::RecoverReaction | Self::PublishNodeScalar | Self::RecoverStressElement | Self::PublishStressScalar | Self::PublishProgress => "fem3d.validation",
+            Self::Complete => "fem3d.validated",
+            _ => "fem3d.preparation",
+        }
+    }
+}

```

## 14 ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs

### 1 

```diff
-    allocate_operation_id, default_now_us, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RevisionId,
+    allocate_operation_id, default_now_us, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RevisionId,

```

### 3 

```diff
+
+#[path = "🪪️input-identity/🦀️.rs"]
+pub mod input_identity;
+use input_identity::EnergyInputIdentity;
+#[path="♻️close/🦀️.rs"]
+mod original_terminal_retirement;
+use original_terminal_retirement::EnergyAuthorityRetirement;
+
+#[path="🧺️batch/🦀️.rs"]
+pub mod batch;
+pub use batch::EnergyBatchRun;
+pub use semio_framework_job::Operation;
+#[cfg(any(test,feature="testing"))]
+#[path="🧪️testing/🦀️.rs"]
+pub mod testing;

```

### 5 terminal_is_empty

```diff
+    CheckPvThermal,

```

### 7 terminal_is_empty

```diff
-const P7C1_VALIDATION_STAGES: [ValidationStage; 16] = [
+const P7C1_VALIDATION_STAGES: [ValidationStage; 17] = [

```

### 9 terminal_is_empty

```diff
+    ValidationStage::CheckPvThermal,

```

### 11 default

```diff
+    ChronologyRejected,
+    RunHourMissing,

```

### 13 default

```diff
+    pub input_identity: EnergyInputIdentity,

```

### 15 default

```diff
+    pub input_identity_bytes: usize,
+    pub input_identity_ready: bool,

```

### 17 default

```diff
+    precompute_close: semio_framework_value::retained_clone::RetainedCloneClose,

```

### 19 default

```diff
+    closing_tail:Option<semio_framework_value::retirement::controlled::ControlledRetirement<EnergyAuthorityRetirement>>,
+    tail_moved:bool,

```

### 21 admit

```diff
+                input_identity: EnergyInputIdentity::default(),

```

### 23 admit

```diff
+                precompute_close: Default::default(),

```

### 25 admit

```diff
+                closing_tail:None,
+                tail_moved:false,

```

### 27 cursor

```diff
+            input_identity_bytes: self.input_identity.processed_bytes(),
+            input_identity_ready: self.input_identity.digest().is_some(),

```

### 29 set_stage

```diff
-    fn run_start_record(&self) -> usize {
-        if self.weather.len() < 8_760 {
-            return 0;
-        }
-        let period = SimulationKernel::run_period(&self.config);
-        (calendar::SimDate::new(period.year, period.start_month, period.start_day).day_of_year() as usize - 1) * 24
+    fn weather_calendar(&self)->crate::site::WeatherCalendar{self.config.weather.as_ref().map_or(crate::site::WeatherCalendar::ActualDates,|weather|weather.calendar)}
+
+    /// 🔎️ Exact authored calendar matching over the admitted timestamp sequence.
+    fn weather_index(&self,date:crate::calendar::SimDate,hour:u8)->Option<usize>{crate::site::chronology::record_index(self.weather_calendar(),self.weather.len(),|i|self.weather.get_index(i).copied(),date,hour)}
+
+    /// 🌦️ The run starts at its authored first date, independent of weather-file length.
+    fn run_start_record(&self)->Option<usize>{let period=SimulationKernel::run_period(&self.config);self.weather_index(calendar::SimDate::new(period.year,period.start_month,period.start_day),0)}
+
+    /// 🌗️ Continuous simulation keeps real neighbour timestamps; warmup explicitly repeats its selected day.
+    fn hour_weather(&self,index:usize,repeat_day:bool)->Option<HourWeather>{let[previous,current,next]=crate::site::chronology::neighbours(self.weather.len(),|i|self.weather.get_index(i).copied(),index,if repeat_day{crate::site::chronology::NeighbourPolicy::RepeatDay}else{crate::site::chronology::NeighbourPolicy::Continuous})?;Some(HourWeather{previous,current,next})}
+
+    fn weather_record(&self,index:usize)->WeatherRecord{
+        if let Some(weather)=&self.config.weather{return weather.records[index];}
+        let mut record=match self.config.environment{SimulationEnvironment::HeatingDesignDay=>design_day_hour((index%24)as u32,-10.0),SimulationEnvironment::CoolingDesignDay=>design_day_hour((index%24)as u32,35.0),SimulationEnvironment::WeatherRunPeriod|SimulationEnvironment::CustomDesignPeriod=>synthetic_hour(index as u32)};
+        let period=SimulationKernel::run_period(&self.config);let date=crate::site::chronology::date_after_days(calendar::SimDate::new(period.year,period.start_month,period.start_day),(index/24)as u32).expect("validated bounded synthetic calendar");record.year=date.year;record.month=date.month;record.day=date.day;record

```

### 31 run_start_record

```diff
-    fn hour_weather(&self, index: usize, first_of_period: bool) -> HourWeather {
-        let len = self.weather.len().max(1);
-        let record = |i: usize| *self.weather.get_index(i % len).expect("admitted weather record");
-        let hour = index % 24;
-        let day_start = index - hour;
-        let previous = if hour == 0 && (first_of_period || index == 0) { record(day_start + 23) } else { record(index + len - 1) };
-        let next = if hour == 23 { record(day_start) } else { record(index + 1) };
-        HourWeather { previous, current: record(index), next }
-    }
-
-    fn weather_record(&self, index: usize) -> WeatherRecord {
-        if let Some(epw) = &self.config.weather {
-            if let Some(record) = epw.records.get(index) {
-                return *record;
-            }
-        }
-        match self.config.environment {
-            SimulationEnvironment::HeatingDesignDay => site::design_day_hour(index as u32, -10.0),
-            SimulationEnvironment::CoolingDesignDay => site::design_day_hour(index as u32, 35.0),
-            SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => site::synthetic_hour(index as u32),
-        }
+    pub fn step_input_identity(&mut self, context: &mut StepContext<'_>) -> Result<bool, EnergyWireRejection> {
+        if self.input_identity.digest().is_some() { return Ok(true); }
+        let progress = match self.input_identity.advance(&self.model, &self.config, context.retained_grant()) {
+            Ok(progress) => progress,
+            Err(error) => { let _ = context.consume_retained(error.retained_progress()); return Err(EnergyWireRejection::Backing); }
+        };
+        context.consume_retained(progress).map_err(|_| EnergyWireRejection::Backing)?;
+        Ok(self.input_identity.digest().is_some())

```

### 33 start_wire

```diff
+        if kind == EnergyWireKind::Checkpoint && self.input_identity.digest().is_none() { return Err(EnergyWireRejection::Identity); }

```

### 35 wire_fragment

```diff
+            }
+            4 if kind == EnergyWireKind::Checkpoint => {
+                bytes[..32].copy_from_slice(self.input_identity.digest()?);
+                32

```

### 37 step_validation

```diff
-                        work.stage = ValidationStage::Complete;
+                        advance_validation(work, ValidationStage::CheckPvThermal);

```

### 39 step_validation

```diff
+            }
+            ValidationStage::CheckPvThermal => {
+                if let Some(pv) = self.model.pv_systems.get(work.cursor) {
+                    if !pv.thermal_parameters_valid() { work.fatal_code = 15; }
+                    work.cursor += 1;
+                } else { advance_validation(work, ValidationStage::Complete); }

```

### 41 run

```diff
+                if self.input_identity.digest().is_none() { return match self.step_input_identity(context) { Ok(_) => EnergyRun::Yield, Err(_) => self.begin_fault() }; }

```

### 43 run

```diff
+                    let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<WeatherRecord>()*4,..Default::default()};
+                    if !receipt.fits(context.retained_grant()){return EnergyRun::Yield;}

```

### 45 run

```diff
+                    let first=self.weather.get_index(0).copied().unwrap_or(record);let previous=self.weather_cursor.checked_sub(1).and_then(|i|self.weather.get_index(i)).copied();
+                    if !crate::site::chronology::admit_row(self.weather_calendar(),first,previous,record){self.weather_fault=Some(WeatherFault::ChronologyRejected);return self.begin_fault();}

```

### 47 run

```diff
+                    if context.consume_retained(receipt).is_err(){return self.begin_fault();}

```

### 49 run

```diff
+                let period=SimulationKernel::run_period(&self.config);let start=self.run_start_record();let end=crate::site::chronology::date_after_days(crate::calendar::SimDate::new(period.year,period.start_month,period.start_day),period.total_hours()/24-1).and_then(|date|self.weather_index(date,23));
+                if start.zip(end).is_none_or(|(start,end)|end.checked_sub(start).and_then(|n|n.checked_add(1))!=Some(period.total_hours()as usize)){self.weather_fault=Some(WeatherFault::RunHourMissing);return self.begin_fault();}

```

### 51 run

```diff
-                    let weather = *self.weather.get_index(0).expect("admitted weather record");
+                    let weather = *self.weather.get_index(self.run_start_record().expect("admitted run start")).expect("admitted weather record");

```

### 53 run

```diff
-                    let start = self.run_start_record();
-                    let weather = self.hour_weather(start + self.warmup_hour as usize % 24, true);
+                    let Some(start)=self.run_start_record()else{return self.begin_fault();};
+                    let Some(weather)=self.hour_weather(start+self.warmup_hour as usize%24,true)else{return self.begin_fault();};

```

### 55 run

```diff
-                self.timestep_work.as_mut().expect("warmup timestep work exists").step(&self.model, &self.config, pre, self.state.as_mut().expect("state initialized before warmup"));
+                let turn=self.timestep_work.as_mut().expect("warmup timestep work exists").step_funded(&self.model,&self.config,pre,self.state.as_mut().expect("state initialized before warmup"),context.retained_grant());
+                if context.consume_retained(turn.progress).is_err()||turn.fault.is_some(){return self.begin_fault();}

```

### 57 run

```diff
-                    let record = if self.weather.len() >= 8_760 { (date.day_of_year() as usize - 1) * 24 + hour as usize } else { self.hour_index as usize };
-                    let mut weather = self.hour_weather(record, self.hour_index == 0);
-                    for record in [&mut weather.previous, &mut weather.current, &mut weather.next] {
-                        record.year = date.year;
-                        record.month = date.month;
-                        record.day = date.day;
-                    }
-                    weather.current.hour = hour;
+                    let Some(record)=self.weather_index(date,hour)else{self.weather_fault=Some(WeatherFault::RunHourMissing);return self.begin_fault();};
+                    let Some(weather)=self.hour_weather(record,false)else{return self.begin_fault();};

```

### 59 run

```diff
-                self.timestep_work.as_mut().expect("run timestep work exists").step(&self.model, &self.config, pre, self.state.as_mut().expect("state initialized before run"));
+                let turn=self.timestep_work.as_mut().expect("run timestep work exists").step_funded(&self.model,&self.config,pre,self.state.as_mut().expect("state initialized before run"),context.retained_grant());
+                if context.consume_retained(turn.progress).is_err()||turn.fault.is_some(){return self.begin_fault();}

```

### 61 commit_payloads

```diff
+    fn precompute_close_demands(&self, body: usize) -> Option<Result<RetirementDemand, ValueError>> {
+        if !self.precompute_close.is_empty() || self.precompute.is_some() {
+            return Some(self.precompute_close.next_owner_demands::<PrecomputeBuilder>(self.precompute.is_some(), &None, body));
+        }
+        self.pre.as_ref().map(|_| self.precompute_close.next_owner_demands::<PrecomputedModel>(true, &None, body))
+    }
+

```

### 63 close_demands

```diff
+        if authority_inputs_are_retired(self){return self.retained_tail_demands(usize::MAX)}

```

### 65 close_demands

```diff
+        if !self.input_identity.terminal_is_empty() { return Ok(self.input_identity.close_demand()); }

```

### 67 close_demands

```diff
+        if let Some(demand) = self.precompute_close_demands(0) { return demand; }

```

### 69 close_step

```diff
+        if authority_inputs_are_retired(self){return self.retire_retained_tail(grant)}

```

### 71 close_step

```diff
+        }
+        if !self.input_identity.terminal_is_empty() {
+            return self.input_identity.close_step(grant).map(|progress|InteractiveJobCloseStep::Pending{progress});

```

### 73 close_step

```diff
-        if let Some(builder) = self.precompute.as_mut() {
-            if !builder.close_step(maximum_items) {
-                return Ok(energy_one());
-            }
-            self.precompute = None;
-            return Ok(energy_one());
-        }
-        if let Some(pre) = self.pre.as_mut() {
-            if !pre.close_step(maximum_items) {
-                return Ok(energy_one());
-            }
-            self.pre = None;
-            return Ok(energy_one());
-        }
+

```

### 75 begin_close

```diff
-        EnergyJobAuthority::begin_close(self);
+        if let Some(authority)=self.authority.as_deref_mut(){authority.begin_close();}

```

### 77 close_step

```diff
-        Ok(EnergyJobAuthority::close_step(self, context.retained_grant())?)
+        self.begin_close();
+        if self.authority.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
+        let demand=self.original_close_demands()?;
+        let step=semio_framework_job::close_original_retained_step(context,demand,|grant|{
+            if self.authority.as_deref().is_some_and(authority_is_terminal_empty){
+                drop(self.authority.take());
+                release_energy_abandonment_slot(self.abandonment_slot);
+                return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}));
+            }
+            let step=self.authority.as_deref_mut().expect("original Energy authority remains retained").close_step(grant)?;
+            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress()))
+        })?;
+        Ok(match step{InteractiveJobCloseStep::Pending{progress}if self.authority.is_none()=>InteractiveJobCloseStep::Complete{progress},step=>step})

```

### 79 next_close_copy_byte_demand

```diff
-        Ok(self.close_demands()?.copy_bytes)
+        Ok(self.original_close_demands()?.copy_bytes)

```

### 81 next_close_capacity_byte_demand

```diff
-        Ok(self.close_demands()?.capacity_bytes)
+        Ok(self.original_close_demands()?.capacity_bytes)

```

### 83 next_close_release_byte_demand

```diff
-        Ok(self.close_demands()?.release_bytes)
+        Ok(self.original_close_demands()?.release_bytes)

```

### 85 next_close_depth_demand

```diff
-        Ok(self.close_demands()?.depth)
+        Ok(self.original_close_demands()?.depth)

```

### 87 terminal_is_empty

```diff
-        EnergyJobAuthority::terminal_is_empty(self)
+        self.authority.is_none()
+    }
+}
+impl EnergyJob{
+    fn original_close_demands(&self)->Result<RetirementDemand,ValueError>{
+        let Some(authority)=self.authority.as_deref()else{return Ok(Default::default())};
+        if authority.closing&&authority_is_terminal_empty(authority){return Ok(RetirementDemand{release_bytes:std::mem::size_of::<EnergyJobAuthority>(),depth:1,..Default::default()})}
+        authority.close_demands()

```

### 89 close_aggregate_work_step

```diff
-fn authority_is_terminal_empty(authority: &EnergyJobAuthority) -> bool {
+fn authority_is_terminal_empty(authority:&EnergyJobAuthority)->bool{authority_inputs_are_retired(authority)&&authority.tail_moved&&authority.closing_tail.is_none()}
+
+fn authority_inputs_are_retired(authority: &EnergyJobAuthority) -> bool {

```

### 91 authority_is_terminal_empty

```diff
+        && authority.input_identity.terminal_is_empty()

```

### 93 authority_is_terminal_empty

```diff
+        && authority.precompute_close.is_empty()

```

### 95 job

```diff
-    /// ⚡️ Run full building energy simulation.
-    pub fn run(model: Model, config: SimulationConfig) -> Result<Results, Error> {
-        enum Seen {
-            Yield,
-            Preview,
-            Checkpoint,
-            Complete,
-            Fault,
-            Cancelled,
-        }
-        let mut job = Self::job(model, config).map_err(|_| Error::severe("energy numerical admission rejected"))?;
-        let operation = job.operation.operation;
-        let generation = job.operation.generation;
-        let mut sequence = 0;
-        let seen = loop {
-            let mut receipt = RetainedCloneProgress::default();
-            let mut context = StepContext::new(operation, generation, StepBudget::new(32, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut receipt);
-            let seen = match InteractiveJob::step(&mut job, &mut context) {
-                Ok(Some(JobOutcomeBorrow::PreviewReady { .. })) => Seen::Preview,
-                Ok(Some(JobOutcomeBorrow::CheckpointReady { .. })) => Seen::Checkpoint,
-                Ok(Some(JobOutcomeBorrow::Complete { .. })) => Seen::Complete,
-                Ok(Some(JobOutcomeBorrow::Fault { .. })) | Err(_) => Seen::Fault,
-                Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => Seen::Cancelled,
-                Ok(Some(JobOutcomeBorrow::Yield { .. }) | None) => Seen::Yield,
-            };
-            match seen {
-                Seen::Preview => {
-                    if let Some(mut packet) = job.take_preview_packet(generation).map_err(|_| Error::severe("energy preview generation rejected"))? {
-                        close_retained_packet(&mut packet);
-                    }
-                }
-                Seen::Checkpoint => {
-                    if let Some(mut lease) = job.take_checkpoint_packet(generation).map_err(|_| Error::severe("energy checkpoint generation rejected"))? {
-                        close_retained_packet(lease.packet_mut());
-                        job.ack_checkpoint_packet(lease).map_err(|_| Error::severe("energy checkpoint ACK rejected"))?;
-                    }
-                }
-                Seen::Yield => {}
-                terminal => break terminal,
-            }
-        };
-        let result = match seen {
-            Seen::Complete => job.take_results().ok_or_else(|| Error::severe("energy job completed without results")),
-            Seen::Fault => Err(Error::severe("energy simulation fault")),
-            Seen::Cancelled => Err(Error::severe("energy simulation cancelled")),
-            Seen::Yield | Seen::Preview | Seen::Checkpoint => Err(Error::severe("energy batch adapter stopped before a terminal outcome")),
-        };
-        close_owned_job(&mut job);
-        result
-    }
+

```

### 97 run

```diff
-/// 🚪️ Runs the job's own close ladder to completion with grants it quotes itself.
-pub fn close_owned_job(job: &mut EnergyJob) {
-    InteractiveJob::begin_close(job);
-    let mut idle = 0usize;
-    loop {
-        if InteractiveJob::terminal_is_empty(job) {
-            return;
-        }
-        let grant = energy_self_funded(job.close_demands().expect("a locally owned energy job quotes its close demand"));
-        let step = EnergyJobAuthority::close_step(job, grant).expect("a locally owned energy job close is never refused");
-        assert!(step.progress().fits(grant), "an energy close receipt must fit its own quoted grant");
-        assert!(!matches!(step, InteractiveJobCloseStep::Blocked), "a locally owned energy job close has no external owner");
-        idle = if step.progress() == RetainedCloneProgress::default() && !matches!(step, InteractiveJobCloseStep::Complete { .. }) { idle + 1 } else { 0 };
-        assert!(idle < 1_000, "the energy close ladder stalled");
-    }
-}
-

```

### 99 close_owned_job

```diff
+
+#[cfg(test)]
+#[global_allocator]
+static ENERGY_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

```

## 20 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py

### 1 create_pv_system

```diff
+    if not math.isfinite(payload["temperatureCoefficient"]) or not math.isfinite(payload["faimanHeatLossConstantWM2k"]) or payload["faimanHeatLossConstantWM2k"] <= 0 or not math.isfinite(payload["faimanHeatLossWindWSM3k"]) or payload["faimanHeatLossWindWSM3k"] < 0:
+        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])

```

### 3 create_pv_system

```diff
-    after["model"]["pv_systems"].insert(payload["index"], {"id": payload["id"], "dc_capacity_w": payload["dcCapacityW"], "area_m2": payload["areaM2"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"], "module_efficiency": payload["moduleEfficiency"], "inverter_efficiency": payload["inverterEfficiency"]})
+    after["model"]["pv_systems"].insert(payload["index"], {"id": payload["id"], "dc_capacity_w": payload["dcCapacityW"], "area_m2": payload["areaM2"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"], "module_efficiency": payload["moduleEfficiency"], "inverter_efficiency": payload["inverterEfficiency"],"temperature_coefficient":payload["temperatureCoefficient"],"faiman_heat_loss_constant_w_m2k":payload["faimanHeatLossConstantWM2k"],"faiman_heat_loss_wind_w_s_m3k":payload["faimanHeatLossWindWSM3k"],"mounting":payload["mounting"]})

```

### 5 _invert_delete_pv_system

```diff
-    return [("create-pv-system", {"index": index, "id": item["id"], "dcCapacityW": item["dc_capacity_w"], "areaM2": item["area_m2"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"], "moduleEfficiency": item["module_efficiency"], "inverterEfficiency": item["inverter_efficiency"]})]
+    return [("create-pv-system", {"index": index, "id": item["id"], "dcCapacityW": item["dc_capacity_w"], "areaM2": item["area_m2"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"], "moduleEfficiency": item["module_efficiency"], "inverterEfficiency": item["inverter_efficiency"],"temperatureCoefficient":item["temperature_coefficient"],"faimanHeatLossConstantWM2k":item["faiman_heat_loss_constant_w_m2k"],"faimanHeatLossWindWSM3k":item["faiman_heat_loss_wind_w_s_m3k"],"mounting":item["mounting"]})]

```

## 21 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs

### 1 

```diff
-//! fuel is one computed timestep (warmup or run), so a single step of a paused run advances exactly one.
+//! semantic progress is one computed timestep (warmup or run); physical work remains charged to the original caller.

```

### 3 

```diff
-use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};

```

### 5 

```diff
-use semio_framework_job::{Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, OperationId, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPayload, StepBudget, StepContext};
+use semio_framework_job::{RetainedPayloadBuilder, JobPayloadStagedSource, JobPayloadAdmissionFault, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPayload, StepContext};

```

### 7 

```diff
-    ToolRunTickWriter, ToolRunTraceKind, ToolRunVerdict,
+    ToolRunTickWriter, ToolRunTickWireCursor, ToolRunTickPayloadSource, ToolRunTraceKind, ToolRunVerdict,

```

### 9 

```diff
+use std::mem::{size_of,MaybeUninit};
+use semio_framework_value::retirement::controlled::ControlledRetirement;

```

### 11 of

```diff
+    InputIdentityBytes,

```

### 13 of

```diff
-    pub const ALL: [Self; 4] = [Self::WarmupTimesteps, Self::RunTimesteps, Self::TiersPublished, Self::FacilityElectricityWh];
+    pub const ALL: [Self; 5] = [Self::WarmupTimesteps, Self::RunTimesteps, Self::TiersPublished, Self::FacilityElectricityWh, Self::InputIdentityBytes];

```

### 15 id

```diff
+            Self::InputIdentityBytes => "energy.input-identity-bytes",

```

### 17 label

```diff
+            Self::InputIdentityBytes => LocalizedLabel::native("Input bytes verified", "Geprüfte Eingabebytes"),

```

### 19 simulation_config_for

```diff
+        run_period_year: model.run_period.year,

```

### 21 step_one

```diff
-                inverter_efficiency: item.inverter_efficiency
+                inverter_efficiency: item.inverter_efficiency,
+                temperature_coefficient: item.temperature_coefficient,
+                faiman_heat_loss_constant_w_m2k: item.faiman_heat_loss_constant_w_m2k,
+                faiman_heat_loss_wind_w_s_m3k: item.faiman_heat_loss_wind_w_s_m3k,
+                mounting: item.mounting

```

### 23 finish

```diff
+    Activity,

```

### 25 new

```diff
-/// 📮️ One encoded tick lent as a preview: the publication binds the source bytes until they are paid back.
-struct EnergyTickPublication {
-    single: RetainedJobPublication,
-    bytes: Vec<u8>,
-    delivered: bool,
-}
-
-impl EnergyTickPublication {
-    fn new(bytes: Vec<u8>) -> Box<Self> {
-        Box::new(Self { single: RetainedJobPublication::new(), bytes, delivered: false })
-    }
-
-    fn poll<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
-        let result = self.single.advance_from_source(JobPublicationKind::Preview, &self.bytes, cx)?;
-        if result.is_some() {
-            self.delivered = true;
-        }
-        Ok(result)
-    }
-
-    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
-        self.single.borrow_outcome(descriptor)
-    }
-
-    fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
-        if !self.single.terminal_is_empty() {
-            return self.single.retirement_demands();
-        }
-        Ok(RetirementDemand { release_bytes: self.bytes.capacity(), depth: usize::from(self.bytes.capacity() != 0), ..Default::default() })
-    }
-
-    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
-        let demand = match self.retirement_demands() {
-            Ok(demand) => demand,
-            Err(error) => return InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
-        };
-        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
-            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
-        }
-        if !self.single.terminal_is_empty() {
-            return match self.single.close_step(grant) {
-                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress },
-                Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
-            };
-        }
-        if self.bytes.capacity() != 0 {
-            let released_bytes = self.bytes.capacity();
-            self.bytes = Vec::new();
-            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() } };
-        }
-        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
-    }
-
-    fn terminal_is_empty(&self) -> bool {
-        self.single.terminal_is_empty() && self.bytes.capacity() == 0
-    }
-}
+#[path="📮️publication/🦀️.rs"]
+mod publication;
+use publication::EnergyTickPublication;

```

### 27 terminal_is_empty

```diff
+    input_identity_bytes: u64,
+    input_identity_ready: bool,

```

### 29 new

```diff
-        Self { stage: EnergyJobStage::Validate, tier: EnergyQualityTier::SteadyStateEstimate, timestep: 0, total_timesteps: 0, warmup_hours: 0, warmup_timesteps: 0, run_timesteps: 0, tiers_published: 0, facility_electricity_kwh: 0.0, surface_payload_due: false }
+        Self { input_identity_bytes: 0, input_identity_ready: false, stage: EnergyJobStage::Validate, tier: EnergyQualityTier::SteadyStateEstimate, timestep: 0, total_timesteps: 0, warmup_hours: 0, warmup_timesteps: 0, run_timesteps: 0, tiers_published: 0, facility_electricity_kwh: 0.0, surface_payload_due: false }

```

### 31 publish_tier

```diff
-    /// 🔭️ Folds one numerical cursor; `true` when exactly one warmup or run timestep was computed.
+    /// 🔭️ Folds actual numerical activity while keeping identity bytes separate from timestep counters.

```

### 33 observe

```diff
+        let input_progress = cursor.input_identity_bytes as u64 / 4096 > self.input_identity_bytes / 4096 || cursor.input_identity_ready && !self.input_identity_ready;
+        self.input_identity_bytes = cursor.input_identity_bytes as u64;
+        self.input_identity_ready = cursor.input_identity_ready;

```

### 35 observe

```diff
-        warmup || run
+        warmup || run || input_progress

```

### 37 progress

```diff
-        let values = [self.warmup_timesteps, self.run_timesteps, self.tiers_published, (self.facility_electricity_kwh * 1_000.0).round().max(0.0) as u64];
+        let values = [self.warmup_timesteps, self.run_timesteps, self.tiers_published, (self.facility_electricity_kwh * 1_000.0).round().max(0.0) as u64, self.input_identity_bytes];

```

### 39 progress

```diff
-/// `EnergyJob` and steps it inline under the caller's deadline. It consumes one unit of fuel per computed
-/// timestep and flushes one tick whenever at least one timestep was computed or the run settled; the
+/// `EnergyJob` and steps it inline under the caller's physical work and deadline. It flushes a tick for input activity,
+/// a computed timestep or settlement; the semantic completed count retains timestep units and the

```

### 41 progress

```diff
-    writer: ToolRunTickWriter,
+    writer: Option<ToolRunTickWriter>,
+    writer_close:Option<ControlledRetirement<ToolRunTickWriter>>,
+    tick_pending:bool,
+    surface_requested:bool,
+    pending_state:ToolRunState,

```

### 43 progress

```diff
-    template: SimulationConfig,
+    template: Option<SimulationConfig>,
+    template_close:Option<ControlledRetirement<SimulationConfig>>,
+    snapshot_close:Option<semio_framework_value::retirement::shared::SharedControlledRetirement<EnergyModelSnapshot>>,

```

### 45 progress

```diff
-    operation: OperationId,
-    generation: Generation,
-    numerical_sequence: u64,
+    checkpoint: Option<sim_wire::EnergyWireLease>,

```

### 47 new

```diff
-            writer: ToolRunTickWriter::new(identity),
+            writer: Some(ToolRunTickWriter::new(identity)),
+            writer_close:None,
+            tick_pending:false,
+            surface_requested:false,
+            pending_state:ToolRunState::Running,

```

### 49 new

```diff
-            template,
+            template:Some(template),
+            template_close:None,
+            snapshot_close:None,

```

### 51 new

```diff
-            operation: semio_framework_job::allocate_operation_id(),
-            generation: Generation(u64::from(identity.generation)),
-            numerical_sequence: 0,
+            checkpoint: None,

```

### 53 refuse

```diff
-        let _ = self.writer.step(ToolRunStepKind::Danger, self.stage().index(), reason.code(), None, args);
+        let _ = self.writer.as_mut().expect("original running tick writer").step(ToolRunStepKind::Danger, self.stage().index(), reason.code(), None, args);

```

### 55 census_one

```diff
-    fn capture_one(&mut self) -> EnergyRunGrant {
+    fn capture_one(&mut self, cx: &StepContext<'_>) -> EnergyRunGrant {

```

### 57 capture_one

```diff
-            Ok(true) => self.admit(),
+            Ok(true) => self.admit(cx),

```

### 59 capture_one

```diff
-    fn admit(&mut self) -> EnergyRunGrant {
+    fn admit(&mut self, cx: &StepContext<'_>) -> EnergyRunGrant {

```

### 61 admit

```diff
-        let config = simulation_config_for(&self.template, &model);
+        let config = simulation_config_for(self.template.as_ref().expect("original Energy template remains retained"), &model);

```

### 63 admit

```diff
-        let operation = semio_framework_job::Operation::new(self.operation, semio_framework_job::RevisionId(base), self.generation, self.identity.id.run ^ base);
-        self.snapshot = None;
+        let operation = semio_framework_job::Operation::new(cx.operation(), semio_framework_job::RevisionId(base), cx.generation(), self.identity.id.run ^ base);

```

### 65 admit

```diff
-    /// 🦶️ Steps the numerical job until one timestep is computed, the run settles or the caller must yield. Each
-    /// numerical step gets its own context, because a step context grants at most one payload page.
-    fn simulate(&mut self, cx: &mut StepContext<'_>) -> EnergyRunGrant {
-        let Self { numerical, numerical_sequence, operation, generation, cursor, writer, phase, .. } = self;
-        let Some(job) = numerical.as_mut() else { return EnergyRunGrant::Settled(EnergyRunSettled::Fault) };
+    /// 🦶️ Lends the actual original work and retained stock to the numerical child until its next timestep.
+    fn simulate(&mut self, cx: &mut StepContext<'_>) -> Result<EnergyRunGrant, semio_framework_job::OriginalTurnFailure> {
+        let Self { numerical, checkpoint, cursor, writer, phase, .. } = self;
+        let Some(job) = numerical.as_mut() else { return Ok(EnergyRunGrant::Settled(EnergyRunSettled::Fault)) };
+        if checkpoint.is_some() {
+            acknowledge_checkpoint(job, checkpoint, cx)?;
+            return Ok(EnergyRunGrant::Continue);
+        }
+        let writer=writer.as_mut().expect("original numerical tick writer");

```

### 67 simulate

```diff
-            if cx.should_yield() {
-                return EnergyRunGrant::Continue;
+            if cx.should_yield()||cx.retained_grant().maximum_items==0 {
+                return Ok(EnergyRunGrant::Continue);

```

### 69 simulate

```diff
-            let mut receipt = RetainedCloneProgress::default();
-            let mut inner = StepContext::new(*operation, *generation, StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, cx.deadline_us(), cx.retained_grant()), cx.cancel_token(), semio_framework_job::default_now_us, numerical_sequence, &mut receipt);
-            let seen = match InteractiveJob::step(job, &mut inner) {
-                Ok(None | Some(JobOutcomeBorrow::Yield { .. })) => EnergyChildSeen::Yield,
-                Ok(Some(JobOutcomeBorrow::PreviewReady { .. })) => EnergyChildSeen::Preview,
-                Ok(Some(JobOutcomeBorrow::CheckpointReady { .. })) => EnergyChildSeen::Checkpoint,
-                Ok(Some(JobOutcomeBorrow::Complete { .. })) => EnergyChildSeen::Complete,
-                Ok(Some(JobOutcomeBorrow::Fault { .. })) | Err(_) => EnergyChildSeen::Fault,
-                Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => EnergyChildSeen::Cancelled,
-            };
-            drop(inner);
-            if cx.consume_retained(receipt).is_err() {
-                *phase = EnergyRunPhase::Settled;
-                return EnergyRunGrant::Settled(EnergyRunSettled::Fault);
-            }
+            let before = (cx.fuel_remaining(), cx.retained_progress());
+            let grant = cx.retained_grant();
+            let deadline = cx.deadline_us();
+            let fuel = cx.fuel_remaining().min(semio_framework_job::INTERACTIVE_LANE_FUEL);
+            let seen = cx.with_original_work_loan(fuel, deadline, |cx| cx.with_retained_loan(grant, |cx| {
+                Ok(match InteractiveJob::step(job, cx)? {
+                    None | Some(JobOutcomeBorrow::Yield { .. }) => EnergyChildSeen::Yield,
+                    Some(JobOutcomeBorrow::PreviewReady { .. }) => EnergyChildSeen::Preview,
+                    Some(JobOutcomeBorrow::CheckpointReady { .. }) => EnergyChildSeen::Checkpoint,
+                    Some(JobOutcomeBorrow::Complete { .. }) => EnergyChildSeen::Complete,
+                    Some(JobOutcomeBorrow::Fault { .. }) => EnergyChildSeen::Fault,
+                    Some(JobOutcomeBorrow::Cancelled { .. }) => EnergyChildSeen::Cancelled,
+                })
+            }))?;

```

### 71 simulate

```diff
-                EnergyChildSeen::Checkpoint => (!acknowledge_checkpoint(job, *generation, cx)).then_some(EnergyRunSettled::Fault),
+                EnergyChildSeen::Checkpoint => { acknowledge_checkpoint(job, checkpoint, cx)?; None },

```

### 73 simulate

```diff
-                return EnergyRunGrant::Settled(outcome);
+                return Ok(EnergyRunGrant::Settled(outcome));

```

### 75 simulate

```diff
+            let completed = cursor.completed();

```

### 77 simulate

```diff
-                return EnergyRunGrant::Timestep;
+                return Ok(if cursor.completed() > completed { EnergyRunGrant::Timestep } else { EnergyRunGrant::Activity });

```

### 79 simulate

```diff
+            if before == (cx.fuel_remaining(), cx.retained_progress()) { return Ok(EnergyRunGrant::Continue); }

```

### 81 simulate

```diff
-    fn next_action(&mut self, cx: &mut StepContext<'_>) -> EnergyRunAction {
+    fn next_action(&mut self, cx: &mut StepContext<'_>) -> Result<EnergyRunAction, semio_framework_job::OriginalTurnFailure> {

```

### 83 next_action

```diff
-            return EnergyRunAction::Cancelled;
+            return Ok(EnergyRunAction::Cancelled);

```

### 85 next_action

```diff
-            return EnergyRunAction::Publish;
+            return Ok(EnergyRunAction::Publish);
+        }
+        if self.tick_pending {
+            let stage=self.stage();
+            if let Some(publication)=EnergyTickPublication::admit(&mut self.writer,&self.cursor,self.identity,self.pending_state,stage,self.surface_requested,cx)?{self.publication=Some(publication);self.tick_pending=false;}
+            return Ok(EnergyRunAction::Yield);

```

### 87 next_action

```diff
-            return EnergyRunAction::Settled(settled);
+            return Ok(EnergyRunAction::Settled(settled));

```

### 89 next_action

```diff
+        let mut activity = false;

```

### 91 next_action

```diff
-            if cx.should_yield() || self.phase == EnergyRunPhase::Settled {
+            if cx.should_yield() || cx.retained_grant().maximum_items==0 || self.phase == EnergyRunPhase::Settled {

```

### 93 next_action

```diff
+            let before = (cx.fuel_remaining(), cx.retained_progress());

```

### 95 next_action

```diff
-                EnergyRunPhase::Census => self.census_one(),
-                EnergyRunPhase::Capture => self.capture_one(),
-                EnergyRunPhase::Simulate => self.simulate(cx),
+                EnergyRunPhase::Census => { cx.consume_fuel(1); self.census_one() },
+                EnergyRunPhase::Capture => { cx.consume_fuel(1); self.capture_one(cx) },
+                EnergyRunPhase::Simulate => self.simulate(cx)?,

```

### 97 next_action

```diff
-                EnergyRunGrant::Continue => {}
+                EnergyRunGrant::Continue => { if before == (cx.fuel_remaining(), cx.retained_progress()) { break None; } }
+                EnergyRunGrant::Activity => { activity = true; break None; }

```

### 99 next_action

```diff
-                    cx.consume_fuel(1);

```

### 101 next_action

```diff
-        if !computed && settled.is_none() {
-            return EnergyRunAction::Yield;
+        if !computed && !activity && settled.is_none() {
+            return Ok(EnergyRunAction::Yield);

```

### 103 next_action

```diff
-        if std::mem::take(&mut self.cursor.surface_payload_due) || interval_due || complete {
-            if let Some(table) = self.numerical.as_ref().and_then(|job| job.per_surface_energy()) {
-                if table.has_energy() {
-                    self.writer.payload(encode_surface_energy_payload(table.summaries()));
-                    self.ticks_since_surface_payload = 0;
-                }
-            }
-        }
-        self.writer.progress(self.cursor.progress(self.identity, state, self.stage()));
-        self.settled = settled;
-        match self.writer.finish().and_then(|tick| tick.encode().ok()) {
-            Some(bytes) => {
-                self.publication = Some(EnergyTickPublication::new(bytes));
-                EnergyRunAction::Publish
-            }
-            None => {
-                self.settled = Some(EnergyRunSettled::Fault);
-                EnergyRunAction::Settled(EnergyRunSettled::Fault)
-            }
-        }
+        self.surface_requested=std::mem::take(&mut self.cursor.surface_payload_due)||interval_due||complete;
+        self.pending_state=state;
+        self.settled=settled;
+        self.tick_pending=true;
+        Ok(EnergyRunAction::Yield)

```

### 105 next_action

```diff
-    fn retire_delivered<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
+    fn retire_delivered<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {

```

### 107 retire_delivered

```diff
-        let step = publication.close_step(cx.retained_grant());
-        cx.consume_retained(step.progress())?;
-        if let InteractiveJobCloseStep::Refused { kind, progress } = step {
-            return Err(ValueError::literal(kind, "energy tick publication close was refused").with_retained_progress(progress));
-        }
-        if publication.terminal_is_empty() {
-            self.publication = None;
-        }
+        if !cx.is_cancelled()&&publication.published_surface()&&self.ticks_since_surface_payload!=0{let demand=RetirementDemand{copy_bytes:size_of::<u64>(),depth:1,..Default::default()};let grant=cx.retained_grant();if grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_depth>=demand.depth{self.ticks_since_surface_payload=0;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;}return Ok(None);}
+        if self.writer.is_none()&&!self.closing&&!cx.is_cancelled(){if let Some(writer)=publication.admit_successor(cx)?{self.writer=Some(writer);}return Ok(None);}
+        if !self.closing&&!cx.is_cancelled()&&publication.has_more_ranges(self.numerical.as_ref().and_then(|job|job.per_surface_energy()))?{publication.advance_range(&mut self.writer,cx)?;return Ok(None);}
+        let demand=publication.retirement_demands()?;
+        semio_framework_job::close_original_retained_step(cx,demand,|grant|{
+            let progress=self.publication.as_mut().expect("original delivered Energy tick remains retained").close_step(grant)?.progress();
+            if self.publication.as_ref().is_some_and(|publication|publication.terminal_is_empty()){self.publication=None;}
+            Ok(RetainedCloneStep::Progress(progress))
+        })?;

```

### 109 close_demands

```diff
+        if self.checkpoint.is_some() { return checkpoint_demands(&self.checkpoint); }

```

### 111 close_demands

```diff
+        if let Some(close)=self.writer_close.as_ref(){if close.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<ToolRunTickWriter>>>(),depth:1,..Default::default()})}return Ok(RetirementDemand{copy_bytes:close.next_copy_byte_demand()?,capacity_bytes:close.next_capacity_byte_demand(usize::MAX)?,release_bytes:close.next_release_byte_demand()?,depth:close.next_depth_demand()?+1})}
+        if self.writer.is_some(){return Ok(RetirementDemand{copy_bytes:2*size_of::<ControlledRetirement<ToolRunTickWriter>>()+size_of::<Option<ToolRunTickWriter>>(),depth:1,..Default::default()})}

```

### 113 close_demands

```diff
+            if InteractiveJob::terminal_is_empty(job){return Ok(RetirementDemand{copy_bytes:size_of::<Option<EnergyJob>>(),depth:1,..Default::default()})}

```

### 115 close_demands

```diff
+        if let Some(close)=self.snapshot_close.as_ref(){return if close.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retirement::shared::SharedControlledRetirement<EnergyModelSnapshot>>>(),depth:1,..Default::default()})}else{close.next_demand(usize::MAX)}}
+        if self.snapshot.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value::retirement::shared::SharedControlledRetirement<EnergyModelSnapshot>>()+size_of::<Option<Arc<EnergyModelSnapshot>>>(),depth:1,..Default::default()})}
+        if let Some(close)=self.template_close.as_ref(){return if close.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<SimulationConfig>>>(),depth:1,..Default::default()})}else{close.selected_demand(usize::MAX)}}
+        if self.template.is_some(){return Ok(RetirementDemand{copy_bytes:2*size_of::<ControlledRetirement<SimulationConfig>>()+size_of::<Option<SimulationConfig>>(),depth:1,..Default::default()})}

```

### 117 close_demands

```diff
+#[derive(Clone, Copy)]

```

### 119 close_demands

```diff
-fn acknowledge_checkpoint(job: &mut EnergyJob, generation: Generation, cx: &mut StepContext<'_>) -> bool {
-    let mut lease = match job.take_checkpoint_packet(generation) {
-        Ok(Some(lease)) => lease,
-        Ok(None) => return true,
-        Err(_) => return false,
-    };
-    for _ in 0..64 {
-        if lease.packet().terminal_is_empty() {
-            return job.ack_checkpoint_packet(lease).is_ok();
+fn acknowledge_checkpoint(job: &mut EnergyJob, checkpoint: &mut Option<sim_wire::EnergyWireLease>, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
+    let demand = checkpoint_demands(checkpoint)?;
+    let generation = cx.generation();
+    semio_framework_job::close_original_retained_step(cx, demand, |grant| {
+        if checkpoint.as_ref().is_some_and(|lease| !lease.packet().terminal_is_empty()) {
+            return checkpoint.as_mut().expect("original numerical checkpoint").packet_mut().close_step(grant);

```

### 121 acknowledge_checkpoint

```diff
-        match lease.packet_mut().close_step(cx.retained_grant()) {
-            Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => {
-                if cx.consume_retained(progress).is_err() {
-                    return false;
-                }
+        if let Some(lease) = checkpoint.take() {
+            if let Err(lease) = job.ack_checkpoint_packet(lease) {
+                *checkpoint = Some(lease);
+                return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "original Energy checkpoint acknowledgement refused its retained lease"));

```

### 123 acknowledge_checkpoint

```diff
-            Err(_) => return false,
+        } else {
+            *checkpoint = job.take_checkpoint_packet(generation).map_err(|_| ValueError::literal(ValueRefusalKind::InvariantViolated, "original Energy checkpoint differs from its admitted generation"))?;

```

### 125 acknowledge_checkpoint

```diff
-    }
-    false
+        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
+    })?;
+    Ok(checkpoint.is_none())
+}
+
+/// 📏️ Quotes the exact original checkpoint body or one lease transfer before mutation.
+fn checkpoint_demands(checkpoint: &Option<sim_wire::EnergyWireLease>) -> Result<RetirementDemand, ValueError> {
+    if let Some(lease) = checkpoint.as_ref() { if !lease.packet().terminal_is_empty() { return lease.packet().retirement_demands(); } }
+    Ok(RetirementDemand { copy_bytes: size_of::<Option<sim_wire::EnergyWireLease>>(), depth: 1, ..Default::default() })

```

### 127 acknowledge_checkpoint

```diff
-    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
+    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {

```

### 129 step

```diff
-        match self.next_action(cx) {
+        match self.next_action(cx)? {

```

### 131 step

```diff
-            EnergyRunAction::Cancelled => JobOutcomeBorrow::admit_cancelled(cx),
-            EnergyRunAction::Publish => self.publication.as_mut().expect("a staged tick publication").poll(cx),
+            EnergyRunAction::Cancelled => JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into),
+            EnergyRunAction::Publish => {let table=self.numerical.as_ref().and_then(|job|job.per_surface_energy());self.publication.as_mut().expect("a staged tick publication").poll(cx,table).map_err(Into::into)},

```

### 133 begin_close

```diff
-    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
+    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{

```

### 135 close_step

```diff
-        let one = RetainedCloneProgress { copied_items: 1, ..Default::default() };
-        let idle = InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
-        if self.terminal_is_empty() {
-            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
+        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
+        if self.checkpoint.is_some() {
+            let job = self.numerical.as_mut().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original Energy checkpoint retains its numerical owner"))?;
+            let before = cx.retained_progress();
+            acknowledge_checkpoint(job, &mut self.checkpoint, cx)?;
+            let after = cx.retained_progress();
+            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: after.copied_items - before.copied_items, copied_bytes: after.copied_bytes - before.copied_bytes, retained_capacity_bytes: after.retained_capacity_bytes - before.retained_capacity_bytes, released_bytes: after.released_bytes - before.released_bytes } });

```

### 137 close_step

```diff
-        if let Some(publication) = self.publication.as_mut() {
-            let step = publication.close_step(grant);
-            if publication.terminal_is_empty() {
-                self.publication = None;
+        if self.publication.is_none()&&self.writer_close.is_none()&&self.writer.is_none(){
+            if let Some(job)=self.numerical.as_mut(){
+                if !InteractiveJob::terminal_is_empty(job){return InteractiveJob::close_step(job,cx)}

```

### 139 close_step

```diff
-            return match step {
-                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
-                step => step,
-            };

```

### 141 close_step

```diff
-        if let Some(job) = self.numerical.as_mut() {
-            return match InteractiveJob::close_step(job, grant) {
-                InteractiveJobCloseStep::Complete { progress } => {
-                    self.numerical = None;
-                    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } }
-                }
-                pending => pending,
-            };
-        }
-        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
-            return idle;
-        }
-        if let Some(rejected) = self.rejected.as_mut() {
-            return match rejected.close_step(grant) {
-                InteractiveJobCloseStep::Complete { .. } if rejected.terminal_is_empty() => {
-                    self.rejected = None;
-                    InteractiveJobCloseStep::Pending { progress: one }
-                }
-                InteractiveJobCloseStep::Complete { .. } => InteractiveJobCloseStep::Blocked,
-                pending => pending,
-            };
-        }
-        if let Some(capture) = self.capture.take() {
-            self.capture_close = Some(EnergyModelCloseCursor::new(capture.finish()));
-            return InteractiveJobCloseStep::Pending { progress: one };
-        }
-        if let Some(close) = self.capture_close.as_mut() {
-            return match close.close_step(grant) {
-                InteractiveJobCloseStep::Complete { .. } => {
-                    self.capture_close = None;
-                    InteractiveJobCloseStep::Pending { progress: one }
-                }
-                pending => pending,
-            };
-        }
-        if self.snapshot.take().is_some() {
-            return InteractiveJobCloseStep::Pending { progress: one };
-        }
-        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
+        let demand=self.close_demands()?;
+        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
+            let one=RetainedCloneProgress{copied_items:1,..Default::default()};
+            let progress=if let Some(publication)=self.publication.as_mut(){
+                let progress=publication.close_step(grant)?.progress();
+                if publication.terminal_is_empty(){self.publication=None;}
+                progress
+            }else if let Some(close)=self.writer_close.as_mut(){
+                if close.terminal_is_empty(){self.writer_close=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
+                else{close.step(grant)?.progress()}
+            }else if let Some(writer)=self.writer.take(){
+                match ControlledRetirement::new(writer){Ok(close)=>self.writer_close=Some(close),Err((error,writer))=>{self.writer=Some(writer);return Err(error)}}
+                RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
+            }else if self.numerical.is_some(){
+                self.numerical=None;
+                RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
+            }else if let Some(rejected)=self.rejected.as_mut(){
+                let step=rejected.close_step(grant);
+                if rejected.terminal_is_empty(){self.rejected=None;}
+                step.progress()
+            }else if let Some(capture)=self.capture.take(){
+                self.capture_close=Some(EnergyModelCloseCursor::new(capture.finish()));one
+            }else if let Some(close)=self.capture_close.as_mut(){
+                let progress=close.close_step(grant).progress();
+                if close.terminal_is_empty(){self.capture_close=None;}
+                progress
+            }else if let Some(close)=self.snapshot_close.as_mut(){
+                if close.terminal_is_empty(){self.snapshot_close=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
+                else{close.step(grant)?.progress()}
+            }else if let Some(snapshot)=self.snapshot.take(){
+                self.snapshot_close=Some(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(snapshot));
+                RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
+            }else if let Some(close)=self.template_close.as_mut(){
+                if close.terminal_is_empty(){self.template_close=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
+                else{close.step(grant)?.progress()}
+            }else if let Some(template)=self.template.take(){
+                match ControlledRetirement::new(template){Ok(close)=>self.template_close=Some(close),Err((error,template))=>{self.template=Some(template);return Err(error)}}
+                RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
+            }else{Default::default()};
+            Ok(RetainedCloneStep::Progress(progress))
+        })?;
+        Ok(match step{InteractiveJobCloseStep::Pending{progress}if self.terminal_is_empty()=>InteractiveJobCloseStep::Complete{progress},step=>step})

```

### 143 terminal_is_empty

```diff
-        self.closing && self.publication.is_none() && self.numerical.is_none() && self.rejected.is_none() && self.capture.is_none() && self.capture_close.is_none() && self.snapshot.is_none()
+        self.closing && self.checkpoint.is_none() && self.writer.is_none() && self.writer_close.is_none() && self.publication.is_none() && self.numerical.is_none() && self.rejected.is_none() && self.capture.is_none() && self.capture_close.is_none() && self.snapshot.is_none() && self.snapshot_close.is_none() && self.template.is_none() && self.template_close.is_none()

```

### 145 terminal_is_empty

```diff
+
+#[cfg(test)]
+#[global_allocator]
+static ENERGY_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

```

## 56 🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🦀️.rs

### 1 target

```diff
-        Some(handle) => json!({ "tenant": TENANT, "kind": "quiz-handle", "id": course::handle_actor_id(&handle.key) }),
-        None => json!({ "tenant": TENANT, "kind": "quiz-learner", "id": command["learner"] }),
+        Some(handle) => json!({ "tenant": TENANT, "kind": "course-handle", "id": course::handle_actor_id(&handle.key) }),
+        None => json!({ "tenant": TENANT, "kind": "course-learner", "id": command["learner"] }),

```

### 3 envelope

```diff
-        "kind": format!("quiz.{kind}"),
+        "kind": format!("course.{kind}"),

```

### 5 query_envelope

```diff
-    json!({ "queryId": id(0), "kind": format!("quiz.{kind}"), "version": course::WIRE_VERSION, "scope": TENANT, "principal": { "kind": "anonymous" }, "arguments": bytes(&query.to_string()), "consistency": { "kind": "authority" }, "cursor": null })
+    json!({ "queryId": id(0), "kind": format!("course.{kind}"), "version": course::WIRE_VERSION, "scope": TENANT, "principal": { "kind": "anonymous" }, "arguments": bytes(&query.to_string()), "consistency": { "kind": "authority" }, "cursor": null })

```

### 7 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
-    assert!(declared.len() == 12 && declared.iter().all(|(kind, version)| kind.starts_with("quiz.") && *version == u64::from(course::WIRE_VERSION)), "a client agrees on the contract by the versions GET /instance declares: {declared:?}");
+    assert!(declared.len() == 12 && declared.iter().all(|(kind, version)| kind.starts_with("course.") && *version == u64::from(course::WIRE_VERSION)), "a client agrees on the contract by the versions GET /instance declares: {declared:?}");

```

### 9 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
-    assert_eq!(get(&base, &format!("/actors/{TENANT}/quiz-handle/{}/events", course::handle_actor_id("ada lovelace")), &[]).0, 403, "who holds a handle is not readable as a stream");
+    assert_eq!(get(&base, &format!("/actors/{TENANT}/course-handle/{}/events", course::handle_actor_id("ada lovelace")), &[]).0, 403, "who holds a handle is not readable as a stream");

```

### 11 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
-    let (_, _, registered) = get(&base, &format!("/actors/{TENANT}/quiz-learner/{ada}/events"), &[]);
+    let (_, _, registered) = get(&base, &format!("/actors/{TENANT}/course-learner/{ada}/events"), &[]);

```

### 13 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
-    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/quiz-learner/{ada}/events"), &[]);
+    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/course-learner/{ada}/events"), &[]);

```

### 15 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
-    assert_eq!(stream.iter().filter(|record| record["kind"] == "quiz.answer-recorded").count(), 1);
-    assert_eq!(stream[0]["kind"], "quiz.learner-registered");
-    assert_eq!(rejected(&base, &json!({ "type": "submit-run", "id": id(21), "learner": ada, "run": run })), "run-incomplete");
+    assert_eq!(stream.iter().filter(|record| record["kind"] == "course.answer-recorded").count(), 1);
+    assert_eq!(stream[0]["kind"], "course.learner-registered");

```

### 17 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
+    let early_run = id(0x110);
+    accepted(&base, &start(&anonymous, &early_run, "power", "medium", 40));
+    let early_open = view(&base, &json!({ "type": "run", "run": early_run }));
+    let early_tasks = early_open["sheet"]["tasks"].as_array().expect("tasks");
+    accepted(&base, &record(&anonymous, &early_run, &early_tasks[0], &perfect(&document, &early_tasks[0]), 41));
+    let early = accepted(&base, &json!({ "type": "submit-run", "id": id(42), "learner": anonymous, "run": early_run }));
+    let early_score = early[0]["result"]["score"].as_f64().expect("score");
+    assert!(early_score > 0.0 && early_score < 1.0, "an open task scores as a miss and the answered task still earns its share: {early_score}");
+    assert_eq!(early[0]["result"]["points"].as_f64(), Some(early_score * 200.0));

```

### 19 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
-    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/quiz-learner/{ada}/events"), &[]);
+    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/course-learner/{ada}/events"), &[]);

```

### 21 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
-    assert_eq!((board["rows"][0]["total"].as_f64(), board["rows"][0]["badges"].as_array().map(Vec::len), board["submissions"].as_u64()), (Some(400.0), Some(3), Some(2)));
+    assert_eq!((board["rows"][0]["total"].as_f64(), board["rows"][0]["badges"].as_array().map(Vec::len), board["submissions"].as_u64()), (Some(400.0), Some(3), Some(3)));

```

### 23 a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart

```diff
-    assert_eq!((homes["own"]["total"].as_f64(), homes["own"]["best"].clone(), homes["own"]["runs"].as_u64(), homes["own"]["badges"].clone(), homes["submissions"].as_u64()), (Some(200.0), json!({ "homes": { "challenge": "medium", "score": 1.0, "points": 200.0 } }), Some(1), json!(["complete"]), Some(2)), "the board of one quiz is made of that quiz's runs and the badges they earned: {homes}");
+    assert_eq!((homes["own"]["total"].as_f64(), homes["own"]["best"].clone(), homes["own"]["runs"].as_u64(), homes["own"]["badges"].clone(), homes["submissions"].as_u64()), (Some(200.0), json!({ "homes": { "challenge": "medium", "score": 1.0, "points": 200.0 } }), Some(1), json!(["complete"]), Some(3)), "the board of one quiz is made of that quiz's runs and the badges they earned: {homes}");

```

### 25 sign_up

```diff
-/// 📏️ The size of the largest `record-answer` body a course can produce: every item of every task
-/// answered — with a guess of the longest a number can be written where the keys are hidden —,
+/// 📏️ The size of the largest `record-answer` body a course can produce: every presented item
+/// answered, using the longest ids allowed by each pool slot or draw and the longest numeric guesses,

```

### 27 largest_answer

```diff
-            let items = ids(&task["items"]);
+            let items = if let Some(slots) = task["pool"]["slots"].as_array() {
+                slots.iter().map(|slot| slot.as_array().expect("slot").iter().map(|item| item.as_str().expect("item id")).max_by_key(|item| item.len()).expect("nonempty slot").to_string()).collect::<Vec<_>>()
+            } else {
+                let mut items = ids(&task["items"]);
+                items.sort_by_key(|item| std::cmp::Reverse(item.len()));
+                items.truncate(task["draw"].as_u64().map_or(items.len(), |draw| draw as usize));
+                items
+            };

```

### 29 an_anonymous_caller_can_neither_read_nor_occupy_an_enrollment_key

```diff
-        relayed["kind"] = json!("quiz.enroll-learner");
+        relayed["kind"] = json!("course.enroll-learner");

```

### 31 sockets_are_counted_per_address_and_an_oversized_message_ends_the_socket

```diff
-    let mut stream = socket(address, &format!("/actors/{TENANT}/quiz-learner/{learner}/events/ws"), &[]);
+    let mut stream = socket(address, &format!("/actors/{TENANT}/course-learner/{learner}/events/ws"), &[]);

```

## 73 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts

### 0 

```diff
-/** 🧭️ The quiz client session: local-first, event-driven state of one learner on one device, and the controller that
+/** 🧭️ The course client session: local-first, event-driven state of one learner on one device, and the controller that

```

### 2 

```diff
-import { latestWins } from "@semio-tech/framework";
+import { latestWins } from "@semio-tech/framework/async";

```

### 4 

```diff
-/** 📄️ The pages of home besides the quizzes' own pages (a quiz's page is its id): each is a card of the overview and
- * the page behind it, addressed by hash (`#board`). */
-export const HOME_PAGES = { learner: "learner", introduction: "intro", leaderboard: "board", badges: "badges", preferences: "prefs" } as const;
+/** 📄️ The root home pages: each is a card and its backdrop, addressed by hash (`#board`); quizzes holds topic or quiz grids. */
+export const HOME_PAGES = { learner: "learner", quizzes: "quizzes", leaderboard: "board", badges: "badges", preferences: "prefs" } as const;

```

### 6 

```diff
- * of {@link HOME_PAGES} or a quiz id. */
+ * of {@link HOME_PAGES}, a topic id, or a quiz id. */

```

### 8 initialCourseState

```diff
-  const step: CourseStep = !persisted.introduced ? { screen: "introduction" } : persisted.learner === undefined ? { screen: "identity" } : { screen: "home" };
+  const step: CourseStep = { screen: "home" };

```

### 10 stepAfter

```diff
-/** ⏫️ The place above the step in front: the overview above its pages, the page of its quiz above a run and its
+/** ⏫️ The place above the step in front: its topic above a quiz, quizzes above a topic, the overview above general pages, the page of its quiz above a run and its

```

### 12 stepAbove

```diff
-  if (step.screen === "home") return step.page === undefined ? undefined : { screen: "home" };
+  if (step.screen === "home") {
+    const selected = step.page;
+    if (selected === undefined) return undefined;
+    const topic = state.course?.topics?.find((candidate) => candidate.quizzes.includes(selected));
+    const page = topic?.id ?? (state.course?.quizzes.some((candidate) => candidate.id === selected) || state.course?.topics?.some((candidate) => candidate.id === selected) ? HOME_PAGES.quizzes : undefined);
+    return page === undefined ? { screen: "home" } : { screen: "home", page };
+  }

```

### 14 evolveCourseState

```diff
-      return { ...state, introduced: true, step: state.learner === undefined ? { screen: "identity" } : { screen: "home" }, trail: EMPTY_TRAIL };
+      return { ...state, introduced: true, step: state.step.screen === "introduction" ? { screen: "home" } : state.step };

```

## 95 🧰️framework/🛍️products/❓️quiz/🧪️tests/🏠️home-grid/🟦️.tsx

### 1 

```diff
- * vectors), the nine card sections over the glass with the leaderboard fifth and each heading linking its page, every
+ * vectors), five general cards over the glass and nested topic and quiz grids, each heading linking its page, every

```

### 3 

```diff
+import { produce } from "immer";

```

### 5 

```diff
+  FollowProvider,

```

### 7 

```diff
+  addressPages,
+  parentPage,
+  pagesWithin,
+  previewCamera,
+  placePeers,

```

### 9 

```diff
-import grid from "../🧫️fixtures/🏠️home-grid/🔣️.json";
+import grid from "../../🧫️fixtures/🏠️home-grid/🔣️.json";
+import followedLayouts from "../../🧫️fixtures/👀️follow/🔣️.json";

```

### 11 

```diff
+  readonly hierarchy: { readonly topics: CourseView["topics"]; readonly scopes: readonly { readonly page: string | null; readonly pages: readonly string[] }[]; readonly parents: readonly { readonly page: string; readonly parent: string | null }[] };

```

### 13 

```diff
-const PAGES =["learner", "physics", "intro", "heating", "board", "cooling", "badges", "demand", "prefs"];
-const DESKTOP_CELLS_OF_THE_REFERENCE = {
-  learner: { column: 0, row: 0 },
-  q0: { column: 1, row: 0 },
-  intro: { column: 2, row: 0 },
-  q1: { column: 0, row: 1 },
-  board: { column: 1, row: 1 },
-  q2: { column: 2, row: 1 },
-  badges: { column: 0, row: 2 },
-  q3: { column: 1, row: 2 },
-  prefs: { column: 2, row: 2 },
-};
-const TABLET_CELLS_OF_THE_REFERENCE = {
-  learner: { column: 0, row: 0 },
-  q0: { column: 1, row: 0 },
-  intro: { column: 0, row: 1 },
-  q1: { column: 1, row: 1 },
-  board: { column: 0, row: 2 },
-  q2: { column: 0, row: 3 },
-  badges: { column: 1, row: 3 },
-  q3: { column: 0, row: 4 },
-  prefs: { column: 1, row: 4 },
-};
+const PAGES = ["learner", "quizzes", "board", "badges", "prefs"];
+const DESKTOP_CELLS_OF_THE_REFERENCE = Object.fromEntries(PAGES.map((page, index) => [page, { column: index % 3, row: Math.floor(index / 3) }]));
+const TABLET_CELLS_OF_THE_REFERENCE = Object.fromEntries(PAGES.map((page, index) => [page, { column: index % 2, row: Math.floor(index / 2) }]));

```

### 15 declared

```diff
-/** 🎨️ Every declaration of `.quiz-home-grid` and `.quiz-home-cell` with the minimum viewport width it applies from. */
+/** 🎨️ Every declaration of `.course-home-grid` and `.course-home-cell` with the minimum viewport width it applies from. */

```

### 17 layerRules

```diff
-        if (!name.startsWith(".quiz-home-grid") && !name.startsWith(".quiz-home-cell")) continue;
+        if (!name.startsWith(".course-home-grid") && !name.startsWith(".course-home-cell")) continue;

```

### 19 layerAt

```diff
-    if (width < rule.minWidth || rule.selector !== ".quiz-home-grid") continue;
+    if (width < rule.minWidth || rule.selector !== ".course-home-grid") continue;

```

### 21 atViewport

```diff
-function Home(props: { readonly session: ReturnType<typeof stubSession>; readonly initial?: CourseStep; readonly preferences?: CoursePreferences }): ReactElement {
+function Home(props: { readonly session: ReturnType<typeof stubSession>; readonly initial?: CourseStep; readonly preferences?: CoursePreferences; readonly course?: CourseView }): ReactElement {

```

### 23 Home

```diff
-  return <HomeScreen session={session as unknown as CourseSession} state={{ ...STATE, step }} text={courseText("en")} locale="en" preferences={props.preferences ?? PREFERENCES} onPreferences={() => undefined} />;
+  return <HomeScreen session={session as unknown as CourseSession} state={{ ...STATE, step, course: props.course ?? COURSE }} text={courseText("en")} locale="en" preferences={props.preferences ?? PREFERENCES} onPreferences={() => undefined} />;

```

### 25 moveMouseOver

```diff
-  const root = document.querySelector<HTMLElement>("[data-layered-overview]")!;
+  const root = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview]')!;

```

### 27 cards

```diff
-  return [...document.querySelectorAll<HTMLElement>(".quiz-home-grid section[data-card]")];
+  return [...document.querySelectorAll<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid section[data-card]')];

```

### 29 cardHost

```diff
+  it.each([375, 900])("brings each guided introduction card into the swipe viewport at %s px", (width) => {
+    atViewport(width, 300);
+    const props = { session: stubSession() as unknown as CourseSession, state: STATE, text: courseText("en"), locale: "en" as const, preferences: PREFERENCES, onPreferences: () => undefined };
+    const { rerender } = render(<HomeScreen {...props} introductionPage="board" />);
+    expect(document.querySelector('[data-grid-active] > [data-layered-overview]')?.getAttribute("data-layered-resting")).toBe("board");
+    rerender(<HomeScreen {...props} introductionPage="prefs" />);
+    expect(document.querySelector('[data-grid-active] > [data-layered-overview]')?.getAttribute("data-layered-resting")).toBe("prefs");
+    for (const page of PAGES) expect(document.querySelector(`[data-element-alias="course.home.${page}"]`)).not.toBeNull();
+  });
+  for (const vector of followedLayouts.layouts) it(`mirrors the peer's ${vector.grid} ${vector.mode} layout and settled ${vector.swipe} page from a desktop observer`, () => {
+    atViewport(1440, 1200);
+    const captured = vi.fn();
+    const noop = () => undefined;
+    const session = stubSession() as unknown as CourseSession;
+    const presentation = { layout: { grid: vector.grid as HomeGrid, mode: vector.mode as LayeredMode }, swipe: vector.swipe };
+    render(<FollowProvider following="alice" presentation={presentation} onCamera={noop} onRevealed={noop} onSubmission={noop} onSort={noop} onTaskPage={noop} onLayout={captured} onSwipe={captured}><HomeScreen session={session} state={STATE} text={courseText("en")} locale="en" preferences={PREFERENCES} onPreferences={noop} /></FollowProvider>);
+    const root = document.querySelector<HTMLElement>('[data-course-grid="root"] > [data-layered-overview]')!;
+    expect(root.parentElement?.hasAttribute("data-grid-active")).toBe(true);
+    expect(document.querySelectorAll("[data-grid-active]")).toHaveLength(1);
+    expect(root.hasAttribute("data-interactive")).toBe(false);
+    expect(root.dataset.mode).toBe(vector.mode);
+    const columns = vector.grid === "tablet" ? 2 : 3;
+    const rows = Math.ceil(PAGES.length / columns);
+    if (vector.mode === "swipe") {
+      const offset = /translate3d\(([^%]+)%, ([^%]+)%, 0\)/u.exec(root.querySelector<HTMLElement>(":scope > [data-layered-strip]")!.style.transform)!;
+      expect(-Number(offset[1]) * columns / 100).toBeCloseTo(vector.column);
+      expect(-Number(offset[2]) * rows / 100).toBeCloseTo(vector.row);
+      const overlay = root.querySelector<HTMLElement>(":scope > [data-layered-overlay]")!;
+      const cells = [...overlay.querySelectorAll<HTMLElement>(":scope > [data-layered-cell]")];
+      expect(cells).toHaveLength(PAGES.length);
+      expect([...new Set(cells.map((cell) => Number(cell.style.gridColumn) - 1))].sort()).toEqual(Array.from({ length: columns }, (_, index) => index));
+      expect([...new Set(cells.map((cell) => Number(cell.style.gridRow) - 1))].sort()).toEqual(Array.from({ length: rows }, (_, index) => index));
+    }
+    expect(captured).not.toHaveBeenCalled();
+  });
+  it("replays language-agnostic hierarchy children, parents and addresses", () => {
+    const course = { ...COURSE, quizzes: COURSE.quizzes.map((quiz, index) => ({ ...quiz, id: `q${index}` })), topics: fixture.hierarchy.topics };
+    for (const vector of fixture.hierarchy.scopes) expect(pagesWithin(vector.page ?? undefined, course), vector.page ?? "root").toEqual(vector.pages);
+    for (const vector of fixture.hierarchy.parents) expect(parentPage(vector.page, course), vector.page).toBe(vector.parent ?? undefined);
+    expect(addressPages(course)).toEqual([...PAGES, "energy", "parametric", "q0", "q1", "q2", "q3"]);
+    expect(pagesWithin("quizzes", COURSE)).toEqual(QUIZZES);
+    expect(parentPage("physics", COURSE)).toBe("quizzes");
+  });
+
+  it("keeps the active grid free of an inherited camera and gives every grid behind it that pointer", () => {
+    const pointer = { x: 0.25, y: 0.75 };
+    const followed = { x: 1, y: 0 };
+    const cases = [
+      { active: true, followed: undefined, inherited: pointer, expected: undefined },
+      { active: true, followed, inherited: pointer, expected: followed },
+      { active: false, followed: undefined, inherited: pointer, expected: pointer },
+      { active: false, followed, inherited: undefined, expected: undefined },
+    ] as const;
+    for (const item of cases) {
+      const actual = previewCamera(item.active, item.followed, item.inherited);
+      const oracle = produce({ camera: item.active ? item.followed : item.inherited }, (draft) => draft);
+      expect(actual).toEqual(item.expected);
+      expect(actual).toEqual(oracle.camera);
+    }
+  });
+
+  it("opens nested topic grids while keeping parents mounted and empty topics as accessible placeholders", async () => {
+    const session = stubSession();
+    const course = { ...COURSE, topics: fixture.hierarchy.topics!.map((topic) => ({ ...topic, quizzes: topic.id === "energy" ? [...QUIZZES] : [] })) };
+    render(<Home session={session} course={course} />);
+    const user = userEvent.setup();
+    await user.click(within(screen.getByRole("region", { name: "Quizzes" })).getByRole("button", { name: "Explore" }));
+    expect(cards().map((card) => card.dataset.card)).toEqual(["topic:energy", "topic:parametric"]);
+    expect(pane("quizzes").hasAttribute("data-opened")).toBe(true);
+    await user.click(within(screen.getByRole("region", { name: "Energy" })).getByRole("button", { name: "Explore" }));
+    expect(cards().map((card) => card.dataset.card)).toEqual(QUIZZES.map((id) => `quiz:${id}`));
+    expect(pane("quizzes").hasAttribute("data-opened")).toBe(true);
+    expect(pane("energy").hasAttribute("data-opened")).toBe(true);
+    await user.keyboard("{Escape}");
+    expect(session.open).toHaveBeenLastCalledWith({ screen: "home", page: "quizzes" });
+    await user.click(within(screen.getByRole("region", { name: "Parametric Design" })).getByRole("button", { name: "Explore" }));
+    expect(pane("parametric").hasAttribute("data-opened")).toBe(true);
+    expect(within(pane("parametric")).getByText("Quizzes for this topic are coming soon.")).toBeTruthy();
+    expect(within(pane("parametric")).queryByRole("button")).toBeNull();
+    expect(session.startRun).not.toHaveBeenCalled();
+  });
+

```

### 31 cardHost

```diff
-      expect(homePages(order.quizzes)).toEqual(order.pages);
+      expect(homePages()).toEqual(order.pages);

```

### 33 cardHost

```diff
-  it("keeps the strip cells of the nine pages those of the reference layout", () => {
-    const nine = homePages(["q0", "q1", "q2", "q3"]);
-    expect(homeCells(nine, "desktop")).toEqual(DESKTOP_CELLS_OF_THE_REFERENCE);
-    expect(homeCells(nine, "tablet")).toEqual(TABLET_CELLS_OF_THE_REFERENCE);
+  it("keeps the strip cells of the five root pages those of the reference layout", () => {
+    const pages = homePages();
+    expect(homeCells(pages, "desktop")).toEqual(DESKTOP_CELLS_OF_THE_REFERENCE);
+    expect(homeCells(pages, "tablet")).toEqual(TABLET_CELLS_OF_THE_REFERENCE);

```

### 35 cardHost

```diff
-  it("lays nine named card sections in reading order over their nine pages, the leaderboard fifth, each heading linking its page", () => {
+  it("lays five named card sections in reading order over their immediate pages, each heading linking its page", () => {

```

### 37 cardHost

```diff
-    expect(shown.map((card) => card.getAttribute("data-card"))).toEqual(["learner", "quiz:physics", "introduction", "quiz:heating", "board", "quiz:cooling", "badges", "quiz:demand", "preferences"]);
+    expect(shown.map((card) => card.getAttribute("data-card"))).toEqual(["learner", "quizzes", "board", "badges", "preferences"]);

```

### 39 cardHost

```diff
-    expect(names).toEqual(["Ada", "Quiz physics", "How it works", "Quiz heating", "Leaderboard", "Quiz cooling", "Badges", "Quiz demand", "Settings"]);
+    expect(names).toEqual(["Ada", "Quizzes", "Leaderboard", "Badges", "Settings"]);

```

### 41 cardHost

```diff
-    expect([...document.querySelectorAll("[data-layered-pane]")].map((element) => element.getAttribute("data-layered-pane"))).toEqual(PAGES);
+    expect([...document.querySelectorAll('[data-grid-active] > [data-layered-overview] > [data-layered-strip] > [data-layered-pane]')].map((element) => element.getAttribute("data-layered-pane"))).toEqual(PAGES);

```

### 43 cardHost

```diff
-    expect(screen.getAllByRole("heading", { level: 1 }).map((heading) => heading.textContent)).toEqual(["Quizzes"]);
-    expect(screen.getAllByRole("heading", { level: 2 })).toHaveLength(9);
+    expect(screen.getAllByRole("heading", { level: 1 }).map((heading) => heading.textContent)).toEqual(["Course"]);
+    expect(screen.getAllByRole("heading", { level: 2 })).toHaveLength(5);

```

### 45 cardHost

```diff
-    render(<Home session={stubSession()} />);
+    render(<Home initial={{ screen: "home", page: "quizzes" }} session={stubSession()} />);

```

### 47 cardHost

```diff
+    cleanup();
+    render(<Home session={stubSession()} />);

```

### 49 cardHost

```diff
-    expect(session.open).toHaveBeenLastCalledWith({ screen: "home" });
-    expect(cards()).toHaveLength(9);
+    expect(session.open).toHaveBeenLastCalledWith({ screen: "home", page: "quizzes" });
+    expect(cards()).toHaveLength(4);

```

### 51 cardHost

```diff
-  it("shows a quiz's page clear behind its hovered or focused card and never starts or resumes a run for it", async () => {
+  it("shows a quiz's page clear behind its hovered or focused card and never starts or resumes a run for it", () => {

```

### 53 cardHost

```diff
-    const user = userEvent.setup();
-    render(<Home session={session} />);
+    render(<Home initial={{ screen: "home", page: "quizzes" }} session={session} />);

```

### 55 cardHost

```diff
-    const overlay = [...document.querySelectorAll<HTMLElement>(".quiz-home-grid a[href], .quiz-home-grid button")];
-    overlay[overlay.indexOf(link) - 1]!.focus();
-    await user.tab();
+    act(() => link.focus());

```

### 57 cardHost

```diff
-    const { unmount } = render(<Home session={session} />);
+    const { unmount } = render(<Home initial={{ screen: "home", page: "quizzes" }} session={session} />);

```

### 59 cardHost

```diff
+    fireEvent.keyDown(window, { key: "Escape" });

```

### 61 cardHost

```diff
+    expect(pane("board").hasAttribute("data-opened")).toBe(true);

```

### 63 cardHost

```diff
-    const root = document.querySelector<HTMLElement>("[data-layered-overview]")!;
+    const root = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview]')!;

```

### 65 cardHost

```diff
-    expect([strip.style.width, strip.style.height, strip.style.transform]).toEqual(["300%", "300%", "translate3d(0%, 0%, 0)"]);
-    expect(root.querySelector<HTMLElement>("[data-layered-veil]")!.dataset.veil).toBe("whole");
+    expect([strip.style.width, strip.style.height, strip.style.transform]).toEqual(["300%", "200%", "translate3d(0%, 0%, 0)"]);
+    expect(root.querySelector<HTMLElement>(":scope > [data-layered-veil]")!.dataset.veil).toBe("whole");

```

### 67 cardHost

```diff
-      const cell = cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
+      const cell = cardHost(page).querySelector<HTMLElement>(".course-home-cell")!;

```

### 69 cardHost

```diff
-    const overlay = document.querySelector<HTMLElement>(".quiz-home-grid")!;
-    expect(weights(overlay.style.getPropertyValue("--quiz-home-columns"))).toEqual(desktop.columns);
-    expect(weights(overlay.style.getPropertyValue("--quiz-home-rows"))).toEqual(desktop.rows);
+    const overlay = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid')!;
+    expect(weights(overlay.style.getPropertyValue("--course-home-columns"))).toEqual(desktop.columns);
+    expect(weights(overlay.style.getPropertyValue("--course-home-rows"))).toEqual(desktop.rows);

```

### 71 cardHost

```diff
-    const strip = document.querySelector<HTMLElement>("[data-layered-strip]")!;
-    const veil = document.querySelector<HTMLElement>("[data-layered-veil]")!;
+    const strip = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-strip]')!;
+    const veil = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-veil]')!;

```

### 73 cardHost

```diff
-    expect(strip.style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
+    expect(strip.style.transform).toBe("translate3d(-33.3333%, -50%, 0)");

```

### 75 cardHost

```diff
-    const card = screen.getByRole("region", { name: "Quiz heating" });
+    const card = screen.getByRole("region", { name: "Leaderboard" });

```

### 77 cardHost

```diff
-    expect(strip.style.transform).toBe("translate3d(0%, -33.3333%, 0)");
+    expect(strip.style.transform).toBe("translate3d(-66.6667%, 0%, 0)");

```

### 79 cardHost

```diff
-    expect(strip.style.transform).toBe("translate3d(0%, -33.3333%, 0)");
+    expect(strip.style.transform).toBe("translate3d(-66.6667%, 0%, 0)");

```

### 81 cardHost

```diff
-    expect(strip.style.transform).toBe("translate3d(0%, -33.3333%, 0)");
+    expect(strip.style.transform).toBe("translate3d(-66.6667%, 0%, 0)");

```

### 83 cardHost

```diff
-    expect(strip.style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
+    expect(strip.style.transform).toBe("translate3d(-33.3333%, -50%, 0)");

```

### 85 cardHost

```diff
+  });
+
+  /** 🧭 The strip directly under one overview. */
+  function gridStrip(id: string): HTMLElement {
+    return document.querySelector<HTMLElement>(`[data-course-grid="${id}"] > [data-layered-overview] > [data-layered-strip]`)!;
+  }
+
+  /** 🪟 The glass directly under one overview. */
+  function gridVeil(id: string): HTMLElement {
+    return document.querySelector<HTMLElement>(`[data-course-grid="${id}"] > [data-layered-overview] > [data-layered-veil]`)!;
+  }
+
+  it("zooms a hovered topic and pans the quizzes behind its cards without zooming those quizzes", () => {
+    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
+    const topics = fixture.hierarchy.topics!.map((topic) => ({ ...topic, quizzes: topic.id === "energy" ? ["physics", "heating", "cooling", "demand", "storage", "network"] : [] }));
+    render(<Home session={stubSession()} course={{ ...COURSE, topics }} initial={{ screen: "home", page: "quizzes" }} />);
+    moveMouseOver(1, 1);
+    expect(gridStrip("quizzes").style.transform).toBe("translate3d(-50%, 0%, 0)");
+    expect(gridStrip("energy").style.transform).toBe("translate3d(-66.6667%, -50%, 0)");
+    expect(gridVeil("energy").dataset.veil).toBe("whole");
+    fireEvent.pointerOver(screen.getByRole("region", { name: "Energy" }), { pointerType: "mouse" });
+    for (let spent = 0; spent < 600; spent += 10) act(() => void vi.advanceTimersByTime(10));
+    expect(gridStrip("quizzes").style.transform).toBe("translate3d(0%, 0%, 0)");
+    expect(gridVeil("quizzes").dataset.veil).toBe("clear");
+    expect(gridStrip("energy").style.transform).toBe("translate3d(-66.6667%, -50%, 0)");
+    moveMouseOver(0, 0);
+    expect(gridStrip("quizzes").style.transform).toBe("translate3d(0%, 0%, 0)");
+    expect(gridVeil("quizzes").dataset.veil).toBe("clear");
+    expect(gridStrip("energy").style.transform).toBe("translate3d(0%, 0%, 0)");
+    expect(gridVeil("energy").dataset.veil).toBe("whole");
+    for (const id of ["physics", "heating", "cooling", "demand", "storage", "network"]) expect(document.querySelector(`[data-course-grid="energy"] [data-layered-card="${id}"]`)!.hasAttribute("data-revealed"), id).toBe(false);
+  });
+
+  it("pans every grid behind a zoomed overview without zooming those deeper cards", () => {
+    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
+    const topics = fixture.hierarchy.topics!.map((topic) => ({ ...topic, quizzes: topic.id === "energy" ? ["physics", "heating", "cooling", "demand", "storage", "network"] : [] }));
+    render(<Home session={stubSession()} course={{ ...COURSE, topics }} />);
+    fireEvent.pointerOver(screen.getByRole("region", { name: "Quizzes" }), { pointerType: "mouse" });
+    for (let spent = 0; spent < 600; spent += 10) act(() => void vi.advanceTimersByTime(10));
+    expect(gridStrip("root").style.transform).toBe("translate3d(-33.3333%, 0%, 0)");
+    expect(gridVeil("root").dataset.veil).toBe("clear");
+    moveMouseOver(1, 1);
+    expect(gridStrip("root").style.transform).toBe("translate3d(-33.3333%, 0%, 0)");
+    expect(gridVeil("root").dataset.veil).toBe("clear");
+    expect(gridStrip("quizzes").style.transform).toBe("translate3d(-50%, 0%, 0)");
+    expect(gridVeil("quizzes").dataset.veil).toBe("whole");
+    expect(gridStrip("energy").style.transform).toBe("translate3d(-66.6667%, -50%, 0)");
+    expect(gridVeil("energy").dataset.veil).toBe("whole");
+    expect(document.querySelector('[data-layered-card="energy"]')!.hasAttribute("data-revealed")).toBe(false);
+    for (const id of ["physics", "heating", "cooling", "demand", "storage", "network"]) expect(document.querySelector(`[data-layered-card="${id}"]`)!.hasAttribute("data-revealed"), id).toBe(false);

```

### 87 cardHost

```diff
-    expect(document.querySelector<HTMLElement>("[data-layered-overview]")!.dataset.pan).toBe("pointer");
-    const strip = document.querySelector<HTMLElement>("[data-layered-strip]")!;
-    const veil = document.querySelector<HTMLElement>("[data-layered-veil]")!;
+    expect(document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview]')!.dataset.pan).toBe("pointer");
+    const strip = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-strip]')!;
+    const veil = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-veil]')!;

```

### 89 cardHost

```diff
-    expect(strip.style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
-    fireEvent.pointerOver(screen.getByRole("region", { name: "Leaderboard" }), { pointerType: "mouse" });
-    expect(strip.style.transform, "the glide starts where the pan stood").toBe("translate3d(-66.6667%, -66.6667%, 0)");
+    expect(strip.style.transform).toBe("translate3d(-33.3333%, -50%, 0)");
+    fireEvent.pointerOver(screen.getByRole("region", { name: "Badges" }), { pointerType: "mouse" });
+    expect(strip.style.transform, "the glide starts where the pan stood").toBe("translate3d(-33.3333%, -50%, 0)");

```

### 91 cardHost

```diff
-    expect(strip.style.transform).toBe("translate3d(-33.3333%, -33.3333%, 0)");
+    expect(strip.style.transform).toBe("translate3d(0%, -50%, 0)");

```

### 93 cardHost

```diff
+      const anchorElement = pane(page).querySelector<HTMLElement>(`[data-presence-anchor="${anchor}"]`)!;
+      vi.spyOn(anchorElement, "getBoundingClientRect").mockReturnValue({ left: 0, top: 0, right: 200, bottom: 100, width: 200, height: 100, x: 0, y: 0, toJSON: () => ({}) });
+      const layer = mark.closest<HTMLElement>("[data-pane-peers]")!;
+      placePeers(layer, layer.parentElement!);

```

### 95 cardHost

```diff
+      expect(mark.style.transform, page).toBe("translate(100px, 50px)");

```

### 97 cardHost

```diff
-    for (const page of PAGES.filter((candidate) => candidate !== "board" && candidate !== "heating")) expect(pane(page).querySelector("[data-pane-peers]"), page).toBeNull();
+    for (const page of PAGES.filter((candidate) => candidate !== "board" && candidate !== "quizzes")) expect(pane(page).querySelector("[data-pane-peers]"), page).toBeNull();

```

### 99 cardHost

```diff
-              if (name === "quiz-nowrap" || name === "quiz-name") declarations.set(name, Object.fromEntries(rule.value.declarations.declarations.map((declaration) => [declaration.property, declaration.value])));
+              if (name === "course-nowrap" || name === "course-name") declarations.set(name, Object.fromEntries(rule.value.declarations.declarations.map((declaration) => [declaration.property, declaration.value])));

```

### 101 cardHost

```diff
-    expect(declarations.get("quiz-nowrap")).toEqual({ "white-space": "nowrap", hyphens: "none", "overflow-wrap": "normal" });
-    expect(declarations.get("quiz-name")).toEqual({ hyphens: "none", "overflow-wrap": "anywhere" });
+    expect(declarations.get("course-nowrap")).toEqual({ "white-space": "nowrap", hyphens: "none", "overflow-wrap": "normal" });
+    expect(declarations.get("course-name")).toEqual({ hyphens: "none", "overflow-wrap": "anywhere" });

```

### 103 cardHost

```diff
-      for (const head of heads) expect(head.classList.contains("quiz-nowrap"), head.textContent ?? "").toBe(true);
+      for (const head of heads) expect(head.classList.contains("course-nowrap"), head.textContent ?? "").toBe(true);

```

### 105 cardHost

```diff
-        expect(name.classList.contains("quiz-name")).toBe(true);
-        expect(name.classList.contains("quiz-nowrap")).toBe(false);
+        expect(name.classList.contains("course-name")).toBe(true);
+        expect(name.classList.contains("course-nowrap")).toBe(false);

```

### 107 cardHost

```diff
-        const holder = cell.classList.contains("quiz-nowrap") ? cell : cell.querySelector(".quiz-nowrap");
+        const holder = cell.classList.contains("course-nowrap") ? cell : cell.querySelector(".course-nowrap");

```

### 109 cardHost

```diff
-    const glyph = container.querySelector(".quiz-glyph")!;
+    const glyph = container.querySelector(".course-glyph")!;

```

### 111 cardHost

```diff
-    expect(stylesheet).toMatch(/\.quiz-glyph \{[^}]*font-variant-emoji: text;/u);
+    expect(stylesheet).toMatch(/\.course-glyph \{[^}]*font-variant-emoji: text;/u);

```

### 113 cardHost

```diff
-    expect([...new Set(rules.filter((rule) => rule.selector === ".quiz-home-grid").map((rule) => rule.minWidth))]).toEqual([0]);
-    expect(rules.filter((rule) => rule.selector.startsWith(".quiz-home-grid") && SPACING.includes(rule.property))).toEqual([]);
-    expect(rules.filter((rule) => rule.selector === ".quiz-home-cell").map((rule) => rule.property)).toEqual(expect.arrayContaining(["padding", "align-items", "justify-content", "min-width", "min-height"]));
+    expect([...new Set(rules.filter((rule) => rule.selector === ".course-home-grid").map((rule) => rule.minWidth))]).toEqual([0]);
+    expect(rules.filter((rule) => rule.selector.startsWith(".course-home-grid") && SPACING.includes(rule.property))).toEqual([]);
+    expect(rules.filter((rule) => rule.selector === ".course-home-cell").map((rule) => rule.property)).toEqual(expect.arrayContaining(["padding", "align-items", "justify-content", "min-width", "min-height"]));

```

### 115 cardHost

```diff
-      expect(document.querySelector("[data-layered-overview]")!.getAttribute("data-mode")).toBe(expected.mode);
+      expect(document.querySelector('[data-grid-active] > [data-layered-overview]')!.getAttribute("data-mode")).toBe(expected.mode);

```

### 117 cardHost

```diff
-        expect(document.querySelector(".quiz-home-grid")).toBeNull();
-        expect(document.querySelectorAll(".quiz-home-cell")).toHaveLength(0);
+        expect(document.querySelector('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid')).toBeNull();
+        expect(document.querySelectorAll('[data-grid-active] > [data-layered-overview] > [data-layered-overlay] .course-home-cell')).toHaveLength(0);

```

### 119 cardHost

```diff
-          expect(cell.querySelector("[data-layered-card] .quiz-home-entry"), page).not.toBeNull();
+          expect(cell.querySelector("[data-layered-card] .course-home-entry"), page).not.toBeNull();

```

### 121 cardHost

```diff
-      expect(layerAt(layerRules(stylesheet), expected.width)).toEqual({ columns: "var(--quiz-home-columns)", rows: "var(--quiz-home-rows)" });
-      const overlay = document.querySelector<HTMLElement>(".quiz-home-grid")!;
-      expect(weights(overlay.style.getPropertyValue("--quiz-home-columns"))).toEqual(expected.columns);
-      expect(weights(overlay.style.getPropertyValue("--quiz-home-rows"))).toEqual(expected.rows);
+      expect(layerAt(layerRules(stylesheet), expected.width)).toEqual({ columns: "var(--course-home-columns)", rows: "var(--course-home-rows)" });
+      const overlay = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid')!;
+      expect(weights(overlay.style.getPropertyValue("--course-home-columns"))).toEqual(expected.columns);
+      expect(weights(overlay.style.getPropertyValue("--course-home-rows"))).toEqual(expected.rows);

```

### 123 cardHost

```diff
-        const cell = cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
+        const cell = cardHost(page).querySelector<HTMLElement>(".course-home-cell")!;

```

### 125 Choosing

```diff
-    render(<Home session={session} preferences={{ ...PREFERENCES, challenges: { cooling: "hard", heating: "expert" } }} />);
+    render(<Home initial={{ screen: "home", page: "quizzes" }} session={session} preferences={{ ...PREFERENCES, challenges: { cooling: "hard", heating: "expert" } }} />);

```

### 127 Choosing

```diff
-      render(<HomeScreen session={stubSession() as unknown as CourseSession} state={{ ...STATE, learnerView: longest }} text={courseText(locale)} locale={locale} preferences={{ ...PREFERENCES, challenges: Object.fromEntries(QUIZZES.map((quiz) => [quiz, "expert" as const])) }} onPreferences={() => undefined} />);
+      render(<HomeScreen session={stubSession() as unknown as CourseSession} state={{ ...STATE, step: { screen: "home", page: "quizzes" }, learnerView: longest }} text={courseText(locale)} locale={locale} preferences={{ ...PREFERENCES, challenges: Object.fromEntries(QUIZZES.map((quiz) => [quiz, "expert" as const])) }} onPreferences={() => undefined} />);

```

### 129 Choosing

```diff
-        const card = document.querySelector<HTMLElement>(`.quiz-home-grid section[data-card="quiz:${quiz}"]`)!;
+        const card = document.querySelector<HTMLElement>(`.course-home-grid section[data-card="quiz:${quiz}"]`)!;

```

### 131 Choosing

```diff
-      .map((button) => `${button.textContent}${button.getAttribute("aria-pressed") === "true" ? " ✓" : ""}`);
+      .map((button) => `${button.textContent?.trim()}${button.getAttribute("aria-pressed") === "true" ? " ✓" : ""}`);

```

### 133 Choosing

```diff
-    expect(screen.getByRole("status").textContent).toBe("Loading from the quiz server…");
+    expect(screen.getByRole("status").textContent).toBe("Loading from the course server…");

```

### 135 Choosing

```diff
-    expect(within(region).getByText("Loading from the quiz server…")).toBeTruthy();
+    expect(within(region).getByText("Loading from the course server…")).toBeTruthy();

```

### 137 Choosing

```diff
-    expect(minimum.tablet).toBe(chromeHeightPx + rowHeightPx * 5);
+    expect(minimum.tablet).toBe(chromeHeightPx + rowHeightPx * 3);

```

### 139 Choosing

```diff
-    const cell = (property: string): string => new RegExp(`\\.quiz-home-cell \\{[^}]*\\b${property}: ([^;]+);`, "u").exec(stylesheet)?.[1] ?? "";
+    const cell = (property: string): string => new RegExp(`\\.course-home-cell \\{[^}]*\\b${property}: ([^;]+);`, "u").exec(stylesheet)?.[1] ?? "";

```

### 141 Choosing

```diff
-    expect(homeLayoutQueries(1.5, PAGES)).toEqual({ narrow: "(max-width: 1150px)", medium: "(min-width: 1151px) and (max-width: 1534px)", short: { desktop: "(max-height: 899px)", tablet: "(max-height: 1283px)" } });
+    expect(homeLayoutQueries(1.5, PAGES)).toEqual({ narrow: "(max-width: 1150px)", medium: "(min-width: 1151px) and (max-width: 1534px)", short: { desktop: "(max-height: 563px)", tablet: "(max-height: 803px)" } });

```

### 143 Choosing

```diff
-      const overview = document.querySelector("[data-layered-overview]")!;
+      const overview = document.querySelector('[data-grid-active] > [data-layered-overview]')!;

```

### 145 Choosing

```diff
-        const cell = mode === "swipe" ? document.querySelector<HTMLElement>(`[data-layered-cell="${page}"]`)! : cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
+        const cell = mode === "swipe" ? document.querySelector<HTMLElement>(`[data-layered-cell="${page}"]`)! : cardHost(page).querySelector<HTMLElement>(".course-home-cell")!;

```

### 147 Choosing

```diff
-    const overview = document.querySelector<HTMLElement>("[data-layered-overview]")!;
+    const overview = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview]')!;

```

### 149 Choosing

```diff
-    expect(within(overview).queryByRole("button", { name: "Overview" })).toBeNull();
+    expect(within(overview).queryByRole("button", { name: "Course overview" })).toBeNull();

```

### 151 Choosing

```diff
-    for (const [locale, right, down] of [["en", "Go right to Quiz physics", "Go down to Quiz heating"], ["de", "Nach rechts zu Quiz physics", "Nach unten zu Quiz heating"]] as const) {
+    for (const [locale, right, down] of [["en", "Go right to Quizzes", "Go down to Badges"], ["de", "Nach rechts zu Quizze", "Nach unten zu Abzeichen"]] as const) {

```

### 153 Choosing

```diff
-      const hints = [...document.querySelectorAll<HTMLElement>("[data-layered-neighbour]")].map((hint) => [hint.dataset.layeredNeighbour, hint.dataset.pane, hint.getAttribute("aria-label"), hint.textContent?.trim()]);
-      expect(hints.map(([direction, page]) => [direction, page]), `${locale}: the ring wraps, so the badges lie above the learner and how it works on the left`).toEqual([
+      const hints = [...document.querySelectorAll<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-neighbour]')].map((hint) => [hint.dataset.layeredNeighbour, hint.dataset.pane, hint.getAttribute("aria-label"), hint.textContent?.trim()]);
+      expect(hints.map(([direction, page]) => [direction, page]), `${locale}: the ring wraps, so badges lie above the learner and the leaderboard on the left`).toEqual([

```

### 155 Choosing

```diff
-        ["left", "intro"],
-        ["right", "physics"],
-        ["down", "heating"],
+        ["left", "board"],
+        ["right", "quizzes"],
+        ["down", "badges"],

```

### 157 Choosing

```diff
-        ["right", "physics", right, "Quiz physics"],
-        ["down", "heating", down, "Quiz heating"],
+        ["right", "quizzes", right, locale === "en" ? "Quizzes" : "Quizze"],
+        ["down", "badges", down, locale === "en" ? "Badges" : "Abzeichen"],

```

### 159 Choosing

```diff
-    const layers = [...document.querySelectorAll<HTMLElement>("[data-layered-cell]")].filter((layer) => layer.querySelector("[data-layered-card]") !== null);
+    const layers = [...document.querySelectorAll<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-overlay] > [data-layered-cell]')].filter((layer) => layer.querySelector("[data-layered-card]") !== null);

```

## 96 🧰️framework/🛍️products/❓️quiz/🧪️tests/🐾️pet-companions/🟦️.tsx

### 0 

```diff
-/** 🐾️ Pet companions: the learner's choice between no pets, motionless, calm (unless chosen otherwise) and lively ones is
- * kept on the device and offered in both languages, and a switch on the footer of every screen hides the pets and
- * brings them back as lively as they were; a device that asks for reduced motion decides the default only — its learner
- * gets motionless pets and is told why until they choose, and what a learner chose in the preferences or with the
- * switch holds whatever the device asks for, which is why that the learner chose is stored as a fact of its own;
+/** 🐾️ Pet companions: the learner's choice between no pets, motionless, calm and lively ones is lively until they
+ * choose otherwise, kept on the device and offered in both languages, and a switch on the footer of every screen hides
+ * the pets and brings them back as lively as they were; a device that asks for reduced motion does not hold them still,
+ * and what a learner chose in the preferences or with the switch is stored as a fact of its own;

```

### 2 

```diff
+import find from "lodash/find";
+import flatMap from "lodash/flatMap";

```

### 4 

```diff
-import type { CourseView, Query, RunResult, RunView, Sheet, SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/course";
+import type { CourseView, LearnerView, Query, RunResult, RunView, Sheet, SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/course";

```

### 6 

```diff
+  evolveCourseState,

```

### 8 

```diff
+  shinyPetSpecies,

```

### 10 

```diff
+  usePetQuestion,

```

### 12 

```diff
-import sheets from "../🧫️fixtures/🃏️sheet-assembly/🔣️.json";
+import sheets from "../../🧫️fixtures/🃏️sheet-assembly/🔣️.json";
+import contexts from "../../🧫️fixtures/🐾️pet-contexts/🔣️.json";
+import shinyCases from "../../🧫️fixtures/✨️shiny-pets/🔣️.json";

```

### 14 species

```diff
-  introduction: { title: text("Welcome", "Willkommen"), paragraphs: [] },
+  introduction: { title: text("Welcome", "Willkommen"), paragraphs: [text("Explore quiz topics with the pet companions.", "Entdecke Quizthemen mit den Tierchen.")] },

```

### 16 courseProctor

```diff
-function at(step: CourseStep): Pick<CourseState, "step" | "runs" | "course"> {
-  return { step, runs: RUNS, course: COURSE };
+function at(step: CourseStep): Pick<CourseState, "step" | "runs" | "course" | "awards"> {
+  return { step, runs: RUNS, course: COURSE, awards: {} };

```

### 18 named

```diff
-    it("is calm unless the learner chose otherwise, and stays on the device", () => {
+    it("is lively until the learner chooses otherwise, and stays on the device", () => {

```

### 20 named

```diff
-      expect(readPreferences(store).pets).toBe("calm");
+      expect(readPreferences(store).pets).toBe("lively");

```

### 22 named

```diff
-      expect(readPreferences(store)).toMatchObject({ theme: "dark", pets: "calm", petsLiveliness: "calm" });
+      expect(readPreferences(store)).toMatchObject({ theme: "dark", pets: "lively", petsLiveliness: "lively" });

```

### 24 named

```diff
-      expect(readPreferences(store).petsLiveliness).toBe("calm");
+      expect(readPreferences(store).petsLiveliness).toBe("lively");

```

### 26 named

```diff
-      expect(readPreferences(store)).toMatchObject({ pets: "off", petsLiveliness: "calm" });
+      expect(readPreferences(store)).toMatchObject({ pets: "off", petsLiveliness: "lively" });

```

### 28 named

```diff
-      expect(readPreferences(store)).toMatchObject({ pets: "calm", petsLiveliness: "calm", petsChosen: false });
+      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively", petsChosen: false });

```

### 30 named

```diff
-      expect(readPreferences(localStore(origin.tab(), "arch"))).toEqual({ locale: "de", theme: "dark", textSize: "large", showCursors: false, others: "always", animateIcons: false, iconsChosen: false, pets: "calm", petsLiveliness: "calm", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} });
+      expect(readPreferences(localStore(origin.tab(), "arch"))).toEqual({ locale: "de", theme: "dark", textSize: "large", showCursors: false, others: "always", animateIcons: false, iconsChosen: false, pets: "lively", petsLiveliness: "lively", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} });

```

### 32 named

```diff
-      expect(withPets(untouched, switchedPets(false, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "off", petsLiveliness: "calm", petsChosen: true });
-      expect(withPets(untouched, switchedPets(true, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "calm", petsLiveliness: "calm", petsChosen: true });
+      expect(withPets(untouched, switchedPets(false, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "off", petsLiveliness: "lively", petsChosen: true });
+      expect(withPets(untouched, switchedPets(true, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "lively", petsLiveliness: "lively", petsChosen: true });

```

### 34 named

```diff
-      expect(readPreferences(store)).toMatchObject({ pets: "calm", petsChosen: true });
+      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsChosen: true });

```

### 36 named

```diff
-        expect(buttons.map((button) => button.textContent)).toEqual(labels[locale].choices);
+        expect(buttons.map((button) => button.textContent?.trim())).toEqual(labels[locale].choices);

```

### 38 named

```diff
-        expect(courseText(locale)("quiz.preferences.petsShown")).toBe(names[locale]);
+        expect(courseText(locale)("course.preferences.petsShown")).toBe(names[locale]);

```

### 40 named

```diff
-    it("is what the learner chose whatever the device asks for, and until the learner chooses motionless on a device that asks for reduced motion", () => {
-      const table: readonly (readonly [choice: PetChoice, chosen: boolean, reducedMotion: boolean, mode: PetChoice])[] = [
-        ["off", false, false, "off"],
-        ["off", false, true, "off"],
-        ["off", true, false, "off"],
-        ["off", true, true, "off"],
-        ["still", false, false, "still"],
-        ["still", false, true, "still"],
-        ["still", true, false, "still"],
-        ["still", true, true, "still"],
-        ["calm", false, false, "calm"],
-        ["calm", false, true, "still"],
-        ["calm", true, false, "calm"],
-        ["calm", true, true, "calm"],
-        ["lively", false, false, "lively"],
-        ["lively", false, true, "still"],
-        ["lively", true, false, "lively"],
-        ["lively", true, true, "lively"],
+    it("is the learner's choice whatever the device asks for", () => {
+      const table: readonly (readonly [choice: PetChoice, chosen: boolean, reducedMotion: boolean])[] = [
+        ["off", false, false],
+        ["off", false, true],
+        ["off", true, false],
+        ["off", true, true],
+        ["still", false, false],
+        ["still", false, true],
+        ["still", true, false],
+        ["still", true, true],
+        ["calm", false, false],
+        ["calm", false, true],
+        ["calm", true, false],
+        ["calm", true, true],
+        ["lively", false, false],
+        ["lively", false, true],
+        ["lively", true, false],
+        ["lively", true, true],

```

### 42 named

```diff
-      for (const [choice, chosen, reducedMotion, mode] of table) expect(effectivePetMode(choice, chosen, reducedMotion), `${choice}, chosen ${chosen}, reduced motion ${reducedMotion}`).toBe(mode);
+      for (const [choice, chosen, reducedMotion] of table) expect(effectivePetMode(choice), `${choice}, chosen ${chosen}, reduced motion ${reducedMotion}`).toBe(choice);

```

### 44 named

```diff
-    it("follows the device's motion preference while the page is open until the learner chooses, and the learner's choice from then on", async () => {
+    it("keeps the learner's choice while the device's motion preference changes", async () => {

```

### 46 named

```diff
-      expect(layer()?.dataset.mode).toBe("still");
+      expect(layer()?.dataset.mode).toBe("lively");

```

### 48 named

```diff
-      expect(layer()?.dataset.mode).toBe("still");
-      expect(handed.every((props) => props.mode === "still" || props.mode === "lively")).toBe(true);
+      expect(layer()?.dataset.mode).toBe("lively");
+      expect(handed.every((props) => props.mode === "lively")).toBe(true);

```

### 50 named

```diff
+    it("narrows the committed contexts identically to independent course lookups", () => {
+      const course = { ...COURSE, topics: contexts.topics.map((id) => ({ id, emoji: "📚", title: text(id, id), quizzes: id === "energy" ? ["heating", "cooling"] : [] })) };
+      for (const vector of contexts.cases) {
+        const step = vector.step as CourseStep;
+        const question = "question" in vector ? vector.question : undefined;
+        expect(petScene(step, RUNS, course, question)).toBe(vector.expected);
+        const quiz = step.screen === "run" || step.screen === "results" ? RUNS[step.run]?.quiz : step.screen === "home" ? find(course.quizzes, { id: step.page })?.id : undefined;
+        const topic = step.screen === "home" ? find(course.topics, { id: step.page })?.id : undefined;
+        const oracle = quiz === undefined ? topic === undefined ? "home" : `topic-${topic}` : step.screen === "run" && question !== undefined ? `question-${quiz}-${question}` : quiz;
+        expect(petScene(step, RUNS, course, question)).toBe(oracle);
+      }
+    });
+

```

### 52 named

```diff
-      const steps: readonly CourseStep[] = [{ screen: "introduction" }, { screen: "identity" }, { screen: "home" }, { screen: "home", page: "learner" }, { screen: "home", page: "intro" }, { screen: "home", page: "board" }, { screen: "home", page: "badges" }, { screen: "home", page: "prefs" }, { screen: "home", page: "nowhere" }];
+      const steps: readonly CourseStep[] = [{ screen: "introduction" }, { screen: "identity" }, { screen: "home" }, { screen: "home", page: "learner" }, { screen: "home", page: "board" }, { screen: "home", page: "badges" }, { screen: "home", page: "prefs" }];

```

### 54 named

```diff
-    it("stays home until the course and the run are known", () => {
-      expect(petScene({ screen: "home", page: "heating" }, RUNS, undefined)).toBe("home");
-      expect(petScene({ screen: "run", run: "r9" }, RUNS, COURSE)).toBe("home");
-      expect(petScene({ screen: "results", run: "r9" }, {}, undefined)).toBe("home");
+    it("keeps unresolved pages and runs outside the general ensemble", () => {
+      expect(petScene({ screen: "home", page: "heating" }, RUNS, undefined)).toBe("page-heating");
+      expect(petScene({ screen: "home", page: "nowhere" }, RUNS, COURSE)).toBe("page-nowhere");
+      expect(petScene({ screen: "run", run: "r9" }, RUNS, COURSE)).toBe("run-r9");
+      expect(petScene({ screen: "results", run: "r9" }, {}, undefined)).toBe("run-r9");

```

### 56 named

```diff
+    it("restores a mounted owner's question after a followed question leaves", async () => {
+      function Question(props: { run: string; task: string }): null {
+        usePetQuestion(props.run, props.task);
+        return null;
+      }
+      const source = fakeSource();
+      const { stage, handed } = fakeStage();
+      const tree = (run: string, following: boolean): ReactElement => <Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run }}><><Question key="own" run="r1" task="systems" />{following ? <Question key="followed" run="r2" task="air" /> : null}</></Pets>;
+      const first = contexts.follow[0]!;
+      const { rerender } = render(tree(first.run, first.following));
+      await settle();
+      for (const vector of contexts.follow) {
+        rerender(tree(vector.run, vector.following));
+        expect(handed.at(-1)?.scene).toBe(vector.expected);
+        const oracle = find(contexts.cases, (candidate) => candidate.step.screen === "run" && "run" in candidate.step && candidate.step.run === vector.run && "question" in candidate && candidate.question !== undefined);
+        expect(handed.at(-1)?.scene).toBe(oracle?.expected);
+      }
+    });
+    it("tracks the actual question tabs of a run and returns to quiz scope on results", async () => {
+      const source = fakeSource();
+      const { stage, handed } = fakeStage();
+      const state = { ...homeAt({ screen: "run", run: OPEN_RUN.run }), runs: { [OPEN_RUN.run]: OPEN_RUN } };
+      const tree = (current: CourseState): ReactElement => <CoursePetsProvider source={source} stage={stage} choice="lively" chosen={true} play={true} mischief={true} state={current} locale="en">
+        {current.step.screen === "run" ? <RunScreen session={idleSession()} state={current} run={OPEN_RUN.run} text={courseText("en")} locale="en" /> : null}
+        <CoursePets />
+      </CoursePetsProvider>;
+      const { rerender } = render(tree(state));
+      await settle();
+      expect(handed.at(-1)).toMatchObject({ scene: `question-${OPEN_RUN.quiz}-${SHEET.tasks[0]!.id}`, quiet: true });
+      for (const task of SHEET.tasks.slice(1)) {
+        fireEvent.click(screen.getByRole("button", { name: new RegExp(task.title.en) }));
+        expect(handed.at(-1)).toMatchObject({ scene: `question-${OPEN_RUN.quiz}-${task.id}`, quiet: true });
+      }
+      rerender(tree({ ...state, step: { screen: "results", run: OPEN_RUN.run } }));
+      expect(handed.at(-1)).toMatchObject({ scene: OPEN_RUN.quiz, quiet: false });
+      rerender(tree({ ...state, step: { screen: "home" } }));
+      expect(handed.at(-1)).toMatchObject({ scene: "home", quiet: false });
+      rerender(tree(state));
+      expect(handed.at(-1)).toMatchObject({ scene: `question-${OPEN_RUN.quiz}-${SHEET.tasks[0]!.id}`, quiet: true });
+    });
+
+    it("ignores a question reporter belonging to another run and clears it when it leaves", async () => {
+      function Question(props: { run: string; task: string }): null {
+        usePetQuestion(props.run, props.task);
+        return null;
+      }
+      const source = fakeSource();
+      const { stage, handed } = fakeStage();
+      const { rerender } = render(<Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run: "r1" }}><Question run="r2" task="air" /></Pets>);
+      await settle();
+      expect(handed.at(-1)?.scene).toBe("heating");
+      rerender(<Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run: "r1" }}><Question run="r1" task="systems" /></Pets>);
+      expect(handed.at(-1)?.scene).toBe("question-heating-systems");
+      rerender(<Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run: "r1" }} />);
+      expect(handed.at(-1)?.scene).toBe("heating");
+    });
+

```

### 58 named

```diff
+        shiny: [],

```

### 60 named

```diff
-        <div className="quiz-app">
+        <div className="course-app">

```

### 62 named

```diff
-          <main id="quiz-main">
+          <main id="course-main">

```

### 64 named

```diff
-          <footer className="quiz-footer" />
+          <footer className="course-footer" />

```

### 66 named

```diff
-      document.querySelector('#quiz-main [data-slot="window-chrome-chip-cap"]')!.parentElement!.after(body);
+      document.querySelector('#course-main [data-slot="window-chrome-chip-cap"]')!.parentElement!.after(body);

```

### 68 named

```diff
-      expect(clear.filter((element) => element.tagName === "HEADER")).toEqual([document.querySelector(".quiz-app > header")]);
+      expect(clear.filter((element) => element.tagName === "HEADER")).toEqual([document.querySelector(".course-app > header")]);

```

### 70 named

```diff
-          <div data-presence-anchor="run" />
+          <div data-presence-anchor="run" ref={(element) => { if (element !== null) boxed(element, 0, 0); }} />

```

### 72 named

```diff
-      const marks = [...document.querySelectorAll<HTMLElement>("[data-presence-layer] .quiz-peer")];
+      const marks = [...document.querySelectorAll<HTMLElement>("[data-presence-layer] .course-peer")];

```

### 74 named

```diff
-          <div data-presence-anchor="run" />
+          <div data-presence-anchor="run" ref={(element) => { if (element !== null) boxed(element, 0, 0); }} />

```

### 76 named

```diff
-        const group = screen.getByRole("group", { name: courseText(locale)("quiz.preferences.pets") });
+        const group = screen.getByRole("group", { name: courseText(locale)("course.preferences.pets") });

```

### 78 named

```diff
-    it("stay still for a device that asks for reduced motion until the learner chooses: the preferences press what is in effect and say why in both languages, and say nothing once the learner chose", async () => {
+    it("keeps the learner's liveliness on a device that asks for reduced motion and does not say the device held them still", async () => {

```

### 80 named

```diff
-        const group = screen.getByRole("group", { name: courseText(locale)("quiz.preferences.pets") });
+        const group = screen.getByRole("group", { name: courseText(locale)("course.preferences.pets") });

```

### 82 named

```diff
-        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
-        expect(group.nextElementSibling?.tagName).toBe("P");
-        expect(pressed()).toEqual(["still"]);
+        expect(screen.queryByText(notes[locale])).toBeNull();
+        expect(pressed()).toEqual(["lively"]);

```

### 84 named

```diff
-        expect(layer()?.dataset.mode).toBe("still");
-        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
-        expect(group.nextElementSibling?.nextElementSibling?.textContent).toContain(" · ");
-
-        for (const choice of ["calm", "lively", "still"] as const) {
+        expect(layer()?.dataset.mode).toBe("lively");
+        for (const choice of ["calm", "still", "off"] as const) {

```

### 86 named

```diff
-          expect(layer()?.dataset.mode, choice).toBe(choice);

```

### 88 named

```diff
-          expect(group.nextElementSibling?.textContent, choice).toContain(" · ");
-          rerender(pets(choice, false));
-          expect(layer()?.dataset.mode, choice).toBe("still");
-          expect(pressed(), choice).toEqual(["still"]);
-          expect(group.nextElementSibling?.textContent, choice).toBe(notes[locale]);
-        }
-        for (const chosen of [true, false]) {
-          rerender(pets("off", chosen));
-          expect(layer()).toBeNull();
-          expect(pressed()).toEqual(["off"]);
-          expect(screen.queryByText(notes[locale])).toBeNull();
+          if (choice === "off") expect(layer()).toBeNull();
+          else expect(layer()?.dataset.mode, choice).toBe(choice);

```

### 90 named

```diff
-        expect(screen.queryByText(notes[locale])).toBeNull();

```

### 92 named

```diff
-      prefer(true);
-      render(
-        <Pets choice="lively">
-          <PreferencesPanel preferences={{ ...PREFERENCES, pets: "lively" }} locale="en" text={courseText("en")} onChange={() => undefined} />
-        </Pets>,
-      );
-      expect(screen.queryByText(notes.en)).toBeNull();
-      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }).getAttribute("aria-pressed")).toBe("true");

```

### 94 named

```diff
-        const group = screen.getByRole("group", { name: courseText(locale)("quiz.preferences.pets") });
+        const group = screen.getByRole("group", { name: courseText(locale)("course.preferences.pets") });

```

### 96 named

```diff
-        expect(layer()?.dataset.mode).toBe("still");
+        expect(layer()?.dataset.mode).toBe("calm");

```

### 98 named

```diff
-    const client = (storage: ReturnType<ReturnType<typeof memoryStorageOrigin>["tab"]>, pets?: CoursePetsSource): ReactElement => (
-      <CourseApp proctor="" tenant={COURSE.id} presence={QUIET_PRESENCE} transport={() => courseProctor()} storage={storage} languages={["en"]} timing={TIMING} pets={pets} />
-    );
-    const app = (): HTMLElement => document.querySelector<HTMLElement>(".quiz-app")!;
+    const client = (storage: ReturnType<ReturnType<typeof memoryStorageOrigin>["tab"]>, pets?: CoursePetsSource): ReactElement => {
+      localStore(storage, COURSE.id).write("introduced", true);
+      return <CourseApp proctor="" tenant={COURSE.id} presence={QUIET_PRESENCE} transport={() => courseProctor()} storage={storage} languages={["en"]} timing={TIMING} pets={pets} />;
+    };
+    const settings = async (): Promise<void> => {
+      await waitFor(() => expect(document.querySelector('[data-course-grid="root"]')).not.toBeNull());
+      if (window.location.hash !== "#prefs") fireEvent.click(screen.getByRole("link", { name: "Settings" }));
+      await screen.findByRole("group", { name: "Pets" });
+    };
+    const app = (): HTMLElement => document.querySelector<HTMLElement>(".course-app")!;

```

### 100 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
-      expect(app().dataset.pets).toBe("calm");
+      await settings();
+      expect(app().dataset.pets).toBe("lively");

```

### 102 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 104 named

```diff
-    it("holds the pets of a device that asks for reduced motion still only until the learner chooses in the preferences, and keeps that choice for the next visit", async () => {
-      const note = "Your device asks for less motion, so the pets stay still until you choose.";
+    it("starts lively on a device that asks for reduced motion, and keeps a later choice for the next visit", async () => {

```

### 106 named

```diff
-      const pressed = (): (string | null)[] => within(row()).getAllByRole("button").filter((button) => button.getAttribute("aria-pressed") === "true").map((button) => button.textContent);
+      const pressed = (): (string | null)[] => within(row()).getAllByRole("button").filter((button) => button.getAttribute("aria-pressed") === "true").map((button) => button.textContent?.trim() ?? null);

```

### 108 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 110 named

```diff
-      expect(app().dataset.pets).toBe("still");
-      expect(pressed()).toEqual(["Still"]);
-      expect(row().nextElementSibling?.textContent).toBe(note);
+      expect(app().dataset.pets).toBe("lively");
+      expect(app().dataset.iconMotion).toBe("on");
+      expect(pressed()).toEqual(["Lively"]);
+      expect((screen.getByRole("checkbox", { name: "Animate icons" }) as HTMLInputElement).checked).toBe(true);

```

### 112 named

```diff
-      expect(stored()).toMatchObject({ theme: "dark", animateIcons: true, iconsChosen: true, pets: "calm", petsLiveliness: "calm", petsChosen: false });
-      expect(app().dataset.pets).toBe("still");
-      expect(row().nextElementSibling?.textContent).toBe(note);
+      expect(stored()).toMatchObject({ theme: "dark", animateIcons: false, iconsChosen: true, pets: "lively", petsLiveliness: "lively", petsChosen: false });
+      expect(app().dataset.pets).toBe("lively");
+      expect(app().dataset.iconMotion).toBe("off");

```

### 114 named

```diff
-      expect(screen.queryByText(note)).toBeNull();

```

### 116 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 118 named

```diff
-      expect(screen.queryByText(note)).toBeNull();
+      expect(app().dataset.iconMotion).toBe("off");

```

### 120 named

```diff
-    it("counts the switch as a choice: on a device that asks for reduced motion it hides the still pets and brings calm ones back", async () => {
-      const note = "Your device asks for less motion, so the pets stay still until you choose.";
+    it("counts the switch as a choice: it hides the lively pets and brings them back as lively as they were", async () => {

```

### 122 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 124 named

```diff
-      expect(app().dataset.pets).toBe("still");
-      expect(screen.getByText(note)).not.toBeNull();
+      expect(app().dataset.pets).toBe("lively");

```

### 126 named

```diff
-      expect(screen.queryByText(note)).toBeNull();
-      expect(stored()).toMatchObject({ pets: "off", petsLiveliness: "calm", petsChosen: true });
+      expect(stored()).toMatchObject({ pets: "off", petsLiveliness: "lively", petsChosen: true });

```

### 128 named

```diff
-      expect(app().dataset.pets).toBe("calm");
+      expect(app().dataset.pets).toBe("lively");

```

### 130 named

```diff
-      expect(screen.queryByText(note)).toBeNull();
-      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Calm" }).getAttribute("aria-pressed")).toBe("true");
-      expect(stored()).toMatchObject({ pets: "calm", petsLiveliness: "calm", petsChosen: true });
+      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }).getAttribute("aria-pressed")).toBe("true");
+      expect(stored()).toMatchObject({ pets: "lively", petsLiveliness: "lively", petsChosen: true });

```

### 132 named

```diff
-    it("takes a liveliness that was stored without the word that the learner chose it for the default: still on a device that asks for reduced motion, as stored on any other", async () => {
-      const note = "Your device asks for less motion, so the pets stay still until you choose.";
+    it("keeps a liveliness that was stored without the word that the learner chose it, including on a device that asks for reduced motion", async () => {

```

### 134 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 136 named

```diff
-      expect(app().dataset.pets).toBe("still");
-      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling?.textContent).toBe(note);
+      expect(app().dataset.pets).toBe("lively");

```

### 138 named

```diff
-      expect(screen.queryByText(note)).toBeNull();

```

### 140 named

```diff
-      fireEvent.click(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }));
-      expect(app().dataset.pets).toBe("lively");
-      expect(screen.queryByText(note)).toBeNull();
-      expect(readPreferences(localStore(origin.tab(), COURSE.id))).toMatchObject({ theme: "dark", pets: "lively", petsLiveliness: "lively", petsChosen: true });
+      fireEvent.click(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Still" }));
+      expect(app().dataset.pets).toBe("still");
+      expect(readPreferences(localStore(origin.tab(), COURSE.id))).toMatchObject({ theme: "dark", pets: "still", petsLiveliness: "still", petsChosen: true });

```

### 142 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 144 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 146 named

```diff
-      expect(app().dataset.pets).toBe("calm");
+      expect(app().dataset.pets).toBe("lively");

```

### 148 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 150 named

```diff
-      expect(stored()).toMatchObject({ pets: "calm", petsChosen: false, petsPlay: false, petsMischief: true });
+      expect(stored()).toMatchObject({ pets: "lively", petsChosen: false, petsPlay: false, petsMischief: true });

```

### 152 named

```diff
-      await screen.findByRole("heading", { level: 1, name: "Welcome" });
+      await settings();

```

### 154 named

```diff
-        expect(readPreferences(localStore(origin.tab(), "arch"))).toMatchObject({ pets: "calm", petsChosen: false, petsPlay, petsMischief });
+        expect(readPreferences(localStore(origin.tab(), "arch"))).toMatchObject({ pets: "lively", petsChosen: false, petsPlay, petsMischief });

```

### 156 named

```diff
-    it("lets nothing play while a device that asks for reduced motion holds the pets still by default", async () => {
+    it("lets calm pets keep playing on a device that asks for reduced motion", async () => {

```

### 158 named

```diff
-      expect(handed.at(-1)).toMatchObject({ mode: "still", play: false, mischief: false });
-      expect((screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }) as HTMLInputElement).disabled).toBe(true);
-      expect((screen.getByRole("checkbox", { name: "Pets may play with the page" }) as HTMLInputElement).disabled).toBe(true);
-      expect(screen.getByText("Only calm and lively pets play.")).not.toBeNull();
-      expect(screen.queryByRole("group", { name: "Play with the pets" })).toBeNull();
+      expect(handed.at(-1)).toMatchObject({ mode: "calm", play: true, mischief: true });
+      expect((screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }) as HTMLInputElement).disabled).toBe(false);
+      expect((screen.getByRole("checkbox", { name: "Pets may play with the page" }) as HTMLInputElement).disabled).toBe(false);
+      expect(screen.queryByText("Only calm and lively pets play.")).toBeNull();
+      expect(screen.getByRole("group", { name: "Play with the pets" })).not.toBeNull();

```

### 160 named

```diff
-        <div className="quiz-app">
-          <main id="quiz-main">
+        <div className="course-app">
+          <main id="course-main">

```

### 162 named

```diff
-      const grips = [...document.querySelectorAll<HTMLElement>("[data-quiz-grip]")];
+      const grips = [...document.querySelectorAll<HTMLElement>("[data-course-grip]")];

```

### 164 named

```diff
-      const zones = [...document.querySelectorAll<HTMLElement>("[data-quiz-drop]")];
+      const zones = [...document.querySelectorAll<HTMLElement>("[data-course-drop]")];

```

### 166 named

```diff
-      const ghost = document.querySelector<HTMLElement>(".quiz-drag-ghost");
+      const ghost = document.querySelector<HTMLElement>(".course-drag-ghost");

```

### 168 named

```diff
-      expect(document.querySelector(".quiz-drag-ghost")).toBeNull();
+      expect(document.querySelector(".course-drag-ghost")).toBeNull();

```

### 170 named

```diff
-        ["LI", "heating/u-values", "quiz-tasks"],
-        ["LI", "heating/heating-load-and-demand", "quiz-tasks"],
+        ["LI", "heating/u-values", "course-tasks"],
+        ["LI", "heating/heating-load-and-demand", "course-tasks"],

```

### 172 named

```diff
-      const valueCards = [...document.querySelectorAll("li[data-quiz-drag]")];
+      const valueCards = [...document.querySelectorAll("li[data-course-drag]")];

```

### 174 named

```diff
+
+describe("badge-driven shiny pets", () => {
+  const menagerie: Menagerie = { ...MENAGERIE, species: [...MENAGERIE.species, species("chilly", text("Chilly", "Frosti"))], casts: shinyCases.casts };
+  const course: CourseView = { ...COURSE, quizzes: shinyCases.quizzes.map(quiz => ({ ...COURSE.quizzes[0], ...quiz, tasks: quiz.tasks.map(task => ({ ...task, kind: "sorting", title: text(task.id, task.id) })) })) };
+  const learner = (badges: LearnerView["badges"]): LearnerView => ({ learner: "a", identity: { kind: "anonymous" }, runs: shinyCases.runs.map(run => ({ ...run, challenge: "easy", status: "submitted", startedAt: 0 })), badges, best: {}, total: 0 });
+  for (const example of shinyCases.cases) {
+    it(example.name, () => {
+      const learnerView = learner(example.badges);
+      const awards = (example.awards ?? {}) as CourseState["awards"];
+      const localRuns = shinyCases.runs.map(run => ({ ...run, learner: example.localLearner ?? "a" })) as unknown as RunView[];
+      const eligible = uniq([...example.badges.map(award => award.run), ...Object.entries(awards).filter(([, badges]) => badges.length > 0).map(([run]) => run)]);
+      const scenes = uniq(flatMap(course.quizzes.filter(quiz => eligible.some(id => find(localRuns, { run: id, learner: "a" })?.quiz === quiz.id)), quiz => [quiz.id, ...quiz.tasks.map(task => `question-${quiz.id}-${task.id}`)]));
+      const reference = uniq(flatMap(shinyCases.casts.filter(cast => scenes.includes(cast.scene)), cast => [...cast.core, ...cast.rotation]));
+      expect(menagerie.species.filter(entry => reference.includes(entry.id)).map(entry => entry.id)).toEqual(example.shiny);
+      expect(shinyPetSpecies(menagerie, { course, learnerView, learner: { id: "a" }, awards, runs: Object.fromEntries(localRuns.map(run => [run.run, run])) })).toEqual(example.shiny);
+    });
+  }
+  it("hands updated awards to the layer and clears them when learners switch", async () => {
+    const source: CoursePetsSource = async () => menagerie;
+    const { stage, handed } = fakeStage();
+    const earned = shinyCases.cases.find(example => example.name === "core, rotation and question pets")!;
+    const show = (learnerView?: LearnerView): ReactElement => <CoursePetsProvider source={source} stage={stage} choice="still" chosen play mischief locale="en" state={{ ...at({ screen: "home" }), course, learnerView, learner: learnerView === undefined ? undefined : { id: learnerView.learner } }}><CoursePets /></CoursePetsProvider>;
+    const view = render(show());
+    await settle();
+    expect(handed.at(-1)?.shiny).toEqual([]);
+    view.rerender(show(learner(earned.badges)));
+    expect(handed.at(-1)?.shiny).toEqual(earned.shiny);
+    view.rerender(show(learner([])));
+    expect(handed.at(-1)?.shiny).toEqual([]);
+    view.rerender(show());
+    expect(handed.at(-1)?.shiny).toEqual([]);
+    expect(stage).toHaveBeenCalledTimes(1);
+  });
+  it("shines on the submission event before the learner view refreshes", async () => {
+    const source: CoursePetsSource = async () => menagerie;
+    const { stage, handed } = fakeStage();
+    const current: CourseState = { ...initialCourseState({ introduced: true, runs: {} }), course, learner: { id: "a" }, runs: { r1: { ...RUNS.r1, quiz: "heat", learner: "a" } } };
+    const show = (state: CourseState): ReactElement => <CoursePetsProvider source={source} stage={stage} choice="still" chosen play mischief locale="de" state={state}><CoursePets /></CoursePetsProvider>;
+    const view = render(show(current));
+    await settle();
+    expect(handed.at(-1)?.shiny).toEqual([]);
+    const submitted = evolveCourseState(current, { type: "run-submitted", run: "r1", result: {} as RunResult, badges: ["warm"], at: 1 });
+    view.rerender(show(submitted));
+    expect(handed.at(-1)?.shiny).toEqual(["sunny", "housy", "flamy"]);
+    view.rerender(show(evolveCourseState(submitted, { type: "learner-forgotten" })));
+    expect(handed.at(-1)?.shiny).toEqual([]);
+  });
+});

```

## 106 🧰️framework/🛍️products/❓️quiz/🧬️schema/🦀️.rs

### 1 

```diff
-pub const WIRE_VERSION: WireVersion = 4;
+pub const WIRE_VERSION: WireVersion = 8;

```

### 3 

```diff
+/// 🧺️ Fixed semantic slots with rotating variants and a per-question time budget.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct QuestionPool {
+    pub seconds: u32,
+    pub slots: Vec<Vec<Slug>>,
+}
+

```

### 5 

```diff
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    pub pool: Option<QuestionPool>,

```

### 7 

```diff
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    pub pool: Option<QuestionPool>,

```

### 9 

```diff
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    pub pool: Option<QuestionPool>,

```

### 11 

```diff
+    /// 🎠️ The fixed semantic slots and time budget, when authored.
+    pub fn pool(&self) -> Option<&QuestionPool> {
+        match self {
+            Self::Classification(task) => task.pool.as_ref(),
+            Self::Sorting(task) => task.pool.as_ref(),
+            Self::Matching(task) => task.pool.as_ref(),
+        }
+    }
+

```

### 13 icon

```diff
+/// 🧭️ Page names reserved for the course's root navigation.
+pub const COURSE_ROOT_IDS: [&str; 11] = ["home", "identity", "introduction", "learner", "intro", "board", "badges", "prefs", "quizzes", "run", "results"];
+
+/// 🗂️ An ordered course group; an empty quiz list marks a placeholder.
+#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct Topic {
+    pub id: Slug,
+    pub emoji: String,
+    pub title: Text,
+    pub quizzes: Vec<Slug>,
+}
+

```

### 15 icon

```diff
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    pub topics: Option<Vec<Topic>>,

```

### 17 default

```diff
-    StartRun { id: Id, learner: Id, run: Id, quiz: Slug, challenge: Challenge, at: Timestamp },
+    StartRun { id: Id, learner: Id, #[serde(default, skip_serializing_if = "Option::is_none")] origin: Option<Id>, run: Id, quiz: Slug, challenge: Challenge, at: Timestamp },

```

### 19 type_name

```diff
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    pub topics: Option<Vec<Topic>>,

```

### 21 type_name

```diff
-#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

```

### 23 type_name

```diff
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub view: Option<PresencePresentation>,
+}
+
+/// 🪞️ The public run presentation, preserving randomized sheet and answers while omitting private ids.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(rename_all = "camelCase", deny_unknown_fields)]
+pub struct PresenceRun {
+    pub quiz: Slug,
+    pub status: RunStatus,
+    pub sheet: Sheet,
+    pub answers: BTreeMap<Slug, Answer>,
+    pub started_at: Timestamp,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub result: Option<RunResult>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub submitted_at: Option<Timestamp>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub opened: Option<BTreeMap<Slug, Timestamp>>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub hints: Option<BTreeMap<Slug, Vec<Hint>>>,
+}
+
+/// 📐️ A normalized scroll or camera position.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct PresencePoint { pub x: f64, pub y: f64 }
+
+/// 🎨️ Public display choices affecting the followed presentation, admitted against the normative enums.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(rename_all = "camelCase", deny_unknown_fields)]
+pub struct PresencePreferences {
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub device: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub theme: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub text_size: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub show_cursors: Option<bool>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub others: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub animate_icons: Option<bool>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub icons_chosen: Option<bool>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub pets: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub pets_chosen: Option<bool>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub pets_liveliness: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub pets_play: Option<bool>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub pets_mischief: Option<bool>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub challenges: Option<BTreeMap<Slug, Challenge>>,
+}
+
+/// 🏃️ A public history entry without its private run link.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(rename_all = "camelCase", deny_unknown_fields)]
+pub struct PresenceRunSummary {
+    pub quiz: Slug,
+    pub challenge: Challenge,
+    pub status: RunStatus,
+    pub started_at: Timestamp,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub score: Option<Score>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub points: Option<f64>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub submitted_at: Option<Timestamp>,
+}
+
+/// 🎗️ A public badge award without its private run link.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct PresenceBadgeAward { pub badge: Slug, pub at: Timestamp }
+
+/// 🏅️ The visible public profile, excluding private learner and run ids.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct PresenceProfile { pub runs: Vec<PresenceRunSummary>, pub badges: Vec<PresenceBadgeAward>, pub best: BTreeMap<Slug, Best>, pub total: f64 }
+
+/// 📊️ The scope and period of the leaderboard being viewed.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct PresenceBoard {
+    pub period: LeaderboardPeriod,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub quiz: Option<Slug>,
+}
+
+/// ↕️ The public leaderboard's selected sort key and direction.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct PresenceSort { pub key: String, pub direction: String }
+
+/// ⏳️ A visible submission dialog or progress state, without executable commands.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct PresenceSubmission {
+    pub phase: String,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub done: Option<u16>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub total: Option<u16>,
+}
+
+/// 🖥️ The public overview grid and navigation mode resolved on the publishing device.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct PresenceLayout { pub grid: String, pub mode: String }
+
+/// 🪟️ Ephemeral screen presentation followed without modifying the follower's saved state.
+#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
+#[serde(rename_all = "camelCase", deny_unknown_fields)]
+pub struct PresencePresentation {
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub locale: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub home: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub swipe: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub layout: Option<PresenceLayout>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub reveal: Option<Slug>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub revealed_by: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub task_page: Option<u16>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub viewport: Option<PresencePoint>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub camera: Option<PresencePoint>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub preferences: Option<PresencePreferences>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub run: Option<PresenceRun>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub profile: Option<PresenceProfile>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub asked: Option<Vec<Slug>>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub board: Option<PresenceBoard>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub sort: Option<PresenceSort>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub submission: Option<PresenceSubmission>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub leaderboard: Option<Leaderboard>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub leaderboard_at: Option<Timestamp>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub overall: Option<Leaderboard>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub overall_at: Option<Timestamp>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub crowd: Option<CrowdView>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub awards: Option<Vec<Slug>>,

```

### 25 type_name

```diff
-    #[serde(default, skip_serializing_if = "Option::is_none")]
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub place: Option<Place>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
+    pub home: Option<String>,
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]

```

### 27 type_name

```diff
-    #[serde(default, skip_serializing_if = "Option::is_none")]
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]

```

### 29 type_name

```diff
-    #[serde(default, skip_serializing_if = "Option::is_none")]
+    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]

```

### 31 type_name

```diff
+fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<Option<T>, D::Error> {
+    T::deserialize(deserializer).map(Some)
+}

```
