//! 🪆️ Retained original SemioFlow fields build the actual stable execution scene.
use super::*;
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use flow::FlowJsonField as JsonField;
use semio_framework_pack_json::{JsonReadSource,JsonMemberPolicy};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original composed Flow source changed its funded frontier")}
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
/// 🔁️ Reads a settled close poll as a grant-shaped outcome; a suspended effect keeps its state and retries next turn.
fn poll_outcome(poll:OriginalRetirementPoll)->semio_framework_value::RetainedWorkOutcome{
 match poll{
  RetainedWorkPoll::Pending{work}=>semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},
  RetainedWorkPoll::Complete(turn)=>semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(|failure|failure.into_value_error()),work:turn.work},
 }
}
/// 🧹️ Funds one grant-shaped close step of an owner ladder through the close turn's effect.
fn close_with_effect(cx:&mut StepContext<'_>,demand:RetirementDemand,close:impl FnOnce(RetainedCloneGrant,&mut RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{
 semio_framework_job::close_original_effect_step(cx,demand,|effect|->OriginalRetirementPoll{let grant=effect.grant();let turn=close(grant,effect);RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})})
}
/// 🪪️ Admits one owner into its close ladder and reads the admission as a grant-shaped outcome.
fn begin_close_owner<T:RetireOwned>(close:&mut RetainedCloneClose,slot:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome{
 let turn=close.begin_granted(slot,effect);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}
}
/// 🫴️ Runs one quoted effect-shaped step inside one original effect; the turn is charged exactly the work it performed.
fn effected(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<bool,OriginalTurnFailure>{
 if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}
 let grant=cx.retained_grant();
 let Some((result,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(false)};
 let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 cx.consume_retained(progress)?;cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
 result?;Ok(true)
}
#[path="📥️field/🦀️.rs"]mod field;
#[path="❓️unknown/🦀️.rs"]mod unknown;
#[path="🧩️node/🦀️.rs"]mod node;
#[path="🧵️assembly/🦀️.rs"]mod assembly;
pub use assembly::{FlowComposedSourceWork,FlowComposedRead,FlowComposedScene};

pub use node::{NodeWork as FlowNodeReceiveWork,NodeOutput as FlowNodeReceiveOutput};
