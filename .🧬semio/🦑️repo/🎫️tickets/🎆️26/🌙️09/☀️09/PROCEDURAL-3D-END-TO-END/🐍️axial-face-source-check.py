"""Compile and exercise the production trimmed axial face cursor under an in-memory topology contract."""
import contextlib
import io
import json
from pathlib import Path

here=Path(__file__).parent
raw=io.StringIO()
with contextlib.redirect_stdout(raw):
    helper=here/"🐍️point-face-source-check.py"
    exec(compile(helper.read_text().split("solid = (root /",1)[0],str(helper),"exec"),{"__file__":str(helper)})
preamble=raw.getvalue().split("mod solid_publication {",1)[0]
preamble=preamble.replace("use polygon_face_publication::PolygonFacePublication;","").replace("use polygon_wire_publication::{HelixWireRecipe,PolygonWirePublication,PolygonWireRecipe};","")
preamble=preamble.replace("type EdgeId=", "type SurfaceId=usize;type EdgeId=")
preamble=preamble.replace("mod arena{pub(super) use super::{CoedgeId,LoopId,Curve3Id};}","")
preamble=preamble.replace("enum Curve3{Line{origin:Pnt3,dir:Vec3},Other}", "enum Curve3{Line{origin:Pnt3,dir:Vec3},Circle{frame:Frame3,radius:f64},Ellipse{frame:Frame3,width:f64,height:f64}}")
preamble=preamble.replace("enum Surface{Plane{frame:Frame3},Other}", "enum Surface{Cylinder{frame:Frame3,radius:f64},Cone{frame:Frame3,half_angle:f64}}")
print(preamble)
print(r"""
use std::f64::consts::TAU;
impl Pnt3{
 fn from_array(p:EVec3)->Self{pnt(p)}
 fn distance(self,other:Self)->f64{(self-other).norm()}
}
impl Frame3{
 const WORLD:Self=Self{origin:Vec3::ZERO,x:Vec3::new(1.,0.,0.),y:Vec3::new(0.,1.,0.),z:Vec3::Z};
 fn world(self, p:Pnt3)->Pnt3{self.origin+self.x*p.x+self.y*p.y+self.z*p.z}
}
impl Curve2{
 fn eval(&self,t:f64)->Pnt2{let Self::Line{origin,dir}=self;Pnt2::new(origin.x+dir.x*t,origin.y+dir.y*t)}
}
impl Curve3{
 fn eval(&self,t:f64)->Pnt3{
  match *self {
   Self::Line{origin,dir}=>origin+dir*t,
   Self::Circle{frame,radius}=>frame.world(Pnt3::new(radius*t.cos(),radius*t.sin(),0.)),
   Self::Ellipse{frame,width,height}=>frame.world(Pnt3::new(width*t.cos(),height*t.sin(),0.)),
  }
 }
}
impl Surface{
 fn point_original(&self,u:f64,v:f64)->Option<Pnt3>{
  let (frame,r)=match *self {
   Self::Cylinder{frame,radius}=>(frame,radius),
   Self::Cone{frame,half_angle}=>(frame,v*half_angle.tan())
  };
  Some(frame.world(Pnt3::new(r*u.cos(),r*u.sin(),v)))
 }
 fn normal_original(&self,u:f64,v:f64)->Option<Vec3>{
  let (frame,r,slope)=match *self {
   Self::Cylinder{frame,radius}=>(frame,radius,0.),
   Self::Cone{frame,half_angle}=>(frame,v*half_angle.tan(),half_angle.tan()),
  };
  let du=frame.x*(-r*u.sin())+frame.y*(r*u.cos());
  let dv=frame.x*(slope*u.cos())+frame.y*(slope*u.sin())+frame.z;
  du.cross(dv).normalized()
 }
}
mod actual{
""")
implementation=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🪟️axial-face-preview/🦀️.rs")
source=implementation.read_text().split("\n",2)[2]
for header in (
    "use semio_framework_3d_brep_curve::{Curve2, Curve3};",
    "use semio_framework_3d_brep_representation::arena::{CoedgeId, FaceId, LoopId, SurfaceId};",
    "use semio_framework_3d_brep_representation::topology::Body;",
    "use semio_framework_3d_brep_surface::Surface;",
    "use semio_framework_3d_brep_vector::Pnt3;",
):
    source=source.replace(header,"")
