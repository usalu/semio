//! 🪂 Owns the paid natural input and terminal document publication coordinates between turns.

use semio_framework_job::{Operation, OperationId, OriginalSourceBinding, OriginalTurnFailure, StepContext, close_original_effect_step};
use semio_framework_value::{
    OriginalRetirementFailure, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep,
    RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, SharedUtf8, ValueError, retained_clone::RetainedCloneClose,
};
use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_decl_media::original_media::natural_source::{NaturalFilePagedSource, NaturalFileSourceGuest};
use semio_plugin_ownership::bounded_ownership::{PluginLifecycleFailure, PluginLifecycleStep, original_lifecycle_delta, original_lifecycle_step};

pub struct NaturalFilePublication {
    pub source: OriginalSourceBinding,
    pub meta: Option<ActionMeta>,
    pub input: Option<NaturalFilePagedSource>,
    pub operation: Option<Operation>,
    pub mount_id: Option<OperationId>,
    pub committed: bool,
    input_donated: bool,
    close: RetainedCloneClose,
}

impl NaturalFilePublication {
    /// 📥 Retains the actual local actor and reserves the terminal Store witness before mounting.
    pub fn admit(instance: u32, actor: &SharedUtf8, cx: &mut StepContext<'_>) -> Result<Option<Box<Self>>, OriginalTurnFailure> {
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Self>>>() + size_of::<Self>() + size_of::<bool>(),
            retained_capacity_bytes: size_of::<Self>(), ..Default::default()
        };
        let grant = cx.retained_grant();
        if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() < 2 || grant.maximum_depth == 0
            || grant.maximum_items < 2 || !progress.fits(grant) {
            return Ok(None);
        }
        let mut attached = None;
        cx.with_retained_loan(RetainedCloneGrant {
            maximum_items: grant.maximum_items - progress.copied_items,
            maximum_copy_bytes: grant.maximum_copy_bytes - progress.copied_bytes,
            maximum_capacity_bytes: grant.maximum_capacity_bytes - progress.retained_capacity_bytes,
            ..grant
        }, |cx| cx.with_original_work_loan(cx.fuel_remaining() - 1, u64::MAX, |cx| {
            let Some((turn, work)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| actor.admit_clone(effect).result)? else {
                return Ok(());
            };
            cx.consume_fuel(work as u64);
            let (actor, lease) = turn.map_err(|error| OriginalTurnFailure::debit_original(error, cx))?;
            cx.consume_retained(lease)?;
            attached = Some(actor);
            Ok(())
        }))?;
        let Some(actor) = attached else { return Ok(None); };
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(Some(Box::new(Self {
            source: OriginalSourceBinding::borrow(cx),
            meta: Some(ActionMeta { actor, instance_id: instance, expected_document_revision: None, view_state: None }),
            input: None, operation: None, mount_id: None, committed: false, input_donated: false, close: Default::default(),
        })))
    }

    /// 🎟️ Takes the sealed original without copying any of its bytes.
    pub fn take_source(&mut self, source: &mut NaturalFileSourceGuest, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.input_donated { return Ok(true); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<bool>(), ..Default::default() };
        let grant = cx.retained_grant();
        if grant.maximum_items < 2 || cx.fuel_remaining() < 2 || !progress.fits(grant) { return Ok(false); }
        let mut donated = false;
        cx.with_retained_loan(RetainedCloneGrant {
            maximum_items: grant.maximum_items - progress.copied_items,
            maximum_copy_bytes: grant.maximum_copy_bytes - progress.copied_bytes,
            ..grant
        }, |cx| cx.with_original_work_loan(cx.fuel_remaining() - 1, u64::MAX, |cx| {
            donated = source.take_source(source.loan().operation, true, &mut self.input, cx)?;
            Ok(())
        }))?;
        if donated {
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            self.input_donated = true;
        }
        Ok(donated)
    }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(input) = self.input.as_ref() {
            return if input.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<NaturalFilePagedSource>>(), depth: 1, ..Default::default() })
            } else { input.next_demand(body) };
        }
        if self.meta.is_some() || !self.close.is_empty() {
            return self.close.next_owner_demands::<ActionMeta>(self.meta.is_some(), &None, body);
        }
        Ok(RetirementDemand { copy_bytes: size_of::<Option<Box<Self>>>(), release_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }

    /// ♻️ Returns undonated bytes one backing at a time and closes the retained action metadata.
    pub fn close_input(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.input.is_none() && self.meta.is_none() && self.close.is_empty() { return Ok(true); }
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.close_demands(body)?;
        if self.input.as_ref().is_some_and(|input| input.terminal_is_empty()) {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
            if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0
                || !progress.fits(cx.retained_grant()) { return Ok(false); }
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            self.input.take();
            return Ok(false);
        }
        cx.with_original_retirement(|cx| close_original_effect_step(cx, demand, |effect| {
            if let Some(input) = self.input.as_mut() { return input.step(body, effect); }
            if !self.close.is_empty() { return self.close.step_granted(body, effect); }
            let turn = self.close.begin_granted(&mut self.meta, effect);
            RetainedWorkPoll::Complete(RetainedWorkOutcome {
                work: turn.work,
                result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Complete(Default::default())))
                    .map_err(OriginalRetirementFailure::from),
            })
        }))?;
        Ok(self.input.is_none() && self.meta.is_none() && self.close.is_empty())
    }

    pub fn close_mounted(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>, grant: RetainedCloneGrant)
        -> Result<PluginLifecycleStep, PluginLifecycleFailure>
    {
        let Some(owner) = slot.as_ref() else { return Ok(PluginLifecycleStep::Complete(Default::default())); };
        if !owner.source.matches(cx) {
            return Ok(PluginLifecycleStep::AwaitingInput { reason: "natural source awaits its retained receiving source" });
        }
        let demand = owner.close_demands(grant.maximum_copy_bytes)?;
        original_lifecycle_step(cx, grant, demand, |same, _| {
            let before = same.retained_progress();
            if slot.as_mut().expect("natural source owner is mounted").close_input(same)? {
                same.with_original_retirement(|same| Self::retire(slot, same))?;
            }
            let progress = original_lifecycle_delta(before, same.retained_progress())?;
            Ok((PluginLifecycleStep::Progress(progress), false))
        })
    }

    pub fn retire(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Self>>>(), released_bytes: size_of::<Self>(), ..Default::default()
        };
        if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0
            || !progress.fits(cx.retained_grant()) { return Ok(false); }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        slot.take();
        Ok(true)
    }
}
