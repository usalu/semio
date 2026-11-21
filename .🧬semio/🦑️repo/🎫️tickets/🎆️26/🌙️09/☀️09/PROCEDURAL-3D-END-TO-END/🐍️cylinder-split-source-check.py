"""✂️ Compile the retained plane-split recipe and exercise neutral rows with a small Rust harness."""
from pathlib import Path
from tempfile import TemporaryDirectory
from math import cos, sin
import json
import os
import subprocess

ticket=Path(__file__).parent
kernel=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
stock=next(kernel.rglob("🧊️analytic-solids/🦀️.rs"))
box_split=(stock.parent/"✂️section/🦀️.rs").read_text().split("impl SolidPrimitive {",1)[1]
recipe=next(kernel.rglob("✂️plane-split/🦀️.rs"))

def vector(values):
    return "["+",".join(repr(float(value)) for value in values)+"]"

source=r"""
extern crate self as semio_framework_3d_brep_vector;
use std::f64::consts::{FRAC_PI_2, PI, TAU};
#[derive(Clone, Copy, PartialEq, Debug)] struct Vec3([f64; 3]);
type Pnt3=Vec3;
mod matrix{pub(super) use super::Frame3;}
impl Vec3 {
    fn from_array(values:[f64;3])->Self{Self(values)}
    fn new(x:f64,y:f64,z:f64)->Self{Self([x,y,z])}
    fn distance(self,b:Self)->f64{(self-b).norm()}
    fn to_array(self)->[f64;3]{self.0}
    fn dot(self,b:Self)->f64{(0..3).map(|i|self.0[i]*b.0[i]).sum()}
    fn cross(self,b:Self)->Self{Self(std::array::from_fn(|i|{
        let [j,k]=[(i+1)%3,(i+2)%3]; self.0[j]*b.0[k]-self.0[k]*b.0[j]
    }))}
    fn norm(self)->f64{self.dot(self).sqrt()}
    fn normalized(self)->Option<Self>{
        let n=self.norm();
        (n.is_finite()&&n>0.).then(||self*(1./n))
    }
}
impl std::ops::Add for Vec3{type Output=Self;fn add(self,b:Self)->Self{Self(std::array::from_fn(|i|self.0[i]+b.0[i]))}}
impl std::ops::Sub for Vec3{type Output=Self;fn sub(self,b:Self)->Self{Self(std::array::from_fn(|i|self.0[i]-b.0[i]))}}
impl std::ops::Mul<f64> for Vec3{type Output=Self;fn mul(self,b:f64)->Self{Self(self.0.map(|a|a*b))}}
impl std::ops::Neg for Vec3{type Output=Self;fn neg(self)->Self{Self(self.0.map(|x|-x))}}
#[derive(Clone, Copy)] struct Frame3{origin:Pnt3,x:Vec3,y:Vec3,z:Vec3}
#[derive(Clone, Copy)] enum SolidPrimitive{
  Box{origin:Pnt3,width:f64,depth:f64,height:f64},Cylinder{frame:Frame3,radius:f64,height:f64},Cone{frame:Frame3,radius:f64,height:f64},Other,
}
impl Frame3{
 fn to_world(self,local:Pnt3)->Pnt3{self.origin+self.x*local.0[0]+self.y*local.0[1]+self.z*local.0[2]}
}
enum SolidBoundary{Analytic(SolidPrimitive),Frustum(ConicalFrustum)}
#[derive(Clone,Copy)] struct Pnt2(f64,f64);
impl Pnt2{fn new(x:f64,y:f64)->Self{Self(x,y)}}
type Vec2=Pnt2;
enum Curve2{Line{origin:Pnt2,dir:Vec2},Circle{origin:Pnt2,x:Vec2,y:Vec2}}
impl Curve2{fn eval(&self,t:f64)->Pnt2{match *self{
 Self::Line{origin,dir}=>Pnt2(origin.0+dir.0*t,origin.1+dir.1*t),
 Self::Circle{origin,x,y}=>Pnt2(origin.0+x.0*t.cos()+y.0*t.sin(),origin.1+x.1*t.cos()+y.1*t.sin()),
}}}
enum Curve3{Line{origin:Pnt3,dir:Vec3},Circle{frame:Frame3,radius:f64}}
impl Curve3{fn eval(&self,t:f64)->Pnt3{match *self{
 Self::Line{origin,dir}=>origin+dir*t,
 Self::Circle{frame,radius}=>frame.to_world(Pnt3::new(radius*t.cos(),radius*t.sin(),0.)),
}}}
enum Surface{Plane{frame:Frame3},Cone{frame:Frame3,half_angle:f64}}
impl Surface{fn eval(&self,u:f64,v:f64)->Pnt3{match *self{
 Self::Plane{frame}=>frame.to_world(Pnt3::new(u,v,0.)),
 Self::Cone{frame,half_angle}=>{
  let r=v*half_angle.tan();
  frame.to_world(Pnt3::new(r*u.cos(),r*u.sin(),v))
 },
}}}
struct PrimitiveEdge{vertices:[usize;2],range:(f64,f64),curve:Curve3}
struct PrimitiveTrim{edge:usize,forward:bool,range:(f64,f64),curve:Curve2}
fn planar_pcurve(curve:&Curve3,plane:Frame3)->Curve2{
 let Curve3::Circle{frame,radius}=*curve else{unreachable!();};
 let projected=|vector:Vec3| Pnt2(vector.dot(plane.x),vector.dot(plane.y));
 Curve2::Circle{origin:projected(frame.origin-plane.origin),x:projected(frame.x*radius),y:projected(frame.y*radius)}
}
struct Tol;
impl Tol{const DEFAULT:Self=Self;fn value(self)->f64{1e-7}}
#[derive(PartialEq,Debug)] enum ValueRefusalKind{UnsupportedOwner,InvalidValue}
#[derive(Debug)] struct ValueError(ValueRefusalKind,&'static str);
impl ValueError{fn literal(kind:ValueRefusalKind,message:&'static str)->Self{Self(kind,message)}}
impl SolidPrimitive{fn validate(self)->Result<(), &'static str>{Ok(())}}
fn cylinder(origin:[f64;3],x:[f64;3],z:[f64;3],radius:f64,height:f64)->SolidPrimitive{
 let (x,z)=(Vec3(x),Vec3(z));
 SolidPrimitive::Cylinder{frame:Frame3{origin:Pnt3::from_array(origin),x,y:z.cross(x),z},radius,height}
}
fn dimensions(shape:SolidPrimitive)->(Frame3,f64,f64){
 let SolidPrimitive::Cylinder{frame,radius,height}=shape else{panic!("expected exactly two cylinders");};
 (frame,radius,height)
}
fn exact_split(stock:SolidPrimitive, origin:[f64;3], normal:[f64;3], center:[f64;3], volumes:[f64;2], lower_index:usize){
 let (frame,radius,height)=dimensions(stock);
 let halves=plane_split(stock,origin,normal).unwrap().map(|shape|{
  let SolidBoundary::Analytic(shape)=shape else{unreachable!();};
  dimensions(shape)
 });
 for ((half,r,h),volume) in halves.into_iter().zip(volumes) {
  assert_eq!((half.x,half.y,half.z,r),(frame.x,frame.y,frame.z,radius));
  assert!((r.powi(2)*h*PI-volume).abs()<1e-10);
 }
 let [(main,_,lower),(top,_,upper)]=[halves[lower_index],halves[1-lower_index]];
 assert_eq!(main.origin,frame.origin);
 assert_eq!(main.origin+main.z*lower,top.origin);
 assert!((top.origin-Vec3(center)).norm()<1e-12);
 assert!((top.origin+top.z*upper-(frame.origin+frame.z*height)).norm()<1e-12);
 assert!((lower+upper-height).abs()<1e-12);
}
"""
source+="impl SolidPrimitive{"+box_split
partition=next(kernel.rglob("✂️axial-section/🦀️.rs")).read_text().split("pub struct AxialPartition",1)[1]
source+="pub struct AxialPartition"+partition
source+=r"""
struct OrientedBox;
impl OrientedBox{
 fn from_primitive(_:SolidPrimitive)->Option<Self>{Some(Self)}
 fn split(self,_:[f64;3],_:[f64;3])->Result<[SolidBoundary;2],ValueError>{unreachable!("oblique box uses its own proof")}
}
"""
dispatcher=recipe.read_text().split("pub fn plane_split",1)[1].split("impl SolidProvenance",1)[0]
source+="pub fn plane_split"+dispatcher
frustum=next(kernel.rglob("📐️conical-frustum/🦀️.rs")).read_text()
source+=frustum[frustum.index("/// 🪣️"):]
cylinder_rows=json.loads((flow/"🔣️original-cylinder-split.json").read_text())
cylinder_data=cylinder_rows["cylinder"]
angle=cylinder_data["angleY"]
x=[cos(angle),0,-sin(angle)]
z=[sin(angle),0,cos(angle)]
source+="\n#[test]fn fixture_orders_both_halves_without_changing_their_frames(){\n"
source+=f'let stock=cylinder({vector(cylinder_data["origin"])},{vector(x)},{vector(z)},{float(cylinder_data["radius"])},{float(cylinder_data["height"])});\n'
for row in cylinder_rows["cases"]:
    volumes=[row["positive"]["volume"],row["negative"]["volume"]]
    lower=0 if sum(n*d for n,d in zip(row["normal"],z))<0 else 1
    inputs=[row["origin"],row["normal"],row["sectionCenter"],volumes]
    source+="exact_split(stock,"+",".join(map(vector,inputs))+f",{lower});\n"
