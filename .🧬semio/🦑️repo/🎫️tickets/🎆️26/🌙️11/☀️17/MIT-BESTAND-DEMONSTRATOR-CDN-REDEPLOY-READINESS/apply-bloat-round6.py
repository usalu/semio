#!/usr/bin/env python3
"""Round 6 monomorphization cuts for the native receiving framework (idempotent, drafted without a compiler).

Applies five independent steps to the shared value framework. Each step is guarded by a marker, re-reads its file,
asserts every anchor matches exactly once and is skipped when its marker is already present, so the script can be
re-run safely and applied step by step (`apply-bloat-round6.py [--root DIR] [step ...]`, steps: cursor relay tree derive backing).

  cursor   `value/♻️retirement/🦀️.rs`: `admit_retirement_cursor` and `retirement_cursor_birth_demand` share non-generic cores
           (`admit_cursor_slot` over a `Layout`, `retirement_cursor_birth_core` over plain facts).
  relay    `native_receiver_protocol` packages: `native_relay` forwards to the erased `native_relay_dyn`.
  tree     `owned/🌳️value/🦀️.rs`: `NativeFieldPlanSlot` gains the close, begin-close, advance and advance-work edges and the
           non-generic helpers `apply_field_stage`, `apply_field_take`, `plan_close_field`, `close_field_slot`,
           `close_field_step`, `begin_field_close`, `advance_field`, `advance_field_work`.
  derive   `derive/🛬️receive/🦀️.rs`: the per-field `apply`, `close_plan`, `close_poll`, `retire_normal`, `advance_work_demand`
           and `advance_nested` arms call those helpers instead of expanding the stage machines per field.
  backing  `native_receiver_protocol` backing: `NativeReceiverBackingClose::{quote,drive}` run once over the object-safe
           `BackingView`/`BackingOwner` adapters; the public generic wrappers stay one-line forwards.

Order matters only for tree before derive (the derive output calls the tree helpers). Nothing here compiles in this
session: run `repo cargo check` for the retirement, receiver-protocol, native-owned and derive crates afterwards.
Deliberately not done: the enum derive (`derive/🛬️receive/🔀️enum`), `apply` typed `take`/`place_field` erasure (needs a sound
`Option<T>` downcast or layout move) and the retirement `next_owner_demands` edge (kept as one small closure per field).
"""
import pathlib
import re
import sys

args=sys.argv[1:]
root=None
if '--root' in args:
    i=args.index('--root');root=pathlib.Path(args[i+1]);del args[i:i+2]
if root is None:
    root=pathlib.Path(__file__).resolve().parents[7]
if not any(c.name.replace('️','')=='🧰framework' for c in root.iterdir()):
    sys.exit('repository root not found: '+str(root))
only=set(args)
VS='️'


def locate(*parts):
    here=root
    for part in parts:
        want=part.replace(VS,'')
        hit=[c for c in here.iterdir() if c.name.replace(VS,'')==want]
        assert len(hit)==1,('path part missing',part,str(here))
        here=hit[0]
    return here


V=('🧰framework','🔨modules','🌱value')
D=V+('🛬decode',)
P=D+('🧳owned','🧬protocol')
FILES={
 'retirement':locate(*V,'♻retirement','🦀.rs'),
 'packages':locate(*P,'📦packages','🦀rust','🦀.rs'),
 'tree':locate(*D,'🧳owned','🌳value','🦀.rs'),
 'derive':locate(*V,'✨derive','🛬receive','🦀.rs'),
 'backing':locate(*P,'♻backing','🦀.rs'),
}
EMOJI='🪅🪘🪬🥁🪃🪤🛎🧶🪝🪄🪗🪠🪩🪁🪂'


def read(key):
    return FILES[key].read_text(encoding='utf8')


def write(key,text):
    text=re.sub(r'/// (['+EMOJI+r'])(?!️)',lambda m:'/// '+m.group(1)+VS,text)
    FILES[key].write_text(text,encoding='utf8')


def once(text,old,new,label):
    assert text.count(old)==1,('anchor not unique',label,text.count(old))
    return text.replace(old,new)


def between(text,start,end,new,label):
    a=text.index(start) if start in text else -1
    assert a>=0 and text.count(start)==1,('start anchor',label)
    b=text.index(end,a)
    assert text.count(end)==1,('end anchor',label)
    return text[:a]+new+text[b:]


def run(name,marker_key,marker,fn):
    if only and name not in only:
        return
    text=read(marker_key)
    if marker in text:
        print(name,'already applied');return
    fn()
    print(name,'applied')


