//! 🎞️ Moves the authenticated picker source into the existing PNG raster decoder.

use crate::{guest::OriginalGuestMediaInput, ingress::OriginalMediaLoan, receiver::MediaOrigin};
use semio_framework_job::{InteractiveJobCloseStep, OriginalSourceBinding, OriginalTurnFailure, StepContext, close_original_effect_step};
use semio_framework_pixels_codecs::{RasterImage, png_decoding::{PngDecodeJob, PngDecodeLimits, PngDecodeProgress}};
use semio_framework_value::{
    RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind, paged::PagedBytes,
    retained_clone::retained_clone_grant_admits, retirement::controlled::ControlledRetirement,
};

type Decoder<const N: usize> = Option<PngDecodeJob<PagedBytes<N>>>;

pub struct RawPngWork<T, const N: usize> {
    decoder: Option<ControlledRetirement<Decoder<N>>>,
    target: Option<T>,
    source: Option<OriginalSourceBinding>,
    origin: Option<MediaOrigin>,
    closing: bool,
}

fn refusal() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Local PNG import could not decode its original source")
}

impl<T: Copy, const N: usize> RawPngWork<T, N> {
    pub fn empty() -> Self {
        Self { decoder: None, target: None, source: None, origin: None, closing: false }
    }

    pub fn birth_demand() -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() }
    }

    fn job(&self) -> Option<&PngDecodeJob<PagedBytes<N>>> {
        self.decoder.as_ref()?.original()?.as_ref()
    }

    fn check_source(&self, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if self.source.is_some_and(|source| !source.matches(cx)) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "PNG decoder changed its Original caller Source").into());
        }
        Ok(())
    }

    pub fn demand(&self) -> Result<RetirementDemand, ValueError> {
        let Some(job) = self.job() else { return Ok(Self::birth_demand()); };
        Ok(RetirementDemand {
            copy_bytes: job.next_copy_byte_demand().map_err(|_| refusal())?,
            capacity_bytes: job.next_capacity_byte_demand().map_err(|_| refusal())?,
            depth: job.next_depth_demand().map_err(|_| refusal())?,
            ..Default::default()
        })
    }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.decoder.as_ref().map_or(Ok(Default::default()), |owner| owner.next_demand(body))
    }

    fn check_live(&mut self, live: OriginalMediaLoan<'_>) -> Result<(), OriginalTurnFailure> {
        if self.origin.is_some_and(|origin| !origin.matches(live)) {
            self.closing = true;
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "PNG decoder lost its captured PDF revision").into());
        }
        Ok(())
    }

    /// 🔑 Revalidates the full live revision and source on every format turn.
    pub fn advance(
        &mut self, input: &mut OriginalGuestMediaInput<T, N>, live: OriginalMediaLoan<'_>,
        limits: PngDecodeLimits, cx: &mut StepContext<'_>,
    ) -> Result<Option<PngDecodeProgress>, OriginalTurnFailure> {
        self.check_source(cx)?;
        self.check_live(live)?;
        if self.closing { return Err(ValueError::literal(ValueRefusalKind::Canceled, "PNG decoder is closing").into()); }
        let received = match input.current_input(live, cx) {
            Ok(Some(received)) => received,
            Ok(None) => return Ok(None),
            Err(error) => { self.closing = true; return Err(error); }
        };
        if self.decoder.is_none() {
            let demand = Self::birth_demand();
            let grant = cx.retained_grant();
            let work = 9 + 256 * 4;
            if cx.fuel_remaining() < work || !retained_clone_grant_admits(grant, demand) { return Ok(None); }
            let owner = ControlledRetirement::new(None::<PngDecodeJob<PagedBytes<N>>>)
                .map_err(|(error, _)| error)?;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(work);
            self.origin = Some(MediaOrigin::capture(live)?);
            self.source = Some(OriginalSourceBinding::borrow(cx));
            self.target = Some(received.target);
            self.decoder = Some(owner);
            let owner = self.decoder.as_mut().expect("retained PNG decoder");
            let admitted = PngDecodeJob::admit_paged(&mut received.bytes, limits);
            if let Ok(job) = admitted {
                *owner.original_mut().expect("live PNG decoder workspace") = Some(job);
                return Ok(self.job().map(PngDecodeJob::progress));
            }
            self.closing = true;
            return Err(refusal().into());
        }
        let job = self.decoder.as_mut().and_then(ControlledRetirement::original_mut)
            .and_then(Option::as_mut).ok_or_else(refusal)?;
        while !cx.should_yield() && !job.progress().done {
            let mut grant = cx.retained_grant();
            if grant.maximum_items == 0 { break; }
            grant.maximum_items = 1;
            let Some((result, performed)) = cx.with_original_source_effect(grant, |_, effect| job.advance(effect))? else { break; };
            cx.consume_fuel(u64::try_from(performed).map_err(|_| ValueError::invariant("PNG decode work exceeded its admitted turn"))?);
            let receipt = job.last_turn_progress();
            cx.consume_retained(receipt)?;
            if result.is_err() { self.closing = true; return Err(refusal().into()); }
            if receipt.copied_items == 0 { break; }
        }
        Ok(Some(job.progress()))
    }

    fn completed_job<'a>(
        &'a mut self, input: &mut OriginalGuestMediaInput<T, N>, live: OriginalMediaLoan<'_>, cx: &StepContext<'_>,
    ) -> Result<Option<(T, &'a PngDecodeJob<PagedBytes<N>>)>, OriginalTurnFailure> {
        self.check_source(cx)?;
        self.check_live(live)?;
        if self.closing { return Ok(None); }
        match input.current_input(live, cx) {
            Ok(Some(_)) => {}
            Ok(None) => return Ok(None),
            Err(error) => { self.closing = true; return Err(error); }
        }
        let Some(job) = self.job().filter(|job| job.progress().done) else { return Ok(None); };
        Ok(Some((self.target.expect("PNG decoder retains its original placement"), job)))
    }

    /// 🖼️ Makes the raster and original target visible together only under the current picker loan.
    pub fn raster<'a>(
        &'a mut self, input: &mut OriginalGuestMediaInput<T, N>, live: OriginalMediaLoan<'_>, cx: &StepContext<'_>,
    ) -> Result<Option<(T, &'a RasterImage)>, OriginalTurnFailure> {
        let Some((target, job)) = self.completed_job(input, live, cx)? else { return Ok(None); };
        Ok(Some((target, job.result().map_err(|_| refusal())?)))
    }

    /// 🧾 Lends the exact encoded PNG pages after validation of the live picker loan and caller.
    pub fn source<'a>(
        &'a mut self, input: &mut OriginalGuestMediaInput<T, N>, live: OriginalMediaLoan<'_>, cx: &StepContext<'_>,
    ) -> Result<Option<&'a PagedBytes<N>>, OriginalTurnFailure> {
        Ok(self.completed_job(input, live, cx)?.map(|(_, job)| job.paged_source()))
    }

    /// ♻️ Closes the sampled result, working decoder, and original encoded pages under the same caller.
    pub fn close(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.check_source(cx)?;
        self.closing = true;
        let Some(owner) = self.decoder.as_mut().filter(|owner| !owner.terminal_is_empty()) else {
            return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() });
        };
        let body = cx.retained_grant().maximum_copy_bytes;
        close_original_effect_step(cx, owner.next_demand(body)?, |effect| owner.step(body, effect))
    }
}