print("use super::*;")
print(source)
print(r"""
}
fn build(radius:f64,height:f64,cone:bool)->Body{
 let start=Pnt3::new(radius,0.,0.);
 let top=Pnt3::new(if cone{0.}else{radius},0.,height);
 let h=Frame3{origin:Pnt3::new(0.,0.,height),..Frame3::WORLD};
 let surface=if cone { Surface::Cone{frame:Frame3{y:-h.y,z:-h.z,..h},half_angle:radius.atan2(height)} }
    else {Surface::Cylinder{frame:Frame3::WORLD,radius}};
 let mut body=Body::default();
 body.surfaces=Arena(vec![Some(surface)]);
 body.faces=Arena(vec![Some(Face{surface:0,outer:Some(0),inners:vec![],flipped:false,tol:Tol::DEFAULT,label:PersistentLabel(99)})]);
 body.loops=Arena(vec![Some(Loop{first:0,face:0})]);
 body.vertices=Arena(vec![Some(Vertex{position:start,label:PersistentLabel(20),tol:Tol::DEFAULT}),
   Some(Vertex{position:top,label:PersistentLabel(21),tol:Tol::DEFAULT})]);
 let mut curves=vec![Curve3::Circle{frame:Frame3::WORLD,radius}];
 let edge=|curve,v0,v1,finish| Edge{curve,range:(0.,finish),v0,v1,tol:Tol::DEFAULT,label:PersistentLabel(30+curve as u64)};
 let mut edges=vec![edge(0,0,0,TAU)];
 let seam=if cone{1}else{2};
 if !cone{edges.push(edge(1,1,1,TAU));curves.push(Curve3::Circle{frame:h,radius});}
 curves.push(Curve3::Line{origin:start,dir:top-start});
 edges.push(edge(seam,0,1,1.));
 body.edges=Arena(edges.into_iter().map(Some).collect());
 body.curves3=Arena(curves.into_iter().map(Some).collect());
 let data=if cone {
  vec![(0,true,[TAU,height],[-1.,0.],TAU),(1,true,[0.,height],[0.,-height],1.),(1,false,[TAU,height],[0.,-height],1.)]
 }else{
  vec![(0,true,[0.,0.],[1.,0.],TAU),(2,true,[TAU,0.],[0.,height],1.),
       (1,false,[0.,height],[1.,0.],TAU),(2,false,[0.,0.],[0.,height],1.)]
 };
 let count=data.len();
 for (i,(edge,forward,[u,v],[du,dv],end)) in data.into_iter().enumerate(){
  body.curves2.0.push(Some(Curve2::Line{origin:Pnt2::new(u,v),dir:Vec2::new(du,dv)}));
  body.coedges.0.push(Some(Coedge{edge,forward,pcurve:Some(i),prange:(0.,end),loop_id:0,next:(i+1)%count,prev:(i+count-1)%count}));
 }
 body
}
fn verify(radius:f64,height:f64,deflection:f64,cone:bool,expected:f64){
 use actual::{AxialFacePreview,AxialFacePreviewStep};
 let body=build(radius,height,cone);
 let mut job=AxialFacePreview::new(0,7,deflection);
 let mut ready=None;
 for _ in 0..30_000{
   let mut grant=job.demand(&body,0,7).unwrap();
   assert!(grant.copy_bytes<=128);
   let (step,idle)=job.step(&body,0,7,grant).unwrap();
   assert!(matches!(step,AxialFacePreviewStep::Working));
   assert_eq!(idle.copied_items,0);
   grant.maximum_items=1;
   let (step,paid)=job.step(&body,0,7,grant).unwrap();
   assert!(paid.fits(grant));
   assert_eq!(paid.copied_items,1);
   match step {
    AxialFacePreviewStep::Unsupported=>panic!("native trim was unsupported"),
    AxialFacePreviewStep::Ready(mesh)=>{ready=Some(mesh);break;}
    AxialFacePreviewStep::Working=>{}
   }
 }
 let mesh=ready.expect("supported axial trim preview");
 assert_eq!(mesh.component_references.0[0].1,["99"]);
 assert_eq!(mesh.component_references.0[1].1.len(),if cone{2}else{3});
 assert_eq!(mesh.component_references.0[2].1.len(),2);
 assert_eq!(mesh.vertex_ids.len(),mesh.positions.len()/3);
 assert_eq!(mesh.face_ids.len(),mesh.indices.len()/3);
 let seam=if cone{"31"}else{"32"};
 let seam_id=mesh.component_references.0[1].1.iter().position(|x|x==seam).unwrap() as u32;
 assert_eq!(mesh.edge_ids.iter().filter(|&&id|id==seam_id).count(),1);
 let point=|index:u32|Pnt3::from_array(std::array::from_fn(|axis|mesh.positions[3*index as usize+axis].into()));
 let mut area=0.;
 for tri in mesh.indices.chunks_exact(3){
   let [a,b,c]=[point(tri[0]),point(tri[1]),point(tri[2])];
   let cross=(b-a).cross(c-a);
   let p=a+(b-a)*(1./3.)+(c-a)*(1./3.);
   let radial=Vec3::new(p.x,p.y,0.);
   assert!(cross.dot(radial)>0.);
   let exact_radius=if cone{radius*(height-p.z)/height}else{radius};
   assert!((radial.norm()-exact_radius).abs()<deflection);
   area+=cross.norm()*0.5;
 }
 assert!((area-expected).abs()<deflection);
}
#[test]fn native_rectangular_and_apex_loops(){
""")
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json"))
for row in json.loads(fixture.read_text())["axialFacePreview"]:
    print(f'verify({row["radius"]},{row["height"]},{row["deflection"]},{str(row["kind"]=="cone").lower()},{row["area"]});')
print("}")
