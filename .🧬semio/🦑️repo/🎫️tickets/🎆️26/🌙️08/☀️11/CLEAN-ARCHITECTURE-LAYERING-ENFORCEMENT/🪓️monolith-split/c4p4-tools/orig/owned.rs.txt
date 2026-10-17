//! 🧰️ Drawing owned envelope decoder, recursive retirement, and retained store initializer.


use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_s_artifact_draw_drawing_host_arena::DRAWING_DUPLICATE_ID_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_LAYER_DEPTH;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_ITEMS;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_RETAINED_PAGE_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_OWNED_FIELD_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DrawingDecodedFieldRetirement;
use semio_s_artifact_draw_drawing_host_arena::DrawingFillCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingFixedOwnerCensus;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerAddress;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneWorkAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerLocator;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBootstrapJob;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBootstrapStep;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBorrowError;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaOwner;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPool;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeLayers;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeSegments;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeText;
use semio_s_artifact_draw_drawing_host_arena::DrawingRebuiltLayerOwners;
use semio_s_artifact_draw_drawing_host_arena::DrawingSegmentsCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotBoundsAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotOwnerTotals;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotRetirementFactory;
use semio_s_artifact_draw_drawing_host_arena::DrawingStrokeCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingTextCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingTraversalFrame;
use semio_s_artifact_draw_drawing_host_arena::borrow_drawing_mutation_arena;
use semio_s_artifact_draw_drawing_host_arena::borrow_drawing_mutation_arena_from;
use semio_s_artifact_draw_drawing_host_arena::clone_drawing_string;
use semio_s_artifact_draw_drawing_host_arena::drawing_fill_clone_work_totals;
use semio_s_artifact_draw_drawing_host_arena::drawing_stroke_clone_work_totals;
use semio_s_artifact_draw_drawing_host_arena::initial_snapshot_clone_grant;
use semio_s_artifact_draw_drawing_host_digest::DrawingContainerRebuildAuthority;
use semio_s_artifact_draw_drawing_host_digest::DrawingDuplicateRewriteAuthority;
use semio_s_artifact_draw_drawing_host_digest::DrawingMutationDigestAuthority;
use semio_s_artifact_draw_drawing_host_digest::DrawingMutationWorksetPlan;
use semio_s_artifact_draw_drawing_host_digest::drawing_mutation_target_owner;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::{DrawingImageAsset, DrawingLayerNode, FillStyle, PathSegment, StrokeStyle};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
#[cfg(test)] use semio_s_artifact_draw_drawing_model::GradientStop;
use protocol::{Mutation, OpBinary};
use store::ErasedSnapshotRetirement as _;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep};


use semio_s_artifact_draw_drawing_host_arena::native_retirement::DrawingRetirementOwner;

use semio_s_artifact_draw_drawing_host_arena::initialization_catalog_close::{close_initialization_catalog, initialization_catalog_close_demands};

//#region 🔖️OwnedSprCatalog


//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization


>;
>;
>;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingMutationCandidatePhase {
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
enum DrawingContainerRebuildRole {
    Source,
    Destination,
    CloseSourceUndo,
}

#[derive(Clone, Copy)]
struct DrawingContainerSourceUndo {
    pub parent: Option<DrawingLayerAddress>,
    pub index: usize,
}

struct DrawingMutationOverlayPatch {
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
enum DrawingSourceAccess<'a, 'r> {
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

fn drawing_mutation_asset_id(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::ImportImageAsset(value) => &value.asset_id, _ => unreachable!("asset id clone source is projected from its own variant") } }
fn drawing_mutation_asset(mutation: &DrawingMutation) -> &DrawingImageAsset { match mutation { DrawingMutation::ImportImageAsset(value) => &value.asset, _ => unreachable!("asset clone source is projected from its own variant") } }
fn drawing_mutation_layer(mutation: &DrawingMutation) -> &DrawingLayerNode { match mutation { DrawingMutation::CreateLayer(value) => &value.layer, _ => unreachable!("layer clone source is projected from its own variant") } }
fn drawing_mutation_image_key(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::UpdateImage(value) => &value.image_key, _ => unreachable!("image key clone source is projected from its own variant") } }
fn drawing_mutation_content(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::UpdateText(value) => &value.content, _ => unreachable!("text content clone source is projected from its own variant") } }
fn drawing_mutation_blend_mode(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::SetLayerBlendMode(value) => &value.blend_mode, _ => unreachable!("blend mode clone source is projected from its own variant") } }
fn drawing_mutation_new_name(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::RenameLayer(value) => &value.new_name, _ => unreachable!("layer name clone source is projected from its own variant") } }
fn drawing_mutation_boolean_operation(mutation: &DrawingMutation) -> &DrawingNativeText { match mutation { DrawingMutation::SetLayerBooleanOperation(value) => &value.boolean_operation, _ => unreachable!("boolean operation clone source is projected from its own variant") } }
fn drawing_mutation_segments(mutation: &DrawingMutation) -> &DrawingNativeSegments { match mutation { DrawingMutation::UpdatePathGeometry(value) => &value.segments, _ => unreachable!("path segments clone source is projected from its own variant") } }
fn drawing_mutation_fill(mutation: &DrawingMutation) -> &FillStyle { match mutation { DrawingMutation::ReplaceLayerFill(value) => value.fill.as_ref().expect("fill clone source is present"), _ => unreachable!("fill clone source is projected from its own variant") } }
fn drawing_mutation_stroke(mutation: &DrawingMutation) -> &StrokeStyle { match mutation { DrawingMutation::ReplaceLayerStroke(value) => value.stroke.as_ref().expect("stroke clone source is present"), _ => unreachable!("stroke clone source is projected from its own variant") } }

struct DrawingMutationCandidateAuthority {
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

