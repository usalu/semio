//! 📋️ Layout inverse rows close through the real mutation factory and original loan.
pub use semio_s_artifact_layout_layout_mutations_decisions::original_field::{OriginalField,close_binding,settle_poll};
use semio_framework_value::{OriginalRetirementCloseLoan,RetainedAllocationEffect,RetainedWorkPoll,RetirementPollExit,ValueError,retained_clone::{RetainedCloneProgress,RetainedCloneStep}};
fn invalid()->ValueError{ValueError::invariant("Layout command changed its original inverse rows")}

use semio_os_kernel::RetainedMutationCloseContext;
use semio_framework_value_factory::vector::FactoryVectorRetirement;

pub type MutationRows<M> = FactoryVectorRetirement<M>;

pub fn rows_work_demand<M:Send+'static>(
    owner:&Option<MutationRows<M>>,present:bool,context:RetainedMutationCloseContext<'_,M>,body:usize,
)->Result<u64,ValueError>{
    match owner{
        Some(owner) if !owner.terminal_is_empty()=>owner.close_work_demand(context.mutations,body),
        Some(_)=>Ok(1),
        None=>Ok(u64::from(present)),
    }
}

pub fn close_rows<M:Send+'static>(
    owner:&mut Option<MutationRows<M>>,rows:&mut Option<Vec<M>>,context:RetainedMutationCloseContext<'_,M>,
    original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,
)->Result<RetainedCloneStep,RetirementPollExit>{
    if let Some(active)=owner.as_mut(){
        if active.terminal_is_empty(){
            if effect.remaining_work_units()?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
            if !effect.performed(1)?{return Err(invalid().into())}
            *owner=None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{
                copied_items:1,copied_bytes:2*size_of::<Option<MutationRows<M>>>(),..Default::default()
            }))
        }
        let poll=if active.requires_original_control(){
            active.close_original_step(original.ok_or_else(invalid)?,body,effect)
        }else{active.close_step(context.mutations,body,effect)};
        return settle_poll(poll).map(|step|RetainedCloneStep::Progress(step.progress()))
    }
    if rows.is_some(){
        return match MutationRows::<M>::admit(rows,owner,effect){
            RetainedWorkPoll::Pending{work}=>Err(RetirementPollExit::Pending(work)),
            RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into),
        }
    }
    Ok(RetainedCloneStep::Complete(Default::default()))
}