# ---------------------------------------------------------------- cursor
def cursor():
    t=read('retirement')
    old_demand_start='pub fn retirement_cursor_birth_demand<T:RetireOwned>(original:&T)->Result<crate::RetirementDemand,crate::ValueError>{\n'
    old_demand_end='\n\npub fn retirement_cursor_birth_work()->u64{4}'
    new_demand=r'''pub fn retirement_cursor_birth_demand<T:RetireOwned>(original:&T)->Result<crate::RetirementDemand,crate::ValueError>{
 retirement_cursor_birth_core(original.retirement_ready()&&T::controlled_retirement_supported(),std::mem::size_of::<T::RetirementCursor>(),original.retirement_birth_bytes(),std::mem::size_of::<Option<T>>())
}
/// 🪝️ Quotes one cursor birth from plain facts so the arithmetic and refusals are compiled once instead of once per owner type.
fn retirement_cursor_birth_core(ready:bool,capacity_bytes:usize,declared:Option<usize>,option_bytes:usize)->Result<crate::RetirementDemand,crate::ValueError>{
 if !ready{return Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original cursor seed is not ready"))}
 if declared!=Some(capacity_bytes){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"original cursor birth differs from its defining layout"))}
 let copy_bytes=option_bytes.checked_add(capacity_bytes).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"original cursor header transfer overflow"))?;
 Ok(crate::RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()})
}'''
    t=between(t,old_demand_start,old_demand_end,new_demand,'birth demand')
    a=t.index('/// 🏗️ Admits one exact cursor allocation before consuming its original resident input.\n')
    b=t.index('/// 🦜️ Composes a composite')
    new_admit=r'''/// 🏗️ Admits one exact cursor allocation before consuming its original resident input.
pub fn admit_retirement_cursor<T:RetireOwned>(original:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,crate::RetainedCloneProgress)>>{
 let Some(value)=original.as_ref()else{return crate::RetainedWorkOutcome{result:Ok(None),work:0}};
 let demand=match retirement_cursor_birth_demand(value){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};let layout=std::alloc::Layout::new::<T::RetirementCursor>();
 let(pointer,work)=match admit_cursor_slot(demand,layout,effect){Err((error,work))=>return crate::RetainedWorkOutcome{result:Err(error),work},Ok((None,work))=>return crate::RetainedWorkOutcome{result:Ok(None),work},Ok((Some(pointer),work))=>(pointer.cast::<T::RetirementCursor>(),work)};
 let value=original.take().expect("admitted original cursor input");let owner=unsafe{pointer.as_ptr().write(value.retirement_seed());Box::from_raw(pointer.as_ptr())};
 crate::RetainedWorkOutcome{result:Ok(Some((owner,crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()}))),work}
}
/// 🪄️ Admits and allocates one cursor frame by its layout so the grant checks and the allocation are compiled once; `None` means the grant does not yet admit it.
fn admit_cursor_slot(demand:crate::RetirementDemand,layout:std::alloc::Layout,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<(Option<std::ptr::NonNull<u8>>,u128),(crate::ValueError,u128)>{
 let grant=effect.grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<layout.size()||grant.maximum_copy_bytes<demand.copy_bytes||effect.maximum_work_units()==0{return Ok((None,0))}
 if effect.actual().is_some(){return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"original cursor allocation marker is already occupied"),0))}
 let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return Err((error,work))};
 if layout.size()==0{return Ok((std::ptr::NonNull::new(layout.align() as *mut u8),work))}
 *effect.actual()=Some(0);let raw=unsafe{std::alloc::alloc(layout)};if !raw.is_null(){*effect.actual()=Some(layout.size())}
 let Some(pointer)=std::ptr::NonNull::new(raw)else{return Err((crate::ValueError::literal(crate::ValueRefusalKind::AllocationFailed,"original cursor allocation failed"),work))};Ok((Some(pointer),work))
}

'''
    t=t[:a]+new_admit+t[b:]
    write('retirement',t)


