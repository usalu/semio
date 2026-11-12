//! 🧵️ Flow-owned retained command frontiers: scanned id lists close through the original controlled retirement under supplied grants.

use semio_framework_job::{close_original_effect_step, InteractiveJobCloseStep, OriginalTurnFailure, StepContext};
use semio_framework_value::{retained_clone::{retained_clone_grant_admits, OriginalEffectReply, OriginalEffectReplyComplete, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::controlled::ControlledRetirement, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetireOwned, RetirementDemand, ValueError};

/// 🔁️ Reads a settled close poll as a grant-shaped outcome; a suspended effect keeps its state and retries next turn.
pub(crate) fn poll_outcome(poll: OriginalRetirementPoll) -> RetainedWorkOutcome {
    match poll {
        RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work },
        RetainedWorkPoll::Complete(turn) => RetainedWorkOutcome { result: turn.result.map_err(|failure| failure.into_value_error()), work: turn.work },
    }
}

/// 🧹️ Funds one close step of an owner ladder through the close turn's own effect.
pub(crate) fn close_with_effect(cx: &mut StepContext<'_>, demand: RetirementDemand, close: impl FnOnce(RetainedCloneGrant, &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
    close_original_effect_step(cx, demand, |effect| -> OriginalRetirementPoll {
        let grant = effect.grant();
        let turn = close(grant, effect);
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result: turn.result.map_err(Into::into), work: turn.work })
    })
}

/// 🫴️ Runs one quoted effect-shaped step inside one original effect; the turn is charged exactly the work it performed.
pub(crate) fn effected(cx: &mut StepContext<'_>, demand: RetirementDemand, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError>) -> Result<bool, OriginalTurnFailure> {
    if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
        return Ok(false);
    }
    let grant = cx.retained_grant();
    let Some((result, performed)) = cx.with_original_effect(grant, |_, effect| run(effect))? else { return Ok(false) };
    let progress = match &result {
        Ok(step) => step.progress(),
        Err(error) => error.retained_progress(),
    };
    cx.consume_retained(progress)?;
    cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
    result?;
    Ok(true)
}

/// 🪢️ Runs one reply-shaped callee and returns its recorded result; `None` means the callee suspended and must be recovered next turn.
pub(crate) fn replied<E>(effect: &mut RetainedAllocationEffect<'_>, action: impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>, OriginalEffectReply<'r, RetainedCloneStep, E>) -> Result<OriginalEffectReplyComplete<'r, RetainedCloneStep, E>, ValueError>) -> Result<Option<Result<RetainedCloneStep, E>>, ValueError> {
    let mut slot = None;
    let reply = OriginalEffectReply::borrow(&mut slot)?;
    action(&mut *effect, reply)?;
    Ok(slot.take())
}

/// 🪪️ Admits one owner into its close ladder; a vacant begin is zero progress.
pub(crate) fn begin_close_owner<T: RetireOwned>(close: &mut RetainedCloneClose, slot: &mut Option<T>, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome {
    let turn = close.begin_granted(slot, effect);
    RetainedWorkOutcome { result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))), work: turn.work }
}

//#region 🧹️Retirement
#[derive(Default)]
pub struct Retirement {
    pending: Vec<Vec<String>>,
    owner: Option<ControlledRetirement<Vec<Vec<String>>>>,
}

impl Drop for Retirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.is_empty(), "Flow app retirement must reach terminal-empty before drop");
        }
    }
}

impl Retirement {
    pub fn push(&mut self, values: Vec<String>) {
        self.pending.push(values);
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.owner.is_none()
    }

    pub fn step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        if let Some(owner) = self.owner.as_mut() {
            let poll = owner.step(body, effect);
            if owner.terminal_is_empty() {
                self.owner = None;
            }
            return poll;
        }
        let step = if self.pending.is_empty() {
            RetainedCloneStep::Complete(RetainedCloneProgress::default())
        } else {
            self.owner = Some(ControlledRetirement::retain(std::mem::take(&mut self.pending)));
            RetainedCloneStep::Progress(RetainedCloneProgress::default())
        };
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Ok(step), work: 0 })
    }

    pub fn copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.owner.as_ref().map_or(Ok(0), |owner| owner.next_copy_byte_demand())
    }

    pub fn capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        self.owner.as_ref().map_or(Ok(0), |owner| owner.next_capacity_byte_demand(maximum_copy_bytes))
    }

    pub fn release_byte_demand(&self) -> Result<usize, ValueError> {
        self.owner.as_ref().map_or(Ok(0), |owner| owner.next_release_byte_demand())
    }

    pub fn depth_demand(&self) -> Result<usize, ValueError> {
        self.owner.as_ref().map_or(Ok(usize::from(!self.pending.is_empty())), |owner| owner.next_depth_demand())
    }
}
//#endregion 🧹️Retirement

//#region 🧪️Tests

//#endregion 🧪️Tests

#[path="✏️authoring/🦀️.rs"]
mod authoring;
pub use authoring::FlowAuthoringCommandWork;

#[path="📥️original/🦀️.rs"]
mod original;
pub use original::FlowOriginalIntake;

#[path="🧮️evaluation/🦀️.rs"]
mod evaluation;
pub use evaluation::{FlowEvaluationCommandWork,FlowEvaluationPrepared,FlowEvaluationJobWork};

#[path="➕️descriptor.rs"]
mod descriptor;
pub use descriptor::FlowAddDescriptorWork;

#[path="📬️resolve/🦀️.rs"]
mod resolve;
pub use resolve::FlowResolveJobWork;
