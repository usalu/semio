//! 🕯️ Owns one bounded, unpublished JPEG preview under the renderer’s retained allocation authority.

use crate::{BaselineJpgMetadata, JpgByteSource, preview::{BaselineJpgPreview, JpgPreviewSize}};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind,
    paged::PagedBytes, retained_clone::retained_clone_grant_admits, retirement::controlled::ControlledRetirement};
use semio_framework_value_persistent::list::{PagedListReservationFailure, PagedListReservationOutcome};

type Samples = PagedBytes<{256 * 256 * 3}>;
const MAX_BLOCK_WRITES: usize = 1024;

pub struct RetainedJpgPreview {
    size: JpgPreviewSize,
    decoder: BaselineJpgPreview,
    samples: Option<ControlledRetirement<Samples>>,
    result: Option<BaselineJpgMetadata>,
    closing: bool,
}

impl RetainedJpgPreview {
    /// 🪴️ Opens an empty paged owner after the renderer has admitted its source length and dimensions.
    pub fn new(length: usize, size: JpgPreviewSize) -> Result<Self, ValueError> {
        let decoder = BaselineJpgPreview::new(length, size).map_err(invalid)?;
        let samples = ControlledRetirement::new(Samples::default()).map_err(|(error, _)| error)?;
        Ok(Self { size, decoder, samples: Some(samples), result: None, closing: false })
    }

    /// ⏱️ Reserves one backing, initializes a funded range, or decodes one bounded coefficient block.
    pub fn step(&mut self, source: &dyn JpgByteSource, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        if self.closing || self.result.is_some() { return Ok(Default::default()); }
        let samples = self.samples.as_mut().and_then(ControlledRetirement::original_mut)
            .ok_or_else(|| ValueError::invariant("JPEG preview has no sample owner"))?;
        let JpgPreviewSize { display: [width, height], channels, .. } = self.size;
        let extent = width * height * usize::from(channels);
        let grant = effect.grant();
        if grant.maximum_items == 0 { return Ok(Default::default()); }
        if samples.len() < extent {
            let backing = samples.retained_bytes().capacity() - samples.len();
            if backing == 0 {
                let demand = samples.byte_backing_demand()?;
                if !retained_clone_grant_admits(grant, demand) { return Ok(Default::default()); }
                return match samples.retained_bytes_mut().reserve_one_funded(effect).result {
                    Ok(PagedListReservationOutcome::Ready(progress)) => Ok(progress),
                    Ok(PagedListReservationOutcome::Denied) => Ok(Default::default()),
                    Err(PagedListReservationFailure::Work(error)) => Err(error),
                    Err(PagedListReservationFailure::Allocation { error, .. }) => Err(ValueError::from(error.refusal())
                        .with_retained_progress(RetainedCloneProgress { retained_capacity_bytes: error.allocated_bytes, ..Default::default() })),
                };
            }
            let depth = samples.retained_bytes().next_push_depth_demand()?;
            let count = backing.min(extent - samples.len()).min(grant.maximum_copy_bytes)
                .min(usize::try_from(effect.remaining_work_units()?).unwrap_or(usize::MAX));
            if count == 0 || depth > grant.maximum_depth || !effect.performed(count as u64)? { return Ok(Default::default()); }
            for written in 0..count {
                samples.push_reserved_byte(0).map_err(|error| error.with_retained_progress(RetainedCloneProgress {
                    copied_items: usize::from(written > 0), copied_bytes: written, ..Default::default()
                }))?;
            }
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() });
        }
        let depth = samples.retained_bytes().get_source_work_demand(extent - 1)? as usize;
        let demand = RetirementDemand { copy_bytes: MAX_BLOCK_WRITES, depth, ..Default::default() };
        if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok(Default::default()); }
        let mut copied_bytes = 0;
        let result = self.decoder.step(source, &mut |x, y, channel, value| {
            let offset = (y * width + x) * usize::from(channels) + channel;
            *samples.retained_bytes_mut().get_mut(offset).expect("admitted JPEG sample slot") = value;
            copied_bytes += 1;
        });
        assert!(copied_bytes <= MAX_BLOCK_WRITES);
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() };
        self.result = result.map_err(|error| invalid(error).with_retained_progress(progress))?;
        Ok(progress)
    }

    pub fn rendered(&self) -> Option<(BaselineJpgMetadata, &Samples)> {
        (!self.closing).then_some(self.result).flatten()
            .and_then(|metadata| self.samples.as_ref()?.original().map(|samples| (metadata, samples)))
    }

    /// 🧹️ Withdraws the preview and retires its actual private backing under the caller’s grant.
    pub fn close(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Option<semio_framework_value::OriginalRetirementPoll> {
        self.closing = true;
        self.samples.as_mut().map(|samples| samples.step(body, effect))
    }

    pub fn closed(&self) -> bool {
        self.closing && self.samples.as_ref().is_none_or(ControlledRetirement::terminal_is_empty)
    }
}

fn invalid(error: crate::JpgError) -> ValueError {
    use crate::JpgError;
    let text = match error {
        JpgError::Malformed(text) | JpgError::Unsupported(text) => text,
    };
    ValueError::new(ValueRefusalKind::InvalidValue, text)
}
