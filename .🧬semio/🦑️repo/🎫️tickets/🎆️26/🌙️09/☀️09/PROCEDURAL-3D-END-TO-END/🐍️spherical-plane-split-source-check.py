"""✂️ Exercise the registered sphere Split recipe with the original cap representations."""
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).resolve().parent
brep = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
fixture = json.loads((brep / "💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/spherical-plane-split.json").read_text(encoding="utf-8"))
parts = runpy.run_path(str(ticket / "🐍️coaxial-bore-source-check.py"))["parts"]
parts = [part.replace("enum SolidProvenance{Primitive(PrimitiveProvenance),","enum SolidProvenance{SphericalCaps(SphericalCaps,u64),Primitive(PrimitiveProvenance),") for part in parts]
registry = (brep / "⚙️engine/🧠️native-registry/🧊️solid-publication/🦀️.rs").read_text(encoding="utf-8")
plan = registry.split("let [positive,negative]=match solid {",1)[1].split("_=>solid.split_by_plane(origin,normal)?",1)[0]
plan = plan.replace("ValueError::literal(ValueRefusalKind::InvalidValue,", "ValueError(")
helper = (brep / "⚙️engine/🏗️solid-topology/🪓️spherical-split/🦀️.rs").read_text(encoding="utf-8")
helper = helper[helper.index("pub fn spherical_axial_span"):].replace("ValueError::literal(ValueRefusalKind::InvalidValue,", "ValueError(").replace("ValueError::literal(ValueRefusalKind::UnsupportedOwner,", "ValueError(")
parts.append(helper)
parts.append("""
fn split_bank(solid:SolidProvenance, origin:EVec3, normal:EVec3)->Result<[SolidBoundary;2],ValueError> {
 Ok(match solid {""" + plan + """
 _=>unreachable!("fixture uses the native sphere arm"),
 })
}
fn check_sphere_split(radius:f64,origin:EVec3,normal:EVec3,volumes:[f64;2],span:Option<[f64;2]>){
 let frame=Frame3::from_normal(pnt([0.;3]),Vec3::Z).unwrap();
 let solid=if let Some([low,high])=span{
  let shape=if high==radius{SphericalCaps::from_plane(frame,radius,low,1e-6)}
   else{SphericalCaps::from_band(frame,radius,[low,high],1e-6)}.unwrap();
  SolidProvenance::SphericalCaps(shape,0)
 }else{SolidProvenance::Primitive(PrimitiveProvenance{primitive:SolidPrimitive::Sphere{frame,radius},nurbs_cuboid:false})};
 for sign in [1.,-1.]{
  let normal=normal.map(|value|value*sign);
  let [a,b]=split_bank(solid,origin,normal).unwrap();
  for (i,shape) in [a,b].into_iter().enumerate(){
   let SolidBoundary::SphericalCaps(shape)=shape else{panic!("sphere half");};
   assert!((shape.volume()-volumes[i^usize::from(sign<0.)]).abs()<1e-9);
   let axis=pnt(normal)*if i==0{1.}else{-1.};
   assert!((1..=if shape.is_band(){2}else{1}).map(|face|shape.frame(face)).any(|rim|
    (rim.origin-pnt(origin)).dot(axis).abs()<1e-12&&(rim.z+axis).norm()<1e-12));
   assert!((shape.center_of_mass()-pnt(origin)).dot(axis)>0.);
   assert_eq!(shape.edge_count(),3);
   assert!(shape.is_disk(1));
  }
 }
 assert!(split_bank(solid,[0.,0.,radius],[0.,0.,1.]).is_err());
 assert!(split_bank(solid,[0.,0.,0.],[0.;3]).is_err());
}
""")
def rust(values):
    return "["+",".join(map(rust,values))+"]" if isinstance(values,list) else repr(float(values))
for i,row in enumerate(fixture["cases"]):
    arguments = ",".join(rust(n) for n in (fixture["radius"],row["origin"],row["normal"],row["volumes"]))
    span = "Some(" + rust(row["span"]) + ")" if "span" in row else "None"
    parts.append(f"#[test]fn sphere_split_{i}(){{check_sphere_split({arguments},{span});}}")
with TemporaryDirectory(dir=ticket / "🗑️generated") as temp:
    source=Path(temp)/"sphere-split.rs"
    executable=source.with_suffix(".exe" if os.name=="nt" else "")
    source.write_text("\n".join(parts),encoding="utf-8")
    command=["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20"]
    subprocess.run(command+["rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(executable)],check=True)
    subprocess.run(command+[str(executable),"sphere_split_","--nocapture"],check=True)
