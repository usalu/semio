"""🧭️ Compile retained signed support distances for the complete revolution corpus."""
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
kernel=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
fixture=json.loads((flow/"🔣️original-revolved-classification.json").read_text())
parts=runpy.run_path(str(ticket/"🐍️coaxial-bore-source-check.py"))["parts"]
torus=next(kernel.rglob("🍩️torus-sector/🦀️.rs")).read_text().split("/// 🌀️",1)[1]
parts.append("mod sector{use super::*;use std::f64::consts::{PI,TAU};/// 🌀️"+torus+"} use sector::TorusSector;")
registry=next(kernel.rglob("📍️certified-solid-queries/🦀️.rs")).read_text()
routing=next(kernel.rglob("📍️solid-distance/🦀️.rs")).read_text()
assert "source.certified_signed_distance(" in registry
arms=[next(line.strip() for line in routing.splitlines() if f"Self::{name}(source,_)=>" in line)
      for name in ("Frustum","TorusSector","Annular")]
tolerance=registry.split("const CONTACT_TOLERANCE: f64 = ",1)[1].split(";",1)[0]
parts.append("""
use revolution_distance::frustum_signed_distance;
struct Frustum{frame:Frame3,radii:[f64;2],height:f64}
enum Revolved{Frustum(Frustum,u64),TorusSector(TorusSector,u64),Annular(AnnularRevolution,u64)}
impl Revolved{
 fn distance(self,point:Pnt3)->f64{
  match self{"""+ "".join(arms)+"""}
 }
}
""")
parts.append(r"""
fn frame(kind:usize)->Frame3{
 if kind==0{return Frame3::WORLD;}
 Frame3{origin:pnt([2.,-5.,3.]),x:pnt([0.6,0.,-0.8]),y:pnt([0.,1.,0.]),z:pnt([0.8,0.,0.6])}
}
fn signed(shape:Revolved,frame:Frame3,local:EVec3,expected:f64){
 let [x,y,z]=local;
 let world=frame.origin+frame.x*x+frame.y*y+frame.z*z;
 let value=shape.distance(world);
 assert!((value-expected).abs()<1e-10,"{local:?}: {value} != {expected}");
 let classify=|distance:f64|if distance.abs()<=CONTACT{2}else if distance<0.{0}else{1};
 assert_eq!(classify(value),classify(expected));
}
#[test]fn original_revolutions_classify_exact_walls_caps_and_cavities(){
 for world in 0..2{
  let frame=frame(world);
""")
def number(n):
    return repr(float(n))
def array(items):
    return "["+",".join(number(n) for n in items)+"]"
for row in fixture["cases"]:
    kind=row["kind"]
    a,b=map(number,row["radii"])
    if kind=="frustum":
        shape=f'Revolved::Frustum(Frustum{{frame,radii:[{a},{b}],height:{number(row["height"])} }},1)'
    elif kind=="torus":
        sweep=f'{row["rotationDegrees"]}_f64.to_radians()'
        shape=f'Revolved::TorusSector(TorusSector::new(frame,{a},{b},{sweep},1e-7).unwrap(),1)'
    elif kind=="annular-sector":
        sweep=f'{row["rotationDegrees"]}_f64.to_radians()'
        shape=f'Revolved::Annular(AnnularRevolution::new(frame,{a},{b},{number(row["height"])},{sweep},1e-7).unwrap(),1)'
    else:
        lo,hi=map(number,row["span"])
        height=number(row["height"])
        if kind=="bore":
            maker=f'from_axial_cut(frame,{a},{b},{height},[{lo},{hi}],1e-7)'
        elif row["span"][0]==0:
            maker=f'new_step(frame,{a},{b},{height},{hi},1e-7)'
        else:
            maker=f'new_two_steps(frame,{a},{b},{height},[{lo},{hi}],1e-7)'
        shape=f'Revolved::Annular(AnnularRevolution::{maker}.unwrap().with_offset(frame.x*{number(row["offset"])},1e-7).unwrap(),1)'
    for point,expected in row["probes"]:
        parts.append(f'signed({shape},frame,{array(point)},{number(expected)});')
parts.append("}} const CONTACT:f64="+tolerance+";")
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    source=Path(folder)/"revolved-classification.rs"
    exe=source.with_suffix(".exe" if os.name=="nt" else "")
    source.write_text("\n".join(parts))
    for args in (("rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(exe)),(str(exe),)):
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",*args],check=True)
