"""Check the exact conical loft topology and preview against the neutral section fixtures."""
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
source=source.replace("enum Surface{Plane{frame:Frame3},Other}","enum Surface{Plane{frame:Frame3},Cone{frame:Frame3,half_angle:f64},Other}")
recipe=(brep/"🛠️operations/📐️conical-frustum/🦀️.rs").read_text()
recipe=recipe[recipe.index("/// 🪣️"):]
sampling=(brep/"⚙️engine/📦️mesh-io/🖼️primitive-preview/📐️frustum-sampling/🦀️.rs").read_text().split("\n",4)[4]
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json"))
rows=json.loads(fixture.read_text())["retainedConicalLoft"]

def array(xs):
    return "["+",".join(str(float(x)) for x in xs)+"]"

helpers=r"""
use primitives::planar_pcurve;
use std::f64::consts::{FRAC_PI_2,PI,TAU};
struct SampleVertex{point:Pnt3,normal:Vec3}
impl Pnt3{fn distance(self,p:Pnt3)->f64{(self-p).norm()}}
impl Frame3{fn to_world(&self,p:Pnt3)->Pnt3{self.origin+self.x*p.x+self.y*p.y+self.z*p.z}}
impl Surface{fn eval(&self,u:f64,v:f64)->Pnt3{match *self{
 Self::Plane{frame}=>frame.to_world(Pnt3::new(u,v,0.)),
 Self::Cone{frame,half_angle}=>{let r=v*half_angle.tan();frame.to_world(Pnt3::new(r*u.cos(),r*u.sin(),v))},
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
fn check(start:[f64;3],end:[f64;3],normal:[f64;3],r:f64,s:f64,volume:f64,area:f64){
 let (start,end,small,large)=if r<s{(pnt(start),pnt(end),r,s)}else{(pnt(end),pnt(start),s,r)};
 let travel=end-start;
 let mut frame=Frame3::from_normal(start,pnt(normal)).unwrap();
 if travel.dot(frame.z)<0.{frame.y=-frame.y;frame.z=-frame.z;}
 let shape=ConicalFrustum::new(frame,[small,large],travel.norm()).unwrap();
 assert!((shape.volume-volume).abs()<1e-10);
 assert!((shape.rim(1).origin-end).norm()<1e-12);
 assert!(ConicalFrustum::new(frame,[0.,large],travel.norm()).is_none());
 assert!(ConicalFrustum::new(frame,[small,large],0.).is_none());
 assert!(ConicalFrustum::new(frame,[large,small],travel.norm()).is_none());
 let mut uses=[(0,0);3];
 let mut mesh_volume=0.;
 let mut mesh_area=0.;
 for (face,&count) in ConicalFrustum::RINGS.iter().enumerate(){
  let surface=shape.surface(face);
  let mut previous=None;
  let mut first=None;
  for member in 0..count{
   let trim=shape.trim(face,member);
   if trim.forward{uses[trim.edge].0+=1;}else{uses[trim.edge].1+=1;}
   let edge=shape.edge(trim.edge);
   let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
   if let Some(p)=previous{assert_eq!(p,a);}
   first.get_or_insert(a);previous=Some(b);
   for s in [0.,0.25,0.5,0.75,1.] {
    let t=trim.range.0+(trim.range.1-trim.range.0)*s;
    let [u,v]=eval_uv(&trim.curve,t);
    assert!((surface.eval(u,v)-edge.curve.eval(t)).norm()<1e-12);
   }
  }
  assert_eq!(previous,first);
  for slot in 0..frustum_sampling::slots(face,192) {
   let triangle=frustum_sampling::triangle(shape,face,slot,192);
   let [a,b,c]=triangle.each_ref().map(|v|v.point);
   let cross=(b-a).cross(c-a);
   let normals=triangle[0].normal+triangle[1].normal+triangle[2].normal;
   assert!(cross.norm()>0.);
   mesh_area+=cross.norm()/2.;
   mesh_volume+=a.dot(b.cross(c))*if cross.dot(normals)<0.{-1./6.}else{1./6.};
  }
 }
 assert!(uses.into_iter().all(|uses|uses==(1,1)));
 assert!((mesh_area-area).abs()<0.02);
 assert!((mesh_volume-volume).abs()<0.02);
}
#[test]fn conical_loft_recipes_keep_the_exact_caps_trims_and_preview(){
"""
for row in rows:
    helpers+=f"check({array(row['start'])},{array(row['end'])},{array(row['normal'])},{row['radius0']},{row['radius1']},{row['volume']},{row['area']});\n"
helpers+="}"
preview="mod frustum_sampling{use super::{ConicalFrustum,TAU,SampleVertex as Vertex};"+sampling+"}"
with TemporaryDirectory(dir=ticket/"🗑️generated") as folder:
    src=Path(folder)/"loft.rs"
    src.write_text("\n".join([source,recipe,preview,helpers]))
    exe=src.with_suffix("")
    subprocess.run(["rustup","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe)],check=True)
