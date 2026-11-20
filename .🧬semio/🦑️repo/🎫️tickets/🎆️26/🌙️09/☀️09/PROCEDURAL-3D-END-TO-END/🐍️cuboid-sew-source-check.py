"""Run cuboid sewing certification against six original point-list faces."""
import contextlib
import io
import json
from pathlib import Path
import runpy

here = Path(__file__).parent
capture = io.StringIO()
with contextlib.redirect_stdout(capture):
    runpy.run_path(str(here / "🐍️point-face-source-check.py"))
source = capture.getvalue()
source = source.replace("pub(super) use super::{CoedgeId,LoopId,Curve3Id};", "pub(super) use super::{CoedgeId,LoopId,Curve3Id,FaceId};")
print(source)

root = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry/🏗️solid-transaction/🧵️cuboid-sew")
source = (root / "🦀️.rs").read_text().split("\n",2)[2]
source = source.replace("use semio_framework_3d_brep_representation::{arena::{CoedgeId, FaceId, LoopId}, topology::Body};", "use super::{Body,CoedgeId,FaceId,LoopId};")
source = source.replace("use semio_framework_3d_brep_construction::primitives::SolidPrimitive;", "use super::SolidPrimitive;")
source = source.replace("use semio_framework_3d_brep_surface::Surface;", "use super::Surface;")
print("mod cuboid{"+source+"}")
rectangle = (root.parent / "📐️rectangular-extrusion/🦀️.rs").read_text().split("\n",2)[2]
rectangle = rectangle.replace("use semio_framework_3d_brep_representation::{arena::FaceId,topology::{Body,Wire}};", "use super::{Body,FaceId,Wire};")
rectangle = rectangle.replace("use semio_framework_3d_brep_construction::primitives::SolidPrimitive;", "use super::SolidPrimitive;")
rectangle = rectangle.replace("use semio_framework_3d_brep_surface::Surface;", "use super::Surface;")
rectangle = rectangle.replace("use semio_framework_3d_brep_tolerance::Tol;", "use super::Tol;")
print("mod rectangle{"+rectangle+"}")
fixture = next(Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🐚️brep-topology/🔣️.json"))
row = json.loads(fixture.read_text())["retainedCuboidSew"]
def array(xs):
    return '['+','.join(array(x) if isinstance(x,list) else str(float(x)) for x in xs)+']'
print(f'const CUBOID_RINGS:[[[f64;3];4];6]={array(row["rings"])};')
print(r"""
enum SolidPrimitive{Box{origin:Pnt3,width:f64,depth:f64,height:f64}}
impl Pnt3 {fn from_array(p:[f64;3])->Self{pnt(p)} fn distance(self,other:Self)->f64{(self-other).norm()}}
fn make_sew_face(kernel:&mut BrepRegistry, points:&[EVec3]) -> usize {
 let mut job=kernel.plan_polygon_face_points(points.len()).unwrap();
 for _ in 0..500{
  if job.topology_ready(){break;}
  advance(kernel,&mut job,points).unwrap();
 }
 assert!(job.topology_ready());
 let handle=job.handle().to_owned();
 let grant=RetirementDemand{maximum_items:1,..Default::default()};
 assert!(kernel.publish_polygon_face(&mut job,handle.clone(),grant).is_ok());
 let Entity::Face(id)=kernel.live.0[&handle] else{unreachable!()};
 close(kernel,&mut job);
 id
}
#[test]
fn relocated_rectangle_supports_exact_signed_extrusion(){
 let mut kernel=BrepRegistry::default();
 let points=[[1.,2.,3.],[4.,2.,3.],[4.,6.,3.],[1.,6.,3.]];
 let face=make_sew_face(&mut kernel,&points);
 let result=|direction| rectangle::rectangular_extrusion(&kernel.body,rectangle::RectangleProfile::Face(face),direction).unwrap();
 let Some(SolidPrimitive::Box{origin,width,depth,height})=result([0.,0.,-2.])else{panic!("expected prism box");};
 assert_eq!(origin.to_array(),[1.,2.,1.]);assert_eq!([width,depth,height],[3.,4.,2.]);
 assert!(result([1.,0.,2.]).is_none());
}
#[test]
fn exact_cuboid_certifies_six_quads_refuses_overlap_and_pins_revision() {
 for malformed in 0..4 {
  let mut kernel=BrepRegistry::default();
  let mut rings=CUBOID_RINGS;
  if malformed==1{rings[5]=rings[0];}
  if malformed==2{for p in &mut rings[5]{p[2]-=0.25;}}
  if malformed==3{rings[5].reverse();}
  let faces=rings.map(|ring|make_sew_face(&mut kernel,&ring));
  let before=kernel.body.counts();
  let mut sew=cuboid::CuboidSew::new(1e-6).unwrap();
  let revision=kernel.shape_mutation_revision;
  let mut advanced=0;
  while let Some(i)=sew.next_face(){
   let demand=sew.demand(&kernel.body,faces[i],revision,1).unwrap();
   assert!(demand.copy_bytes<=128);
   sew.step(&kernel.body,faces[i],revision).unwrap();
   advanced+=1;
  }
  assert_eq!(advanced,30);
  assert!(sew.cuboid(revision+1).is_err());
  if malformed==0{
   let SolidPrimitive::Box{origin,width,depth,height}=sew.cuboid(revision).unwrap();
   assert_eq!(origin.to_array(),[1.,2.,3.]);assert_eq!([width,depth,height],[2.,3.,4.]);
  }else{assert!(sew.cuboid(revision).is_err());}
  assert_eq!(kernel.body.counts(),before);
 }
}
""")