for row in cylinder_rows["refused"]:
    source+=f'assert!(plane_split(stock,{vector(row["origin"])},{vector(row["normal"])}).is_err());\n'
source+=r"""
}
#[test]fn oblique_stock_accepts_only_a_perpendicular_plane(){
 let shape=cylinder([1.,-3.,5.],[0.8,0.,-0.6],[0.6,0.,0.8],2.,6.);
 let expected=[9.*PI,15.*PI];
 exact_split(shape,[2.35,21.,6.8],[-0.6,0.,-0.8],[2.35,-3.,6.8],expected,0);
 exact_split(shape,[2.35,21.,6.8],[6e-300,0.,8e-300],[2.35,-3.,6.8],[expected[1],expected[0]],1);
 assert!(matches!(plane_split(shape,[2.35,21.,6.8],[0.6,1e-11,0.8]),Err(ValueError(ValueRefusalKind::InvalidValue,_))));
 assert!(matches!(plane_split(SolidPrimitive::Other,[0.;3],[0.,0.,1.]),Err(ValueError(ValueRefusalKind::UnsupportedOwner,_))));
}
#[test]fn boxes_keep_existing_order_and_volume(){
 let stock=SolidPrimitive::Box{origin:Vec3([1.,2.,3.]),width:4.,depth:6.,height:8.};
 for (origin,normal,expected) in [([3.,12.,40.],[2.,0.,0.],[96.,96.]),([0.,0.,9.],[0.,0.,1.],[48.,144.])]{
  let pair=plane_split(stock,origin,normal).unwrap();
  let volumes=pair.map(|shape| match shape{SolidBoundary::Analytic(SolidPrimitive::Box{width,depth,height,..})=>width*depth*height,_=>panic!()});
  assert_eq!(volumes,expected);
 }
}
"""
source+=r"""
fn exact_cone_split(stock:SolidPrimitive,origin:[f64;3],normal:[f64;3],rim:[f64;3],radius:f64,volumes:[f64;2],cone_index:usize){
 let SolidPrimitive::Cone{frame:stock_frame,radius:stock_radius,height:stock_height}=stock else{unreachable!();};
 let [first,second]=plane_split(stock,origin,normal).unwrap();
 let (cone,wide)=if cone_index==0{(first,second)}else{(second,first)};
 let SolidBoundary::Analytic(SolidPrimitive::Cone{frame,radius:r,height})=cone else{panic!("missing tip cone");};
 let SolidBoundary::Frustum(wide)=wide else{panic!("missing base frustum");};
 assert!((r-radius).abs()<1e-12);
 assert!(frame.origin.distance(Vec3(rim))<1e-12&&frame.z==stock_frame.z);
 assert_eq!(wide.frame.z,frame.z*(-1.));
 assert!((wide.height+height-stock_height).abs()<1e-12);
 assert!(wide.frame.origin.distance(frame.origin)<1e-12);
 let apex=stock_frame.origin+stock_frame.z*stock_height;
 assert!(wide.apex().distance(apex)<1e-12&& (frame.origin+frame.z*height).distance(apex)<1e-12);
 assert!((wide.radii[0]-r).abs()<1e-12&&(wide.radii[1]-stock_radius).abs()<1e-12);
 let cap=|face|match wide.surface(face){Surface::Plane{frame}=>frame,_=>panic!("missing cap")};
 assert_eq!(cap(1).z,stock_frame.z);
 assert_eq!(cap(2).z,stock_frame.z*(-1.));
 assert!((wide.volume-volumes[1-cone_index]).abs()<1e-10);
 let cone_volume=PI/3.*r*r*height;
 assert!((cone_volume-volumes[cone_index]).abs()<1e-10);
 assert!((cone_volume+wide.volume-PI/3.*stock_radius*stock_radius*stock_height).abs()<1e-10);
 let mut uses=[[0;2];3];
 for (face,members) in ConicalFrustum::RINGS.iter().copied().enumerate(){
  let surface=wide.surface(face);
  let mut ring=Vec::new();
  for member in 0..members{
   let trim=wide.trim(face,member);
   let edge=wide.edge(trim.edge);
   let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
   ring.push([a,b]);
   uses[trim.edge][usize::from(trim.forward)]+=1;
   for fraction in [0.,0.4,1.]{
    let t=trim.range.0+(trim.range.1-trim.range.0)*fraction;
    let uv=trim.curve.eval(t);
    assert!(surface.eval(uv.0,uv.1).distance(edge.curve.eval(t))<1e-12);
   }
  }
  assert!((0..members).all(|i|ring[i][1]==ring[(i+1)%members][0]));
 }
 assert_eq!(uses,[[1,1];3]);
}
#[test]fn cone_fixture_preserves_rims_original_apex_and_volume(){
"""
cone_rows=json.loads((flow/"🔣️original-cone-split.json").read_text())
cone_data=cone_rows["cone"]
angle=cone_data["angleY"]
x=[cos(angle),0,-sin(angle)]
z=[sin(angle),0,cos(angle)]
source+=f'let x=Vec3({vector(x)}); let z=Vec3({vector(z)}); let frame=Frame3{{origin:Vec3({vector(cone_data["origin"])}),x,y:z.cross(x),z}};'+chr(10)
source+=f'let stock=SolidPrimitive::Cone{{frame,radius:{float(cone_data["radius"])},height:{float(cone_data["height"])}}};'+chr(10)
for row in cone_rows["cases"]:
    volumes=[row["positive"]["volume"],row["negative"]["volume"]]
    inputs=[row["origin"],row["normal"],row["sectionCenter"]]
    cone_index=int(row["negative"]["kind"]=="cone")
    source+="exact_cone_split(stock,"+",".join(map(vector,inputs))+f',{float(row["sectionRadius"])},{vector(volumes)},{cone_index});'+chr(10)
for row in cone_rows["refused"]:
    source+=f'assert!(plane_split(stock,{vector(row["origin"])},{vector(row["normal"])}).is_err());'+chr(10)
source+="}"+chr(10)
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    module=Path(folder)/"split.rs"
    module.write_text(source)
    output=module.with_name("split.exe" if os.name=="nt" else "split")
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(module),"-o",str(output)],check=True)
    subprocess.run([str(output)],check=True)
