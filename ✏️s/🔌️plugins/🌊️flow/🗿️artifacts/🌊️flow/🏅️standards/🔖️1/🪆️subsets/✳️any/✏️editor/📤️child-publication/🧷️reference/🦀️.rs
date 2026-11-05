//! 🧷️ The genuine Flow parent lends its one exact content handle.
use semio_plugin::app::child_read::{ChildReadReference,ChildReadReferenceAdvance,ChildReadReferenceLenderV1};
use semio_plugin::app::child_read::ChildAdmissionSource;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,retained_clone::retained_clone_grant_admits};
pub(super) struct FlowContentReferenceLender;
pub(super) static FLOW_CONTENT_REFERENCE_LENDER:FlowContentReferenceLender=FlowContentReferenceLender;
impl ChildReadReferenceLenderV1 for FlowContentReferenceLender{
 fn demand(&self,_ordinal:usize)->RetirementDemand{RetirementDemand{copy_bytes:2*std::mem::size_of::<RetainedWorkOutcome<ChildReadReferenceAdvance<'static>>>()+std::mem::size_of::<usize>(),depth:1,..Default::default()}}
 fn read<'a>(&self,parent:ChildAdmissionSource<'a>,ordinal:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<ChildReadReferenceAdvance<'a>>{
  let d=self.demand(ordinal);if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(ChildReadReferenceAdvance{progress:Default::default(),done:false,reference:None}),work:0}}let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  let result=(||{let snapshot=parent.get().downcast_ref::<crate::FlowSnapshot>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original content reference requires the actual Flow parent").with_retained_progress(p))?;
  let handle=&snapshot.content;let reference=(ordinal==0).then_some(ChildReadReference{parent_identity:snapshot as*const _ as usize,identity:handle as*const _ as usize,owner:"",slot:"content",child_id:&handle.child_id,artifact_id:&handle.target.artifact_id,artifact_kind:&handle.target.dialect.artifact_kind,standard:&handle.target.dialect.standard,subset:&handle.target.dialect.subset});
  Ok(ChildReadReferenceAdvance{progress:p,done:true,reference})})();RetainedWorkOutcome{result,work:1}
 }
}
