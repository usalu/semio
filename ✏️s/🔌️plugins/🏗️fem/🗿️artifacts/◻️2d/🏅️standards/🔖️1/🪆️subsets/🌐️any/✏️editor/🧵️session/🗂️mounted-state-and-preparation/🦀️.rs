//! 🗂️ Mounted state and preparation: the mounted state, the child end and failure helpers, the close constants and the state construction, progress, domain preparation and visual drive. See [`super`].

use super::*;

//#region 🧰️FixedArena
pub(super) struct MountedState {
    pub(super) identity: MountedIdentity,
    pub(super) snapshot: Option<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>,
    pub(super) snapshot_return: Option<store::os_store::snapshot_read::SnapshotReadReturn>,
    pub(super) cancel: semio_framework_job::CancelToken,
    pub(super) stage: MountedStage,
    pub(super) admitted_items: usize,
    pub(super) graph_plans: Vec<FemStagePlan>,
    pub(super) graph: Option<FemJobGraph>,
    pub(super) domain: Option<PlanarDomain>,
    domain_outer_cursor: usize,
    domain_hole_cursor: usize,
    domain_hole_point_cursor: usize,
    pub(super) domain_close_lane: u8,
    pub(super) mesh: Option<MeshJob>,
    pub(super) model_build: Option<MountedModelBuild>,
    pub(super) assembly_build: Option<AssemblyJobConstruction>,
    pub(super) assembly: Option<AssemblyJob<'static>>,
    pub(super) csr_build: Option<AssemblyCsrBuild>,
    pub(super) pcg_build: Option<PcgJobConstruction>,
    pub(super) pcg: Option<PcgJob>,
    pub(super) visual: Fem2dLiveVisual,
    pub(super) visual_region_owner: Option<(String, RegionVisualQuality)>,
    pub(super) visual_job_candidate: Option<Fem2dVisualJob>,
    pub(super) visual_rejected: Option<Fem2dVisualJob>,
    pub(super) visual_current: Option<Fem2dMountedVisualLease>,
    pub(super) visual_displaced: Option<Fem2dMountedVisualLease>,
    pub(super) visual_dirty: bool,
    pub(super) visual_field_cursor: usize,
    pub(super) visual_pcg_complete: bool,
    preview_sequence: u64,
    pub(super) close_cursor: u8,
    pub(super) fault: Option<Vec<u8>>,
}

/// 📋️ Copies a lent payload's pages when it fits the bound; the child job keeps owning the payload.
fn lent_bytes(payload: &RetainedJobPayload, maximum_bytes: usize) -> Option<Vec<u8>> {
    (payload.len() <= maximum_bytes).then(|| (0..payload.page_count()).map(|page| payload.page(page)).collect::<Option<Vec<&[u8]>>>().map(|pages| pages.concat())).flatten()
}

fn retained_payload_byte(payload: &RetainedJobPayload, index: usize) -> Option<u8> {
    let mut offset = index;
    for page in 0..payload.page_count() {
        let bytes = payload.page(page)?;
        if offset < bytes.len() {
            return bytes.get(offset).copied();
        }
        offset -= bytes.len();
    }
    None
}

/// 🧭️ How one child job step ended, copied out before the lent outcome borrow is released.
pub(super) enum ChildEnd {
    Yield,
    Preview { bytes: Option<Vec<u8>>, quality: Option<u8> },
    Checkpoint(Option<Vec<u8>>),
    Complete(Option<Vec<u8>>),
    Fault(Option<Vec<u8>>),
    Cancelled,
}

pub(super) fn child_end(result: Result<Option<JobOutcomeBorrow<'_>>, ValueError>) -> ChildEnd {
    match result {
        Ok(None | Some(JobOutcomeBorrow::Yield { .. })) => ChildEnd::Yield,
        Ok(Some(JobOutcomeBorrow::PreviewReady { payload, .. })) => ChildEnd::Preview { bytes: lent_bytes(payload, SESSION_MAXIMUM_OUTPUT_BYTES), quality: retained_payload_byte(payload, 8) },
        Ok(Some(JobOutcomeBorrow::CheckpointReady { state, .. })) => ChildEnd::Checkpoint(lent_bytes(state, SESSION_MAXIMUM_OUTPUT_BYTES)),
        Ok(Some(JobOutcomeBorrow::Complete { output, .. })) => ChildEnd::Complete(output.and_then(|payload| lent_bytes(payload, SESSION_MAXIMUM_OUTPUT_BYTES))),
        Ok(Some(JobOutcomeBorrow::Fault { detail, .. })) => ChildEnd::Fault(lent_bytes(detail, SESSION_MAXIMUM_FAULT_BYTES)),
        Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => ChildEnd::Cancelled,
        Err(_) => ChildEnd::Fault(None),
    }
}

