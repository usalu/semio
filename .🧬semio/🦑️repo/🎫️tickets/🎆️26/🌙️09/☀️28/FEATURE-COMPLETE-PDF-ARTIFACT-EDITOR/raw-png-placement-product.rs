//! 🧾 Binds a typed PDF edit to exactly one private original PNG placement Pack.
use super::{
    check_image_placement, ImagePlacement,
    plane_identity::PlaneFacts, plane_rows::PngPlacementRows,
    raw_pack::{OriginalPack, RawPngPlacementPack},
};
use semio_plugin::__job::{OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_value::{
    OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError,
    ValueRefusalKind, paged::ByteReadSource, retained_clone::retained_clone_grant_admits,
    retirement::controlled::ControlledRetirement,
};
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{
    mutations::{AppendPageContent, PdfMutation, ReplaceContent},
    snapshot::{PdfOp, PdfSnapshot},
};

pub const PDF_RAW_PNG_ARCHIVE_BYTES: usize = 32768;

fn fault(kind: ValueRefusalKind, message: &'static str) -> ValueError { ValueError::literal(kind, message) }

/// 📖 Holds the canonical short control record beside the exact operations that authored it.
pub struct PdfRawPngRows {
    rows: ControlledRetirement<PngPlacementRows>,
    record: ControlledRetirement<Vec<u8>>,
}

impl PdfRawPngRows {
    /// 🧮 Builds the bounded domain control in the registered row-encoding preparation lane.
    pub fn new(rows: PngPlacementRows) -> Result<Self, ValueError> {
        let record = rows.encode().map_err(|_| fault(ValueRefusalKind::InvalidValue, "PDF PNG placement control could not encode its typed rows"))?;
        Ok(Self {
            rows: ControlledRetirement::new(rows).map_err(|(error, _)| error)?,
            record: ControlledRetirement::new(record).map_err(|(error, _)| error)?,
        })
    }

    pub fn record(&self) -> Option<&[u8]> { self.record.original().map(Vec::as_slice) }

    /// 📤 Moves the already prepared rows into their funded mounted mutation-page owner.
    pub fn donate_rows(
        &mut self, output: &mut Option<PngPlacementRows>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        let demand = RetirementDemand { copy_bytes: 2 * size_of::<Option<PngPlacementRows>>(), depth: 1, ..Default::default() };
        if output.is_some() || !self.rows.original_is_untouched() || !retained_clone_grant_admits(effect.grant(), demand)
            || !effect.performed(1)?
        { return Ok(Default::default()); }
        let rows = self.rows.take_original().ok_or_else(|| fault(ValueRefusalKind::InvariantViolated, "PDF PNG transaction lost its typed rows"))?;
        *output = Some(rows);
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.record.terminal_is_empty() { self.record.next_demand(body) } else { self.rows.next_demand(body) }
    }

    pub fn retire(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        if !self.record.terminal_is_empty() { self.record.step(body, effect) } else { self.rows.step(body, effect) }
    }

    pub fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() && self.record.terminal_is_empty() }
}

/// 🔏 Joins the native resource and all typed placement rows before any publication is observable.
pub struct PdfRawPngPublication {
    source: OriginalSourceBinding,
    revision: [u8; 32],
    revoked: bool,
    rows: PdfRawPngRows,
    pack: OriginalPack<PDF_RAW_PNG_ARCHIVE_BYTES>,
}

impl PdfRawPngPublication {
    /// 📥 Transfers both owners into a single private publication slot with one placement quote.
    pub fn admit(
        pack: &mut RawPngPlacementPack<PDF_RAW_PNG_ARCHIVE_BYTES>,
        rows: &mut Option<PdfRawPngRows>,
        live: &[u8; 32], cx: &StepContext<'_>, output: &mut Option<Self>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        let canceled = cx.is_cancelled();
        let Some((source, control)) = rows.as_ref().and_then(|rows| rows.rows.original().zip(rows.record())) else { return Ok(Default::default()); };
        if canceled || source.revision() != live {
            pack.revoke();
            return Err(fault(ValueRefusalKind::Canceled, "PDF PNG placement lost its live snapshot").into());
        }
        if !pack.matches_control(control) { return Err(fault(ValueRefusalKind::InvalidValue, "PDF PNG placement Pack and typed operation rows differ").into()); }
        let extra = size_of::<Option<Self>>() + size_of::<Option<PdfRawPngRows>>();
        let quote = RetirementDemand { copy_bytes: extra + 2 * size_of::<Option<OriginalPack<PDF_RAW_PNG_ARCHIVE_BYTES>>>(),
            depth: 1, ..Default::default() };
        if output.is_some() || !retained_clone_grant_admits(effect.grant(), quote) { return Ok(Default::default()); }
        let mut original = None;
        let mut receipt = pack.take(live, canceled, &mut original, effect)?;
        let Some(original) = original else { return Ok(receipt); };
        receipt.copied_bytes += extra;
        *output = Some(Self { source: OriginalSourceBinding::borrow(cx), revision: *live, revoked: false, pack: original, rows: rows.take().expect("admitted PNG placement rows") });
        Ok(receipt)
    }

    /// 🎯 Offers an exact typed mutation and native archive cohort to the mounted document transaction.
    pub fn original(
        &mut self, live: &[u8; 32], cx: &StepContext<'_>,
    ) -> Option<(&PngPlacementRows, &dyn ByteReadSource)> {
        if !self.source.matches(cx) { return None; }
        self.revoked |= cx.is_cancelled() || live != &self.revision;
        if self.revoked { return None; }
        Some((self.rows.rows.original()?, self.pack.original()?))
    }

    /// 🧳 Lets only the authenticated mounted transaction acquire both original owners.
    pub fn into_owners(mut self, cx: &StepContext<'_>, live: &[u8; 32]) -> Result<(PdfRawPngRows, OriginalPack<PDF_RAW_PNG_ARCHIVE_BYTES>), Self> {
        if self.original(live, cx).is_none() { return Err(self); }
        Ok((self.rows, self.pack))
    }

    /// ♻️ Refused and canceled installs retain both owners until their controlled close is terminal.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.rows.terminal_is_empty() { self.rows.close_demand(body) } else { self.pack.next_demand(body) }
    }

    pub fn retire(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        self.revoked = true;
        if !self.rows.terminal_is_empty() { self.rows.retire(body, effect) } else { self.pack.step(body, effect) }
    }

    pub fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() && self.pack.terminal_is_empty() }
}

