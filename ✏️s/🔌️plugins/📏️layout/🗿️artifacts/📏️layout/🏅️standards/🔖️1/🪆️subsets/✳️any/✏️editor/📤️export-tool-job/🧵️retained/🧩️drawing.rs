//! 🧩️ Export retains actual parent and child source custody before lending a Drawing graph to its core.
use super::*;
use semio_framework_plugin::app::{MemberKeyRef,vector_read::{OriginalChildMemberLookup,ChildVectorReadCursor,ChildVectorReadStamp,ChildVectorSourceCustody,ChildVectorAdmissionProvider,OriginalChildMemberOperation}};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_editor::child_editor::DRAWING_CHILD_VECTOR_READ;
use semio_s_artifact_layout_layout_model::{DrawingSourceAddress,drawing_member_at,drawing_read_at,drawing_read_demand,drawing_member::source::reference::LAYOUT_DRAWING_REFERENCE_LENDER};
use semio_framework_io_ports::{erase_child_admission_source};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
pub(crate) type ParentSource=semio_framework_value::retained_clone::RetainedCloneSource<LayoutSnapshot,store::os_store::snapshot_read::SnapshotRead<LayoutSnapshot>>;
#[derive(semio_framework_value::RetireOwned)]struct DrawingOwners{lookup:Option<OriginalChildMemberLookup>,qualification:Option<ChildVectorReadCursor>,custody:ChildVectorSourceCustody<DrawingSnapshot>}
pub(crate) struct OriginalExportDrawingRead{owners:DrawingOwners,stamp:Option<ChildVectorReadStamp>,phase:u8,quote:RetirementDemand}
impl RetireOwned for OriginalExportDrawingRead{fn retirement(self)->Box<dyn RetirementCursor>{self.owners.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
pub(crate) struct OriginalExportDrawingContext<'a>{pub snapshot:&'a LayoutSnapshot,pub children:&'a semio_framework_plugin::app::ChildContentView,pub operation:OriginalChildMemberOperation<'a>}
impl OriginalExportDrawingRead{
 const HEADER:usize=2*std::mem::size_of::<Self>();
 pub fn new()->Self{Self{owners:DrawingOwners{lookup:None,qualification:None,custody:ChildVectorSourceCustody::new()},stamp:None,phase:0,quote:Default::default()}}
 pub fn ready(&self)->bool{self.phase==4}
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{if self.phase==0{return sum(RetirementDemand{copy_bytes:Self::HEADER,depth:1,..Default::default()},OriginalChildMemberLookup::birth_demand())}if self.phase==4{return Ok(Default::default())}let read=self.owners.lookup.as_ref().unwrap().demand::<DrawingSnapshot>();let child=match self.phase{1=>ChildVectorReadCursor::birth_demand(),2=>self.quote,3=>self.owners.custody.next_admit_demand()?,_=>return Err(invalid("export original Drawing read phase is invalid"))};sum(RetirementDemand{copy_bytes:Self::HEADER,depth:1,..Default::default()},sum(read,child)?)}
 pub fn advance(&mut self,parent:&ParentSource,address:DrawingSourceAddress,input:OriginalExportDrawingContext<'_>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.ready(){return Ok(true)}let demand=self.demand()?;if cx.should_yield()||cx.is_cancelled()||cx.retained_grant().maximum_items<3||!cx.retained_work_can_enter(demand){return Ok(false)}
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:Self::HEADER,..Default::default()})?;
  if self.phase==0{let(cursor,p)=OriginalChildMemberLookup::admit(cx.retained_grant())?;self.owners.lookup=Some(cursor);cx.consume_retained(p)?;cx.consume_fuel(1);self.phase=1;return Ok(false)}
  let parent=parent.try_borrow()?;if parent.get()as*const _!=input.snapshot as*const _{return Err(invalid("export Drawing parent differs from original retained snapshot").into())}let member=drawing_member_at(parent.get(),address)?.ok_or_else(||invalid("export original Drawing member is absent"))?;let ordinal=ordinal(address)?;
  let step=self.owners.lookup.as_mut().unwrap().advance_operation::<DrawingSnapshot,_>(parent,input.children,MemberKeyRef::root(if ordinal==0{"backgroundDrawing"}else{"pages"},&member.handle.child_id),input.operation,cx.retained_grant());let step=paid(cx,step,|step|step.progress)?;let Some(read)=step.read else{return Ok(false)};
  match self.phase{
   1=>{let(cursor,p)=ChildVectorReadCursor::admit(cx.retained_grant())?;self.owners.qualification=Some(cursor);cx.consume_retained(p)?;self.quote=self.owners.qualification.as_ref().unwrap().next_demand(erase_child_admission_source(parent,0x4c41594f),&LAYOUT_DRAWING_REFERENCE_LENDER,ordinal,&read,&DRAWING_CHILD_VECTOR_READ.identity)?;self.phase=2},
   2=>{let step=DRAWING_CHILD_VECTOR_READ.qualify(erase_child_admission_source(parent,0x4c41594f),&LAYOUT_DRAWING_REFERENCE_LENDER,ordinal,&read,self.owners.qualification.as_mut().unwrap(),cx.retained_grant());let step=paid(cx,step,|step|step.progress)?;if let Some(stamp)=step.stamp{self.stamp=Some(stamp);self.phase=3}else{self.quote=self.owners.qualification.as_ref().unwrap().next_demand(erase_child_admission_source(parent,0x4c41594f),&LAYOUT_DRAWING_REFERENCE_LENDER,ordinal,&read,&DRAWING_CHILD_VECTOR_READ.identity)?}},
   3=>{let step=DRAWING_CHILD_VECTOR_READ.admit_original_source(&read,self.stamp.as_ref().unwrap(),&mut self.owners.custody,cx.retained_grant());paid(cx,step,|step|step.progress())?;if self.owners.custody.ready(){self.phase=4}},
   _=>return Err(invalid("export original Drawing read advanced after terminal").into())
  }cx.consume_fuel(1);Ok(self.ready())
 }
 pub fn provider_demand()->Result<RetirementDemand,ValueError>{sum(ChildVectorAdmissionProvider::<DrawingSnapshot>::demand(),drawing_read_demand())}
 fn with_core_read<T>(&self,parent:&ParentSource,address:DrawingSourceAddress,cx:&mut StepContext<'_>,drive:impl FnOnce(&semio_framework_io_ports::ChildAdmissionRead<'_>,&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<Option<T>,OriginalTurnFailure>{
  if !self.ready(){return Err(invalid("export Drawing source custody is not ready").into())}let demand=Self::provider_demand()?;if cx.should_yield()||cx.is_cancelled()||cx.retained_grant().maximum_items<4||!cx.retained_work_can_enter(demand){return Ok(None)}
  let issued=DRAWING_CHILD_VECTOR_READ.admission_provider(&self.owners.custody,ordinal(address)?,cx.retained_grant());let(provider,p)=paid(cx,issued,|step|step.1)?;
  let _=p;let routed=drawing_read_at(parent.try_borrow()?.get(),address,&provider,provider.address(),cx.retained_grant());let routed=paid(cx,routed,|step|step.progress)?;let read=routed.read.ok_or_else(||invalid("export qualified original Drawing route returned no read"))?;
  drive(&read,cx).map(Some)
 }
 pub fn advance_core(&self,parent:&ParentSource,address:DrawingSourceAddress,inner:&mut LayoutExportJob,cx:&mut StepContext<'_>)->Result<semio_s_artifact_layout_layout_engine_export::LayoutExportWorkTurn,OriginalTurnFailure>{Ok(self.with_core_read(parent,address,cx,|read,cx|inner.advance_original_work(cx,Some(read)))?.unwrap_or(semio_s_artifact_layout_layout_engine_export::LayoutExportWorkTurn::Pending))}
 pub fn step_core<'a>(&self,parent:&ParentSource,address:DrawingSourceAddress,inner:&'a mut LayoutExportJob,cx:&mut StepContext<'_>)->LayoutExportTurn<'a>{Ok(self.with_core_read(parent,address,cx,|read,cx|inner.step_with_drawing_read(cx,Some(read)))?.flatten())}

}
pub(super) fn ordinal(address:DrawingSourceAddress)->Result<usize,ValueError>{match address{DrawingSourceAddress::Document=>Ok(0),DrawingSourceAddress::Page(index)=>index.checked_add(1).filter(|ordinal|*ordinal<MAX_DRAWING_SOURCES).ok_or_else(||invalid("export original Drawing page address exceeds its bound"))}}
