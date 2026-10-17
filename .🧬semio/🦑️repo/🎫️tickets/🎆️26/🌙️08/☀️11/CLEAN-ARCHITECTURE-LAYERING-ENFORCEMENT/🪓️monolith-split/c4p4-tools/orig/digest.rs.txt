//! 🔏️ Drawing host digest: the semantic digest authorities of fills, strokes, path segments, layers and mutations, the mutation target owner and the duplicate reference rewrite.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use protocol::{Mutation, OpBinary};
use store::ErasedSnapshotRetirement as _;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep};
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeLayers;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_ITEMS;
use semio_s_artifact_draw_drawing_host_arena::DrawingRebuiltLayerOwners;
use semio_s_artifact_draw_drawing_host_arena::DrawingFixedOwnerCensus;
use semio_s_artifact_draw_drawing_host_arena::native_text_footprint;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_RETAINED_PAGE_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_OWNED_FIELD_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeText;
use semio_s_artifact_draw_drawing_model::FillStyle;
use semio_s_artifact_draw_drawing_model::StrokeStyle;
use semio_s_artifact_draw_drawing_model::PathSegment;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_LAYER_DEPTH;
use semio_s_artifact_draw_drawing_host_arena::DrawingTraversalFrame;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotBoundsAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotOwnerTotals;
use semio_s_artifact_draw_drawing_host_arena::DrawingDecodedFieldRetirement;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerLocator;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneAuthority;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_host_arena::initial_snapshot_clone_grant;
const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY: usize = DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY * 4 + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingContainerRebuildMove {
    Empty,
    SourceToReverse,
    PendingToOutput,
    ReverseToOutput,
    ReverseToRemoved,
    OutputToReverse,
    ReverseToSource,
}

pub struct DrawingContainerRebuildAuthority {
    pub source: std::mem::ManuallyDrop<Option<DrawingNativeLayers>>,
    pub reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub pending: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    pub removed: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    pub moves: [DrawingContainerRebuildMove; DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY],
    pub move_count: usize,
    pub rollback_cursor: Option<usize>,
    pub remove_index: Option<usize>,
    pub insert_index: Option<usize>,
    pub original_index: usize,
    pub phase: u8,
    pub terminal: bool,
}

struct DrawingContainerRebuildRejected {
    pub source: DrawingNativeLayers,
    pub pending: Option<DrawingLayerNode>,
    pub reverse: Vec<DrawingLayerNode>,
    pub output: Vec<DrawingLayerNode>,
}

impl DrawingContainerRebuildAuthority {
    pub fn new(
        source: DrawingNativeLayers,
        remove_index: Option<usize>,
        insert_index: Option<usize>,
        pending: Option<DrawingLayerNode>,
        reverse: Vec<DrawingLayerNode>,
        output: Vec<DrawingLayerNode>,
        workset: DrawingMutationWorksetPlan,
    ) -> Result<Self, DrawingContainerRebuildRejected> {
        let extra = usize::from(pending.is_some());
        let Some(output_capacity) = source.len().saturating_sub(usize::from(remove_index.is_some())).checked_add(extra) else {
            return Err(DrawingContainerRebuildRejected { source, pending, reverse, output });
        };
        if output_capacity > DRAWING_MAXIMUM_NESTED_ITEMS
            || source.len().saturating_add(output_capacity) > workset.container_slots
            || source.len() > workset.maximum_container.saturating_add(1)
            || output_capacity > workset.maximum_container.saturating_add(1)
            || source.capacity() < output_capacity
            || reverse.capacity() < source.len().max(output_capacity)
            || output.capacity() < output_capacity
            || !reverse.is_empty()
            || !output.is_empty()
        {
            return Err(DrawingContainerRebuildRejected { source, pending, reverse, output });
        }
        Ok(Self {
            source: std::mem::ManuallyDrop::new(Some(source)),
            reverse: std::mem::ManuallyDrop::new(Some(reverse)),
            output: std::mem::ManuallyDrop::new(Some(output)),
            pending: std::mem::ManuallyDrop::new(pending),
            removed: std::mem::ManuallyDrop::new(None),
            moves: [DrawingContainerRebuildMove::Empty; DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY],
            move_count: 0,
            rollback_cursor: None,
            remove_index,
            insert_index,
            original_index: 0,
            phase: 0,
            terminal: false,
        })
    }

