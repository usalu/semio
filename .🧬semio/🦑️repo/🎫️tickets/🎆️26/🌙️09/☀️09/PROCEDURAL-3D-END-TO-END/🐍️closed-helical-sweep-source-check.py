"""Check the actual retained torus recipe on the neutral helical-sweep witness."""
import json
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

brep=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
vector=(brep/"📸️representation/➡️vector/🦀️.rs").read_text()
vector=vector[vector.index("// #region 🔖️Scalars"):vector.index("// #region 🔖️Tests")]
vector="\n".join(line.replace(", value_derive::ToValue, value_derive::FromValue","") for line in vector.splitlines() if not line.startswith(("#[value(","semio_framework_value::")))
owner=(brep/"⚙️engine/🧠️native-registry/🔄️circular-revolution/🦀️.rs").read_text()
recipe=owner[owner.index("fn closed_helical_torus("):]
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json"))
row=json.loads(fixture.read_text())["retainedClosedHelicalSweep"]

def array(xs):
    return "["+",".join(array(x) if isinstance(x,list) else str(float(x)) for x in xs)+"]"

preamble=r"""
type EVec3=[f64;3];
fn vec3(a:EVec3)->Vec3{Vec3::from_array(a)}
fn pnt(a:EVec3)->Pnt3{Pnt3::from_array(a)}
struct Frame3{origin:Pnt3,x:Vec3,y:Vec3,z:Vec3}
enum SolidPrimitive{Torus{frame:Frame3,major:f64,minor:f64}}
#[derive(Debug)]struct ValueError;
enum ValueRefusalKind{InvalidValue}
impl ValueError{fn literal(_:ValueRefusalKind,_:&str)->Self{Self}}
fn unsupported(_:&str)->ValueError{ValueError}
struct Tol;
impl Tol{const DEFAULT:Self=Self;fn value(self)->f64{1e-7}}
"""
cases=[ [row["radius"],row["pitch"],turn] for turn in row["turns"] ]
refused=[[r["radius"],r["pitch"],r["turns"]] for r in row["refused"]]
values=f"""
const ORIGIN:EVec3={array(row['axisOrigin'])};
const DIRECTION:EVec3={array(row['axisDirection'])};
const SECTION:f64={row['profile']['radius']};
const VOLUME:f64={row['volume']};
const AREA:f64={row['area']};
const CASES:[[f64;3];{len(cases)}]={array(cases)};
const REFUSED:[[f64;3];{len(refused)}]={array(refused)};
"""
tests=r"""
#[test]fn exact_frame_and_recipe_match_neutral_fixture(){
 for parameters in CASES{
  let SolidPrimitive::Torus{frame,major,minor}=closed_helical_torus(ORIGIN,DIRECTION,parameters,SECTION).unwrap();
  assert_eq!(frame.origin.to_array(),ORIGIN);
  assert!((frame.z-vec3(DIRECTION).normalized().unwrap()).norm()<1e-14);
  assert!((frame.x.cross(frame.y)-frame.z).norm()<1e-14);
  let pi=std::f64::consts::PI;
  assert!((2.*pi.powi(2)*major*minor.powi(2)-VOLUME).abs()<1e-12);
  assert!((4.*pi.powi(2)*major*minor-AREA).abs()<1e-12);
  for u in [0_f64,0.5,2.5,4.]{
   for v in [0_f64,0.9,2.4,6.] {
    let radial=frame.x*u.cos()+frame.y*u.sin();
    let point=frame.origin+radial*(major+minor*v.cos())+frame.z*(minor*v.sin());
    let center=frame.origin+radial*major;
    assert!(((point-center).norm()-SECTION).abs()<1e-13);
   }
  }
 }
}
#[test]fn refuses_a_nonrepresentable_or_invalid_request(){
 for parameters in REFUSED{assert!(closed_helical_torus(ORIGIN,DIRECTION,parameters,SECTION).is_err());}
 assert!(closed_helical_torus(ORIGIN,[0.;3],CASES[0],SECTION).is_err());
 assert!(closed_helical_torus([f64::NAN,0.,0.],DIRECTION,CASES[0],SECTION).is_err());
}
"""
with TemporaryDirectory(dir=Path(__file__).parent/"🗑️generated") as folder:
    source=Path(folder)/"closed_helical.rs"
    source.write_text("\n".join([vector,preamble,recipe,values,tests]))
    out=source.with_suffix("")
    subprocess.run(["rustup","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(out)],check=True)
    subprocess.run([str(out)],check=True)