/// 🎟️ One close unit's receipt: `items` released owners and `bytes` of released backing.
pub(super) fn session_child_failure(step: InteractiveJobCloseStep) -> Option<PluginLifecycleStep> {
    match step {
        InteractiveJobCloseStep::Blocked => Some(PluginLifecycleStep::Blocked { reason: "mounted FEM child close is blocked" }),
        InteractiveJobCloseStep::Refused { .. } => Some(PluginLifecycleStep::Blocked { reason: "mounted FEM child close was refused" }),
        InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. } => None,
    }
}

pub(super) fn session_pending(items: usize, bytes: usize) -> PluginLifecycleStep {
    PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() })
}

const fn largest(sizes: &[usize]) -> usize {
    let mut index = 0;
    let mut largest = 0;
    while index < sizes.len() {
        if sizes[index] > largest {
            largest = sizes[index];
        }
        index += 1;
    }
    largest
}

/// 📏️ The largest logical element one close unit retires; every quote of the next close turn is bounded by it.
pub(super) const SESSION_CLOSE_COPY_BYTES: usize = 4_096;

/// 🪜️ The deepest retained frontier one close unit reaches: session owner, job, writer and payload.
pub(super) const SESSION_CLOSE_DEPTH: usize = 3;

/// 📏️ The largest single owner release one close unit performs; every quote of the next close turn is bounded by it.
pub(super) const SESSION_CLOSE_RELEASE_BYTES: usize = largest(&[
    SESSION_OWNER_PAGE_BYTES,
    INPUT_BYTES,
    size_of::<FemJobGraph>(),
    size_of::<MeshJob>(),
    size_of::<MountedModelBuild>(),
    size_of::<AssemblyJobConstruction>(),
    size_of::<AssemblyJob<'static>>(),
    size_of::<AssemblyCsrBuild>(),
    size_of::<PcgJobConstruction>(),
    size_of::<PcgJob>(),
    size_of::<store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>>(),
    size_of::<store::os_store::snapshot_read::SnapshotReadReturn>(),
    size_of::<PendingSnapshotAdmission>(),
    size_of::<PendingSnapshotFault>(),
]);

impl MountedState {
    pub(super) fn new(identity: MountedIdentity, snapshot: store::os_store::snapshot_read::SnapshotRead<Fem2dSnapshot>, admitted_items: usize) -> Self {
        Self {
            identity,
            snapshot: Some(snapshot),
            snapshot_return: None,
            cancel: semio_framework_job::root_cancel_token(),
            stage: MountedStage::PrepareGraph,
            admitted_items,
            graph_plans: Vec::new(),
            graph: None,
            domain: None,
            domain_outer_cursor: 0,
            domain_hole_cursor: 0,
            domain_hole_point_cursor: 0,
            domain_close_lane: 0,
            mesh: None,
            model_build: None,
            assembly_build: None,
            assembly: None,
            csr_build: None,
            pcg_build: None,
            pcg: None,
            visual: Fem2dLiveVisual::default(),
            visual_region_owner: None,
            visual_job_candidate: None,
            visual_rejected: None,
            visual_current: None,
            visual_displaced: None,
            visual_dirty: true,
            visual_field_cursor: 0,
            visual_pcg_complete: false,
            preview_sequence: 0,
            close_cursor: 0,
            fault: None,
        }
    }

    pub(super) fn fail(&mut self, detail: impl Into<Vec<u8>>) -> JobStep {
        let mut detail = detail.into();
        if detail.capacity() > SESSION_MAXIMUM_FAULT_BYTES {
            detail = b"fem2d.session-fault-capacity".to_vec();
        }
        self.fault = Some(detail.clone());
        self.visual.state = FemVisualState::FaultedCancelled;
        self.stage = MountedStage::Fault;
        JobStep::Failed(detail)
    }

    pub(super) fn progress(&self, _label: &'static [u8]) -> JobStep {
        JobStep::Running(None)
    }