# ----------------------------------------------------------------- relay
def relay():
    t=read('packages')
    old=r'''pub fn native_relay<'r,T,Child,Parent:From<Child>>(reply:OriginalEffectReply<'r,T,Parent>,drive:impl for<'s> FnOnce(OriginalEffectReply<'s,T,Child>)->Result<OriginalEffectReplyComplete<'s,T,Child>,ValueError>)->Result<OriginalEffectReplyComplete<'r,T,Parent>,ValueError>{
 let mut slot=None;{let inner=OriginalEffectReply::borrow(&mut slot)?;drop(drive(inner)?);}
 Ok(match slot.take(){Some(result)=>reply.complete(result.map_err(Into::into)),None=>reply.pending()})
}
'''
    new=r'''pub fn native_relay<'r,T,Child,Parent:From<Child>>(reply:OriginalEffectReply<'r,T,Parent>,drive:impl for<'s> FnOnce(OriginalEffectReply<'s,T,Child>)->Result<OriginalEffectReplyComplete<'s,T,Child>,ValueError>)->Result<OriginalEffectReplyComplete<'r,T,Parent>,ValueError>{
 let mut drive=Some(drive);native_relay_dyn::<T,Child,Parent>(reply,&mut |inner|drive.take().expect("native relay drives once")(inner))
}
/// 🪂️ Relays one erased child drive so the reply bookkeeping is compiled once per reply type instead of once per call site.
#[inline(never)]
pub fn native_relay_dyn<'r,T,Child,Parent:From<Child>>(reply:OriginalEffectReply<'r,T,Parent>,drive:&mut dyn for<'s> FnMut(OriginalEffectReply<'s,T,Child>)->Result<OriginalEffectReplyComplete<'s,T,Child>,ValueError>)->Result<OriginalEffectReplyComplete<'r,T,Parent>,ValueError>{
 let mut slot=None;{let inner=OriginalEffectReply::borrow(&mut slot)?;drop(drive(inner)?);}
 Ok(match slot.take(){Some(result)=>reply.complete(result.map_err(Into::into)),None=>reply.pending()})
}
'''
    t=once(t,old,new,'native_relay')
    write('packages',t)


