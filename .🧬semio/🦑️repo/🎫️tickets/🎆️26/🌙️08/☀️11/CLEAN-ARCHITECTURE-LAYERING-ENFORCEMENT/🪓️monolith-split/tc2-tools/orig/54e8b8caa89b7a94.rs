//! 🧩️ Drawing mutation candidate: the candidate phase machine state, the overlay patch, the source access, the clone source projections and the candidate authority with its construction, rebuild, retirement and close steps.

use super::*;

//#region 🔖️RetainedStoreInitialization
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DrawingMutationCandidatePhase {
    PreflightSource,
    PreflightMutation,
    LocateCloneSource,
    PrepareOwnedValue,
    PlanOwnedValue,
    BindOverlay,
    LocatePrimary,
    LocateSecondary,
    Apply,
    RebuildSource,
    LocateDestination,
    RebuildDestination,
    Complete,
    Retire,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DrawingContainerRebuildRole {
    Source,
    Destination,
    CloseSourceUndo,
}

#[derive(Clone, Copy)]
pub(super) struct DrawingContainerSourceUndo {
    pub parent: Option<DrawingLayerAddress>,
    pub index: usize,
}

pub(super) struct DrawingMutationOverlayPatch {
    pub source_owner: usize,
    pub committed: bool,
}

impl DrawingMutationOverlayPatch {
    pub fn bind(source: &DrawingSnapshot) -> Self {
        Self { source_owner: std::ptr::from_ref(source) as usize, committed: false }
    }

    pub fn validate(&self, source: &DrawingSnapshot) -> Result<(), &'static str> {
        if self.source_owner != std::ptr::from_ref(source) as usize {
            return Err("drawing-store.mutation-overlay-owner-changed");
        }
        Ok(())
    }

    pub fn commit(&mut self, source: &DrawingSnapshot) -> Result<(), &'static str> {
        self.validate(source)?;
        self.committed = true;
        Ok(())
    }
}

/// 🧭️ Borrowed or exclusive snapshot access; the borrowed form carries the sealed projection duplicate-layer clones read from.
pub(super) enum DrawingSourceAccess<'a, 'r> {
    Exclusive(&'a mut DrawingSnapshot),
    Shared(&'a DrawingSnapshot, RetainedCloneRef<'r, DrawingSnapshot>),
}

impl<'a, 'r> DrawingSourceAccess<'a, 'r> {
    pub fn read(&self) -> &DrawingSnapshot {
        match self {
            Self::Exclusive(source) => source,
            Self::Shared(source, _) => source,
        }
    }

    pub fn write(&mut self) -> Result<&mut DrawingSnapshot, &'static str> {
        match self {
            Self::Exclusive(source) => Ok(source),
            Self::Shared(..) => Err("drawing-store.mutation-exclusive-source-required"),
        }
    }

    pub fn base(&self) -> Option<RetainedCloneRef<'r, DrawingSnapshot>> {
        match self {
            Self::Exclusive(_) => None,
            Self::Shared(_, base) => Some(*base),
        }
    }
}

