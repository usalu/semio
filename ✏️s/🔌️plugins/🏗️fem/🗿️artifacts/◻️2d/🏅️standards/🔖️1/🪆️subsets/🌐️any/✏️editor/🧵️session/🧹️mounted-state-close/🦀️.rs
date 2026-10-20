//! 🧹️ Mounted state close: the close stepping and the terminal emptiness check of the mounted state. See [`super`].

use super::*;

//#region 🧰️FixedArena
impl MountedState {
    pub(super) fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
        let (maximum_items, maximum_bytes) = (grant.maximum_items.min(1), grant.maximum_release_bytes);
        let child = RetainedCloneGrant { maximum_items, ..grant };
        if maximum_items == 0 {
            return Ok(session_pending(0, 0));
        }
        self.cancel.cancel_now();
        self.stage = MountedStage::Closing;
        if let Some(matrix) = self.rejected_pcg_matrix.as_mut() {
            let (terminal, items, bytes) = matrix.close_step(maximum_bytes);
            if terminal { self.rejected_pcg_matrix = None; }
            return Ok(session_pending(items.max(usize::from(terminal)), bytes));
        }
        loop {
            match self.close_cursor {
                0 => if let Some(step) = self.close_phase_0(cx, maximum_bytes, child)? { return Ok(step); },
                1 => if let Some(step) = self.close_phase_1(cx, maximum_bytes, child)? { return Ok(step); },
                2 => if let Some(step) = self.close_phase_2(maximum_bytes)? { return Ok(step); },
                3 => if let Some(step) = self.close_phase_3(maximum_bytes)? { return Ok(step); },
                4 => if let Some(step) = self.close_phase_4(maximum_bytes)? { return Ok(step); },
                5 => if let Some(step) = self.close_phase_5(maximum_bytes)? { return Ok(step); },
                6 => if let Some(step) = self.close_phase_6(cx, maximum_bytes, child)? { return Ok(step); },
                7 => if let Some(step) = self.close_phase_7(cx, maximum_bytes, child)? { return Ok(step); },
                8 => {
                    if let Some(domain) = self.domain.as_mut() {
                        match self.domain_close_lane {
                            0 if domain.outer.pop().is_some() => return Ok(session_pending(1, 0)),
                            0 => {
                                let bytes = domain.outer.capacity() * size_of::<[f64; 2]>();
                                if bytes > maximum_bytes {
                                    return Ok(session_pending(0, 0));
                                }
                                domain.outer = Vec::new();
                                self.domain_close_lane = 1;
                                return Ok(session_pending(1, bytes));
                            }
                            1 => {
                                if let Some(hole) = domain.holes.last_mut() {
                                    if hole.pop().is_some() {
                                        return Ok(session_pending(1, 0));
                                    }
                                    let bytes = hole.capacity() * size_of::<[f64; 2]>();
                                    if bytes > maximum_bytes {
                                        return Ok(session_pending(0, 0));
                                    }
                                    *hole = Vec::new();
                                    domain.holes.pop();
                                    return Ok(session_pending(1, bytes));
                                }
                                self.domain_close_lane = 2;
                                continue;
                            }
                            2 => {
                                let bytes = domain.holes.capacity() * size_of::<Vec<[f64; 2]>>();
                                if bytes > maximum_bytes {
                                    return Ok(session_pending(0, 0));
                                }
                                domain.holes = Vec::new();
                                self.domain_close_lane = 3;
                                return Ok(session_pending(1, bytes));
                            }
                            _ => {
                                self.domain = None;
                                return Ok(session_pending(1, 0));
                            }
                        }
                    }
                    self.close_cursor += 1;
                }
                9 => if let Some(step) = self.close_phase_9(maximum_bytes)? { return Ok(step); },
                10 => if let Some(step) = self.close_phase_10(maximum_bytes)? { return Ok(step); },
                11 => if let Some(step) = self.close_phase_11(maximum_bytes)? { return Ok(step); },
                12 => if let Some(step) = self.close_phase_12(maximum_bytes)? { return Ok(step); },
                13 => if let Some(step) = self.close_phase_13(maximum_bytes)? { return Ok(step); },
                14 => if let Some(step) = self.close_phase_14(maximum_bytes)? { return Ok(step); },
                15 => if let Some(step) = self.close_phase_15()? { return Ok(step); },
                16 => if let Some(step) = self.close_phase_16(maximum_bytes)? { return Ok(step); },
                _ => {
                    self.stage = MountedStage::Empty;
                    return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
                }
            }
        }
    }

    /// 🧱️ Runs close phase 0 of `close_step`.
    fn close_phase_0(
        &mut self,
        cx: &mut StepContext<'_>,
        maximum_bytes: usize,
        child: RetainedCloneGrant,
    ) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if self.source.take().is_some() { return Ok(Some(session_pending(1, 0))); }
        if self.graph_plans.pop().is_some() {
            return Ok(Some(session_pending(1, 0)));
        }
        let plan_bytes = self.graph_plans.capacity() * size_of::<FemStagePlan>();
        if plan_bytes != 0 {
            if plan_bytes > maximum_bytes {
                return Ok(Some(session_pending(0, 0)));
            }
            self.graph_plans = Vec::new();
            return Ok(Some(session_pending(1, plan_bytes)));
        }
        if let Some(graph) = self.graph.as_mut() {
            if !InteractiveJob::terminal_is_empty(graph) {
                let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(graph, same))?;
                return Ok(Some(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress()))));
            }
            if maximum_bytes < size_of::<FemJobGraph>() { return Ok(Some(session_pending(0, 0))); }
            self.graph = None;
            return Ok(Some(session_pending(1, size_of::<FemJobGraph>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🪵️ Runs close phase 1 of `close_step`.
    fn close_phase_1(
        &mut self,
        cx: &mut StepContext<'_>,
        maximum_bytes: usize,
        child: RetainedCloneGrant,
    ) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(mesh) = self.mesh.as_mut() {
            if !InteractiveJob::terminal_is_empty(mesh) {
                let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(mesh, same))?;
                return Ok(Some(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress()))));
            }
            if maximum_bytes < size_of::<MeshJob>() { return Ok(Some(session_pending(0, 0))); }
            self.mesh = None;
            return Ok(Some(session_pending(1, size_of::<MeshJob>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🔩️ Runs close phase 2 of `close_step`.
    fn close_phase_2(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(model_build) = self.model_build.as_mut() {
            let (terminal, released_items, released_bytes) = model_build.close_step(maximum_bytes);
            if !terminal {
                return Ok(Some(session_pending(released_items, released_bytes)));
            }
            self.model_build = None;
            return Ok(Some(session_pending(1, size_of::<MountedModelBuild>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// ⚙️ Runs close phase 3 of `close_step`.
    fn close_phase_3(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(assembly_build) = self.assembly_build.as_mut() {
            let (terminal, released_items, released_bytes) = assembly_build.close_step(maximum_bytes);
            if !terminal {
                return Ok(Some(session_pending(released_items, released_bytes)));
            }
            self.assembly_build = None;
            return Ok(Some(session_pending(1, size_of::<AssemblyJobConstruction>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🧲️ Runs close phase 4 of `close_step`.
    fn close_phase_4(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(assembly) = self.assembly.as_mut() {
            let (terminal, released_items, released_bytes) = assembly.close_retained_step(maximum_bytes);
            if !terminal {
                return Ok(Some(session_pending(released_items, released_bytes)));
            }
            self.assembly = None;
            return Ok(Some(session_pending(1, size_of::<AssemblyJob<'static>>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🪛️ Runs close phase 5 of `close_step`.
    fn close_phase_5(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(csr_build) = self.csr_build.as_mut() {
            let (terminal, released_items, released_bytes) = csr_build.close_step(maximum_bytes);
            if !terminal {
                return Ok(Some(session_pending(released_items, released_bytes)));
            }
            self.csr_build = None;
            return Ok(Some(session_pending(1, size_of::<AssemblyCsrBuild>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🔧️ Runs close phase 6 of `close_step`.
    fn close_phase_6(
        &mut self,
        cx: &mut StepContext<'_>,
        maximum_bytes: usize,
        child: RetainedCloneGrant,
    ) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(pcg_build) = self.pcg_build.as_mut() {
            if !pcg_build.terminal_is_empty() {
                let step = cx.with_retained_loan(child, |same| pcg_build.close_step(same))?;
                return Ok(Some(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress()))));
            }
            if maximum_bytes < size_of::<PcgJobConstruction>() { return Ok(Some(session_pending(0, 0))); }
            self.pcg_build = None;
            return Ok(Some(session_pending(1, size_of::<PcgJobConstruction>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🔨️ Runs close phase 7 of `close_step`.
    fn close_phase_7(
        &mut self,
        cx: &mut StepContext<'_>,
        maximum_bytes: usize,
        child: RetainedCloneGrant,
    ) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(loads) = self.load_build.as_mut() {
            if !loads.close_step() { return Ok(Some(session_pending(1, 0))); }
            self.load_build = None;
            return Ok(Some(session_pending(1, size_of::<MountedLoadBuild>())));
        }
        if let Some(pcg) = self.pcg.as_mut() {
            if !InteractiveJob::terminal_is_empty(pcg) {
                let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(pcg, same))?;
                return Ok(Some(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress()))));
            }
            if maximum_bytes < size_of::<PcgJob>() { return Ok(Some(session_pending(0, 0))); }
            self.pcg = None;
            return Ok(Some(session_pending(1, size_of::<PcgJob>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🪚️ Runs close phase 9 of `close_step`.
    fn close_phase_9(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(field) = self.visual.fields.last_mut() {
            if field.node_id.capacity() != 0 {
                let bytes = field.node_id.capacity();
                if bytes > maximum_bytes {
                    return Ok(Some(session_pending(0, 0)));
                }
                field.node_id = String::new();
                return Ok(Some(session_pending(1, bytes)));
            }
            self.visual.fields.pop();
            return Ok(Some(session_pending(1, 0)));
        }
        let bytes = self.visual.fields.capacity() * size_of::<semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField>();
        if bytes != 0 {
            if bytes > maximum_bytes {
                return Ok(Some(session_pending(0, 0)));
            }
            self.visual.fields = Vec::new();
            return Ok(Some(session_pending(1, bytes)));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🧰️ Runs close phase 10 of `close_step`.
    fn close_phase_10(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(owner) = self.visual.assembling_element_ids.last_mut() {
            if owner.capacity() != 0 {
                let bytes = owner.capacity();
                if bytes > maximum_bytes {
                    return Ok(Some(session_pending(0, 0)));
                }
                *owner = String::new();
                return Ok(Some(session_pending(1, bytes)));
            }
            self.visual.assembling_element_ids.pop();
            return Ok(Some(session_pending(1, 0)));
        }
        let bytes = self.visual.assembling_element_ids.capacity() * size_of::<String>();
        if bytes != 0 {
            if bytes > maximum_bytes {
                return Ok(Some(session_pending(0, 0)));
            }
            self.visual.assembling_element_ids = Vec::new();
            return Ok(Some(session_pending(1, bytes)));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🧪️ Runs close phase 11 of `close_step`.
    fn close_phase_11(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if self.visual_region_owner.is_none() {
            if let Some(owner) = self.visual.region_quality.take_one() {
                self.visual_region_owner = Some(owner);
                return Ok(Some(session_pending(1, 0)));
            }
        }
        if let Some((id, _)) = self.visual_region_owner.as_mut() {
            if id.capacity() != 0 {
                let bytes = id.capacity();
                if bytes > maximum_bytes {
                    return Ok(Some(session_pending(0, 0)));
                }
                *id = String::new();
                return Ok(Some(session_pending(1, bytes)));
            }
            self.visual_region_owner = None;
            return Ok(Some(session_pending(1, 0)));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🧫️ Runs close phase 12 of `close_step`.
    fn close_phase_12(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(candidate) = self.visual_rejected.as_mut() {
            let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
            if !terminal {
                return Ok(Some(session_pending(released_items, released_bytes)));
            }
            if !candidate.terminal_is_empty() {
                return Ok(Some(PluginLifecycleStep::Blocked { reason: "mounted FEM rejected visual reported false terminal" }));
            }
            self.visual_rejected = None;
            return Ok(Some(session_pending(1, 0)));
        }
        if let Some(candidate) = self.visual_job_candidate.as_mut() {
            let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
            if !terminal {
                return Ok(Some(session_pending(released_items, released_bytes)));
            }
            if !candidate.terminal_is_empty() {
                return Ok(Some(PluginLifecycleStep::Blocked { reason: "mounted FEM live visual candidate reported false terminal" }));
            }
            self.visual_job_candidate = None;
            return Ok(Some(session_pending(1, 0)));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🧬️ Runs close phase 13 of `close_step`.
    fn close_phase_13(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(displaced) = self.visual_displaced.as_mut() {
            let (terminal, released_items, released_bytes) = displaced.close_step(maximum_bytes);
            if !terminal {
                return Ok(Some(session_pending(released_items, released_bytes)));
            }
            if !displaced.terminal_is_empty() {
                return Ok(Some(PluginLifecycleStep::Blocked { reason: "mounted FEM displaced visual reported false terminal" }));
            }
            self.visual_displaced = None;
            return Ok(Some(session_pending(1, 0)));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🔬️ Runs close phase 14 of `close_step`.
    fn close_phase_14(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(current) = self.visual_current.as_mut() {
            let (terminal, released_items, released_bytes) = current.close_step(maximum_bytes);
            if !terminal {
                return Ok(Some(session_pending(released_items, released_bytes)));
            }
            if !current.terminal_is_empty() {
                return Ok(Some(PluginLifecycleStep::Blocked { reason: "mounted FEM current visual reported false terminal" }));
            }
            self.visual_current = None;
            return Ok(Some(session_pending(1, 0)));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 🔭️ Runs close phase 15 of `close_step`.
    fn close_phase_15(&mut self) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(snapshot) = self.snapshot.take() {
            let Some(witness) = snapshot.return_to_registry_witness() else {
                return Ok(Some(PluginLifecycleStep::Blocked { reason: "mounted FEM snapshot lease was already returned" }));
            };
            self.snapshot_return = Some(witness);
            return Ok(Some(session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>())));
        }
        if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
            return Ok(Some(session_pending(0, 0)));
        }
        if self.snapshot_return.take().is_some() {
            return Ok(Some(session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotReadReturn>())));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    /// 📡️ Runs close phase 16 of `close_step`.
    fn close_phase_16(&mut self, maximum_bytes: usize) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(fault) = self.fault.as_mut() {
            if fault.pop().is_some() {
                return Ok(Some(session_pending(1, 0)));
            }
            let bytes = fault.capacity();
            if bytes > maximum_bytes {
                return Ok(Some(session_pending(0, 0)));
            }
            *fault = Vec::new();
            self.fault = None;
            return Ok(Some(session_pending(1, bytes)));
        }
        self.close_cursor += 1;
        Ok(None)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.stage == MountedStage::Empty
            && self.snapshot.is_none()
            && self.snapshot_return.is_none()
            && self.graph_plans.capacity() == 0
            && self.graph.is_none()
            && self.mesh.is_none()
            && self.model_build.is_none()
            && self.assembly_build.is_none()
            && self.assembly.is_none()
            && self.csr_build.is_none()
            && self.load_build.is_none()
            && self.source.is_none()
            && self.rejected_pcg_matrix.is_none()
            && self.pcg_build.is_none()
            && self.pcg.is_none()
            && self.domain.is_none()
            && self.visual.region_quality.is_empty()
            && self.visual_region_owner.is_none()
            && self.visual.assembling_element_ids.is_empty()
            && self.visual.fields.is_empty()
            && self.visual_job_candidate.is_none()
            && self.visual_rejected.is_none()
            && self.visual_current.is_none()
            && self.visual_displaced.is_none()
            && self.fault.is_none()
    }
}
//#endregion 🧰️FixedArena
