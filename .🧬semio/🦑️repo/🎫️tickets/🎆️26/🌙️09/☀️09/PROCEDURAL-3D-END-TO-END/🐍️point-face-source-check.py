"""Compile the actual polygon publication owners against a minimal in-memory BRep contract."""
import contextlib
import io
import json
from pathlib import Path
import runpy

root = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry")
output = io.StringIO()
with contextlib.redirect_stdout(output):
    runpy.run_path(str(Path(__file__).parent / "🐍️planar-preview-source-check.py"))
preamble = output.getvalue().split("type FaceId=", 1)[0]
preamble = preamble.replace("pub struct PersistentLabel", "#[derive(Clone,Copy,PartialEq)] pub struct PersistentLabel")
preamble = preamble.replace("struct RetirementDemand {copy_bytes:", "struct RetirementDemand {maximum_items:usize,maximum_copy_bytes:usize,release_bytes:usize,copy_bytes:")
preamble = preamble.replace("struct RetainedCloneProgress {copied_items:", "struct RetainedCloneProgress {released_bytes:usize,copied_items:")
preamble = preamble.replace("self.copied_bytes<=g.copy_bytes", "self.copied_items<=g.maximum_items&&self.released_bytes<=g.release_bytes&&self.copied_bytes<=g.copy_bytes")
preamble = preamble.replace("g.depth>=d.depth", "g.depth>=d.depth&&g.release_bytes>=d.release_bytes")
preamble = preamble.replace("pub mod history {", """pub mod history {
    pub struct LabelSource(u64);
    impl LabelSource {
       pub fn next(&self) -> u64 {self.0}
       pub fn from_next(n:u64)->Self{Self(n)}
    }
""")
preamble = preamble.replace("struct Pnt2(f64,f64);impl Pnt2{fn new(x:f64,y:f64)->Self{Self(x,y)}}", "#[derive(Clone,Copy)] struct Pnt2{x:f64,y:f64} impl Pnt2{fn new(x:f64,y:f64)->Self{Self{x,y}}}")
for name in ("a", "b", "c"):
    preamble = preamble.replace(f"{name}.0", f"{name}.x").replace(f"{name}.1", f"{name}.y")
