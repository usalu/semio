"""Compile and exercise the retained analytic projection implementation."""
import contextlib
import io
import json
from pathlib import Path
import runpy

here = Path(__file__).parent
preamble = io.StringIO()
with contextlib.redirect_stdout(preamble):
    runpy.run_path(str(here / "🐍️planar-preview-source-check.py"))
preamble = preamble.getvalue().split("struct Pnt2", 1)[0]
print(preamble.replace("InvalidValue,WorkLimit", "InvalidValue,UnsupportedOwner,WorkLimit"))
print(r"""
#[derive(Clone,Copy,PartialEq)]
struct Pnt2 {x:f64,y:f64}
impl Pnt2{fn new(x:f64,y:f64)->Self{Self{x,y}}}
type NativeVec3=Vec3;
type EVec3=[f64;3];
fn evec(p:Pnt3)->EVec3{[p.x,p.y,p.z]}
fn pnt(p:EVec3)->Pnt3{Pnt3::new(p[0],p[1],p[2])}
impl std::ops::Add for Vec3 {type Output=Self;fn add(self,v:Self)->Self{Self::new(self.x+v.x,self.y+v.y,self.z+v.z)}}
impl std::ops::Mul<f64> for Vec3 {type Output=Self;fn mul(self,k:f64)->Self{Self::new(self.x*k,self.y*k,self.z*k)}}
impl std::ops::Div<f64> for Vec3 {type Output=Self;fn div(self,k:f64)->Self{Self::new(self.x/k,self.y/k,self.z/k)}}
type Frame3=Frame;
#[derive(Clone,Copy)]
struct Frame{origin:Pnt3,x:Vec3,y:Vec3,z:Vec3}
impl Frame{
 fn to_local(&self,p:Pnt3)->Pnt3{let v=p-self.origin;Pnt3::new(v.dot(self.x),v.dot(self.y),v.dot(self.z))}
 fn to_world(&self,p:Pnt3)->Pnt3{self.origin+self.x*p.x+self.y*p.y+self.z*p.z}
}
enum Curve3{Line{origin:Pnt3,dir:Vec3},Circle{frame:Frame,radius:f64},Ellipse{frame:Frame,major_radius:f64,minor_radius:f64},Nurbs}
impl Curve3 {fn eval(&self,t:f64)->Pnt3{
 match *self{
  Self::Line{origin,dir}=>origin+dir*t,
  Self::Circle{frame,radius}=>frame.to_world(Pnt3::new(radius*t.cos(),radius*t.sin(),0.)),
  Self::Ellipse{frame,major_radius,minor_radius}=>frame.to_world(Pnt3::new(major_radius*t.cos(),minor_radius*t.sin(),0.)),
  _=>unreachable!()
 }
}}
enum Surface{
 Plane{frame:Frame},Cylinder{frame:Frame,radius:f64},Cone{frame:Frame,half_angle:f64},
 Sphere{frame:Frame,radius:f64},Torus{frame:Frame,major_radius:f64,minor_radius:f64},Nurbs
}
impl Surface{fn eval(&self,u:f64,v:f64)->Pnt3{
 let (x,y)=(u.cos(),u.sin());
 match *self{
  Self::Plane{frame}=>frame.to_world(Pnt3::new(u,v,0.)),
  Self::Cylinder{frame,radius}=>frame.to_world(Pnt3::new(radius*x,radius*y,v)),
  Self::Cone{frame,half_angle}=>frame.to_world(Pnt3::new(v*half_angle.tan()*x,v*half_angle.tan()*y,v)),
  Self::Sphere{frame,radius}=>frame.to_world(Pnt3::new(radius*v.cos()*x,radius*v.cos()*y,radius*v.sin())),
  Self::Torus{frame,major_radius,minor_radius}=>{
    let radial=major_radius+minor_radius*v.cos();
    frame.to_world(Pnt3::new(radial*x,radial*y,minor_radius*v.sin()))
  }
  _=>unreachable!()
 }
}}
#[derive(Clone,Copy)]
enum SolidPrimitive{
 Box {origin:Pnt3,width:f64,depth:f64,height:f64},
 Sphere {frame:Frame,radius:f64},Cylinder {frame:Frame,radius:f64,height:f64},
 Cone {frame:Frame,radius:f64,height:f64},Torus {frame:Frame,major:f64,minor:f64},
}
fn frame(origin:Pnt3)->Frame{
 Frame{origin,x:Vec3::new(0.,1.,0.),y:Vec3::new(-1.,0.,0.),z:Vec3::new(0.,0.,1.)}
}
""")
plane = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🎯️planar-closest/🦀️.rs").read_text()
print("mod analysis {", "use super::*;", plane[plane.index("struct Plane"):plane.index("/// 📏️ Quotes the bounded face")],
      plane[plane.index("fn invalid()"):plane.index("fn unsupported()")], "}")