# ------------------------------------------------------------------ tree
SRC='crate::retained_clone::RetainedOwnedProjection<dyn NativeValueSource>'
REPLY="OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>"
DONE="Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>"
ADV_POLL='crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>'
WIT='semio_framework_value_control_contract::OriginalSourceTurnWitness<\'_>'
TRAIT_TAIL=f'''
 fn plan_close_step_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:{REPLY})->{DONE};
 fn plan_begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:{REPLY})->{DONE};
 fn plan_advance_work_demand(&self,source:&Option<{SRC}>,body:usize)->Result<u64,ValueError>;
 fn plan_advance(&mut self,source:&Option<{SRC}>,control:&mut NativeDecodeControl<'_>,witness:{WIT},body:usize,maximum:u64,grant:RetainedCloneGrant)->{ADV_POLL};
'''
RECEIVE_IMPL=f'''
 fn plan_close_step_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:{REPLY})->{DONE}{{self.close_into(body,effect,reply)}}
 fn plan_begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:{REPLY})->{DONE}{{self.begin_close_into(effect,reply)}}
 fn plan_advance_work_demand(&self,source:&Option<{SRC}>,body:usize)->Result<u64,ValueError>{{self.advance_work_demand(source.as_ref().ok_or_else(invalid)?.borrow()?,body)}}
 fn plan_advance(&mut self,source:&Option<{SRC}>,control:&mut NativeDecodeControl<'_>,witness:{WIT},body:usize,maximum:u64,grant:RetainedCloneGrant)->{ADV_POLL}{{let source=match source.as_ref().ok_or_else(invalid).and_then(|source|source.borrow()){{Ok(source)=>source,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{{result:Err(error.into()),work:0}})}};self.advance(source,control,witness,body,maximum,grant)}}
'''
DEFAULT_IMPL=f'''
 fn plan_close_step_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:{REPLY})->{DONE}{{self.close_step(body,effect,reply)}}
 fn plan_begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:{REPLY})->{DONE}{{self.begin_close_into(effect,reply)}}
 fn plan_advance_work_demand(&self,_:&Option<{SRC}>,body:usize)->Result<u64,ValueError>{{self.advance_work_demand(body)}}
 fn plan_advance(&mut self,_:&Option<{SRC}>,control:&mut NativeDecodeControl<'_>,witness:{WIT},body:usize,maximum:u64,grant:RetainedCloneGrant)->{ADV_POLL}{{self.advance(control,witness,body,maximum,grant)}}
'''
TREE_HELPERS=f'''
/// 🪅️ Names the three outcomes of publishing one field's received child into its pending slot.
pub enum NativeFieldTaken{{Pending,Settled(RetainedCloneStep),Work(u128)}}
/// 🪘️ Settles one pending-slot publication of a field: a refusal propagates, an empty slot settles the turn, otherwise the field moves to placement.
pub fn apply_field_take(poll:crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,pending_none:bool,stage:&mut u8)->Result<NativeFieldTaken,OriginalReceivingPrimary>{{
 match poll{{crate::RetainedWorkPoll::Pending{{..}}=>Ok(NativeFieldTaken::Pending),crate::RetainedWorkPoll::Complete(turn)=>{{let work=turn.work;turn.result?;if pending_none{{return Ok(NativeFieldTaken::Settled(RetainedCloneStep::Progress(Default::default())))}}*stage=3;Ok(NativeFieldTaken::Work(work+2))}}}}
}}
/// 🪬️ Advances every field apply stage that touches no typed value: reserve, close handoff, source release and field step; returns the work it recorded.
pub fn apply_field_stage(stage:&mut u8,field:&mut usize,source:&mut Option<{SRC}>,present:bool,slot_empty:bool)->Result<u128,ValueError>{{
 Ok(match(*stage,present){{
  (0,false)=>{{*stage=1;1}},
  (1,_)=>{{*stage=2;1}},
  (4,_)=>{{*stage=if slot_empty{{5}}else{{7}};1}},
  (5,true)=>{{if source.is_some(){{source.take();}}else{{*stage=6;}}1}},
  (5,false)=>{{*stage=6;1}},
  (6,_)=>{{*field+=1;*stage=0;2}},
  (7,_)=>{{*stage=5;1}},
  _=>return Err(invalid())}})
}}
/// 🥁️ Quotes the close step of one non-empty field slot, or `None` when the slot holds nothing.
fn plan_close_one(slot:&dyn NativeFieldPlanSlot,body:usize)->Option<Result<(RetirementDemand,u64),ValueError>>{{
 if slot.plan_terminal_is_empty(){{return None}}
 Some(if slot.plan_is_closing(){{slot.plan_close_demands(body).and_then(|demand|Ok((demand,slot.plan_close_work_demand(body)?)))}}else{{slot.plan_begin_close_demand().and_then(|demand|Ok((demand,slot.plan_begin_close_work_demand()?)))}})
}}
/// 🪗️ Quotes the close step of one field: its receiving slot, then its default slot, then the pending owner, then the placed value, else the bare skip.
pub fn plan_close_field(slot:&dyn NativeFieldPlanSlot,default:Option<&dyn NativeFieldPlanSlot>,pending:bool,pending_owner:&dyn Fn()->Result<(RetirementDemand,u64),ValueError>,placed:bool,field_demand:fn(usize)->Result<RetirementDemand,ValueError>,body:usize)->Result<(RetirementDemand,u64),ValueError>{{
 if let Some(quote)=plan_close_one(slot,body){{return quote}}if let Some(default)=default{{if let Some(quote)=plan_close_one(default,body){{return quote}}}}
 if pending{{return pending_owner()}}
 if placed{{let mut d=field_demand(0)?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(invalid)?;return Ok((d,2))}}
 Ok((fixed(size_of::<usize>()),1))
}}
/// 🪃️ Closes the first non-empty slot of one field, receiving slot before default slot; `None` leaves the field to its pending owner or placed value.
pub fn close_field_slot(slot:&mut dyn NativeFieldPlanSlot,default:Option<&mut dyn NativeFieldPlanSlot>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->Option<crate::RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>>{{
 if let Some(poll)=close_one_field(slot,body,effect){{return Some(poll)}}
 default.and_then(|default|close_one_field(default,body,effect))
}}
fn close_one_field(slot:&mut dyn NativeFieldPlanSlot,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->Option<crate::RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>>{{
 if slot.plan_terminal_is_empty(){{return None}}
 Some(if slot.plan_is_closing(){{close_field_step(slot,body,effect)}}else{{begin_field_close(slot,effect)}})
}}
/// 🪤️ Settles one close step of a field slot into a progress poll.
pub fn close_field_step(slot:&mut dyn NativeFieldPlanSlot,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>{{
 native_close_settle_dyn(effect,&mut |effect,reply|slot.plan_close_step_into(body,effect,reply)).map(|s|RetainedCloneStep::Progress(s.progress()))
}}
/// 🪁️ Settles the paid close start of a field slot into a progress poll.
pub fn begin_field_close(slot:&mut dyn NativeFieldPlanSlot,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>{{
 native_close_settle_dyn(effect,&mut |effect,reply|slot.plan_begin_close_into(effect,reply)).map(|s|RetainedCloneStep::Progress(s.progress()))
}}
/// 🛎️ Advances one field slot through its nested source turn and reports only its progress.
pub fn advance_field(slot:&mut dyn NativeFieldPlanSlot,source:&Option<{SRC}>,control:&mut NativeDecodeControl<'_>,witness:{WIT},body:usize,maximum:u64,grant:RetainedCloneGrant)->{ADV_POLL}{{
 slot.plan_advance(source,control,witness,body,maximum,grant).map(|s|RetainedCloneStep::Progress(s.progress()))
}}
/// 🧶️ Quotes the work of advancing one field slot.
pub fn advance_field_work(slot:&dyn NativeFieldPlanSlot,source:&Option<{SRC}>,body:usize)->Result<u64,ValueError>{{slot.plan_advance_work_demand(source,body)}}
'''


