//! 📐️ One original tessellation invocation owns request, query and canonical mesh through paid closure.
use super::{Session,SessionCapture};
use flow_extension_sdk::geometry::{GeometryStep,GeometryWorkFailure,GeometryWorkIdentity};
use semio_framework_job::{StepContext,OriginalDomainProjection,OriginalDomainFailure,OriginalDomainView as View};
use semio_framework_pack_json::{ArtifactCanonicalJsonTree as Tree,ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text,JsonGrammarCursor,JsonMemberPolicy,Value};
use pack::record::{OwnedProjectedPackWireCursor,BorrowedProjectedPackFailure};
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, ErasedSnapshotRetirement, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value_native_owned::native_decoding::NativeDecodeContinuation;
use semio_framework_value::retirement::RetireOwned;
use std::sync::{Mutex,TryLockError};

type Wire=semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>;
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
fn invariant()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation projection ordinal is absent")}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn header()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn birth<T:RetireOwned>()->RetirementDemand{RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()}}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn admit<T:RetireOwned>(original:&mut Option<T>,active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 if !permits(grant,birth::<T>()){return Ok(Default::default())}
 let source=original.take().ok_or_else(invariant)?;match semio_framework_value::retirement::admit_owned_retirement(source,grant){Ok((owner,receipt))=>{*active=Some(owner);Ok(receipt)},Err((error,source))=>{*original=Some(source);Err(error)}}
}

#[derive(Default,semio_framework_value::RetireOwned)]
struct TessellationRequest{handle:String,tolerance:f64,node_hash:u64,window_id:String,window_kind_id:String}

#[derive(semio_framework_value::RetireOwned)]
struct TessellationCompleted{request:TessellationRequest,mesh:semio_framework::MeshData,done:bool,cancellable:bool,phase:&'static str,units_done:u64,units_total:u64}
impl Tree for TessellationCompleted{
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(11))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{["cancellable","done","handle","mesh","nodeHash","phase","tolerance","unitsDone","unitsTotal","windowId","windowKindId"].get(ordinal).map(|key|Text::from(*key)).ok_or_else(invariant)}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match ordinal{0=>Ok(&self.cancellable),1=>Ok(&self.done),2=>Ok(&self.request.handle),3=>Ok(&self.mesh),4=>Ok(&self.request.node_hash),5=>Ok(&self.phase),6=>Ok(&self.request.tolerance),7=>Ok(&self.units_done),8=>Ok(&self.units_total),9=>Ok(&self.request.window_id),10=>Ok(&self.request.window_kind_id),_=>Err(invariant())}}
}
impl OriginalDomainProjection for TessellationCompleted{
 fn projection_identity(&self)->usize{self.request.handle.as_ptr()as usize}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{let mut original: &dyn Tree=self;for ordinal in path{original=original.canonical_tree_child(*ordinal)?;}match original.canonical_tree_node()?{Node::Null=>Ok(View::IntrinsicNull),Node::Bool(value)=>Ok(View::IntrinsicBool(value)),Node::I64(value)=>Ok(View::IntrinsicNumber(semio_framework_value::Number::Int(value))),Node::U64(value)=>Ok(View::IntrinsicNumber(semio_framework_value::Number::UInt(value))),Node::F32(value)=>Ok(View::IntrinsicNumber(semio_framework_value::Number::Float(f64::from(value)))),Node::F64(value)=>Ok(View::IntrinsicNumber(semio_framework_value::Number::Float(value))),Node::String(value)|Node::Text(Text::Contiguous(value))=>Ok(View::IntrinsicText(value)),Node::Array(length)=>Ok(View::IntrinsicArray(length)),Node::Object(length)=>Ok(View::IntrinsicObject(length)),_=>Err(invariant())}}
 fn projection_key(&self,path:&[usize],ordinal:usize)->Result<&str,ValueError>{let mut original: &dyn Tree=self;for child in path{original=original.canonical_tree_child(*child)?;}match original.canonical_tree_key(ordinal)?{Text::Contiguous(key)=>Ok(key),_=>Err(invariant())}}
}