/// 🎨 Uses the exact typed insert or replacement shape registered by the PDF editor.
pub fn prepare_raw_png_rows(
    snapshot: &PdfSnapshot, placement: ImagePlacement, revision: [u8; 32],
    facts: PlaneFacts, operation: u64, generation: u64, instance: u32,
) -> Result<PngPlacementRows, ValueError> {
    let page = check_image_placement(snapshot, placement)
        .map_err(|_| fault(ValueRefusalKind::InvalidValue, "PDF PNG placement target no longer exists"))?;
    let color = format!("SemioImage{instance:08x}{operation:016x}{generation:016x}");
    let mask = facts.shape.has_alpha().then(|| format!("{color}Mask"));
    let paint = PdfOp::PaintXObject { name: color.clone() };
    let paint = match placement {
        ImagePlacement::Insert { matrix, .. } => PdfMutation::AppendPageContent(AppendPageContent {
            index: page, content: vec![PdfOp::Save, PdfOp::Transform { matrix }, paint, PdfOp::Restore],
        }),
        ImagePlacement::Replace { operator, .. } => PdfMutation::ReplaceContent(ReplaceContent { index: page, at: operator, op: paint }),
    };
    PngPlacementRows::prepare(revision, snapshot, facts, &color, mask.as_deref(), paint)
        .map_err(|_| fault(ValueRefusalKind::InvalidValue, "PDF PNG placement could not stage its exact SetImage rows"))
}