pub(super) fn drawing_mutation_asset_id(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::ImportImageAsset(value) => &value.asset_id, _ => unreachable!("asset id clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_asset(mutation: &DrawingMutation) -> &DrawingImageAsset { match mutation { DrawingMutation::ImportImageAsset(value) => &value.asset, _ => unreachable!("asset clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_layer(mutation: &DrawingMutation) -> &DrawingLayerNode { match mutation { DrawingMutation::CreateLayer(value) => &value.layer, _ => unreachable!("layer clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_image_key(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::UpdateImage(value) => &value.image_key, _ => unreachable!("image key clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_content(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::UpdateText(value) => &value.content, _ => unreachable!("text content clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_blend_mode(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::SetLayerBlendMode(value) => &value.blend_mode, _ => unreachable!("blend mode clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_new_name(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::RenameLayer(value) => &value.new_name, _ => unreachable!("layer name clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_boolean_operation(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::SetLayerBooleanOperation(value) => &value.boolean_operation, _ => unreachable!("boolean operation clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_segments(mutation: &DrawingMutation) -> &DrawingNativeSegments { match mutation { DrawingMutation::UpdatePathGeometry(value) => &value.segments, _ => unreachable!("path segments clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_fill(mutation: &DrawingMutation) -> &FillStyle { match mutation { DrawingMutation::ReplaceLayerFill(value) => value.fill.as_ref().expect("fill clone source is present"), _ => unreachable!("fill clone source is projected from its own variant") } }

pub(super) fn drawing_mutation_stroke(mutation: &DrawingMutation) -> &StrokeStyle { match mutation { DrawingMutation::ReplaceLayerStroke(value) => value.stroke.as_ref().expect("stroke clone source is present"), _ => unreachable!("stroke clone source is projected from its own variant") } }

pub(super) struct DrawingMutationCandidateAuthority {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub arena_pool: Option<std::sync::Arc<DrawingMutationArenaPool>>,
    pub arena_slot: usize,
    pub arena_generation: u64,
    pub arena_return_phase: u8,
    pub preflight_source: Option<DrawingSnapshotBoundsAuthority>,
    pub preflight_mutation: std::mem::ManuallyDrop<Option<DrawingMutationDigestAuthority>>,
    pub preflight_digest: Option<store::ArtifactStoreInitializationDigest>,
    pub workset: Option<DrawingMutationWorksetPlan>,
    pub overlay: Option<DrawingMutationOverlayPatch>,
    pub locator: Option<DrawingLayerLocator>,
    pub primary: Option<DrawingLayerAddress>,
    pub secondary: Option<DrawingLayerAddress>,
    pub layer_clone: std::mem::ManuallyDrop<Option<Box<DrawingLayerCloneAuthority>>>,
    pub clone_work: Option<DrawingLayerCloneWorkAuthority>,
    pub fill_clone: std::mem::ManuallyDrop<Option<DrawingFillCloneAuthority>>,
    pub stroke_clone: std::mem::ManuallyDrop<Option<DrawingStrokeCloneAuthority>>,
    pub segments_clone: std::mem::ManuallyDrop<Option<DrawingSegmentsCloneAuthority>>,
    pub text_clone: std::mem::ManuallyDrop<Option<DrawingTextCloneAuthority>>,
    pub asset_clone: std::mem::ManuallyDrop<Option<DrawingNativeCloneAuthority<DrawingImageAsset>>>,
    pub clone_text_work: semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingTextFootprintCursor,
    pub duplicate_rewrite: Option<DrawingDuplicateRewriteAuthority>,
    pub duplicate_id_owner: std::mem::ManuallyDrop<Option<String>>,
    pub rebuild: std::mem::ManuallyDrop<Option<DrawingContainerRebuildAuthority>>,
    pub rebuild_target: Option<Option<DrawingLayerAddress>>,
    pub rebuild_role: Option<DrawingContainerRebuildRole>,
    pub rebuild_close_phase: u8,
    pub source_undo: Option<DrawingContainerSourceUndo>,
    pub container_reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub container_output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub overlay_pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    pub pending_layer: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    pub retire_pending: std::mem::ManuallyDrop<Option<DrawingRetirementOwner>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub phase: DrawingMutationCandidatePhase,
    pub terminal: bool,
    pub fault: Option<&'static str>,
}

impl DrawingMutationCandidateAuthority {
    /// 🧮️ Borrows one slot of the process arena pool. `NotReady` (the pool is still bootstrapping under
    /// maintenance) and `Contended` (another thread holds the pool for one bounded borrow) are
    /// transient: the caller yields and borrows again on a later step.
    pub fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, DrawingMutationArenaBorrowError> {
        let (arena_pool, arena_slot, arena_generation, owner) = borrow_drawing_mutation_arena()?;
        Ok(Self::from_arena(operation, generation, arena_pool, arena_slot, arena_generation, owner))
    }

    #[cfg(test)]
    pub fn try_new_from_pool(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pool: std::sync::Arc<DrawingMutationArenaPool>) -> Result<Self, &'static str> {
        let (arena_pool, arena_slot, arena_generation, owner) = borrow_drawing_mutation_arena_from(pool)?;
        Ok(Self::from_arena(operation, generation, arena_pool, arena_slot, arena_generation, owner))
    }

    pub fn from_arena(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, arena_pool: std::sync::Arc<DrawingMutationArenaPool>, arena_slot: usize, arena_generation: u64, owner: DrawingMutationArenaOwner) -> Self {
        Self {
            operation,
            generation,
            arena_pool: Some(arena_pool),
            arena_slot,
            arena_generation,
            arena_return_phase: 0,
            preflight_source: Some(DrawingSnapshotBoundsAuthority::new()),
            preflight_mutation: std::mem::ManuallyDrop::new(Some(DrawingMutationDigestAuthority::new())),
            preflight_digest: Some(store::ArtifactStoreInitializationDigest::new(b"drawing.mutation-preflight")),
            workset: None,
            overlay: None,
            locator: None,
            primary: None,
            secondary: None,
            layer_clone: std::mem::ManuallyDrop::new(None),
            clone_work: None,
            fill_clone: std::mem::ManuallyDrop::new(None),
            stroke_clone: std::mem::ManuallyDrop::new(None),
            segments_clone: std::mem::ManuallyDrop::new(None),
            text_clone: std::mem::ManuallyDrop::new(None),
            asset_clone: std::mem::ManuallyDrop::new(None),
            clone_text_work: semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingTextFootprintCursor::default(),
            duplicate_rewrite: None,
            duplicate_id_owner: std::mem::ManuallyDrop::new(Some(owner.duplicate_id)),
            rebuild: std::mem::ManuallyDrop::new(None),
            rebuild_target: None,
            rebuild_role: None,
            rebuild_close_phase: 0,
            source_undo: None,
            container_reverse: std::mem::ManuallyDrop::new(Some(owner.reverse)),
            container_output: std::mem::ManuallyDrop::new(Some(owner.output)),
            overlay_pages: std::mem::ManuallyDrop::new(Some(owner.pages)),
            pending_layer: std::mem::ManuallyDrop::new(None),
            retire_pending: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: DrawingMutationCandidatePhase::PreflightSource,
            terminal: false,
            fault: None,
        }
    }

    /// 🧭️ Applies one relative selection transform through its own diff — the leaf resolves every addressed layer against
    /// its parent chain on `source`, and each patched transform or path geometry lands in place; the replaced geometry is
    /// retired through the candidate's retirement owner.
    pub fn apply_selection_transform(&mut self, source: &mut DrawingSnapshot, mutation: &DrawingMutation) -> Result<(), &'static str> {
        let outcome = <DrawingMutation as Mutation<DrawingSnapshot>>::diff(mutation, source);
        if !outcome.is_applicable(protocol::MergePolicy::default()) {
            return Err("drawing-store.selection-transform-rejected");
        }
        let mut retired: semio_framework_value::list::PagedList<semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>, {usize::MAX}> = Default::default();
        for entry in outcome.diff().layers.iter().flat_map(|delta| delta.modified.iter()) {
            let mut fault = None;
            let found = semio_s_artifact_draw_drawing_model::update_layer_in_tree(&mut source.layers, &entry.id, &mut |layer| {
                if let Some(transform) = entry.patch.transform.as_ref() {
                    semio_s_artifact_draw_drawing_model::layer_base_mut(layer).transform = transform.clone();
                }
                if let Some(segments) = entry.patch.path_segments.as_ref() {
                    match layer {
                        DrawingLayerNode::Path(path) => retired.push(std::mem::replace(&mut path.segments, segments.clone())),
                        _ => fault = Some("drawing-store.path-target-kind"),
                    }
                }
            });
            if let Some(fault) = fault {
                return Err(fault);
            }
            if !found {
                return Err("drawing-store.mutation-target-lost");
            }
        }
        if !retired.is_empty() {
            *self.retire_pending = Some(DrawingRetirementOwner::SegmentCollections(retired));
        }
        Ok(())
    }

    pub fn parent(mutation: &DrawingMutation) -> Option<&DrawingNativeText> {
        match mutation {
            DrawingMutation::CreateLayer(value) => value.parent_id.as_ref(),
            DrawingMutation::ReorderLayer(value) => value.parent_id.as_ref(),
            _ => None,
        }
    }

    pub fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        self.phase = DrawingMutationCandidatePhase::Fault;
        Err(fault)
    }

    pub fn return_arena_owner(&mut self) -> Result<Option<bool>, &'static str> {
        let Some(pool) = self.arena_pool.as_ref() else {
            return Ok(Some(true));
        };
        let Ok(mut state) = pool.state.try_lock() else {
            return Ok(None);
        };
        let slot = state.slots.get_mut(self.arena_slot).ok_or("drawing-store.mutation-arena-slot")?;
        if !slot.leased || slot.generation != self.arena_generation {
            return Err("drawing-store.mutation-arena-stale-generation");
        }
        match self.arena_return_phase {
            0 => {
                let reverse = self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?;
                if !reverse.is_empty() || reverse.capacity() < DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY || slot.reverse.is_some() {
                    return Err("drawing-store.mutation-reverse-arena-not-terminal");
                }
                slot.reverse = Some(self.container_reverse.take().expect("validated Drawing reverse owner remains retained"));
            }
            1 => {
                let output = self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?;
                if !output.is_empty() || output.capacity() < DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY || slot.output.is_some() {
                    return Err("drawing-store.mutation-output-arena-not-terminal");
                }
                slot.output = Some(self.container_output.take().expect("validated Drawing output owner remains retained"));
            }
            2 => {
                let pages = self.overlay_pages.as_ref().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                if pages.len() != DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
                    || pages.capacity() < DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
                    || pages.iter().any(|page| !page.is_empty() || page.capacity() < DRAWING_MUTATION_RETAINED_PAGE_BYTES)
                    || slot.pages.is_some()
                {
                    return Err("drawing-store.mutation-overlay-arena-not-terminal");
                }
                slot.pages = Some(self.overlay_pages.take().expect("validated Drawing page owner remains retained"));
            }
            3 => {
                let duplicate_id = self.duplicate_id_owner.as_ref().ok_or("drawing-store.mutation-duplicate-id-owner-missing")?;
                if !duplicate_id.is_empty() || duplicate_id.capacity() < DRAWING_DUPLICATE_ID_BYTES || slot.duplicate_id.is_some() {
                    return Err("drawing-store.mutation-duplicate-id-owner-not-terminal");
                }
                slot.duplicate_id = Some(self.duplicate_id_owner.take().expect("validated Drawing duplicate owner remains retained"));
                slot.leased = false;
            }
            _ => return Err("drawing-store.mutation-arena-return-phase"),
        }
        self.arena_return_phase += 1;
        let complete = self.arena_return_phase == 4;
        if complete && !slot.is_available() {
            return Err("drawing-store.mutation-arena-return-false-terminal");
        }
        drop(state);
        if complete {
            self.arena_pool = None;
        }
        Ok(Some(complete))
    }

    pub fn prepare_native_text(&mut self, source: RetainedCloneRef<'_, DrawingNativeText>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.text_clone.is_none() {
            *self.text_clone = Some(DrawingTextCloneAuthority::new());
            cx.consume_fuel(1);
            return Ok(false);
        }
        let clone = self.text_clone.as_mut().ok_or("drawing-store.text-clone-missing")?;
        if !clone.step(source, cx)? { return Ok(false); }
        let value = clone.value.as_ref().ok_or("drawing-store.text-clone-value")?;
        let complete = self.clone_text_work.step(value, 1)?;
        cx.consume_fuel(1);
        if !complete { return Ok(false); }
        let totals = self.clone_text_work.totals().ok_or("drawing-store.text-clone-work-incomplete")?;
        self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.chunks.checked_add(1).ok_or("drawing-store.text-clone-item-overflow")?, totals.backing_bytes)?;
        self.clone_text_work.close_step(1);
        Ok(true)
    }

    pub fn adopt_native_text(&mut self, target: &mut DrawingNativeText) -> Result<(), &'static str> {
        let replacement = self.text_clone.as_mut().ok_or("drawing-store.text-clone-missing")?.take().ok_or("drawing-store.text-false-terminal")?;
        let old = std::mem::replace(target, replacement);
        *self.retire_pending = Some(DrawingRetirementOwner::String(old));
        drop(self.text_clone.take());
        Ok(())
    }

    pub fn start_rebuild(&mut self, source: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>, remove_index: Option<usize>, insert_index: Option<usize>, role: DrawingContainerRebuildRole) -> Result<(), &'static str> {
        let workset = self.workset.ok_or("drawing-store.mutation-workset-missing")?;
        if self.container_reverse.is_none() {
            return Err("drawing-store.mutation-reverse-arena-missing");
        }
        if self.container_output.is_none() {
            return Err("drawing-store.mutation-output-arena-missing");
        }
        let container = DrawingLayerLocator::container_mut(source, parent).ok_or("drawing-store.mutation-container-missing")?;
        let source = std::mem::take(container);
        let pending = self.pending_layer.take();
        let reverse = self.container_reverse.take().expect("validated Drawing reverse arena remains retained");
        let output = self.container_output.take().expect("validated Drawing output arena remains retained");
        match DrawingContainerRebuildAuthority::new(source, remove_index, insert_index, pending, reverse, output, workset) {
            Ok(rebuild) => {
                *self.rebuild = Some(rebuild);
                self.rebuild_target = Some(parent);
                self.rebuild_role = Some(role);
                self.rebuild_close_phase = 0;
                Ok(())
            }
            Err(rejected) => {
                *container = rejected.source;
                *self.pending_layer = rejected.pending;
                *self.container_reverse = Some(rejected.reverse);
                *self.container_output = Some(rejected.output);
                Err("drawing-store.mutation-container-admission")
            }
        }
    }

    pub fn finish_rebuild(&mut self, source: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>) -> Result<Option<DrawingLayerNode>, &'static str> {
        if DrawingLayerLocator::container_mut(source, parent).is_none() {
            return Err("drawing-store.mutation-container-lost");
        }
        let (rebuilt_source, removed, reverse_arena, output_arena) = self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.take().ok_or("drawing-store.mutation-rebuild-false-terminal")?;
        *DrawingLayerLocator::container_mut(source, parent).expect("validated Drawing mutation container remains available") = rebuilt_source;
        *self.container_reverse = Some(reverse_arena);
        *self.container_output = Some(output_arena);
        let mut rebuild = self.rebuild.take().expect("Drawing completed rebuild remains exact");
        rebuild.terminal = true;
        drop(rebuild);
        self.rebuild_target = None;
        self.rebuild_role = None;
        self.rebuild_close_phase = 0;
        Ok(removed)
    }
}

impl DrawingMutationCandidateAuthority {
    pub fn needs_shared_snapshot(&self, mutation: &DrawingMutation) -> bool {
        matches!(mutation, DrawingMutation::DuplicateLayer(_)) && matches!(self.phase, DrawingMutationCandidatePhase::LocateCloneSource | DrawingMutationCandidatePhase::PrepareOwnedValue)
    }

    pub fn pump_retirement(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let grant = cx.retained_grant();
        let step = if self.retirement.is_some() {
            store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant)
        } else if self.retire_pending.is_some() {
            store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.retirement, grant)
        } else {
            return Ok(false);
        }
        .map_err(|_| "drawing-store.mutation-retirement")?;
        cx.consume_retained(step.progress()).map_err(|_| "drawing-store.mutation-retirement-grant")?;
        cx.consume_fuel(1);
        Ok(true)
    }

    pub fn take(&mut self) -> Option<()> {
        if !self.terminal {
            return None;
        }
        self.preflight_source = None;
        self.preflight_digest = None;
        self.workset = None;
        self.clone_work = None;
        if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            return None;
        }
        self.overlay = None;
        Some(())
    }

    pub fn pump_rebuild_close(&mut self, source: &mut DrawingSnapshot) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        let role = self.rebuild_role.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rebuild role missing"))?;
        let target = self.rebuild_target.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rebuild target missing"))?;
        let rebuild = self.rebuild.as_mut().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rebuild missing"))?;
        let ready = if self.rebuild_close_phase > 0 {
            true
        } else if role == DrawingContainerRebuildRole::CloseSourceUndo {
            rebuild.close_forward_step().map_err(|e|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,e))?
        } else {
            rebuild.rollback_step().map_err(|e|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,e))?
        };
        if !ready {
            return Ok(DrawingMutationCandidateAuthority::unit_step());
        }
        match self.rebuild_close_phase {
            0 => {
                let container = DrawingLayerLocator::container_mut(source, target).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback container missing"))?;
                if !container.is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback destination was not empty"));
                }
                *container = rebuild.source.take().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback source missing"))?;
            }
            1 => {
                if rebuild.pending.is_some() && self.pending_layer.is_some() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback pending owner collision"));
                }
                *self.pending_layer = rebuild.pending.take();
            }
            2 => {
                if self.container_reverse.is_some() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback reverse owner collision"));
                }
                *self.container_reverse = Some(rebuild.reverse.take().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback reverse owner missing"))?);
            }
            3 => {
                if self.container_output.is_some() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback output owner collision"));
                }
                *self.container_output = Some(rebuild.output.take().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback output owner missing"))?);
            }
            4 => {
                if rebuild.removed.is_some() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback retained an unexpected removed owner"));
                }
                rebuild.finish_handoff().map_err(|e|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,e))?;
                drop(self.rebuild.take());
                self.rebuild_target = None;
                self.rebuild_role = None;
                self.rebuild_close_phase = 0;
                if role != DrawingContainerRebuildRole::Destination {
                    self.source_undo = None;
                }
                return Ok(DrawingMutationCandidateAuthority::unit_step());
            }
            _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback handoff phase invalid")),
        }
        self.rebuild_close_phase += 1;
        Ok(DrawingMutationCandidateAuthority::unit_step())
    }

    pub fn unit_step() -> RetainedCloneStep {
        RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })
    }

    pub fn coarse_demand() -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { copy_bytes: DRAWING_OWNED_FIELD_BYTES, capacity_bytes: DRAWING_OWNED_FIELD_BYTES, release_bytes: DRAWING_OWNED_FIELD_BYTES, depth: 4 }
    }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "Drawing mutation candidate close depth overflow"))?; Ok(demand) };
        let unit = semio_framework_value::RetirementDemand { depth: 1, ..Default::default() };
        if self.preflight_mutation.is_some() { return Ok(unit); }
        if let Some(active) = self.retirement.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?); }
        if self.retire_pending.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.retire_pending)?); }
        if self.duplicate_rewrite.is_some() || self.rebuild.is_some() { return Ok(Self::coarse_demand()); }
        if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) && self.source_undo.is_some() { return Ok(unit); }
        let clone_demand = |grant: RetainedCloneGrant| semio_framework_value::RetirementDemand { copy_bytes: grant.maximum_copy_bytes, capacity_bytes: grant.maximum_capacity_bytes, release_bytes: grant.maximum_release_bytes, depth: grant.maximum_depth };
        if let Some(layer) = self.layer_clone.as_ref() { return nested(clone_demand(layer.close_demands(body)?)); }
        if let Some(fill) = self.fill_clone.as_ref() { return nested(clone_demand(fill.close_demands(body)?)); }
        if let Some(text) = self.text_clone.as_ref() { return nested(clone_demand(text.close_demands(body)?)); }
        if let Some(asset) = self.asset_clone.as_ref() { return nested(clone_demand(asset.close_demands(body)?)); }
        if let Some(segments) = self.segments_clone.as_ref() { return nested(clone_demand(segments.close_demands(body)?)); }
        if let Some(stroke) = self.stroke_clone.as_ref() { return nested(clone_demand(stroke.close_demands(body)?)); }
        if self.pending_layer.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_layer)?); }
        Ok(unit)
    }

    pub fn close_step(&mut self, mut source: Option<&mut DrawingSnapshot>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let false_terminal = |label: &'static str| ValueError::new(ValueRefusalKind::InvariantViolated, label);
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        self.clone_text_work.close_step(1);
        if let Some(preflight) = self.preflight_mutation.as_mut() {
            let step = preflight.close_step();
            if !preflight.terminal_is_empty() { return Err(false_terminal("Drawing mutation preflight reported false terminal")); }
            drop(self.preflight_mutation.take());
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.retire_pending.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.retirement, grant);
        }
        if let Some(rewrite) = self.duplicate_rewrite.as_mut() {
            let step = rewrite.close_step(child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                let pages = self.overlay_pages.as_mut().ok_or_else(|| false_terminal("Drawing duplicate name arena missing"))?;
                if pages.len() >= pages.capacity() {
                    return Err(false_terminal("Drawing duplicate name arena return saturated"));
                }
                let (id_owner, name_owner) = rewrite.take_owners().ok_or_else(|| false_terminal("Drawing duplicate rewrite reported false terminal"))?;
                *self.duplicate_id_owner = Some(id_owner);
                pages.push(name_owner);
                drop(self.duplicate_rewrite.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..step.progress() }));
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.rebuild.is_some() {
            let Some(source) = source.as_deref_mut() else { return Ok(RetainedCloneStep::Progress(empty)) };
            return self.pump_rebuild_close(source);
        }
        if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            if let Some(undo) = self.source_undo {
                let Some(source) = source else { return Ok(RetainedCloneStep::Progress(empty)) };
                self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo).map_err(|e| ValueError::new(ValueRefusalKind::InvariantViolated, e))?;
                return Ok(RetainedCloneStep::Progress(empty));
            }
        }
        self.clone_work = None;
        macro_rules! close_clone {
            ($slot:expr, $label:literal) => {
                if let Some(owner) = $slot.as_mut() {
                    let step = owner.close_step(child)?;
                    if matches!(step, RetainedCloneStep::Complete(_)) {
                        if !owner.terminal_is_empty() { return Err(false_terminal(concat!($label, " reported false terminal"))); }
                        drop($slot.take());
                    }
                    return Ok(RetainedCloneStep::Progress(step.progress()));
                }
            };
        }
        close_clone!(self.layer_clone, "Drawing mutation layer clone");
        close_clone!(self.fill_clone, "Drawing mutation fill clone");
        close_clone!(self.text_clone, "Text clone");
        close_clone!(self.asset_clone, "Asset clone");
        close_clone!(self.segments_clone, "Path clone");
        close_clone!(self.stroke_clone, "Drawing mutation stroke clone");
        if let Some(value) = self.pending_layer.take() {
            *self.retire_pending = Some(DrawingRetirementOwner::Layer(value));
            return Ok(RetainedCloneStep::Progress(empty));
        }
        match self.return_arena_owner() {
            Ok(Some(true)) => {}
            Ok(Some(false)) => return Ok(DrawingMutationCandidateAuthority::unit_step()),
            Ok(None) => return Ok(RetainedCloneStep::Progress(empty)),
            Err(error) => return Err(ValueError::new(ValueRefusalKind::InvariantViolated, error)),
        }
        self.preflight_source = None;
        self.preflight_digest = None;
        self.workset = None;
        self.clone_work = None;
        self.overlay = None;
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal
            && self.clone_text_work.terminal_is_empty()
            && self.preflight_source.is_none()
            && self.preflight_mutation.is_none()
            && self.preflight_digest.is_none()
            && self.workset.is_none()
            && self.arena_pool.is_none()
            && self.arena_return_phase == 4
            && self.overlay.is_none()
            && self.layer_clone.is_none()
            && self.clone_work.is_none()
            && self.fill_clone.is_none()
            && self.stroke_clone.is_none()
            && self.segments_clone.is_none()
            && self.text_clone.is_none()
            && self.asset_clone.is_none()
            && self.duplicate_rewrite.is_none()
            && self.duplicate_id_owner.is_none()
            && self.rebuild.is_none()
            && self.rebuild_target.is_none()
            && self.rebuild_role.is_none()
            && self.rebuild_close_phase == 0
            && self.source_undo.is_none()
            && self.container_reverse.is_none()
            && self.container_output.is_none()
            && self.overlay_pages.is_none()
            && self.pending_layer.is_none()
            && self.retire_pending.is_none()
            && self.retirement.is_none()
    }
}

impl Drop for DrawingMutationCandidateAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation candidate reached Drop before atomic handoff or cursor retirement");
    }
}
//#endregion 🔖️RetainedStoreInitialization
