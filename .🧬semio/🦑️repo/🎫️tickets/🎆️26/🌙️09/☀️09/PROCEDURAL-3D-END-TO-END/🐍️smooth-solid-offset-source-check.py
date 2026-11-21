"""Check the actual signed analytic offset recipe against the authored neutral witnesses."""
import json
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

brep=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
vector=(brep/"📸️representation/➡️vector/🦀️.rs").read_text()
vector=vector[vector.index("// #region 🔖️Scalars"):vector.index("// #region 🔖️Tests")]
vector="\n".join(l.replace(", value_derive::ToValue, value_derive::FromValue","") for l in vector.splitlines() if not l.startswith(("#[value(","semio_framework_value::")))
matrix=(brep/"📸️representation/➡️vector/🔢️matrix/🦀️.rs").read_text()
frame=matrix[matrix.rfind("#[derive",0,matrix.index("pub struct Frame3")):matrix.index("    /// 🧭️ Maps every axis")]+"}\n"
frame="\n".join(l.replace(", value_derive::ToValue, value_derive::FromValue","") for l in frame.splitlines() if not l.startswith(("#[value(","semio_framework_value::")))
solids=(brep/"🛠️operations/🧱️primitives/🧊️analytic-solids/🦀️.rs").read_text()
primitive=solids[solids.index("pub enum SolidPrimitive"):solids.index("/// 📏️ Complete analytic definition")]
volume=solids[solids.index("    pub fn volume(self)"):solids.index("    /// 📐️ Gives the exact total surface area")]
recipe=(brep/"⚙️engine/🧠️native-registry/🏗️solid-transaction/↔️solid-offset/🦀️.rs").read_text()
recipe=recipe[recipe.index("fn unsupported()"):recipe.index("fn offset_frustum(")]

fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🛠️brep-feature/🔣️.json"))
rows=json.loads(fixture.read_text())["retainedSmoothSolidOffset"]

def array(xs):
    return "["+",".join(str(float(x)) for x in xs)+"]"

preamble=r"""
#[derive(Debug)] struct ValueError;
enum ValueRefusalKind{InvalidValue,UnsupportedOwner}
impl ValueError{fn literal(_:ValueRefusalKind,_:&str)->Self{Self}}
struct Tol;
impl Tol{const DEFAULT:Self=Self;fn value(self)->f64{1e-7}}
"""
tests=r"""
#[test] fn exact_offset_preserves_support_frame_volume_and_type(){
 assert!(std::mem::size_of::<SolidPrimitive>()+std::mem::size_of::<f64>()<=128);
"""
for row in rows:
    kind=row["kind"]
    tests+=f"let frame=Frame3::from_normal(Pnt3::from_array({array(row['origin'])}),Vec3::from_array({array(row['axis'])})).unwrap();\n"
    fields=f"radius:{row['minor']}" if kind=="sphere" else f"major:{row['major']},minor:{row['minor']}"
    tests+=f"let source=SolidPrimitive::{kind.capitalize()}{{frame,{fields}}};\n"
    for case in row["cases"]:
        tests+=f"let result=offset_certified_primitive(source,{case['distance']}).unwrap();\n"
        tests+=f"assert!((result.volume()-{case['volume']}).abs()<1e-11);\n"
        if kind=="sphere":
            tests+=f"let SolidPrimitive::Sphere{{frame:offset,radius}}=result else{{panic!(\"sphere\")}}; assert_eq!(offset,frame);assert_eq!(radius,{case['radius']});\n"
        else:
            tests+=f"let SolidPrimitive::Torus{{frame:offset,major,minor}}=result else{{panic!(\"torus\")}}; assert_eq!(offset,frame);assert_eq!((major,minor),({row['major']},{case['radius']}));\n"
    tests+=f"for distance in {array(row['refused'])}{{assert!(offset_certified_primitive(source,distance).is_err());}}\n"
tests+="}"
generated=Path(__file__).parent/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    source=Path(folder)/"smooth_offset.rs"
    source.write_text("\n".join([vector,frame,preamble,"#[derive(Clone,Copy)]"+primitive,"impl SolidPrimitive{"+volume+"}",recipe,tests]))
    output=source.with_suffix(".exe" if os.name=="nt" else "")
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(output)],check=True)
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",str(output)],check=True)
