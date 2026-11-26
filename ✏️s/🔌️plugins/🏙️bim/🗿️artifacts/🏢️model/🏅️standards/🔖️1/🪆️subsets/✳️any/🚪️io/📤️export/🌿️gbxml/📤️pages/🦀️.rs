//! 📤️ Writes a borrowed gbXML tree through the mounted caller's existing segmented queue.

use super::{GbxmlTextCursor, GbxmlTreeSource};
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind, retirement::controlled::ControlledRetirement};
use semio_plugin::app::ArtifactOutputChunks;

const PAGE: usize = ArtifactOutputChunks::CHUNK_BYTES;
const LEXICAL_WORK: usize = 512;

#[derive(semio_framework_value::RetireOwned, Default)]
struct Output {
    chunks: Option<ArtifactOutputChunks>,
    page: Option<Vec<u8>>,
}

/// 📑️ Holds only the admitted output queue, a page and fixed XML traversal state.
pub struct GbxmlPages {
    output: ControlledRetirement<Output>,
    cursor: GbxmlTextCursor,
    sealed: bool,
}

impl Default for GbxmlPages {
    fn default() -> Self {
        Self { output: ControlledRetirement::retain(Output::default()), cursor: Default::default(), sealed: false }
    }
}

impl GbxmlPages {
    /// 📨️ Receives the caller's original page queue under that same source turn.
    pub fn receive(&mut self, original: &mut Option<ArtifactOutputChunks>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?;
        let output = self.output.original_mut().ok_or_else(|| ValueError::invariant("gbXML output is already retiring"))?;
        if output.chunks.is_some() { return Ok(true); }
        if original.is_none() { return Err(ValueError::invariant("gbXML output queue is absent").into()); }
        let copy = 2 * size_of::<Option<ArtifactOutputChunks>>();
        if !cx.charge_retained_item(copy, 0, 0)? { return Ok(false); }
        output.chunks = original.take();
        Ok(true)
    }

    /// 🧮️ Quotes only the next physical page operation.
    pub fn work_demand(&self) -> RetirementDemand {
        let Some(output) = self.output.original() else { return Default::default(); };
        if self.sealed {
            return RetirementDemand { copy_bytes: 2 * size_of::<Option<ArtifactOutputChunks>>(), depth: 1, ..Default::default() };
        }
        if self.ready_page(output) {
            return ArtifactOutputChunks::original_push_demands();
        }
        if self.cursor.is_complete() {
            return ArtifactOutputChunks::original_seal_demands();
        }
        if output.page.is_none() {
            return RetirementDemand { capacity_bytes: PAGE, depth: 1, ..Default::default() };
        }
        RetirementDemand { copy_bytes: 1, depth: 1, ..Default::default() }
    }

    fn ready_page(&self, output: &Output) -> bool {
        output.page.as_ref().is_some_and(|page| page.len() == PAGE || self.cursor.is_complete() && !page.is_empty())
    }

    /// 🧾️ Fills, gives away or seals one bounded page under the original command caller.
    pub fn advance(&mut self, source: &impl GbxmlTreeSource, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?;
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "gbXML export was cancelled").into());
        }
        if self.sealed { return Ok(true); }
        let demand = self.work_demand();
        if !cx.retained_work_can_enter(demand) { return Ok(false); }
        let ready = self.output.original().is_some_and(|output| self.ready_page(output));
        let output = self.output.original_mut().ok_or_else(|| ValueError::invariant("gbXML output is already retiring"))?;
        let chunks = output.chunks.as_ref().ok_or_else(|| ValueError::invariant("gbXML output queue is absent"))?;
        if ready {
            chunks.push_original(&mut output.page, None, cx)?;
        } else if self.cursor.is_complete() {
            self.sealed = chunks.seal_original(cx)?;
        } else if let Some(page) = output.page.as_mut() {
            let room = (PAGE - page.len()).min(cx.retained_grant().maximum_copy_bytes);
            if room == 0 || !cx.charge_work_unit() { return Ok(false); }
            let mut bytes = [0; PAGE];
            let mut next = self.cursor.clone();
            let count = next.write_source(source, &mut bytes[..room], LEXICAL_WORK)
                .map_err(|message| ValueError::literal(ValueRefusalKind::InvalidValue, message))?;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() })?;
            page.extend_from_slice(&bytes[..count]);
            self.cursor = next;
        } else if cx.charge_work_unit() {
            let mut page = Vec::new();
            page.try_reserve_exact(PAGE).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "gbXML page allocation failed"))?;
            let capacity = page.capacity();
            output.page = Some(page);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() })?;
        }
        Ok(self.sealed)
    }

    /// 🎁️ Gives the sealed original queue back to the command's funded publication turn.
    pub fn take(&mut self, cx: &mut StepContext<'_>) -> Result<Option<ArtifactOutputChunks>, OriginalTurnFailure> {
        cx.original_source_parent()?;
        let copy = self.work_demand().copy_bytes;
        let output = self.output.original_mut().ok_or_else(|| ValueError::invariant("gbXML output is already retiring"))?;
        if !self.sealed || output.chunks.is_none() || !cx.charge_retained_item(copy, 0, 0)? { return Ok(None); }
        Ok(output.chunks.take())
    }

    /// ♻️ Quotes retirement of the one original page or queue position selected for this turn.
    pub fn close_demand(&self, copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.output.selected_demand(copy_bytes)
    }

    /// 🔚️ Reports whether all output owners have been returned.
    pub fn terminal_is_empty(&self) -> bool { self.output.terminal_is_empty() }

    /// 📥️ Pays for and performs a single original output retirement turn.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let demand = self.close_demand(cx.retained_grant().maximum_copy_bytes)?;
        semio_framework_job::close_original_effect_step(cx, demand, |effect| self.output.step(effect.grant().maximum_copy_bytes, effect))
    }
}