print(preamble.replace("ValueRefusalKind{InvalidValue,", "ValueRefusalKind{InvalidValue,UnsupportedOwner,"))
print(r"""
extern crate self as semio_framework_3d_brep_curve;
extern crate self as semio_framework_3d_brep_construction;
extern crate self as semio_framework_3d_brep_spline_trim;
pub use quadratic_pcurve::QuadraticPcurve;
mod matrix{pub(super) use super::Frame3;}
use std::collections::BTreeMap;
use topology::history::LabelSource;
use polygon_face_publication::PolygonFacePublication;
use polygon_wire_publication::{HelixWireRecipe,PolygonWirePublication,PolygonWireRecipe};
type EVec3=[f64;3];
type NativeVec3=Vec3;
fn pnt(a:EVec3)->Pnt3{Pnt3::new(a[0],a[1],a[2])}
fn vec3(a:EVec3)->Vec3{pnt(a)}
fn evec(p:Pnt3)->EVec3{[p.x,p.y,p.z]}
impl Pnt3 {fn to_array(self)->EVec3{evec(self)}}
#[derive(Clone,Copy)]struct Affine3{linear:[Vec3;3],delta:Vec3}
impl Affine3{
 fn translation(delta:Vec3)->Self{Self{linear:[Vec3::new(1.,0.,0.),Vec3::new(0.,1.,0.),Vec3::Z],delta}}
 fn determinant(self)->f64{self.linear[0].cross(self.linear[1]).dot(self.linear[2])}
 fn max_singular_value(self)->f64{self.linear.iter().map(|v|v.norm()).fold(0., f64::max)}
 fn apply_vector(self,v:Vec3)->Vec3{Vec3::new(self.linear[0].dot(v),self.linear[1].dot(v),self.linear[2].dot(v))}
 fn apply_point(self,p:Pnt3)->Pnt3{self.apply_vector(p)+self.delta}
}
#[derive(Clone,Copy)]struct AnalyticTransform(Affine3);
impl AnalyticTransform{fn map(self)->Result<Affine3,ValueError>{Ok(self.0)}}
impl Vec3{const Z:Self=Self::new(0.,0.,1.);}
impl std::ops::Add for Vec3 {
 type Output=Self;fn add(self,w:Self)->Self{Self::new(self.x+w.x,self.y+w.y,self.z+w.z)}
}
#[derive(Clone,Copy)]struct Frame3{origin:Pnt3,x:Vec3,y:Vec3,z:Vec3}
impl Frame3{
 fn from_normal(origin:Pnt3,normal:Vec3)->Option<Self>{
   let z=normal.normalized()?;
   let [ax,ay,az]=[z.x.abs(),z.y.abs(),z.z.abs()];
   let seed=if ax<=ay && ax<=az{Vec3::new(1.,0.,0.)}else if ay<=az{Vec3::new(0.,1.,0.)}else{Vec3::Z};
   let x=z.cross(seed).normalized()?;
   Some(Self{origin,x,y:z.cross(x),z})
 }
 fn to_local(self,p:Pnt3)->Pnt3{let v=p-self.origin;Pnt3::new(v.dot(self.x),v.dot(self.y),v.dot(self.z))}
}
impl std::ops::Mul<f64> for Vec3 {
 type Output=Self;fn mul(self,a:f64)->Self{Self::new(self.x*a,self.y*a,self.z*a)}
}
type EdgeId=usize;type CoedgeId=usize;type LoopId=usize;type FaceId=usize;type VertexId=usize;type Curve3Id=usize;
mod arena{pub(super) use super::{CoedgeId,LoopId,Curve3Id};}
#[derive(Clone,Copy)]struct Tol(f64);
impl Tol{const DEFAULT:Self=Self(1e-7);fn value(self)->f64{self.0}fn scaled(self,factor:f64)->Self{Self(self.0*factor)}}
struct Vertex{position:Pnt3,label:PersistentLabel,tol:Tol}
struct Edge{curve:usize,range:(f64,f64),v0:usize,v1:usize,tol:Tol,label:PersistentLabel}
struct Coedge{edge:usize,forward:bool,pcurve:Option<usize>,prange:(f64,f64),loop_id:usize,next:usize,prev:usize}
struct Face{surface:usize,outer:Option<usize>,inners:Vec<usize>,flipped:bool,tol:Tol,label:PersistentLabel}
struct Loop{first:usize,face:usize}
fn dummy_coedge()->usize{usize::MAX}
#[derive(Default)]struct Wire{members:Vec<(EdgeId,bool)>,vertices:Vec<VertexId>,closed:bool}
enum Curve3{Line{origin:Pnt3,dir:Vec3},Nurbs{knots:bspline::KnotVector,controls:Vec<Pnt3>,weights:Vec<f64>},Other}
impl Curve3{fn eval(&self,t:f64)->Pnt3{
 match self{Self::Line{origin,dir}=>*origin+*dir*t,Self::Nurbs{knots,controls,weights}=>{
  let s=(t-knots.knots[2])/(knots.knots[3]-knots.knots[2]);let a=[(1.-s)*(1.-s),2.*s*(1.-s),s*s];
  let w=(0..3).map(|i|a[i]*weights[i]).sum::<f64>();
  (0..3).fold(Pnt3::new(0.,0.,0.),|p,i|p+controls[i]*(a[i]*weights[i]/w))
 },_=>unreachable!()}
}}
mod bspline{pub struct KnotVector{pub knots:Vec<f64>,pub degree:usize}}
impl Default for Pnt2{fn default()->Self{Self::new(0.,0.)}}
impl Pnt2{fn dot(self,b:Self)->f64{self.x*b.x+self.y*b.y}}
impl std::ops::Sub for Pnt2{type Output=Self;fn sub(self,b:Self)->Self{Self::new(self.x-b.x,self.y-b.y)}}
type Vec2=Pnt2;
enum Curve2{Line{origin:Pnt2,dir:Vec2},Nurbs{knots:bspline::KnotVector,controls:Vec<Pnt2>,weights:Vec<f64>}}
enum Surface{Plane{frame:Frame3},Other}
#[derive(Clone,Copy)]enum EntityRef{Vertex(usize),Curve3(usize),Edge(usize),Surface(usize),Face(usize),Loop(usize),Curve2(usize),Coedge(usize)}
enum Entity{Wire(Wire,PersistentLabel),Face(usize)}
struct Arena<T>(Vec<Option<T>>);
impl<T> Default for Arena<T>{fn default()->Self{Self(vec![])}}
impl<T> Arena<T>{
 fn get(&self,id:usize)->Option<&T>{self.0.get(id)?.as_ref()}
 fn get_mut(&mut self,id:usize)->Option<&mut T>{self.0.get_mut(id)?.as_mut()}
 fn remove(&mut self,id:usize)->Option<T>{self.0.get_mut(id)?.take()}
 fn len(&self)->usize{self.0.iter().filter(|v|v.is_some()).count()}
 fn next_remove_depth_demand(&self,_:usize)->Result<usize,ValueError>{Ok(1)}
 fn next_insert_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn next_insert_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn reserve_insert_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,RetainedCloneProgress)>{unreachable!()}
 fn insert_reserved(&mut self,v:T,_:RetainedCloneGrant)->Result<(usize,RetainedCloneProgress),(ValueError,T)>{
  let id=self.0.len();self.0.push(Some(v));Ok((id,item()))
 }
}
struct Body{
 labels:LabelSource,vertices:Arena<Vertex>,edges:Arena<Edge>,curves3:Arena<Curve3>,curves2:Arena<Curve2>,
 faces:Arena<Face>,loops:Arena<Loop>,coedges:Arena<Coedge>,surfaces:Arena<Surface>,
}
impl Default for Body{
 fn default()->Self{
  Self{labels:LabelSource::from_next(1),vertices:Default::default(),edges:Default::default(),curves3:Default::default(),
   curves2:Default::default(),faces:Default::default(),loops:Default::default(),coedges:Default::default(),surfaces:Default::default()}
 }
}
impl Body {
 fn new_label(&mut self)->PersistentLabel{let n=self.labels.next();self.labels=LabelSource::from_next(n+1);PersistentLabel(n)}
 fn counts(&self)->[usize;8]{[self.vertices.len(),self.edges.len(),self.curves3.len(),self.curves2.len(),self.faces.len(),self.loops.len(),self.coedges.len(),self.surfaces.len()]}
}
#[derive(Default)]struct Live(BTreeMap<String,Entity>);
impl Live{
 fn contains_key(&self,key:&str)->bool{self.0.contains_key(key)}
 fn next_insert_capacity_byte_demand(&self,_:&str,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_insert_depth_demand(&self,_:&str)->Result<usize,ValueError>{Ok(1)}
 fn reserve_insert_step(&mut self,_:&str,_:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,RetainedCloneProgress)>{unreachable!()}
 fn insert_reserved(&mut self,key:String,v:Entity,_:RetainedCloneGrant)->Result<(Option<Entity>,RetainedCloneProgress),(ValueError,String,Entity)>{
  Ok((self.0.insert(key,v),item()))
 }
}
#[derive(Default)]struct BrepRegistry{live:Live,body:Body,unpublished_topology:usize,shape_mutation_revision:u64}
impl BrepRegistry {
 fn original_query_entity(&self,s:&str)->Result<&Entity,ValueError>{self.live.0.get(s).ok_or(ValueError("missing"))}
 fn tessellation_source_depth_demand(&self,_:&str)->Result<usize,ValueError>{Ok(1)}
 fn advance_live_revision(&mut self){self.shape_mutation_revision+=1;}
}
mod analytic_publication{
 pub(super) fn primitive_handle(prefix:&[u8],label:u64)->[u8;64]{
  let mut bytes=[b'0';64];bytes[0]=prefix[0];let label=format!("{label:016x}");bytes[48..].copy_from_slice(label.as_bytes());bytes
 }
}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
struct RetainedAllocationEffect<'a>(&'a RetirementDemand);
impl RetainedAllocationEffect<'_>{
 fn grant(&self)->RetiredGrant{*self.0}
 fn performed(&mut self,n:usize)->Result<bool,ValueError>{Ok(self.0.maximum_items>=n)}
}
type RetiredGrant=RetainedCloneGrant;
enum RetainedCloneStep{Progress(RetainedCloneProgress),Complete(RetainedCloneProgress)}
impl RetainedCloneStep{
 fn receipt(self)->RetainedCloneProgress{match self{Self::Progress(r)|Self::Complete(r)=>r}}
}
mod solid_publication {
 use super::*;
 type Grant=RetainedCloneGrant;type Progress=RetainedCloneProgress;
 macro_rules! each_entity {($entity:ident,$act:ident)=>{match $entity{
    EntityRef::Vertex(id)=>$act!(vertices,id),EntityRef::Curve3(id)=>$act!(curves3,id),EntityRef::Edge(id)=>$act!(edges,id),
    EntityRef::Surface(id)=>$act!(surfaces,id),EntityRef::Face(id)=>$act!(faces,id),EntityRef::Loop(id)=>$act!(loops,id),
    EntityRef::Curve2(id)=>$act!(curves2,id),EntityRef::Coedge(id)=>$act!(coedges,id),
 }};}
""")
solid = (root / "🏗️solid-transaction/🦀️.rs").read_text()
store = solid[solid.index("pub fn store_demand"):solid.index("impl SolidPublication")]
print(store.replace("<T, Id: ArenaId>", "<T>").replace("Store<T, Id>", "Arena<T>").replace("Option<Id>", "Option<usize>"))
print(solid[solid.index("pub fn topology_slot_depth"):solid.index("impl SolidPublication",solid.index("pub fn topology_slot_depth"))])
print("}")
spline=(root.parent.parent/"💡️queries/🧩tessellation/🪡️spline-trim/🔳️quadratic-pcurve/🦀️.rs").read_text().split("\n",2)[2]
print("mod quadratic_pcurve{"+spline+"}")
suffix = "}"
for name,path in [("polygon_wire_publication","🛤️polygon-wire-publication"),("polygon_face_publication","📐️polygon-face-publication")]:
    source = (root / path / "🦀️.rs").read_text().split("\n",2)[2]
    source = source.replace("use semio_framework_3d_brep_representation::topology::{Edge, Vertex};", "")
    source = source.replace("use semio_framework_3d_brep_representation::topology::{Coedge, Face, Loop, dummy_coedge};", "")
    if name == "polygon_face_publication":
        source = source.replace("use semio_framework_3d_brep_vector::{Pnt2, Vec2};", "").replace("use semio_framework_3d_brep_curve::Curve2;", "")
        submodule = (root / path / "🔁️mapped-boundary/🦀️.rs").read_text().split("\n",2)[2]
        submodule = submodule.replace("use semio_framework_3d_brep_representation::topology::Edge;", "")
        source = source.replace('#[path = "🔁️mapped-boundary/🦀️.rs"]\nmod mapped_boundary;', "mod mapped_boundary {\n" + submodule + "\n}")
        start=source.index("    /// 📦️ Recognizes one axis-aligned horizontal rectangular wire")
        end=source.index("    /// 🗄️ Quotes the live index reservation",start)
        source=source[:start]+source[end:]
    print(f"mod {name} {{\n{source}\n{suffix}")

