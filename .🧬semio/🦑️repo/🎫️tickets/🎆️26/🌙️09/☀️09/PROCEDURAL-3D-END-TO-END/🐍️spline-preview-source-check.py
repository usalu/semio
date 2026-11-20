"""Exercise the production rational patch algebra and preview cursor with neutral fixtures."""
import contextlib
import io
import json
from pathlib import Path
import runpy

base = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
here = Path(__file__).parent
preamble = io.StringIO()
with contextlib.redirect_stdout(preamble):
    runpy.run_path(str(here / "🐍️planar-preview-source-check.py"))
print(preamble.getvalue().split("struct Frame", 1)[0])
print(r"""
extern crate self as semio_framework_3d_brep_curve;
impl Vec3 {fn distance(self,b:Self)->f64{(self-b).norm()}}
impl std::ops::Add for Vec3 {type Output=Self;fn add(self,v:Self)->Self{Self::new(self.x+v.x,self.y+v.y,self.z+v.z)}}
impl std::ops::Mul<f64> for Vec3 {type Output=Self;fn mul(self,k:f64)->Self{Self::new(self.x*k,self.y*k,self.z*k)}}
mod bspline{
pub const ORIGINAL_SPLINE_MAXIMUM_DEGREE:usize=32;
pub struct KnotVector{pub degree:usize,pub knots:Vec<f64>}
impl KnotVector{
 pub fn find_span(&self,t:f64)->usize{
   let end=self.knots.len()-self.degree-1;
   (self.degree..end).find(|i|self.knots[*i]<=t&&t<self.knots[*i+1]).unwrap_or(end-1)
 }
}
""")
knots = (base / "📸️representation/➰️curve/🪢️bspline/🦀️.rs").read_text()
print(knots[knots.index("pub fn homogeneous_jet_at_span_original("):knots.index("// #endregion 🔖️DeBoor")])
print("}")
print(r"""
use bspline::KnotVector;
struct Frame {origin:Pnt3,x:Vec3,y:Vec3,z:Vec3}
impl Frame{const WORLD:Self=Self{origin:Vec3::ZERO,x:Vec3::new(1.,0.,0.),y:Vec3::new(0.,1.,0.),z:Vec3::new(0.,0.,1.)};}
enum Surface{
 Nurbs{u_knots:KnotVector,v_knots:KnotVector,controls:Vec<Vec<Pnt3>>,weights:Vec<Vec<f64>>},Plane,
 Cylinder{frame:Frame,radius:f64},Cone{frame:Frame,half_angle:f64},Sphere{frame:Frame,radius:f64},
 Torus{frame:Frame,major_radius:f64,minor_radius:f64},
}
impl Surface{
 fn point_original(&self,u:f64,v:f64)->Option<Pnt3>{
  let Self::Cylinder{radius,..}=self else{return None;};Some(Pnt3::new(radius*u.cos(),radius*u.sin(),v))
 }
 fn normal_original(&self,u:f64,_:f64)->Option<Vec3>{Some(Vec3::new(u.cos(),u.sin(),0.))}
""")
surface = base / "📸️representation/🏄️surface"
code = (surface / "🦀️.rs").read_text()
print(code[code.index("    pub fn original_spline_patch_sample("):code.index("    /// 🗺️ Unit surface normal")])
print("}")
print(code[code.index("fn nurbs_patch_original("):code.index("fn eval_nurbs_point(")])
print("mod bounds{")
print((surface / "🧮️rational-preview/🦀️.rs").read_text().split("\n",2)[2])
print("}")
print("mod mesh_cursor{")
print((base / "💡️queries/🧩tessellation/🏄️surface-preview/🦀️.rs").read_text().split("\n",2)[2])
print(r"""
}
fn run(surface:&Surface,tolerance:f64)->MeshData {
 let mut job=mesh_cursor::SurfacePatchPreview::new(surface,100,tolerance).unwrap();
 for _ in 0..200010 {
   let demand=job.demand();let (mesh,receipt)=job.advance(surface,100,demand).unwrap();
   assert!(receipt.fits(demand));assert_eq!(receipt.copied_items,1);
   if let Some(mesh)=mesh{return mesh;}
 }
 panic!("surface preview stalled");
}
#[test]fn chord_and_disconnected_span(){
""")
fixture = json.loads((surface / "🧪️tests/🧫️fixtures/rational-patch.json").read_text())