    pub(super) fn prepare_domain_one(&mut self) -> Result<bool, &'static [u8]> {
        let snapshot = self.snapshot.as_ref().ok_or(b"fem2d.session-snapshot-missing" as &'static [u8])?;
        let Some(region) = snapshot.regions.first() else { return Ok(true) };
        if self.domain.is_none() {
            let mut outer = Vec::new();
            outer.try_reserve_exact(region.outline.len()).map_err(|_| b"fem2d.session-domain-outer-allocation" as &'static [u8])?;
            let mut holes = Vec::new();
            holes.try_reserve_exact(region.holes.len()).map_err(|_| b"fem2d.session-domain-holes-allocation" as &'static [u8])?;
            if outer.capacity() * size_of::<[f64; 2]>() > SESSION_OWNER_PAGE_BYTES || holes.capacity() * size_of::<Vec<[f64; 2]>>() > SESSION_OWNER_PAGE_BYTES {
                return Err(b"fem2d.session-domain-page-capacity");
            }
            self.domain = Some(PlanarDomain { outer, holes });
            return Ok(false);
        }
        let domain = self.domain.as_mut().expect("domain initialized above");
        if let Some(point) = region.outline.get(self.domain_outer_cursor).copied() {
            domain.outer.push(point);
            self.domain_outer_cursor += 1;
            return Ok(false);
        }
        if self.domain_hole_cursor < region.holes.len() {
            if self.domain_hole_cursor == domain.holes.len() {
                let mut hole = Vec::new();
                hole.try_reserve_exact(region.holes[self.domain_hole_cursor].len()).map_err(|_| b"fem2d.session-domain-hole-allocation" as &'static [u8])?;
                if hole.capacity() * size_of::<[f64; 2]>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.session-domain-hole-page-capacity");
                }
                domain.holes.push(hole);
                return Ok(false);
            }
            if let Some(point) = region.holes[self.domain_hole_cursor].get(self.domain_hole_point_cursor).copied() {
                domain.holes[self.domain_hole_cursor].push(point);
                self.domain_hole_point_cursor += 1;
                return Ok(false);
            }
            self.domain_hole_cursor += 1;
            self.domain_hole_point_cursor = 0;
            return Ok(false);
        }
        Ok(true)
    }

    fn publish_visual_candidate(&mut self, candidate: Fem2dMountedVisualLease) -> Result<(), Fem2dMountedVisualLease> {
        if self.visual_displaced.is_some() {
            return Err(candidate);
        }
        self.visual_displaced = self.visual_current.take();
        self.visual_current = Some(candidate);
        Ok(())
    }

    fn visual_freshness(&self) -> Fem2dVisualFreshness {
        Fem2dVisualFreshness {
            app_instance_id: self.identity.app_instance_id,
            model_revision: self.identity.base_revision.0,
            document_generation: self.identity.generation.0,
            operation: self.identity.operation.0,
            numerical_preview_sequence: self.preview_sequence,
            surface_generation: self.identity.generation.0,
            renderer_scene_generation: self.identity.generation.0,
        }
    }

    pub(super) fn drive_visual_one(&mut self, cx: &mut StepContext<'_>) -> Option<JobStep> {
        if cx.should_yield() {
            return Some(JobStep::Running(None));
        }
        if let Some(rejected) = self.visual_rejected.as_mut() {
            cx.consume_fuel(1);
            let (terminal, _, _) = rejected.close_step(SESSION_OWNER_PAGE_BYTES);
            if terminal {
                self.visual_rejected = None;
            }
            return Some(self.progress(b"fem2d.visual-rejected-close"));
        }
        if let Some(displaced) = self.visual_displaced.as_mut() {
            cx.consume_fuel(1);
            let (terminal, _, _) = displaced.close_step(SESSION_OWNER_PAGE_BYTES);
            if terminal {
                self.visual_displaced = None;
            }
            return Some(self.progress(b"fem2d.visual-displaced-close"));
        }
        if self.visual_dirty && self.visual_job_candidate.is_some() {
            cx.consume_fuel(1);
            self.visual_rejected = self.visual_job_candidate.take();
            return Some(self.progress(b"fem2d.visual-candidate-displaced"));
        }
        if self.visual_dirty && self.visual_job_candidate.is_none() {
            cx.consume_fuel(1);
            self.visual_job_candidate = Some(Fem2dVisualJob::new(self.visual_freshness()));
            self.visual_dirty = false;
            return Some(self.progress(b"fem2d.visual-candidate-open"));
        }
        let freshness = self.visual_freshness();
        let candidate = self.visual_job_candidate.as_mut()?;
        let Some(snapshot) = self.snapshot.as_ref() else { return Some(self.fail(b"fem2d.visual-snapshot-owner".to_vec())) };
        cx.consume_fuel(1);
        match candidate.step_one(snapshot, &self.visual, freshness) {
            Ok(false) => Some(self.progress(b"fem2d.visual-candidate")),
            Ok(true) => {
                let Some(lease) = candidate.take_complete() else { return Some(self.fail(b"fem2d.visual-complete-owner".to_vec())) };
                let live = current_identity(self.identity.app_instance_id) == Some(self.identity) && snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision) && !self.cancel.is_cancelled_now();
                if !live {
                    self.visual_displaced = Some(lease);
                    self.visual_job_candidate = None;
                    return Some(self.progress(b"fem2d.visual-stale-close"));
                }
                if !lease.matches_freshness(freshness) {
                    self.visual_displaced = Some(lease);
                    self.visual_job_candidate = None;
                    return Some(self.progress(b"fem2d.visual-generation-stale-close"));
                }
                if let Err(exact_candidate) = self.publish_visual_candidate(lease) {
                    self.visual_displaced = Some(exact_candidate);
                    return Some(self.progress(b"fem2d.visual-publication-full"));
                }
                self.visual_job_candidate = None;
                Some(self.progress(b"fem2d.visual-published"))
            }
            Err(detail) => Some(self.fail(detail)),
        }
    }

}
//#endregion 🧰️FixedArena
