//! 🔁️ Mounted state step: the stepping of the mounted state through the model build, mesh, assembly and solve stages. See [`super`].

use super::*;

//#region 🧰️FixedArena
impl MountedState {
    pub(super) fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep {
        if self.cancel.is_cancelled_now() {
            self.stage = MountedStage::Closing;
            return self.fail(b"fem2d.session-cancelled".to_vec());
        }
        if budget.fuel == 0 || budget.deadline_ms == 0 {
            return JobStep::Running(None);
        }
        (|| {
            if cx.should_yield() {
                return JobStep::Running(None);
            }
            if let Some(step) = self.drive_visual_one(cx) {
                return step;
            }
            match self.stage {
                MountedStage::PrepareGraph => {
                    const PLANS: [FemStagePlan; 8] = [
                        FemStagePlan { stage: FemJobStage::ValidateReferences, units: 1 },
                        FemStagePlan { stage: FemJobStage::BuildDofMap, units: 1 },
                        FemStagePlan { stage: FemJobStage::OrderEquations, units: 1 },
                        FemStagePlan { stage: FemJobStage::Assemble, units: 1 },
                        FemStagePlan { stage: FemJobStage::Factor, units: 1 },
                        FemStagePlan { stage: FemJobStage::Solve, units: 1 },
                        FemStagePlan { stage: FemJobStage::Recover, units: 1 },
                        FemStagePlan { stage: FemJobStage::Finalize, units: 1 },
                    ];
                    if self.graph_plans.capacity() == 0 {
                        if self.graph_plans.try_reserve_exact(PLANS.len()).is_err() || self.graph_plans.capacity() * size_of::<FemStagePlan>() > SESSION_OWNER_PAGE_BYTES {
                            return self.fail(b"fem2d.graph-plan-allocation".to_vec());
                        }
                    } else if let Some(plan) = PLANS.get(self.graph_plans.len()).cloned() {
                        self.graph_plans.push(plan);
                    } else {
                        self.graph = Some(FemJobGraph::new(self.identity.operation(), std::mem::take(&mut self.graph_plans), 1));
                        self.stage = MountedStage::Graph;
                    }
                    cx.consume_fuel(1);
                    self.progress(b"fem2d.graph-admitted")
                }
                MountedStage::Graph => match child_end(self.graph.as_mut().expect("graph stage owns graph").step(cx)) {
                    ChildEnd::Complete(output) => {
                        self.stage = MountedStage::PrepareDomain;
                        match output {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.fail(b"fem2d.graph-output-capacity".to_vec()),
                        }
                    }
                    ChildEnd::Preview { bytes: payload, .. } | ChildEnd::Checkpoint(payload) => match payload {
                        Some(bytes) => JobStep::Running(Some(bytes)),
                        None => self.fail(b"fem2d.graph-output-capacity".to_vec()),
                    },
                    ChildEnd::Yield => self.progress(b"fem2d.graph-yield"),
                    ChildEnd::Cancelled => self.fail(b"fem2d.graph-cancelled".to_vec()),
                    ChildEnd::Fault(detail) => self.fail(detail.unwrap_or_else(|| b"fem2d.graph-fault-capacity".to_vec())),
                },
                MountedStage::PrepareDomain => match self.prepare_domain_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.domain")
                    }
                    Ok(true) => {
                        let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
                        if let (Some(region), Some(domain)) = (snapshot.regions.first(), self.domain.take()) {
                            if self.visual.region_quality.insert(region.id.clone(), RegionVisualQuality::Unmeshed).is_err() {
                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
                            }
                            self.visual.state = FemVisualState::Unmeshed;
                            self.visual_dirty = true;
                            self.mesh = Some(MeshJob::new_bounded(domain, MeshOpts { max_edge: region.mesh_size, min_angle_deg: 20.0 }, self.identity.operation(), SESSION_MAXIMUM_MESH_POINTS, SESSION_MAXIMUM_MESH_TRIANGLES));
                            self.stage = MountedStage::Mesh;
                        } else {
                            self.stage = MountedStage::BuildModel;
                        }
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.domain-complete")
                    }
                    Err(detail) => self.fail(detail.to_vec()),
                },
                MountedStage::Mesh => match child_end(self.mesh.as_mut().expect("mesh stage owns mesh").step(cx)) {
                    ChildEnd::Complete(output) => {
                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
                            if !self.visual.region_quality.update(&region.id, RegionVisualQuality::Final) {
                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
                            }
                            self.visual.state = FemVisualState::Refined;
                            self.visual_dirty = true;
                        }
                        self.model_build = Some(MountedModelBuild::new(self.mesh.as_mut().and_then(MeshJob::take_completed_mesh)));
                        self.stage = MountedStage::BuildModel;
                        match output {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.mesh-complete"),
                        }
                    }
                    ChildEnd::Preview { bytes: payload, quality } => {
                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
                            let quality = match quality {
                                Some(0) => RegionVisualQuality::Coarse,
                                Some(1) => RegionVisualQuality::Refined,
                                Some(2) => RegionVisualQuality::Final,
                                _ => RegionVisualQuality::Unmeshed,
                            };
                            if !self.visual.region_quality.update(&region.id, quality) {
                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
                            }
                            self.visual.state = match quality {
                                RegionVisualQuality::Unmeshed => FemVisualState::Unmeshed,
                                RegionVisualQuality::Coarse => FemVisualState::Coarse,
                                RegionVisualQuality::Refined | RegionVisualQuality::Final => FemVisualState::Refined,
                            };
                            self.visual_dirty = true;
                        }
                        match payload {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.mesh-preview"),
                        }
                    }
                    ChildEnd::Checkpoint(payload) => match payload {
                        Some(bytes) => JobStep::Running(Some(bytes)),
                        None => self.progress(b"fem2d.mesh-yield"),
                    },
                    ChildEnd::Yield => self.progress(b"fem2d.mesh-yield"),
                    ChildEnd::Cancelled => self.fail(b"fem2d.mesh-cancelled".to_vec()),
                    ChildEnd::Fault(detail) => self.fail(detail.unwrap_or_else(|| b"fem2d.mesh-fault-capacity".to_vec())),
                },
                MountedStage::BuildModel => {
                    let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
                    if self.model_build.is_none() {
                        self.model_build = Some(MountedModelBuild::new(None));
                        cx.consume_fuel(1);
                        return self.progress(b"fem2d.model-admitted");
                    }
                    match self.model_build.as_mut().expect("model builder admitted above").step_one(snapshot) {
                        Ok(false) => {
                            cx.consume_fuel(1);
                            self.progress(b"fem2d.model-building")
                        }
                        Ok(true) => {
                            let model = std::sync::Arc::new(self.model_build.as_mut().and_then(MountedModelBuild::take_complete).expect("complete model transfers exactly once"));
                            self.assembly_build = Some(AssemblyJobConstruction::new_owned(model, self.identity.operation(), 1));
                            self.stage = MountedStage::PrepareAssembly;
                            self.visual.state = FemVisualState::Assembling;
                            cx.consume_fuel(1);
                            self.progress(b"fem2d.model-built")
                        }
                        Err(error) => self.fail(error),
                    }
                }
                MountedStage::PrepareAssembly => match self.assembly_build.as_mut().expect("assembly construction retained").step_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.assembly-preparing")
                    }
                    Ok(true) => {
                        self.assembly = self.assembly_build.as_mut().and_then(AssemblyJobConstruction::take_complete);
                        if self.assembly.is_none() {
                            return self.fail(b"fem2d.assembly-construction-false-terminal".to_vec());
                        }
                        self.stage = MountedStage::Assembly;
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.assembly-admitted")
                    }
                    Err(error) => self.fail(error.to_string().into_bytes()),
                },
                MountedStage::Assembly => match child_end(self.assembly.as_mut().expect("assembly stage owns assembly").step(cx)) {
                    ChildEnd::Complete(output) => {
                        let assembly = self.assembly.take().expect("assembly owner retained");
                        let csr_build = match AssemblyCsrBuild::new(assembly) {
                            Ok(builder) => builder,
                            Err(assembly) => {
                                self.assembly = Some(assembly);
                                return self.fail(b"fem2d.assembly-false-terminal".to_vec());
                            }
                        };
                        self.csr_build = Some(csr_build);
                        self.stage = MountedStage::BuildCsr;
                        match output {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.assembly-complete"),
                        }
                    }
                    ChildEnd::Preview { bytes: payload, .. } | ChildEnd::Checkpoint(payload) => match payload {
                        Some(bytes) => JobStep::Running(Some(bytes)),
                        None => self.progress(b"fem2d.assembly-yield"),
                    },
                    ChildEnd::Yield => self.progress(b"fem2d.assembly-yield"),
                    ChildEnd::Cancelled => self.fail(b"fem2d.assembly-cancelled".to_vec()),
                    ChildEnd::Fault(detail) => self.fail(detail.unwrap_or_else(|| b"fem2d.assembly-fault-capacity".to_vec())),
                },
                MountedStage::BuildCsr => match self.csr_build.as_mut().expect("CSR builder retained").step_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.csr-building")
                    }
                    Ok(true) => {
                        let matrix = self.csr_build.as_mut().and_then(AssemblyCsrBuild::take_complete).expect("complete CSR transfers exactly once");
                        self.pcg_build = Some(PcgJobConstruction::new(self.identity.operation(), matrix));
                        self.stage = MountedStage::PreparePcg;
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.csr-complete")
                    }
                    Err(detail) => self.fail(detail.to_vec()),
                },
                MountedStage::PreparePcg => match self.pcg_build.as_mut().expect("PCG builder retained").step_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.pcg-preparing")
                    }
                    Ok(true) => {
                        self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                        if self.pcg.is_none() {
                            return self.fail(b"fem2d.pcg-false-terminal".to_vec());
                        }
                        self.stage = MountedStage::Pcg;
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.pcg-admitted")
                    }
                    Err(detail) => self.fail(detail.to_vec()),
                },
                MountedStage::Pcg => match child_end(self.pcg.as_mut().expect("pcg stage owns pcg").step(cx)) {
                    ChildEnd::Complete(output) => {
                        if let Some(job) = self.pcg.as_ref() {
                            let (completed, total, residual, tolerance, converged) = job.visual_progress();
                            self.visual.progress_completed = completed;
                            self.visual.progress_total = total;
                            self.visual.residual_norm = residual;
                            self.visual.tolerance = tolerance;
                            self.visual.converged = converged;
                            self.visual.state = if converged { FemVisualState::SolvingConverged } else { FemVisualState::SolvingUnconverged };
                        }
                        self.visual_field_cursor = 0;
                        self.visual_pcg_complete = true;
                        self.stage = MountedStage::SyncPcgVisual;
                        match output {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.pcg-complete"),
                        }
                    }
                    ChildEnd::Preview { bytes: payload, .. } | ChildEnd::Checkpoint(payload) => {
                        if let Some(job) = self.pcg.as_ref() {
                            let (completed, total, residual, tolerance, converged) = job.visual_progress();
                            self.visual.progress_completed = completed;
                            self.visual.progress_total = total;
                            self.visual.residual_norm = residual;
                            self.visual.tolerance = tolerance;
                            self.visual.converged = converged;
                            self.visual.state = if converged { FemVisualState::SolvingConverged } else { FemVisualState::SolvingUnconverged };
                        }
                        self.visual_field_cursor = 0;
                        self.visual_pcg_complete = false;
                        self.stage = MountedStage::SyncPcgVisual;
                        match payload {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.pcg-output-capacity"),
                        }
                    }
                    ChildEnd::Yield => self.progress(b"fem2d.pcg-yield"),
                    ChildEnd::Cancelled => self.fail(b"fem2d.pcg-cancelled".to_vec()),
                    ChildEnd::Fault(detail) => self.fail(detail.unwrap_or_else(|| b"fem2d.pcg-fault-capacity".to_vec())),
                },
                MountedStage::SyncPcgVisual => {
                    let Some(snapshot) = self.snapshot.as_ref() else { return self.fail(b"fem2d.visual-snapshot-owner".to_vec()) };
                    let Some(node) = snapshot.nodes.get(self.visual_field_cursor) else {
                        self.visual_dirty = true;
                        self.stage = if self.visual_pcg_complete { MountedStage::CommitReady } else { MountedStage::Pcg };
                        cx.consume_fuel(1);
                        return self.progress(b"fem2d.visual-field-page-complete");
                    };
                    let base = self.visual_field_cursor * 3;
                    let Some(tx) = self.pcg.as_ref().and_then(|job| job.visual_scalar(base)) else { return self.fail(b"fem2d.visual-tx-scalar".to_vec()) };
                    let Some(ty) = self.pcg.as_ref().and_then(|job| job.visual_scalar(base + 1)) else { return self.fail(b"fem2d.visual-ty-scalar".to_vec()) };
                    let field = semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField {
                        node_id: node.id.clone(),
                        displacement: [tx.displacement, ty.displacement],
                        residual: [tx.residual, ty.residual],
                        reaction: [tx.reaction, ty.reaction],
                        contour: tx.contour.max(ty.contour),
                        mode_shape: [tx.mode_estimate, ty.mode_estimate],
                    };
                    if let Some(target) = self.visual.fields.get_mut(self.visual_field_cursor) {
                        *target = field;
                    } else {
                        self.visual.fields.push(field);
                    }
                    self.visual_field_cursor += 1;
                    cx.consume_fuel(1);
                    self.progress(b"fem2d.visual-field-entry")
                }
                MountedStage::CommitReady => {
                    let validation = current_identity(self.identity.app_instance_id);
                    let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                    if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
                        return self.fail(b"fem2d.session-stale-commit".to_vec());
                    }
                    self.visual.validated_final = self.visual.converged;
                    self.visual.state = if self.visual.validated_final { FemVisualState::ValidatedFinal } else { FemVisualState::SolvingUnconverged };
                    self.visual_dirty = true;
                    self.stage = MountedStage::PublishFinalVisual;
                    cx.consume_fuel(1);
                    self.progress(b"fem2d.final-visual-requested")
                }
                MountedStage::PublishFinalVisual => {
                    let validation = current_identity(self.identity.app_instance_id);
                    let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                    if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
                        return self.fail(b"fem2d.session-stale-final-visual".to_vec());
                    }
                    self.stage = MountedStage::Complete;
                    let mut output = Vec::with_capacity(32);
                    output.extend_from_slice(&self.identity.operation.0.to_le_bytes());
                    output.extend_from_slice(&self.identity.base_revision.0.to_le_bytes());
                    output.extend_from_slice(&self.identity.generation.0.to_le_bytes());
                    output.extend_from_slice(&(self.admitted_items as u64).to_le_bytes());
                    cx.consume_fuel(1);
                    JobStep::Done(output)
                }
                MountedStage::Complete => JobStep::Done(Vec::new()),
                MountedStage::Fault => JobStep::Failed(self.fault.clone().unwrap_or_else(|| b"fem2d.session-fault".to_vec())),
                MountedStage::Closing | MountedStage::Empty => JobStep::Failed(b"fem2d.session-closed".to_vec()),
            }
        })()
    }

}
//#endregion 🧰️FixedArena