    pub fn reserve_move(&self) -> Result<(), &'static str> {
        self.moves.get(self.move_count).map(|_| ()).ok_or("drawing-store.container-move-capacity")
    }

    pub fn record_reserved_move(&mut self, value: DrawingContainerRebuildMove) {
        self.moves[self.move_count] = value;
        self.move_count += 1;
    }

    pub fn advance(&mut self) -> Result<(bool, u64), &'static str> {
        if self.terminal {
            return Ok((true, 0));
        }
        if self.rollback_cursor.is_some() {
            return Err("drawing-store.container-advance-after-rollback");
        }
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("drawing-store.container-owner-missing");
        }
        if self.phase == 0 {
            if !self.source.as_ref().expect("validated Drawing source remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.source.as_mut().expect("validated Drawing source remains retained").pop().expect("nonempty Drawing source yields one owner");
                self.reverse.as_mut().expect("validated Drawing reverse remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::SourceToReverse);
                return Ok((false, 1));
            }
            self.phase = 1;
            return Ok((false, 0));
        }
        if self.phase == 1 {
            if self.pending.is_some() && self.insert_index.is_some_and(|index| index.min(self.reverse.as_ref().map_or(0, Vec::len) + self.original_index) == self.output.as_ref().map_or(0, Vec::len)) {
                self.reserve_move()?;
                self.output.as_mut().expect("validated Drawing output remains retained").push(self.pending.take().expect("Drawing insertion owner remains retained"));
                self.record_reserved_move(DrawingContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            if !self.reverse.as_ref().expect("validated Drawing reverse remains retained").is_empty() {
                if self.remove_index == Some(self.original_index) && self.removed.is_some() {
                    return Err("drawing-store.container-duplicate-removal");
                }
                self.reserve_move()?;
                let value = self.reverse.as_mut().expect("validated Drawing reverse remains retained").pop().expect("nonempty Drawing reverse yields one owner");
                if self.remove_index == Some(self.original_index) {
                    *self.removed = Some(value);
                    self.record_reserved_move(DrawingContainerRebuildMove::ReverseToRemoved);
                } else {
                    self.output.as_mut().expect("validated Drawing output remains retained").push(value);
                    self.record_reserved_move(DrawingContainerRebuildMove::ReverseToOutput);
                }
                self.original_index += 1;
                return Ok((false, 1));
            }
            if self.pending.is_some() {
                self.reserve_move()?;
                let value = self.pending.take().expect("validated Drawing pending owner remains retained");
                self.output.as_mut().expect("validated Drawing output remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            self.phase = 2;
            return Ok((false, 0));
        }
        if self.phase == 2 {
            if !self.output.as_ref().expect("validated Drawing output remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.output.as_mut().expect("validated Drawing output remains retained").pop().expect("nonempty Drawing output yields one owner");
                self.reverse.as_mut().expect("validated Drawing reverse remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::OutputToReverse);
                return Ok((false, 1));
            }
            self.phase = 3;
            return Ok((false, 0));
        }
        if !self.reverse.as_ref().expect("validated Drawing reverse remains retained").is_empty() {
            self.reserve_move()?;
            let value = self.reverse.as_mut().expect("validated Drawing reverse remains retained").pop().expect("nonempty Drawing reverse yields one owner");
            self.source.as_mut().expect("validated Drawing source remains retained").push_reserved(value).unwrap_or_else(|_| panic!("admitted native Drawing source slot remains reserved"));
            self.record_reserved_move(DrawingContainerRebuildMove::ReverseToSource);
            return Ok((false, 1));
        }
        self.terminal = true;
        Ok((true, 0))
    }

    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let (complete, fuel) = self.advance()?;
        if fuel > 0 {
            cx.consume_fuel(fuel);
        }
        Ok(complete)
    }

    pub fn close_forward_step(&mut self) -> Result<bool, &'static str> {
        self.advance().map(|(complete, _)| complete)
    }

    pub fn take(&mut self) -> Option<DrawingRebuiltLayerOwners> {
        self.terminal.then(|| {
            (
                self.source.take().expect("Drawing rebuilt source container remains retained"),
                self.removed.take(),
                self.reverse.take().expect("Drawing emptied reverse arena remains retained"),
                self.output.take().expect("Drawing emptied output arena remains retained"),
            )
        })
    }

    pub fn rollback_step(&mut self) -> Result<bool, &'static str> {
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("drawing-store.container-rollback-owner-missing");
        }
        let cursor = *self.rollback_cursor.get_or_insert(self.move_count);
        if cursor == 0 {
            self.phase = 0;
            self.original_index = 0;
            return Ok(true);
        }
        let index = cursor - 1;
        match self.moves[index] {
            DrawingContainerRebuildMove::SourceToReverse => {
                let value = self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.pop().ok_or("drawing-store.container-rollback-reverse")?;
                self.source.as_mut().ok_or("drawing-store.container-source")?.push_reserved(value).unwrap_or_else(|_| panic!("native Drawing rollback source slot remains reserved"));
            }
            DrawingContainerRebuildMove::PendingToOutput => {
                if self.pending.is_some() {
                    return Err("drawing-store.container-rollback-pending");
                }
                let value = self.output.as_mut().ok_or("drawing-store.container-output")?.pop().ok_or("drawing-store.container-rollback-output")?;
                *self.pending = Some(value);
            }
            DrawingContainerRebuildMove::ReverseToOutput => {
                let value = self.output.as_mut().ok_or("drawing-store.container-output")?.pop().ok_or("drawing-store.container-rollback-output")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::ReverseToRemoved => {
                let value = self.removed.take().ok_or("drawing-store.container-rollback-removed")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::OutputToReverse => {
                let value = self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.pop().ok_or("drawing-store.container-rollback-reverse")?;
                self.output.as_mut().ok_or("drawing-store.container-output")?.push(value);
            }
            DrawingContainerRebuildMove::ReverseToSource => {
                let value = self.source.as_mut().ok_or("drawing-store.container-source")?.pop().ok_or("drawing-store.container-rollback-source")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::Empty => return Err("drawing-store.container-rollback-empty-move"),
        }
        self.moves[index] = DrawingContainerRebuildMove::Empty;
        self.rollback_cursor = Some(index);
        Ok(false)
    }

    pub fn rollback_complete(&self) -> bool {
        self.rollback_cursor == Some(0)
    }

    #[cfg(test)]
    pub fn recorded_move_in_phase(&self, phase: u8) -> bool {
        let Some(index) = self.move_count.checked_sub(1) else { return false };
        self.phase == phase
            && match phase {
                0 => self.moves[index] == DrawingContainerRebuildMove::SourceToReverse,
                1 => matches!(self.moves[index], DrawingContainerRebuildMove::PendingToOutput | DrawingContainerRebuildMove::ReverseToOutput | DrawingContainerRebuildMove::ReverseToRemoved),
                2 => self.moves[index] == DrawingContainerRebuildMove::OutputToReverse,
                3 => self.moves[index] == DrawingContainerRebuildMove::ReverseToSource,
                _ => false,
            }
    }

    pub fn finish_handoff(&mut self) -> Result<(), &'static str> {
        if !(self.terminal || self.rollback_complete()) || self.source.is_some() || self.reverse.is_some() || self.output.is_some() || self.pending.is_some() || self.removed.is_some() {
            return Err("drawing-store.container-rollback-handoff-incomplete");
        }
        self.terminal = true;
        Ok(())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.source.is_none() && self.reverse.is_none() && self.output.is_none() && self.pending.is_none() && self.removed.is_none()
    }
}

impl Drop for DrawingContainerRebuildAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing container rebuild reached Drop before exact handoff or cursor retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingSemanticDigestTotals {
    pub semantic_items: usize,
    pub semantic_bytes: usize,
    pub maximum_field_bytes: usize,
    pub source_owner_items: usize,
    pub source_owner_bytes: usize,
    pub derived_owner_items: usize,
    pub derived_owner_bytes: usize,
}

struct DrawingSemanticDigestCredit {
    pub items: usize,
    pub bytes: usize,
    pub maximum_field_bytes: usize,
    pub source_owner_items: usize,
    pub source_owner_bytes: usize,
    pub derived_owner_items: usize,
    pub derived_owner_bytes: usize,
    pub owner_census: DrawingFixedOwnerCensus,
    pub semantic: Option<semio_framework_hash::Sha256>,
    pub text: Option<semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingTextDigestCursor>,
}

impl Default for DrawingSemanticDigestCredit {
    fn default() -> Self {
        Self {
            items: 0,
            bytes: 0,
            maximum_field_bytes: 0,
            source_owner_items: 1,
            source_owner_bytes: size_of::<DrawingMutation>(),
            derived_owner_items: 0,
            derived_owner_bytes: 0,
            owner_census: DrawingFixedOwnerCensus::new(),
            semantic: Some(semio_framework_hash::Sha256::new()),
            text: None,
        }
    }
}

impl DrawingSemanticDigestCredit {
    pub fn add_source_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(items, bytes, 0, 0)?;
        self.source_owner_items = self.source_owner_items.checked_add(items).ok_or("drawing-store.mutation-source-owner-item-overflow")?;
        self.source_owner_bytes = self.source_owner_bytes.checked_add(bytes).ok_or("drawing-store.mutation-source-owner-byte-overflow")?;
        if self.source_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-source-owner-item-capacity");
        }
        if self.source_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-source-owner-byte-capacity");
        }
        Ok(())
    }

    pub fn add_derived_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(0, 0, items, bytes)?;
        self.derived_owner_items = self.derived_owner_items.checked_add(items).ok_or("drawing-store.mutation-derived-owner-item-overflow")?;
        self.derived_owner_bytes = self.derived_owner_bytes.checked_add(bytes).ok_or("drawing-store.mutation-derived-owner-byte-overflow")?;
        if self.derived_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-derived-owner-item-capacity");
        }
        if self.derived_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-derived-owner-byte-capacity");
        }
        Ok(())
    }

    pub fn source_vec<T, const N: usize>(&mut self, value: &semio_framework_value::list::PagedList<T, N>) -> Result<(), &'static str> {
        let items = 1usize.checked_add(value.capacity()).ok_or("drawing-store.mutation-source-owner-item-overflow")?;
        let bytes = value.allocated_bytes().checked_add(size_of_val(value)).ok_or("drawing-store.mutation-source-owner-byte-overflow")?;
        self.add_source_owner(items, bytes)
    }

    pub fn derived_vec<T, const N: usize>(&mut self, value: &semio_framework_value::list::PagedList<T, N>) -> Result<(), &'static str> {
        let bytes = value.len().checked_mul(size_of::<T>()).ok_or("drawing-store.mutation-derived-owner-byte-overflow")?;
        let pages = bytes.checked_add(DRAWING_MUTATION_RETAINED_PAGE_BYTES - 1).ok_or("drawing-store.mutation-derived-owner-byte-overflow")? / DRAWING_MUTATION_RETAINED_PAGE_BYTES;
        self.add_derived_owner(pages.max(1), 0)
    }

    pub fn observe_owned_string(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: &DrawingNativeText, cloned: bool, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let cursor = self.text.get_or_insert_with(semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingTextDigestCursor::default);
        let semantic = self.semantic.as_mut().ok_or("drawing-store.mutation-digest-sealed")?;
        let step = cursor.step(value, tag, 1, DRAWING_OWNED_FIELD_BYTES, |bytes| { semantic.update(bytes); digest.observe(bytes); })?;
        cx.consume_fuel(1);
        if !step.complete { return Ok(false); }
        let totals = cursor.totals().ok_or("drawing-store.text-digest-false-terminal")?;
        cursor.close_step(1);
        self.text = None;
        self.add_source_owner(1usize.checked_add(totals.chunks).ok_or("drawing-store.mutation-source-owner-item-overflow")?, size_of_val(value).checked_add(totals.backing_bytes).ok_or("drawing-store.mutation-source-owner-byte-overflow")?)?;
        if cloned { self.add_derived_owner(totals.chunks.max(1), 0)?; }
        self.maximum_field_bytes = self.maximum_field_bytes.max(value.len());
        self.items = self.items.checked_add(1).ok_or("drawing-store.mutation-item-overflow")?;
        self.bytes = self.bytes.checked_add(11).and_then(|bytes| bytes.checked_add(value.len())).ok_or("drawing-store.mutation-byte-overflow")?;
        Ok(true)
    }

    pub fn observe(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        if value.len() > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.mutation-field-capacity");
        }
        self.maximum_field_bytes = self.maximum_field_bytes.max(value.len());
        self.items = self.items.checked_add(1).ok_or("drawing-store.mutation-item-overflow")?;
        self.bytes = self.bytes.checked_add(11).and_then(|bytes| bytes.checked_add(value.len())).ok_or("drawing-store.mutation-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-byte-capacity");
        }
        let prefix = [0xd8];
        let tag = tag.to_be_bytes();
        let length = (value.len() as u64).to_be_bytes();
        let semantic = self.semantic.as_mut().ok_or("drawing-store.mutation-digest-sealed")?;
        semantic.update(&prefix);
        semantic.update(&tag);
        semantic.update(&length);
        semantic.update(value);
        digest.observe(&prefix);
        digest.observe(&tag);
        digest.observe(&length);
        digest.observe(value);
        cx.consume_fuel(1);
        Ok(())
    }

    pub fn scalar_f64(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: f64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, tag, &value.to_bits().to_be_bytes(), cx)
    }

    pub fn scalar_usize(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, tag, &(value as u64).to_be_bytes(), cx)
    }

    pub fn seal(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, 99, &[], cx)?;
        let semantic = self.semantic.take().ok_or("drawing-store.mutation-digest-sealed")?.finalize();
        digest.observe(b"drawing.semantic.sha256");
        digest.observe(&semantic);
        Ok(())
    }

    pub fn totals(&self) -> Option<DrawingSemanticDigestTotals> {
        self.semantic.is_none().then_some(DrawingSemanticDigestTotals {
            semantic_items: self.items,
            semantic_bytes: self.bytes,
            maximum_field_bytes: self.maximum_field_bytes,
            source_owner_items: self.source_owner_items,
            source_owner_bytes: self.source_owner_bytes,
            derived_owner_items: self.derived_owner_items,
            derived_owner_bytes: self.derived_owner_bytes,
        })
    }
}

