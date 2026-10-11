//! 📢️ Writer field updates construct preserved text incrementally under Store ownership.

use super::{WriterEditorSelection, WriterMainWindowTransient, WriterMainWindowTransientMutation};
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::RetireOwned, ValueError, ValueRefusalKind};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ArtifactEphemeralPreparationTask, ArtifactEphemeralPreparationTaskStep, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ErasedSnapshotRetirement};

impl RetireOwned for WriterEditorSelection {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { semio_framework_value::retirement::leaf((self.start, self.end, self.splice)) }
}

impl RetireOwned for WriterMainWindowTransient {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::sequence(vec![self.editor_selection.retirement(), self.lint_generation.retirement(), self.engagement_input.retirement()])
    }
}

fn footprint(mutation: &WriterMainWindowTransientMutation) -> Result<ArtifactStoreOneItemFootprint, String> {
    let capacity = match mutation { WriterMainWindowTransientMutation::SetEngagementInput(value) => value.value.capacity(), _ => 0 };
    if capacity > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES { return Err("Writer engagement input exceeds retained capacity".into()); }
    Ok(ArtifactStoreOneItemFootprint::for_ephemeral_item(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
}

pub(super) fn owners() -> semio_framework_plugin::WindowTransientOwnerBundle<WriterMainWindowTransient, WriterMainWindowTransientMutation> {
    let state: Arc<dyn store::ArtifactOwnedValueRetirementFactory<WriterMainWindowTransient>> = Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default());
    let mutation: Arc<dyn store::ArtifactOwnedValueRetirementFactory<WriterMainWindowTransientMutation>> = Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default());
    let preparation = Arc::new(store::ArtifactEphemeralTaskPreparationFactory::new(footprint, create_task, state.clone(), mutation.clone()));
    semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
}

fn create_task(base: &WriterMainWindowTransient, _: &WriterMainWindowTransientMutation) -> Result<Box<dyn ArtifactEphemeralPreparationTask<WriterMainWindowTransient, WriterMainWindowTransientMutation>>, String> {
    if base.engagement_input.capacity() > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES { return Err("Writer preserved engagement input exceeds retained capacity".into()); }
    Ok(Box::new(WriterPreparation { bytes: ManuallyDrop::new(Vec::new()), reserved: false, checkpoint: ArtifactStoreOneItemCheckpoint::default() }))
}

struct WriterPreparation {
    bytes: ManuallyDrop<Vec<u8>>,
    reserved: bool,
    checkpoint: ArtifactStoreOneItemCheckpoint,
}

impl ArtifactEphemeralPreparationTask<WriterMainWindowTransient, WriterMainWindowTransientMutation> for WriterPreparation {
    /// 🧵️ Copied bytes are a complete, unchanged immutable String before unchecked conversion; every turn pays its reservation, copy page and root allocation from its own grant.
    fn advance(&mut self, base: &WriterMainWindowTransient, mutation: &mut Option<WriterMainWindowTransientMutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactEphemeralPreparationTaskStep<WriterMainWindowTransient>, ValueError> {
        if grant.maximum_items == 0 { return Ok(ArtifactEphemeralPreparationTaskStep::Blocked); }
        let Some(operation) = mutation.as_ref() else { return Ok(ArtifactEphemeralPreparationTaskStep::Blocked) };
        let copy_text = !matches!(operation, WriterMainWindowTransientMutation::SetEngagementInput(_));
        let length = base.engagement_input.len();
        if copy_text && !self.reserved {
            if grant.maximum_capacity_bytes < length { return Ok(ArtifactEphemeralPreparationTaskStep::Blocked); }
            self.bytes.try_reserve_exact(length).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "Writer preserved input buffer allocation failed"))?;
            self.reserved = true;
            return Ok(ArtifactEphemeralPreparationTaskStep::Progress(self.checkpoint, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: self.bytes.capacity(), ..Default::default() }));
        }
        if copy_text && self.bytes.len() < length {
            if grant.maximum_copy_bytes == 0 { return Ok(ArtifactEphemeralPreparationTaskStep::Blocked); }
            let start = self.bytes.len();
            let end = start + grant.maximum_copy_bytes.min(length - start);
            self.bytes.extend_from_slice(&base.engagement_input.as_bytes()[start..end]);
            self.checkpoint.cursor = end as u32;
            self.checkpoint.completed_bytes += (end - start) as u64;
            return Ok(ArtifactEphemeralPreparationTaskStep::Progress(self.checkpoint, RetainedCloneProgress { copied_items: 1, copied_bytes: end - start, ..Default::default() }));
        }
        let root_bytes = semio_framework_value::shared_retirement_allocation_bytes::<WriterMainWindowTransient>();
        if grant.maximum_capacity_bytes < root_bytes { return Ok(ArtifactEphemeralPreparationTaskStep::Blocked); }
        let operation = mutation.take().expect("Writer construction retains its mutation until root transfer");
        let mut root = WriterMainWindowTransient { editor_selection: base.editor_selection.clone(), lint_generation: base.lint_generation, engagement_input: String::new() };
        match operation {
            WriterMainWindowTransientMutation::SetEditorSelection(value) => root.editor_selection = value.selection,
            WriterMainWindowTransientMutation::SetLintGeneration(value) => root.lint_generation = value.value,
            WriterMainWindowTransientMutation::SetEngagementInput(value) => root.engagement_input = value.value,
        }
        if copy_text { root.engagement_input = unsafe { String::from_utf8_unchecked(std::mem::take(&mut *self.bytes)) }; }
        self.checkpoint.completed_items = 1;
        Ok(ArtifactEphemeralPreparationTaskStep::Prepared { root: Arc::new(root), checkpoint: self.checkpoint, ownership: RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: root_bytes, ..Default::default() } })
    }

    fn begin_close(&mut self) {}
}

impl ErasedSnapshotRetirement for WriterPreparation {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let bytes = self.bytes.capacity();
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        drop(std::mem::take(&mut *self.bytes));
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool { self.bytes.is_empty() && self.bytes.capacity() == 0 }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.bytes.capacity()) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(!self.terminal_is_empty())) }
}

impl Drop for WriterPreparation {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.bytes.is_empty() && self.bytes.capacity() == 0), "Writer construction dropped before byte-buffer retirement"); }
}
