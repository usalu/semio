//! 🔁️ Mounted state step: the stepping of the mounted state through the model build, mesh, assembly and solve stages. See [`super`].

use super::*;

//#region 🧰️FixedArena
impl MountedState {
    pub(super) fn step(&mut self, cx: &mut StepContext<'_>) -> Result<JobStep, semio_framework_job::OriginalTurnFailure> {
        if self.cancel.is_cancelled_now() {
            self.stage = MountedStage::Closing;
            return Ok(self.fail(b"fem2d.session-cancelled".to_vec()));
        }
        if cx.should_yield() {
            return Ok(JobStep::Running(None));
        }
        (|| {
            if cx.should_yield() {
                return Ok(JobStep::Running(None));
            }
            if let Some(step) = self.drive_visual_one(cx) {
                return Ok(step);
            }
            Ok(match self.stage {
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
                            return Ok(self.fail(b"fem2d.graph-plan-allocation".to_vec()));
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
                MountedStage::Graph => match child_end(self.graph.as_mut().expect("graph stage owns graph").step(cx))? {
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
                        if let (Some(region), Some(domain)) = (snapshot.regions.get(self.region_cursor), self.domain.take()) {
                            if self.visual.region_quality.insert(region.id.clone(), RegionVisualQuality::Unmeshed).is_err() {
                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
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
                MountedStage::Mesh => match child_end(self.mesh.as_mut().expect("mesh stage owns mesh").step(cx))? {
                    ChildEnd::Complete(output) => {
                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.get(self.region_cursor)) {
                            if !self.visual.region_quality.update(&region.id, RegionVisualQuality::Final) {
                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
                            }
                            self.visual.state = FemVisualState::Refined;
                            self.visual_dirty = true;
                        }
                        let completed_mesh = self.mesh.as_mut().and_then(MeshJob::take_completed_mesh);
                        if let Some(builder) = self.model_build.as_mut() {
                            let Some(mesh) = completed_mesh else { return Ok(self.fail(b"fem2d.mesh-terminal-owner".to_vec())); };
                            builder.attach_region(self.region_cursor, mesh);
                        } else {
                            self.model_build = Some(MountedModelBuild::new(completed_mesh));
                        }
                        self.stage = MountedStage::BuildModel;
                        match output {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.mesh-complete"),
                        }
                    }
                    ChildEnd::Preview { bytes: payload, quality } => {
                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.get(self.region_cursor)) {
                            let quality = match quality {
                                Some(0) => RegionVisualQuality::Coarse,
                                Some(1) => RegionVisualQuality::Refined,
                                Some(2) => RegionVisualQuality::Final,
                                _ => RegionVisualQuality::Unmeshed,
                            };
                            if !self.visual.region_quality.update(&region.id, quality) {
                                return Ok(self.fail(b"fem2d.visual-region-capacity".to_vec()));
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
                        return Ok(self.progress(b"fem2d.model-admitted"));
                    }
                    match self.model_build.as_mut().expect("model builder admitted above").step_one(snapshot) {
                        Ok(false) => {
                            cx.consume_fuel(1);
                            self.progress(b"fem2d.model-building")
                        }
                        Ok(true) => {
                            if self.region_cursor + 1 < snapshot.regions.len() {
                                self.model_build.as_mut().expect("completed region retains builder").close_lane = 7;
                                self.stage = MountedStage::RetireRegion;
                                cx.consume_fuel(1);
                                return Ok(self.progress(b"fem2d.region-complete"));
                            }
                            let model = self.model_build.as_mut().and_then(MountedModelBuild::take_complete).expect("complete model transfers exactly once");
                            self.assembly_build = Some(AssemblyJobConstruction::new_mounted(model, self.identity.operation(), 1));
                            self.stage = MountedStage::PrepareAssembly;
                            self.visual.state = FemVisualState::Assembling;
                            cx.consume_fuel(1);
                            self.progress(b"fem2d.model-built")
                        }
                        Err(error) => self.fail(error),
                    }
                }
                MountedStage::RetireRegion => {
                    cx.consume_fuel(1);
                    let builder = self.model_build.as_mut().expect("region retirement retains model");
                    if !builder.close_step(SESSION_OWNER_PAGE_BYTES).0 { return Ok(self.progress(b"fem2d.region-mesh-close")); }
                    if let Some(mesh) = self.mesh.as_mut() {
                        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_OWNER_PAGE_BYTES, maximum_capacity_bytes: SESSION_OWNER_PAGE_BYTES, maximum_release_bytes: SESSION_OWNER_PAGE_BYTES, maximum_depth: SESSION_CLOSE_DEPTH };
                        match cx.with_retained_loan(grant, |same| mesh.close_step(same))? {
                            InteractiveJobCloseStep::Complete { .. } => {}
                            InteractiveJobCloseStep::Pending { .. } => return Ok(self.progress(b"fem2d.region-job-close")),
                            _ => return Ok(self.fail(b"fem2d.region-job-close-refused".to_vec())),
                        }
                    }
                    self.mesh = None;
                    self.region_cursor += 1;
                    self.domain_outer_cursor = 0;
                    self.domain_hole_cursor = 0;
                    self.domain_hole_point_cursor = 0;
                    self.stage = MountedStage::PrepareDomain;
                    self.progress(b"fem2d.region-next")
                }
                MountedStage::PrepareAssembly => match self.assembly_build.as_mut().expect("assembly construction retained").step_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.assembly-preparing")
                    }
                    Ok(true) => {
                        self.assembly = self.assembly_build.as_mut().and_then(AssemblyJobConstruction::take_complete);
                        if self.assembly.is_none() {
                            return Ok(self.fail(b"fem2d.assembly-construction-false-terminal".to_vec()));
                        }
                        self.stage = MountedStage::Assembly;
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.assembly-admitted")
                    }
                    Err(error) => self.fail(error.to_string().into_bytes()),
                },
                MountedStage::Assembly => match child_end(self.assembly.as_mut().expect("assembly stage owns assembly").step(cx))? {
                    ChildEnd::Complete(output) => {
                        self.load_build = Some(MountedLoadBuild::new(self.source));
                        self.stage = MountedStage::BuildLoads;
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
                MountedStage::BuildLoads => {
                    cx.consume_fuel(1);
                    match self.load_build.as_mut().expect("load owner retained").step_one(self.snapshot.as_ref().expect("snapshot retained"), self.assembly.as_ref().expect("assembly retained")) {
                        Ok(false) => self.progress(b"fem2d.load-building"),
                        Ok(true) => {
                            let assembly = self.assembly.take().expect("assembly owner retained");
                            self.csr_build = match AssemblyCsrBuild::new_free(assembly) {
                                Ok(builder) => Some(builder),
                                Err(assembly) => { self.assembly = Some(assembly); return Ok(self.fail(b"fem2d.assembly-false-terminal".to_vec())); }
                            };
                            self.stage = MountedStage::BuildCsr;
                            self.progress(b"fem2d.loads-complete")
                        }
                        Err(detail) => self.fail(detail),
                    }
                }
                MountedStage::BuildCsr => match self.csr_build.as_mut().expect("CSR builder retained").step_one_funded(cx.retained_grant()).and_then(|(complete, progress)| {
                    cx.consume_retained(progress).map_err(|_| b"fem2d.csr-grant" as &'static [u8])?;
                    Ok(complete)
                }) {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.csr-building")
                    }
                    Ok(true) => {
                        let Some((matrix, progress)) = self.csr_build.as_mut().and_then(|build| build.take_complete(cx.retained_grant())) else { return Ok(self.progress(b"fem2d.csr-transfer-pending")); };
                        if cx.consume_retained(progress).is_err() { return Ok(self.fail(b"fem2d.csr-transfer-grant".to_vec())); }
                        let rhs = std::mem::take(&mut self.load_build.as_mut().expect("assembled loads retained").free);
                        self.pcg_build = match PcgJobConstruction::new_with_mounted_rhs(self.identity.operation(), matrix, rhs) {
                            Ok(builder) => Some(builder),
                            Err((matrix, rhs)) => { self.rejected_pcg_matrix = Some(matrix); self.load_build.as_mut().expect("load owner retained").free = rhs; return Ok(self.fail(b"fem2d.pcg-rhs-shape".to_vec())); }
                        };
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
                            return Ok(self.fail(b"fem2d.pcg-false-terminal".to_vec()));
                        }
                        self.stage = MountedStage::Pcg;
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.pcg-admitted")
                    }
                    Err(detail) => self.fail(detail.to_vec()),
                },
                MountedStage::Pcg => match child_end(self.pcg.as_mut().expect("pcg stage owns pcg").step(cx))? {
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
                        self.stage = MountedStage::RecoverReactions;
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
                MountedStage::RecoverReactions => {
                    cx.consume_fuel(1);
                    let loads = self.load_build.as_mut().expect("physical RHS retained for equilibrium");
                    if !loads.reactions.admit_one(loads.full.len()).unwrap_or(false) { return Ok(self.progress(b"fem2d.reaction-admitting")); }
                    if loads.reactions.len() < loads.full.len() {
                        let value = -loads.full.get(loads.reactions.len()).expect("full RHS scalar retained");
                        if loads.reactions.push(value).is_err() { return Ok(self.fail(b"fem2d.reaction-capacity".to_vec())); }
                        return Ok(self.progress(b"fem2d.reaction-initializing"));
                    }
                    let csr = self.csr_build.as_ref().expect("full stiffness retained");
                    if let Some((row, column, coefficient)) = csr.visual_full_entry(self.reaction_entry) {
                        let displacement = csr.visual_compact_index(column).and_then(|index| self.pcg.as_ref()?.visual_scalar(index)).map_or(0.0, |scalar| scalar.displacement);
                        if loads.reactions.add_at(row, coefficient * displacement).is_err() { return Ok(self.fail(b"fem2d.reaction-index".to_vec())); }
                        self.reaction_entry += 1;
                    } else { self.stage = MountedStage::SyncPcgVisual; }
                    self.progress(b"fem2d.reaction-recovering")
                }
                MountedStage::SyncPcgVisual => {
                    let Some(node) = self.csr_build.as_ref().and_then(|assembly| assembly.model_node(self.visual_field_cursor)) else {
                        self.visual_dirty = true;
                        self.stage = if self.visual_pcg_complete { MountedStage::CommitReady } else { MountedStage::Pcg };
                        cx.consume_fuel(1);
                        return Ok(self.progress(b"fem2d.visual-field-page-complete"));
                    };
                    let scalar = |dof| self.csr_build.as_ref().and_then(|assembly| assembly.visual_equation_indices(&node.id, dof)).and_then(|(_, free)| free).and_then(|index| self.pcg.as_ref()?.visual_scalar(index));
                    let tx = scalar(semio_s_fem_model::Dof::Tx);
                    let ty = scalar(semio_s_fem_model::Dof::Ty);
                    let reaction = |dof| self.csr_build.as_ref().and_then(|assembly| assembly.visual_equation_indices(&node.id, dof)).and_then(|(full, _)| self.load_build.as_ref()?.reactions.get(full)).unwrap_or(0.0);
                    let field = semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField {
                        node_id: node.id.clone(),
                        displacement: [tx.map_or(0.0, |scalar| scalar.displacement), ty.map_or(0.0, |scalar| scalar.displacement)],
                        residual: [tx.map_or(0.0, |scalar| scalar.residual), ty.map_or(0.0, |scalar| scalar.residual)],
                        reaction: [reaction(semio_s_fem_model::Dof::Tx), reaction(semio_s_fem_model::Dof::Ty)],
                        contour: tx.map_or(0.0, |scalar| scalar.contour).max(ty.map_or(0.0, |scalar| scalar.contour)),
                        mode_shape: [0.0; 2],
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
                    let validation = current_identity(self.identity);
                    let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                    if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
                        return Ok(self.fail(b"fem2d.session-stale-commit".to_vec()));
                    }
                    self.visual.validated_final = self.visual.converged;
                    if self.visual.validated_final {
                        MOUNTED.with(|registry| {
                            let mut registry = registry.borrow_mut();
                            let Some(slot) = registry.current.iter().position(|current| current.is_some_and(|current| current.identity == self.identity)) else { return };
                            if let Some(current) = registry.current[slot].as_mut().filter(|current| current.identity == self.identity) { current.previous_valid = None; }
                        });
                    }
                    self.visual.state = if self.visual.validated_final { FemVisualState::ValidatedFinal } else { FemVisualState::SolvingUnconverged };
                    self.visual_dirty = true;
                    self.stage = MountedStage::PublishFinalVisual;
                    cx.consume_fuel(1);
                    self.progress(b"fem2d.final-visual-requested")
                }
                MountedStage::PublishFinalVisual => {
                    let validation = current_identity(self.identity);
                    let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                    if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
                        return Ok(self.fail(b"fem2d.session-stale-final-visual".to_vec()));
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
            })
        })()
    }

}
//#endregion 🧰️FixedArena
