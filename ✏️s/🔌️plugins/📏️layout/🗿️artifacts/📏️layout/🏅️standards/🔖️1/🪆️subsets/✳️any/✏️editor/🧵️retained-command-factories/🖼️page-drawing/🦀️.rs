//! 🖼️ Layout retains its original page command and genuine child read throughout every asynchronous turn.
use super::*;
use semio_framework_value::{ValueError,RetirementDemand,RetainedCloneProgress,retained_clone::RetainedCloneSource,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_s_artifact_layout_layout_editor_commands::patch_page::{PatchPage,drawing::source::OriginalPageDrawingSource};

use semio_framework_plugin::app::{vector_read::OriginalChildMemberLookup,MemberKeyRef};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_editor::child_editor::{DRAWING_CHILD_VECTOR_READ,DRAWING_CHILD_SET_ARTBOARD};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_framework_plugin::app::vector_read::command::{OriginalChildEditorCommandCursor,OriginalChildEditorCommandAuthority};
use semio_framework_plugin::ChildEmitPreparation;
fn invalid_page(message:&'static str)->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message)}
fn add_demand(a:RetirementDemand,b:RetirementDemand)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:a.copy_bytes.checked_add(b.copy_bytes).ok_or_else(||invalid_page("Drawing command copy quote overflow"))?,capacity_bytes:a.capacity_bytes.checked_add(b.capacity_bytes).ok_or_else(||invalid_page("Drawing command capacity quote overflow"))?,release_bytes:a.release_bytes.checked_add(b.release_bytes).ok_or_else(||invalid_page("Drawing command release quote overflow"))?,depth:a.depth.max(b.depth)})}