fixture = next(Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json"))
spec = json.loads(fixture.read_text())
def points(name):
    case = next(row for row in spec["cases"] if row["name"] == name)
    return "[" + ",".join("["+",".join(str(float(v)) for v in point)+"]" for point in case["inputs"]["points"]) + "]"

print(r"""
fn advance(kernel:&mut BrepRegistry,job:&mut PolygonFacePublication,points:&[EVec3])->Result<(),ValueError>{
 let mut d=kernel.polygon_face_topology_demand(job,None)?;
 assert!(d.copy_bytes<=128);
 let point=job.next_point_index().map(|index|points[index]);
 let zero=kernel.step_polygon_face_topology(job,None,point,&mut RetainedAllocationEffect(&d))?;
 assert_eq!(zero.copied_items,0);
 d.maximum_items=1;
 let receipt=kernel.step_polygon_face_topology(job,None,point,&mut RetainedAllocationEffect(&d))?;
 assert_eq!(receipt.copied_items,1);assert!(receipt.fits(d));
 Ok(())
}
fn close(kernel:&mut BrepRegistry,job:&mut PolygonFacePublication){
 for _ in 0..200{
  if job.terminal_is_empty(){return;}
  let mut d=kernel.polygon_face_close_demand(job).unwrap();
  let zero=kernel.close_polygon_face(job,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
  assert_eq!(zero.copied_items,0);
  d.maximum_items=1;
  let paid=kernel.close_polygon_face(job,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
  assert_eq!(paid.copied_items,1);assert!(paid.fits(d));
 }
 panic!("polygon rollback stalled")
}
""")
print(f"const CONCAVE:[[f64;3];5]={points(spec['pointFacePreview']['name'])};")
print(f"const NONPLANAR:[[f64;3];4]={points('face from points that leave their plane')};")
print(r"""
#[test]fn successful_publication_transfers_only_face_topology(){
 let mut kernel=BrepRegistry::default();
 let mut job=kernel.plan_polygon_face_points(CONCAVE.len()).unwrap();
 for _ in 0..500{
  if job.topology_ready(){break;}
  assert!(kernel.live.0.is_empty());
  advance(&mut kernel,&mut job,&CONCAVE).unwrap();
 }
 assert!(job.topology_ready());assert_eq!(kernel.unpublished_topology,2);
 let handle=job.handle().to_owned();
 let grant=RetirementDemand{maximum_items:1,..Default::default()};
 assert!(kernel.publish_polygon_face(&mut job,handle.clone(),grant).is_ok());
 assert_eq!(kernel.unpublished_topology,0);
 assert_eq!(kernel.live.0.len(),1);
 let Entity::Face(id)=kernel.live.0[&handle] else{panic!("expected face")};
 let body=&kernel.body;
 let face=body.faces.get(id).unwrap();
 let lead=body.loops.get(face.outer.unwrap()).unwrap().first;
 let mut member=lead;
 for _ in &CONCAVE {
  let record=body.coedges.get(member).unwrap();
  assert!(record.pcurve.is_some());
  member=record.next;
 }
 assert_eq!(lead,member);
 close(&mut kernel,&mut job);
 assert_eq!(kernel.body.counts(),[5,5,5,5,1,1,5,1]);
}
#[test]fn cancellation_and_invalid_boundary_remove_every_record(){
 for (points,cancel) in [(&CONCAVE[..],true),(&NONPLANAR[..],false)]{
  let mut kernel=BrepRegistry::default();
  let mut job=kernel.plan_polygon_face_points(points.len()).unwrap();
  let mut stopped=false;
  for _ in 0..500{
   assert!(kernel.live.0.is_empty());
   if cancel && kernel.body.coedges.len()>0 {
     assert!(kernel.body.edges.len()>0 && kernel.body.faces.len()>0);
     stopped=true;break;
   }
   if advance(&mut kernel,&mut job,points).is_err(){assert!(!cancel);stopped=true;break;}
  }
  assert!(stopped);
  close(&mut kernel,&mut job);
  assert_eq!(kernel.body.counts(),[0;8]);
  assert_eq!(kernel.unpublished_topology,0);
  assert!(kernel.live.0.is_empty());
 }
}
""")

row = spec["exactSplineFace"]
def array(xs):
    return '['+','.join(array(x) if isinstance(x,list) else str(float(x)) for x in xs)+']'

for label,key,shape in [('CONTROLS','controls','[[f64;3];3]'),('ENDS','ends','[[f64;3];2]'),
                        ('KNOTS','knots','[f64;6]'),('TRIM','trim','[f64;2]'),('WEIGHTS','weights','[f64;3]')]:
    print(f'const {label}:{shape}={array(row[key])};')

print(r"""
#[test]fn rational_face_transfers_exact_pcurve_and_rolls_back_dynamic_payload(){
 for (reverse,abort,bend) in [(false,0,false),(true,0,false),(false,1,false),(false,2,false),(false,0,true)] {
  let mut kernel=BrepRegistry::default();
  let points=[pnt(ENDS[0]),pnt(ENDS[1]),Pnt3::new(0.,0.,0.)];
  let vertices=points.map(|position| {
   let label=kernel.body.new_label();
   kernel.body.vertices.insert_reserved(Vertex{position,label,tol:Tol::DEFAULT},Default::default()).ok().unwrap().0
  });
  let mut authored=CONTROLS.map(pnt);
  if bend{authored[1].z=0.05;}
  let range=(TRIM[0],TRIM[1]);
  let edges=std::array::from_fn::<_,3,_>(|i|{
   let j=(i+1)%3;
   let curve=if i==0{Curve3::Nurbs{knots:bspline::KnotVector{knots:KNOTS.to_vec(),degree:2},controls:authored.to_vec(),weights:WEIGHTS.to_vec()}}
    else{Curve3::Line{origin:points[i],dir:points[j]-points[i]}};
   let curve=kernel.body.curves3.insert_reserved(curve,Default::default()).ok().unwrap().0;
   let label=kernel.body.new_label();
   kernel.body.edges.insert_reserved(Edge{curve,range:if i==0{range}else{(0.,1.)},v0:vertices[i],v1:vertices[j],tol:Tol::DEFAULT,label},Default::default()).ok().unwrap().0
  });
  let order=if reverse{[0,2,1]}else{[0,1,2]};
  let wire=Wire{members:order.map(|i|(edges[i],!reverse)).to_vec(),vertices:vertices.to_vec(),closed:true};
  let label=kernel.body.new_label();
  let source="wire";
  kernel.live.0.insert(source.to_owned(),Entity::Wire(wire,label));
  let mut job=kernel.plan_polygon_face(source,false).unwrap().unwrap();
  let mut stored=0;
  let mut rejected=false;
  for _ in 0..500 {
   if job.topology_ready(){break;}
   let mut d=kernel.polygon_face_topology_demand(&job,Some(source)).unwrap();
   assert!(d.copy_bytes<=128);
   let idle=kernel.step_polygon_face_topology(&mut job,Some(source),None,&mut RetainedAllocationEffect(&d)).unwrap();
   assert_eq!(idle.copied_items,0);
   d.maximum_items=1;
   let receipt=match kernel.step_polygon_face_topology(&mut job,Some(source),None,&mut RetainedAllocationEffect(&d)){
    Ok(r)=>r, Err(_)=>{rejected=true;break;}
   };
   assert!(receipt.fits(d));assert_eq!(receipt.copied_items,1);
   if receipt.retained_capacity_bytes==QuadraticPcurve::STORAGE{stored+=1;}
   if abort==1 && stored==1{break;}
   if abort==2 && kernel.body.curves2.0.iter().any(|c|matches!(c,Some(Curve2::Nurbs{..}))){break;}
  }
  assert_eq!(rejected,bend);
  assert_eq!(stored,usize::from(!bend));
  if abort==0 && !bend{
   assert!(job.topology_ready());
   let key=job.handle().to_owned();
   let grant=RetirementDemand{maximum_items:1,..Default::default()};
   assert!(kernel.publish_polygon_face(&mut job,key,grant).is_ok());
   let spline_use=kernel.body.coedges.0.iter().flatten().find(|u|u.edge==edges[0]).unwrap();
   assert_eq!(spline_use.forward,!reverse);assert_eq!(spline_use.prange, range);
   let Some(Curve2::Nurbs{knots,controls,weights})=kernel.body.curves2.get(spline_use.pcurve.unwrap())else{panic!("missing exact spline");};
   assert_eq!(knots.degree,2);assert_eq!(knots.knots,KNOTS);assert_eq!(weights, &WEIGHTS);
   let Some(Surface::Plane{frame})=kernel.body.surfaces.get(0)else{panic!("missing plane");};
   for (actual,source) in controls.iter().zip(authored){let p=frame.to_local(source);assert_eq!([actual.x,actual.y],[p.x,p.y]);}
  }
  let mut released_pcurve=false;
  for _ in 0..50{
   if job.terminal_is_empty(){break;}
   let mut d=kernel.polygon_face_close_demand(&job).unwrap();
   d.maximum_items=1;
   let receipt=kernel.close_polygon_face(&mut job,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
   assert_eq!(receipt.released_bytes,d.release_bytes);
   if d.release_bytes==QuadraticPcurve::STORAGE{released_pcurve=true;}
  }
  assert!(job.terminal_is_empty());assert_eq!(released_pcurve,abort!=0);
  if abort!=0 || bend{assert_eq!(kernel.body.counts(),[3,3,3,0,0,0,0,0]);assert_eq!(kernel.live.0.len(),1);}
 }
}
""")
