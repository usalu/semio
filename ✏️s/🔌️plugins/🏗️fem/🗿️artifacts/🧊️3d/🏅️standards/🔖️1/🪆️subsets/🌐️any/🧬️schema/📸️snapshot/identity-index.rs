//! 🗂️ Analytical identity indexes borrow ranked source keys and retain only original-funded row locators.
use super::{Fem3dSnapshot, Fem3dSnapshotValue};
use semio_framework_dsl_record::receiving::{NativeSnapshotDecodeOwner,NativeSnapshotFailure,NativeSnapshotDecodeEffectQuote,ReceivingWork,receiving_turn};
use semio_framework_value::RetainedWorkOutcome;
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};
use std::{cmp::Ordering, mem::size_of};

/// 🏷️ Canonical analytical namespaces retain independent source identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FemIdentityKind {
    Node,
    Element,
    Material,
    Section,
    Support,
    LoadCase,
}
impl FemIdentityKind {
    fn index(self) -> u8 {
        match self {
            Self::Node => 0,
            Self::Element => 1,
            Self::Material => 2,
            Self::Section => 3,
            Self::Support => 4,
            Self::LoadCase => 5,
        }
    }
    fn from_index(index: u8) -> Result<Self, ValueError> {
        match index {
            0 => Ok(Self::Node),
            1 => Ok(Self::Element),
            2 => Ok(Self::Material),
            3 => Ok(Self::Section),
            4 => Ok(Self::Support),
            5 => Ok(Self::LoadCase),
            _ => Err(invalid()),
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Element => "element",
            Self::Material => "material",
            Self::Section => "section",
            Self::Support => "support",
            Self::LoadCase => "loadCase",
        }
    }
}
/// 👓️ Immutable authored and actual owners lend constant-time ranked identities until the index retires.
pub trait FemIdentitySource {
    fn identity_count(&self, kind: FemIdentityKind) -> usize;
    fn identity(&self, kind: FemIdentityKind, row: usize) -> Option<&str>;
    fn load_count(&self, case: usize) -> usize;
    fn load_identity(&self, case: usize, row: usize) -> Option<&str>;
    fn identity_source(&self) -> usize
    where
        Self: Sized,
    {
        self as *const Self as usize
    }
}
macro_rules! source{
 ($($source:ty),+)=>{$(impl FemIdentitySource for $source{
  fn identity_count(&self,kind:FemIdentityKind)->usize{match kind{FemIdentityKind::Node=>self.nodes.len(),FemIdentityKind::Element=>self.elements.len(),FemIdentityKind::Material=>self.materials.len(),FemIdentityKind::Section=>self.sections.len(),FemIdentityKind::Support=>self.supports.len(),FemIdentityKind::LoadCase=>self.load_cases.len()}}
  fn identity(&self,kind:FemIdentityKind,row:usize)->Option<&str>{match kind{FemIdentityKind::Node=>(*self.nodes).get(row).map(|v|v.id.as_str()),FemIdentityKind::Element=>self.elements.get(row).map(crate::standards::v1::subsets::any::schema::structure::element_id),FemIdentityKind::Material=>(*self.materials).get(row).map(|v|v.id.as_str()),FemIdentityKind::Section=>self.sections.get(row).map(|v|v.id.as_str()),FemIdentityKind::Support=>self.supports.get(row).map(|v|v.id.as_str()),FemIdentityKind::LoadCase=>self.load_cases.get(row).map(|v|v.id.as_str())}}
  fn load_count(&self,case:usize)->usize{self.load_cases.get(case).map_or(0,|v|v.loads.len())}
  fn load_identity(&self,case:usize,row:usize)->Option<&str>{self.load_cases.get(case)?.loads.get(row).map(crate::standards::v1::subsets::any::schema::structure::load_id)}
 })+};
}
source!(Fem3dSnapshot, Fem3dSnapshotValue);
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "FEM analytical identity is repeated, absent, or outside its original namespace")
}

