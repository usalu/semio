"""Compile native orthogonal projection and registered sphere admission outside the application graph."""
import json
import math
import os
from pathlib import Path
from tempfile import TemporaryDirectory
import re
import subprocess

ticket = Path(__file__).parent
flow = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
kernel = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
data = json.loads((flow/"🔣️original-solid-queries.json").read_text())
shapes = json.loads((flow/"🔣️original-box-booleans.json").read_text())

def array(values):
    return "["+",".join(repr(float(x)) for x in values)+"]"

def bounds(box):
    low = box["origin"]
    high = [a+b for a,b in zip(low,box["size"])]
    return "["+array(low)+","+array(high)+"]"

def function(text, signature):
    start = text.index(signature)
    body = text.index("{", start)
    depth = 1
    for end in range(body + 1,len(text)):
        depth += (text[end] == "{") - (text[end] == "}")
        if depth == 0:
            return text[start:end+1]
    raise ValueError(signature)

registry = next(kernel.rglob("📍️certified-solid-queries/🦀️.rs")).read_text()
routing = next(kernel.rglob("📍️solid-distance/🦀️.rs")).read_text()
projection = (kernel/"⚙️engine/🔍️intersection-evaluation-and-measure/🦀️.rs").read_text()
project_source = next(line.strip() for line in projection.splitlines() if "self.native.original_orthogonal_nearest" in line)
distance_source = next(kernel.rglob("📦️box-distance/🦀️.rs")).read_text()
material_admission = next(line.strip() for line in registry.splitlines() if "=> return Ok(Material(source))" in line)
material_distance = re.search(
    r"\(Sphere\(center, radius\), Material\(shape\)\).*?sphere_boundary_gap\(shape\.certified_signed_distance\(center\)\?, radius\),",
    registry, re.S,
).group()
routes = [line.strip() for line in routing.splitlines() if "Self::Orthogonal(source)" in line or "Self::PlacedOrthogonal(source,frame)" in line]
orthogonal = next(kernel.rglob("🧱️orthogonal-boundary/🦀️.rs"))
boundary = orthogonal.read_text()
cells = (orthogonal.parent/"🧊️material-cells/🦀️.rs").read_text().replace(
    "semio_framework_value::artifact_retire_leaf!(OrthogonalCells);", "",
)
source = r"""
extern crate self as semio_framework_value;
#[derive(Debug)] struct ValueError(&'static str);
enum ValueRefusalKind { InvalidValue, UnsupportedOwner, OwnershipLimit }
impl ValueError { fn literal(_:ValueRefusalKind,message:&'static str)->Self { Self(message) } }
fn unsupported()->ValueError { ValueError("invalid material") }
fn empty_result()->ValueError { ValueError("empty material") }
#[derive(Clone,Copy)] enum BooleanOp { Unite,Cut,Intersect }
type Axes=[[f64;4];3];
type Bounds=[[f64;3];2];
const CELL_COUNT:usize=27;
struct OrthogonalBoundary(OrthogonalCells);
impl OrthogonalBoundary {
 fn from_cells(axes:Axes,mut keep:impl FnMut(usize,[usize;3])->bool)->Result<Self,ValueError>{
  let mut occupied=0;
  for slot in 0..CELL_COUNT {
   let cell=decode(slot,3);
   if (0..3).all(|axis| cell[axis]+1<axis_count(&axes[axis])) && keep(slot,cell) {
    occupied|=1<<slot;
   }
  }
  Ok(Self(OrthogonalCells::new(axes,occupied)))
 }
}
"""
for signature in ["fn address(","fn decode(","fn adjacent_cell(","fn ordered(","fn axis_count("]:
    source += "\n" + function(boundary,signature)
