"""Check the exact torus sector topology and preview against the authored Revolve fixture."""
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
source=output.getvalue().split("mod orthogonal_fixture",1)[0]
for key in ["pub use quadratic_pcurve::QuadraticPcurve;","use polygon_face_publication::PolygonFacePublication;",
            "use polygon_wire_publication::{HelixWireRecipe,PolygonWirePublication,PolygonWireRecipe};",
            "enum SolidBoundary{Bore(BoreBoundary),Orthogonal(OrthogonalBoundary)}"]:
    source=source.replace(key,"")
source=source.replace("enum Surface{Plane{frame:Frame3},Other}",
                      "enum Surface{Plane{frame:Frame3},Torus{frame:Frame3,major_radius:f64,minor_radius:f64},Other}")
recipe=(brep/"📸️representation/🔖️contract/🍩️torus-sector/🦀️.rs").read_text()
recipe=recipe[recipe.index("/// 🌀️"):].replace("super::revolution_distance","crate::revolution_distance")
metric=next(brep.rglob("🧭️revolution-distance/🦀️.rs")).read_text().split("/// 🪣️",1)[1]
recipe+="mod revolution_distance{use super::*;use std::f64::consts::TAU;/// 🪣️"+metric+"}"
boundary=(brep/"⚙️engine/🧠️native-registry/🏗️solid-transaction/🍩️sector-boundary/🦀️.rs").read_text()
boundary=boundary[boundary.index("pub(super) fn edge("):]
sampling=(brep/"⚙️engine/📦️mesh-io/🖼️primitive-preview/🍩️sector-sampling/🦀️.rs").read_text().split("\n",4)[4]
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json"))
rows=json.loads(fixture.read_text())["retainedPartialRevolve"]

def array(xs):
    return "["+",".join(str(float(x)) for x in xs)+"]"

helpers=r"""
use std::f64::consts::{PI,TAU};
struct SampleVertex{point:Pnt3,normal:Vec3}
impl Pnt3{fn distance(self,p:Pnt3)->f64{(self-p).norm()}}
impl Frame3{fn to_world(&self,p:Pnt3)->Pnt3{self.origin+self.x*p.x+self.y*p.y+self.z*p.z}}
impl Surface{fn eval(&self,u:f64,v:f64)->Pnt3{match *self{
 Self::Plane{frame}=>frame.to_world(Pnt3::new(u,v,0.)),
 Self::Torus{frame,major_radius,minor_radius}=>{
   let r=major_radius+minor_radius*v.cos();
   frame.to_world(Pnt3::new(r*u.cos(),r*u.sin(),minor_radius*v.sin()))
 }
 _=>unreachable!()
}}}
fn eval_uv(curve:&Curve2,t:f64)->[f64;2]{match curve{
 Curve2::Line{origin,dir}=>[origin.x+dir.x*t,origin.y+dir.y*t],
 Curve2::Ellipse{center,x_axis,major_radius,minor_radius}=>{
  [center.x+x_axis.x*major_radius*t.cos()-x_axis.y*minor_radius*t.sin(),
   center.y+x_axis.y*major_radius*t.cos()+x_axis.x*minor_radius*t.sin()]
 }
 _=>unreachable!()
}}
fn check(origin:EVec3,start:EVec3,axis:EVec3,end:EVec3,angle:f64,major:f64,minor:f64,area:f64,volume:f64){
 let z=(pnt(axis)*angle.signum()).normalized().unwrap();
 let origin=pnt(origin);
 let center=pnt(start)-origin;
 let radial=(center-z*center.dot(z)).normalized().unwrap();
 let frame=Frame3{origin:origin+z*center.dot(z),x:radial,y:z.cross(radial),z};
 let shape=TorusSector::new(frame,major,minor,angle.abs(),1e-7).unwrap();
 assert!(TorusSector::new(frame,major,minor,0.,1e-7).is_none());
 assert!(TorusSector::new(frame,major,minor,TAU-1e-10,1e-7).is_none());
 assert!(TorusSector::new(frame,minor,minor,1.,1e-7).is_none());
 assert!(shape.rim(0).origin.distance(pnt(start))<1e-12);
 assert!(shape.rim(1).origin.distance(pnt(end))<1e-12);
 assert!((shape.volume()-volume).abs()<1e-10);
 let mut uses=[(0,0);3];
 let mut mesh_volume=0.;
 let mut mesh_area=0.;
 for (face,count) in [4,1,1].into_iter().enumerate(){
  let surface=sector_boundary::surface(shape,face);
  let mut previous=None;
  let mut first=None;
  for member in 0..count {
   let trim=sector_boundary::trim(shape,face,member);
   if trim.forward{uses[trim.edge].0+=1;}else{uses[trim.edge].1+=1;}
   let edge=sector_boundary::edge(shape,trim.edge);
   let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
   if let Some(p)=previous{assert_eq!(p,a);}
   first.get_or_insert(a);previous=Some(b);
   for i in [0.,0.25,0.5,0.75,1.] {
    let t=trim.range.0+(trim.range.1-trim.range.0)*i;
    let [u,v]=eval_uv(&trim.curve,t);
    assert!(surface.eval(u,v).distance(edge.curve.eval(t))<1e-12);
   }
  }
  assert_eq!(previous,first);
  for slot in 0..sector_sampling::slots(face,160,128) {
   let triangle=sector_sampling::triangle(shape,face,slot,160,128);
   let [a,b,c]=triangle.each_ref().map(|v|v.point);
   let cross=(b-a).cross(c-a);
   let normals=triangle[0].normal+triangle[1].normal+triangle[2].normal;
   assert!(cross.norm()>0.);
   mesh_area+=cross.norm()/2.;
   mesh_volume+=a.dot(b.cross(c))*if cross.dot(normals)<0.{-1./6.}else{1./6.};
  }
 }
 assert!(uses.into_iter().all(|uses|uses==(1,1)));
 assert!((mesh_area-area).abs()<0.03);
 assert!((mesh_volume-volume).abs()<0.025);
}
#[test]fn signed_partial_revolution_has_exact_capped_and_identified_topology(){
"""
for row in rows:
    coords=",".join(array(row[key]) for key in ["origin","center","axis","end"])
    dims=",".join(str(float(row[key])) for key in ["angle","major","radius","area","volume"])
    helpers+=f"check({coords},{dims});\n"
helpers+="}"
preview="mod sector_sampling{use super::{TorusSector,TAU,SampleVertex as Vertex};"+sampling+"}"
boundary="mod sector_boundary{use super::*;use primitives::planar_pcurve;"+boundary+"}"
with TemporaryDirectory(dir=ticket/"🗑️generated") as folder:
    src=Path(folder)/"sector.rs"
    src.write_text("\n".join([source,recipe,boundary,preview,helpers]))
    exe=src.with_suffix("")
    subprocess.run(["rustup","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe)],check=True)
