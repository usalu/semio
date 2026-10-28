//! 📖️ The outer original retained owner lends one Drawing read to one publication-free export work turn.
use super::*;
use semio_framework_io_ports::ChildAdmissionRead;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum LayoutExportWorkTurn{Pending,Progress,Ready}
semio_framework_value::artifact_retire_leaf!(LayoutExportWorkTurn);
pub(super) fn require_original_drawing_read<'a,T>(source:Option<DrawingSourceAddress>,read:Option<&'a T>)->Result<Option<&'a T>,ValueError>{if source.is_some()&&read.is_none(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Layout export requires the original qualified Drawing child read"))}Ok(read)}
impl LayoutExportJob{
 /// 🧭️ Returns only the active numeric address; lookup, qualification and source custody belong to the actual outer owner.
 pub fn required_drawing_source(&self)->Option<DrawingSourceAddress>{if self.closing||self.stage!=ExportStage::Plan||!self.drawing.as_ref().is_some_and(DrawingCompositionState::has_active){None}else{self.drawing_source}}
 /// ♻️ Original parent and child custody may close after every retained Drawing consumer and retirement has returned.
 pub fn drawing_consumers_terminal_is_empty(&self)->bool{self.drawing.is_none()&&self.drawing_retirement.is_none()}
 /// 📏️ Quotes the last selected original plan event and the same work milestone metadata.
 pub fn original_work_demand(&self)->Result<RetirementDemand,ValueError>{let own=RetirementDemand{copy_bytes:size_of::<u64>()*2+if self.restore_target.is_some(){size_of::<Option<LayoutExportCheckpoint>>()}else{0},depth:1,..Default::default()};let child=match self.stage{ExportStage::Plan=>self.plan_work_demand,ExportStage::BeginEncode|ExportStage::Encode=>self.output_work_demand(false)?,ExportStage::Base64=>self.output_work_demand(true)?,_=>Default::default()};Ok(RetirementDemand{copy_bytes:own.copy_bytes.checked_add(child.copy_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Layout export original work quote overflow"))?,depth:own.depth.max(child.depth),..child})}
 /// 🧵️ Advances one actual unit and its original milestone without minting a second owner's publications.
 pub fn advance_original_work(&mut self,context:&mut StepContext<'_>,read:Option<&ChildAdmissionRead<'_>>)->Result<LayoutExportWorkTurn,semio_framework_job::OriginalTurnFailure>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout export original work is closing").into())}
  if self.slot.publication.is_some()||self.slot.delivered{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout export original work cannot consume another publication owner's slot").into())}
  if context.is_cancelled()||context.should_yield(){return Ok(LayoutExportWorkTurn::Pending)}
  if (context.operation(),context.generation())!=self.source_identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout export original work requires its initiating operation").into())}
  if self.stage==ExportStage::Complete{return Ok(LayoutExportWorkTurn::Ready)}
  let grant=context.retained_grant();let restoring=self.restore_target.is_some();let own_items=2+usize::from(restoring);let own_copy=size_of::<u64>()*2+if restoring{size_of::<Option<LayoutExportCheckpoint>>()}else{0};if grant.maximum_items<own_items||grant.maximum_copy_bytes<own_copy||grant.maximum_depth==0{return Ok(LayoutExportWorkTurn::Pending)}
  context.set_stage(self.stage_name());let before=self.milestone();context.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_items:grant.maximum_items-own_items,maximum_copy_bytes:grant.maximum_copy_bytes-own_copy,..grant},|same|self.advance_one(same,read))?;let committed=checkpoint::committed_milestone(before,self.milestone());self.commit_work_turn(committed,context)?;context.consume_fuel(1);
  Ok(if self.stage==ExportStage::Complete{LayoutExportWorkTurn::Ready}else{LayoutExportWorkTurn::Progress})
 }
}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;