#[derive(Debug,semio_framework_value::RetireOwned)]
enum TessellationCause{Input(flow_extension_sdk::EvaluationFailure),Geometry(GeometryWorkFailure),Pack(BorrowedProjectedPackFailure),Query(String)}
#[derive(Debug,semio_framework_value::RetireOwned)]
struct TessellationFailure{primary:TessellationCause,receipt_refusal:Option<ValueError>}
impl From<ValueError> for TessellationFailure{fn from(original:ValueError)->Self{Self{primary:TessellationCause::Input(original.into()),receipt_refusal:None}}}
impl From<BorrowedProjectedPackFailure> for TessellationFailure{fn from(original:BorrowedProjectedPackFailure)->Self{Self{primary:TessellationCause::Pack(original),receipt_refusal:None}}}
impl TessellationFailure{fn json(original:semio_framework_pack_json::JsonError)->Self{Self{primary:TessellationCause::Input(original.into()),receipt_refusal:None}}fn geometry(original:GeometryWorkFailure)->Self{Self{primary:TessellationCause::Geometry(original),receipt_refusal:None}}fn query(original:String)->Self{Self{primary:TessellationCause::Query(original),receipt_refusal:None}}}
impl OriginalDomainProjection for TessellationFailure{
 fn projection_identity(&self)->usize{match &self.primary{TessellationCause::Input(original)=>original.projection_identity(),TessellationCause::Geometry(original)=>original.projection_identity(),TessellationCause::Pack(original)=>&original.reason as*const ValueError as usize,TessellationCause::Query(original)=>original.as_ptr()as usize}}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{
  if path.is_empty(){return Ok(View::IntrinsicObject(3))}if path==[0]{return Ok(View::IntrinsicText(match self.primary{TessellationCause::Input(_)=>"input",TessellationCause::Geometry(_)=>"geometry",TessellationCause::Pack(_)=>"pack",TessellationCause::Query(_)=>"query"}))}
  match path[0]{1=>match &self.primary{TessellationCause::Input(original)=>original.projection_view(&path[1..]),TessellationCause::Geometry(original)=>original.projection_view(&path[1..]),TessellationCause::Pack(original)=>match &path[1..]{[]=>Ok(View::IntrinsicObject(2)),[0,rest @ ..]=>semio_framework_job::project_original_value_error(&original.reason,rest),[1]=>Ok(View::UInt(u64::try_from(original.written_bytes).map_err(|_|invariant())?)),_=>Err(invariant())},TessellationCause::Query(original)if path.len()==1=>Ok(View::IntrinsicText(original)),_=>Err(invariant())},2=>match &self.receipt_refusal{Some(original)=>semio_framework_job::project_original_value_error(original,&path[1..]),None if path.len()==1=>Ok(View::IntrinsicNull),_=>Err(invariant())},_=>Err(invariant())}
 }
 fn projection_key(&self,path:&[usize],ordinal:usize)->Result<&str,ValueError>{if path.is_empty(){return ["source","original","receiptRefusal"].get(ordinal).copied().ok_or_else(invariant)}match path[0]{1=>match &self.primary{TessellationCause::Input(original)=>original.projection_key(&path[1..],ordinal),TessellationCause::Geometry(original)=>original.projection_key(&path[1..],ordinal),TessellationCause::Pack(original)=>match &path[1..]{[]=>["reason","writtenBytes"].get(ordinal).copied().ok_or_else(invariant),[0,rest @ ..]=>semio_framework_job::key_original_value_error(&original.reason,rest,ordinal),_=>Err(invariant())},_=>Err(invariant())},2=>self.receipt_refusal.as_ref().ok_or_else(invariant).and_then(|original|semio_framework_job::key_original_value_error(original,&path[1..],ordinal)),_=>Err(invariant())}}
}
impl OriginalDomainFailure for TessellationFailure{
 fn domain_code(&self)->&str{"geometry.tessellation"}
 fn domain_message(&self)->&str{match &self.primary{TessellationCause::Input(original)=>original.domain_message(),TessellationCause::Geometry(original)=>original.domain_message(),TessellationCause::Pack(original)=>original.reason.message.as_ref(),TessellationCause::Query(original)=>original}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match &self.primary{TessellationCause::Input(original)=>original.domain_retained_progress(),TessellationCause::Geometry(original)=>original.domain_retained_progress(),TessellationCause::Pack(original)=>original.reason.retained_progress(),TessellationCause::Query(_)=>Default::default()}}
}

