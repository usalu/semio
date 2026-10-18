//! 🔌️ Extension invocation and command ingress of the reactor turn: the retained extension invocation with its receive and advance steps and the retained command ingress with its owner cursor, retirement, park and close steps. See [`super`].

use super::*;

#[doc(hidden)]
pub struct RetainedExtensionInvocation{
    pub req:semio_framework::kernel::RequestId,
    pub source:Option<(String,Vec<u8>,String)>,
    origin_kind:u8,
    pub result:Option<semio_framework::kernel::RequestOutcome>,
    pub failure:Option<RetainedExtensionFaultReply>,
    refused_output:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<u8>>>,
    pub closing:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(String,Vec<u8>,String)>>,
}

impl RetainedExtensionInvocation{
    pub fn new(req:semio_framework::kernel::RequestId,from:MessageEndpoint,capability:String,payload:Vec<u8>)->Self{
        let(origin_kind,origin)=match from{MessageEndpoint::Shell{instance}=>(0,instance.0),MessageEndpoint::Backbone{uri}=>(1,uri),MessageEndpoint::PluginInstance{id}=>(2,id.0),MessageEndpoint::Extension{id}=>(3,id),MessageEndpoint::Topic{name}=>(4,name)};
        Self{req,source:Some((capability,payload,origin)),origin_kind,result:None,failure:None,refused_output:None,closing:None}
    }

    pub async fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework::kernel::RequestOutcome>,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(None)}
        let grant=semio_framework_value::RetainedCloneGrant{maximum_items:cx.retained_grant().maximum_items.min(1),..cx.retained_grant()};
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        if let Some(closing)=self.refused_output.as_mut(){
            if closing.terminal_is_empty(){cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.refused_output=None;return Ok(None)}
            semio_framework_job::close_original_retained_step(cx,closing.selected_demand(grant.maximum_copy_bytes)?,|loan|closing.step(loan))?;
            return Ok(None)
        }
        if let Some(failure)=self.failure.as_mut(){
            if let Some(payload)=failure.advance(cx)?{self.result=Some(semio_framework::kernel::RequestOutcome::Err(payload));self.failure=None;}
            return Ok(None)
        }
        if let Some(closing)=self.closing.as_mut(){
            if !closing.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,closing.selected_demand(grant.maximum_copy_bytes)?,|loan|closing.step(loan))?;return Ok(None)}
            cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.closing=None;return Ok(self.result.take())
        }
        if self.result.is_some(){cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.begin_source_close();return Ok(None)}
        let Some((capability,payload,_))=self.source.as_ref()else{return Ok(None)};
        let answer=match semio_framework_plugin_extension::extension_runtime_lift::extension_invoke(capability,payload,cx).await{Ok(answer)=>answer,Err(cause)=>{self.failure=Some(RetainedExtensionFaultReply::new(ExtensionInvocationCause::Fault(cause)));return Ok(None)}};
        if let Some(cause)=answer.refusal{self.failure=Some(RetainedExtensionFaultReply::new(ExtensionInvocationCause::Turn(cause)));if let Some(payload)=answer.payload{self.refused_output=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(payload).unwrap_or_else(|_|unreachable!("original refused bytes declare typed retirement")))}return Ok(None)}
        let Some(payload)=answer.payload else{return Ok(None)};
        self.result=Some(semio_framework::kernel::RequestOutcome::Ok(payload));
        Ok(None)
    }
    fn begin_source_close(&mut self){let original=self.source.take().expect("original invocation retains caller fields until actual output");self.closing=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original request fields declare complete typed retirement")));}
}

/// 🔌️ How many extension invocations still own their caller fields or output: a settled reactor holds none, and a thread must not end while one still owns a controlled retirement.
pub fn retained_extension_invocation_occupancy()->usize{
    RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow().iter().filter(|owner|owner.is_some()).count())
}

pub(super) async fn advance_retained_extension_invocation(cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Effect>,semio_framework_job::OriginalTurnFailure>{
    let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
    let start=RETAINED_EXTENSION_CURSOR.get();let selected=RETAINED_EXTENSION_INVOCATIONS.with(|owners|{let mut owners=owners.borrow_mut();(0..requests::REQUEST_SLOTS).map(|offset|(start+offset)%requests::REQUEST_SLOTS).find_map(|index|owners[index].take().map(|owner|(index,owner)))});
    let Some((index,mut owner))=selected else{return Ok(None)};let result=owner.step(cx).await;RETAINED_EXTENSION_CURSOR.set((index+1)%requests::REQUEST_SLOTS);
    receive_retained_extension_invocation(index,owner,result)
}

#[doc(hidden)]
pub fn receive_retained_extension_invocation(index:usize,owner:RetainedExtensionInvocation,result:Result<Option<semio_framework::kernel::RequestOutcome>,semio_framework_job::OriginalTurnFailure>)->Result<Option<Effect>,semio_framework_job::OriginalTurnFailure>{
    match result{Ok(Some(result))=>Ok(Some(Effect::Respond{req:owner.req,result})),result=>{RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow_mut()[index]=Some(owner));result.map(|_|None)}}
}