def tree():
    t=read('tree')
    t=once(t,'use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,NativeSlotOperation,NativeSlotProducer,OriginalEffectReply,OriginalEffectReplyComplete};\n','use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,NativeSlotOperation,NativeSlotProducer,OriginalEffectReply,OriginalEffectReplyComplete,native_close_settle_dyn};\n','tree import')
    t=once(t,'pub trait NativeFieldPlanSlot{\n','pub trait NativeFieldPlanSlot{'+TRAIT_TAIL,'trait')
    t=once(t,'impl<T:NativeReceive<dyn NativeValueSource>> NativeFieldPlanSlot for NativeReceiveSlot<dyn NativeValueSource,T>{\n','impl<T:NativeReceive<dyn NativeValueSource>> NativeFieldPlanSlot for NativeReceiveSlot<dyn NativeValueSource,T>{'+RECEIVE_IMPL,'receive impl')
    t=once(t,'impl<T:semio_framework_value_native_default::NativeDefault> NativeFieldPlanSlot for semio_framework_value_native_default::NativeDefaultSlot<T>{\n','impl<T:semio_framework_value_native_default::NativeDefault> NativeFieldPlanSlot for semio_framework_value_native_default::NativeDefaultSlot<T>{'+DEFAULT_IMPL,'default impl')
    t=t.rstrip('\n')+'\n'+TREE_HELPERS
    write('tree',t)


# ---------------------------------------------------------------- derive
FIELD_APPLY=r'''  let field_apply=indices.iter().map(|i|{
   let ty=types[*i];let member=&members[*i];let slot=&slots[*i];let default=&defaults[*i];let pending=&pending[*i];
   let arm=|slot:&syn::Ident,present:bool|quote!{match self.stage{
    2=>{match #tree::apply_field_take(self.#slot.take(&mut self.#pending,effect),self.#pending.is_none(),&mut self.stage)?{#tree::NativeFieldTaken::Pending=>return Ok(None),#tree::NativeFieldTaken::Settled(step)=>return Ok(Some(step)),#tree::NativeFieldTaken::Work(taken)=>work=taken}},
    3=>{let extra=::std::mem::size_of::<bool>()+::std::mem::size_of::<u8>();let turn=unsafe{self.partial.place_field::<#ty>(::std::mem::offset_of!(#name,#member),&mut self.#pending,extra,effect)};work=turn.work;if turn.result?.is_some(){self.initialized[#i]=true;self.stage=4;work+=2;}},
    _=>{work=#tree::apply_field_stage(&mut self.stage,&mut self.field,&mut self.source,#present,self.#slot.terminal_is_empty())?}}};
   let present=arm(slot,true);
   let missing=if required[*i]{quote!(return Err(#tree::invalid().into()))}else{arm(default,false)};
   quote!(#i=>if self.positions[#i].is_some(){#present}else{#missing})
  }).collect::<Vec<_>>();
'''
WORK_STEPS="  let work_steps=indices.iter().map(|i|{let slot=&slots[*i];let default=&defaults[*i];let missing=if required[*i]{quote!(Err(#tree::invalid()))}else{quote!(#tree::advance_field_work(&self.state.#default,&self.state.source,body))};quote!(#i=>if self.state.positions[#i].is_some(){#tree::advance_field_work(&self.state.#slot,&self.state.source,body)}else{#missing})}).collect::<Vec<_>>();\n"
NESTED_STEPS="  let nested_steps=indices.iter().map(|i|{let slot=&slots[*i];let default=&defaults[*i];let missing=if required[*i]{quote!(#value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:Err(#tree::invalid().into()),work:0}))}else{quote!(#tree::advance_field(&mut self.state.#default,&self.state.source,control,witness,body,maximum,grant))};quote!(#i=>if self.state.positions[#i].is_some(){#tree::advance_field(&mut self.state.#slot,&self.state.source,control,witness,body,maximum,grant)}else{#missing})}).collect::<Vec<_>>();\n"
NESTED_RETIRE="  let nested_retire=indices.iter().map(|i|{let slot=&slots[*i];let default=&defaults[*i];let missing=if required[*i]{quote!(#value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:Err(#tree::invalid().into()),work:0}))}else{quote!(#tree::close_field_step(&mut self.#default,body,effect))};quote!(#i=>if self.positions[#i].is_some(){#tree::close_field_step(&mut self.#slot,body,effect)}else{#missing})}).collect::<Vec<_>>();\n"
NESTED_BEGIN="  let nested_begin=indices.iter().map(|i|{let slot=&slots[*i];let default=&defaults[*i];let missing=if required[*i]{quote!(#value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:Err(#tree::invalid().into()),work:0}))}else{quote!(#tree::begin_field_close(&mut self.#default,effect))};quote!(#i=>if self.positions[#i].is_some(){#tree::begin_field_close(&mut self.#slot,effect)}else{#missing})}).collect::<Vec<_>>();\n"
CLOSE_PLANS="  let close_plans=indices.iter().map(|i|{let ty=types[*i];let slot=&slots[*i];let default=&defaults[*i];let pending=&pending[*i];let default_slot=if required[*i]{quote!(None)}else{quote!(Some(&self.#default))};quote!(#i=>#tree::plan_close_field(&self.#slot,#default_slot,self.#pending.is_some(),&||Ok((self.retirement.next_owner_demands::<#ty>(true,&None,body)?,self.retirement.next_owner_work::<#ty>(true,&None,body)?)),!self.partial.taken()&&self.initialized[#i],#owner::NativeRecordStorage::<#name>::field_demand::<#ty>,body))}).collect::<Vec<_>>();\n"
CLOSE_STEPS="  let close_steps=indices.iter().map(|i|{let ty=types[*i];let member=&members[*i];let slot=&slots[*i];let default=&defaults[*i];let pending=&pending[*i];let default_slot=if required[*i]{quote!(None)}else{quote!(Some(&mut self.#default))};quote!(#i=>{if let Some(poll)=#tree::close_field_slot(&mut self.#slot,#default_slot,body,effect){return poll}if self.#pending.is_some(){let turn=self.retirement.begin_granted(&mut self.#pending,effect);return #value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(#value::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}if !self.partial.taken()&&self.initialized[#i]{let turn=unsafe{self.partial.remove_field::<#ty>(::std::mem::offset_of!(#name,#member),&mut self.#pending,::std::mem::size_of::<bool>(),effect)};let mut work=turn.work;let result=turn.result;if result.is_ok()&&self.#pending.is_some(){self.initialized[#i]=false;work+=1;}return Self::finish(effect,work,result.map_err(Into::into))}self.close_field+=1;Self::finish(effect,1,Ok(#value::RetainedCloneStep::Progress(#tree::receipt(d))))})}).collect::<Vec<_>>();\n"


