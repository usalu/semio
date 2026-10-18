//! 🧹️ Mounted state close: the close stepping and the terminal emptiness check of the mounted state. See [`super`].

use super::*;

//#region 🧰️FixedArena
impl MountedState {
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> PluginLifecycleStep {
        let (maximum_items, maximum_bytes) = (grant.maximum_items.min(1), grant.maximum_release_bytes);
        let child = RetainedCloneGrant { maximum_items, ..grant };
        if maximum_items == 0 {
            return session_pending(0, 0);
        }
        self.cancel.cancel_now();
        self.stage = MountedStage::Closing;
        loop {
            match self.close_cursor {
                0 => {
                    if self.graph_plans.pop().is_some() {
                        return session_pending(1, 0);
                    }
                    let plan_bytes = self.graph_plans.capacity() * size_of::<FemStagePlan>();
                    if plan_bytes != 0 {
                        if plan_bytes > maximum_bytes {
                            return session_pending(0, 0);
                        }
                        self.graph_plans = Vec::new();
                        return session_pending(1, plan_bytes);
                    }
                    if let Some(graph) = self.graph.as_mut() {
                        let step = InteractiveJob::close_step(graph, child);
                        if let Some(blocked) = session_child_failure(step) {
                            return blocked;
                        }
                        if !InteractiveJob::terminal_is_empty(graph) {
                            return PluginLifecycleStep::Progress(step.progress());
                        }
                        self.graph = None;
                        return session_pending(1, size_of::<FemJobGraph>());
                    }
                    self.close_cursor += 1;
                }
                1 => {
                    if let Some(mesh) = self.mesh.as_mut() {
                        let step = InteractiveJob::close_step(mesh, child);
                        if let Some(blocked) = session_child_failure(step) {
                            return blocked;
                        }
                        if !InteractiveJob::terminal_is_empty(mesh) {
                            return PluginLifecycleStep::Progress(step.progress());
                        }
                        self.mesh = None;
                        return session_pending(1, size_of::<MeshJob>());
                    }
                    self.close_cursor += 1;
                }
                2 => {
                    if let Some(model_build) = self.model_build.as_mut() {
                        let (terminal, released_items, released_bytes) = model_build.close_step(maximum_bytes);
                        if !terminal {
                            return session_pending(released_items, released_bytes);
                        }
                        self.model_build = None;
                        return session_pending(1, size_of::<MountedModelBuild>());
                    }
                    self.close_cursor += 1;
                }
                3 => {
                    if let Some(assembly_build) = self.assembly_build.as_mut() {
                        let (terminal, released_items, released_bytes) = assembly_build.close_step(maximum_bytes);
                        if !terminal {
                            return session_pending(released_items, released_bytes);
                        }
                        self.assembly_build = None;
                        return session_pending(1, size_of::<AssemblyJobConstruction>());
                    }
                    self.close_cursor += 1;
                }
                4 => {
                    if let Some(assembly) = self.assembly.as_mut() {
                        let (terminal, released_items, released_bytes) = assembly.close_retained_step(maximum_bytes);
                        if !terminal {
                            return session_pending(released_items, released_bytes);
                        }
                        self.assembly = None;
                        return session_pending(1, size_of::<AssemblyJob<'static>>());
                    }
                    self.close_cursor += 1;
                }
                5 => {
                    if let Some(csr_build) = self.csr_build.as_mut() {
                        let (terminal, released_items, released_bytes) = csr_build.close_step(maximum_bytes);
                        if !terminal {
                            return session_pending(released_items, released_bytes);
                        }
                        self.csr_build = None;
                        return session_pending(1, size_of::<AssemblyCsrBuild>());
                    }
                    self.close_cursor += 1;
                }
                6 => {
                    if let Some(pcg_build) = self.pcg_build.as_mut() {
                        let step = pcg_build.close_step(child);
                        if let Some(blocked) = session_child_failure(step) {
                            return blocked;
                        }
                        if !matches!(step, InteractiveJobCloseStep::Complete { .. }) {
                            return PluginLifecycleStep::Progress(step.progress());
                        }
                        self.pcg_build = None;
                        return session_pending(1, size_of::<PcgJobConstruction>());
                    }
                    self.close_cursor += 1;
                }
                7 => {
                    if let Some(pcg) = self.pcg.as_mut() {
                        let step = InteractiveJob::close_step(pcg, child);
                        if let Some(blocked) = session_child_failure(step) {
                            return blocked;
                        }
                        if !InteractiveJob::terminal_is_empty(pcg) {
                            return PluginLifecycleStep::Progress(step.progress());
                        }
                        self.pcg = None;
                        return session_pending(1, size_of::<PcgJob>());
                    }
                    self.close_cursor += 1;
                }
                8 => {
                    if let Some(domain) = self.domain.as_mut() {
                        match self.domain_close_lane {
                            0 if domain.outer.pop().is_some() => return session_pending(1, 0),
                            0 => {
                                let bytes = domain.outer.capacity() * size_of::<[f64; 2]>();
                                if bytes > maximum_bytes {
                                    return session_pending(0, 0);
                                }
                                domain.outer = Vec::new();
                                self.domain_close_lane = 1;
                                return session_pending(1, bytes);
                            }
                            1 => {
                                if let Some(hole) = domain.holes.last_mut() {
                                    if hole.pop().is_some() {
                                        return session_pending(1, 0);
                                    }
                                    let bytes = hole.capacity() * size_of::<[f64; 2]>();
                                    if bytes > maximum_bytes {
                                        return session_pending(0, 0);
                                    }
                                    *hole = Vec::new();
                                    domain.holes.pop();
                                    return session_pending(1, bytes);
                                }
                                self.domain_close_lane = 2;
                                continue;
                            }
                            2 => {
                                let bytes = domain.holes.capacity() * size_of::<Vec<[f64; 2]>>();
                                if bytes > maximum_bytes {
                                    return session_pending(0, 0);
                                }
                                domain.holes = Vec::new();
                                self.domain_close_lane = 3;
                                return session_pending(1, bytes);
                            }
                            _ => {
                                self.domain = None;
                                return session_pending(1, 0);
                            }
                        }
                    }
                    self.close_cursor += 1;
                }
                9 => {
                    if let Some(field) = self.visual.fields.last_mut() {
                        if field.node_id.capacity() != 0 {
                            let bytes = field.node_id.capacity();
                            if bytes > maximum_bytes {
                                return session_pending(0, 0);
                            }
                            field.node_id = String::new();
                            return session_pending(1, bytes);
                        }
                        self.visual.fields.pop();
                        return session_pending(1, 0);
                    }
                    let bytes = self.visual.fields.capacity() * size_of::<semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return session_pending(0, 0);
                        }
                        self.visual.fields = Vec::new();
                        return session_pending(1, bytes);
                    }
                    self.close_cursor += 1;
                }
                10 => {
                    if let Some(owner) = self.visual.assembling_element_ids.last_mut() {
                        if owner.capacity() != 0 {
                            let bytes = owner.capacity();
                            if bytes > maximum_bytes {
                                return session_pending(0, 0);
                            }
                            *owner = String::new();
                            return session_pending(1, bytes);
                        }
                        self.visual.assembling_element_ids.pop();
                        return session_pending(1, 0);
                    }
                    let bytes = self.visual.assembling_element_ids.capacity() * size_of::<String>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return session_pending(0, 0);
                        }
                        self.visual.assembling_element_ids = Vec::new();
                        return session_pending(1, bytes);
                    }
                    self.close_cursor += 1;
                }
                11 => {
                    if self.visual_region_owner.is_none() {
                        if let Some(owner) = self.visual.region_quality.take_one() {
                            self.visual_region_owner = Some(owner);
                            return session_pending(1, 0);
                        }
                    }
                    if let Some((id, _)) = self.visual_region_owner.as_mut() {
                        if id.capacity() != 0 {
                            let bytes = id.capacity();
                            if bytes > maximum_bytes {
                                return session_pending(0, 0);
                            }
                            *id = String::new();
                            return session_pending(1, bytes);
                        }
                        self.visual_region_owner = None;
                        return session_pending(1, 0);
                    }
                    self.close_cursor += 1;
                }
                12 => {
                    if let Some(candidate) = self.visual_rejected.as_mut() {
                        let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
                        if !terminal {
                            return session_pending(released_items, released_bytes);
                        }
                        if !candidate.terminal_is_empty() {
                            return PluginLifecycleStep::Blocked { reason: "mounted FEM rejected visual reported false terminal" };
                        }
                        self.visual_rejected = None;
                        return session_pending(1, 0);
                    }
                    if let Some(candidate) = self.visual_job_candidate.as_mut() {
                        let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
                        if !terminal {
                            return session_pending(released_items, released_bytes);
                        }
                        if !candidate.terminal_is_empty() {
                            return PluginLifecycleStep::Blocked { reason: "mounted FEM live visual candidate reported false terminal" };
                        }
                        self.visual_job_candidate = None;
                        return session_pending(1, 0);
                    }
                    self.close_cursor += 1;
                }
                13 => {
                    if let Some(displaced) = self.visual_displaced.as_mut() {
                        let (terminal, released_items, released_bytes) = displaced.close_step(maximum_bytes);
                        if !terminal {
                            return session_pending(released_items, released_bytes);
                        }
                        if !displaced.terminal_is_empty() {
                            return PluginLifecycleStep::Blocked { reason: "mounted FEM displaced visual reported false terminal" };
                        }
                        self.visual_displaced = None;
                        return session_pending(1, 0);
                    }
                    self.close_cursor += 1;
                }
                14 => {
                    if let Some(current) = self.visual_current.as_mut() {
                        let (terminal, released_items, released_bytes) = current.close_step(maximum_bytes);
                        if !terminal {
                            return session_pending(released_items, released_bytes);
                        }
                        if !current.terminal_is_empty() {
                            return PluginLifecycleStep::Blocked { reason: "mounted FEM current visual reported false terminal" };
                        }
                        self.visual_current = None;
                        return session_pending(1, 0);
                    }
                    self.close_cursor += 1;
                }
                15 => {
                    if let Some(snapshot) = self.snapshot.take() {
                        let Some(witness) = snapshot.return_to_registry_witness() else {
                            return PluginLifecycleStep::Blocked { reason: "mounted FEM snapshot lease was already returned" };
                        };
                        self.snapshot_return = Some(witness);
                        return session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>());
                    }
                    if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
                        return session_pending(0, 0);
                    }
                    if self.snapshot_return.take().is_some() {
                        return session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotReadReturn>());
                    }
                    self.close_cursor += 1;
                }
                16 => {
                    if let Some(fault) = self.fault.as_mut() {
                        if fault.pop().is_some() {
                            return session_pending(1, 0);
                        }
                        let bytes = fault.capacity();
                        if bytes > maximum_bytes {
                            return session_pending(0, 0);
                        }
                        *fault = Vec::new();
                        self.fault = None;
                        return session_pending(1, bytes);
                    }
                    self.close_cursor += 1;
                }
                _ => {
                    self.stage = MountedStage::Empty;
                    return PluginLifecycleStep::Complete(RetainedCloneProgress::default());
                }
            }
        }
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