pub(super) type PageCommandSource=RetainedCloneSource<PatchPage>;
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct PageDrawingOwners{pub output:Option<Emit<LayoutMutation,NoConfigMutation>>,pub preparation:Option<ChildEmitPreparation>,pub mutation:Option<DrawingMutation>,pub authority:Option<OriginalChildEditorCommandAuthority>,pub qualifier:Option<OriginalChildEditorCommandCursor>,pub lookup:Option<OriginalChildMemberLookup>,pub drawing:Option<OriginalPageDrawingSource>,pub command:Option<PageCommandSource>}
pub(super) struct PageDrawingWork{pub owners:ControlledRetirement<PageDrawingOwners>,pub closing:bool,pub qualified:bool}
impl PageDrawingWork{
 pub fn new()->Self{Self{owners:ControlledRetirement::retain(PageDrawingOwners{output:None,preparation:None,mutation:None,authority:None,qualifier:None,lookup:None,drawing:None,command:None}),closing:false,qualified:false}}
 pub fn active(&self)->bool{self.owners.original().is_some_and(|owners|owners.command.is_some())}
 pub fn receive(&mut self,command:&mut LayoutCommand,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  let LayoutCommand::PatchPage(payload)=command else{return Ok(true)};
  if !matches!(payload.field.as_str(),"pageDrawingWidth"|"pageDrawingHeight"){return Ok(true)}
  if self.active(){return Ok(true)}
  let birth=PageCommandSource::owned_constructor_demand::<()>();let demand=RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth,copy_bytes:std::mem::size_of::<PatchPage>(),..Default::default()};
  if cx.retained_grant().maximum_items<2||cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(demand){return Ok(false)}
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;
  let original=std::mem::take(payload);
  match PageCommandSource::admit_owned(original,(),cx.retained_grant()){
   Ok((source,progress))=>{self.owners.original_mut().unwrap().command=Some(source);cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(true)},
   Err((error,original,()))=>{*payload=original;Err(error.into())}
  }
 }
 pub fn read_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let owners=self.owners.original().ok_or_else(||invalid_page("Drawing command is closing"))?;
  let Some(source)=owners.drawing.as_ref()else{return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Self>()*2,depth:1,..Default::default()})};
  if source.needs_member_read(){let Some(lookup)=owners.lookup.as_ref()else{return Ok(OriginalChildMemberLookup::birth_demand())};let loan=lookup.demand::<DrawingSnapshot>();return add_demand(loan,source.demand(body.saturating_sub(loan.copy_bytes))?)}source.demand(body)
 }
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let owners=self.owners.original().ok_or_else(||invalid_page("Drawing command is closing"))?;
  let Some(source)=owners.drawing.as_ref().filter(|source|source.ready())else{return self.read_demand(body)};
  if owners.output.is_some(){return Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<Emit<LayoutMutation,NoConfigMutation>>(),depth:1,..Default::default()})}
  if owners.preparation.is_some(){return Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<Emit<LayoutMutation,NoConfigMutation>>()+std::mem::size_of::<ChildEmitPreparation>(),capacity_bytes:std::mem::size_of::<ChildEmitPreparation>(),depth:1,..Default::default()})}
  if owners.mutation.is_some(){let parent=source.original_parent()?;let member=semio_s_artifact_layout_layout_model::drawing_member_at(parent.get(),source.source())?.ok_or_else(||invalid_page("Original page member disappeared"))?;return ChildEmitPreparation::owned_qualified_birth_demand::<DrawingMutation>("pages".len(),member.handle.child_id.len())}
  if owners.authority.is_some(){let command=owners.command.as_ref().unwrap().try_borrow()?;return Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<DrawingMutation>()+command.get().field.len()+command.get().value.len(),depth:1,..Default::default()})}
  let Some(qualifier)=owners.qualifier.as_ref()else{return Ok(OriginalChildEditorCommandCursor::birth_demand())};let quote=if self.qualified{qualifier.take_authority_demand()?}else{qualifier.next_demand()?};add_demand(owners.lookup.as_ref().unwrap().demand::<DrawingSnapshot>(),quote)
 }

 pub fn prepare(&mut self,input:&semio_framework_plugin::retained_command::ArtifactCommandInputs<'_,EditorApp<LayoutPlayApp>>,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if self.owners.original().and_then(|owners|owners.drawing.as_ref()).is_some_and(OriginalPageDrawingSource::ready){return Ok(true)}
  if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(self.read_demand(cx.retained_grant().maximum_copy_bytes)?){return Ok(false)}
  let owners=self.owners.original_mut().ok_or_else(||invalid_page("Drawing command is closing"))?;
  if owners.drawing.is_none(){
   if cx.retained_grant().maximum_items<2{return Ok(false)}
   let command=owners.command.as_ref().ok_or_else(||invalid_page("Original page command is absent"))?.try_borrow()?;
   let configured=input.context.and_then(|context|context.window_config.as_ref());let fallback=blueprint::config::from_snapshot(configured);
   let page_id=command.get().page_id.as_deref().unwrap_or(&fallback.active_page_id);
   let index=input.snapshot.pages.iter().position(|page|page.id==page_id).ok_or_else(||invalid_page("Original page Drawing target is absent"))?;
   if input.snapshot.pages[index].background_drawing.is_none(){return Err(invalid_page("Original page has no Drawing member").into())}
   cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Self>()*2,..Default::default()})?;
   let Some(original)=input.snapshot_read.take_current(input.operation.generation,input.operation.canonical_base_revision,cx)?else{return Ok(false)};
   owners.drawing=Some(OriginalPageDrawingSource::from_original(semio_s_artifact_layout_layout_model::DrawingSourceAddress::Page(index),original));return Ok(false)
  }
  let source=owners.drawing.as_mut().unwrap();if source.ready(){return Ok(true)}
  if !source.needs_member_read(){let result=source.advance_original(input.snapshot,None,cx.retained_grant());cx.consume_retained(source.last_receipt())?;cx.consume_fuel(1);return result.map_err(Into::into)}
  if owners.lookup.is_none(){let(lookup,progress)=OriginalChildMemberLookup::admit(cx.retained_grant())?;owners.lookup=Some(lookup);cx.consume_retained(progress)?;cx.consume_fuel(1);return Ok(false)}
  if cx.retained_grant().maximum_items<4{return Ok(false)}
  let context=input.context.ok_or_else(||invalid_page("Original Drawing command context is absent"))?;let parent=source.original_parent()?;
  let member=semio_s_artifact_layout_layout_model::drawing_member_at(parent.get(),source.source())?.ok_or_else(||invalid_page("Original Drawing member reference disappeared"))?;
  let read=DRAWING_CHILD_VECTOR_READ.read_original(parent,&context.children,MemberKeyRef::root("pages",&member.handle.child_id),input.operation,owners.lookup.as_mut().unwrap(),cx.retained_grant());
  let read=match read{Ok(read)=>read,Err(error)=>{cx.consume_retained(error.retained_progress())?;return Err(error.into())}};cx.consume_retained(read.progress)?;cx.consume_fuel(1);
  let Some(read)=read.read else{return Ok(false)};
  let result=source.advance_original(input.snapshot,Some(&read),cx.retained_grant());cx.consume_retained(source.last_receipt())?;cx.consume_fuel(1);result.map_err(Into::into)
 }


 pub fn step(&mut self,input:&semio_framework_plugin::retained_command::ArtifactCommandInputs<'_,EditorApp<LayoutPlayApp>>,cx:&mut StepContext<'_>)->Result<Option<Emit<LayoutMutation,NoConfigMutation>>,semio_framework_job::OriginalTurnFailure>{
  if !self.prepare(input,cx)?{return Ok(None)}let demand=self.demand(cx.retained_grant().maximum_copy_bytes)?;if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(demand){return Ok(None)}
  let owners=self.owners.original_mut().ok_or_else(||invalid_page("Original page Drawing work is closing"))?;
  if owners.output.is_some(){cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(owners.output.take())}
  if owners.preparation.is_some(){let mut output=Emit::default();output.child_preparations.try_reserve_exact(1).map_err(|_|invalid_page("Original child preparation queue birth failed"))?;let capacity=output.child_preparations.capacity()*std::mem::size_of::<ChildEmitPreparation>();output.child_preparations.push_back(owners.preparation.take().unwrap());owners.output=Some(output);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);return Ok(None)}
  let source=owners.drawing.as_ref().unwrap();
  if owners.mutation.is_some(){let parent=source.original_parent()?;let member=semio_s_artifact_layout_layout_model::drawing_member_at(parent.get(),source.source())?.ok_or_else(||invalid_page("Original page member disappeared"))?;let result=ChildEmitPreparation::admit_owned_qualified::<DrawingSnapshot,DrawingMutation>("pages",&member.handle.child_id,&mut owners.mutation,&mut owners.authority,cx.retained_grant());match result{Ok(Some((preparation,p)))=>{owners.preparation=Some(preparation);cx.consume_retained(p)?},Ok(None)=>{},Err(error)=>{cx.consume_retained(error.retained_progress())?;return Err(error.into())}}cx.consume_fuel(1);return Ok(None)}
  if owners.authority.is_some(){cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;let command=owners.command.as_ref().unwrap().try_borrow()?;let artboard=semio_s_artifact_layout_layout_editor_commands::patch_page::drawing::artboard::from_original(command,source)?;owners.mutation=Some(semio_s_artifact_draw_drawing_mutations::set_artboard::<DrawingMutation>(Some(artboard)));cx.consume_fuel(1);return Ok(None)}
  if owners.qualifier.is_none(){let(qualifier,p)=OriginalChildEditorCommandCursor::admit(cx.retained_grant())?;owners.qualifier=Some(qualifier);cx.consume_retained(p)?;cx.consume_fuel(1);return Ok(None)}
  if cx.retained_grant().maximum_items<2{return Ok(None)}let context=input.context.ok_or_else(||invalid_page("Original child command context disappeared"))?;let parent=source.original_parent()?;let member=semio_s_artifact_layout_layout_model::drawing_member_at(parent.get(),source.source())?.ok_or_else(||invalid_page("Original member reference disappeared"))?;
  let read=DRAWING_CHILD_VECTOR_READ.read_original(parent,&context.children,MemberKeyRef::root("pages",&member.handle.child_id),input.operation,owners.lookup.as_mut().unwrap(),cx.retained_grant());let read=match read{Ok(read)=>read,Err(error)=>{cx.consume_retained(error.retained_progress())?;return Err(error.into())}};cx.consume_retained(read.progress)?;cx.consume_fuel(1);let Some(read)=read.read else{return Ok(None)};
  let qualifier=owners.qualifier.as_mut().unwrap();if self.qualified{match source.take_command_authority(qualifier,&read,cx.retained_grant()){Ok(Some((authority,p)))=>{owners.authority=Some(authority);cx.consume_retained(p)?},Ok(None)=>{},Err(error)=>{cx.consume_retained(error.retained_progress())?;return Err(error.into())}}}else{let command=owners.command.as_ref().unwrap().try_borrow()?;match DRAWING_CHILD_SET_ARTBOARD.qualify_original(parent,command,source.read_qualification()?,&read,qualifier,cx.retained_grant()){Ok(step)=>{cx.consume_retained(step.progress)?;self.qualified=step.stamp.is_some()},Err(error)=>{cx.consume_retained(error.retained_progress())?;return Err(error.into())}}}cx.consume_fuel(1);Ok(None)
 }

 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owners.next_demand(body)}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{let step=self.owners.step(cx.retained_grant());let progress=self.owners.step_progress();cx.consume_retained(progress)?;cx.consume_fuel(1);step?;Ok(if self.owners.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress}}else{semio_framework_job::InteractiveJobCloseStep::Pending{progress}})}
}