def replace_line(text,prefix,new,label):
    hits=[m.start() for m in re.finditer('^'+re.escape(prefix),text,re.M)]
    assert len(hits)==1,('line anchor',label,len(hits))
    a=hits[0];b=text.index('\n',a)+1
    return text[:a]+new+text[b:]


def derive():
    t=read('derive')
    t=between(t,'  let field_apply=indices.iter()','  let field_admit=indices.iter()',FIELD_APPLY,'field_apply')
    for prefix,new,label in (
        ('  let work_steps=indices.iter()',WORK_STEPS,'work_steps'),
        ('  let nested_steps=indices.iter()',NESTED_STEPS,'nested_steps'),
        ('  let nested_retire=indices.iter()',NESTED_RETIRE,'nested_retire'),
        ('  let nested_begin=indices.iter()',NESTED_BEGIN,'nested_begin'),
        ('  let close_plans=indices.iter()',CLOSE_PLANS,'close_plans'),
        ('  let close_steps=indices.iter()',CLOSE_STEPS,'close_steps')):
        t=replace_line(t,prefix,new,label)
    write('derive',t)


# --------------------------------------------------------------- backing
BACKING_TRAITS=r'''/// 🪠️ Read-only view of one backing owner pair, so the dispatch quote is compiled once instead of once per receiver type.
trait BackingView{
 fn address(&self)->Option<usize>;
 fn retired_address(&self)->Option<usize>;
 fn select(&self,child_body:usize,working:bool)->Result<(u8,RetirementDemand,u64),ValueError>;
}
/// 🪩️ Mutable view of one backing owner pair that runs the selected child action.
trait BackingOwner:BackingView{
 fn view(&self)->&dyn BackingView;
 fn run(&mut self,code:u8,recover:bool,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,child:&mut RetainedAllocationEffect<'_>,answer:&mut Option<Result<RetainedCloneStep,OriginalRetirementFailure>>,demand:RetirementDemand)->Result<(),ValueError>;
}
struct View<'a,I:?Sized,T,C>{storage:&'a NativeOwnedStorage<C>,retired:&'a Option<Box<C>>,marker:std::marker::PhantomData<fn(&I)->T>}
struct Owner<'a,I:?Sized,T,C>{storage:&'a mut NativeOwnedStorage<C>,retired:&'a mut Option<Box<C>>,marker:std::marker::PhantomData<fn(&I)->T>}
impl<I:?Sized,T,C:NativeReceiver<I,T>> BackingView for View<'_,I,T,C>{
 fn address(&self)->Option<usize>{self.storage.address()}
 fn retired_address(&self)->Option<usize>{self.retired.as_ref().map(|owner|std::ptr::from_ref(&**owner)as usize)}
 fn select(&self,child_body:usize,working:bool)->Result<(u8,RetirementDemand,u64),ValueError>{
  let(storage,retired)=(self.storage,self.retired);
  Ok(if let Some(owner)=retired.as_ref(){if storage.reserved(){return Err(invalid())}let release=owner.native_terminal_release().ok_or_else(unsupported)?;if !release.validates(owner){return Err(unsupported())}(4,release.boxed_demand(),1)}
  else if let Some(owner)=storage.get(){if !owner.terminal_is_empty(){if owner.is_closing(){(2,owner.close_demands(child_body)?,owner.close_work_demand(child_body)?)}else{(1,owner.begin_close_demand()?,owner.begin_close_work_demand()?)}}
   else{let release=owner.native_terminal_release().ok_or_else(unsupported)?;if !release.validates(owner){return Err(unsupported())}(3,NativeOwnedStorage::<C>::box_demand(),storage.box_work_demand())}}
  else if storage.reserved(){(5,storage.release_demand(),1)}
  else if working{(6,RetirementDemand::default(),0)}
  else{(0,RetirementDemand::default(),0)})
 }
}
impl<I:?Sized,T,C:NativeReceiver<I,T>> BackingView for Owner<'_,I,T,C>{
 fn address(&self)->Option<usize>{self.storage.address()}
 fn retired_address(&self)->Option<usize>{View::<I,T,C>{storage:&*self.storage,retired:&*self.retired,marker:std::marker::PhantomData}.retired_address()}
 fn select(&self,child_body:usize,working:bool)->Result<(u8,RetirementDemand,u64),ValueError>{View::<I,T,C>{storage:&*self.storage,retired:&*self.retired,marker:std::marker::PhantomData}.select(child_body,working)}
}
impl<I:?Sized,T,C:NativeReceiver<I,T>> BackingOwner for Owner<'_,I,T,C>{
 fn view(&self)->&dyn BackingView{self}
 fn run(&mut self,code:u8,recover:bool,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,child:&mut RetainedAllocationEffect<'_>,answer:&mut Option<Result<RetainedCloneStep,OriginalRetirementFailure>>,demand:RetirementDemand)->Result<(),ValueError>{
  let storage=&mut *self.storage;let retired=&mut *self.retired;
  match code{
   1=>{let owner=storage.get_mut().ok_or_else(invalid)?;let inner=OriginalEffectReply::borrow(answer)?;if recover{owner.recover_begin_close_into(child,inner)?;}else{owner.begin_close_into(child,inner)?;}},
   2=>{let owner=storage.get_mut().ok_or_else(invalid)?;let inner=OriginalEffectReply::borrow(answer)?;match(original.as_deref_mut(),recover){(Some(original),false)=>{owner.close_original_step(original,body,child,inner)?;},(Some(original),true)=>{owner.recover_close_original_step(original,body,child,inner)?;},(None,false)=>{owner.close_step(body,child,inner)?;},(None,true)=>{owner.recover_close_step(body,child,inner)?;}}},
   3=>{let owner=storage.get().ok_or_else(invalid)?;let release=owner.native_terminal_release().ok_or_else(unsupported)?;if !release.validates(owner){return Err(unsupported())}let turn=storage.take_box_into(retired,child);match turn{RetainedWorkPoll::Pending{..}=>{},RetainedWorkPoll::Complete(turn)=>{*answer=Some(turn.result.map_err(Into::into));}}},
   4=>{let owner=retired.as_ref().ok_or_else(invalid)?;let release=owner.native_terminal_release().ok_or_else(unsupported)?;if !release.validates(owner){return Err(unsupported())}let result=match child.performed(1){Ok(true)=>{drop(retired.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))},Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"receiver header release exceeded original Work").into()),Err(error)=>Err(error.into())};*answer=Some(result);},
   5=>{let turn=storage.release(child);*answer=Some(turn.result.map_err(Into::into));},
   _=>return Err(invalid())
  }
  Ok(())
 }
}
'''
BACKING_QUOTE=r''' fn quote<I:?Sized,T,C:NativeReceiver<I,T>>(&self,storage:&NativeOwnedStorage<C>,retired:&Option<Box<C>>,body:usize,original:bool)->Result<Selection,ValueError>{self.quote_view(&View::<I,T,C>{storage,retired,marker:std::marker::PhantomData},body,original)}
 #[inline(never)]
 fn quote_view(&self,view:&dyn BackingView,body:usize,original:bool)->Result<Selection,ValueError>{
  if let Some(selected)=self.selected{return if selected.body==body{Ok(selected)}else{Err(ValueError::invariant("native receiver backing body changed"))}}let prefix=Self::metadata();let child_body=body.saturating_sub(prefix);
  let(code,child,child_work)=view.select(child_body,self.work.is_some())?;
  let demand=if code==0{Default::default()}else{RetirementDemand{copy_bytes:child.copy_bytes.checked_add(prefix).ok_or_else(invalid)?,depth:child.depth.checked_add(1).ok_or_else(invalid)?.max(1),..child}};let work=child_work.checked_add(u64::from(code!=0)).ok_or_else(invalid)?;
  Ok(Selection{invocation:None,grant:RetainedCloneGrant::default(),body,address:view.address(),retired:view.retired_address(),original,loan:None,code,demand,child,work,child_work,prefix})
 }
'''
BACKING_DRIVE=r''' fn drive<'r,I:?Sized,T,C:NativeReceiver<I,T>>(&mut self,storage:&mut NativeOwnedStorage<C>,retired:&mut Option<Box<C>>,body:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{self.drive_owner(&mut Owner::<I,T,C>{storage,retired,marker:std::marker::PhantomData},body,original,effect,reply,recover)}
 #[inline(never)]
 fn drive_owner<'r>(&mut self,owner:&mut dyn BackingOwner,body:usize,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  let invocation=effect.invocation().ok_or_else(invalid)?.identity();let loan=match original.as_deref_mut(){None=>None,Some(original)=>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(invalid())}Some(CloseLoanBinding{ledger:original.ledger_identity(),recipient:original.retirement_recipient_identity(),work})}};if self.work.is_some_and(|work|!work.matches_effect(effect)){return Err(ValueError::invariant("native receiver backing changed original Work scope"))}
  let selected=if let Some(selected)=self.selected{if !recover{return Err(ValueError::invariant("native receiver backing requires recovery of its resident action"))}if selected.invocation!=Some(invocation){return Err(ValueError::invariant("native receiver backing changed original invocation"))}if selected.grant!=effect.grant(){return Err(ValueError::invariant("native receiver backing changed original grant"))}if selected.body!=body||selected.original!=original.is_some()||selected.loan!=loan{return Err(ValueError::invariant("native receiver backing changed original close request"))}if selected.address!=owner.address()||selected.retired!=owner.retired_address(){return Err(ValueError::invariant("native receiver backing replaced its physical owner"))}selected}
  else{if recover{return Err(invalid())}let mut selected=self.quote_view(owner.view(),body,original.is_some())?;if selected.code==0{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if !retained_clone_grant_admits(effect.grant(),selected.demand)||effect.remaining_work_units()?<selected.work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"receiver backing dispatch exceeded original Work"))}self.work=Some(OriginalWorkBindingIdentity::from_effect(effect));selected.invocation=Some(invocation);selected.grant=effect.grant();selected.loan=loan;self.selected=Some(selected);selected};
  if self.answer.is_none(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:selected.child.copy_bytes,maximum_capacity_bytes:selected.child.capacity_bytes,maximum_release_bytes:selected.child.release_bytes,maximum_depth:selected.child.depth};let mut child=effect.reborrow(grant,selected.child_work.min(effect.remaining_work_units()?))?;
   if selected.code==6{self.work=None;*self.answer=Some(Ok(RetainedCloneStep::Complete(Default::default())));}else{owner.run(selected.code,recover,original.as_deref_mut(),selected.body.saturating_sub(selected.prefix),&mut child,&mut self.answer,selected.child)?}
  }self.finish(reply)
 }
'''


