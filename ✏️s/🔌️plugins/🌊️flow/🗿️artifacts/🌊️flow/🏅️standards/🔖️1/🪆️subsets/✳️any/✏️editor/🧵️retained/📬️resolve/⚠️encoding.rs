//! ⚠️ Native request encoding retains its exact primary and independent continuation refusal.
use semio_framework_job::{OriginalDomainProjection,OriginalDomainView as View,OriginalDomainFailure,project_original_value_error,key_original_value_error};
use semio_framework_value::{RetireOwned,RetainedCloneProgress,ValueError,ValueRefusalKind,Number};
#[derive(Debug,RetireOwned)]
pub(crate) struct FlowReplyEncodingFailure{pub error:ValueError,pub continuation_refusal:Option<ValueError>,pub effects:RetainedCloneProgress}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original reply encoding projection path is absent")}
impl OriginalDomainProjection for FlowReplyEncodingFailure{
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{if path.is_empty(){return Ok(View::IntrinsicObject(3))}match path[0]{0=>project_original_value_error(&self.error,&path[1..]),1=>match self.continuation_refusal.as_ref(){Some(error)=>project_original_value_error(error,&path[1..]),None if path.len()==1=>Ok(View::IntrinsicNull),_=>Err(invalid())},2 if path.len()==1=>Ok(View::IntrinsicObject(4)),2 if path.len()==2=>{let value=*[self.effects.copied_items,self.effects.copied_bytes,self.effects.retained_capacity_bytes,self.effects.released_bytes].get(path[1]).ok_or_else(invalid)?;Ok(View::IntrinsicNumber(Number::UInt(u64::try_from(value).map_err(|_|invalid())?)))},_=>Err(invalid())}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty(){return ["error","continuationRefusal","effects"].get(index).copied().ok_or_else(invalid)}match path[0]{0=>key_original_value_error(&self.error,&path[1..],index),1=>self.continuation_refusal.as_ref().ok_or_else(invalid).and_then(|error|key_original_value_error(error,&path[1..],index)),2 if path.len()==1=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(invalid),_=>Err(invalid())}}
}
impl OriginalDomainFailure for FlowReplyEncodingFailure{
 fn domain_code(&self)->&str{"flow.reply-encoding"}
 fn domain_message(&self)->&str{&self.error.message}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.effects}
}
