"""Check the exact registered annular bounds route against authored neutral geometry."""
from pathlib import Path
from tempfile import TemporaryDirectory
import json
import math
import os
import subprocess

ticket = Path(__file__).parent
kernel = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
flow = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
fixture = json.loads((flow / "🔣️original-bounding-box.json").read_text())
registry = (kernel / "⚙️engine/🧠️native-registry/🔳️analytic-bounds/🦀️.rs").read_text()
bounds = (kernel / "💡️queries/🔎️analysis/📦️bounds/🦀️.rs").read_text()
annular = (kernel / "📸️representation/🔖️contract/🧿️annular-revolution/🦀️.rs").read_text()

def function(text, signature):
    start = text.index(signature)
    opening = text.index("{", start)
    depth = 1
    for end in range(opening + 1,len(text)):
        depth += (text[end] == "{") - (text[end] == "}")
        if depth == 0:
            return text[start:end+1]
    raise ValueError(signature)

def array(values):
    return "["+",".join(repr(float(x)) for x in values)+"]"

def point(values):
    return f"point({array(values)})"

source = r"""
use std::f64::consts::{TAU, PI};
use std::ops::{Add, Sub, Mul, Neg};
#[derive(Clone,Copy,Debug)]
struct Vec3 { x:f64, y:f64, z:f64 }
type Pnt3=Vec3;
impl Vec3 {
 const X:Self=Self::new(1.,0.,0.);
 const Y:Self=Self::new(0.,1.,0.);
 const Z:Self=Self::new(0.,0.,1.);
 const fn new(x:f64,y:f64,z:f64)->Self { Self { x,y,z } }
 fn to_array(self)->[f64;3] { [self.x,self.y,self.z] }
 fn dot(self,rhs:Self)->f64 { self.x*rhs.x+self.y*rhs.y+self.z*rhs.z }
}
impl Add for Vec3 { type Output=Self; fn add(self,b:Self)->Self { Self::new(self.x+b.x,self.y+b.y,self.z+b.z) } }
impl Sub for Vec3 { type Output=Self; fn sub(self,b:Self)->Self { self+(-b) } }
impl Mul<f64> for Vec3 { type Output=Self; fn mul(self,b:f64)->Self { Self::new(self.x*b,self.y*b,self.z*b) } }
impl Neg for Vec3 { type Output=Self; fn neg(self)->Self { self*(-1.) } }
fn point([x,y,z]:[f64;3])->Pnt3 { Pnt3::new(x,y,z) }
#[derive(Clone,Copy,Debug)]
struct Frame3 { origin:Pnt3,x:Vec3,y:Vec3,z:Vec3 }
impl Frame3 {
 const WORLD:Self=Self { origin:Pnt3::new(0.,0.,0.),x:Vec3::X,y:Vec3::Y,z:Vec3::Z };
 fn to_world(self,p:Pnt3)->Pnt3 { self.origin+self.x*p.x+self.y*p.y+self.z*p.z }
}
enum Curve3 {
 Line { origin:Pnt3,dir:Vec3 },
 Circle { frame:Frame3,radius:f64 },
 Ellipse { frame:Frame3,major_radius:f64,minor_radius:f64 },
 Nurbs { unused:() },
}
impl Curve3 {
 fn eval(&self,t:f64)->Pnt3 {
  match *self {
   Self::Line {origin,dir}=>origin+dir*t,
   Self::Circle {frame,radius}=>frame.to_world(Pnt3::new(radius*t.cos(),radius*t.sin(),0.)),
   Self::Ellipse {frame,major_radius,minor_radius}=>frame.to_world(Pnt3::new(major_radius*t.cos(),minor_radius*t.sin(),0.)),
   _=>unreachable!(),
  }
 }
}
"""
start = bounds.index("pub struct ShapeBounds")
end = bounds.index("fn add_curve(",start)
source += bounds[start:end]
start = annular.index("pub struct AnnularRevolution")
end = annular.index("impl AnnularRevolution",start)
source += "\n#[derive(Clone,Copy)]\n"+annular[start:end]
source += "\nimpl AnnularRevolution {\n"
for method in ["new","new_blind","new_cavity","from_axial_cut","inner_frame","new_step",
               "new_two_steps","from_axial_union","has_two_steps","has_step","full","vertex_count",
               "edge_count","edge_is_arc","volume","radial","radius","level","center","point","vertex","edge"]:
    source += "\n"+function(annular,"pub fn "+method+"(")
source += "\n}\n"

admission = next(line.strip() for line in registry.splitlines() if "SolidProvenance::Annular(" in line)
if "annular_axis_bounds" in admission:
    source += "\n"+function(bounds,"pub fn annular_axis_bounds(")
else:
    admission = "SolidProvenance::Annular(..) => None,"
source += f"""
enum SolidProvenance {{ Annular(AnnularRevolution, ()) }}
fn original_axis_bounds(provenance: &SolidProvenance)->Option<[Pnt3;2]>{{
 match provenance {{ {admission} }}
}}
"""

for index,row in enumerate(fixture["annular"]):
    source += f"#[test] fn case_{index}(){{\n"
    if row["operator"]=="brep.sweep.revolve":
        origin = list(row["origin"])
        negative = row["angle"] < 0
        if negative:
            origin[2] += row["height"]
        z = "-Vec3::Z" if negative else "Vec3::Z"
        y = "-Vec3::Y" if negative else "Vec3::Y"
        source += f"let frame=Frame3{{origin:{point(origin)},x:Vec3::X,y:{y},z:{z}}};\n"
        source += f"let shape=AnnularRevolution::new(frame,{float(row['inner'])},{float(row['outer'])},{float(row['height'])},{abs(row['angle'])},1e-7).unwrap();\n"
    else:
        a,b = row["a"],row["b"]
        angle = row.get("rotation",0)
        sin,cos = math.sin(angle),math.cos(angle)
        origin = row.get("offset",a["origin"])
        source += f"let frame=Frame3{{origin:{point(origin)},x:{point([cos,0,-sin])},y:Vec3::Y,z:{point([sin,0,cos])}}};\n"
        lo = b["origin"][2]-a["origin"][2]
        span = array([lo,lo+b["height"]])
        operation = "from_axial_union" if row["operator"]=="brep.bool.fuse" else "from_axial_cut"
        source += f"let shape=AnnularRevolution::{operation}(frame,{float(b['radius'])},{float(a['radius'])},{float(a['height'])},{span},1e-7).unwrap();\n"
    low,high=row["bounds"]
    source += f"""
let actual=original_axis_bounds(&SolidProvenance::Annular(shape,())).expect("live annular bounds");
for (p,expected) in actual.into_iter().zip([{array(low)},{array(high)}]) {{
 for (a,b) in p.to_array().into_iter().zip(expected) {{ assert!((a-b).abs()<1e-10,"{{a}} != {{b}}"); }}
}}
let mut invalid=shape;
invalid.frame.origin.x=f64::INFINITY;
assert!(original_axis_bounds(&SolidProvenance::Annular(invalid,())).is_none());
}}
"""

generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    exe = Path(folder) / ("annular-bounds.exe" if os.name == "nt" else "annular-bounds")
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc",
                    "--edition=2024","--test","-Awarnings","-","-o",str(exe)],input=source,text=True,check=True)
    subprocess.run([str(exe)],check=True)