print("mod original{")
source = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🔍️intersection-evaluation-and-measure/🎯️nearest-analytic/🦀️.rs")
print("\n".join(line for line in source.read_text().splitlines()[2:] if not line.startswith("use semio_framework_3d_brep_vector")))
print(r"""
}
fn near(a:f64,b:f64){assert!((a-b).abs()<1e-10,"{a} differs from {b}");}
fn same<const N:usize>(a:[f64;N],b:[f64;N]){for i in 0..N{near(a[i],b[i]);}}
#[test]fn fixture(){
""")
fixture = json.loads(Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️original-analytic-projection.json").read_text())

def point(items):
    return "Pnt3::new(" + ",".join(str(float(v)) for v in items) + ")"

def array(items):
    return "[" + ",".join(str(float(v)) for v in items) + "]"

line = fixture["line"]
ellipse = fixture["ellipse"]
plane = fixture["plane"]
print(f"let start={point(line['start'])};")
print(f"let line=Curve3::Line{{origin:start,dir:{point(line['end'])}-start}};")
print(f"let ellipse=Curve3::Ellipse{{frame:frame(Pnt3::ZERO),major_radius:{float(ellipse['radii'][0])},minor_radius:{float(ellipse['radii'][1])}}};")
for key, row in [("line", line), ("ellipse", ellipse)]:
    print(f"let ([u],p,d)=original::curve_nearest(&{key},{point(row['target'])}).unwrap();")
    print(f"near(u,{row['parameter']});same(p,{array(row['point'])});near(d,{row['distance']});")
print(f"let plane=Surface::Plane{{frame:frame({point(plane['origin'])})}};")
print(f"let (uv,p,d)=original::surface_nearest(&plane,{point(plane['target'])}).unwrap();")
print(f"same(uv,{array(plane['uv'])});same(p,{array(plane['point'])});near(d,{float(plane['distance'])});")
skew = fixture["skewPlane"]
print(f"let skew=Surface::Plane{{frame:Frame{{origin:{point(skew['origin'])},x:{point(skew['xAxis'])},y:{point(skew['yAxis'])},z:{point(skew['normal'])}}}}};")
print(f"let (uv,p,d)=original::surface_nearest(&skew,{point(skew['target'])}).unwrap();")
print(f"same(uv,{array(skew['uv'])});same(p,{array(skew['point'])});near(d,{float(skew['distance'])});")
print("let circle=Curve3::Circle{frame:Frame{origin:Pnt3::ZERO,x:Vec3::new(1.,0.,0.),y:Vec3::new(0.,1.,0.),z:Vec3::new(0.,0.,1.)},radius:2.0};")
for row in fixture["circularEdge"]:
    lo, hi = row["range"]
    print(f"same(evec(original::edge_nearest(&circle,{point(row['target'])},({float(lo)},{float(hi)})).unwrap()),{array(row['point'])});")
for row in fixture["disk"]["points"]:
    print(f"same(evec(original::conic_disk_nearest(&circle,{point(row['target'])}).unwrap()),{array(row['point'])});")
for solid in fixture["solids"]:
    kind = solid["kind"]
    fields = ", ".join(f"{key}:{float(value)}" for key,value in solid["parameters"].items())
    frame = "Frame{origin:Pnt3::ZERO,x:Vec3::new(1.,0.,0.),y:Vec3::new(0.,1.,0.),z:Vec3::new(0.,0.,1.)}"
    placement = "origin:Pnt3::ZERO" if kind == "box" else "frame:" + frame
    print(f"let solid=SolidPrimitive::{kind.title()}{{{placement},{fields}}};")
    for row in solid["points"]:
        print(f"same(evec(original::primitive_nearest(solid,{point(row['target'])}).unwrap()),{array(row['point'])});")
print("}")