#[doc(hidden)]
pub enum CommandIngressOwner {
    ReservedPresence { cursor: semio_framework::kernel::CommandPageCursor, admission: semio_framework_plugin_peer::PresenceRosterAdmission, page: semio_framework::kernel::FixedCommandPage },
    Presence { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64 },
    PendingPresencePage { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64, page: semio_framework::kernel::FixedCommandPage },
    GenericAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
    ClosingAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
    Generic { cursor: semio_framework::kernel::CommandPageCursor, command: semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress },
}

#[doc(hidden)]
pub struct RetainedCommandIngress {
    pub key: instance_lifetime::NativeCloseKey,
    pub state: CommandIngressOwner,
}

/// 📥️ The cursor the retained owner is holding, whatever shape it holds it in — the one answer
/// [`command_ingress_never_idle_while_owned`](crate::plugin::reactor::turn) needs to turn "this turn
/// advanced nothing" into "this command is still mine", which is the difference between a host that
/// keeps driving and a host that spins on `Idle` for a thousand crossings.
/// 📥️ The law a turn that still OWNS a command ingress answers by: `Idle` means "no owner", and
/// nothing else, so an owner that advanced nothing this turn says `CommandPending` rather than going
/// silent. Without it the host's drain cannot tell "still mine, working" from "gone", and the only
/// bound left is a crossing count — which is how `command ingress did not complete within 1024
/// continuations (observed statuses: idle)` came to be the whole diagnosis.
#[doc(hidden)]
pub fn command_ingress_while_owned(status: semio_framework::kernel::CommandIngressStatus, owner: Option<semio_framework::kernel::CommandPageCursor>) -> semio_framework::kernel::CommandIngressStatus {
    match (status, owner) {
        (semio_framework::kernel::CommandIngressStatus::Idle, Some(cursor)) => semio_framework::kernel::CommandIngressStatus::CommandPending(cursor),
        (status, _) => status,
    }
}

pub(super) fn command_ingress_owner_cursor(owner: &CommandIngressOwner) -> semio_framework::kernel::CommandPageCursor {
    match owner {
        CommandIngressOwner::ReservedPresence { cursor, .. }
        | CommandIngressOwner::Presence { cursor, .. }
        | CommandIngressOwner::PendingPresencePage { cursor, .. }
        | CommandIngressOwner::GenericAssembly { cursor, .. }
        | CommandIngressOwner::ClosingAssembly { cursor, .. }
        | CommandIngressOwner::Generic { cursor, .. } => cursor.clone(),
    }
}

fn retire_command_ingress(state: CommandIngressOwner,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<CommandIngressOwner>,semio_framework_job::OriginalTurnFailure> {
    Ok(match state {
        CommandIngressOwner::ReservedPresence { admission, .. } => {
            admission.cancel.cancel_now();
            None
        }
        CommandIngressOwner::Presence { .. } | CommandIngressOwner::PendingPresencePage { .. } => None,
        CommandIngressOwner::GenericAssembly { cursor, mut pages } | CommandIngressOwner::ClosingAssembly { cursor, mut pages } => {
            if pages.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).0 {
                None
            } else {
                Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
            }
        }
        CommandIngressOwner::Generic { cursor, command } if matches!(command,semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded{..}) => match command.retire_failed_step_on_original(cx){
            semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Pending(command)=>Some(CommandIngressOwner::Generic{cursor,command}),
            semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Failed(failure)=>return Err(failure),
        },
        CommandIngressOwner::Generic { cursor, command } => command.retire_step().map(|command| CommandIngressOwner::Generic { cursor, command }),
    })
}

