# FEM2D Session Immutable Incoming Diff

```diff
--- B

+++ S

@@ -1,7 +1,13 @@

 //! 🧵️ Mounted Fem2d revision job: fixed session arena, retained worker step and live visual lease.
 
-use semio_s_fem_analyses::{FemJobGraph, FemJobStage, FemStagePlan};
-use semio_s_fem_analyses_assembly::{AssemblyCsrBuild, AssemblyJob, AssemblyJobConstruction};
-use semio_s_fem_analyses_model::AnalysisModel;
+use semio_s_fem_analyses_model::MountedAnalysisModel;
+use semio_s_fem_analyses_model::MountedAnalysisSupport;
+use semio_s_fem_analyses_assembly::AssemblyCsrBuild;
+use semio_s_fem_analyses_assembly::AssemblyJob;
+use semio_s_fem_analyses_assembly::AssemblyJobConstruction;
+use semio_s_fem_analyses::FemJobGraph;
+use semio_s_fem_analyses::FemJobStage;
+use semio_s_fem_analyses::FemStagePlan;
+use semio_s_fem_analyses_model::ResultSource;
 use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dLiveVisual;
 use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dMountedVisualLease;
@@ -10,17 +16,40 @@

 use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::FemVisualState;
 use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::RegionVisualQuality;
-use semio_s_fem_mesh::{MeshJob, MeshOpts, PlanarDomain, TriMesh2};
+use semio_s_fem_mesh::MeshJob;
+use semio_s_fem_mesh::MeshOpts;
+use semio_s_fem_mesh::PlanarDomain;
+use semio_s_fem_mesh::TriMesh2;
 use semio_s_fem_model::Element;
-use semio_s_fem_sparse_pcg::{PcgJob, PcgJobConstruction};
+use crate::sparse::Csr;
+use crate::sparse::MountedScalarSlots;
+use semio_s_fem_sparse_pcg::PcgJob;
+use semio_s_fem_sparse_pcg::PcgJobConstruction;
 use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
 use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
 use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
-use semio_framework::kernel::{Effect, JobPlacement};
+use semio_framework::kernel::Effect;
+use semio_framework::kernel::JobPlacement;
 #[cfg(test)]
 use semio_framework_job::CommitValidation;
-use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, Operation, OperationId, RetainedJobPayload, StepContext};
-use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, IoRunControl, JobBudget, JobStep, SqliteSnapshotControl};
-use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginLifecycleStep};
-use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
+use semio_framework_job::InteractiveJob;
+use semio_framework_job::InteractiveJobCloseStep;
+use semio_framework_job::JobOutcomeBorrow;
+use semio_framework_job::Operation;
+use semio_framework_job::OperationId;
+use semio_framework_job::RetainedJobPayload;
+use semio_framework_job::StepContext;
+use semio_framework_plugin::reactor::jobs::BoundedJob;
+use semio_framework_plugin::reactor::jobs::BoundedJobFactory;
+use semio_framework_plugin::reactor::jobs::IoRunControl;
+use semio_framework_plugin::reactor::jobs::JobStep;
+use semio_framework_plugin::reactor::jobs::SqliteSnapshotControl;
+use semio_framework_plugin::AppRenderOperationContext;
+use semio_framework_plugin::ArtifactView;
+use semio_framework_plugin::{PluginLifecycleStep, PluginLifecycleFailure};
+use semio_framework_value::RetainedCloneGrant;
+use semio_framework_value::RetainedCloneProgress;
+use semio_framework_value::RetirementDemand;
+use semio_framework_value::ValueError;
+use semio_framework_value::ValueRefusalKind;
 use std::cell::RefCell;
 use std::rc::Rc;
@@ -29,12 +58,14 @@

 pub const FEM2D_MOUNTED_JOB_KIND: &str = "semio.fem2d.mounted-analysis";
 const SESSION_ACTIVE_CAPACITY: usize = 32;
+const SESSION_CONSUMER_CAPACITY: usize = 128;
+type RequestKey = semio_framework_value::ContentRequestKey;
+#[derive(Clone,Copy,Debug,PartialEq,Eq)]
+pub enum ConsumerKey{Model,Window(semio_framework_plugin::WindowConfigAuthorityIdentity)}
 const SESSION_SHELL_CAPACITY: usize = 64;
 const SESSION_MAXIMUM_INPUT_ITEMS: usize = 4_096;
 const SESSION_MAXIMUM_INPUT_BYTES: usize = 4 * 1_024 * 1_024;
-/// 📏️ Fixed model extents: document nodes/elements plus the region mesh's points/triangles share one
-/// `SESSION_OWNER_PAGE_BYTES` owner each (`model.nodes` at 48 B a node → 128 fit; `model.elements` at
-/// 256 B an element → 64 fit), and the bundled demo alone is 12 nodes + 9 elements + one meshed slab.
-const SESSION_MAXIMUM_NODES: usize = 48;
-const SESSION_MAXIMUM_ELEMENTS: usize = 24;
+/// 📏️ Shared paged model admission covers the complete accumulated document and region geometry.
+const SESSION_MAXIMUM_NODES: usize = semio_s_fem_analyses_model::MOUNTED_ANALYSIS_NODE_SLOTS;
+const SESSION_MAXIMUM_ELEMENTS: usize = semio_s_fem_analyses_model::MOUNTED_ANALYSIS_ELEMENT_SLOTS;
 const SESSION_MAXIMUM_SUPPORTS: usize = 64;
 const SESSION_MAXIMUM_MESH_POINTS: usize = 80;
@@ -114,7 +145,7 @@

                 MountedOwnerClaim { class: MountedOwnerClass::DomainHoleList, roots: 1, items: SESSION_MAXIMUM_REGION_HOLES },
                 MountedOwnerClaim { class: MountedOwnerClass::DomainHoleVectors, roots: SESSION_MAXIMUM_REGION_HOLES, items: SESSION_MAXIMUM_REGION_HOLES * SESSION_MAXIMUM_BOUNDARY_POINTS },
-                MountedOwnerClaim { class: MountedOwnerClass::ModelNodes, roots: 1, items: SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS },
-                MountedOwnerClaim { class: MountedOwnerClass::ModelElements, roots: 1, items: SESSION_MAXIMUM_ELEMENTS + SESSION_MAXIMUM_MESH_TRIANGLES },
-                MountedOwnerClaim { class: MountedOwnerClass::ModelSupports, roots: 1, items: SESSION_MAXIMUM_SUPPORTS },
+                MountedOwnerClaim { class: MountedOwnerClass::ModelNodes, roots: (SESSION_MAXIMUM_NODES * size_of::<semio_s_fem_model::Node>() + 4095) / 4096, items: SESSION_MAXIMUM_NODES },
+                MountedOwnerClaim { class: MountedOwnerClass::ModelElements, roots: (SESSION_MAXIMUM_ELEMENTS * size_of::<semio_s_fem_static::Elements>() + 4095) / 4096, items: SESSION_MAXIMUM_ELEMENTS },
+                MountedOwnerClaim { class: MountedOwnerClass::ModelSupports, roots: (SESSION_MAXIMUM_SUPPORTS * size_of::<MountedAnalysisSupport>() + 4095) / 4096, items: SESSION_MAXIMUM_SUPPORTS },
                 MountedOwnerClaim { class: MountedOwnerClass::ModelRegionNodeIds, roots: 1, items: SESSION_MAXIMUM_MESH_POINTS },
                 MountedOwnerClaim {
@@ -151,5 +182,5 @@

                 },
                 MountedOwnerClaim { class: MountedOwnerClass::VisualStrings, roots: 64 + SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS + SESSION_MAXIMUM_ELEMENTS + SESSION_MAXIMUM_MESH_TRIANGLES + 1, items: 0 },
-                MountedOwnerClaim { class: MountedOwnerClass::OutputPages, roots: 3 * (SESSION_MAXIMUM_OUTPUT_BYTES / SESSION_OWNER_PAGE_BYTES), items: 6 },
+                MountedOwnerClaim { class: MountedOwnerClass::OutputPages, roots: 3 * semio_framework_ui_scene::CANVAS2D_SNAPSHOT_PAGE_CAPACITY + 3 * (SESSION_MAXIMUM_OUTPUT_BYTES / SESSION_OWNER_PAGE_BYTES), items: 6 },
                 MountedOwnerClaim { class: MountedOwnerClass::FaultPages, roots: 2, items: 0 },
             ],
@@ -234,9 +265,12 @@

     Mesh,
     BuildModel,
+    RetireRegion,
     PrepareAssembly,
     Assembly,
     BuildCsr,
+    BuildLoads,
     PreparePcg,
     Pcg,
+    RecoverReactions,
     SyncPcgVisual,
     CommitReady,
@@ -277,4 +311,5 @@

     RegionElementIdReserve,
     RegionElementIdOwner,
+    RegionElementIdentity,
     RegionElementNodeOwner,
     RegionElementCommit,
@@ -288,4 +323,5 @@

     support_cursor: usize,
     region_point_cursor: usize,
+    region_cursor: usize,
     region_triangle_cursor: usize,
     lookup_cursor: usize,
@@ -297,5 +333,5 @@

     pending_element_start: Option<String>,
     pending_element_end: Option<String>,
-    pending_support: Option<semio_s_fem_model::Support>,
+    pending_support: Option<MountedAnalysisSupport>,
     pending_region_element_id: Option<String>,
     pending_region_element_nodes: [Option<String>; 3],
@@ -304,5 +340,5 @@

     mesh: Option<TriMesh2>,
     region_node_ids: Vec<String>,
-    model: AnalysisModel,
+    model: MountedAnalysisModel,
     close_lane: u8,
 }
@@ -316,4 +352,5 @@

             support_cursor: 0,
             region_point_cursor: 0,
+            region_cursor: 0,
             region_triangle_cursor: 0,
             lookup_cursor: 0,
@@ -332,5 +369,5 @@

             mesh,
             region_node_ids: Vec::new(),
-            model: AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() },
+            model: MountedAnalysisModel::new(),
             close_lane: 0,
         }
@@ -345,5 +382,4 @@

             || mesh_points > SESSION_MAXIMUM_MESH_POINTS
             || mesh_triangles > SESSION_MAXIMUM_MESH_TRIANGLES
-            || snapshot.regions.len() > 1
         {
             return Err(b"fem2d.model-fixed-capacity".to_vec());
@@ -351,22 +387,15 @@

         match self.stage {
             ModelBuildStage::ReserveNodes => {
-                self.model.nodes.try_reserve_exact(snapshot.nodes.len() + mesh_points).map_err(|_| b"fem2d.model-node-allocation".to_vec())?;
-                if self.model.nodes.capacity() * size_of::<semio_s_fem_model::Node>() > SESSION_OWNER_PAGE_BYTES {
-                    return Err(b"fem2d.model-node-page-capacity".to_vec());
-                }
+                let target = snapshot.nodes.len().max(self.model.nodes_len()) + mesh_points;
+                if !self.model.admit_node_one(target, SESSION_OWNER_PAGE_BYTES).map_err(|_| b"fem2d.model-node-admission".to_vec())?.complete { return Ok(false); }
                 self.stage = ModelBuildStage::ReserveElements;
             }
             ModelBuildStage::ReserveElements => {
-                self.model.elements.try_reserve_exact(snapshot.elements.len() + mesh_triangles).map_err(|_| b"fem2d.model-element-allocation".to_vec())?;
-                if self.model.elements.capacity() * size_of::<semio_s_fem_static::Elements>() > SESSION_OWNER_PAGE_BYTES {
-                    return Err(b"fem2d.model-element-page-capacity".to_vec());
-                }
+                let target = snapshot.elements.len().max(self.model.elements_len()) + mesh_triangles;
+                if !self.model.admit_element_one(target, SESSION_OWNER_PAGE_BYTES).map_err(|_| b"fem2d.model-element-admission".to_vec())?.complete { return Ok(false); }
                 self.stage = ModelBuildStage::ReserveSupports;
             }
             ModelBuildStage::ReserveSupports => {
-                self.model.supports.try_reserve_exact(snapshot.supports.len()).map_err(|_| b"fem2d.model-support-allocation".to_vec())?;
-                if self.model.supports.capacity() * size_of::<semio_s_fem_model::Support>() > SESSION_OWNER_PAGE_BYTES {
-                    return Err(b"fem2d.model-support-page-capacity".to_vec());
-                }
+                if !self.model.admit_support_one(snapshot.supports.len(), SESSION_OWNER_PAGE_BYTES).map_err(|_| b"fem2d.model-support-admission".to_vec())?.complete { return Ok(false); }
                 self.stage = ModelBuildStage::ReserveRegionNodeIds;
             }
@@ -376,5 +405,5 @@

                     return Err(b"fem2d.model-region-node-page-capacity".to_vec());
                 }
-                self.stage = ModelBuildStage::Nodes;
+                self.stage = if self.region_cursor == 0 { ModelBuildStage::Nodes } else { ModelBuildStage::RegionNodeCoordinate };
             }
             ModelBuildStage::Nodes => {
@@ -390,5 +419,5 @@

                 bounded_derived_string(self.pending_node_id.as_ref().ok_or_else(|| b"fem2d.model-node-id-owner-missing".to_vec())?)?;
                 let id = self.pending_node_id.take().ok_or_else(|| b"fem2d.model-node-id-owner-missing".to_vec())?;
-                self.model.nodes.push(semio_s_fem_model::Node { id, pos: [node.x, node.y, 0.0] });
+                self.model.push_node(semio_s_fem_model::Node { id, pos: [node.x, node.y, 0.0] }).map_err(|_| b"fem2d.model-node-capacity".to_vec())?;
                 self.node_cursor += 1;
                 self.stage = ModelBuildStage::Nodes;
@@ -510,5 +539,5 @@

                     .into(),
                 };
-                self.model.elements.push(resolved);
+                self.model.push_element(resolved).map_err(|_| b"fem2d.model-element-capacity".to_vec())?;
                 self.element_cursor += 1;
                 self.stage = ModelBuildStage::ElementStart;
@@ -516,5 +545,5 @@

             ModelBuildStage::Supports => {
                 if let Some(support) = snapshot.supports.get(self.support_cursor) {
-                    self.pending_support = Some(semio_s_fem_model::Support { node_id: support.node_id.clone(), fixed: Vec::new() });
+                    self.pending_support = Some(MountedAnalysisSupport::new(support.node_id.clone()));
                     self.stage = ModelBuildStage::SupportReserveDofs;
                 } else {
@@ -524,10 +553,7 @@

             ModelBuildStage::SupportReserveDofs => {
                 let source = snapshot.supports.get(self.support_cursor).ok_or_else(|| b"fem2d.model-support-owner-missing".to_vec())?;
-                let owner = self.pending_support.as_mut().ok_or_else(|| b"fem2d.model-support-candidate-missing".to_vec())?;
-                bounded_derived_string(&owner.node_id)?;
-                owner.fixed.try_reserve_exact(source.fixed.len()).map_err(|_| b"fem2d.model-support-dof-allocation".to_vec())?;
-                if owner.fixed.capacity() * size_of::<semio_s_fem_model::Dof>() > SESSION_OWNER_PAGE_BYTES {
-                    return Err(b"fem2d.model-support-dof-page-capacity".to_vec());
-                }
+                if self.pending_support.is_none() { return Err(b"fem2d.model-support-candidate-missing".to_vec()); }
+                bounded_derived_string(&source.node_id)?;
+                if source.fixed.len() > 6 { return Err(b"fem2d.model-support-dof-capacity".to_vec()); }
                 self.lookup_cursor = 0;
                 self.stage = ModelBuildStage::SupportDofs;
@@ -536,5 +562,5 @@

                 let source = snapshot.supports.get(self.support_cursor).ok_or_else(|| b"fem2d.model-support-owner-missing".to_vec())?;
                 if let Some(dof) = source.fixed.get(self.lookup_cursor).copied() {
-                    self.pending_support.as_mut().ok_or_else(|| b"fem2d.model-support-candidate-missing".to_vec())?.fixed.push(dof.into());
+                    self.pending_support.as_mut().ok_or_else(|| b"fem2d.model-support-candidate-missing".to_vec())?.push_fixed(dof.into()).map_err(|_| b"fem2d.model-support-dof-capacity".to_vec())?;
                     self.lookup_cursor += 1;
                 } else {
@@ -544,5 +570,5 @@

             }
             ModelBuildStage::SupportCommit => {
-                self.model.supports.push(self.pending_support.take().ok_or_else(|| b"fem2d.model-support-candidate-missing".to_vec())?);
+                self.model.push_support(self.pending_support.take().ok_or_else(|| b"fem2d.model-support-candidate-missing".to_vec())?).map_err(|_| b"fem2d.model-support-capacity".to_vec())?;
                 self.support_cursor += 1;
                 self.stage = ModelBuildStage::Supports;
@@ -557,6 +583,6 @@

                     return Ok(false);
                 };
-                if let Some(node) = snapshot.nodes.get(self.lookup_cursor) {
-                    if (node.x - point[0]).abs() < 1e-9 && (node.y - point[1]).abs() < 1e-9 {
+                if let Some(node) = self.model.node(self.lookup_cursor) {
+                    if (node.pos[0] - point[0]).abs() < 1e-9 && (node.pos[1] - point[1]).abs() < 1e-9 {
                         self.pending_region_id = Some(node.id.clone());
                         self.pending_region_insert = false;
@@ -571,5 +597,5 @@

             }
             ModelBuildStage::RegionNodeIdReserve => {
-                let region = snapshot.regions.first().ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
+                let region = snapshot.regions.get(self.region_cursor).ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
                 let required = region.id.len().checked_add(24).ok_or_else(|| b"fem2d.model-generated-node-id-overflow".to_vec())?;
                 if required > SESSION_MAXIMUM_STRING_BYTES {
@@ -585,5 +611,5 @@

             }
             ModelBuildStage::RegionNodeIdWrite => {
-                let region = snapshot.regions.first().ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
+                let region = snapshot.regions.get(self.region_cursor).ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
                 let id = self.pending_region_id.as_mut().ok_or_else(|| b"fem2d.model-generated-node-id-owner-missing".to_vec())?;
                 use std::fmt::Write as _;
@@ -595,5 +621,5 @@

             ModelBuildStage::RegionNodeId => {
                 let id = self.pending_region_id.as_ref().expect("region node id retained");
-                if let Some(node) = self.model.nodes.get(self.lookup_cursor) {
+                if let Some(node) = self.model.node(self.lookup_cursor) {
                     if node.id == *id {
                         return Err(b"fem2d.model-generated-node-id-collision".to_vec());
@@ -608,5 +634,5 @@

                 let point = self.mesh.as_ref().and_then(|mesh| mesh.points.get(self.region_point_cursor)).copied().ok_or_else(|| b"fem2d.model-region-point-missing".to_vec())?;
                 let id = self.pending_region_id.as_ref().ok_or_else(|| b"fem2d.model-region-node-owner-missing".to_vec())?.clone();
-                self.model.nodes.push(semio_s_fem_model::Node { id, pos: [point[0], point[1], 0.0] });
+                self.model.push_node(semio_s_fem_model::Node { id, pos: [point[0], point[1], 0.0] }).map_err(|_| b"fem2d.model-region-node-capacity".to_vec())?;
                 self.pending_region_insert = false;
                 self.stage = ModelBuildStage::RegionNodeCommit;
@@ -626,5 +652,5 @@

             }
             ModelBuildStage::RegionElementMaterial => {
-                let Some(region) = snapshot.regions.first() else {
+                let Some(region) = snapshot.regions.get(self.region_cursor) else {
                     self.stage = ModelBuildStage::Complete;
                     return Ok(false);
@@ -645,5 +671,5 @@

             }
             ModelBuildStage::RegionElementIdReserve => {
-                let region = snapshot.regions.first().ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
+                let region = snapshot.regions.get(self.region_cursor).ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
                 let required = region.id.len().checked_add(24).ok_or_else(|| b"fem2d.model-generated-element-id-overflow".to_vec())?;
                 if required > SESSION_MAXIMUM_STRING_BYTES {
@@ -659,10 +685,18 @@

             }
             ModelBuildStage::RegionElementIdOwner => {
-                let region = snapshot.regions.first().ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
+                let region = snapshot.regions.get(self.region_cursor).ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
                 let id = self.pending_region_element_id.as_mut().ok_or_else(|| b"fem2d.model-generated-element-id-owner-missing".to_vec())?;
                 use std::fmt::Write as _;
                 write!(id, "{}_t{}", region.id, self.region_triangle_cursor).map_err(|_| b"fem2d.model-generated-element-id-format".to_vec())?;
                 self.pending_region_element_node_cursor = 0;
-                self.stage = ModelBuildStage::RegionElementNodeOwner;
+                self.lookup_cursor = 0;
+                self.stage = ModelBuildStage::RegionElementIdentity;
+            }
+            ModelBuildStage::RegionElementIdentity => {
+                let id = self.pending_region_element_id.as_ref().ok_or_else(|| b"fem2d.model-generated-element-id-owner-missing".to_vec())?;
+                if let Some(element) = self.model.element(self.lookup_cursor) {
+                    if element.id() == id { return Err(b"fem2d.model-generated-element-id-collision".to_vec()); }
+                    self.lookup_cursor += 1;
+                } else { self.stage = ModelBuildStage::RegionElementNodeOwner; }
             }
             ModelBuildStage::RegionElementNodeOwner => {
@@ -684,8 +718,8 @@

                     bounded_derived_string(owner.as_ref().ok_or_else(|| b"fem2d.model-region-element-node-owner-missing".to_vec())?)?;
                 }
-                let region = snapshot.regions.first().expect("region element retains region");
+                let region = snapshot.regions.get(self.region_cursor).expect("region element retains region");
                 let material = &snapshot.materials[self.resolved_material];
                 let nodes = std::array::from_fn(|index| self.pending_region_element_nodes[index].take().expect("region element node owner retained"));
-                self.model.elements.push(
+                self.model.push_element(
                     semio_s_fem_elements2d::Tri3Cst {
                         id: self.pending_region_element_id.take().ok_or_else(|| b"fem2d.model-region-element-id-owner-missing".to_vec())?,
@@ -698,5 +732,5 @@

                     }
                     .into(),
-                );
+                ).map_err(|_| b"fem2d.model-region-element-capacity".to_vec())?;
                 self.region_triangle_cursor += 1;
                 self.stage = ModelBuildStage::RegionElementMaterial;
@@ -707,6 +741,16 @@

     }
 
-    fn take_complete(&mut self) -> Option<AnalysisModel> {
-        (self.stage == ModelBuildStage::Complete).then(|| std::mem::replace(&mut self.model, AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() }))
+    fn take_complete(&mut self) -> Option<MountedAnalysisModel> {
+        (self.stage == ModelBuildStage::Complete).then(|| std::mem::take(&mut self.model))
+    }
+
+    fn attach_region(&mut self, region_cursor: usize, mesh: TriMesh2) {
+        self.region_cursor = region_cursor;
+        self.region_point_cursor = 0;
+        self.region_triangle_cursor = 0;
+        self.lookup_cursor = 0;
+        self.mesh = Some(mesh);
+        self.stage = ModelBuildStage::ReserveNodes;
+        self.close_lane = 0;
     }
 
@@ -735,23 +779,6 @@

         }
         if let Some(support) = self.pending_support.as_mut() {
-            if support.node_id.capacity() != 0 {
-                let bytes = support.node_id.capacity();
-                if bytes > maximum_bytes {
-                    return (false, 0, 0);
-                }
-                support.node_id = String::new();
-                return (false, 1, bytes);
-            }
-            if support.fixed.pop().is_some() {
-                return (false, 1, 0);
-            }
-            let bytes = support.fixed.capacity() * size_of::<semio_s_fem_model::Dof>();
-            if bytes != 0 {
-                if bytes > maximum_bytes {
-                    return (false, 0, 0);
-                }
-                support.fixed = Vec::new();
-                return (false, 1, bytes);
-            }
+            let progress = support.close_step(maximum_bytes);
+            if !progress.0 { return progress; }
             self.pending_support = None;
             return (false, 1, 0);
@@ -759,94 +786,8 @@

         loop {
             let released_bytes = match self.close_lane {
-                0 => {
-                    let Some(node) = self.model.nodes.last_mut() else {
-                        self.close_lane += 1;
-                        continue;
-                    };
-                    if node.id.capacity() != 0 {
-                        let bytes = node.id.capacity();
-                        if bytes > maximum_bytes {
-                            return (false, 0, 0);
-                        }
-                        node.id = String::new();
-                        return (false, 1, bytes);
-                    }
-                    self.model.nodes.pop();
-                    return (false, 1, 0);
-                }
-                1 => {
-                    let bytes = self.model.nodes.capacity() * size_of::<semio_s_fem_model::Node>();
-                    if bytes > maximum_bytes {
-                        return (false, 0, 0);
-                    }
-                    self.model.nodes = Vec::new();
-                    self.close_lane += 1;
-                    bytes
-                }
-                2 => {
-                    let Some(element) = self.model.elements.last_mut() else {
-                        self.close_lane += 1;
-                        continue;
-                    };
-                    if let Some(next_bytes) = element.mounted_next_string_bytes() {
-                        if next_bytes > maximum_bytes {
-                            return (false, 0, 0);
-                        }
-                        let bytes = element.close_mounted_string_step().expect("mounted element next-owner witness changed without mutation");
-                        return (false, 1, bytes);
-                    }
-                    if !element.mounted_strings_terminal_is_empty() {
-                        return (false, 0, 0);
-                    }
-                    self.model.elements.pop();
-                    return (false, 1, 0);
-                }
-                3 => {
-                    let bytes = self.model.elements.capacity() * size_of::<semio_s_fem_static::Elements>();
-                    if bytes > maximum_bytes {
-                        return (false, 0, 0);
-                    }
-                    self.model.elements = Vec::new();
-                    self.close_lane += 1;
-                    bytes
-                }
-                4 => {
-                    let Some(support) = self.model.supports.last_mut() else {
-                        self.close_lane += 1;
-                        continue;
-                    };
-                    if support.node_id.capacity() != 0 {
-                        let bytes = support.node_id.capacity();
-                        if bytes > maximum_bytes {
-                            return (false, 0, 0);
-                        }
-                        support.node_id = String::new();
-                        return (false, 1, bytes);
-                    }
-                    if support.fixed.pop().is_some() {
-                        return (false, 1, 0);
-                    }
-                    let bytes = support.fixed.capacity() * size_of::<semio_s_fem_model::Dof>();
-                    if bytes != 0 {
-                        if bytes > maximum_bytes {
-                            return (false, 0, 0);
-                        }
-                        support.fixed = Vec::new();
-                        return (false, 1, bytes);
-                    }
-                    self.model.supports.pop();
-                    return (false, 1, 0);
-                }
-                5 => {
-                    let bytes = self.model.supports.capacity() * size_of::<semio_s_fem_model::Support>();
-                    if bytes > maximum_bytes {
-                        return (false, 0, 0);
-                    }
-                    self.model.supports = Vec::new();
-                    self.close_lane += 1;
-                    bytes
-                }
-                6 => {
-                    self.close_lane += 1;
+                0..=6 => {
+                    let progress = self.model.close_step(maximum_bytes);
+                    if !progress.0 { return progress; }
+                    self.close_lane = 7;
                     continue;
                 }
@@ -923,7 +864,153 @@

 //#endregion 🔖️Contract
 
+#[derive(Clone, Copy, PartialEq, Eq)]
+enum LoadBuildStage { Select, Expand, AdmitFull, AdmitFree, ZeroFull, ZeroFree, Nodal, Element, Positions, ElementLoads, ElementRows, Complete }
+
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+enum ResolvedResultSource { LoadCase(usize), Combination(usize) }
+
+struct MountedLoadBuild {
+    source: Option<ResolvedResultSource>,
+    stage: LoadBuildStage,
+    factors: [f64; 64],
+    stack: [(usize, usize, f64); 64],
+    depth: usize,
+    case: usize,
+    load: usize,
+    element: usize,
+    position: usize,
+    lookup: usize,
+    positions: [[f64; 3]; 3],
+    row: usize,
+    udl: semio_s_fem_model::MemberUdl,
+    pressure: f64,
+    full: MountedScalarSlots,
+    free: MountedScalarSlots,
+    reactions: MountedScalarSlots,
+}
+
+impl MountedLoadBuild {
+    fn new(source: Option<ResolvedResultSource>) -> Self {
+        Self { source, stage: LoadBuildStage::Select, factors: [0.0; 64], stack: [(0, 0, 0.0); 64], depth: 0, case: 0, load: 0, element: 0, position: 0, lookup: 0, positions: [[0.0; 3]; 3], row: 0, udl: semio_s_fem_model::MemberUdl { wx: 0.0, wy: 0.0, wz: 0.0 }, pressure: 0.0, full: MountedScalarSlots::new(), free: MountedScalarSlots::new(), reactions: MountedScalarSlots::new() }
+    }
+
+    fn add(&mut self, assembly: &AssemblyJob<'static>, node: &str, dof: semio_s_fem_model::Dof, value: f64) -> Result<(), Vec<u8>> {
+        if !value.is_finite() { return Err(b"fem2d.load-non-finite".to_vec()); }
+        if let Some((full, free)) = assembly.visual_equation_indices(node, dof) {
+            self.full.add_at(full, value).map_err(|_| b"fem2d.load-full-index".to_vec())?;
+            if let Some(free) = free { self.free.add_at(free, value).map_err(|_| b"fem2d.load-free-index".to_vec())?; }
+        } else if value != 0.0 { return Err(b"fem2d.load-inactive-node-dof".to_vec()); }
+        Ok(())
+    }
+
+    fn step_one(&mut self, snapshot: &Fem2dSnapshot, assembly: &AssemblyJob<'static>) -> Result<bool, Vec<u8>> {
+        if snapshot.load_cases.len() > self.factors.len() || snapshot.combinations.len() > self.stack.len() { return Err(b"fem2d.source-capacity".to_vec()); }
+        match self.stage {
+            LoadBuildStage::Select => {
+                match self.source {
+                    Some(ResolvedResultSource::LoadCase(index)) => {
+                        if snapshot.load_cases.get(index).is_none() { return Err(b"fem2d.source-case-missing".to_vec()); }
+                        self.factors[index] = 1.0;
+                        self.stage = LoadBuildStage::AdmitFull;
+                    }
+                    Some(ResolvedResultSource::Combination(index)) => {
+                        if snapshot.combinations.get(index).is_none() { return Err(b"fem2d.source-combination-missing".to_vec()); }
+                        self.stack[0] = (index, 0, 1.0);
+                        self.depth = 1;
+                        self.stage = LoadBuildStage::Expand;
+                    }
+                    None => self.stage = LoadBuildStage::AdmitFull,
+                }
+            }
+            LoadBuildStage::Expand => {
+                if self.depth == 0 { self.stage = LoadBuildStage::AdmitFull; return Ok(false); }
+                let (index, term, factor) = self.stack[self.depth - 1];
+                if let Some(term) = snapshot.combinations[index].terms.get(term) {
+                    self.stack[self.depth - 1].1 += 1;
+                    let factor = factor * term.factor;
+                    if !factor.is_finite() { return Err(b"fem2d.source-factor".to_vec()); }
+                    if let Some(case) = snapshot.load_cases.iter().position(|case| case.id == term.case_id) {
+                        self.factors[case] += factor;
+                        if !self.factors[case].is_finite() { return Err(b"fem2d.source-factor".to_vec()); }
+                    } else {
+                        let nested = snapshot.combinations.iter().position(|combination| combination.id == term.case_id).ok_or_else(|| b"fem2d.source-term-missing".to_vec())?;
+                        if self.depth == self.stack.len() || self.stack[..self.depth].iter().any(|frame| frame.0 == nested) { return Err(b"fem2d.source-cycle".to_vec()); }
+                        self.stack[self.depth] = (nested, 0, factor);
+                        self.depth += 1;
+                    }
+                } else { self.depth -= 1; }
+            }
+            LoadBuildStage::AdmitFull => if self.full.admit_one(assembly.visual_full_order()).map_err(|_| b"fem2d.load-full-capacity".to_vec())? { self.stage = LoadBuildStage::AdmitFree; },
+            LoadBuildStage::AdmitFree => if self.free.admit_one(assembly.visual_free_order()).map_err(|_| b"fem2d.load-free-capacity".to_vec())? { self.stage = LoadBuildStage::ZeroFull; },
+            LoadBuildStage::ZeroFull => if self.full.len() < assembly.visual_full_order() { self.full.push(0.0).map_err(|_| b"fem2d.load-full-owner".to_vec())?; } else { self.stage = LoadBuildStage::ZeroFree; },
+            LoadBuildStage::ZeroFree => if self.free.len() < assembly.visual_free_order() { self.free.push(0.0).map_err(|_| b"fem2d.load-free-owner".to_vec())?; } else { self.stage = LoadBuildStage::Nodal; },
+            LoadBuildStage::Nodal => {
+                let Some(case) = snapshot.load_cases.get(self.case) else { self.stage = LoadBuildStage::Complete; return Ok(false); };
+                if self.factors[self.case] == 0.0 { self.case += 1; self.load = 0; return Ok(false); }
+                if let Some(load) = case.loads.get(self.load) {
+                    match load {
+                        FemLoad::Nodal { node_id, dof, value, .. } => self.add(assembly, node_id, (*dof).into(), value * self.factors[self.case])?,
+                        FemLoad::MemberUdl { element_id, .. } if !snapshot.elements.iter().any(|element| semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id(element) == element_id) => return Err(b"fem2d.load-member-missing".to_vec()),
+                        FemLoad::Area { region_id, .. } if !snapshot.regions.iter().any(|region| region.id == *region_id) => return Err(b"fem2d.load-region-missing".to_vec()),
+                        _ => {}
+                    }
+                    self.load += 1;
+                } else { self.element = 0; self.stage = LoadBuildStage::Element; }
+            }
+            LoadBuildStage::Element => {
+                if assembly.model_element(self.element).is_none() { self.case += 1; self.load = 0; self.stage = LoadBuildStage::Nodal; }
+                else { self.position = 0; self.lookup = 0; self.udl = semio_s_fem_model::MemberUdl { wx: 0.0, wy: 0.0, wz: 0.0 }; self.pressure = 0.0; self.stage = LoadBuildStage::Positions; }
+            }
+            LoadBuildStage::Positions => {
+                let element = assembly.model_element(self.element).ok_or_else(|| b"fem2d.load-element-owner".to_vec())?;
+                if let Some(id) = element.mounted_node_id(self.position) {
+                    let node = assembly.model_node(self.lookup).ok_or_else(|| b"fem2d.load-node-missing".to_vec())?;
+                    if node.id == id { self.positions[self.position] = node.pos; self.position += 1; self.lookup = 0; } else { self.lookup += 1; }
+                } else { self.load = 0; self.stage = LoadBuildStage::ElementLoads; }
+            }
+            LoadBuildStage::ElementLoads => {
+                let element = assembly.model_element(self.element).ok_or_else(|| b"fem2d.load-element-owner".to_vec())?;
+                if let Some(load) = snapshot.load_cases[self.case].loads.get(self.load) {
+                    match load {
+                        FemLoad::MemberUdl { element_id, wx, wy, .. } if element.id() == element_id => { self.udl.wx += wx; self.udl.wy += wy; }
+                        FemLoad::Area { region_id, pressure, .. } if matches!(element, semio_s_fem_static::Elements::Tri3Cst(_)) => {
+                            if element.id().strip_prefix(region_id).and_then(|suffix| suffix.strip_prefix("_t")).is_some_and(|suffix| suffix.parse::<usize>().is_ok()) { self.pressure += pressure; }
+                        }
+                        _ => {}
+                    }
+                    self.load += 1;
+                } else { self.row = 0; self.stage = LoadBuildStage::ElementRows; }
+            }
+            LoadBuildStage::ElementRows => {
+                let element = assembly.model_element(self.element).ok_or_else(|| b"fem2d.load-element-owner".to_vec())?;
+                let dofs = element.dofs_per_node();
+                let nodes = element.mounted_node_id_count().ok_or_else(|| b"fem2d.load-element-shape".to_vec())?;
+                if self.row == nodes * dofs.len() { self.element += 1; self.stage = LoadBuildStage::Element; return Ok(false); }
+                let gravity = [0.0, if snapshot.load_cases[self.case].self_weight { -9.81 } else { 0.0 }];
+                let value = match element {
+                    semio_s_fem_static::Elements::Bar2(element) => element.mounted_load_row(&self.positions[..nodes], self.row, &self.udl, gravity),
+                    semio_s_fem_static::Elements::BeamEb2(element) => element.mounted_load_row(&self.positions[..nodes], self.row, &self.udl, gravity),
+                    semio_s_fem_static::Elements::Tri3Cst(element) => element.mounted_load_row(&self.positions[..nodes], self.row, self.pressure, gravity),
+                    _ => None,
+                }.ok_or_else(|| b"fem2d.load-row-shape".to_vec())?;
+                self.add(assembly, element.mounted_node_id(self.row / dofs.len()).ok_or_else(|| b"fem2d.load-node-owner".to_vec())?, dofs[self.row % dofs.len()], value * self.factors[self.case])?;
+                self.row += 1;
+            }
+            LoadBuildStage::Complete => return Ok(true),
+        }
+        Ok(false)
+    }
+
+    fn close_step(&mut self) -> bool {
+        if !self.full.close_step() || !self.free.close_step() || !self.reactions.close_step() { return false; }
+        self.source = None;
+        true
+    }
+}
+
 //#region 🧰️FixedArena
 struct MountedState {
     identity: MountedIdentity,
+    source: Option<ResolvedResultSource>,
     snapshot: Option<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>,
     snapshot_return: Option<store::os_store::snapshot_read::SnapshotReadReturn>,
@@ -934,4 +1021,5 @@

     graph: Option<FemJobGraph>,
     domain: Option<PlanarDomain>,
+    region_cursor: usize,
     domain_outer_cursor: usize,
     domain_hole_cursor: usize,
@@ -943,5 +1031,7 @@

     assembly: Option<AssemblyJob<'static>>,
     csr_build: Option<AssemblyCsrBuild>,
+    load_build: Option<MountedLoadBuild>,
     pcg_build: Option<PcgJobConstruction>,
+    rejected_pcg_matrix: Option<Csr>,
     pcg: Option<PcgJob>,
     visual: Fem2dLiveVisual,
@@ -953,4 +1043,5 @@

     visual_dirty: bool,
     visual_field_cursor: usize,
+    reaction_entry: usize,
     visual_pcg_complete: bool,
     preview_sequence: u64,
@@ -986,14 +1077,13 @@

 }
 
-fn child_end(result: Result<Option<JobOutcomeBorrow<'_>>, ValueError>) -> ChildEnd {
-    match result {
-        Ok(None | Some(JobOutcomeBorrow::Yield { .. })) => ChildEnd::Yield,
-        Ok(Some(JobOutcomeBorrow::PreviewReady { payload, .. })) => ChildEnd::Preview { bytes: lent_bytes(payload, SESSION_MAXIMUM_OUTPUT_BYTES), quality: retained_payload_byte(payload, 8) },
-        Ok(Some(JobOutcomeBorrow::CheckpointReady { state, .. })) => ChildEnd::Checkpoint(lent_bytes(state, SESSION_MAXIMUM_OUTPUT_BYTES)),
-        Ok(Some(JobOutcomeBorrow::Complete { output, .. })) => ChildEnd::Complete(output.and_then(|payload| lent_bytes(payload, SESSION_MAXIMUM_OUTPUT_BYTES))),
-        Ok(Some(JobOutcomeBorrow::Fault { detail, .. })) => ChildEnd::Fault(lent_bytes(detail, SESSION_MAXIMUM_FAULT_BYTES)),
-        Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => ChildEnd::Cancelled,
-        Err(_) => ChildEnd::Fault(None),
-    }
+fn child_end(result: Result<Option<JobOutcomeBorrow<'_>>, semio_framework_job::OriginalTurnFailure>) -> Result<ChildEnd, semio_framework_job::OriginalTurnFailure> {
+    Ok(match result? {
+        None | Some(JobOutcomeBorrow::Yield { .. }) => ChildEnd::Yield,
+        Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => ChildEnd::Preview { bytes: lent_bytes(payload, SESSION_MAXIMUM_OUTPUT_BYTES), quality: retained_payload_byte(payload, 8) },
+        Some(JobOutcomeBorrow::CheckpointReady { state, .. }) => ChildEnd::Checkpoint(lent_bytes(state, SESSION_MAXIMUM_OUTPUT_BYTES)),
+        Some(JobOutcomeBorrow::Complete { output, .. }) => ChildEnd::Complete(output.and_then(|payload| lent_bytes(payload, SESSION_MAXIMUM_OUTPUT_BYTES))),
+        Some(JobOutcomeBorrow::Fault { detail, .. }) => ChildEnd::Fault(lent_bytes(detail, SESSION_MAXIMUM_FAULT_BYTES)),
+        Some(JobOutcomeBorrow::Cancelled { .. }) => ChildEnd::Cancelled,
+    })
 }
 
@@ -1002,5 +1092,4 @@

     match step {
         InteractiveJobCloseStep::Blocked => Some(PluginLifecycleStep::Blocked { reason: "mounted FEM child close is blocked" }),
-        InteractiveJobCloseStep::Refused { .. } => Some(PluginLifecycleStep::Blocked { reason: "mounted FEM child close was refused" }),
         InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. } => None,
     }
@@ -1024,5 +1113,5 @@

 
 /// 📏️ The largest logical element one close unit retires; every quote of the next close turn is bounded by it.
-const SESSION_CLOSE_COPY_BYTES: usize = 4_096;
+const SESSION_CLOSE_COPY_BYTES: usize = 4096;
 
 /// 🪜️ The deepest retained frontier one close unit reaches: session owner, job, writer and payload.
@@ -1041,4 +1130,5 @@

     size_of::<PcgJobConstruction>(),
     size_of::<PcgJob>(),
+    size_of::<MountedLoadBuild>(),
     size_of::<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>(),
     size_of::<store::os_store::snapshot_read::SnapshotReadReturn>(),
@@ -1048,7 +1138,16 @@

 
 impl MountedState {
+    fn child_close_demands(&self, demand: &mut RetirementDemand) -> Result<(), ValueError> {
+        if let Some(child) = self.graph.as_ref() { merge_close_demand(demand, semio_s_fem_sparse::numerical_close_demand(child)?); }
+        if let Some(child) = self.mesh.as_ref() { merge_close_demand(demand, semio_s_fem_sparse::numerical_close_demand(child)?); }
+        if let Some(child) = self.pcg.as_ref() { merge_close_demand(demand, semio_s_fem_sparse::numerical_close_demand(child)?); }
+        if let Some(child) = self.pcg_build.as_ref() { merge_close_demand(demand, child.close_demand()?); }
+        Ok(())
+    }
+
     fn new(identity: MountedIdentity, snapshot: store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>, admitted_items: usize) -> Self {
         Self {
             identity,
+            source: None,
             snapshot: Some(snapshot),
             snapshot_return: None,
@@ -1059,4 +1158,5 @@

             graph: None,
             domain: None,
+            region_cursor: 0,
             domain_outer_cursor: 0,
             domain_hole_cursor: 0,
@@ -1068,5 +1168,7 @@

             assembly: None,
             csr_build: None,
+            load_build: None,
             pcg_build: None,
+            rejected_pcg_matrix: None,
             pcg: None,
             visual: Fem2dLiveVisual::default(),
@@ -1078,4 +1180,5 @@

             visual_dirty: true,
             visual_field_cursor: 0,
+            reaction_entry: 0,
             visual_pcg_complete: false,
             preview_sequence: 0,
@@ -1102,5 +1205,5 @@

     fn prepare_domain_one(&mut self) -> Result<bool, &'static [u8]> {
         let snapshot = self.snapshot.as_ref().ok_or(b"fem2d.session-snapshot-missing" as &'static [u8])?;
-        let Some(region) = snapshot.regions.first() else { return Ok(true) };
+        let Some(region) = snapshot.regions.get(self.region_cursor) else { return Ok(true) };
         if self.domain.is_none() {
             let mut outer = Vec::new();
@@ -1202,5 +1305,5 @@

             Ok(true) => {
                 let Some(lease) = candidate.take_complete() else { return Some(self.fail(b"fem2d.visual-complete-owner".to_vec())) };
-                let live = current_identity(self.identity.app_instance_id) == Some(self.identity) && snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision) && !self.cancel.is_cancelled_now();
+                let live = current_identity(self.identity) == Some(self.identity) && snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision) && !self.cancel.is_cancelled_now();
                 if !live {
                     self.visual_displaced = Some(lease);
@@ -1224,20 +1327,20 @@

     }
 
-    fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep {
+    fn step(&mut self, cx: &mut StepContext<'_>) -> Result<JobStep, semio_framework_job::OriginalTurnFailure> {
         if self.cancel.is_cancelled_now() {
             self.stage = MountedStage::Closing;
-            return self.fail(b"fem2d.session-cancelled".to_vec());
-        }
-        if budget.fuel == 0 || budget.deadline_ms == 0 {
-            return JobStep::Running(None);
+            return Ok(self.fail(b"fem2d.session-cancelled".to_vec()));
+        }
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
-            }
-            match self.stage {
+                return Ok(step);
+            }
+            Ok(match self.stage {
                 MountedStage::PrepareGraph => {
                     const PLANS: [FemStagePlan; 8] = [
@@ -1253,5 +1356,5 @@

                     if self.graph_plans.capacity() == 0 {
                         if self.graph_plans.try_reserve_exact(PLANS.len()).is_err() || self.graph_plans.capacity() * size_of::<FemStagePlan>() > SESSION_OWNER_PAGE_BYTES {
-                            return self.fail(b"fem2d.graph-plan-allocation".to_vec());
+                            return Ok(self.fail(b"fem2d.graph-plan-allocation".to_vec()));
                         }
                     } else if let Some(plan) = PLANS.get(self.graph_plans.len()).cloned() {
@@ -1264,5 +1367,5 @@

                     self.progress(b"fem2d.graph-admitted")
                 }
-                MountedStage::Graph => match child_end(self.graph.as_mut().expect("graph stage owns graph").step(cx)) {
+                MountedStage::Graph => match child_end(self.graph.as_mut().expect("graph stage owns graph").step(cx))? {
                     ChildEnd::Complete(output) => {
                         self.stage = MountedStage::PrepareDomain;
@@ -1287,7 +1390,7 @@

                     Ok(true) => {
                         let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
-                        if let (Some(region), Some(domain)) = (snapshot.regions.first(), self.domain.take()) {
+                        if let (Some(region), Some(domain)) = (snapshot.regions.get(self.region_cursor), self.domain.take()) {
                             if self.visual.region_quality.insert(region.id.clone(), RegionVisualQuality::Unmeshed).is_err() {
-                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
+                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                             }
                             self.visual.state = FemVisualState::Unmeshed;
@@ -1303,14 +1406,20 @@

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
@@ -1320,5 +1429,5 @@

                     }
                     ChildEnd::Preview { bytes: payload, quality } => {
-                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
+                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.get(self.region_cursor)) {
                             let quality = match quality {
                                 Some(0) => RegionVisualQuality::Coarse,
@@ -1328,5 +1437,5 @@

                             };
                             if !self.visual.region_quality.update(&region.id, quality) {
-                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
+                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                             }
                             self.visual.state = match quality {
@@ -1355,5 +1464,5 @@

                         self.model_build = Some(MountedModelBuild::new(None));
                         cx.consume_fuel(1);
-                        return self.progress(b"fem2d.model-admitted");
+                        return Ok(self.progress(b"fem2d.model-admitted"));
                     }
                     match self.model_build.as_mut().expect("model builder admitted above").step_one(snapshot) {
@@ -1363,6 +1472,12 @@

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
@@ -1373,4 +1488,24 @@

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
@@ -1381,5 +1516,5 @@

                         self.assembly = self.assembly_build.as_mut().and_then(AssemblyJobConstruction::take_complete);
                         if self.assembly.is_none() {
-                            return self.fail(b"fem2d.assembly-construction-false-terminal".to_vec());
+                            return Ok(self.fail(b"fem2d.assembly-construction-false-terminal".to_vec()));
                         }
                         self.stage = MountedStage::Assembly;
@@ -1389,16 +1524,8 @@

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
@@ -1414,5 +1541,24 @@

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
@@ -1420,6 +1566,11 @@

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
@@ -1436,5 +1587,5 @@

                         self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                         if self.pcg.is_none() {
-                            return self.fail(b"fem2d.pcg-false-terminal".to_vec());
+                            return Ok(self.fail(b"fem2d.pcg-false-terminal".to_vec()));
                         }
                         self.stage = MountedStage::Pcg;
@@ -1444,5 +1595,5 @@

                     Err(detail) => self.fail(detail.to_vec()),
                 },
-                MountedStage::Pcg => match child_end(self.pcg.as_mut().expect("pcg stage owns pcg").step(cx)) {
+                MountedStage::Pcg => match child_end(self.pcg.as_mut().expect("pcg stage owns pcg").step(cx))? {
                     ChildEnd::Complete(output) => {
                         if let Some(job) = self.pcg.as_ref() {
@@ -1457,5 +1608,5 @@

                         self.visual_field_cursor = 0;
                         self.visual_pcg_complete = true;
-                        self.stage = MountedStage::SyncPcgVisual;
+                        self.stage = MountedStage::RecoverReactions;
                         match output {
                             Some(bytes) => JobStep::Running(Some(bytes)),
@@ -1485,22 +1636,39 @@

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
@@ -1514,10 +1682,17 @@

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
@@ -1527,8 +1702,8 @@

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
@@ -1544,40 +1719,44 @@

                 MountedStage::Fault => JobStep::Failed(self.fault.clone().unwrap_or_else(|| b"fem2d.session-fault".to_vec())),
                 MountedStage::Closing | MountedStage::Empty => JobStep::Failed(b"fem2d.session-closed".to_vec()),
-            }
+            })
         })()
     }
 
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
@@ -1585,13 +1764,11 @@

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
@@ -1601,8 +1778,8 @@

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
@@ -1612,8 +1789,8 @@

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
@@ -1623,8 +1800,8 @@

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
@@ -1634,8 +1811,8 @@

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
@@ -1643,27 +1820,28 @@

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
@@ -1672,26 +1850,26 @@

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
@@ -1701,13 +1879,13 @@

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
@@ -1720,19 +1898,19 @@

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
@@ -1743,19 +1921,19 @@

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
@@ -1765,5 +1943,5 @@

                         if let Some(owner) = self.visual.region_quality.take_one() {
                             self.visual_region_owner = Some(owner);
-                            return session_pending(1, 0);
+                            return Ok(session_pending(1, 0));
                         }
                     }
@@ -1772,11 +1950,11 @@

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
@@ -1786,22 +1964,22 @@

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
@@ -1811,11 +1989,11 @@

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
@@ -1825,11 +2003,11 @@

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
@@ -1838,14 +2016,14 @@

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
@@ -1854,13 +2032,13 @@

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
@@ -1868,5 +2046,5 @@

                 _ => {
                     self.stage = MountedStage::Empty;
-                    return PluginLifecycleStep::Complete(RetainedCloneProgress::default());
+                    return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
                 }
             }
@@ -1885,4 +2063,7 @@

             && self.assembly.is_none()
             && self.csr_build.is_none()
+            && self.load_build.is_none()
+            && self.source.is_none()
+            && self.rejected_pcg_matrix.is_none()
             && self.pcg_build.is_none()
             && self.pcg.is_none()
@@ -1905,5 +2086,10 @@

     shell: u16,
     identity: MountedIdentity,
-}
+    previous_valid: Option<u16>,
+}
+
+/// 🧷️ Retirement keeps immutable app affinity while its original shell is checked out.
+#[derive(Clone, Copy)]
+struct RetiringShell { shell: u16, app_instance_id: u32 }
 
 #[derive(Clone, Copy)]
@@ -1954,9 +2140,9 @@

         if !self.owner_opened && !matches!(self.lane, 4 | 9 | 11) {
             let bytes = match self.lane {
-                0 => snapshot.nodes.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemNode>(),
+                0 => snapshot.nodes.capacity() * size_of::<semio_s_fem_node::planar::FemNodeValue>(),
                 1 => snapshot.elements.capacity() * size_of::<FemElement>(),
                 2 => snapshot.regions.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion>(),
                 3 => 0,
-                5 => snapshot.materials.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemMaterial>(),
+                5 => snapshot.materials.capacity() * size_of::<semio_s_fem_material::planar::FemMaterialValue>(),
                 6 => snapshot.sections.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection>(),
                 7 => snapshot.supports.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSupport>(),
@@ -1989,5 +2175,5 @@

             }
             2 => {
-                if snapshot.regions.len() > 1 {
+                if snapshot.regions.len() > 64 {
                     return Err(b"fem2d.session-region-capacity");
                 }
@@ -2155,11 +2341,26 @@

 }
 
+
 struct MountedRegistry {
+    app_slots: semio_framework_value::FixedKeySlots<u32, SESSION_ACTIVE_CAPACITY>,
+    requests: semio_framework_value::FixedRequestOwners<RequestKey, ConsumerKey, SESSION_ACTIVE_CAPACITY, SESSION_CONSUMER_CAPACITY>,
+    families: [semio_s_fem_app_surface::ResultMode; SESSION_ACTIVE_CAPACITY],
+    mode_indices: [usize; SESSION_ACTIVE_CAPACITY],
+    request_cancelled: [bool; SESSION_ACTIVE_CAPACITY],
+    model_requests: [Option<RequestKey>; SESSION_ACTIVE_CAPACITY],
+    snapshot_owners: [Option<semio_framework_value::FixedKeySlot>; SESSION_ACTIVE_CAPACITY],
+    shell_requests: [Option<semio_framework_value::FixedKeySlot>; SESSION_SHELL_CAPACITY],
+    request_cursor: usize,
+    consumer_cursor: usize,
+    verified_sources: [Option<(semio_framework_value::RequestCandidateVersion,RequestKey)>;SESSION_CONSUMER_CAPACITY],
+    source_kinds: [u8;SESSION_ACTIVE_CAPACITY],
     shells: [Rc<RefCell<Option<MountedState>>>; SESSION_SHELL_CAPACITY],
     current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY],
+    sources: [Option<ResolvedResultSource>; SESSION_ACTIVE_CAPACITY],
+    source_apps: [Option<u32>; SESSION_ACTIVE_CAPACITY],
     preflight: [Option<PendingSnapshotAdmission>; SESSION_ACTIVE_CAPACITY],
     preflight_fault: [Option<PendingSnapshotFault>; SESSION_ACTIVE_CAPACITY],
     pending: [Option<PendingAdmission>; SESSION_ACTIVE_CAPACITY],
-    retiring: [Option<u16>; SESSION_SHELL_CAPACITY],
+    retiring: [Option<RetiringShell>; SESSION_SHELL_CAPACITY],
     free: [u16; SESSION_SHELL_CAPACITY],
     free_read: usize,
@@ -2175,6 +2376,20 @@

     fn new() -> Self {
         Self {
+            app_slots: Default::default(),
+            requests: Default::default(),
+            families: [semio_s_fem_app_surface::ResultMode::Static; SESSION_ACTIVE_CAPACITY],
+            mode_indices: [0; SESSION_ACTIVE_CAPACITY],
+            request_cancelled: [false; SESSION_ACTIVE_CAPACITY],
+            model_requests: [None; SESSION_ACTIVE_CAPACITY],
+            snapshot_owners: [None; SESSION_ACTIVE_CAPACITY],
+            shell_requests: [None; SESSION_SHELL_CAPACITY],
+            request_cursor: 0,
+            consumer_cursor: 0,
+            verified_sources:[None;SESSION_CONSUMER_CAPACITY],
+            source_kinds:[2;SESSION_ACTIVE_CAPACITY],
             shells: std::array::from_fn(|_| Rc::new(RefCell::new(None))),
             current: [None; SESSION_ACTIVE_CAPACITY],
+            sources: std::array::from_fn(|_| None),
+            source_apps: [None; SESSION_ACTIVE_CAPACITY],
             preflight: [None; SESSION_ACTIVE_CAPACITY],
             preflight_fault: [None; SESSION_ACTIVE_CAPACITY],
@@ -2205,4 +2420,5 @@

         assert_eq!(self.credit_items[shell as usize], 0, "mounted FEM shell released before its process item credit");
         assert_eq!(self.credit_bytes[shell as usize], 0, "mounted FEM shell released before its process byte credit");
+        self.shell_requests[shell as usize] = None;
         let write = (self.free_read + self.free_len) % SESSION_SHELL_CAPACITY;
         self.free[write] = shell;
@@ -2234,7 +2450,7 @@

     }
 
-    fn retain_retirement(&mut self, shell: u16) -> bool {
+    fn retain_retirement(&mut self, shell: u16, app_instance_id: u32) -> bool {
         let Some(slot) = self.retiring.iter_mut().find(|slot| slot.is_none()) else { return false };
-        *slot = Some(shell);
+        *slot = Some(RetiringShell { shell, app_instance_id });
         true
     }
@@ -2258,32 +2474,25 @@

     /// publishes output, or the session leaves `Running` — the host owes the job one round trip per
     /// ceiling, never one per unit.
-    fn step(&mut self, budget: JobBudget, _original: &mut IoRunControl<'_, '_>, _snapshot: &mut SqliteSnapshotControl<'_>, cx: &mut StepContext<'_>) -> Result<JobStep, ValueError> {
-        let Ok(mut shell) = self.shell.try_borrow_mut() else { return Ok(JobStep::Running(None)) };
-        let Some(state) = shell.as_mut() else { return Ok(JobStep::Failed(b"fem2d.session-owner-missing".to_vec())) };
-        if state.identity != self.identity {
-            return Ok(JobStep::Failed(b"fem2d.session-aba".to_vec()));
-        }
-        let Some(started) = semio_framework_job::default_now_us() else { return Ok(state.step(budget, cx)) };
-        let ceiling_ms = u64::from(budget.deadline_ms).min(SESSION_STEP_CEILING_MS);
-        let deadline = started.saturating_add(ceiling_ms.saturating_mul(1_000));
-        let mut fuel = budget.fuel;
-        let mut step = JobStep::Running(None);
-        for _ in 0..SESSION_UNITS_PER_STEP {
-            let now = semio_framework_job::default_now_us().unwrap_or(deadline);
-            if fuel == 0 || now >= deadline || cx.should_yield() {
-                break;
-            }
-            let remaining_ms = u32::try_from(deadline.saturating_sub(now).div_ceil(1_000)).unwrap_or(u32::MAX).max(1);
-            step = state.step(JobBudget { fuel, deadline_ms: remaining_ms }, cx);
-            fuel = fuel.saturating_sub(1);
-            if !matches!(step, JobStep::Running(None)) {
-                break;
-            }
-        }
+    fn step(&mut self, _original: &mut IoRunControl<'_, '_>, _snapshot: &mut SqliteSnapshotControl<'_>, cx: &mut StepContext<'_>) -> Result<JobStep, semio_framework_job::OriginalTurnFailure> {
+        let step = (|| {
+            let Ok(mut shell) = self.shell.try_borrow_mut() else { return Ok(JobStep::Running(None)); };
+            let Some(state) = shell.as_mut() else { return Ok(JobStep::Failed(b"fem2d.session-owner-missing".to_vec())); };
+            if state.identity != self.identity { return Ok(JobStep::Failed(b"fem2d.session-aba".to_vec())); }
+            let Some(started) = semio_framework_job::default_now_us() else { return state.step(cx); };
+            let deadline = started.saturating_add(SESSION_STEP_CEILING_MS.saturating_mul(1_000)).min(cx.deadline_us());
+            let mut step = JobStep::Running(None);
+            for _ in 0..SESSION_UNITS_PER_STEP {
+                if cx.should_yield() || semio_framework_job::default_now_us().unwrap_or(deadline) >= deadline { break; }
+                step = state.step(cx)?;
+                if !matches!(step, JobStep::Running(None)) { break; }
+            }
+            Ok::<_, semio_framework_job::OriginalTurnFailure>(step)
+        })()?;
+        match &step { JobStep::Done(_) => record_terminal(self.identity, semio_framework_value::RequestTerminalKind::Validated), JobStep::Failed(_) => record_terminal(self.identity, semio_framework_value::RequestTerminalKind::Faulted), _ => {} }
         Ok(step)
     }
 
     /// 🧹️ The heavy session owner is retired by the registry's own close ladder; this handle releases its two original input buffers and itself.
-    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, ValueError> {
+    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
         let grant = cx.retained_grant();
         let demand = self.retirement_demands(0)?;
@@ -2388,9 +2597,11 @@

 
 //#region 🎛️LiveMount
-fn current_identity(app_instance_id: u32) -> Option<MountedIdentity> {
+fn source_version(render:AppRenderOperationContext)->semio_framework_value::RequestSourceVersion{semio_framework_value::RequestSourceVersion{base_revision:render.base_revision.0,generation:render.generation.0,canonical_revision:render.canonical_base_revision}}
+fn identity_version(identity:MountedIdentity)->semio_framework_value::RequestSourceVersion{semio_framework_value::RequestSourceVersion{base_revision:identity.base_revision.0,generation:identity.generation.0,canonical_revision:identity.canonical_base_revision}}
+fn record_terminal(identity:MountedIdentity,kind:semio_framework_value::RequestTerminalKind){MOUNTED.with(|registry|{let mut registry=registry.borrow_mut();if let Some(index)=registry.current.iter().position(|current|current.is_some_and(|current|current.identity==identity)){if let Some((owner,_,_))=registry.requests.at(index){registry.requests.record_terminal(owner,identity_version(identity),identity.job,kind);}}});}
+fn current_identity(identity: MountedIdentity) -> Option<MountedIdentity> {
     MOUNTED.with(|registry| {
         let registry = registry.borrow();
-        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
-        registry.current[slot].filter(|current| current.app_instance_id == app_instance_id).map(|current| current.identity)
+        registry.current.iter().enumerate().find_map(|(index, current)| { let current = current.filter(|current| current.identity == identity)?; let (owner, _, _) = registry.requests.at(index)?; (registry.requests.references(owner) != 0).then_some(current.identity) })
     })
 }
@@ -2399,14 +2610,41 @@

 /// request an opaque snapshot lease. The fixed shell and exact process credit are reserved only
 /// after the census reaches its complete witness.
-pub fn prepare_snapshot_read(render: AppRenderOperationContext, snapshot: &Fem2dSnapshot) -> bool {
+fn prepare_snapshot_admission(render: AppRenderOperationContext, snapshot: &Fem2dSnapshot,grant:RetainedCloneGrant) -> semio_framework_plugin::MountedSnapshotReadPreparation {
     if render.app_instance_id == 0 {
-        return false;
-    }
-    MOUNTED.with(|registry| {
+        return Default::default();
+    }
+    let mut progress=RetainedCloneProgress::default();
+    let ready =     MOUNTED.with(|registry| {
         let mut registry = registry.borrow_mut();
-        let current_slot = render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
+        let Some(app_slot) = registry.app_slots.lookup(render.app_instance_id).map(|slot| slot.index()) else { return false };
+        let candidate = (0..SESSION_ACTIVE_CAPACITY).find_map(|offset| {
+            let index = (registry.request_cursor + offset) % SESSION_ACTIVE_CAPACITY;
+            let (owner, scope, _) = registry.requests.at(index)?;
+            if scope != render.app_instance_id || !registry.requests.ready(owner) || registry.requests.terminal(owner,source_version(render)).is_some() { return None; }
+            if registry.current[index].is_some_and(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision)) { return None; }
+            Some(owner)
+        });
+        let Some(owner) = candidate else { return false };
+        let current_slot = owner.index();
+        if registry.source_kinds[current_slot]!=2 {
+            let version=source_version(render);let lookup=registry.requests.source_lookup(owner,version);
+            if let Some((position,semio_framework_value::RequestLookupOutcome::Matched))=lookup{registry.sources[current_slot]=Some(if registry.source_kinds[current_slot]==0{ResolvedResultSource::LoadCase(position)}else{ResolvedResultSource::Combination(position)});}
+            else if lookup.is_some_and(|(_,outcome)|outcome==semio_framework_value::RequestLookupOutcome::Missing){return false;}
+            else{if grant.maximum_copy_bytes<8{return false;}let position=lookup.map_or(0,|(position,_)|position);let candidate=if registry.source_kinds[current_slot]==0{snapshot.load_cases.get(position).map(|case|case.id.as_str())}else{snapshot.combinations.get(position).map(|combination|combination.id.as_str())};let lookup_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-8,..grant};if let Ok(step)=registry.requests.source_lookup_step(owner,version,candidate,lookup_grant){progress=step.progress;if progress!=Default::default(){registry.request_cursor=(current_slot+1)%SESSION_ACTIVE_CAPACITY;progress.copied_bytes+=8;}}return false;}
+        }else{registry.sources[current_slot]=None;}
+        registry.request_cursor = (current_slot + 1) % SESSION_ACTIVE_CAPACITY;
+        registry.snapshot_owners[app_slot] = Some(owner);
+        if registry.source_apps[current_slot].is_some_and(|app| app != render.app_instance_id) { return false; }
+        if registry.families[current_slot] != semio_s_fem_app_surface::ResultMode::Static {
+            let detail = if registry.families[current_slot] == semio_s_fem_app_surface::ResultMode::Modal { "fem2d.modal-job-owner-missing" } else { "fem2d.buckling-job-owner-missing" };
+            if registry.preflight_fault[current_slot].is_none() { registry.preflight_fault[current_slot] = Some(PendingSnapshotFault { app_instance_id: render.app_instance_id, render, detail, emitted: false }); }
+            return false;
+        }
+        if let Some(current) = registry.current[current_slot].filter(|current| render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision)) {
+            let Ok(owner) = registry.shells[current.shell as usize].try_borrow() else { return false };
+            if owner.as_ref().is_some_and(|state| state.source == registry.sources[current_slot]) { return false }
+        }
         let previous = match registry.current[current_slot] {
             Some(current) if current.app_instance_id != render.app_instance_id => return false,
-            Some(current) if current.identity.base_revision == render.base_revision && current.identity.generation == render.generation => return false,
             Some(current) => Some(current),
             None => None,
@@ -2441,5 +2679,6 @@

             let preflight = registry.preflight[current_slot].as_mut().expect("matching FEM preflight retained");
             let mut outcome = Ok(false);
-            for _ in 0..SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY {
+            for _ in 0..SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY.min(grant.maximum_items).min(grant.maximum_copy_bytes/64) {
+                progress.copied_items+=1;progress.copied_bytes+=64;
                 outcome = preflight.cursor.step_one(snapshot);
                 if outcome != Ok(false) {
@@ -2481,11 +2720,52 @@

         let identity = MountedIdentity { app_instance_id: render.app_instance_id, base_revision: render.base_revision, generation: render.generation, canonical_base_revision: render.canonical_base_revision, operation: OperationId(job), job };
         registry.pending[current_slot] = Some(PendingAdmission { app_instance_id: render.app_instance_id, shell, identity, admitted_items: process_items });
+        registry.shell_requests[shell as usize] = Some(owner);
         true
-    })
-}
-
-/// 🔁️ Reconciles one live store revision and returns only the exact host job effects needed.
-pub fn reconcile(doc: &ArtifactView<'_, Fem2dSnapshot>) -> Vec<Effect> {
-    let Some(render) = doc.render_operation() else { return Vec::new() };
+    });
+    semio_framework_plugin::MountedSnapshotReadPreparation{ready,progress}
+}
+
+/// 🪪️ Physical namespace, complete id, family and mode index own the scheduler identity.
+#[cfg(test)]
+pub fn result_request_key(source: Option<semio_s_fem_analyses_model::ResultSourceIdentity<'_>>, family: semio_s_fem_app_surface::ResultMode, index: usize) -> Option<RequestKey> {
+    let source = if family == semio_s_fem_app_surface::ResultMode::Modal { None } else { source };
+    let family = match family { semio_s_fem_app_surface::ResultMode::Static => "static", semio_s_fem_app_surface::ResultMode::Modal => "modal", semio_s_fem_app_surface::ResultMode::Buckling => "buckling" };
+    let index = if family == "static" { 0 } else { index };
+    if let Some(source)=source { semio_s_fem_analyses_model::validate_result_source_parts(source.kind,source.id).ok()?; }
+    RequestKey::from_segments(&[family.as_bytes(), &(index as u64).to_le_bytes(), source.map_or("none", |source|source.kind).as_bytes(), source.map_or("none", |source|source.id).as_bytes()])
+}
+
+#[cfg(test)]
+fn resolved_source(snapshot:&Fem2dSnapshot,source:Option<semio_s_fem_analyses_model::ResultSourceIdentity<'_>>)->Option<Option<ResolvedResultSource>>{match source{None=>Some(None),Some(source)if source.kind=="loadCase"=>snapshot.load_cases.iter().position(|case|case.id==source.id).map(|index|Some(ResolvedResultSource::LoadCase(index))),Some(source)if source.kind=="combination"=>snapshot.combinations.iter().position(|case|case.id==source.id).map(|index|Some(ResolvedResultSource::Combination(index))),Some(_)=>None}}
+
+fn prepare_result_demand(registry:&mut MountedRegistry,render:AppRenderOperationContext,consumer:ConsumerKey,window:&str,source:Option<semio_s_fem_analyses_model::ResultSourceIdentity<'_>>,family:semio_s_fem_app_surface::ResultMode,index:usize,version:semio_framework_value::RequestCandidateVersion,grant:RetainedCloneGrant)->RetainedCloneProgress{
+ let source=if family==semio_s_fem_app_surface::ResultMode::Modal{None}else{source};let index=if family==semio_s_fem_app_surface::ResultMode::Static{0}else{index};
+ let Some(app)=registry.app_slots.lookup(render.app_instance_id)else{let bytes=size_of::<u32>()+size_of::<semio_framework_value::FixedKeySlot>();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Default::default();}if registry.app_slots.claim(render.app_instance_id).is_none(){return Default::default();}return RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};};
+ let consumer_index=registry.requests.consumer_index(render.app_instance_id,consumer);
+ if consumer==ConsumerKey::Model&&consumer_index.is_none(){return registry.requests.claim_consumer_controlled(render.app_instance_id,consumer,grant).map_or(Default::default(),|step|step.progress);}
+ if consumer!=ConsumerKey::Model&&consumer_index.is_none_or(|index|registry.requests.consumer_name(index).is_none()){return registry.requests.prepare_consumer(render.app_instance_id,consumer,window,grant).map_or(Default::default(),|step|step.progress);}
+ let family_name=match family{semio_s_fem_app_surface::ResultMode::Static=>"static",semio_s_fem_app_surface::ResultMode::Modal=>"modal",semio_s_fem_app_surface::ResultMode::Buckling=>"buckling"};let index_bytes=(index as u64).to_le_bytes();let segments=[family_name.as_bytes(),index_bytes.as_slice(),source.map_or("none",|source|source.kind).as_bytes(),source.map_or("none",|source|source.id).as_bytes()];
+ let Some(key)=registry.requests.selector(render.app_instance_id,consumer,version)else{return registry.requests.prepare_selector(render.app_instance_id,consumer,version,&segments,grant).map_or(Default::default(),|step|step.progress);};
+ let candidate=registry.requests.binding(render.app_instance_id,consumer);if candidate.is_none_or(|owner|registry.requests.key(owner).is_none_or(|(_,current)|current!=key)){
+  let Some(step)=registry.requests.stage_controlled(render.app_instance_id,consumer,key,grant)else{return Default::default();};if step.admission.fresh{let slot=step.admission.owner.index();registry.source_apps[slot]=Some(render.app_instance_id);registry.source_kinds[slot]=source.map_or(2,|source|if source.kind=="loadCase"{0}else{1});registry.families[slot]=family;registry.mode_indices[slot]=index;registry.request_cancelled[slot]=false;}if consumer==ConsumerKey::Model{registry.model_requests[app.index()]=Some(key);}return step.progress;
+ }
+ let consumer_index=consumer_index.unwrap();if registry.verified_sources[consumer_index]!=Some((version,key)){if source.is_none(){registry.verified_sources[consumer_index]=Some((version,key));}else{match registry.requests.prepare_request(render.app_instance_id,key,source.map_or("none",|source|source.id),grant){Ok(step)=>{if step.ready{registry.verified_sources[consumer_index]=Some((version,key));}return step.progress;},Err(_)=>return Default::default()}}}
+ let owner=candidate.unwrap();if registry.requests.live_binding(render.app_instance_id,consumer)!=Some(owner){return registry.requests.admit_controlled(render.app_instance_id,consumer,key,grant).map_or(Default::default(),|step|step.progress);}Default::default()
+}
+
+/// 🧭️ Committed window partitions and source revisions admit one fair original preparation frontier.
+pub fn prepare_snapshot_read(render:AppRenderOperationContext,snapshot:&Fem2dSnapshot,windows:&semio_framework_plugin::WindowConfigOwnerRegistry,view:Option<&semio_framework_plugin::ViewModel>,grant:RetainedCloneGrant)->semio_framework_plugin::MountedSnapshotReadPreparation{
+ if render.app_instance_id==0||grant.maximum_items==0||grant.maximum_depth==0{return Default::default();}
+ let departure=retain_result_windows(render,windows,view,grant);if departure!=Default::default(){return semio_framework_plugin::MountedSnapshotReadPreparation{ready:false,progress:departure};}
+ let progress=MOUNTED.with(|registry|{let mut registry=registry.borrow_mut();let count=view.map_or(1,|view|view.window_instances.len()+1);for _ in 0..count{let index=registry.consumer_cursor%count;registry.consumer_cursor=(index+1)%count;let (consumer,window,source,family,mode,version)=if index==0{(ConsumerKey::Model,"model",snapshot.load_cases.first().map(|case|semio_s_fem_analyses_model::ResultSourceIdentity{kind:"loadCase",id:&case.id}),semio_s_fem_app_surface::ResultMode::Static,0,semio_framework_value::RequestCandidateVersion{source:source_version(render),consumer_generation:0,consumer_revision:[0;32]})}else{let Some(window)=view.and_then(|view|view.window_instances.get(index-1))else{continue;};let Some(authority)=windows.borrow(&window.window_kind_id,&window.id)else{continue;};let Some(config)=authority.get::<semio_s_artifact_fem_2d_results_state::config::Fem2dResultsWindowConfigOwner>()else{continue;};let source=match config.result_source_id.as_deref(){Some(id)=>match semio_s_fem_analyses_model::borrow_result_source_identity(id){Ok(source)=>Some(source),Err(_)=>continue},None=>snapshot.load_cases.first().map(|case|semio_s_fem_analyses_model::ResultSourceIdentity{kind:"loadCase",id:&case.id})};(ConsumerKey::Window(authority.authority),window.id.as_str(),source,config.result_mode,config.result_mode_index,semio_framework_value::RequestCandidateVersion{source:source_version(render),consumer_generation:authority.generation,consumer_revision:authority.revision})};let progress=prepare_result_demand(&mut registry,render,consumer,window,source,family,mode,version,grant);if progress!=Default::default(){return progress;}}Default::default()});
+ if progress!=Default::default(){return semio_framework_plugin::MountedSnapshotReadPreparation{ready:false,progress};}prepare_snapshot_admission(render,snapshot,grant)
+}
+
+/// 🪟️ Original committed partition identities prune departed consumers before request retirement.
+pub fn retain_result_windows(render:AppRenderOperationContext,windows:&semio_framework_plugin::WindowConfigOwnerRegistry,view:Option<&semio_framework_plugin::ViewModel>,grant:RetainedCloneGrant)->RetainedCloneProgress{MOUNTED.with(|registry|{let mut registry=registry.borrow_mut();for index in 0..SESSION_CONSUMER_CAPACITY{let Some((scope,consumer))=registry.requests.consumer_at(index)else{continue;};let ConsumerKey::Window(identity)=consumer else{continue;};if scope!=render.app_instance_id{continue;}let live=view.is_some_and(|view|view.window_instances.iter().any(|window|windows.borrow(&window.window_kind_id,&window.id).is_some_and(|authority|authority.authority==identity)));if !live{if let Some(step)=registry.requests.remove_consumer_controlled(scope,consumer,grant){registry.verified_sources[index]=None;return step.progress;}}}Default::default()})}
+
+/// 🔁️ Reconciles one live store revision and returns the exact retained job effects.
+pub fn reconcile(doc: &semio_framework_plugin::ArtifactPendingView<'_, Fem2dSnapshot>) -> Vec<Effect> {
+    let render = doc.render_operation();
     if render.app_instance_id == 0 {
         return Vec::new();
@@ -2493,5 +2773,10 @@

     MOUNTED.with(|registry| {
         let mut registry = registry.borrow_mut();
-        let current_slot = render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
+        let mut cancellations=Vec::new();
+        for index in 0..SESSION_ACTIVE_CAPACITY{let Some((owner,scope,_))=registry.requests.at(index)else{continue;};if scope!=render.app_instance_id||registry.requests.references(owner)!=0||registry.request_cancelled[index]{continue;}if let Some(current)=registry.current[index]{registry.requests.record_terminal(owner,identity_version(current.identity),current.identity.job,semio_framework_value::RequestTerminalKind::Cancelled);registry.request_cancelled[index]=true;cancellations.push(Effect::CancelJob{job:current.identity.job});}}
+        if !cancellations.is_empty(){return cancellations;}
+        let Some(app_slot) = registry.app_slots.lookup(render.app_instance_id).map(|slot| slot.index()) else { return Vec::new() };
+        let Some(owner) = registry.snapshot_owners[app_slot].filter(|owner| registry.requests.key(*owner).is_some_and(|(scope, _)| scope == render.app_instance_id) && registry.requests.ready(*owner)) else { return Vec::new() };
+        let current_slot = owner.index();
         if let Some(fault) = registry.preflight_fault[current_slot].as_mut().filter(|fault| {
             fault.app_instance_id == render.app_instance_id && fault.render.base_revision == render.base_revision && fault.render.generation == render.generation && fault.render.canonical_base_revision == render.canonical_base_revision
@@ -2507,6 +2792,7 @@

                 return Vec::new();
             }
-            if current.identity.base_revision == render.base_revision && current.identity.generation == render.generation {
-                return Vec::new();
+            if render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision) {
+                let Ok(owner) = registry.shells[current.shell as usize].try_borrow() else { return Vec::new() };
+                if owner.as_ref().is_some_and(|state| state.source == registry.sources[current_slot]) { return Vec::new() }
             }
             if !registry.retiring.iter().any(Option::is_none) {
@@ -2528,6 +2814,10 @@

         };
         registry.pending[current_slot] = None;
+        let previous_valid = previous.and_then(|previous| {
+            let owner = registry.shells[previous.shell as usize].try_borrow().ok()?;
+            if owner.as_ref().is_some_and(|state| state.visual.validated_final) { Some(previous.shell) } else { previous.previous_valid }
+        });
         if let Some(previous) = previous {
-            assert!(registry.retain_retirement(previous.shell), "retirement slot was preflighted under the registry owner");
+            assert!(registry.retain_retirement(previous.shell, previous.app_instance_id), "retirement slot was preflighted under the registry owner");
             if let Ok(shell) = registry.shells[previous.shell as usize].try_borrow() {
                 if let Some(state) = shell.as_ref() {
@@ -2535,10 +2825,14 @@

                 }
             }
+            registry.requests.record_terminal(owner,identity_version(previous.identity),previous.identity.job,semio_framework_value::RequestTerminalKind::Cancelled);
             registry.current[current_slot] = None;
         }
         let identity = pending.identity;
         let job = identity.job;
-        *registry.shells[shell as usize].borrow_mut() = Some(MountedState::new(identity, snapshot, pending.admitted_items));
-        registry.current[current_slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell, identity });
+        assert!(registry.requests.begin_execution(owner,identity_version(identity),job), "only the original verified request admits one current job");
+        let mut state = MountedState::new(identity, snapshot, pending.admitted_items);
+        state.source = registry.sources[current_slot];
+        *registry.shells[shell as usize].borrow_mut() = Some(state);
+        registry.current[current_slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell, identity, previous_valid });
         let mut effects = Vec::with_capacity(2);
         if let Some(previous) = previous {
@@ -2548,4 +2842,23 @@

         effects
     })
+}
+
+/// 📊️ Borrows physical fields from the addressed source and generation without running a solver in render.
+pub struct MountedFieldsView<'a> { pub progress: &'a Fem2dLiveVisual, pub fields: &'a [semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField], pub retained: bool }
+
+pub fn with_live_fields<R>(render: AppRenderOperationContext, authority: Option<semio_framework_plugin::WindowConfigAuthorityIdentity>, read: impl FnOnce(Option<MountedFieldsView<'_>>) -> R) -> R {
+    let shells = MOUNTED.with(|registry| {
+        let registry = registry.borrow();
+        let owner = registry.requests.live_binding(render.app_instance_id, ConsumerKey::Window(authority?))?;
+        let current = registry.current[owner.index()]?;
+        if !render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision) { return None; }
+        Some((registry.shells[current.shell as usize].clone(), current.previous_valid.map(|shell| registry.shells[shell as usize].clone())))
+    });
+    let Some((shell, previous)) = shells else { return read(None); };
+    let Ok(owner) = shell.try_borrow() else { return read(None); };
+    let Some(current) = owner.as_ref() else { return read(None); };
+    let previous = previous.as_ref().and_then(|owner| owner.try_borrow().ok());
+    let previous = previous.as_ref().and_then(|owner| owner.as_ref()).filter(|state| state.visual.validated_final && !current.visual.validated_final);
+    read(Some(MountedFieldsView { progress: &current.visual, fields: previous.map_or(current.visual.fields.as_slice(), |state| state.visual.fields.as_slice()), retained: previous.is_some() }))
 }
 
@@ -2555,6 +2868,8 @@

     let shell = MOUNTED.with(|registry| {
         let registry = registry.borrow();
-        let current = registry.current[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY]?;
-        if current.app_instance_id != render.app_instance_id || current.identity.base_revision != render.base_revision || current.identity.generation != render.generation {
+        let app = registry.app_slots.lookup(render.app_instance_id)?.index();
+        let key = registry.model_requests[app]?;
+        let current = registry.current[registry.requests.lookup(render.app_instance_id, key)?.index()]?;
+        if !render.matches_revision(current.app_instance_id, current.identity.base_revision, current.identity.generation, &current.identity.canonical_base_revision) {
             return None;
         }
@@ -2566,24 +2881,25 @@

 }
 
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
@@ -2591,34 +2907,63 @@

             registry.release_credit(shell);
             registry.release(shell);
-            return session_pending(1, 0);
-        }
-        step
+            return Ok(session_pending(1, 0));
+        }
+        Ok(step)
     })
 }
 
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
 
 /// 📏️ Quotes the next close or maintenance turn: one owner release bounded by the largest session owner.
-pub fn close_demands(app_instance_id: u32) -> RetirementDemand {
-    if terminal_is_empty(app_instance_id) {
-        RetirementDemand::default()
-    } else {
-        RetirementDemand { copy_bytes: SESSION_CLOSE_COPY_BYTES, release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() }
-    }
-}
-
-pub fn close_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+fn merge_close_demand(total: &mut RetirementDemand, child: RetirementDemand) {
+    total.copy_bytes = total.copy_bytes.max(child.copy_bytes);
+    total.capacity_bytes = total.capacity_bytes.max(child.capacity_bytes);
+    total.release_bytes = total.release_bytes.max(child.release_bytes);
+    total.depth = total.depth.max(child.depth);
+}
+
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
+}
+
+fn close_request_step(app_instance_id: u32, lease: semio_framework_value::FixedKeySlot, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
+    if grant.maximum_items == 0 { return Ok(session_pending(0, 0)); }
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
@@ -2626,9 +2971,9 @@

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
@@ -2638,9 +2983,42 @@

             }
             registry.current[slot] = None;
-            return session_pending(1, 0);
+            return Ok(session_pending(1, 0));
         }
         drop(registry);
-        retire_one(app_instance_id, grant)
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
+}
+
+/// 🪦️ App close removes consumers before independently retiring their request children.
+pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
+    semio_framework_plugin::app::original_lifecycle_step(cx, grant, close_demands(app_instance_id)?, |cx, grant| {
+        let step = close_turn(app_instance_id, cx, grant)?;
+        Ok((step, terminal_is_empty(app_instance_id)))
+    })
+}
+
+fn close_turn(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
+    if grant.maximum_items == 0 { return Ok(session_pending(0, 0)); }
+    if let Some(step)=MOUNTED.with(|registry|registry.borrow_mut().requests.remove_one_scoped_consumer_controlled(app_instance_id,grant)){return Ok(PluginLifecycleStep::Progress(step.progress));}
+    if let Some(progress)=MOUNTED.with(|registry|registry.borrow_mut().requests.retire_consumer_step(app_instance_id,grant)){return Ok(PluginLifecycleStep::Progress(progress));}
+    let owner = MOUNTED.with(|registry| { let registry = registry.borrow(); (0..SESSION_ACTIVE_CAPACITY).find_map(|index| { let (owner, scope, _) = registry.requests.at(index)?; (scope == app_instance_id).then_some(owner) }) });
+    if let Some(owner) = owner { return Ok(close_request_step(app_instance_id, owner, cx, grant)?); }
+    let step = retire_one(app_instance_id, cx, grant)?;
+    if !matches!(step, PluginLifecycleStep::Complete(_)) { return Ok(step); }
+    let copy_bytes = size_of::<RequestKey>() + size_of::<semio_framework_value::FixedKeySlot>() * 2 + size_of::<Option<u32>>() + size_of::<usize>() * 2;
+    if grant.maximum_copy_bytes < copy_bytes || grant.maximum_depth == 0 { return Ok(session_pending(0, 0)); }
+    MOUNTED.with(|registry| { let mut registry = registry.borrow_mut(); let Some(app) = registry.app_slots.lookup(app_instance_id) else { return Ok(PluginLifecycleStep::Complete(Default::default())); }; registry.model_requests[app.index()] = None; registry.snapshot_owners[app.index()] = None; assert!(registry.app_slots.release(app_instance_id, app)); Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })) })
 }
 
@@ -2648,10 +3026,8 @@

     MOUNTED.with(|registry| {
         let registry = registry.borrow();
-        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
-        registry.preflight_fault[slot].is_none_or(|fault| fault.app_instance_id != app_instance_id)
-            && registry.preflight[slot].is_none_or(|preflight| preflight.app_instance_id != app_instance_id)
-            && registry.pending[slot].is_none_or(|pending| pending.app_instance_id != app_instance_id)
-            && registry.current[slot].is_none_or(|current| current.app_instance_id != app_instance_id)
-            && registry.retiring.iter().all(|shell| shell.is_none_or(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_none_or(|state| state.identity.app_instance_id != app_instance_id))))
+        registry.app_slots.lookup(app_instance_id).is_none()
+            && !registry.requests.has_consumer(app_instance_id)
+            && (0..SESSION_ACTIVE_CAPACITY).all(|index|registry.requests.at(index).is_none_or(|(_,scope,_)|scope!=app_instance_id))
+            && registry.retiring.iter().all(|entry| entry.is_none_or(|entry| entry.app_instance_id != app_instance_id))
     })
 }
```