    pub fn step(&mut self, mut access: DrawingSourceAccess<'_, '_>, mutation_ref: RetainedCloneRef<'_, DrawingMutation>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let mutation = mutation_ref.get();
        if (cx.operation() != self.operation || cx.generation() != self.generation) && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            return self.fail("drawing-store.mutation-candidate-stale-authority");
        }
        if cx.is_cancelled() && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            self.fault = Some("drawing-store.mutation-candidate-cancelled");
            self.phase = DrawingMutationCandidatePhase::Retire;
            return Err("drawing-store.mutation-candidate-cancelled");
        }
        if let Some(overlay) = self.overlay.as_ref() {
            overlay.validate(access.read())?;
        }
        match self.phase {
            DrawingMutationCandidatePhase::PreflightSource => {
                let source = access.read();
                let bounds = self.preflight_source.as_mut().ok_or("drawing-store.mutation-source-preflight-missing")?;
                if bounds.step(source, cx)? {
                    self.phase = DrawingMutationCandidatePhase::PreflightMutation;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::PreflightMutation => {
                let digest = self.preflight_mutation.as_mut().ok_or("drawing-store.mutation-preflight-missing")?;
                if !digest.step(mutation, self.preflight_digest.as_mut().ok_or("drawing-store.mutation-preflight-digest")?, cx)? {
                    return Ok(false);
                }
                let source_credit = self.preflight_source.as_ref().and_then(DrawingSnapshotBoundsAuthority::totals).ok_or("drawing-store.mutation-source-preflight-incomplete")?;
                let mutation_credit = digest.totals().ok_or("drawing-store.mutation-preflight-incomplete")?;
                let reverse_slots = self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?.capacity();
                let output_slots = self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?.capacity();
                let overlay_pages = self.overlay_pages.as_ref().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                let overlay_slots = overlay_pages.capacity();
                let required_overlay_slots = usize::from(matches!(
                    mutation,
                    DrawingMutation::DuplicateLayer(_)
                ));
                if required_overlay_slots > overlay_slots {
                    return Err("drawing-store.mutation-overlay-slot-capacity");
                }
                let duplicate_id = self.duplicate_id_owner.as_ref().ok_or("drawing-store.mutation-duplicate-id-arena-missing")?;
                let (arena_items, arena_bytes) = DrawingMutationArenaOwner::retained_totals(
                    self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?,
                    self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?,
                    overlay_pages,
                    duplicate_id,
                )?;
                let container_slots = reverse_slots.checked_add(output_slots).ok_or("drawing-store.mutation-container-credit-overflow")?;
                self.workset = Some(DrawingMutationWorksetPlan::admit(source_credit, mutation_credit, arena_items, arena_bytes, container_slots, size_of::<Self>())?);
                drop(self.preflight_mutation.take());
                self.preflight_source = None;
                self.preflight_digest = None;
                if matches!(mutation, DrawingMutation::DuplicateLayer(_)) {
                    self.locator = Some(DrawingLayerLocator::new());
                    self.phase = DrawingMutationCandidatePhase::LocateCloneSource;
                } else if matches!(mutation, DrawingMutation::ImportImageAsset(_) | DrawingMutation::RemoveImageAsset(_) | DrawingMutation::CreateLayer(_) | DrawingMutation::ReplaceLayerFill(_) | DrawingMutation::ReplaceLayerStroke(_) | DrawingMutation::UpdatePathGeometry(_) | DrawingMutation::UpdateText(_) | DrawingMutation::UpdateImage(_) | DrawingMutation::SetLayerBlendMode(_) | DrawingMutation::RenameLayer(_) | DrawingMutation::SetLayerBooleanOperation(_)) {
                    self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
                } else {
                    self.phase = DrawingMutationCandidatePhase::BindOverlay;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocateCloneSource => {
                let source = access.read();
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-clone-locator-missing")?;
                if !locator.step(source, drawing_mutation_target_owner(mutation).ok_or("drawing-store.mutation-target-missing")?, cx)? {
                    return Ok(false);
                }
                self.primary = locator.found();
                self.locator = None;
                if self.primary.is_none() {
                    return Err("drawing-store.mutation-target-missing");
                }
                self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
                Ok(false)
            }
            DrawingMutationCandidatePhase::BindOverlay => {
                let source = access.read();
                self.clone_work = None;
                self.overlay = Some(DrawingMutationOverlayPatch::bind(source));
                if matches!(mutation, DrawingMutation::ImportImageAsset(_) | DrawingMutation::RemoveImageAsset(_) | DrawingMutation::DuplicateLayer(_) | DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_)) {
                    self.phase = DrawingMutationCandidatePhase::Apply;
                } else {
                    self.locator = Some(DrawingLayerLocator::new());
                    self.phase = DrawingMutationCandidatePhase::LocatePrimary;
                }
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocatePrimary => {
                let source = access.read();
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-locator-missing")?;
                if !locator.step(source, drawing_mutation_target_owner(mutation).ok_or("drawing-store.mutation-target-missing")?, cx)? {
                    return Ok(false);
                }
                self.primary = locator.found();
                self.locator = None;
                if matches!(mutation, DrawingMutation::CreateLayer(_)) {
                    if self.primary.is_some() {
                        return Err("drawing-store.mutation-duplicate-layer");
                    }
                    if Self::parent(mutation).is_some() {
                        self.locator = Some(DrawingLayerLocator::new());
                        self.phase = DrawingMutationCandidatePhase::LocateSecondary;
                    } else {
                        self.phase = DrawingMutationCandidatePhase::Apply;
                    }
                } else if self.primary.is_none() {
                    return Err("drawing-store.mutation-target-missing");
                } else {
                    self.phase = DrawingMutationCandidatePhase::Apply;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocateSecondary => {
                let source = access.read();
                let target = Self::parent(mutation).ok_or("drawing-store.mutation-parent-missing")?;
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-parent-locator")?;
                if !locator.step(source, target, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("drawing-store.mutation-parent-not-found") };
                if !matches!(DrawingLayerLocator::node_at(source, address), Some(DrawingLayerNode::Group(_))) {
                    return Err("drawing-store.mutation-parent-not-group");
                }
                self.phase = DrawingMutationCandidatePhase::Apply;
                Ok(false)
            }
            DrawingMutationCandidatePhase::PrepareOwnedValue => {
                let source = access.read();
                match mutation {
                    DrawingMutation::RemoveImageAsset(value)=>{
                        if value.asset_id.is_empty()||value.asset_id.len()>4096||source.assets.get(&value.asset_id).is_none(){return Err("drawing-store.asset-missing");}
                        let locator=self.locator.get_or_insert_with(DrawingLayerLocator::asset_reference);
                        if !locator.step(source,&value.asset_id,cx)?{return Ok(false);}
                        if locator.found().is_some(){return Err("drawing-store.asset-still-referenced");}
                        self.locator=None;
                    }
                    DrawingMutation::ImportImageAsset(value)=>{
                        if value.asset_id.is_empty() || value.asset_id.len()>4096 || value.asset.width==0 || value.asset.height==0 || (value.asset.width as usize).checked_mul(value.asset.height as usize)!=Some(value.asset.samples.len()) || source.assets.get(&value.asset_id).is_some(){return Err("drawing-store.asset-invalid");}
                        if !self.prepare_native_text(mutation_ref.project(1, drawing_mutation_asset_id), cx)?{return Ok(false);}
                        if self.asset_clone.is_none(){self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(value.asset.samples.len()+1,value.asset.samples.allocated_bytes().checked_add(size_of::<DrawingImageAsset>()).ok_or("drawing-store.asset-size-overflow")?)?;}
                        let clone=self.asset_clone.get_or_insert_with(DrawingNativeCloneAuthority::new);
                        if !clone.step(mutation_ref.project(2, drawing_mutation_asset), cx)?{return Ok(false);}
                    }
                    DrawingMutation::CreateLayer(_value) => {
                        if self.layer_clone.is_none() {
                            *self.layer_clone = Some(Box::new(DrawingLayerCloneAuthority::new()));
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                        let clone = self.layer_clone.as_mut().expect("Drawing create layer clone remains retained");
                        if !clone.step(mutation_ref.project(3, drawing_mutation_layer), cx)? {
                            return Ok(false);
                        }
                        *self.pending_layer = clone.take();
                        drop(self.layer_clone.take());
                        self.clone_work = Some(DrawingLayerCloneWorkAuthority::new());
                    }
                    DrawingMutation::DuplicateLayer(value) => {
                        let duplicate_source = DrawingLayerLocator::node_at(source, self.primary.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-duplicate-source")?;
                        if self.pending_layer.is_none() {
                            if self.layer_clone.is_none() {
                                *self.layer_clone = Some(Box::new(DrawingLayerCloneAuthority::new()));
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            let clone = self.layer_clone.as_mut().expect("Drawing duplicate layer clone remains retained");
                            let base = access.base().ok_or("drawing-store.duplicate-source-custody-missing")?;
                            let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                            if !clone.step(base.project(4, move |snapshot| DrawingLayerLocator::node_at(snapshot, address).expect("sealed Drawing duplicate source remains addressable")), cx)? {
                                return Ok(false);
                            }
                            *self.pending_layer = clone.take();
                            drop(self.layer_clone.take());
                            let name_owner = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?.pop().ok_or("drawing-store.duplicate-name-owner-missing")?;
                            self.duplicate_rewrite = Some(DrawingDuplicateRewriteAuthority::new(self.duplicate_id_owner.take().ok_or("drawing-store.duplicate-id-owner-missing")?, name_owner));
                            return Ok(false);
                        }
                        if !self.duplicate_rewrite.as_mut().ok_or("drawing-store.duplicate-rewrite-missing")?.step(self.pending_layer.as_mut().ok_or("drawing-store.duplicate-owner-missing")?, duplicate_source, source, &value.identities, cx)? {
                            return Ok(false);
                        }
                        let pages = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                        if pages.len() >= pages.capacity() {
                            return Err("drawing-store.duplicate-name-owner-return-saturated");
                        }
                        let (id_owner, name_owner) = self.duplicate_rewrite.as_mut().and_then(DrawingDuplicateRewriteAuthority::take_owners).ok_or("drawing-store.duplicate-owner-false-terminal")?;
                        *self.duplicate_id_owner = Some(id_owner);
                        pages.push(name_owner);
                        drop(self.duplicate_rewrite.take());
                        self.clone_work = Some(DrawingLayerCloneWorkAuthority::new());
                    }
                    DrawingMutation::UpdateImage(value)=>{
                        if ![value.width,value.height].iter().all(|number|number.is_finite()&&*number>0.0){return Err("drawing-store.image-dimension-invalid");}
                        if !self.prepare_native_text(mutation_ref.project(5, drawing_mutation_image_key), cx)?{return Ok(false);}
                    }
                    DrawingMutation::UpdateText(value) => {
                        if !value.size.is_finite() || value.size <= 0.0 { return Err("drawing-store.text-size-invalid"); }
                        if !self.prepare_native_text(mutation_ref.project(6, drawing_mutation_content), cx)? { return Ok(false); }
                    }
                    DrawingMutation::SetLayerBlendMode(_value) => { if !self.prepare_native_text(mutation_ref.project(7, drawing_mutation_blend_mode), cx)? { return Ok(false); } }
                    DrawingMutation::RenameLayer(_value) => { if !self.prepare_native_text(mutation_ref.project(8, drawing_mutation_new_name), cx)? { return Ok(false); } }
                    DrawingMutation::SetLayerBooleanOperation(_value) => { if !self.prepare_native_text(mutation_ref.project(9, drawing_mutation_boolean_operation), cx)? { return Ok(false); } }
                    DrawingMutation::UpdatePathGeometry(_value) => {
                        if self.segments_clone.is_none() {
                            *self.segments_clone = Some(DrawingSegmentsCloneAuthority::new());
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                        let clone = self.segments_clone.as_mut().ok_or("drawing-store.path-clone-missing")?;
                        if !clone.step(mutation_ref.project(10, drawing_mutation_segments), cx)? { return Ok(false); }
                        let (items, bytes) = DrawingLayerCloneWorkAuthority::vector(clone.value.as_ref().ok_or("drawing-store.path-clone-value")?)?;
                        self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(items, bytes)?;
                    }
                    DrawingMutation::ReplaceLayerFill(value) => {
                        if value.fill.is_some() {
                            if self.fill_clone.is_none() {
                                *self.fill_clone = Some(DrawingFillCloneAuthority::new());
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            if !self.fill_clone.as_mut().expect("Drawing fill clone remains retained").step(mutation_ref.project(11, drawing_mutation_fill), cx)? {
                                return Ok(false);
                            }
                            let cloned = self.fill_clone.as_ref().and_then(|clone| clone.value.as_ref()).ok_or("drawing-store.fill-clone-work-missing")?;
                            let totals = drawing_fill_clone_work_totals(cloned)?;
                            self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                        }
                    }
                    DrawingMutation::ReplaceLayerStroke(value) => {
                        if value.stroke.is_some() {
                            if self.stroke_clone.is_none() {
                                *self.stroke_clone = Some(DrawingStrokeCloneAuthority::new());
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            if !self.stroke_clone.as_mut().expect("Drawing stroke clone remains retained").step(mutation_ref.project(12, drawing_mutation_stroke), cx)? {
                                return Ok(false);
                            }
                            let cloned = self.stroke_clone.as_ref().and_then(|clone| clone.value.as_ref()).ok_or("drawing-store.stroke-clone-work-missing")?;
                            let totals = drawing_stroke_clone_work_totals(cloned)?;
                            self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                        }
                    }
                    _ => {}
                }
                self.phase = if self.clone_work.is_some() { DrawingMutationCandidatePhase::PlanOwnedValue } else { DrawingMutationCandidatePhase::BindOverlay };
                Ok(false)
            }
            DrawingMutationCandidatePhase::PlanOwnedValue => {
                let pending = self.pending_layer.as_ref().ok_or("drawing-store.mutation-clone-work-owner-missing")?;
                let work = self.clone_work.as_mut().ok_or("drawing-store.mutation-clone-work-missing")?;
                if !work.step(pending, cx)? {
                    return Ok(false);
                }
                let totals = work.totals().ok_or("drawing-store.mutation-clone-work-false-terminal")?;
                self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                self.preflight_digest = None;
                self.phase = DrawingMutationCandidatePhase::BindOverlay;
                Ok(false)
            }
            DrawingMutationCandidatePhase::Apply => {
                let source = access.write()?;
                match mutation {
                    DrawingMutation::ImportImageAsset(_)=>{
                        let key=self.text_clone.as_mut().ok_or("drawing-store.asset-key-clone")?.take().ok_or("drawing-store.asset-key-incomplete")?;
                        let asset=self.asset_clone.as_mut().ok_or("drawing-store.asset-clone")?.take().ok_or("drawing-store.asset-incomplete")?;
                        source.assets.insert(key,asset);drop(self.text_clone.take());
                        self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;self.phase=DrawingMutationCandidatePhase::Complete;cx.consume_fuel(1);return Ok(false);
                    }
                    DrawingMutation::RemoveImageAsset(value)=>{
                        let asset=source.assets.remove(&value.asset_id).ok_or("drawing-store.asset-missing")?;
                        *self.retire_pending = Some(DrawingRetirementOwner::Asset(asset));
                        self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;self.phase=DrawingMutationCandidatePhase::Complete;cx.consume_fuel(1);return Ok(false);
                    }
                    DrawingMutation::CreateLayer(value) => {
                        let parent = self.secondary;
                        let index = match value.index {
                            Some(index) => index,
                            None => DrawingLayerLocator::container_mut(source, parent).map_or(0, |values| values.len()),
                        };
                        self.start_rebuild(source, parent, None, Some(index), DrawingContainerRebuildRole::Destination)?;
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawingMutation::DuplicateLayer(_) => {
                        let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                        self.start_rebuild(source, address.parent(), None, Some(address.index() + 1), DrawingContainerRebuildRole::Destination)?;
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawingMutation::DeleteLayer(_) | DrawingMutation::ReorderLayer(_) => {
                        let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                        self.start_rebuild(source, address.parent(), Some(address.index()), None, DrawingContainerRebuildRole::Source)?;
                        self.source_undo = Some(DrawingContainerSourceUndo { parent: address.parent(), index: address.index() });
                        self.phase = DrawingMutationCandidatePhase::RebuildSource;
                        return Ok(false);
                    }
                    DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_) => {
                        self.apply_selection_transform(source, mutation)?;
                        self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                        self.phase = DrawingMutationCandidatePhase::Complete;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                    _ => {}
                }
                let address = self.primary;
                match mutation {
                    DrawingMutation::SetLayerVisible(value) => {
                        semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).visible = value.visible;
                    }
                    DrawingMutation::SetLayerLocked(value) => {
                        semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).locked = value.locked;
                    }
                    DrawingMutation::SetGroupIsolation(value) => {
                        let DrawingLayerNode::Group(group)=DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {return Err("drawing-store.mutation-isolation-target-not-group");};
                        group.isolation=value.isolation;
                    }
                    DrawingMutation::SetLayerFillRule(value) => {
                        semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.fill_rule=value.fill_rule;
                    }
                    DrawingMutation::SetLayerOpacity(value) if value.opacity.is_finite() => {
                        semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).opacity = value.opacity;
                    }
                    DrawingMutation::SetLayerOpacity(_) => return Err("drawing-store.mutation-opacity-invalid"),
                    DrawingMutation::SetLayerBlendMode(value) => {
                        if !semio_s_artifact_draw_drawing_model::DRAWING_BLEND_MODES.iter().any(|allowed| value.blend_mode.eq_str(allowed)) { return Err("drawing-store.mutation-blend-mode-invalid"); }
                        self.adopt_native_text(&mut semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).blend_mode)?;
                    }
                    DrawingMutation::SetShapeCoordinate(value)=>{
                        let DrawingLayerNode::Shape(shape)=DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else{return Err("drawing-store.shape-target-kind");};
                        semio_s_artifact_draw_drawing_model::shape_geometry::set_shape_coordinate(shape,value.field,value.index,value.value)?;
                    }
                    DrawingMutation::UpdateImage(value)=>{
                        if ![value.width,value.height].iter().all(|number|number.is_finite()&&*number>0.0){return Err("drawing-store.image-dimension-invalid");}
                        let DrawingLayerNode::Image(image)=DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else{return Err("drawing-store.image-target-kind");};
                        self.adopt_native_text(&mut image.image_key)?;image.width=value.width;image.height=value.height;
                    }
                    DrawingMutation::UpdateText(value) => {
                        if !value.size.is_finite() || value.size <= 0.0 { return Err("drawing-store.text-size-invalid"); }
                        let DrawingLayerNode::Text(text) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else { return Err("drawing-store.text-target-kind"); };
                        self.adopt_native_text(&mut text.content)?;
                        text.size = value.size;
                    }
                    DrawingMutation::RenameLayer(_) => {
                        self.adopt_native_text(&mut semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).name)?;
                    }
                    DrawingMutation::UpdateLayerTransform(value)
                        if [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation, value.transform.shear].iter().all(|field| field.is_finite()) =>
                    {
                        semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).transform =
                            semio_s_artifact_draw_drawing_model::DrawingTransform { x: value.transform.x, y: value.transform.y, scale_x: value.transform.scale_x, scale_y: value.transform.scale_y, rotation: value.transform.rotation, shear: value.transform.shear };
                    }
                    DrawingMutation::UpdateLayerTransform(_) => return Err("drawing-store.mutation-transform-invalid"),
                    DrawingMutation::UpdatePathGeometry(_) => {
                        let DrawingLayerNode::Path(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else { return Err("drawing-store.path-target-kind"); };
                        let replacement = self.segments_clone.as_mut().ok_or("drawing-store.path-clone-missing")?.take().ok_or("drawing-store.path-false-terminal")?;
                        let old = std::mem::replace(&mut target.segments, replacement);
                        *self.retire_pending = Some(DrawingRetirementOwner::Segments(old));
                        drop(self.segments_clone.take());
                    }
                    DrawingMutation::ReplaceLayerFill(value) => {
                        let replacement = match value.fill.as_ref() {
                            Some(_) => Some(self.fill_clone.as_mut().ok_or("drawing-store.fill-clone-missing")?.take().ok_or("drawing-store.fill-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.fill,
                            replacement,
                        );
                        if let Some(old) = old {
                            *self.retire_pending = Some(DrawingRetirementOwner::Fill(old));
                        }
                        if let Some(mut clone) = self.fill_clone.take() {
                            clone.terminal = true;
                            drop(clone);
                        }
                    }
                    DrawingMutation::ReplaceLayerStroke(value) => {
                        let replacement = match value.stroke.as_ref() {
                            Some(_) => Some(self.stroke_clone.as_mut().ok_or("drawing-store.stroke-clone-missing")?.take().ok_or("drawing-store.stroke-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.stroke,
                            replacement,
                        );
                        if let Some(old) = old {
                            *self.retire_pending = Some(DrawingRetirementOwner::Stroke(old));
                        }
                        if let Some(mut clone) = self.stroke_clone.take() {
                            clone.terminal = true;
                            drop(clone);
                        }
                    }
                    DrawingMutation::SetLayerBooleanOperation(_value) => {
                        let DrawingLayerNode::Boolean(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {
                            return Err("drawing-store.mutation-boolean-target");
                        };
                        self.adopt_native_text(&mut target.operation)?;
                    }
                    DrawingMutation::UpdateLayerTraceParams(value) if value.params.threshold.is_finite() && value.params.simplify_epsilon.is_finite() => {
                        let DrawingLayerNode::Trace(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {
                            return Err("drawing-store.mutation-trace-target");
                        };
                        target.params = semio_s_artifact_draw_drawing_model::DrawingTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon };
                    }
                    DrawingMutation::UpdateLayerTraceParams(_) => return Err("drawing-store.mutation-trace-invalid"),
                    DrawingMutation::CreateLayer(_) | DrawingMutation::DuplicateLayer(_) | DrawingMutation::DeleteLayer(_) | DrawingMutation::ReorderLayer(_) => {
                        unreachable!("structural Drawing mutations start retained rebuild before scalar mutation")
                    }
                    DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_) => {
                        unreachable!("selection transforms apply their resolved patches before scalar mutation")
                    }
                    DrawingMutation::ImportImageAsset(_) | DrawingMutation::RemoveImageAsset(_) => {
                        unreachable!("image asset mutations commit before scalar mutation")
                    }
                }
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                self.phase = DrawingMutationCandidatePhase::Complete;
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawingMutationCandidatePhase::RebuildSource => {
                let source = access.write()?;
                if !self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent();
                let removed = self.finish_rebuild(source, parent)?.ok_or("drawing-store.mutation-removal-missing")?;
                *self.pending_layer = Some(removed);
                if matches!(mutation, DrawingMutation::DeleteLayer(_)) {
                    self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                    self.source_undo = None;
                    self.phase = DrawingMutationCandidatePhase::Complete;
                } else {
                    self.secondary = None;
                    if Self::parent(mutation).is_some() {
                        self.locator = Some(DrawingLayerLocator::new());
                        self.phase = DrawingMutationCandidatePhase::LocateDestination;
                    } else {
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                    }
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocateDestination => {
                let source = access.write()?;
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-destination-locator")?;
                if !locator.step(source, Self::parent(mutation).ok_or("drawing-store.mutation-parent-missing")?, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("drawing-store.mutation-parent-not-found") };
                if !matches!(DrawingLayerLocator::node_at(source, address), Some(DrawingLayerNode::Group(_))) {
                    return Err("drawing-store.mutation-parent-not-group");
                }
                self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                Ok(false)
            }
            DrawingMutationCandidatePhase::RebuildDestination => {
                let source = access.write()?;
                if self.rebuild.is_none() {
                    let parent = match mutation {
                        DrawingMutation::CreateLayer(_) => self.secondary,
                        DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent(),
                        DrawingMutation::ReorderLayer(_) => self.secondary,
                        _ => return Err("drawing-store.mutation-destination-variant"),
                    };
                    let index = match mutation {
                        DrawingMutation::CreateLayer(value) => value.index.unwrap_or_else(|| DrawingLayerLocator::container_mut(source, parent).map_or(0, |values| values.len())),
                        DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.index() + 1,
                        DrawingMutation::ReorderLayer(value) => value.index,
                        _ => 0,
                    };
                    self.start_rebuild(source, parent, None, Some(index), DrawingContainerRebuildRole::Destination)?;
                }
                if !self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = match mutation {
                    DrawingMutation::CreateLayer(_) | DrawingMutation::ReorderLayer(_) => self.secondary,
                    DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent(),
                    _ => None,
                };
                if self.finish_rebuild(source, parent)?.is_some() {
                    return Err("drawing-store.mutation-unexpected-removal");
                }
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                self.source_undo = None;
                self.phase = DrawingMutationCandidatePhase::Complete;
                Ok(false)
            }
            DrawingMutationCandidatePhase::Complete => {
                let source = access.write()?;
                if self.pump_retirement(cx)? {
                    return Ok(false);
                }
                if let Some(value) = self.pending_layer.take() {
                    *self.retire_pending = Some(DrawingRetirementOwner::Layer(value));
                    return Ok(false);
                }
                if self.return_arena_owner()? != Some(true) {
                    return Ok(false);
                }
                if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
                    self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                }
                self.terminal = true;
                Ok(true)
            }
            DrawingMutationCandidatePhase::Retire | DrawingMutationCandidatePhase::Fault => Err(self.fault.unwrap_or("drawing-store.mutation-candidate-fault")),
        }
    }

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingStoreInitializationPhase {
    InitializeArena,
    AdmitOwners,
    BindGenesis,
    ValidateEnvelope,
    ValidateEditId { edit: usize },
    ValidateEditMeta { edit: usize, meta: usize },
    ValidateEdit { index: usize },
    StartInitialClone,
    CloneInitialSnapshot,
    TakeInitialSnapshot,
    CloseInitialSnapshotClone,
    CloseInitialSnapshotSource,
    MoveInitialOwner,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    ReturnForward { position: usize, edit: usize, mutation: usize },
    PrepareApplied { position: usize, edit: usize, field: u8 },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    PrepareRedo { position: usize, edit: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct DrawingStoreInitializationAuthority {
    pub actor: protocol::ActorId,
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub arena_bootstrap_job: DrawingMutationArenaBootstrapJob,
    pub envelope: std::mem::ManuallyDrop<Option<store::os_store::store_schema::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>>>,
    pub runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<DrawingSnapshot>>>,
    pub candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<DrawingSnapshot, DrawingMutation>>>,
    pub retire_pending: std::mem::ManuallyDrop<Option<DrawingRetirementOwner>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub owner_catalog: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationOwnerCatalog>>,
    pub owners: std::mem::ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<DrawingSnapshot, DrawingMutation>>>,
    pub forward_source: std::mem::ManuallyDrop<Option<RetainedCloneSource<DrawingMutation>>>,
    pub forward_from_ledger: bool,
    pub snapshot_source: std::mem::ManuallyDrop<Option<RetainedCloneSource<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>>>,
    pub mutation_candidate: std::mem::ManuallyDrop<Option<DrawingMutationCandidateAuthority>>,
    pub prepared_history_id: std::mem::ManuallyDrop<Option<String>>,
    pub publication: semio_framework_job_publication::RetainedJobPublication,
    pub edit_index: store::ArtifactStoreInitializationEditIndex,
    /// 🧬️ The paged clone of `envelope.vcs.genesis.facts().snapshot()` that becomes the runtime's live fold —
    /// the envelope keeps its own initial snapshot, which `print_document_pack` / the cold from-scratch
    /// fold read back (moving it out left every loaded document's `.pack` empty — ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17).
    pub initial_clone: std::mem::ManuallyDrop<Option<DrawingSnapshot>>,
    pub initial_snapshot_clone: std::mem::ManuallyDrop<Option<<DrawingSnapshot as RetainedClone>::Cursor>>,
    pub initial_clone_source: std::mem::ManuallyDrop<Option<RetainedCloneSource<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>>>,
    pub phase: DrawingStoreInitializationPhase,
    pub resume_phase: Option<DrawingStoreInitializationPhase>,
    pub cancel_requested: bool,
    pub fault: Option<Vec<u8>>,
    pub terminal_handoff: bool,
}

impl DrawingStoreInitializationAuthority {
    pub fn new(
        envelope: store::os_store::store_schema::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>,
        owner_catalog: Result<store::ArtifactStoreInitializationOwnerCatalog, &'static str>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Self {
        let bootstrap_job = DrawingMutationArenaBootstrapJob::new(operation, generation);
        let (owner_catalog, arena_bootstrap_job, phase, fault) = match (owner_catalog, bootstrap_job) {
            (Ok(owner_catalog), Ok(arena_bootstrap_job)) => (Some(owner_catalog), arena_bootstrap_job, DrawingStoreInitializationPhase::InitializeArena, None),
            (Err(error), Ok(mut arena_bootstrap_job)) => {
                arena_bootstrap_job.terminal = true;
                (None, arena_bootstrap_job, DrawingStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec()))
            }
            (_, Err(error)) => (None, DrawingMutationArenaBootstrapJob::inactive(operation, generation), DrawingStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec())),
        };
        Self {
            actor,
            operation,
            generation,
            arena_bootstrap_job,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            retire_pending: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            owner_catalog: std::mem::ManuallyDrop::new(owner_catalog),
            owners: std::mem::ManuallyDrop::new(None),
            forward_source: std::mem::ManuallyDrop::new(None),
            forward_from_ledger: false,
            snapshot_source: std::mem::ManuallyDrop::new(None),
            mutation_candidate: std::mem::ManuallyDrop::new(None),
            prepared_history_id: std::mem::ManuallyDrop::new(None),
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            initial_clone: std::mem::ManuallyDrop::new(None),
            initial_snapshot_clone: std::mem::ManuallyDrop::new(None),
            initial_clone_source: std::mem::ManuallyDrop::new(None),
            phase,
            resume_phase: None,
            cancel_requested: false,
            fault,
            terminal_handoff: false,
        }
    }

    pub fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    pub fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    pub fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = DrawingStoreInitializationPhase::RetireFault;
    }

    pub fn seal_forward(&mut self, edit: usize, index: usize, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        cx.consume_fuel(1);
        let funded = grant.maximum_items > 0
            && grant.maximum_depth >= 1
            && grant.maximum_copy_bytes >= RetainedCloneSource::<DrawingMutation>::constructor_copy_bytes()
            && grant.maximum_capacity_bytes >= RetainedCloneSource::<DrawingMutation>::constructor_capacity_bytes::<()>();
        if !funded || self.retire_pending.is_some() {
            return DrawingInitializationTurn::Yield;
        }
        let replacement = {
            let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its forward is sealed");
            let runtime = self.runtime.as_ref().expect("Drawing runtime remains retained while its forward is sealed");
            let entry = envelope.vcs.edits.get(edit).expect("Drawing applied edit remains retained");
            let superseded = runtime.effective_forward(entry, index, &envelope.schema).is_some_and(|effective| effective.superseded());
            if superseded {
                let original = entry.forwards.get(index).expect("Drawing superseded forward remains retained");
                let mutation_id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| Mutation::<DrawingSnapshot>::mutation_id(original)).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                runtime.supersessions().get(&mutation_id).map(|supersession| store::os_store::replay::admit_replacement::<DrawingSnapshot, DrawingMutation>(original, &supersession.replacement, &envelope.schema))
            } else {
                None
            }
        };
        let (owned, from_ledger) = match replacement {
            Some(Ok(Some(replacement))) => (replacement, false),
            Some(_) => {
                self.fail(b"drawing-store.initializer-superseded-forward");
                return DrawingInitializationTurn::Yield;
            }
            None => {
                let slot = self.envelope.as_mut().and_then(|envelope| envelope.vcs.edits.get_mut(edit)).and_then(|entry| entry.forwards.get_mut(index)).expect("Drawing applied forward remains retained");
                (std::mem::replace(slot, drawing_forward_placeholder()), true)
            }
        };
        match RetainedCloneSource::admit_owned(owned, (), grant) {
            Ok((source, progress)) => {
                *self.forward_source = Some(source);
                self.forward_from_ledger = from_ledger;
                if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
            }
            Err((error, owned, ())) => {
                if from_ledger {
                    let slot = self.envelope.as_mut().and_then(|envelope| envelope.vcs.edits.get_mut(edit)).and_then(|entry| entry.forwards.get_mut(index)).expect("Drawing applied forward slot remains retained");
                    drop(std::mem::replace(slot, owned));
                } else {
                    *self.retire_pending = Some(DrawingRetirementOwner::Mutation(owned));
                }
                self.fault = Some(error.into_message().into_bytes());
                self.phase = DrawingStoreInitializationPhase::RetireFault;
            }
        }
        DrawingInitializationTurn::Yield
    }

    pub fn seal_snapshot(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        cx.consume_fuel(1);
        let funded = grant.maximum_items > 0
            && grant.maximum_depth >= 1
            && grant.maximum_copy_bytes >= RetainedCloneSource::<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>::constructor_copy_bytes()
            && grant.maximum_capacity_bytes >= RetainedCloneSource::<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>::constructor_capacity_bytes::<()>();
        if !funded {
            return DrawingInitializationTurn::Yield;
        }
        let alias = self.runtime.as_ref().expect("Drawing runtime remains retained while its snapshot is sealed").share_current();
        match RetainedCloneSource::admit_borrowed(alias, |original: &std::sync::Arc<DrawingSnapshot>| &**original, grant) {
            Ok((source, progress)) => {
                *self.snapshot_source = Some(source);
                if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
            }
            Err((error, _alias)) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
        }
        DrawingInitializationTurn::Yield
    }

    pub fn release_snapshot(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        cx.consume_fuel(1);
        let source = self.snapshot_source.as_mut().expect("sealed Drawing snapshot remains retained until its exact release");
        match source.close_step(grant).and_then(|step| { cx.consume_retained(step.progress())?; Ok(()) }) {
            Ok(()) => { if source.terminal_is_empty() { drop(self.snapshot_source.take()); } }
            Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
        }
        DrawingInitializationTurn::Yield
    }

    pub fn pump_active(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
        let grant = cx.retained_grant();
        let step = if self.active.is_some() {
            store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant)?
        } else if self.retire_pending.is_some() {
            store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.active, grant)?
        } else {
            return Ok(false);
        };
        cx.consume_retained(step.progress())?;
        cx.consume_fuel(1);
        Ok(true)
    }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "Drawing initializer close depth overflow"))?; Ok(demand) };
        let unit = semio_framework_value::RetirementDemand { depth: 1, ..Default::default() };
        if !self.publication.terminal_is_empty() { return nested(self.publication.retirement_demands()?); }
        if let Some(active) = self.active.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?); }
        if self.retire_pending.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.retire_pending)?); }
        if self.prepared_history_id.is_some() { return Ok(unit); }
        if let Some(clone) = self.initial_snapshot_clone.as_ref() {
            return nested(semio_framework_value::RetirementDemand { copy_bytes: clone.next_close_copy_byte_demand()?, capacity_bytes: clone.next_close_capacity_byte_demand(body)?, release_bytes: clone.next_close_release_byte_demand()?, depth: clone.next_close_depth_demand()? });
        }
        if let Some(source) = self.initial_clone_source.as_ref() {
            if self.initial_snapshot_clone.is_none() {
                return nested(semio_framework_value::RetirementDemand { copy_bytes: source.next_close_copy_byte_demand()?, capacity_bytes: source.next_close_capacity_byte_demand(body)?, release_bytes: source.next_close_release_byte_demand()?, depth: source.next_close_depth_demand()? });
            }
        }
        if self.initial_clone.is_some() { return Ok(unit); }
        if let Some(candidate) = self.mutation_candidate.as_ref() { return nested(candidate.close_demands(body)?); }
        if let Some(source) = self.snapshot_source.as_ref() {
            return nested(semio_framework_value::RetirementDemand { copy_bytes: source.next_close_copy_byte_demand()?, capacity_bytes: source.next_close_capacity_byte_demand(body)?, release_bytes: source.next_close_release_byte_demand()?, depth: source.next_close_depth_demand()? });
        }
        if let Some(source) = self.forward_source.as_ref() {
            return nested(semio_framework_value::RetirementDemand { copy_bytes: source.next_close_copy_byte_demand()?, capacity_bytes: source.next_close_capacity_byte_demand(body)?, release_bytes: source.next_close_release_byte_demand()?, depth: source.next_close_depth_demand()? });
        }
        if let Some(runtime) = self.runtime.as_ref() { return nested(runtime.initialization_retirement_demands(body)?); }
        if self.owner_catalog.is_some() { return nested(initialization_catalog_close_demands(&self.owner_catalog)?); }
        if let Some(retirement) = self.envelope_retirement.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body)?); }
        if let Some(envelope) = self.envelope.as_ref() {
            return match self.owners.as_ref() {
                Some(owners) => nested(owners.uninstalled_envelope_retirement_demands(envelope)),
                None => nested(store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<DrawingSnapshot, DrawingMutation>()?),
            };
        }
        if let Some(owners) = self.owners.as_ref() { return nested(owners.uninstalled_owners_demands(body)?); }
        Ok(unit)
    }

    pub fn close_original(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty_inner() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Drawing initializer close exceeds original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if !self.publication.terminal_is_empty() { return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.active.is_some() { return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.retire_pending.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.active, child); }
        if let Some(value) = self.prepared_history_id.take() {
            *self.retire_pending = Some(DrawingRetirementOwner::HistoryId(value));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(clone) = self.initial_snapshot_clone.as_mut() {
            clone.begin_close();
            let step = clone.close_step(child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) && clone.terminal_is_empty() { drop(self.initial_snapshot_clone.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(source) = self.initial_clone_source.as_mut() {
            let step = source.close_step(child)?;
            if source.terminal_is_empty() { drop(self.initial_clone_source.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(value) = self.initial_clone.take() {
            *self.retire_pending = Some(DrawingRetirementOwner::Snapshot(value));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.mutation_candidate.is_some() {
            let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut);
            let candidate = self.mutation_candidate.as_mut().expect("Drawing mutation candidate remains retained during close");
            let step = candidate.close_step(current, child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                if !candidate.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing mutation candidate reported a false terminal")); }
                drop(self.mutation_candidate.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(source) = self.snapshot_source.as_mut() {
            let step = source.close_step(child)?;
            if source.terminal_is_empty() { drop(self.snapshot_source.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(source) = self.forward_source.as_mut() {
            let step = source.close_step(child)?;
            if source.terminal_is_empty() { drop(self.forward_source.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<DrawingSnapshot>> = std::sync::Arc::new(DrawingSnapshotRetirementFactory::default());
            let step = runtime.close_step(&factory, child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                if !runtime.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing initialization runtime reported a false terminal")); }
                drop(self.runtime.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.owner_catalog.is_some() {
            let step = close_initialization_catalog(&mut self.owner_catalog, child)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope_retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.envelope_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            let Some(owners) = self.owners.take() else {
                return match store::os_store::document_store_owners::bounded_artifact_store_owners::<DrawingSnapshot, DrawingMutation>(child) {
                    Ok((owners, progress)) => { *self.owners = Some(owners); Ok(RetainedCloneStep::Progress(progress)) }
                    Err(refused) => { *self.owners = refused.owners; Err(refused.error.with_retained_progress(refused.progress)) }
                };
            };
            let envelope = self.envelope.take().expect("observed original Drawing envelope");
            return match owners.retire_envelope_uninstalled(envelope, child) {
                Ok((owner, progress)) => { *self.envelope_retirement = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, owners, envelope)) => { *self.owners = Some(owners); *self.envelope = Some(envelope); Err(error) }
            };
        }
        if let Some(owners) = self.owners.as_mut() {
            let step = owners.close_uninstalled_owners_step(child)?;
            if owners.uninstalled_owners_terminal_is_empty() { drop(self.owners.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.arena_bootstrap_job.terminal
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.retire_pending.is_none()
            && self.envelope_retirement.is_none()
            && self.owner_catalog.is_none()
            && self.owners.is_none()
            && self.forward_source.is_none()
            && self.snapshot_source.is_none()
            && self.mutation_candidate.is_none()
            && self.prepared_history_id.is_none()
            && self.initial_clone.is_none()
            && self.initial_clone_source.is_none()
            && self.initial_snapshot_clone.is_none()
            && self.publication.terminal_is_empty()
    }
}

/// 🔁️ What one Drawing initializer turn produced before the job lends its outcome.
enum DrawingInitializationTurn {
    Yield,
    Complete,
    Cancelled,
    Fault,
}

impl DrawingStoreInitializationAuthority {
    pub fn turn(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.arena_bootstrap_job.terminal = true;
            self.fail(b"drawing-store.initializer-stale-authority");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, DrawingStoreInitializationPhase::InitializeArena | DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::Cancelled) {
            self.phase = DrawingStoreInitializationPhase::RetireCancelled;
        }
        match self.pump_active(cx) {
            Err(error) => {
                self.fault = Some(error.into_message().into_bytes());
                self.phase = DrawingStoreInitializationPhase::RetireFault;
            }
            Ok(true) => return DrawingInitializationTurn::Yield,
            Ok(false) => {}
        }
        if !matches!(self.phase, DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::RetireFault | DrawingStoreInitializationPhase::Cancelled | DrawingStoreInitializationPhase::Fault | DrawingStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(cx.retained_grant()) {
                    Ok(RetainedCloneStep::Complete(_)) => {}
                    Ok(RetainedCloneStep::Progress(progress)) => {
                        if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                        cx.consume_fuel(1);
                        return DrawingInitializationTurn::Yield;
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
            }
        }
        match self.phase {
            DrawingStoreInitializationPhase::InitializeArena => {
                match self.arena_bootstrap_job.step(cx) {
                    DrawingMutationArenaBootstrapStep::Ready => self.phase = DrawingStoreInitializationPhase::AdmitOwners,
                    DrawingMutationArenaBootstrapStep::Pending { .. } | DrawingMutationArenaBootstrapStep::Blocked => {}
                    DrawingMutationArenaBootstrapStep::Cancelled => self.phase = DrawingStoreInitializationPhase::RetireCancelled,
                    DrawingMutationArenaBootstrapStep::Fault(error) => self.fail(error.as_bytes()),
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::AdmitOwners => {
                let grant = cx.retained_grant();
                let funded = store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<DrawingSnapshot, DrawingMutation>().is_ok_and(|demand| grant.maximum_items > 0 && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_depth >= demand.depth);
                if funded {
                    match store::os_store::document_store_owners::bounded_artifact_store_owners::<DrawingSnapshot, DrawingMutation>(grant) {
                        Ok((owners, progress)) => {
                            *self.owners = Some(owners);
                            match cx.consume_retained(progress) {
                                Ok(()) => self.phase = DrawingStoreInitializationPhase::ValidateEnvelope,
                                Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                            }
                        }
                        Err(refused) => {
                            *self.owners = refused.owners;
                            let _ = cx.consume_retained(refused.progress);
                            self.fault = Some(refused.error.into_message().into_bytes());
                            self.phase = DrawingStoreInitializationPhase::RetireFault;
                        }
                    }
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                let owner_catalog = self.owner_catalog.take().expect("pre-admitted Drawing owner catalog");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new_with_owner_catalog(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), self.actor.clone(), owner_catalog));
                self.phase = DrawingStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"drawing-store.initializer-envelope-missing");
                    return DrawingInitializationTurn::Yield;
                };
                if envelope.schema != semio_s_artifact_draw_drawing_model::DRAWING_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > DRAWING_OWNED_FIELD_BYTES {
                    self.fail(b"drawing-store.initializer-envelope-invalid");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditId { edit: 0 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditId { edit } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEdit { index: 0 };
                    return DrawingInitializationTurn::Yield;
                };
                if entry.id.is_empty() || entry.id.len() > DRAWING_OWNED_FIELD_BYTES || entry.actor.as_ref().is_some_and(|actor| actor.len() > DRAWING_OWNED_FIELD_BYTES) || entry.started_at.len() > DRAWING_OWNED_FIELD_BYTES {
                    self.fail(b"drawing-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta: 0 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                let entry = envelope.vcs.edits.get(edit).expect("Drawing edit remains retained during metadata validation");
                let Some(value) = entry.mutation_meta.get(meta) else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditId { edit: edit + 1 };
                    return DrawingInitializationTurn::Yield;
                };
                if value.mutation_id.as_ref().is_some_and(|id| id.0.len() > DRAWING_OWNED_FIELD_BYTES) {
                    self.fail(b"drawing-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta: meta + 1 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, usize::MAX) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = DrawingStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = DrawingStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"drawing-store.initializer-duplicate-edit"),
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::StartInitialClone => {
                let grant = cx.retained_grant();
                let funded = grant.maximum_items > 0
                    && grant.maximum_depth >= 1
                    && grant.maximum_copy_bytes >= RetainedCloneSource::<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>::constructor_copy_bytes()
                    && grant.maximum_capacity_bytes >= RetainedCloneSource::<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>::constructor_capacity_bytes::<()>();
                if funded {
                    let genesis = self.envelope.as_ref().expect("retained Drawing clone genesis").vcs.genesis.facts().share_snapshot();
                    match RetainedCloneSource::admit_borrowed(genesis, |original: &std::sync::Arc<DrawingSnapshot>| &**original, grant) {
                        Ok((source, progress)) => {
                            *self.initial_clone_source = Some(source);
                            *self.initial_snapshot_clone = Some(DrawingSnapshot::retained_clone_cursor());
                            match cx.consume_retained(progress) {
                                Ok(()) => self.phase = DrawingStoreInitializationPhase::CloneInitialSnapshot,
                                Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                            }
                        }
                        Err((error, _genesis)) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                    }
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CloneInitialSnapshot => {
                let grant = cx.retained_grant();
                let source = self.initial_clone_source.as_ref().expect("retained Drawing snapshot clone source");
                let clone = self.initial_snapshot_clone.as_mut().expect("retained native Drawing snapshot clone");
                match clone.advance(source.borrow(), grant).and_then(|step| { cx.consume_retained(step.progress())?; Ok(step) }) {
                    Ok(step) => {
                        cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                        if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = DrawingStoreInitializationPhase::TakeInitialSnapshot; }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes());self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::TakeInitialSnapshot => {
                let clone = self.initial_snapshot_clone.as_mut().expect("completed Drawing snapshot clone");
                *self.initial_clone = clone.take();
                clone.begin_close();
                if self.initial_clone.is_some() { self.phase = DrawingStoreInitializationPhase::CloseInitialSnapshotClone; }
                else { self.fail(b"drawing-store.initializer-clone-false-terminal"); }
                cx.consume_fuel(size_of::<DrawingSnapshot>().max(1) as u64);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CloseInitialSnapshotClone => {
                let grant = cx.retained_grant();
                let clone = self.initial_snapshot_clone.as_mut().expect("closing native Drawing snapshot clone");
                match clone.close_step(grant).and_then(|step| { cx.consume_retained(step.progress())?; Ok(step) }) {
                    Ok(step) => {
                        cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                        if matches!(step, RetainedCloneStep::Complete(_)) && clone.terminal_is_empty() { drop(self.initial_snapshot_clone.take());self.phase = DrawingStoreInitializationPhase::CloseInitialSnapshotSource; }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes());self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CloseInitialSnapshotSource => {
                let grant = cx.retained_grant();
                let source = self.initial_clone_source.as_mut().expect("closing Drawing snapshot clone source");
                match source.close_step(grant).and_then(|step| { cx.consume_retained(step.progress())?; Ok(step) }) {
                    Ok(step) => {
                        cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                        if source.terminal_is_empty() { drop(self.initial_clone_source.take()); self.phase = DrawingStoreInitializationPhase::MoveInitialOwner; }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes());self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::MoveInitialOwner => {
                let _envelope = self.envelope.as_ref().expect("Drawing envelope remains retained during initial owner move");
                let initial = self.initial_clone.take().expect("Drawing initial clone remains retained until the runtime adopts it");
                match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(DrawingSnapshotRetirementFactory::default())) {
                    Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                    Err(initial) => {
                        *self.retire_pending = Some(DrawingRetirementOwner::Snapshot(initial));
                        self.fail(b"initializer-owned-workspace-adoption");
                    }
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawingStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return DrawingInitializationTurn::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = DrawingStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = DrawingStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = DrawingStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Drawing runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = DrawingStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = DrawingStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = DrawingStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Drawing runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = DrawingStoreInitializationPhase::FindRedo { position: 0 };
                    return DrawingInitializationTurn::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"drawing-store.initializer-applied-edit-missing");
                    return DrawingInitializationTurn::Yield;
                };
                if edit.id == id {
                    self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"drawing-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                if self.forward_source.is_none() {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.snapshot_source.is_none() && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    
                    self.phase = DrawingStoreInitializationPhase::StartInitialClone;
                    cx.consume_fuel(1);
                    return DrawingInitializationTurn::Yield;
                }
                if self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).is_none_or(|entry| mutation >= entry.forwards.len()) {
                    self.phase = DrawingStoreInitializationPhase::PrepareApplied { position, edit, field: 0 };
                    return DrawingInitializationTurn::Yield;
                }
                let withdrawn = {
                    let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its forwards fold");
                    envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).is_none_or(|effective| effective.operation().is_none())
                };
                if withdrawn {
                    self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    cx.consume_fuel(1);
                    return DrawingInitializationTurn::Yield;
                }
                if self.mutation_candidate.is_none() {
                    match DrawingMutationCandidateAuthority::try_new(self.operation, self.generation) {
                        Ok(candidate) => *self.mutation_candidate = Some(candidate),
                        Err(DrawingMutationArenaBorrowError::NotReady | DrawingMutationArenaBorrowError::Contended) => return DrawingInitializationTurn::Yield,
                        Err(error) => {
                            self.fail(error.as_str().as_bytes());
                            return DrawingInitializationTurn::Yield;
                        }
                    }
                    cx.consume_fuel(1);
                    return DrawingInitializationTurn::Yield;
                }
                return self.seal_forward(edit, mutation, cx);
                }
                let need_shared = {
                    let forward = self.forward_source.as_ref().expect("sealed Drawing forward remains retained").borrow();
                    self.mutation_candidate.as_ref().expect("Drawing mutation candidate remains retained").needs_shared_snapshot(forward.get())
                };
                if need_shared && self.snapshot_source.is_none() {
                    return self.seal_snapshot(cx);
                }
                if !need_shared && self.snapshot_source.is_some() {
                    return self.release_snapshot(cx);
                }
                let forward = self.forward_source.as_ref().expect("sealed Drawing forward remains retained").borrow();
                let runtime = self.runtime.as_mut().expect("retained initializer runtime");
                let candidate = self.mutation_candidate.as_mut().expect("Drawing mutation candidate remains retained");
                let stepped = match self.snapshot_source.as_ref() {
                    Some(sealed) => candidate.step(DrawingSourceAccess::Shared(runtime.current_ref(), sealed.borrow()), forward, cx),
                    None => candidate.step(DrawingSourceAccess::Exclusive(runtime.current_mut().expect("Drawing runtime current snapshot remains retained")), forward, cx),
                };
                let candidate_complete = match stepped {
                    Ok(complete) => complete,
                    Err(error) => {
                        self.fail(error.as_bytes());
                        return DrawingInitializationTurn::Yield;
                    }
                };
                if candidate_complete {
                    self.mutation_candidate.as_mut().expect("Drawing completed mutation candidate remains retained").take().expect("Drawing mutation overlay terminal commit witness remains exact");
                    drop(self.mutation_candidate.take());
                    self.phase = DrawingStoreInitializationPhase::ReturnForward { position, edit, mutation };
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ReturnForward { position, edit, mutation } => {
                let grant = cx.retained_grant();
                let source = self.forward_source.as_mut().expect("sealed Drawing forward remains retained until its exact return");
                match source.take_authority(grant) {
                    Ok(RetainedCloneSourceTake::Pending(progress)) => {
                        if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                    }
                    Ok(RetainedCloneSourceTake::Ready(original, progress)) => {
                        if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                        drop(self.forward_source.take());
                        if self.forward_from_ledger {
                            let slot = self.envelope.as_mut().and_then(|envelope| envelope.vcs.edits.get_mut(edit)).and_then(|entry| entry.forwards.get_mut(mutation)).expect("Drawing ledger forward slot remains retained");
                            drop(std::mem::replace(slot, original));
                        } else {
                            *self.retire_pending = Some(DrawingRetirementOwner::Mutation(original));
                        }
                        if matches!(self.phase, DrawingStoreInitializationPhase::ReturnForward { .. }) {
                            self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::PrepareApplied { position, edit, field } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                match field {
                    0 => {
                        *self.prepared_history_id = Some(clone_drawing_string(&entry.id).expect("validated Drawing applied id remains admitted"));
                        self.phase = DrawingStoreInitializationPhase::CommitApplied { position, edit };
                        cx.consume_fuel(entry.id.len().max(1) as u64);
                    }
                    _ => self.fail(b"drawing-store.initializer-applied-preparation"),
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CommitApplied { position, edit } => {
                let id = self.prepared_history_id.take().expect("Drawing applied id was retained in its own preparation grant");
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained");
                if let Err(error) = runtime.push_applied_admitted(id, entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = DrawingStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = DrawingStoreInitializationPhase::BuildCandidate;
                    return DrawingInitializationTurn::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"drawing-store.initializer-redo-edit-missing");
                    return DrawingInitializationTurn::Yield;
                };
                if edit.id == id {
                    self.phase = DrawingStoreInitializationPhase::PrepareRedo { position, edit: scan };
                } else {
                    self.fail(b"drawing-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::PrepareRedo { position, edit } => {
                let id = &self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained").id;
                *self.prepared_history_id = Some(clone_drawing_string(id).expect("validated Drawing redo id remains admitted"));
                self.phase = DrawingStoreInitializationPhase::CommitRedo { position, edit };
                cx.consume_fuel(id.len().max(1) as u64);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.prepared_history_id.take().expect("Drawing redo id was retained in its own preparation grant");
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Drawing runtime remains retained").push_redo_admitted(id, entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = DrawingStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"drawing-store.initializer-generation-exhausted");
                    return DrawingInitializationTurn::Yield;
                };
                let envelope = self.envelope.take().expect("Drawing envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Drawing runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, self.owners.take().expect("Drawing document store owners remain admitted until atomic store construction"));
                *self.candidate = Some(candidate);
                self.phase = DrawingStoreInitializationPhase::Complete;
                DrawingInitializationTurn::Complete
            }
            DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::RetireFault => match self.close_original(cx.retained_grant()).and_then(|step| { cx.consume_retained(step.progress())?; cx.consume_fuel(1); Ok(matches!(step, RetainedCloneStep::Complete(_))) }) {
                Ok(false) => DrawingInitializationTurn::Yield,
                Ok(true) => {
                    self.terminal_handoff = true;
                    if self.phase == DrawingStoreInitializationPhase::RetireCancelled {
                        self.phase = DrawingStoreInitializationPhase::Cancelled;
                        DrawingInitializationTurn::Cancelled
                    } else {
                        self.phase = DrawingStoreInitializationPhase::Fault;
                        DrawingInitializationTurn::Fault
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    DrawingInitializationTurn::Yield
                }
            },
            DrawingStoreInitializationPhase::Complete => DrawingInitializationTurn::Complete,
            DrawingStoreInitializationPhase::Cancelled => DrawingInitializationTurn::Cancelled,
            DrawingStoreInitializationPhase::Fault => DrawingInitializationTurn::Fault,
        }
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority {
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.close_demands(maximum_copy_bytes)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.turn(cx) {
            DrawingInitializationTurn::Yield => semio_framework_job::JobOutcomeBorrow::admit_yield(cx),
            DrawingInitializationTurn::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            DrawingInitializationTurn::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            DrawingInitializationTurn::Fault => {
                let detail = self.fault.as_deref().unwrap_or(b"drawing-store.initializer-fault");
                self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, detail, cx)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Drawing initializer lends only yield and terminal outcomes")),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, DrawingStoreInitializationPhase::Cancelled | DrawingStoreInitializationPhase::Fault) {
            self.phase = DrawingStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        self.begin_close();
        let step = self.close_original(grant)?;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            drop(self.mutation_candidate.take());
            self.terminal_handoff = true;
        }
        Ok(step)
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<DrawingSnapshot, DrawingMutation>> {
        if self.phase != DrawingStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        drop(self.mutation_candidate.take());
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for DrawingStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner() || std::thread::panicking(), "Drawing store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn drawing_document_store_initialization_job(
    envelope: store::os_store::store_schema::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<DrawingSnapshot, DrawingMutation> {
    let owner_catalog = store::ArtifactStoreInitializationOwnerCatalog::try_new();
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(DrawingStoreInitializationAuthority::new(envelope, owner_catalog, operation, generation, actor)))
}
//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️RetainedMutationAuthorityTests
#[cfg(test)]
#[path = "🧪️tests/🔬️retained-mutation-authority/🦀️.rs"]
mod retained_mutation_authority_tests;
//#endregion 🧪️RetainedMutationAuthorityTests

/// 🧷️ Allocation-free stand-in kept in the history ledger while its original forward is in sealed custody.
fn drawing_forward_placeholder() -> DrawingMutation {
    DrawingMutation::DeleteLayer(semio_s_artifact_draw_drawing_structure::delete_layer::mutation::DeleteLayer { layer_id: Default::default() })
}