struct DrawingFillDigestAuthority {
    pub phase: u8,
    pub index: usize,
    pub field: u8,
    pub terminal: bool,
}

impl DrawingFillDigestAuthority {
    pub fn new() -> Self {
        Self { phase: 0, index: 0, field: 0, terminal: false }
    }

    pub fn step(&mut self, value: Option<&FillStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            credit.observe(digest, 200, &[u8::from(value.is_some())], cx)?;
            self.phase = 1;
            if value.is_none() {
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        let value = value.ok_or("drawing-store.digest-fill-missing")?;
        if self.phase == 1 {
            let variant = match value {
                FillStyle::Solid { .. } => 1,
                FillStyle::LinearGradient { .. } => 2,
                FillStyle::RadialGradient { .. } => 3,
            };
            credit.observe(digest, 201, &[variant], cx)?;
            self.phase = 2;
            return Ok(false);
        }
        match value {
            FillStyle::Solid { color } => {
                let Some(value) = color.get((self.phase - 2) as usize) else {
                    self.terminal = true;
                    return Ok(true);
                };
                credit.scalar_f64(digest, 202 + u16::from(self.phase - 2), *value, cx)?;
                self.phase += 1;
                self.terminal = self.phase == 6;
            }
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                if self.phase <= 5 {
                    let fields = [*x1, *y1, *x2, *y2];
                    credit.scalar_f64(digest, 210 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                } else if self.phase == 6 {
                    credit.source_vec(stops)?;
                    credit.derived_vec(stops)?;
                    credit.scalar_usize(digest, 214, stops.len(), cx)?;
                    self.phase = 7;
                    self.terminal = stops.is_empty();
                } else {
                    let stop = stops.get(self.index).ok_or("drawing-store.digest-linear-stop")?;
                    if self.field == 0 {
                        credit.scalar_f64(digest, 215, stop.offset, cx)?;
                    } else {
                        credit.scalar_f64(digest, 215 + u16::from(self.field), stop.color[(self.field - 1) as usize], cx)?;
                    }
                    self.field += 1;
                    if self.field == 5 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == stops.len();
                    }
                }
            }
            FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops } => {
                if self.phase <= 4 {
                    let fields = [*center_x, *center_y, *r];
                    credit.scalar_f64(digest, 220 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                } else if self.phase == 5 {
                    credit.source_vec(stops)?;
                    credit.derived_vec(stops)?;
                    credit.scalar_usize(digest, 223, stops.len(), cx)?;
                    self.phase = 6;
                    self.terminal = stops.is_empty();
                } else {
                    let stop = stops.get(self.index).ok_or("drawing-store.digest-radial-stop")?;
                    if self.field == 0 {
                        credit.scalar_f64(digest, 224, stop.offset, cx)?;
                    } else {
                        credit.scalar_f64(digest, 224 + u16::from(self.field), stop.color[(self.field - 1) as usize], cx)?;
                    }
                    self.field += 1;
                    if self.field == 5 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == stops.len();
                    }
                }
            }
        }
        Ok(self.terminal)
    }
}

struct DrawingStrokeDigestAuthority {
    pub phase: u8,
    pub index: usize,
    pub terminal: bool,
}

impl DrawingStrokeDigestAuthority {
    pub fn new() -> Self {
        Self { phase: 0, index: 0, terminal: false }
    }

    pub fn step(&mut self, value: Option<&StrokeStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            credit.observe(digest, 240, &[u8::from(value.is_some())], cx)?;
            self.phase = 1;
            if value.is_none() {
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        let value = value.ok_or("drawing-store.digest-stroke-missing")?;
        match self.phase {
            1..=4 => credit.scalar_f64(digest, 240 + u16::from(self.phase), value.color[(self.phase - 1) as usize], cx)?,
            5 => credit.scalar_f64(digest, 245, value.width, cx)?,
            6 => credit.observe(digest, 246, value.cap.as_str().as_bytes(), cx)?,
            7 => credit.observe(digest, 247, value.join.as_str().as_bytes(), cx)?,
            8 => credit.observe(digest, 248, &[u8::from(value.dash.is_some())], cx)?,
            9 => {
                let dash = value.dash.as_ref().ok_or("drawing-store.digest-dash-missing")?;
                credit.source_vec(dash)?;
                credit.derived_vec(dash)?;
                credit.scalar_usize(digest, 249, dash.len(), cx)?;
                self.terminal = dash.is_empty();
            }
            _ => {
                let dash = value.dash.as_ref().ok_or("drawing-store.digest-dash-missing")?;
                let item = *dash.get(self.index).ok_or("drawing-store.digest-dash-index")?;
                credit.scalar_f64(digest, 250, item, cx)?;
                self.index += 1;
                self.terminal = self.index == dash.len();
            }
        }
        if !self.terminal {
            self.phase = match self.phase {
                8 if value.dash.is_none() => {
                    self.terminal = true;
                    8
                }
                phase => phase + 1,
            };
        }
        Ok(self.terminal)
    }
}

struct DrawingPathSegmentDigestAuthority {
    pub phase: u8,
    pub terminal: bool,
}

impl DrawingPathSegmentDigestAuthority {
    pub fn new() -> Self {
        Self { phase: 0, terminal: false }
    }

    pub fn step(&mut self, value: &PathSegment, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            let variant = match value {
                PathSegment::Move { .. } => 1,
                PathSegment::Line { .. } => 2,
                PathSegment::Quad { .. } => 3,
                PathSegment::Cubic { .. } => 4,
                PathSegment::Arc { .. } => 5,
                PathSegment::Close => 6,
            };
            credit.observe(digest, 300, &[variant], cx)?;
            self.phase = 1;
            self.terminal = matches!(value, PathSegment::Close);
            return Ok(self.terminal);
        }
        let index = (self.phase - 1) as usize;
        match value {
            PathSegment::Move { to } | PathSegment::Line { to } => {
                let Some(field) = to.get(index) else {
                    self.terminal = true;
                    return Ok(true);
                };
                credit.scalar_f64(digest, 301 + index as u16, *field, cx)?;
                self.phase += 1;
                self.terminal = self.phase == 3;
            }
            PathSegment::Quad { ctrl, to } => {
                let fields = [ctrl[0], ctrl[1], to[0], to[1]];
                credit.scalar_f64(digest, 303 + index as u16, fields[index], cx)?;
                self.phase += 1;
                self.terminal = self.phase == 5;
            }
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                let fields = [ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1]];
                credit.scalar_f64(digest, 307 + index as u16, fields[index], cx)?;
                self.phase += 1;
                self.terminal = self.phase == 7;
            }
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                if index < 3 {
                    credit.scalar_f64(digest, 313 + index as u16, [*rx, *ry, *rotation][index], cx)?;
                } else if index < 5 {
                    credit.observe(digest, 313 + index as u16, &[u8::from([*large_arc, *sweep][index - 3])], cx)?;
                } else {
                    credit.scalar_f64(digest, 313 + index as u16, to[index - 5], cx)?;
                }
                self.phase += 1;
                self.terminal = self.phase == 8;
            }
            PathSegment::Close => self.terminal = true,
        }
        Ok(self.terminal)
    }
}

#[derive(Clone, Copy)]
struct DrawingOptionalDigestField {
    pub tag: u16,
    pub present: bool,
    pub next: u8,
    pub absent: u8,
}

struct DrawingLayerVariantDigestAuthority {
    pub phase: u8,
    pub index: usize,
    pub field: u8,
    pub segment: Option<DrawingPathSegmentDigestAuthority>,
    pub terminal: bool,
}

impl DrawingLayerVariantDigestAuthority {
    pub fn new() -> Self {
        Self { phase: 1, index: 0, field: 0, segment: None, terminal: false }
    }

    pub fn option(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, field: DrawingOptionalDigestField, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        let DrawingOptionalDigestField { tag, present, next, absent } = field;
        credit.observe(digest, tag, &[u8::from(present)], cx)?;
        self.phase = if present { next } else { absent };
        Ok(())
    }