struct TessellationInvokeCursor{
 identity:GeometryWorkIdentity,pointer:usize,length:usize,parser:Option<JsonGrammarCursor<Value>>,continuation:Option<NativeDecodeContinuation>,root:Option<Value>,member:Option<(String,Value)>,request:Option<TessellationRequest>,active:Option<Box<dyn ErasedSnapshotRetirement>>,
 wire:Option<OwnedProjectedPackWireCursor<TessellationCompleted>>,completed:Option<TessellationCompleted>,ready_wire:Option<Wire>,mesh:Option<semio_framework::MeshData>,fault:Option<TessellationFailure>,close_fault:Option<TessellationFailure>,required:u8,candidate:usize,position:usize,decoded:bool,query_active:bool,query_complete:bool,units:u64,closing:bool,normal_posted:bool,normal_progress:RetainedCloneProgress,
}
impl TessellationInvokeCursor{
 fn new(source:&[u8],identity:GeometryWorkIdentity)->Self{Self{identity,pointer:source.as_ptr()as usize,length:source.len(),parser:None,continuation:None,root:None,member:None,request:None,active:None,wire:None,completed:None,ready_wire:None,mesh:None,fault:None,close_fault:None,required:0,candidate:0,position:0,decoded:false,query_active:false,query_complete:false,units:0,closing:false,normal_posted:false,normal_progress:Default::default()}}
 fn matches(&self,source:&[u8],cx:&StepContext<'_>)->bool{self.identity.operation==cx.operation().0&&self.identity.generation==cx.generation().0&&self.pointer==source.as_ptr()as usize&&self.length==source.len()}
 fn begin_close(&mut self){self.closing=true;if let Some(wire)=&mut self.wire{wire.begin_close();}}
 fn terminal_is_empty(&self)->bool{self.parser.is_none()&&self.continuation.is_none()&&self.root.is_none()&&self.member.is_none()&&self.request.is_none()&&self.active.is_none()&&self.wire.is_none()&&self.completed.is_none()&&self.ready_wire.is_none()&&self.mesh.is_none()&&self.fault.is_none()&&self.close_fault.is_none()&&!self.query_active}
 fn source_empty(&self)->bool{self.parser.is_none()&&self.continuation.is_none()&&self.root.is_none()&&self.member.is_none()&&self.active.is_none()}
 fn source_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(active)=&self.active{return semio_framework_value::factory_ticket_demands(active,copy)}
  if self.member.is_some(){return Ok(birth::<(String,Value)>())}if self.parser.is_some(){return Ok(birth::<JsonGrammarCursor<Value>>())}if self.root.is_some(){return Ok(birth::<Value>())}Ok(header())
 }
 fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(wire)=&self.wire{return wire.next_demand(copy)}if !self.source_empty(){return self.source_demands(copy)}if self.completed.is_some(){return Ok(birth::<TessellationCompleted>())}if self.ready_wire.is_some(){return Ok(birth::<Wire>())}if self.request.is_some(){return Ok(birth::<TessellationRequest>())}if self.mesh.is_some(){return Ok(birth::<semio_framework::MeshData>())}if self.fault.is_some()||self.close_fault.is_some(){return Ok(birth::<TessellationFailure>())}Ok(header())
 }
 fn close_source(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.active.is_some(){return semio_framework_value::close_factory_ticket(&mut self.active,grant).map(|step|step.progress())}
  if self.member.is_some(){return admit(&mut self.member,&mut self.active,grant)}if self.parser.is_some(){return admit(&mut self.parser,&mut self.active,grant)}if self.root.is_some(){return admit(&mut self.root,&mut self.active,grant)}
  if self.continuation.is_some()&&permits(grant,header()){self.continuation=None;return Ok(item())}Ok(Default::default())
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,TessellationFailure>{
  self.begin_close();if let Some(wire)=&mut self.wire{let receipt=wire.close_step(grant)?;if wire.terminal_is_empty(){self.wire=None;}return Ok(receipt.progress())}
  if !self.source_empty(){return self.close_source(grant).map_err(Into::into)}if self.completed.is_some(){return admit(&mut self.completed,&mut self.active,grant).map_err(Into::into)}if self.ready_wire.is_some(){return admit(&mut self.ready_wire,&mut self.active,grant).map_err(Into::into)}if self.request.is_some(){return admit(&mut self.request,&mut self.active,grant).map_err(Into::into)}if self.mesh.is_some(){return admit(&mut self.mesh,&mut self.active,grant).map_err(Into::into)}if self.fault.is_some(){return admit(&mut self.fault,&mut self.active,grant).map_err(Into::into)}if self.close_fault.is_some(){return admit(&mut self.close_fault,&mut self.active,grant).map_err(Into::into)}
  if self.query_active&&permits(grant,header()){self.query_active=false;return Ok(item())}Ok(Default::default())
 }
 fn fail(&mut self,original:TessellationFailure){if self.fault.is_none(){self.fault=Some(original)}else{assert!(self.close_fault.is_none(),"original cleanup failure remains retained");self.close_fault=Some(original)}self.begin_close();}
 fn decode_field(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,TessellationFailure>{
  let Some(Value::Object(object))=self.root.as_mut()else{return Err(invalid("tessellation request must be an object").into())};
  if object.is_empty(){if self.required!=31{return Err(invalid("tessellation request requires handle, tolerance, nodeHash, windowId and windowKindId").into())}self.decoded=true;return Ok(item())}
  let names=["handle","tolerance","nodeHash","windowId","windowKindId"];
  let(name,value)=object.last_member().ok_or_else(invariant)?;let Some(expected)=names.get(self.candidate)else{return Err(invalid("tessellation request contains an undeclared member").into())};
  if name.len()!=expected.len(){self.candidate+=1;self.position=0;return Ok(item())}
  if self.position<name.len(){if grant.maximum_copy_bytes==0{return Ok(Default::default())}let equal=name.as_bytes()[self.position]==expected.as_bytes()[self.position];self.position+=1;if !equal{self.candidate+=1;self.position=0;}return Ok(RetainedCloneProgress{copied_bytes:1,..item()})}
  let bit=1u8<<self.candidate;if self.required&bit!=0{return Err(invalid("tessellation request repeats a member").into())}
  match(self.candidate,value){(0|3|4,Value::String(value))if !value.is_empty()=>{},(1,Value::Number(value))if value.as_f64().is_finite()&&value.as_f64()>0.0=>{},(2,Value::Number(value))if value.as_u64().is_some()=>{},_=>return Err(invalid("tessellation request member has an invalid value").into())}
  let mut member=object.pop_member().ok_or_else(invariant)?;let request=self.request.as_mut().ok_or_else(invariant)?;
  match(self.candidate,&mut member.1){(0,Value::String(value))=>request.handle=std::mem::take(value),(1,Value::Number(value))=>request.tolerance=value.as_f64(),(2,Value::Number(value))=>request.node_hash=value.as_u64().ok_or_else(invariant)?,(3,Value::String(value))=>request.window_id=std::mem::take(value),(4,Value::String(value))=>request.window_kind_id=std::mem::take(value),_=>return Err(invariant().into())}
  self.member=Some(member);self.required|=bit;self.candidate=0;self.position=0;Ok(item())
 }
 fn cancel_query(&mut self,session:&Session,cx:&mut StepContext<'_>)->Result<RetainedCloneProgress,TessellationFailure>{
  let original=self.request.as_ref().ok_or_else(invariant)?;self.normal_posted=true;
  match session.cancel_tessellation_step(self.identity,&original.handle,original.tolerance,cx){Ok((step,work))=>{if matches!(step,RetainedCloneStep::Complete(_)){self.query_active=false;}Ok(work.retained)},Err(original)=>{self.normal_progress=original.work.retained;Err(TessellationFailure::geometry(original))}}
 }
 fn step(&mut self,session:&Session,source:&[u8],cx:&mut StepContext<'_>)->Result<Option<Wire>,TessellationFailure>{
  self.normal_posted=false;self.normal_progress=Default::default();let available=cx.retained_grant();let grant=RetainedCloneGrant{maximum_items:available.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX)).min(1),..available};
  if !self.matches(source,cx){return Err(invalid("tessellation retains another original invocation").into())}cx.original_source_parent()?;
  if !permits(grant,header())||cx.should_yield()||self.close_fault.is_some(){return Ok(None)}if cx.is_cancelled(){self.begin_close();}
  if self.closing{
   cx.set_stage("closing-tessellation");if self.query_active{self.normal_progress=self.cancel_query(session,cx)?;return Ok(None)}
   if self.wire.is_some()||!self.source_empty()||self.completed.is_some()||self.ready_wire.is_some()||self.request.is_some()||self.mesh.is_some(){self.normal_progress=self.close_step(grant)?;return Ok(None)}
   return Ok(None)
  }
  if let Some(wire)=&mut self.wire{cx.set_stage(wire.phase());let result=wire.advance(grant);match result{Ok(step)=>{self.normal_progress=step.progress;if let Some(ready)=step.ready{assert!(wire.terminal_is_empty());self.wire=None;self.completed=Some(ready.source);self.ready_wire=Some(ready.wire);}},Err(original)=>{self.normal_progress=original.reason.retained_progress();return Err(original.into())}}return Ok(None)}
  if self.ready_wire.is_some(){cx.set_stage("closing-tessellation-output-source");if self.active.is_some(){self.normal_progress=semio_framework_value::close_factory_ticket(&mut self.active,grant)?.progress();return Ok(None)}if self.completed.is_some(){self.normal_progress=admit(&mut self.completed,&mut self.active,grant)?;return Ok(None)}self.normal_progress=item();return Ok(self.ready_wire.take())}
  if self.query_complete{
   if self.query_active{cx.set_stage("closing-tessellation-query");self.normal_progress=self.cancel_query(session,cx)?;return Ok(None)}
   let request=self.request.take().ok_or_else(invariant)?;let mesh=self.mesh.take().ok_or_else(invariant)?;self.wire=Some(OwnedProjectedPackWireCursor::new(TessellationCompleted{request,mesh,done:true,cancellable:false,phase:"complete",units_done:self.units,units_total:self.units},1));self.normal_progress=item();return Ok(None)
  }
  if self.decoded&&!self.source_empty(){cx.set_stage("closing-tessellation-request");self.normal_progress=self.close_source(grant)?;return Ok(None)}
  if self.decoded{
   let original=self.request.as_ref().ok_or_else(invariant)?;self.query_active=true;self.normal_posted=true;let result=session.tessellate_step(self.identity,&original.handle,original.tolerance,cx);
   match result{Ok((step,work))=>{self.normal_progress=work.retained;self.units=self.units.checked_add(work.consumed_units as u64).ok_or_else(invariant)?;match step{GeometryStep::Working{phase,..}=>{cx.set_stage(phase);},GeometryStep::Ready(mesh)=>{self.mesh=Some(mesh);self.query_complete=true;},GeometryStep::Cancelled=>self.begin_close(),GeometryStep::Failed(original)=>return Err(TessellationFailure::query(original))}},Err(original)=>{self.normal_progress=original.work.retained;return Err(TessellationFailure::geometry(original))}}return Ok(None)
  }
  cx.set_stage("decoding-tessellation");if self.active.is_some()||self.member.is_some(){self.normal_progress=if self.active.is_some(){semio_framework_value::close_factory_ticket(&mut self.active,grant)?.progress()}else{admit(&mut self.member,&mut self.active,grant)?};return Ok(None)}
  if self.root.is_some(){if self.request.is_none(){self.request=Some(TessellationRequest::default());self.normal_progress=item();return Ok(None)}self.normal_progress=self.decode_field(grant)?;return Ok(None)}
  if self.parser.is_none(){self.parser=Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject));self.normal_progress=item();return Ok(None)}
  let mut callback=|_|!cx.should_yield()&&!cx.is_cancelled();let mut control=match self.continuation.take(){Some(original)=>NativeDecodeControl::resume(original,&mut callback)?,None=>NativeDecodeControl::new_retained(&mut callback)};
  let parser=self.parser.as_mut().unwrap();let result=parser.step_source(source,1,&mut control,grant,false);self.normal_progress=parser.normal_step_progress();self.continuation=Some(control.pause()?);match result{Ok(Some(original))=>self.root=Some(original),Ok(None)=>{},Err(original)=>return Err(TessellationFailure::json(original))}Ok(None)
 }
}

