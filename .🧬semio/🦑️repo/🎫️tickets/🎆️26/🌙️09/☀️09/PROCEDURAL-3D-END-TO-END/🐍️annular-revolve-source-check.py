"""Check actual meridional certification, curved topology and sampling with neutral fixtures."""
from pathlib import Path
import contextlib
import io
import json
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
brep=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
output=io.StringIO()
with contextlib.redirect_stdout(output):
    runpy.run_path(str(ticket/"🐍️point-face-source-check.py"))
source=output.getvalue().replace("enum Surface{Plane{frame:Frame3},Other}",
                              "enum Surface{Plane{frame:Frame3},Cylinder{frame:Frame3,radius:f64},Other}")
recipe=(brep/"📸️representation/🔖️contract/🧿️annular-revolution/🦀️.rs").read_text()
recipe=recipe[recipe.index("/// 🛞️"):].replace("super::revolution_distance","crate::revolution_distance")
metric=next(brep.rglob("🧭️revolution-distance/🦀️.rs")).read_text().split("/// 🪣️",1)[1]
recipe+="mod revolution_distance{use super::*;use std::f64::consts::TAU;/// 🪣️"+metric+"}"
boundary=(brep/"⚙️engine/🧠️native-registry/🏗️solid-transaction/🧿️annular-boundary/🦀️.rs").read_text()
boundary=boundary[boundary.index("fn unsupported()"):]
two_steps=next(brep.rglob("🪜️double-shoulder/🦀️.rs")).read_text().split("\n",2)[2]
boundary="mod two_steps{"+two_steps+"}"+boundary
sampling=(brep/"⚙️engine/📦️mesh-io/🖼️primitive-preview/🧿️annular-sampling/🦀️.rs").read_text().split("\n",4)[4]
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json"))
rows=json.loads(fixture.read_text())["retainedMeridionalRevolve"]

def array(xs):
    return "["+",".join(array(x) if isinstance(x,list) else str(float(x)) for x in xs)+"]"

helpers=r"""
use std::f64::consts::TAU;
struct SampleVertex{point:Pnt3,normal:Vec3}
impl Pnt3{fn distance(self,p:Pnt3)->f64{(self-p).norm()}}
impl Frame3{fn to_world(&self,p:Pnt3)->Pnt3{self.origin+self.x*p.x+self.y*p.y+self.z*p.z}}
impl Surface{fn eval(&self,u:f64,v:f64)->Pnt3{match *self{
 Self::Plane{frame}=>frame.to_world(Pnt3::new(u,v,0.)),
 Self::Cylinder{frame,radius}=>frame.to_world(Pnt3::new(radius*u.cos(),radius*u.sin(),v)),
 _=>unreachable!()
}}}
fn uv(curve:&Curve2,t:f64)->[f64;2]{match curve{
 Curve2::Line{origin,dir}=>[origin.x+dir.x*t,origin.y+dir.y*t],
 Curve2::Ellipse{center,x_axis,major_radius,minor_radius}=>{
  [center.x+x_axis.x*major_radius*t.cos()-x_axis.y*minor_radius*t.sin(),
   center.y+x_axis.y*major_radius*t.cos()+x_axis.x*minor_radius*t.sin()]
 }
 _=>unreachable!()
}}
fn check(origin:EVec3,axis:EVec3,points:[EVec3;4],angle:f64,inner:f64,outer:f64,h:f64,area:f64,volume:f64){
 let mut registry=BrepRegistry::default();
 let mut draft=registry.plan_polygon_face_points(4).unwrap();
 for _ in 0..256{if draft.topology_ready(){break;}advance(&mut registry,&mut draft,&points).unwrap();}
 assert!(draft.topology_ready());
 let id=registry.body.faces.0.iter().position(Option::is_some).unwrap();
 let direction=(pnt(axis)*angle.signum()).normalized().unwrap();
 let shape=annular_boundary::plan_annular_revolution(&registry.body,id,pnt(origin),direction,angle.abs()).unwrap();
 assert!((shape.inner-inner).abs()<1e-12);
 assert!((shape.outer-outer).abs()<1e-12);
 assert!((shape.height-h).abs()<1e-12);
 assert!((shape.volume()-volume).abs()<1e-10);
 assert!(annular_boundary::plan_annular_revolution(&registry.body,id,pnt(origin),direction,0.).is_err());
 assert!(annular_boundary::plan_annular_revolution(&registry.body,id,pnt(points[0]),direction,angle.abs()).is_err());
 assert!(annular_boundary::plan_annular_revolution(&registry.body,id,pnt(origin),shape.frame.y,angle.abs()).is_err());
 assert_eq!(shape.full(),angle.abs()==TAU);
 let mut uses=vec![(0,0);shape.edge_count()];
 for (ring,count) in annular_boundary::ring_sizes(shape).iter().copied().enumerate(){
  let face=annular_boundary::face_index(shape,ring);
  let surface=annular_boundary::surface(shape,face);
  let mut previous=None;
  let mut first=None;
  for member in 0..count {
   let trim=annular_boundary::trim(shape,ring,member);
   if trim.forward!=annular_boundary::face_flipped(shape,face){uses[trim.edge].0+=1;}else{uses[trim.edge].1+=1;}
   let edge=annular_boundary::edge(shape,trim.edge);
   let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
   if let Some(p)=previous{assert_eq!(p,a);}
   first.get_or_insert(a);previous=Some(b);
   for i in [0.,0.25,0.5,0.75,1.] {
    let t=trim.range.0+(trim.range.1-trim.range.0)*i;
    let [u,v]=uv(&trim.curve,t);
    assert!(surface.eval(u,v).distance(edge.curve.eval(t))<1e-12,"face {face}, ring {ring}, member {member}");
   }
  }
  assert_eq!(previous,first);
 }
 assert!(uses.into_iter().all(|uses|uses==(1,1)));
 let mut mesh_volume=0.;
 let mut mesh_area=0.;
 for face in 0..shape.face_count(){
  for slot in 0..annular_sampling::slots(shape,face,256){
   let triangle=annular_sampling::triangle(shape,face,slot,256);
   let [a,b,c]=triangle.each_ref().map(|v|v.point);
   let cross=(b-a).cross(c-a);
   let normals=triangle[0].normal+triangle[1].normal+triangle[2].normal;
   assert!(cross.norm()>0.);
   mesh_area+=cross.norm()/2.;
   mesh_volume+=a.dot(b.cross(c))*if cross.dot(normals)<0.{-1./6.}else{1./6.};
  }
 }
 assert!((mesh_area-area).abs()<0.025);
 assert!((mesh_volume-volume).abs()<0.025);
 close(&mut registry,&mut draft);
}
#[test]fn exact_annular_revolution_preserves_full_and_partial_trims(){
"""
for row in rows:
    coords=",".join(array(row[key]) for key in ["origin","axis","points"])
    dims=",".join(str(float(row[key])) for key in ["angle","inner","outer","height","area","volume"])
    helpers+=f"check({coords},{dims});\n"
helpers+="}"
preview="mod annular_sampling{use super::{AnnularRevolution,SampleVertex as Vertex};"+sampling+"}"
boundary="mod annular_boundary{use super::*;use primitives::planar_pcurve;"+boundary+"}"
with TemporaryDirectory(dir=ticket/"🗑️generated") as folder:
    src=Path(folder)/"annular.rs"
    src.write_text("\n".join([source,recipe,boundary,preview,helpers]))
    exe=src.with_suffix("")
    subprocess.run(["rustup","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe)],check=True)