/// 📥️ Which of the guest's two fixed ingress slots holds the owner the host is DRIVING, and which
/// holds one it has stopped driving.
///
/// The host drives ONE command per instance at a time and keeps a `kernel::CommandBatchDriver` alive
/// for exactly as long as that command is in flight; there is no message on this wire that tells the
/// guest a command was abandoned. The only evidence the guest ever gets is a page that names a
/// DIFFERENT command on the same instance — at which point the retained owner is provably not being
/// driven any more. It cannot simply be dropped (its pages and its decoder are released one bounded
/// step at a time, like every other close ladder here), so it moves to the retiring lane, where it
/// closes on its own budget and is invisible to this turn's ingress status. The live lane is then
/// free for the command the host IS driving.
///
/// Before this, a superseded owner sat in the one used slot forever: every page for the new command
/// answered `Backpressure` and every turn that carried no page answered `CommandPending` with the
/// STALE cursor, which the host's `validate_cursor` correctly refused with
/// `plugin.command-cursor-mismatch` (ticket 26/09/18 slice A2 §6.1, slice R2).
#[doc(hidden)]
pub const COMMAND_INGRESS_LIVE_SLOT: usize = 1;

#[doc(hidden)]
pub const COMMAND_INGRESS_RETIRING_SLOT: usize = 0;

/// 🧹️ One bounded close step for a superseded owner. It owns no ingress status: the host has no
/// driver left for it, so anything this lane said would name a command the host cannot acknowledge.
#[doc(hidden)]
pub fn step_retiring_command_ingress(cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure> {
    COMMAND_INGRESS.with(|ingress| {
        let mut ingress = ingress.borrow_mut();
        if let Some(entry) = ingress[COMMAND_INGRESS_RETIRING_SLOT].take() {
            let key = entry.key;
            ingress[COMMAND_INGRESS_RETIRING_SLOT] = retire_command_ingress(entry.state,cx)?.map(|state| RetainedCommandIngress { key, state });
        }
        Ok(())
    })
}

pub(super) fn retiring_command_ingress_is_free() -> bool {
    COMMAND_INGRESS.with(|ingress| ingress.borrow()[COMMAND_INGRESS_RETIRING_SLOT].is_none())
}

/// 🧹️ Moves a superseded owner into the retiring lane and takes its first close step there.
pub(super) fn park_retiring_command_ingress(key: instance_lifetime::NativeCloseKey, state: CommandIngressOwner,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure> {
    COMMAND_INGRESS.with(|ingress| {
        ingress.borrow_mut()[COMMAND_INGRESS_RETIRING_SLOT] = retire_command_ingress(state,cx)?.map(|state| RetainedCommandIngress { key, state });
        Ok(())
    })
}

#[doc(hidden)]
pub fn close_command_ingress_step(key: instance_lifetime::NativeCloseKey,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    COMMAND_INGRESS.with(|ingress| {
        let mut ingress = ingress.try_borrow_mut().map_err(|_| reactor_close_fault("command ingress close authority busy"))?;
        let Some(index) = ingress.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.key == key)) else { return Ok(true) };
        let entry = ingress[index].take().expect("selected exact ingress owner");
        ingress[index] = retire_command_ingress(entry.state,cx)?.map(|state| RetainedCommandIngress { key, state });
        Ok(!ingress.iter().any(|entry| entry.as_ref().is_some_and(|entry| entry.key == key)))
    })
}

/// 🧮️ Occupied slots of the guest's retained command-ingress authority, out of its fixed two. A
/// settled reactor holds none: an owner survives a turn only while its own command is still being
/// assembled, dispatched or closed. This is what an ingress-saturation law reads between commands —
/// a stream that leaves an owner behind pins the authority and backpressures every later command.
pub fn retained_command_ingress_occupancy() -> usize {
    COMMAND_INGRESS.with(|ingress| ingress.borrow().iter().filter(|entry| entry.is_some()).count())
}

/// 🔌️ Advances the retained extension invocation and the extension retirement under the mounted close policy; reports whether extension retirement still has work.
pub(super) async fn drive_extension_turn<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, effects: &mut Vec<Effect>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    if let Some(effect)=advance_retained_extension_invocation(cx).await?{effects.push(effect);}
    let original_extension_grant = runtime.mounted_owner_policy().close;
    let remaining=cx.retained_grant();
    let extension_grant = semio_framework_value::RetainedCloneGrant { maximum_items: original_extension_grant.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:original_extension_grant.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:original_extension_grant.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:original_extension_grant.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:original_extension_grant.maximum_depth.min(remaining.maximum_depth) };
    let (extension_retirement_work, _) = cx.with_retained_loan(extension_grant,|same|semio_framework_plugin_extension::extension_runtime_lift::extension_retirement_turn(same))?;
    Ok(extension_retirement_work)
}