def backing():
    t=read('backing')
    t=once(t,"fn unsupported()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,\"native receiver header lacks an admitted real terminal destructor\")}\nimpl NativeReceiverBackingClose{\n","fn unsupported()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,\"native receiver header lacks an admitted real terminal destructor\")}\n"+BACKING_TRAITS+"impl NativeReceiverBackingClose{\n",'traits')
    a=t.index(' fn retired_address<C>(retired:&Option<Box<C>>)->Option<usize>{')
    assert t.count(' fn retired_address<C>(')==1
    b=t.index('\n',a)+1
    t=t[:a]+t[b:]
    t=between(t,' fn quote<I:?Sized,T,C:NativeReceiver<I,T>>(',' /// 📐️ Includes every actual routing',BACKING_QUOTE,'quote')
    t=between(t," fn drive<'r,I:?Sized,T,C:NativeReceiver<I,T>>(",' /// 📬️ Starts only this backing owner',BACKING_DRIVE,'drive')
    write('backing',t)


run('cursor','retirement','fn admit_cursor_slot',cursor)
run('relay','packages','native_relay_dyn',relay)
run('tree','tree','pub fn apply_field_stage',tree)
run('derive','derive','#tree::apply_field_stage',derive)
run('backing','backing','trait BackingOwner',backing)
