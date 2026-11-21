"""🧺️ Verify the production batch cursor and paid planar measure over exact pieces."""
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
kernel=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
parts=runpy.run_path(str(ticket/"🐍️rotated-box-sets-source-check.py"))["parts"]
s=parts[0].replace("struct Body{","struct Body{solids:Arena<Solid>,shells:Arena<Shell>,",1)
s=s.replace("Self{labels:LabelSource::from_next(1)","Self{solids:Default::default(),shells:Default::default(),labels:LabelSource::from_next(1)",1)
parts[0]=s
pattern=next(kernel.rglob("📐️pattern-placement/🦀️.rs")).read_text().split("const MAX_MEMBERS",1)[1]
parts.append("mod layout{use super::*;const MAX_MEMBERS"+pattern+"} use layout::SolidPatternRecipe;")
owner=next(kernel.rglob("🏗️solid-transaction/📐️solid-pattern/🦀️.rs")).read_text().split("const CONTACT_TOLERANCE",1)[1]
parts.append("mod group{use super::*;const CONTACT_TOLERANCE"+owner.replace("map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))","map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, \"fixture bounds\"))")+"} use group::UnpublishedSolidPattern;")
analysis=next(kernel.rglob("🔎️analysis/📦️planar-volume/🦀️.rs")).read_text().split("#[derive(Clone, Copy)]",1)[1]
parts.append("mod planar_measure{use super::*;#[derive(Clone, Copy)]"+analysis+"}")
parts.append(r"""
use std::mem::{size_of,take};
use retained_clone::retained_clone_grant_admits;
impl Affine3{
 fn rotation_axis_angle(_:Vec3,_:f64)->Self{unreachable!()}
}
impl SolidPrimitive{
 fn axis_aligned_bounds(self)->[Pnt3;2]{
  let Self::Box{origin,width,depth,height}=self else{unreachable!();};
  [origin,origin+pnt([width,depth,height])]
 }
}
type SolidId=usize;
type ShellId=usize;
type Progress=RetainedCloneProgress;
type Step=RetainedCloneStep;
impl RetainedCloneStep{fn progress(self)->Progress{self.receipt()}}
struct Solid{outer:ShellId,inners:Vec<ShellId>}
struct Shell{faces:Vec<FaceId>}
impl<T> Arena<T>{
 fn push(&mut self,v:T)->usize{let id=self.0.len();self.0.push(Some(v));id}
 fn slot_count(&self)->usize{self.0.len()}
}
impl Body{
 fn coedge_endpoints(&self,id:CoedgeId)->Option<(VertexId,VertexId)>{
  let trim=self.coedges.get(id)?;
  let e=self.edges.get(trim.edge)?;
  Some(if trim.forward{(e.v0,e.v1)}else{(e.v1,e.v0)})
 }
 fn snapshot(&self)->[usize;10]{
  [self.vertices.0.len(),self.edges.0.len(),self.curves3.0.len(),self.curves2.0.len(),self.faces.0.len(),
   self.loops.0.len(),self.coedges.0.len(),self.surfaces.0.len(),self.shells.0.len(),self.solids.0.len()]
 }
 fn rewind(&mut self,n:[usize;10]){
  self.vertices.0.truncate(n[0]);self.edges.0.truncate(n[1]);self.curves3.0.truncate(n[2]);self.curves2.0.truncate(n[3]);
  self.faces.0.truncate(n[4]);self.loops.0.truncate(n[5]);self.coedges.0.truncate(n[6]);
  self.surfaces.0.truncate(n[7]);self.shells.0.truncate(n[8]);self.solids.0.truncate(n[9]);
 }
}
#[derive(Clone,Copy)]struct SphericalBand;
impl Default for SphericalBand{fn default()->Self{Self}}
impl SphericalBand{
 fn row(&mut self,_:usize,_:&Body,_:&Surface,_:&Coedge,_:&Edge,_:&Curve3)->Result<Option<f64>,ValueError>{unreachable!()}
}
fn area_correction(_:&Curve3,_:&Curve2,_:(f64,f64),_:[Pnt3;2],_:&Frame3)->Result<f64,ValueError>{unreachable!()}
fn populate(body:&mut Body,prism:&PrismBoundary)->SolidId{
 let vertices=(0..prism.vertex_count()).map(|i|{
  let label=body.new_label();
  body.vertices.push(Vertex{position:prism.vertex(i),label,tol:Tol::DEFAULT})
 }).collect::<Vec<_>>();
 let edges=(0..prism.edge_count()).map(|i|{
  let src=prism.edge(i);
  let [v0,v1]=src.vertices.map(|id|vertices[id]);
  let curve=body.curves3.push(src.curve);
  let label=body.new_label();
  body.edges.push(Edge{curve,range:src.range,v0,v1,label,tol:Tol::DEFAULT})
 }).collect::<Vec<_>>();
 let faces=prism.ring_sizes().iter().enumerate().map(|(i,&count)|{
  let surface=body.surfaces.push(prism.surface(i));
  let first=body.coedges.0.len();
  let face=body.faces.0.len();
  let ring=body.loops.push(Loop{first,face});
  for member in 0..count{
   let trim=prism.trim(i,member);
   body.coedges.push(Coedge{edge:edges[trim.edge],forward:trim.forward,pcurve:None,prange:trim.range,
    loop_id:ring,next:first+(member+1)%count,prev:first+(member+count-1)%count});
  }
  let label=body.new_label();
  body.faces.push(Face{surface,outer:Some(ring),inners:Vec::new(),flipped:false,label,tol:Tol::DEFAULT})
 }).collect();
 let outer=body.shells.push(Shell{faces});
 body.solids.push(Solid{outer,inners:Vec::new()})
}
struct SolidPublication {
 shape:SolidBoundary, owner:usize, inserted:Vec<EntityRef>, checkpoint:Option<[usize;10]>, root:Option<SolidId>, committed:bool,closed:bool,
}
impl SolidPublication{
 fn begin(owner:usize,shape:SolidBoundary,_:&mut LabelSource,_:impl FnOnce(u64)->[u8;64])->Result<Self,ValueError>{
  Ok(Self{owner,shape,inserted:Vec::new(),checkpoint:None,root:None,committed:false,closed:false})
 }
 fn topology_ready(&self)->bool{self.root.is_some()}
 fn topology_demand(&self,_:&Body)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 fn advance(&mut self,body:&mut Body,effect:&mut RetainedAllocationEffect<'_>)->Result<Progress,ValueError>{
  if !effect.performed(1)?{return Ok(Progress::default());}
  assert!(self.root.is_none());
  self.checkpoint=Some(body.snapshot());
  let converted;
  let shape=if let SolidBoundary::Analytic(stock)=self.shape{
   converted=OrientedBox::from_primitive(stock).unwrap().boundary().unwrap();
   &converted
  }else{&self.shape};
  let SolidBoundary::BoxPrism(prism,_)=shape else{unreachable!();};
  self.root=Some(populate(body,prism));
  Ok(item())
 }
 fn product(&self,owner:usize)->Result<(SolidId,()),ValueError>{assert_eq!(self.owner,owner);Ok((self.root.unwrap(),()))}
 fn close_demand(&self,_:&Body)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 fn mark_published(&mut self){self.committed=true;}
 fn terminal_is_empty(&self)->bool{self.closed}
 fn close(&mut self,body:&mut Body,effect:&mut RetainedAllocationEffect<'_>)->Result<Step,ValueError>{
  if !effect.performed(1)?{return Ok(Step::Progress(Progress::default()));}
  if !self.committed{if let Some(n)=self.checkpoint{body.rewind(n);}}
  self.closed=true;
  Ok(Step::Complete(item()))
 }
}

fn paid(demand:RetirementDemand)->RetirementDemand{
 RetirementDemand{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,..demand}
}
fn advance_group(body:&mut Body,group:&mut UnpublishedSolidPattern,allow:bool){
 let demand=group.demand(body).unwrap();
 let mut grant=paid(demand);
 grant.maximum_items=usize::from(allow);
 let receipt=group.advance(body,1,0,demand,|_|[b'x';64],&mut RetainedAllocationEffect(&grant)).unwrap();
 assert!(receipt.fits(grant));
}
fn close_group(body:&mut Body,group:&mut UnpublishedSolidPattern) {
 while !group.terminal_is_empty(){
  let grant=paid(group.close_demand(body).unwrap());
  assert!(group.close(body,&mut RetainedAllocationEffect(&grant)).unwrap().progress().fits(grant));
 }
}
fn scalar(body:&Body,source:planar_measure::PlanarMeasureSource,is_area:bool)->f64 {
 let mut job=planar_measure::PlanarMeasure::new(source,is_area);
 (0..3000).find_map(|_|{
  let demand=job.next_demand(body).unwrap();
  assert!(demand.copy_bytes<=128&&demand.depth>0);
  let grant=paid(demand);
  let (out,receipt)=planar_measure::advance_measure(demand,&mut RetainedAllocationEffect(&grant),||job.step(body)).unwrap();
  assert!(receipt.fits(grant));
  out
 }).unwrap()
}
fn batch_case(at:EVec3,tool:EVec3,angle:f64,operation:BooleanOp,expected:[f64;2],pieces:&[[f64;2]]){
 let stock=rotated_box([4.,3.,2.5],0.,[0.;3]);
 let tool=rotated_box(tool,angle,at);
 let make=||UnpublishedSolidPattern::disconnected(stock.disconnected(tool,operation).unwrap());
 let mut body=Body::default();
 let original=body.snapshot();
 for limit in [1,pieces.len()] {
  let mut cancelled=make();
  advance_group(&mut body,&mut cancelled,false);
  assert_eq!(body.snapshot(),original);
  while body.solids.len()<limit{advance_group(&mut body,&mut cancelled,true);}
  assert_eq!(body.solids.len(),limit);
  assert!(!cancelled.ready());
  close_group(&mut body,&mut cancelled);
  assert_eq!(body.snapshot(),original);
 }

 let mut batch=make();
 while !batch.ready(){advance_group(&mut body,&mut batch,true);}
 assert_eq!(batch.draft_count(),pieces.len());
 let roots=batch.take_roots();
 assert_eq!(roots.len(),pieces.len());
 batch.restore_roots(roots);
 assert!(batch.ready());
 let roots=batch.take_roots();
 assert_eq!(batch.commit(),pieces.len());
 close_group(&mut body,&mut batch);
 assert_eq!(body.solids.len(),pieces.len());
 let mut actual=roots.iter().map(|&root|{
  let source=planar_measure::PlanarMeasureSource::Solid(root);
  [scalar(&body,source,false),scalar(&body,source,true)]
 }).collect::<Vec<_>>();
 actual.sort_unstable_by(|a,b|a[0].total_cmp(&b[0]));
 for (a,b) in actual.iter().zip(pieces){
  assert!((a[0]-b[0]).abs()<1e-10&&(a[1]-b[1]).abs()<1e-10);
 }
 let source=planar_measure::PlanarMeasureSource::compound(&roots).unwrap();
 assert!(planar_measure::PlanarMeasureSource::compound(&[roots[0],roots[0]]).is_none());
 assert!((scalar(&body,source,false)-expected[0]).abs()<1e-10);
 assert!((scalar(&body,source,true)-expected[1]).abs()<1e-10);
}
#[test]fn compound_scope_checks_both_original_roots_before_paid_work(){
 let group=group::SolidGroupOwnership::new(3,7,10,None,Some(12)).unwrap();
 let mut reads=Vec::new();
 assert_eq!(group.depth(3,7,|id,key|{assert!(key.is_none());reads.push(id);Ok(id)}).unwrap(),12);
 assert_eq!(reads,[10,12]);
 assert!(group.depth(3,8,|_,_|unreachable!()).is_err());
 assert!(group.depth(4,7,|_,_|unreachable!()).is_err());
}
#[test]fn pattern_still_borrows_only_the_first_original_root(){
 let primitive=box_shape([0.;3],[1.,2.,3.]);
 let SolidBoundary::BoxPrism(initial,_)=OrientedBox::from_primitive(primitive).unwrap().boundary().unwrap()else{unreachable!();};
 let mut body=Body::default();
 assert_eq!(populate(&mut body,&initial),0);
 let original=body.snapshot();
 let mut group=UnpublishedSolidPattern::new(SolidPatternRecipe::linear([1.,0.,0.],4.,3).unwrap(),primitive);
 while !group.ready(){advance_group(&mut body,&mut group,true);}
 assert_eq!(group.draft_count(),2);
 let members=group.take_roots();
 assert_eq!(members,[0,1,2]);
 group.restore_roots(members);
 close_group(&mut body,&mut group);
 assert_eq!(body.snapshot(),original);
}
#[test]fn batch_cancels_and_publishes_complete_exact_measures() {
""")
fixture=runpy.run_path(str(ticket/"🐍️rotated-box-sets-source-check.py"))["fixture"]
def array(values):
    return "["+",".join(repr(float(n)) for n in values)+"]"
for row in fixture["cases"]:
    for op,operator in [("union","Unite"),("difference","Cut")]:
        facts=row["operations"][op]
        if "pieces" not in facts:
            continue
        data=",".join(array(part) for part in facts["pieces"])
        parts.append(f'batch_case({array(row["translation"])},{array(row["tool"])},{row["angleDegrees"]}_f64.to_radians(),BooleanOp::{operator},{array([facts["volume"],facts["area"]])},&[{data}]);')
parts.append("}")
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    rust=Path(folder)/"disconnected-box.rs"
    rust.write_text("\n".join(parts))
    exe=rust.with_suffix(".exe" if os.name=="nt" else "")
    for args in (("rustc","--edition=2024","--test","-Awarnings",str(rust),"-o",str(exe)),(str(exe),)):
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",*args],check=True)
