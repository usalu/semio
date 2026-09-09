//! 📢️ Writer field updates construct preserved text incrementally under Store ownership.

use super::{WriterEditorSelection, WriterMainWindowTransient, WriterMainWindowTransientMutation};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ArtifactEphemeralPreparationTask, ArtifactEphemeralPreparationTaskStep, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint,
    ArtifactStoreOneItemGrant, SnapshotRetirementStep, retirement::RetireOwned};

impl RetireOwned for WriterEditorSelection {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> { store::retirement::leaf((self.start, self.end)) }
}

impl RetireOwned for WriterMainWindowTransient {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::sequence(vec![self.editor_selection.retirement(), self.lint_generation.retirement(), self.engagement_input.retirement()])
    }
}

impl RetireOwned for WriterMainWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::SetEditorSelection(value) => value.selection.retirement(),
            Self::SetLintGeneration(value) => value.value.retirement(),
            Self::SetEngagementInput(value) => value.value.retirement(),
        }
    }
}

fn footprint(mutation: &WriterMainWindowTransientMutation) -> Result<ArtifactStoreOneItemFootprint, String> {
    let capacity = match mutation { WriterMainWindowTransientMutation::SetEngagementInput(value) => value.value.capacity(), _ => 0 };
    if capacity > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES { return Err("Writer engagement input exceeds retained capacity".into()); }
    Ok(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
}

pub(super) fn owners() -> semio_framework_plugin::WindowTransientOwnerBundle<WriterMainWindowTransient, WriterMainWindowTransientMutation> {
    let state: Arc<dyn store::ArtifactOwnedValueRetirementFactory<WriterMainWindowTransient>> = Arc::new(store::retirement::OwnedValueRetirementFactory::default());
    let mutation: Arc<dyn store::ArtifactOwnedValueRetirementFactory<WriterMainWindowTransientMutation>> = Arc::new(store::retirement::OwnedValueRetirementFactory::default());
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
    /// 🧵️ Copied bytes are a complete, unchanged immutable String before unchecked conversion.
    fn advance(&mut self, base: &WriterMainWindowTransient, mutation: &mut Option<WriterMainWindowTransientMutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactEphemeralPreparationTaskStep<WriterMainWindowTransient>, String> {
        if grant.maximum_items == 0 { return Ok(ArtifactEphemeralPreparationTaskStep::Blocked); }
        let Some(operation) = mutation.as_ref() else { return Ok(ArtifactEphemeralPreparationTaskStep::Blocked) };
        let copy_text = !matches!(operation, WriterMainWindowTransientMutation::SetEngagementInput(_));
        if copy_text && !self.reserved {
            self.bytes.try_reserve_exact(base.engagement_input.len()).map_err(|_| "Writer preserved input buffer allocation failed".to_string())?;
            self.reserved = true;
            return Ok(ArtifactEphemeralPreparationTaskStep::Progress(self.checkpoint));
        }
        if copy_text && self.bytes.len() < base.engagement_input.len() {
            if grant.maximum_bytes == 0 { return Ok(ArtifactEphemeralPreparationTaskStep::Blocked); }
            let start = self.bytes.len();
            let end = start + grant.maximum_bytes.min(base.engagement_input.len() - start);
            self.bytes.extend_from_slice(&base.engagement_input.as_bytes()[start..end]);
            self.checkpoint.cursor = end as u32;
            self.checkpoint.completed_bytes += (end - start) as u64;
            return Ok(ArtifactEphemeralPreparationTaskStep::Progress(self.checkpoint));
        }
        let operation = mutation.take().expect("Writer construction retains its mutation until root transfer");
        let mut root = WriterMainWindowTransient { editor_selection: base.editor_selection.clone(), lint_generation: base.lint_generation, engagement_input: String::new() };
        match operation {
            WriterMainWindowTransientMutation::SetEditorSelection(value) => root.editor_selection = value.selection,
            WriterMainWindowTransientMutation::SetLintGeneration(value) => root.lint_generation = value.value,
            WriterMainWindowTransientMutation::SetEngagementInput(value) => root.engagement_input = value.value,
        }
        if copy_text { root.engagement_input = unsafe { String::from_utf8_unchecked(std::mem::take(&mut *self.bytes)) }; }
        self.checkpoint.completed_items = 1;
        Ok(ArtifactEphemeralPreparationTaskStep::Prepared { root, checkpoint: self.checkpoint })
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if grant.maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if !self.bytes.is_empty() {
            let released = grant.maximum_bytes.min(self.bytes.len());
            let next = self.bytes.len() - released;
            self.bytes.truncate(next);
            return Ok(SnapshotRetirementStep::Pending { released_items: usize::from(released != 0), released_bytes: released });
        }
        if self.bytes.capacity() != 0 {
            drop(std::mem::take(&mut *self.bytes));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool { self.bytes.is_empty() && self.bytes.capacity() == 0 }
}

impl Drop for WriterPreparation {
    fn drop(&mut self) { assert!(self.bytes.is_empty() && self.bytes.capacity() == 0, "Writer construction dropped before byte-buffer retirement"); }
}
