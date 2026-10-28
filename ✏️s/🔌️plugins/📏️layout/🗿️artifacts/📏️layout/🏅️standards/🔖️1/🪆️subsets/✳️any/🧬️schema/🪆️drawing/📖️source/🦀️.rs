//! 📖️ Layout resolves only original member references into prepared genuine child read authority.
use crate::{LayoutDrawingMember,DrawingSourceAddress};
use crate::snapshot::LayoutSnapshot;
use semio_framework_io_ports::{ChildAdmissionAddress,ChildAdmissionProviderV1,ChildAdmissionRead};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits}};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn paid(error:ValueError,own:RetainedCloneProgress)->ValueError{let mut p=error.retained_progress();p.copied_items=p.copied_items.max(own.copied_items);p.copied_bytes=p.copied_bytes.saturating_add(own.copied_bytes);error.with_retained_progress(p)}
/// 🧭️ A source address selects exactly one parent-owned handle without reading authored text or current child content.
pub fn drawing_member_at(snapshot:&LayoutSnapshot,source:DrawingSourceAddress)->Result<Option<&LayoutDrawingMember>,ValueError>{match source{DrawingSourceAddress::Document=>Ok(snapshot.background_drawing.as_ref()),DrawingSourceAddress::Page(index)=>snapshot.pages.get(index).map(|page|page.background_drawing.as_ref()).ok_or_else(||invalid("Layout Drawing page source does not exist"))}}
pub struct DrawingReadAdvance<'a>{pub progress:RetainedCloneProgress,pub done:bool,pub read:Option<ChildAdmissionRead<'a>>}
/// 🎟️ Fixed metadata dispatch is independent of authored child size and never constructs its own grant.
pub fn drawing_read_demand()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<DrawingReadAdvance<'static>>()*2+std::mem::size_of::<ChildAdmissionAddress>()*2,depth:1,..Default::default()}}
/// 🛂️ The exact original handle address was qualified against full edge fields before its provider ordinal became usable.
pub fn drawing_read_at<'a>(snapshot:&LayoutSnapshot,source:DrawingSourceAddress,provider:&'a dyn ChildAdmissionProviderV1,address:ChildAdmissionAddress,g:RetainedCloneGrant)->Result<DrawingReadAdvance<'a>,ValueError>{let d=drawing_read_demand();if !retained_clone_grant_admits(g,d){return Ok(DrawingReadAdvance{progress:Default::default(),done:false,read:None})}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let member=drawing_member_at(snapshot,source).map_err(|error|paid(error,progress))?;let Some(member)=member else{return Ok(DrawingReadAdvance{progress,done:true,read:None})};if address.expected_reference_identity()!=&member.handle as*const _ as usize{return Err(invalid("Layout Drawing original expected member reference changed").with_retained_progress(progress))}let read=provider.read(address).map_err(|error|paid(error,progress))?;Ok(DrawingReadAdvance{progress,done:true,read:Some(read)})}

#[cfg(test)]#[path="🧪️tests.rs"]mod tests;

#[path="🧷️reference/🦀️.rs"]pub mod reference;
pub use reference::LAYOUT_DRAWING_REFERENCE_LENDER;
