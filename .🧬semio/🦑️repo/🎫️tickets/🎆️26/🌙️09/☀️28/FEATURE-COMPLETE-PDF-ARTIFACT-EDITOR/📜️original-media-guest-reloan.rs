//! 🔐 Binds paid guest media ingress and its format input to the caller's Original source.

use crate::{bridge::OriginalMediaCommandBinding, command_owner::{OriginalMediaCommand, OriginalMediaFormatInput}, ingress::OriginalMediaLoan, receiver::MediaOrigin};
use semio_framework_job::{
    InteractiveJobCloseStep, OriginalSourceBinding, OriginalTurnFailure, StepContext, close_original_effect_step,
};
use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage};
use semio_framework_value::{
    RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::retained_clone_grant_admits,
};

/// 🎟️ A guest picker action's one-shot target and live revision, held before the host reads.
pub struct OriginalMediaPickerIntent<T> {
    revision: MediaOrigin,
    target: T,
    kind: u8,
    maximum: usize,
    spent: bool,
}

/// 🪪 Keeps the receiver beneath the current instance's retained command-origin loan.
pub struct OriginalMediaGuest<T, const N: usize> {
    source: OriginalSourceBinding,
    command: OriginalMediaCommand<T, N>,
}

/// 🎟️ A format factory receives a page owner and the same original caller authority.
#[derive(semio_framework_value::RetireOwned)]
pub struct OriginalGuestMediaInput<T, const N: usize> {
    #[value(retire_with = "std::mem::drop")]
    source: OriginalSourceBinding,
    input: OriginalMediaFormatInput<T, N>,
    cancelled: bool,
}

fn validate_source(source: OriginalSourceBinding, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
    cx.original_source_parent()?.validate_borrowed_context(cx)?;
    if !source.matches(cx) {
        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Original media borrow changed its caller Source").into());
    }
    Ok(())
}

fn run<R>(
    cx: &mut StepContext<'_>, step: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> Result<(R, RetainedCloneProgress), ValueError>,
) -> Result<Option<R>, OriginalTurnFailure> {
    if cx.is_cancelled() || cx.should_yield() { return Ok(None); }
    let grant = cx.retained_grant();
    let Some((result, performed)) = cx.with_original_source_effect(grant, |_, effect| step(effect))? else {
        return Ok(None);
    };
    cx.consume_fuel(u64::try_from(performed).map_err(|_| ValueError::invariant("Original media work exceeded its Source turn"))?);
    let (value, receipt) = result.map_err(|error| OriginalTurnFailure::debit_original(error, cx))?;
    cx.consume_retained(receipt)?;
    Ok(Some(value))
}

impl<T: Copy> OriginalMediaPickerIntent<T> {
    pub fn loan(&self) -> OriginalMediaLoan<'_> { self.revision.loan() }
    pub fn kind(&self) -> u8 { self.kind }
    pub fn maximum(&self) -> usize { self.maximum }
    pub fn target(&self) -> T { self.target }

    /// 🗃️ Captures the editor's trusted placement before publishing the picker request.
    pub fn issue(
        live: OriginalMediaLoan<'_>, target: T, kind: u8, maximum: usize, slot: &mut Option<Self>,
        cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if slot.is_some() { return Ok(false); }
        if kind == 0 || maximum == 0 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Original media picker requires a declared port and extent").into());
        }
        run(cx, |effect| {
            let demand = RetirementDemand { copy_bytes: size_of::<Option<Self>>(), depth: 1, ..Default::default() };
            let work = OriginalMediaCommand::<T, 1>::BIRTH_WORK;
            if !retained_clone_grant_admits(effect.grant(), demand)
                || effect.remaining_work_units()? < work || !effect.performed(work)? {
                return Ok((false, Default::default()));
            }
            let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
            let revision = MediaOrigin::capture(live).map_err(|error| error.with_retained_progress(receipt))?;
            *slot = Some(Self { revision, target, kind, maximum, spent: false });
            Ok((true, receipt))
        }).map(|value| value.unwrap_or(false))
    }
}

impl<T, const N: usize> OriginalGuestMediaInput<T, N> {
    /// 🪪 Lends the immutable request coordinates while its format owner holds the source.
    pub fn loan(&self) -> OriginalMediaLoan<'_> { self.input.loan() }

    /// 🧬 Rechecks the full live PDF revision and Original source before a format job borrows the input.
    pub fn current_input(
        &mut self, current: OriginalMediaLoan<'_>, cx: &StepContext<'_>,
    ) -> Result<Option<&mut OriginalMediaFormatInput<T, N>>, OriginalTurnFailure> {
        validate_source(self.source, cx)?;
        self.cancelled |= !self.input.admits_origin(current) || cx.is_cancelled();
        if self.cancelled {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "Original media format input lost its live PDF revision").into());
        }
        Ok((!cx.should_yield()).then_some(&mut self.input))
    }

    pub fn close_demand(&self, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        let owner = &self.input.bytes;
        if owner.terminal_is_empty() { return Ok(None); }
        owner.next_demand(body).map(Some)
    }

    pub fn close(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        validate_source(self.source, cx)?;
        self.cancelled = true;
        let owner = &mut self.input.bytes;
        if owner.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        let body = cx.retained_grant().maximum_copy_bytes;
        close_original_effect_step(cx, owner.next_demand(body)?, |effect| owner.step(body, effect))
    }
}

