//! 🧷️ The actual immutable Layout parent lends one exact member handle under a fixed original routing quote.
use semio_framework_plugin::app::vector_read::{ChildVectorReference,ChildVectorReferenceAdvance,ChildVectorReferenceLenderV1};
use semio_framework_io_ports::ChildAdmissionSource;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits}};
use crate::snapshot::LayoutSnapshot;
pub struct LayoutDrawingReferenceLender;
pub static LAYOUT_DRAWING_REFERENCE_LENDER:LayoutDrawingReferenceLender=LayoutDrawingReferenceLender;
impl ChildVectorReferenceLenderV1 for LayoutDrawingReferenceLender{
 fn demand(&self,ordinal:usize)->RetirementDemand{let depth=if ordinal==0{1}else{2};RetirementDemand{copy_bytes:2*std::mem::size_of::<ChildVectorReferenceAdvance<'static>>()+std::mem::size_of::<usize>()*depth,depth,..Default::default()}}
 fn read<'a>(&self,parent:ChildAdmissionSource<'a>,ordinal:usize,g:RetainedCloneGrant)->Result<ChildVectorReferenceAdvance<'a>,ValueError>{let d=self.demand(ordinal);if !retained_clone_grant_admits(g,d){return Ok(ChildVectorReferenceAdvance{progress:Default::default(),done:false,reference:None})}let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let snapshot=parent.get().downcast_ref::<LayoutSnapshot>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Original Drawing reference requires a Layout parent").with_retained_progress(p))?;let member=if ordinal==0{snapshot.background_drawing.as_ref()}else{snapshot.pages.get(ordinal-1).and_then(|page|page.background_drawing.as_ref())};let reference=member.map(|member|{let handle=&member.handle;ChildVectorReference{parent_identity:snapshot as*const _ as usize,identity:handle as*const _ as usize,owner:"",slot:if ordinal==0{"backgroundDrawing"}else{"pages"},child_id:&handle.child_id,artifact_id:&handle.target.artifact_id,artifact_kind:&handle.target.dialect.artifact_kind,standard:&handle.target.dialect.standard,subset:&handle.target.dialect.subset}});Ok(ChildVectorReferenceAdvance{progress:p,done:true,reference})}
}

#[cfg(test)]#[path="🧪️tests.rs"]mod tests;
