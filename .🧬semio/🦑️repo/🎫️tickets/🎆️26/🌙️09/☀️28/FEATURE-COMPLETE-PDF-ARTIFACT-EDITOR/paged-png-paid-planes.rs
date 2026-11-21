//! 🛬️ Funds decoded PNG color and alpha custody ahead of each decoder transition.

use crate::png::{PngPlaneTarget, PngRefusal, PngSampleCursor};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind,
    paged::{ByteReadSource, PagedBytes}, retained_clone::retained_clone_grant_admits};
use semio_framework_value_persistent::list::{PagedListReservationFailure, PagedListReservationOutcome};

pub struct PngPaidPlanes<const N: usize> {
    samples: PngSampleCursor,
    output: (PagedBytes<N>, PagedBytes<N>),
    sealed: bool,
    closing: bool,
}

impl<const N: usize> PngPaidPlanes<N> {
    /// 🧾️ Creates the empty destination under its original source proof.
    pub fn new(samples: PngSampleCursor) -> Result<Self, ValueError> {
        let extent = samples.extent();
        let pixels = extent.width as usize * extent.height as usize;
        if pixels.checked_mul(usize::from(extent.color_samples)).is_none_or(|count| count > N) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "PNG planes exceed the admitted sample limit"));
        }
        Ok(Self { samples, output: Default::default(), sealed: false, closing: false })
    }

    /// 📥️ Reserves the next immutable plane slot or advances one original decoder transition.
    pub fn advance(&mut self, source: &dyn ByteReadSource, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        if self.closing || self.sealed { return Ok(Default::default()); }
        let grant = effect.grant();
        if grant.maximum_items == 0 { return Ok(Default::default()); }
        let history = self.samples.next_history_capacity().unwrap_or(0);
        let lane = self.samples.next_output_alpha();
        let selected = match lane {
            Some(false) => Some(&mut self.output.0),
            Some(true) => Some(&mut self.output.1),
            None => None,
        };
        if let Some(selected) = selected.filter(|plane| history == 0 && !plane.retained_bytes().has_reserved_slot()) {
            if !retained_clone_grant_admits(grant, selected.byte_backing_demand()?) { return Ok(Default::default()); }
            return match selected.retained_bytes_mut().reserve_one_funded(effect).result {
                Ok(PagedListReservationOutcome::Ready(progress)) => Ok(progress),
                Ok(PagedListReservationOutcome::Denied) => Ok(Default::default()),
                Err(PagedListReservationFailure::Work(error)) => Err(error),
                Err(PagedListReservationFailure::Allocation { error, .. }) => Err(ValueError::from(error.refusal())
                    .with_retained_progress(RetainedCloneProgress { retained_capacity_bytes: error.allocated_bytes, ..Default::default() })),
            };
        }
        let pages = self.output.0.len().max(self.output.1.len()).saturating_add(1).div_ceil(4096);
        let depth = pages.ilog2() as usize + 4;
        let demand = RetirementDemand { copy_bytes: usize::from(lane.is_some()), capacity_bytes: history, depth, ..Default::default() };
        if !retained_clone_grant_admits(grant, demand) || history > 0 && effect.actual().is_some() || !effect.performed(1)? {
            return Ok(Default::default());
        }
        let before = self.samples.history_capacity();
        let channels = usize::from(self.samples.extent().color_samples);
        let mut output = PlaneLoan { planes: &mut self.output, channels, copied_bytes: 0 };
        let next = self.samples.step(source, &mut output, grant.maximum_capacity_bytes);
        let allocated = self.samples.history_capacity().saturating_sub(before);
        if allocated > 0 { *effect.actual() = Some(allocated); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: output.copied_bytes, retained_capacity_bytes: allocated, ..Default::default() };
        self.sealed = next.map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "PNG source could not yield its exact plane samples").with_retained_progress(progress))?;
        Ok(progress)
    }

    pub fn rendered(&self) -> Option<(&PagedBytes<N>, &PagedBytes<N>)> {
        (self.sealed && !self.closing).then_some((&self.output.0, &self.output.1))
    }

    pub fn stop(&mut self) { self.closing = true; }

    pub fn into_closing(mut self) -> (PngSampleCursor, (PagedBytes<N>, PagedBytes<N>)) {
        self.stop();
        (self.samples, self.output)
    }
}

struct PlaneLoan<'a, const N: usize> {
    planes: &'a mut (PagedBytes<N>, PagedBytes<N>),
    channels: usize,
    copied_bytes: usize,
}

impl<const N: usize> PngPlaneTarget for PlaneLoan<'_, N> {
    fn previous(&self, pixel: usize, channel: usize) -> Option<u8> {
        if channel == self.channels { self.planes.1.byte_at(pixel) } else { self.planes.0.byte_at(pixel * self.channels + channel) }
    }

    fn put(&mut self, pixel: usize, channel: usize, value: u8) -> Result<(), PngRefusal> {
        let (target, index) = if channel == self.channels { (&mut self.planes.1, pixel) }
            else { (&mut self.planes.0, pixel * self.channels + channel) };
        if target.len() != index { return Err(PngRefusal::SampleTarget); }
        target.push_reserved_byte(value).map_err(|_| PngRefusal::SampleTarget)?;
        self.copied_bytes += 1;
        Ok(())
    }
}