impl<T: Copy, const N: usize> OriginalMediaGuest<T, N> {
    pub fn accepted_bytes(&self) -> usize { self.command.accepted_bytes() }

    /// 🌱 Opens one paid receiver before any command page is taken from the Original input.
    pub fn begin(
        intent: &mut OriginalMediaPickerIntent<T>, offered: &OriginalMediaCommandBinding<'_>, current: OriginalMediaLoan<'_>,
        slot: &mut Option<Self>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        let source = OriginalSourceBinding::borrow(cx);
        validate_source(source, cx)?;
        if slot.is_some() { return Ok(false); }
        intent.spent |= !intent.revision.matches(current);
        if intent.spent || intent.kind != offered.kind {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Original media begin changed or reused its issued picker").into());
        }
        run(cx, |effect| {
            let mut command = None;
            let receipt = OriginalMediaCommand::admit_with_metadata(offered, current, intent.target, intent.maximum, &mut command,
                size_of::<Option<Self>>() + size_of::<bool>(), effect)?;
            let Some(command) = command else { return Ok((false, receipt)); };
            *slot = Some(Self { source, command });
            intent.spent = true;
            Ok((true, receipt))
        }).map(|value| value.unwrap_or(false))
    }

    fn command(&mut self, cx: &StepContext<'_>) -> Result<&mut OriginalMediaCommand<T, N>, OriginalTurnFailure> {
        validate_source(self.source, cx)?;
        let command = &mut self.command;
        if cx.is_cancelled() { command.abandon(); }
        Ok(command)
    }

    /// 📄 Transfers one page to the guest actor and leaves a pending offer with its caller on refusal.
    pub fn stage_page(
        &mut self, current: OriginalMediaLoan<'_>,
        offer: &mut Option<(CommandPageCursor, FixedCommandPage)>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        let command = self.command(cx)?;
        if offer.is_none() { return Ok(false); }
        run(cx, |effect| command.stage_page(current, offer, effect).map(|receipt| (offer.is_none(), receipt)))
            .map(|value| value.unwrap_or(false))
    }

    /// 🔁 Reborrows an existing guest page without reading another host page during backpressure.
    pub fn page_progress(&mut self, current: OriginalMediaLoan<'_>, cx: &mut StepContext<'_>) -> Result<Option<u32>, OriginalTurnFailure> {
        let command = self.command(cx)?;
        let _ = run(cx, |effect| command.copy_page(current, effect).map(|receipt| ((), receipt)))?;
        let body = cx.retained_grant().maximum_copy_bytes;
        if let Some(demand) = command.page_retirement_demand(body)? {
            close_original_effect_step(cx, demand, |effect| command.retire_page(body, effect).expect("quoted media page close"))?;
        }
        run(cx, |effect| command.acknowledge_page(current, effect)
            .map(|ack| ack.map_or((None, Default::default()), |(index, receipt)| (Some(index), receipt))))
            .map(Option::flatten)
    }

    /// 🧩 Hands a complete byte source only to its next format-job recipient.
    pub fn complete(
        &mut self, current: OriginalMediaLoan<'_>, slot: &mut Option<OriginalGuestMediaInput<T, N>>,
        cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        let source = self.source;
        let command = self.command(cx)?;
        if slot.is_some() { return Ok(false); }
        run(cx, |effect| {
            let mut output = None;
            let receipt = command.complete_with_metadata(current, &mut output, size_of::<Option<OriginalGuestMediaInput<T, N>>>(), effect)?;
            let Some(input) = output else { return Ok((false, receipt)); };
            *slot = Some(OriginalGuestMediaInput { source, input, cancelled: false });
            Ok((true, receipt))
        }).map(|value| value.unwrap_or(false))
    }

    pub fn cancel(&mut self, issued: OriginalMediaLoan<'_>, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        Ok(self.command(cx)?.cancel(issued)?)
    }

    pub fn close_demand(&self, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        if self.command.terminal_is_empty() { return Ok(None); }
        self.command.close_demands(body).map(Some)
    }

    /// ♻️ Keeps the paired page and destination retirement below the admitted actor Source.
    pub fn close(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let command = self.command(cx)?;
        if command.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        let body = cx.retained_grant().maximum_copy_bytes;
        close_original_effect_step(cx, command.close_demands(body)?, |effect| command.close_step(body, effect).expect("quoted media command close"))
    }
}