const KINDS: [FemIdentityKind; 6] = [FemIdentityKind::Node, FemIdentityKind::Element, FemIdentityKind::Material, FemIdentityKind::Section, FemIdentityKind::Support, FemIdentityKind::LoadCase];
const LOAD: u8 = 6;
type Row = (u8, usize, usize);
/// 🧳️ One sorted locator vector retains no borrowed text aliases and closes through the original snapshot frame.
#[derive(Default, RetireOwned)]
pub struct FemIdentityIndex {
    rows: Option<Vec<Row>>,
    source: usize,
    counts: [usize; 6],
}
impl FemIdentityIndex {
    /// 🎟️ Admits coarse cardinalities before scanning, then funds backing, sorting, duplicate checks and lookup metadata.
    pub fn receive<S:FemIdentitySource>(source:&S,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
        receiving_turn(|meter|{
            meter.take(owner.scoped_stage(|owner|owner.receive_nested::<Self,Self,store::NativeSnapshotFailure>(output,&mut |slot,output,birth,birth_unwind,original|receiving_turn(|meter|{
                original.native().begin_stage(0)?;
                let counts=semio_s_fem_receiving::inspect(original,birth,birth_unwind,meter,||Ok(KINDS.map(|kind|source.identity_count(kind))))?;
                let mut total=semio_s_fem_receiving::inspect(original,birth,birth_unwind,meter,||{let total=counts.iter().try_fold(0usize,|sum,count|sum.checked_add(*count)).ok_or_else(invalid)?;if total>64000{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"FEM analytical identity cardinality exceeds native receiving admission"))}else{Ok(total)}})?;
                let bytes=size_of::<Self>();let demand=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
                let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{*slot=Some(Self{rows:None,source:source.identity_source(),counts});*performed=true;*physical=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;
                for case in 0..counts[FemIdentityKind::LoadCase.index() as usize]{
                    total=semio_s_fem_receiving::inspect(original,birth,birth_unwind,meter,||{let next=total.checked_add(source.load_count(case)).ok_or_else(invalid)?;if next>64000{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"FEM analytical identity cardinality exceeds native receiving admission"))}else{Ok(next)}})?;
                }
                let frame=slot.as_mut().unwrap();
                meter.take(original.receive::<Vec<Row>,Vec<Row>,NativeSnapshotFailure>(&mut frame.rows,&mut |rows,output,birth,birth_unwind,native,body|receiving_turn(|meter|{
                    let header=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<Row>>(),maximum_depth:1,..Default::default()};let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,header)?;let turn=quote.run(birth,birth_unwind,&mut |performed,physical|{*rows=Some(Vec::new());*performed=true;*physical=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<Row>>(),..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;
                    let bytes=total.checked_mul(size_of::<Row>()).ok_or_else(invalid)?;
                    body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<Row>>(),maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1})?;
                    meter.admit(native)?;
                    body.allocate_vec_into(native,total,rows.as_mut().unwrap(),1)?;
                    let handoff=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,handoff)?;let turn=quote.run(birth,birth_unwind,&mut |performed,physical|{*output=rows.take();*performed=true;*physical=semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())});meter.take(turn)?;birth.reset_completed()?;Ok(())
                })))?;
                for kind in KINDS{let count=semio_s_fem_receiving::inspect(original,birth,birth_unwind,meter,||Ok(source.identity_count(kind)))?;for row in 0..count{
                    semio_s_fem_receiving::inspect(original,birth,birth_unwind,meter,||{let id=source.identity(kind,row).ok_or_else(invalid)?;protocol::wire::validate_scoped_identity_segments(&[kind.name(),id]).map_err(|_|invalid())})?;
                    semio_s_fem_receiving::push_copy(original,frame.rows.as_mut().unwrap(),(kind.index(),row,0),birth,birth_unwind,meter)?;
                }}
                for case in 0..counts[FemIdentityKind::LoadCase.index() as usize]{
                    let(parent,count)=semio_s_fem_receiving::inspect(original,birth,birth_unwind,meter,||Ok((source.identity(FemIdentityKind::LoadCase,case).ok_or_else(invalid)?,source.load_count(case))))?;
                    for row in 0..count{
                        semio_s_fem_receiving::inspect(original,birth,birth_unwind,meter,||{let id=source.load_identity(case,row).ok_or_else(invalid)?;protocol::wire::validate_scoped_identity_segments(&["load",parent,id]).map_err(|_|invalid())})?;
                        semio_s_fem_receiving::push_copy(original,frame.rows.as_mut().unwrap(),(LOAD,case,row),birth,birth_unwind,meter)?;
                    }
                }
                semio_s_fem_receiving::sort(frame.rows.as_mut().unwrap(),original,birth,birth_unwind,meter,&mut |a,b,owner,progress,unwind,meter|compare(a,b,source,owner,progress,unwind,meter))?;
                for pair in frame.rows.as_ref().unwrap().windows(2){let(a,b)=semio_s_fem_receiving::inspect(original,birth,birth_unwind,meter,||Ok((pair[0],pair[1])))?;if compare(a,b,source,original,birth,birth_unwind,meter)?==Ordering::Equal{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"FEM analytical identity repeats in its namespace").into())}}
                let turn=original.move_existing(slot,output,birth,birth_unwind);meter.take(turn)?;birth.reset_completed()?;Ok(())
            }))))
        })
    }
    /// 🔍️ Returns the exact original rank using cancellable binary search over the validated namespace.
    pub fn lookup<S:FemIdentitySource>(&self,source:&S,kind:FemIdentityKind,id:&str,output:&mut Option<usize>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
        owner.scoped_stage(|owner|owner.receive_nested::<(),usize,NativeSnapshotFailure>(output,&mut |_slot,result,progress,unwind,original|receiving_turn(|meter|{
            semio_s_fem_receiving::inspect(original,progress,unwind,meter,||{if self.source!=source.identity_source()||KINDS.map(|k|source.identity_count(k))!=self.counts{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM identity lookup requires its unchanged original source"))}else{Ok(())}})?;
            original.native().begin_stage(0)?;let rows=semio_s_fem_receiving::inspect(original,progress,unwind,meter,||self.rows.as_deref().ok_or_else(invalid))?;let(mut lo,mut hi)=(0,rows.len());
            while lo<hi{
                let mid=lo+(hi-lo)/2;let(k,row,_)=semio_s_fem_receiving::inspect(original,progress,unwind,meter,||Ok(rows[mid]))?;
                let order=if k!=kind.index(){semio_s_fem_receiving::inspect(original,progress,unwind,meter,||Ok(k.cmp(&kind.index())))?}else{let key=semio_s_fem_receiving::inspect(original,progress,unwind,meter,||source.identity(kind,row).ok_or_else(invalid))?;semio_s_fem_receiving::compare_text(key,id,original,progress,unwind,meter)?};
                match order{Ordering::Less=>lo=mid+1,Ordering::Greater=>hi=mid,Ordering::Equal=>{meter.take(original.receive_copy(result,row))?;return Ok(())}}
            }
            Err(ValueError::literal(ValueRefusalKind::InvalidValue,match kind{FemIdentityKind::Node=>"FEM member or support references an absent node",FemIdentityKind::Material=>"FEM member references an absent material",FemIdentityKind::Section=>"FEM member references an absent section",_=>"FEM analytical reference points to an absent identity"}).into())
        })))
    }

}
fn compare<S:FemIdentitySource>(a:Row,b:Row,source:&S,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,progress:&mut semio_framework_dsl_record::receiving::NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,meter:&mut ReceivingWork)->Result<Ordering,NativeSnapshotFailure>{
 let order=semio_s_fem_receiving::inspect(owner,progress,unwind,meter,||Ok(a.0.cmp(&b.0)))?;if !order.is_eq(){return Ok(order)}
 if a.0==LOAD{
  let(x,y)=semio_s_fem_receiving::inspect(owner,progress,unwind,meter,||Ok((source.identity(FemIdentityKind::LoadCase,a.1).ok_or_else(invalid)?,source.identity(FemIdentityKind::LoadCase,b.1).ok_or_else(invalid)?)))?;
  let order=semio_s_fem_receiving::compare_text(x,y,owner,progress,unwind,meter)?;if !order.is_eq(){return Ok(order)}
  let(x,y)=semio_s_fem_receiving::inspect(owner,progress,unwind,meter,||Ok((source.load_identity(a.1,a.2).ok_or_else(invalid)?,source.load_identity(b.1,b.2).ok_or_else(invalid)?)))?;
  return semio_s_fem_receiving::compare_text(x,y,owner,progress,unwind,meter)
 }
 let kind=semio_s_fem_receiving::inspect(owner,progress,unwind,meter,||FemIdentityKind::from_index(a.0))?;let(x,y)=semio_s_fem_receiving::inspect(owner,progress,unwind,meter,||Ok((source.identity(kind,a.1).ok_or_else(invalid)?,source.identity(kind,b.1).ok_or_else(invalid)?)))?;
 semio_s_fem_receiving::compare_text(x,y,owner,progress,unwind,meter)
}