source += "\nmod material_cells {\n"+cells+"\n}\nuse material_cells::OrthogonalCells;\n"
source += "\n"+function(distance_source,"fn certified_gap(")+"\n"+function(distance_source,"pub fn sphere_boundary_gap(")+"\n"
source += f"""
#[derive(Clone,Copy)] struct Pnt3([f64;3]);
impl Pnt3 {{ fn to_array(self)->[f64;3] {{ self.0 }} fn from_array(value:[f64;3])->Self {{ Self(value) }} }}
fn pnt(value:[f64;3])->Pnt3 {{ Pnt3(value) }}
#[derive(Clone,Copy)] struct Frame3 {{ origin:Pnt3,x:Pnt3,y:Pnt3,z:Pnt3 }}
impl Frame3 {{
 const WORLD:Self=Self {{origin:Pnt3([0.0;3]),x:Pnt3([1.0,0.0,0.0]),y:Pnt3([0.0,1.0,0.0]),z:Pnt3([0.0,0.0,1.0])}};
 fn to_world(self, p:Pnt3)->Pnt3 {{
  pnt(std::array::from_fn(|i| self.origin.0[i]+self.x.0[i]*p.0[0]+self.y.0[i]*p.0[1]+self.z.0[i]*p.0[2]))
 }}
 fn to_local(self,p:Pnt3)->Pnt3{{
  let v:[f64;3]=std::array::from_fn(|i| p.0[i]-self.origin.0[i]);
  pnt([self.x,self.y,self.z].map(|axis| v.into_iter().zip(axis.0).map(|(a,b)| a*b).sum()))
 }}
}}
#[derive(Clone,Copy)] enum SolidProvenance {{ Orthogonal(OrthogonalCells), PlacedOrthogonal(OrthogonalCells,Frame3), SphericalShell((),()) }}
impl SolidProvenance {{
 fn certified_signed_distance(self,point:Pnt3)->Result<f64,ValueError>{{
  Ok(match self {{ {" ".join(routes)} _=>unreachable!() }})
 }}
}}
#[derive(Clone,Copy)] enum CertifiedDistanceSolid<'a>{{ Sphere(Pnt3,f64), Material(&'a SolidProvenance) }}
fn admit_material(source:&SolidProvenance)->Result<CertifiedDistanceSolid<'_>,ValueError>{{
 use CertifiedDistanceSolid::Material;
 match source {{
  {material_admission}
  _ => Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"material proof refused")),
 }}
}}
fn exact_distance(a:CertifiedDistanceSolid,b:CertifiedDistanceSolid)->Result<f64,ValueError>{{
 use CertifiedDistanceSolid::{{Material,Sphere}};
 match (a,b) {{
  {material_distance}
  _ => unreachable!(),
 }}
}}
"""
source += "\nstruct BrepRegistry; impl BrepRegistry {\n" + function(registry,"pub fn original_orthogonal_nearest(") + "\n}\n"
source += f"""
enum Entity<'a> {{ Solid(usize,Option<&'a SolidProvenance>) }}
struct Engine {{ native:BrepRegistry }}
impl Engine {{
 fn nearest(&self, proof:&SolidProvenance,target:Pnt3)->Result<Pnt3,ValueError>{{
  Ok(match Entity::Solid(0,Some(proof)) {{
   {project_source}
   _=>unreachable!(),
  }})
 }}
}}
"""
for index,row in enumerate(data["orthogonalSphereDistance"]):
    if row["operator"] == "brep.solid.shell":
        a = row["stock"]
        thickness = a["thickness"]
        b = {"origin":[p+thickness for p in a["origin"]], "size":[d-2*thickness for d in a["size"]]}
    else:
        a,b = (shapes[row["source"]][key] for key in ("a","b"))
    op = "Unite" if row["operator"].endswith("fuse") else "Cut"
    source += f"""#[test] fn sphere_clearance_{index}(){{
let a=OrthogonalCells::from_box({bounds(a)}).unwrap();
let b=OrthogonalCells::from_box({bounds(b)}).unwrap();
let cells=a.combine(b,BooleanOp::{op}).unwrap().0;
"""
    placement = row.get("placement")
    if placement:
        angle = placement["angle"]
        cos,sin = math.cos(angle),math.sin(angle)
        source += f"let frame=Frame3{{origin:pnt({array(placement['offset'])}),x:pnt([{cos},{sin},0.0]),y:pnt([{-sin},{cos},0.0]),z:pnt([0.0,0.0,1.0])}};\n"
        source += "let shape=SolidProvenance::PlacedOrthogonal(cells,frame);\n"
    else:
        source += "let shape=SolidProvenance::Orthogonal(cells);\n"
    source += "let material=admit_material(&shape).unwrap();\n"
    for probe in row["probes"]:
        center = probe["center"]
        if placement:
            x,y,z = center
            d = placement["offset"]
            center = [d[0]+cos*x-sin*y,d[1]+sin*x+cos*y,d[2]+z]
        source += f'let probe=CertifiedDistanceSolid::Sphere(pnt({array(center)}),{float(probe["radius"])});\n'
        source += 'let a=exact_distance(probe,material).unwrap();let b=exact_distance(material,probe).unwrap();\n'
        source += f'assert_eq!(a,b);assert!((a-{float(probe["expected"])}).abs()<1e-12,"actual {{a}}");\n'
    source += "let engine=Engine{native:BrepRegistry};\n"
    def placed(point):
        if not placement:
            return point
        x,y,z = point
        dx,dy,dz = placement["offset"]
        return [dx+cos*x-sin*y,dy+sin*x+cos*y,dz+z]
    for target,expected in row["closestPoints"]:
        source += f'let actual=engine.nearest(&shape,pnt({array(placed(target))})).unwrap().to_array();\n'
        source += f'assert!(actual.into_iter().zip({array(placed(expected))}).all(|(a,b)| (a-b).abs()<1e-12),"actual {{actual:?}}");\n'
    source += "assert!(engine.nearest(&shape,pnt([f64::MAX;3])).is_err());\n}\n"

generated = ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    exe = Path(folder)/("sphere-material.exe" if os.name=="nt" else "sphere-material")
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc",
                    "--edition=2024","--test","-Awarnings","-","-o",str(exe)],input=source,text=True,check=True)
    subprocess.run([str(exe)],check=True)