    pub fn step(&mut self, layer: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        match layer {
            DrawingLayerNode::Shape(value) => match self.phase {
                1 => {
                    if !credit.observe_owned_string(digest, 340, &value.shape_kind, true, cx)? { return Ok(false); }
                    self.phase = 2;
                }
                2 => self.option(digest, credit, DrawingOptionalDigestField { tag: 341, present: value.rect.is_some(), next: 3, absent: 7 }, cx)?,
                3..=6 => {
                    let rect = value.rect.as_ref().ok_or("drawing-store.digest-rect-missing")?;
                    credit.scalar_f64(digest, 342 + u16::from(self.phase - 3), [rect.x, rect.y, rect.width, rect.height][(self.phase - 3) as usize], cx)?;
                    self.phase += 1;
                }
                7 => self.option(digest, credit, DrawingOptionalDigestField { tag: 346, present: value.ellipse.is_some(), next: 8, absent: 12 }, cx)?,
                8..=11 => {
                    let ellipse = value.ellipse.as_ref().ok_or("drawing-store.digest-ellipse-missing")?;
                    credit.scalar_f64(digest, 347 + u16::from(self.phase - 8), [ellipse.cx, ellipse.cy, ellipse.rx, ellipse.ry][(self.phase - 8) as usize], cx)?;
                    self.phase += 1;
                }
                12 => self.option(digest, credit, DrawingOptionalDigestField { tag: 351, present: value.circle.is_some(), next: 13, absent: 16 }, cx)?,
                13..=15 => {
                    let circle = value.circle.as_ref().ok_or("drawing-store.digest-circle-missing")?;
                    credit.scalar_f64(digest, 352 + u16::from(self.phase - 13), [circle.cx, circle.cy, circle.r][(self.phase - 13) as usize], cx)?;
                    self.phase += 1;
                }
                16 => self.option(digest, credit, DrawingOptionalDigestField { tag: 355, present: value.line.is_some(), next: 17, absent: 21 }, cx)?,
                17..=20 => {
                    let line = value.line.as_ref().ok_or("drawing-store.digest-line-missing")?;
                    credit.scalar_f64(digest, 356 + u16::from(self.phase - 17), [line.x1, line.y1, line.x2, line.y2][(self.phase - 17) as usize], cx)?;
                    self.phase += 1;
                }
                21 => self.option(digest, credit, DrawingOptionalDigestField { tag: 360, present: value.polygon.is_some(), next: 22, absent: 24 }, cx)?,
                22 => {
                    let points = &value.polygon.as_ref().ok_or("drawing-store.digest-polygon-missing")?.points;
                    credit.source_vec(points)?;
                    credit.derived_vec(points)?;
                    credit.scalar_usize(digest, 361, points.len(), cx)?;
                    self.phase = 23;
                    self.terminal = points.is_empty();
                }
                23 => {
                    let points = &value.polygon.as_ref().ok_or("drawing-store.digest-polygon-missing")?.points;
                    let point = points.get(self.index).ok_or("drawing-store.digest-point-index")?;
                    credit.scalar_f64(digest, 362 + u16::from(self.field), point[self.field as usize], cx)?;
                    self.field += 1;
                    if self.field == 2 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == points.len();
                    }
                }
                _ => self.terminal = true,
            },
            DrawingLayerNode::Path(value) => match self.phase {
                1 => {
                    credit.source_vec(&value.segments)?;
                    credit.derived_vec(&value.segments)?;
                    credit.scalar_usize(digest, 370, value.segments.len(), cx)?;
                    self.phase = 2;
                    self.terminal = value.segments.is_empty();
                }
                _ => {
                    let segment = value.segments.get(self.index).ok_or("drawing-store.digest-segment-index")?;
                    let cursor = self.segment.get_or_insert_with(DrawingPathSegmentDigestAuthority::new);
                    if cursor.step(segment, digest, credit, cx)? {
                        self.segment = None;
                        self.index += 1;
                        self.terminal = self.index == value.segments.len();
                    }
                }
            },
            DrawingLayerNode::Text(value) => {
                match self.phase {
                    1 => credit.scalar_f64(digest, 380, value.x, cx)?,
                    2 => credit.scalar_f64(digest, 381, value.y, cx)?,
                    3 => { if !credit.observe_owned_string(digest, 382, &value.content, true, cx)? { return Ok(false); } },
                    4 => credit.scalar_f64(digest, 383, value.size, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 5;
            }
            DrawingLayerNode::Image(value) => {
                match self.phase {
                    1 => { if !credit.observe_owned_string(digest, 390, &value.image_key, true, cx)? { return Ok(false); } },
                    2 => credit.scalar_f64(digest, 391, value.width, cx)?,
                    3 => credit.scalar_f64(digest, 392, value.height, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 4;
            }
            DrawingLayerNode::Group(value) => {
                if self.phase==1 {
                    credit.source_vec(&value.children)?;
                    credit.derived_vec(&value.children)?;
                    credit.scalar_usize(digest,400,value.children.len(),cx)?;
                    self.phase=2;
                } else {credit.observe(digest,401,&[u8::from(value.isolation)],cx)?;self.terminal=true;}
            }
            DrawingLayerNode::Boolean(value) => match self.phase {
                1 => {
                    if !credit.observe_owned_string(digest, 410, &value.operation, true, cx)? { return Ok(false); }
                    self.phase = 2;
                }
                2 => {
                    credit.source_vec(&value.children)?;
                    credit.derived_vec(&value.children)?;
                    credit.scalar_usize(digest, 411, value.children.len(), cx)?;
                    self.phase = 3;
                    self.terminal = value.children.is_empty();
                }
                _ => {
                    let item = value.children.get(self.index).ok_or("drawing-store.digest-boolean-child")?;
                    if !credit.observe_owned_string(digest, 412, item, true, cx)? { return Ok(false); }
                    self.index += 1;
                    self.terminal = self.index == value.children.len();
                }
            },
            DrawingLayerNode::Trace(value) => {
                match self.phase {
                    1 => { if !credit.observe_owned_string(digest, 420, &value.source_key, true, cx)? { return Ok(false); } },
                    2 => credit.scalar_f64(digest, 421, value.params.threshold, cx)?,
                    3 => credit.scalar_f64(digest, 422, value.params.simplify_epsilon, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 4;
            }
        }
        Ok(self.terminal)
    }
}

struct DrawingLayerDigestAuthority {
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub fill: Option<DrawingFillDigestAuthority>,
    pub stroke: Option<DrawingStrokeDigestAuthority>,
    pub variant: Option<DrawingLayerVariantDigestAuthority>,
    pub terminal: bool,
}

impl DrawingLayerDigestAuthority {
    pub fn new() -> Self {
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], fill: None, stroke: None, variant: None, terminal: false }
    }

    pub fn step(&mut self, root: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let node = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.digest-layer-path")?;
        let base = semio_s_artifact_draw_drawing_model::layer_base(node);
        let phase = self.frames[self.depth].phase;
        match phase {
            0 => {
                credit.add_source_owner(1, size_of::<DrawingLayerNode>())?;
                credit.add_derived_owner(1, size_of::<DrawingLayerNode>())?;
                let variant = match node {
                    DrawingLayerNode::Shape(_) => 1,
                    DrawingLayerNode::Path(_) => 2,
                    DrawingLayerNode::Text(_) => 3,
                    DrawingLayerNode::Image(_) => 4,
                    DrawingLayerNode::Group(_) => 5,
                    DrawingLayerNode::Boolean(_) => 6,
                    DrawingLayerNode::Trace(_) => 7,
                };
                credit.observe(digest, 100, &[variant], cx)?;
                self.frames[self.depth].phase = 1;
            }
            1 => {
                if !credit.observe_owned_string(digest, 101, &base.id, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 2;
            }
            2 => {
                if !credit.observe_owned_string(digest, 102, &base.name, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 3;
            }
            3 => {
                credit.observe(digest, 103, &[u8::from(base.visible)], cx)?;
                self.frames[self.depth].phase = 4;
            }
            4 => {
                credit.observe(digest, 104, &[u8::from(base.locked)], cx)?;
                self.frames[self.depth].phase = 5;
            }
            5 => {
                credit.scalar_f64(digest, 105, base.opacity, cx)?;
                self.frames[self.depth].phase = 6;
            }
            6 => {
                if !credit.observe_owned_string(digest, 106, &base.blend_mode, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 7;
            }
            7..=12 => {
                let fields = [base.transform.x, base.transform.y, base.transform.scale_x, base.transform.scale_y, base.transform.rotation, base.transform.shear];
                credit.scalar_f64(digest, 107 + u16::from(phase - 7), fields[(phase - 7) as usize], cx)?;
                self.frames[self.depth].phase += 1;
            }
            13 => {
                let fill = self.fill.get_or_insert_with(DrawingFillDigestAuthority::new);
                if fill.step(base.attributes.fill.as_ref(), digest, credit, cx)? {
                    self.fill = None;
                    self.frames[self.depth].phase = 14;
                }
            }
            14 => {
                let stroke = self.stroke.get_or_insert_with(DrawingStrokeDigestAuthority::new);
                if stroke.step(base.attributes.stroke.as_ref(), digest, credit, cx)? {
                    self.stroke = None;
                    self.frames[self.depth].phase = 15;
                }
            }
            15 => {
                credit.observe(digest, 113, &[u8::from(base.attributes.fill_rule == semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Nonzero)], cx)?;
                self.frames[self.depth].phase = 16;
            }
            16 => {
                let variant = self.variant.get_or_insert_with(DrawingLayerVariantDigestAuthority::new);
                if variant.step(node, digest, credit, cx)? {
                    self.variant = None;
                    self.frames[self.depth].phase = 17;
                }
            }
            17 => {
                if let DrawingLayerNode::Group(group) = node {
                    let child = self.frames[self.depth].child;
                    if child < group.children.len() {
                        if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                            return Err("drawing-store.digest-layer-depth");
                        }
                        self.frames[self.depth].child += 1;
                        self.path[self.depth] = child;
                        self.depth += 1;
                        self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 18;
                return Ok(false);
            }
            _ => {
                credit.observe(digest, 199, &[], cx)?;
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }

    pub fn close(&mut self) {
        self.fill = None;
        self.stroke = None;
        self.variant = None;
        self.terminal = true;
    }
}

pub struct DrawingMutationDigestAuthority {
    pub layer: Option<DrawingLayerDigestAuthority>,
    pub fill: Option<DrawingFillDigestAuthority>,
    pub stroke: Option<DrawingStrokeDigestAuthority>,
    pub segment: Option<DrawingPathSegmentDigestAuthority>,
    pub segment_index: usize,
    pub credit: DrawingSemanticDigestCredit,
    pub phase: u8,
    pub terminal: bool,
}

impl DrawingMutationDigestAuthority {
    pub fn new() -> Self {
        Self { layer: None, fill: None, stroke: None, segment: None, segment_index: 0, credit: DrawingSemanticDigestCredit::default(), phase: 0, terminal: false }
    }

    pub fn variant(mutation: &DrawingMutation) -> u8 {
        match mutation {
            DrawingMutation::ImportImageAsset(_) => 25,
            DrawingMutation::RemoveImageAsset(_) => 26,
            DrawingMutation::SetLayerVisible(_) => 1,
            DrawingMutation::SetLayerLocked(_) => 2,
            DrawingMutation::SetLayerOpacity(_) => 3,
            DrawingMutation::SetLayerBlendMode(_) => 4,
            DrawingMutation::RenameLayer(_) => 5,
            DrawingMutation::UpdateLayerTransform(_) => 6,
            DrawingMutation::ReplaceLayerFill(_) => 7,
            DrawingMutation::ReplaceLayerStroke(_) => 8,
            DrawingMutation::SetLayerBooleanOperation(_) => 9,
            DrawingMutation::UpdateLayerTraceParams(_) => 10,
            DrawingMutation::CreateLayer(_) => 11,
            DrawingMutation::DuplicateLayer(_) => 12,
            DrawingMutation::DeleteLayer(_) => 13,
            DrawingMutation::ReorderLayer(_) => 14,
            DrawingMutation::UpdatePathGeometry(_) => 15,
            DrawingMutation::UpdateText(_) => 16,
            DrawingMutation::UpdateImage(_) => 23,
            DrawingMutation::SetShapeCoordinate(_) => 24,
            DrawingMutation::SetLayerFillRule(_) => 17,
            DrawingMutation::SetGroupIsolation(_) => 18,
            DrawingMutation::DragLayers(_) => 19,
            DrawingMutation::RotateLayers(_) => 20,
            DrawingMutation::ScaleLayers(_) => 21,
            DrawingMutation::DragPathPoints(_) => 22,
        }
    }

    pub fn finish(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        self.credit.seal(digest, cx)?;
        self.terminal = true;
        Ok(true)
    }

    /// 🧮️ The scalar parameters of a selection transform leaf in declaration order, and how many there are.
    pub fn selection_scalars(mutation: &DrawingMutation) -> ([f64; 4], usize) {
        match mutation {
            DrawingMutation::DragLayers(value) => ([value.dx, value.dy, 0.0, 0.0], 2),
            DrawingMutation::RotateLayers(value) => ([value.pivot_x, value.pivot_y, value.angle, 0.0], 3),
            DrawingMutation::ScaleLayers(value) => ([value.pivot_x, value.pivot_y, value.scale_x, value.scale_y], 4),
            DrawingMutation::DragPathPoints(value) => ([value.dx, value.dy, 0.0, 0.0], 2),
            _ => ([0.0; 4], 0),
        }
    }

    pub fn step(&mut self, mutation: &DrawingMutation, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            self.credit.observe(digest, 1, &[Self::variant(mutation)], cx)?;
            self.phase = 1;
            return Ok(false);
        }
        if self.phase == 1 {
            match mutation {
                DrawingMutation::CreateLayer(value) => {
                    self.credit.add_source_owner(1, size_of::<Box<DrawingLayerNode>>())?;
                    self.credit.add_derived_owner(1, size_of::<Box<DrawingLayerNode>>())?;
                    self.credit.observe(digest, 2, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 2 } else { 3 };
                }
                DrawingMutation::DragLayers(semio_s_artifact_draw_drawing_transform::drag_layers::mutation::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(semio_s_artifact_draw_drawing_transform::scale_layers::mutation::ScaleLayers { targets, .. }) => {
                    self.credit.source_vec(targets)?;
                    self.credit.scalar_usize(digest, 2, targets.len(), cx)?;
                    self.phase = 2;
                }
                DrawingMutation::DragPathPoints(value) => {
                    self.credit.source_vec(&value.targets)?;
                    self.credit.scalar_usize(digest, 2, value.targets.len(), cx)?;
                    self.phase = 2;
                }
                _ => {
                    let target = drawing_mutation_target_owner(mutation).ok_or("drawing-store.mutation-target-owner")?;
                    if !self.credit.observe_owned_string(digest, 2, target, false, cx)? { return Ok(false); }
                    self.phase = 2;
                }
            }
            return Ok(false);
        }
        match mutation {
            DrawingMutation::DragLayers(semio_s_artifact_draw_drawing_transform::drag_layers::mutation::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(semio_s_artifact_draw_drawing_transform::scale_layers::mutation::ScaleLayers { targets, .. }) => {
                if let Some(target) = targets.get(self.segment_index) {
                    if !self.credit.observe_owned_string(digest, 3, target, false, cx)? { return Ok(false); }
                    self.segment_index += 1;
                    return Ok(false);
                }
                let (scalars, count) = Self::selection_scalars(mutation);
                let offset = usize::from(self.phase - 2);
                if offset < count {
                    self.credit.scalar_f64(digest, 4 + offset as u16, scalars[offset], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::DragPathPoints(value) => {
                if let Some(target) = value.targets.get(self.segment_index) {
                    if !self.credit.observe_owned_string(digest, 3, &target.layer_id, false, cx)? { return Ok(false); }
                    let point = match target.point {
                        semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Anchor => 0u64,
                        semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Control1 => 1,
                        semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Control2 => 2,
                    };
                    self.credit.scalar_usize(digest, 4, (target.index as u64).checked_mul(3).and_then(|index| index.checked_add(point)).ok_or("drawing-store.mutation-point-index-overflow")? as usize, cx)?;
                    self.segment_index += 1;
                    return Ok(false);
                }
                let (scalars, count) = Self::selection_scalars(mutation);
                let offset = usize::from(self.phase - 2);
                if offset < count {
                    self.credit.scalar_f64(digest, 5 + offset as u16, scalars[offset], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdatePathGeometry(value) => match self.phase {
                2 => {
                    self.credit.source_vec(&value.segments)?;
                    self.credit.derived_vec(&value.segments)?;
                    self.credit.scalar_usize(digest, 370, value.segments.len(), cx)?;
                    self.phase = 3;
                    Ok(false)
                }
                _ => {
                    let Some(segment) = value.segments.get(self.segment_index) else { return self.finish(digest, cx) };
                    if !semio_s_artifact_draw_drawing_model::valid_path_segment(segment) { return Err("drawing-store.path-invalid-segment"); }
                    let cursor = self.segment.get_or_insert_with(DrawingPathSegmentDigestAuthority::new);
                    if cursor.step(segment, digest, &mut self.credit, cx)? { self.segment = None; self.segment_index += 1; }
                    Ok(false)
                }
            },
            DrawingMutation::SetLayerVisible(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.visible)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerLocked(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.locked)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerFillRule(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest,3,&[u8::from(value.fill_rule==semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Nonzero)],cx)?;
                    self.phase=3;Ok(false)
                } else {self.finish(digest,cx)}
            }
            DrawingMutation::SetGroupIsolation(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest,3,&[u8::from(value.isolation)],cx)?;
                    self.phase=3;Ok(false)
                } else {self.finish(digest,cx)}
            }
            DrawingMutation::SetLayerOpacity(value) => {
                if self.phase == 2 {
                    self.credit.scalar_f64(digest, 3, value.opacity, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerBlendMode(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.blend_mode, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetShapeCoordinate(value)=>match self.phase {
                2=>{self.credit.observe(digest,3,value.field.as_str().as_bytes(),cx)?;self.phase=3;Ok(false)}
                3=>{self.credit.observe(digest,4,&[u8::from(value.index.is_some())],cx)?;self.phase=4;Ok(false)}
                4=>{if let Some(index)=value.index{self.credit.scalar_usize(digest,5,index,cx)?;}self.phase=5;Ok(false)}
                5=>{self.credit.scalar_f64(digest,6,value.value,cx)?;self.phase=6;Ok(false)}
                _=>self.finish(digest,cx),
            },
            DrawingMutation::UpdateImage(value) => match self.phase {
                2=>{if !self.credit.observe_owned_string(digest,3,&value.image_key,true,cx)?{return Ok(false);}self.phase=3;Ok(false)}
                3=>{self.credit.scalar_f64(digest,4,value.width,cx)?;self.phase=4;Ok(false)}
                4=>{self.credit.scalar_f64(digest,5,value.height,cx)?;self.phase=5;Ok(false)}
                _=>self.finish(digest,cx),
            },
            DrawingMutation::UpdateText(value) => match self.phase {
                2 => { if !self.credit.observe_owned_string(digest, 3, &value.content, true, cx)? { return Ok(false); } self.phase = 3; Ok(false) }
                3 => { self.credit.scalar_f64(digest, 4, value.size, cx)?; self.phase = 4; Ok(false) }
                _ => self.finish(digest, cx),
            },
            DrawingMutation::RenameLayer(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.new_name, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateLayerTransform(value) => {
                if self.phase <= 7 {
                    let fields = [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation, value.transform.shear];
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::ReplaceLayerFill(value) => {
                if self.phase == 2 {
                    let fill = self.fill.get_or_insert_with(DrawingFillDigestAuthority::new);
                    if fill.step(value.fill.as_ref(), digest, &mut self.credit, cx)? {
                        self.fill = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::ReplaceLayerStroke(value) => {
                if self.phase == 2 {
                    let stroke = self.stroke.get_or_insert_with(DrawingStrokeDigestAuthority::new);
                    if stroke.step(value.stroke.as_ref(), digest, &mut self.credit, cx)? {
                        self.stroke = None;
        self.segment = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerBooleanOperation(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.boolean_operation, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateLayerTraceParams(value) => {
                if self.phase <= 3 {
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), [value.params.threshold, value.params.simplify_epsilon][(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::CreateLayer(value) => match self.phase {
                2 => {
                    if !self.credit.observe_owned_string(digest, 3, value.parent_id.as_ref().ok_or("drawing-store.digest-parent-missing")?, false, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                }
                3 => {
                    self.credit.observe(digest, 4, &[u8::from(value.index.is_some())], cx)?;
                    self.phase = if value.index.is_some() { 4 } else { 5 };
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index.ok_or("drawing-store.digest-index-missing")?, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                5 => {
                    let layer = self.layer.get_or_insert_with(DrawingLayerDigestAuthority::new);
                    if layer.step(&value.layer, digest, &mut self.credit, cx)? {
                        self.layer = None;
                        self.phase = 6;
                    }
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
            DrawingMutation::DuplicateLayer(value)=>{
                if self.phase==2 {self.credit.source_vec(&value.identities)?;self.credit.scalar_usize(digest,3,value.identities.len(),cx)?;self.phase=3;return Ok(false);}
                if let Some(assignment)=value.identities.get(self.segment_index){
                    let field=if self.phase==3{&assignment.source}else{&assignment.target};
                    if !self.credit.observe_owned_string(digest,if self.phase==3{4}else{5},field,false,cx)?{return Ok(false);}
                    if self.phase==3{self.phase=4;}else{self.phase=3;self.segment_index+=1;}return Ok(false);
                }self.finish(digest,cx)
            }
            DrawingMutation::RemoveImageAsset(_) | DrawingMutation::DeleteLayer(_) => self.finish(digest, cx),
            DrawingMutation::ImportImageAsset(value)=>match self.phase {
                2=>{self.credit.scalar_usize(digest,3,value.asset.width as usize,cx)?;self.phase=3;Ok(false)}
                3=>{self.credit.scalar_usize(digest,4,value.asset.height as usize,cx)?;self.phase=4;Ok(false)}
                4=>{self.credit.source_vec(&value.asset.samples)?;self.credit.derived_vec(&value.asset.samples)?;self.credit.scalar_usize(digest,5,value.asset.samples.len(),cx)?;self.phase=5;Ok(false)}
                _=>{if let Some(sample)=value.asset.samples.get(self.segment_index){self.credit.observe(digest,6,sample,cx)?;self.segment_index+=1;Ok(false)}else{self.finish(digest,cx)}}
            },
            DrawingMutation::ReorderLayer(value) => match self.phase {
                2 => {
                    self.credit.observe(digest, 3, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 3 } else { 4 };
                    Ok(false)
                }
                3 => {
                    if !self.credit.observe_owned_string(digest, 4, value.parent_id.as_ref().ok_or("drawing-store.digest-parent-missing")?, false, cx)? { return Ok(false); }
                    self.phase = 4;
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
        }
    }

    pub fn totals(&self) -> Option<DrawingSemanticDigestTotals> {
        self.terminal.then(|| self.credit.totals()).flatten()
    }

    pub fn close_step(&mut self) -> RetainedCloneStep {
        if let Some(layer) = self.layer.as_mut() {
            layer.close();
        }
        self.layer = None;
        self.fill = None;
        self.stroke = None;
        self.segment = None;
        self.terminal = true;
        RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.layer.is_none() && self.fill.is_none() && self.stroke.is_none() && self.segment.is_none()
    }
}

impl Drop for DrawingMutationDigestAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation digest reached Drop before exact terminal close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingMutationWorksetPlan {
    pub source_items: usize,
    pub source_bytes: usize,
    pub mutation_items: usize,
    pub mutation_bytes: usize,
    pub arena_items: usize,
    pub arena_bytes: usize,
    pub authority_items: usize,
    pub authority_bytes: usize,
    pub clone_items: usize,
    pub clone_bytes: usize,
    pub maximum_container: usize,
    pub container_slots: usize,
}

impl DrawingMutationWorksetPlan {
    pub fn checked_total(values: &[usize], fault: &'static str) -> Result<usize, &'static str> {
        values.iter().try_fold(0usize, |total, value| total.checked_add(*value).ok_or(fault))
    }

    pub fn admit(
        source: DrawingSnapshotOwnerTotals,
        mutation: DrawingSemanticDigestTotals,
        arena_items: usize,
        arena_bytes: usize,
        container_slots: usize,
        authority_bytes: usize,
    ) -> Result<Self, &'static str> {
        if source.source_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.preflight-item-capacity");
        }
        if source.source_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.preflight-byte-capacity");
        }
        if mutation.source_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-source-owner-item-capacity");
        }
        if mutation.source_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-source-owner-byte-capacity");
        }
        if mutation.maximum_field_bytes > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.mutation-field-capacity");
        }
        Ok(Self {
            source_items: source.source_items,
            source_bytes: source.source_bytes,
            mutation_items: mutation.semantic_items,
            mutation_bytes: mutation.maximum_field_bytes,
            arena_items,
            arena_bytes,
            authority_items: 1,
            authority_bytes,
            clone_items: 0,
            clone_bytes: 0,
            maximum_container: source.maximum_container,
            container_slots,
        })
    }

    pub fn admit_clone(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        if items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-clone-item-capacity");
        }
        if bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-clone-byte-capacity");
        }
        self.clone_items = items;
        self.clone_bytes = bytes;
        self.workset_items()?;
        self.workset_bytes()?;
        Ok(())
    }

    pub fn workset_items(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.arena_items, self.authority_items, self.clone_items], "drawing-store.mutation-workset-item-overflow")
    }

    pub fn workset_bytes(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.arena_bytes, self.authority_bytes, self.clone_bytes], "drawing-store.mutation-workset-byte-overflow")
    }

    pub fn simultaneous_bytes(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.source_bytes, self.mutation_bytes, self.workset_bytes()?], "drawing-store.mutation-simultaneous-byte-overflow")
    }

    pub fn architectural_maximum_bytes(&self) -> Result<usize, &'static str> {
        DRAWING_MAXIMUM_NESTED_BYTES
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(DRAWING_OWNED_FIELD_BYTES))
            .and_then(|bytes| bytes.checked_add(self.arena_bytes))
            .and_then(|bytes| bytes.checked_add(self.authority_bytes))
            .ok_or("drawing-store.mutation-architectural-byte-overflow")
    }
}

/// 🎯️ The owner text a mutation addresses first: its layer or asset id, or the first target of a selection transform.
pub fn drawing_mutation_target_owner(mutation: &DrawingMutation) -> Option<&DrawingNativeText> {
    match mutation {
        DrawingMutation::ImportImageAsset(value)=>Some(&value.asset_id),
        DrawingMutation::RemoveImageAsset(value)=>Some(&value.asset_id),
        DrawingMutation::SetLayerVisible(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerLocked(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerOpacity(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerFillRule(value) => Some(&value.layer_id),
        DrawingMutation::SetGroupIsolation(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerBlendMode(value) => Some(&value.layer_id),
        DrawingMutation::RenameLayer(value) => Some(&value.layer_id),
        DrawingMutation::UpdateLayerTransform(value) => Some(&value.layer_id),
        DrawingMutation::ReplaceLayerFill(value) => Some(&value.layer_id),
        DrawingMutation::ReplaceLayerStroke(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerBooleanOperation(value) => Some(&value.layer_id),
        DrawingMutation::UpdateLayerTraceParams(value) => Some(&value.layer_id),
        DrawingMutation::CreateLayer(value) => Some(semio_s_artifact_draw_drawing_model::layer_id(&value.layer)),
        DrawingMutation::DuplicateLayer(value) => Some(&value.layer_id),
        DrawingMutation::DeleteLayer(value) => Some(&value.layer_id),
        DrawingMutation::ReorderLayer(value) => Some(&value.layer_id),
        DrawingMutation::UpdatePathGeometry(value) => Some(&value.layer_id),
        DrawingMutation::UpdateText(value) => Some(&value.layer_id),
        DrawingMutation::UpdateImage(value) => Some(&value.layer_id),
        DrawingMutation::SetShapeCoordinate(value) => Some(&value.layer_id),
        DrawingMutation::DragLayers(semio_s_artifact_draw_drawing_transform::drag_layers::mutation::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(semio_s_artifact_draw_drawing_transform::scale_layers::mutation::ScaleLayers { targets, .. }) => targets.first(),
        DrawingMutation::DragPathPoints(value) => value.targets.first().map(|target| &target.layer_id),
    }
}

struct DrawingDuplicateReferenceSearch {
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub owners: Option<(usize, usize)>,
    pub equality: semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingTextEqualityCursor,
    pub result: Option<bool>,
}

impl DrawingDuplicateReferenceSearch {
    pub fn new() -> Self { Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], owners: None, equality: Default::default(), result: None } }

    pub fn node_at<'a>(root: &'a DrawingLayerNode, path: &[usize]) -> Option<&'a DrawingLayerNode> {
        let mut node = root;
        for index in path { let DrawingLayerNode::Group(group) = node else { return None };node = group.children.get(*index)?; }
        Some(node)
    }

    pub fn step(&mut self, root: &DrawingLayerNode, target: &DrawingNativeText, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let owners = (root as *const _ as usize, target as *const _ as usize);
        if self.owners.is_some_and(|held| held != owners) { return Err("drawing-store.duplicate-reference-source-changed"); }
        self.owners = Some(owners);
        if self.result.is_some() { return Ok(true); }
        let node = Self::node_at(root, &self.path[..self.depth]).ok_or("drawing-store.duplicate-reference-path")?;
        if self.frames[self.depth].phase == 0 {
            let step = self.equality.step(semio_s_artifact_draw_drawing_model::layer_id(node), target, 1, DRAWING_OWNED_FIELD_BYTES)?;
            cx.consume_fuel(step.compared_bytes.max(1) as u64);
            if !step.complete { return Ok(false); }
            let equal = self.equality.result().ok_or("drawing-store.duplicate-reference-equality")?;
            self.equality.close_step(1);self.equality = Default::default();
            if equal { self.result = Some(true);return Ok(true); }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        if let DrawingLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH { return Err("drawing-store.duplicate-reference-depth-capacity"); }
                self.frames[self.depth].child += 1;self.path[self.depth] = child;self.depth += 1;self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                cx.consume_fuel(1);return Ok(false);
            }
        }
        if self.depth == 0 { self.result = Some(false);Ok(true) } else { self.depth -= 1;cx.consume_fuel(1);Ok(false) }
    }

    pub fn close(&mut self) { self.equality.close_step(1);self.owners = None;self.result = None; }
}

pub struct DrawingDuplicateRewriteAuthority {
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub owners: Option<(usize, usize)>,
    pub identity: semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingDuplicateIdentityCursor,
    pub identity_index:Option<usize>,
    pub assignment_scan:usize,
    pub identities_observed:usize,
    pub collision:Option<DrawingLayerLocator>,
    pub append: semio_framework_value::paged::PagedUtf8AppendCursor,
    pub append_active: bool,
    pub append_chunk: usize,
    pub append_complete: bool,
    pub native_id: std::mem::ManuallyDrop<DrawingNativeText>,
    pub displaced: Option<DrawingDecodedFieldRetirement<DrawingNativeText>>,
    pub search: Option<DrawingDuplicateReferenceSearch>,
    pub reference: usize,
    pub turn: usize,
    pub mode: u8,
    pub pending_id: std::mem::ManuallyDrop<Option<String>>,
    pub pending_name: std::mem::ManuallyDrop<Option<String>>,
    pub terminal: bool,
}

impl DrawingDuplicateRewriteAuthority {
    pub fn new(mut pending_id: String, mut pending_name: String) -> Self {
        pending_id.clear();pending_name.clear();
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], owners: None, identity: Default::default(), identity_index:None, assignment_scan:0, identities_observed:0, collision:None, append: Default::default(), append_active: false, append_chunk: 0, append_complete: false, native_id: std::mem::ManuallyDrop::new(Default::default()), displaced: None, search: None, reference: 0, turn: 0, mode: 0, pending_id: std::mem::ManuallyDrop::new(Some(pending_id)), pending_name: std::mem::ManuallyDrop::new(Some(pending_name)), terminal: false }
    }

    pub fn close_append(&mut self, grant: RetainedCloneGrant) -> Result<bool, &'static str> {
        if !self.append_active { return Ok(true); }
        self.append.begin_close();
        let step = self.append.close_step(grant).map_err(|_| "drawing-store.duplicate-append-close")?;
        if matches!(step, RetainedCloneStep::Complete(_)) && self.append.terminal_is_empty() { self.append = Default::default();self.append_active = false;return Ok(true); }
        Ok(false)
    }

    pub fn append_paged(&mut self, source: &DrawingNativeText, grant: RetainedCloneGrant, label: &'static str) -> Result<bool, &'static str> {
        if self.append_complete {
            if self.close_append(grant)? { self.append_complete = false;self.append_chunk += 1; }
            return Ok(false);
        }
        let Some(chunk) = source.retained_chunks().get(self.append_chunk) else { self.append_chunk = 0;return Ok(true); };
        self.append_active = true;
        let step = self.append.advance(chunk.as_str(), &mut self.native_id, grant).map_err(|_| label)?;
        if matches!(step, RetainedCloneStep::Complete(_)) { self.append_complete = true; }
        Ok(false)
    }

    pub fn retire_displaced_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if let Some(displaced) = self.displaced.as_mut() {
            let step = displaced.step(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) && displaced.terminal_is_empty() { self.displaced = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.native_id.retained_chunks().allocated_bytes() == 0 && self.native_id.is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<DrawingNativeText>(), ..Default::default() };
        if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let value = std::mem::take(&mut *self.native_id);
        match DrawingDecodedFieldRetirement::try_new(value) {
            Ok(retirement) => self.displaced = Some(retirement),
            Err((error, value)) => { *self.native_id = value;return Err(error); }
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    pub fn step(&mut self, root: &mut DrawingLayerNode, original: &DrawingLayerNode, document:&DrawingSnapshot, identities:&semio_framework_value::list::PagedList<semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment,{usize::MAX}>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal { return Ok(true); }
        let owners = (root as *const _ as usize, original as *const _ as usize);
        if self.owners.is_some_and(|held| held != owners) { return Err("drawing-store.duplicate-root-changed"); }
        self.owners = Some(owners);
        let grant = initial_snapshot_clone_grant(self.turn);self.turn = self.turn.wrapping_add(1);
        let source = DrawingDuplicateReferenceSearch::node_at(original, &self.path[..self.depth]).ok_or("drawing-store.duplicate-source-path")?;
        let node = DrawingLayerCloneAuthority::target_at_mut(root, &self.path[..self.depth]).ok_or("drawing-store.duplicate-path")?;
        let phase = self.frames[self.depth].phase;
        if self.mode==0&&phase==0{
            if let Some(assignment)=identities.get(self.assignment_scan){
                if assignment.source==*semio_s_artifact_draw_drawing_model::layer_id(source){if self.identity_index.is_some(){return Err("drawing-store.duplicate-identity-repeated-source");}self.identity_index=Some(self.assignment_scan);}
                self.assignment_scan+=1;cx.consume_fuel(1);return Ok(false);
            }
            let identity=identities.get(self.identity_index.ok_or("drawing-store.duplicate-identity-missing")?).ok_or("drawing-store.duplicate-identity-index")?;
            if identity.target.is_empty(){return Err("drawing-store.duplicate-identity-empty");}
            let locator=self.collision.get_or_insert_with(DrawingLayerLocator::new);
            if !locator.step(document,&identity.target,cx)?{return Ok(false);}
            if locator.found().is_some(){return Err("drawing-store.mutation-duplicate-layer");}self.collision=None;self.frames[self.depth].phase=8;
        }
        let target_identity=if self.mode==0{identities.get(self.identity_index.ok_or("drawing-store.duplicate-identity-missing")?).ok_or("drawing-store.duplicate-identity-index")?}else{identities.get(self.identity_index.unwrap_or(0)).ok_or("drawing-store.duplicate-identity-missing")?};
        if self.mode==0&&self.frames[self.depth].phase==8{
            if self.assignment_scan>=identities.len(){self.assignment_scan=0;}
            if let Some(assignment)=identities.get(self.assignment_scan){
                if Some(self.assignment_scan)!=self.identity_index&&assignment.target==target_identity.target{return Err("drawing-store.duplicate-identity-repeated-target");}
                self.assignment_scan+=1;cx.consume_fuel(1);if self.assignment_scan<identities.len(){return Ok(false);}
            }
            self.frames[self.depth].phase=9;return Ok(false);
        }
        let phase=if self.frames[self.depth].phase==9{0}else{self.frames[self.depth].phase};
        if self.mode == 0 && phase < 7 {
            match phase {
                0 => {
                    let step = self.identity.step(&target_identity.target, 1, DRAWING_OWNED_FIELD_BYTES)?;
                    cx.consume_fuel(step.observed_bytes.max(1) as u64);
                    if step.complete { self.frames[self.depth].phase = 1; }
                }
                1 => {
                    let id = self.identity.identity(&target_identity.target).ok_or("drawing-store.duplicate-identity-missing")?;
                    let done = self.append_paged(id, grant, "drawing-store.duplicate-identity-append")?;
                    cx.consume_fuel(1);
                    if done { self.frames[self.depth].phase = 2; }
                }
                2 => { if self.close_append(grant)? { self.frames[self.depth].phase = 3; }cx.consume_fuel(1); }
                3 => {
                    std::mem::swap(&mut semio_s_artifact_draw_drawing_model::layer_base_mut(node).id, &mut self.native_id);
                    self.identity.close_step(1);self.identity = Default::default();self.identities_observed+=1;self.frames[self.depth].phase = 4;
                    cx.consume_fuel(size_of::<DrawingNativeText>() as u64);
                }
                4 => { if matches!(self.retire_displaced_granted(grant).map_err(|_| "drawing-store.duplicate-text-close")?, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 5; }cx.consume_fuel(1); }
                5 if self.depth == 0 => {
                    self.append_active = true;
                    let step = self.append.advance(" copy", &mut semio_s_artifact_draw_drawing_model::layer_base_mut(node).name, grant).map_err(|_| "drawing-store.duplicate-name-append")?;
                    cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                    if matches!(step, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 6; }
                }
                5 => { self.frames[self.depth].phase = 7;cx.consume_fuel(1); }
                6 => { if self.close_append(grant)? { self.frames[self.depth].phase = 7; }cx.consume_fuel(1); }
                _ => unreachable!(),
            }
            return Ok(false);
        }
        if self.mode == 1 {
            if let (DrawingLayerNode::Boolean(source), DrawingLayerNode::Boolean(node)) = (source, &mut *node) {
                if self.reference < source.children.len() {
                    let target = source.children.get(self.reference).ok_or("drawing-store.duplicate-reference-target")?;

                    match phase {
                        0 => {
                            let search = self.search.get_or_insert_with(DrawingDuplicateReferenceSearch::new);
                            if !search.step(original, target, cx)? { return Ok(false); }
                            let internal = search.result.ok_or("drawing-store.duplicate-reference-result")?;
                            search.close();self.search = None;
                            if internal { self.identity_index=None;self.assignment_scan=0;self.frames[self.depth].phase = 1; } else { self.reference += 1; }
                        }
                        1 => {
                            if let Some(assignment)=identities.get(self.assignment_scan){if assignment.source==*target{self.identity_index=Some(self.assignment_scan);}self.assignment_scan+=1;cx.consume_fuel(1);return Ok(false);}
                            let reference_identity=identities.get(self.identity_index.ok_or("drawing-store.duplicate-reference-identity-missing")?).ok_or("drawing-store.duplicate-reference-identity-index")?;
                            let step = self.identity.step(&reference_identity.target, 1, DRAWING_OWNED_FIELD_BYTES)?;
                            cx.consume_fuel(step.observed_bytes.max(1) as u64);
                            if step.complete { self.frames[self.depth].phase = 2; }
                        }
                        2 => {
                            let reference_identity=identities.get(self.identity_index.ok_or("drawing-store.duplicate-reference-identity-missing")?).ok_or("drawing-store.duplicate-reference-identity-index")?;
                            let id = self.identity.identity(&reference_identity.target).ok_or("drawing-store.duplicate-reference-identity")?;
                            let done = self.append_paged(id, grant, "drawing-store.duplicate-reference-append")?;
                            cx.consume_fuel(1);
                            if done { self.frames[self.depth].phase = 3; }
                        }
                        3 => { if self.close_append(grant)? { self.frames[self.depth].phase = 4; }cx.consume_fuel(1); }
                        4 => {
                            let child = node.children.get_mut(self.reference).ok_or("drawing-store.duplicate-reference-destination")?;
                            std::mem::swap(child, &mut self.native_id);self.identity.close_step(1);self.identity = Default::default();self.frames[self.depth].phase = 5;
                            cx.consume_fuel(size_of::<DrawingNativeText>() as u64);
                        }
                        5 => { if matches!(self.retire_displaced_granted(grant).map_err(|_| "drawing-store.duplicate-text-close")?, RetainedCloneStep::Complete(_)) { self.reference += 1;self.frames[self.depth].phase = 0; }cx.consume_fuel(1); }
                        _ => return Err("drawing-store.duplicate-reference-phase"),
                    }
                    return Ok(false);
                }
            }
        }
        if let DrawingLayerNode::Group(group) = source {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH { return Err("drawing-store.duplicate-depth-capacity"); }
                self.frames[self.depth].child += 1;self.path[self.depth] = child;self.identity_index=None;self.assignment_scan=0;self.depth += 1;self.frames[self.depth] = DrawingTraversalFrame::EMPTY;self.reference = 0;
                cx.consume_fuel(1);return Ok(false);
            }
        }
        if self.depth == 0 {
            if self.mode == 0 { if self.identities_observed!=identities.len(){return Err("drawing-store.duplicate-identity-census");}self.mode = 1;self.frames = [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH];self.path = [0; DRAWING_MAXIMUM_LAYER_DEPTH];self.reference = 0;cx.consume_fuel(1);return Ok(false); }
            self.identity_index=None;self.assignment_scan=0;self.identities_observed=0;self.collision=None;self.terminal = true;self.owners = None;Ok(true)
        } else { self.depth -= 1;self.reference = 0;cx.consume_fuel(1);Ok(false) }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.terminal { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.identity.terminal_is_empty() { self.identity.close_step(1);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if let Some(search) = self.search.as_mut() { search.close();self.search = None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if self.append_active {
            self.append.begin_close();
            let step = self.append.close_step(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) && self.append.terminal_is_empty() { self.append = Default::default();self.append_active = false; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = self.retire_displaced_granted(grant)?;
        if !matches!(step, RetainedCloneStep::Complete(_)) { return Ok(step); }
        self.identity_index=None;self.assignment_scan=0;self.identities_observed=0;self.collision=None;self.owners = None;self.terminal = true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    pub fn take_owners(&mut self) -> Option<(String, String)> {
        if !self.terminal || self.append_active || self.displaced.is_some() || self.search.is_some() || !self.identity.terminal_is_empty() { return None; }
        let mut id = self.pending_id.take()?;let mut name = self.pending_name.take()?;id.clear();name.clear();Some((id, name))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.pending_id.is_none() && self.pending_name.is_none() && self.owners.is_none() && !self.append_active && self.displaced.is_none() && self.search.is_none() && self.collision.is_none() && self.identity_index.is_none() && self.assignment_scan==0 && self.identities_observed==0 && self.identity.terminal_is_empty() && self.native_id.retained_chunks().allocated_bytes() == 0 && self.native_id.is_empty()
    }
}

impl Drop for DrawingDuplicateRewriteAuthority {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing duplicate rewrite reached Drop before staged id/name retirement"); }
}