def vector(items):
    return "vec![" + ",".join(str(float(x)) for x in items) + "]"

def surface_source(uknots, udegree, controls, weights):
    control_rows = ",".join("vec![" + ",".join(f"Pnt3::new({float(x)},{float(y)},{float(z)})" for x,y,z in row) + "]" for row in controls)
    weight_rows = ",".join(vector(row) for row in weights)
    return (
        f"Surface::Nurbs{{u_knots:KnotVector{{degree:{udegree},knots:{vector(uknots)}}},"
        f"v_knots:KnotVector{{degree:{fixture['vDegree']},knots:{vector(fixture['vKnots'])}}},"
        f"controls:vec![{control_rows}],weights:vec![{weight_rows}]}}"
    )

print("let patch="+surface_source(fixture["uKnots"], fixture["uDegree"], fixture["controls"], fixture["weights"])+";")
print(f'let tolerance={fixture["previewDeflection"]};')
print(r"""
let mesh=run(&patch,tolerance);
let n=((mesh.indices.len()/6)as f64).sqrt() as usize;
assert!(n>1);
assert_eq!(mesh.indices.len(),6*n*n);
let read=|i:u32|{let p=&mesh.positions[i as usize*3..];Pnt3::new(p[0].into(),p[1].into(),p[2].into())};
for (index,t) in mesh.indices.chunks_exact(3).enumerate(){
 let [a,b,c]=[read(t[0]),read(t[1]),read(t[2])];
 let (du,dv)=if index%2==0{(2./3.,1./3.)}else{(1./3.,2./3.)};
 let u=((index/2/n)as f64+du)/n as f64;
 let v=((index/2%n)as f64+dv)/n as f64;
 let (p,normal)=patch.original_spline_patch_sample([2,1],u,v).unwrap();
 let center=Pnt3::new((a.x+b.x+c.x)/3.,(a.y+b.y+c.y)/3.,(a.z+b.z+c.z)/3.);
 assert!(center.distance(p)<=tolerance);
 assert!((b-a).cross(c-a).dot(normal.unwrap())>0.);
}
""")
gap = fixture["discontinuous"]
points = [[[x, 0, 0], [x, 1, 0]] for section in gap["sections"] for x in section]
print("let gap="+surface_source(gap["uKnots"], 1, points, [[1, 1]] * len(points))+";")
print("let mesh=run(&gap,tolerance);")
print(f'assert_eq!(mesh.indices.len()/3,{gap["triangles"]});')
limits = ",".join(f"({float(lo)},{float(hi)})" for lo, hi in gap["sections"])
print(f"let limits=[{limits}];")
print(r"""
assert!(mesh.indices.chunks_exact(3).all(|tri| limits.iter().any(|&(lo,hi)|
 tri.iter().all(|&id|(lo..=hi).contains(&(mesh.positions[id as usize*3] as f64)))
)));
}
""")

cylinder = fixture["analyticCylinder"]
print("#[test]fn finite_cylinder(){")
print(f"let cylinder=Surface::Cylinder{{frame:Frame::WORLD,radius:{float(cylinder['radius'])}}};")
print(f"let mesh=run(&cylinder,{cylinder['tolerance']});")
print(r"""
let p=|i:u32|{let p=&mesh.positions[i as usize*3..];Pnt3::new(p[0].into(),p[1].into(),p[2].into())};
let area=mesh.indices.chunks_exact(3).map(|tri|{
 let [a,b,c]=[p(tri[0]),p(tri[1]),p(tri[2])];
 let normal=(b-a).cross(c-a);
 assert!(normal.dot(Vec3::new(a.x,a.y,0.))>0.);
 normal.norm()*0.5
}).sum::<f64>();
""")
print(f"assert!((area-{cylinder['area']}).abs()<{cylinder['tolerance']});}}")