/// 🧭️ The extension retains one actual initiating source rather than replacing its operation on each progress hop.
pub struct TessellationResources{cursor:Mutex<Option<TessellationInvokeCursor>>}
impl TessellationResources{
 pub fn new()->Self{Self{cursor:Mutex::new(None)}}
 pub fn invoke(&self,session:&SessionCapture,source:&[u8],cx:&mut StepContext<'_>)->Result<semio_framework_plugin::ExtensionInvokeStep,semio_framework::Fault>{
  let pending=||semio_framework_plugin::ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:None};
  let mut owner=match self.cursor.try_lock(){Ok(owner)=>owner,Err(TryLockError::WouldBlock)=>return Ok(pending()),Err(TryLockError::Poisoned(_))=>return Ok(semio_framework_plugin::ExtensionInvokeStep{refusal:Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation owner is poisoned").into()),..pending()})};
  if owner.as_ref().is_some_and(|cursor|!cursor.matches(source,cx)){return Ok(semio_framework_plugin::ExtensionInvokeStep{refusal:Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"tessellation invocation retains another original caller").into()),..pending()})}
  if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(pending())}
  if owner.is_none()&&cx.is_cancelled(){return Ok(pending())}
  if owner.is_none(){if let Err(original)=cx.original_source_parent(){return Ok(semio_framework_plugin::ExtensionInvokeStep{refusal:Some(original.into()),..pending()})}*owner=Some(TessellationInvokeCursor::new(source,GeometryWorkIdentity{operation:cx.operation().0,generation:cx.generation().0}));cx.consume_fuel(1);let receipt=item();let refusal=cx.consume_retained(receipt).err().map(Into::into);return Ok(semio_framework_plugin::ExtensionInvokeStep{retained_progress:receipt,refusal,..pending()})}
  let cursor=owner.as_mut().unwrap();let incoming=cx.retained_progress();let before=cx.fuel_remaining();let result=cursor.step(session,source,cx);let receipt=cursor.normal_progress;
  let payload=match result{Ok(payload)=>payload,Err(original)=>{cursor.fail(original);None}};
  if !cursor.normal_posted{let refusal=cx.consume_retained(receipt).err();cx.consume_fuel(receipt.copied_items as u64);if let Some(original)=refusal{if let Some(failure)=&mut cursor.fault{failure.receipt_refusal=Some(original)}else{cursor.fail(original.into())}}}else{assert!(cx.fuel_remaining()<=before);}
  let mut refusal=None;if receipt==RetainedCloneProgress::default(){let selected=if cursor.close_fault.is_some(){Some(&mut cursor.close_fault)}else if cursor.closing&&!cursor.query_active&&cursor.wire.is_none()&&cursor.source_empty()&&cursor.request.is_none()&&cursor.completed.is_none()&&cursor.ready_wire.is_none()&&cursor.mesh.is_none()&&cursor.fault.is_some(){Some(&mut cursor.fault)}else{None};if let Some(selected)=selected{refusal=match semio_framework_job::admit_original_domain_failure(selected,cx){Ok(original)=>original,Err(original)=>Some(original)};}}let after=cx.retained_progress();let receipt=RetainedCloneProgress{copied_items:after.copied_items-incoming.copied_items,copied_bytes:after.copied_bytes-incoming.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-incoming.retained_capacity_bytes,released_bytes:after.released_bytes-incoming.released_bytes};
  if payload.is_some(){assert!(cursor.terminal_is_empty());*owner=None;}else if cursor.terminal_is_empty(){*owner=None;}
  Ok(semio_framework_plugin::ExtensionInvokeStep{payload:payload.map(|wire|wire.to_vec_owner()),retained_progress:receipt,refusal})
 }
 pub fn begin_close(&mut self){if let Some(cursor)=self.cursor.get_mut().expect("exclusive tessellation owner").as_mut(){cursor.begin_close();}}
 pub fn terminal_is_empty(&self)->bool{self.cursor.lock().is_ok_and(|owner|owner.is_none())}
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.cursor.lock().map_err(|_|invariant())?.as_ref().map_or(Ok(Default::default()),|cursor|cursor.retirement_demands(copy))}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
  let owner=self.cursor.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner);let Some(cursor)=owner.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};
  if cursor.close_fault.is_some(){return match semio_framework_job::admit_original_domain_failure(&mut cursor.close_fault,cx){Ok(Some(original))=>Err(original),Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(original)=>Err(original)}}
  cursor.begin_close();let demand=cursor.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;let available=cx.retained_grant();if cx.should_yield()||!permits(available,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}let grant=RetainedCloneGrant{maximum_items:1,..available};cx.consume_fuel(1);
  let receipt=match cursor.close_step(grant){Ok(receipt)=>receipt,Err(original)=>{let receipt=original.domain_retained_progress();cursor.close_fault=Some(original);receipt}};if let Err(original)=cx.consume_retained(receipt){if let Some(failure)=&mut cursor.close_fault{failure.receipt_refusal=Some(original)}else{cursor.close_fault=Some(original.into())}return Ok(RetainedCloneStep::Progress(receipt))}
  if cursor.terminal_is_empty(){*owner=None;}Ok(if owner.is_none(){RetainedCloneStep::Complete(receipt)}else{RetainedCloneStep::Progress(receipt)})
 }

}
