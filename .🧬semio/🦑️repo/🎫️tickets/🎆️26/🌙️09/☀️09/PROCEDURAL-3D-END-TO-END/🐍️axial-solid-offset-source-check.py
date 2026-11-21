"""↔️ Check the exact production offsets, analytic measures, classification and frustum trims."""
import json
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
root=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
fixture=json.loads((flow/"🔣️original-axial-solid-offset.json").read_text())
stub=(ticket/"🐍️cylinder-split-source-check.py").read_text().split('source=r"""',1)[1].split("fn cylinder(origin:",1)[0]
stub=stub.replace("},Other,", "},Sphere{frame:Frame3,radius:f64},Torus{frame:Frame3,major:f64,minor:f64},")
parts=[stub,"""
impl Frame3{fn to_local(self,point:Pnt3)->Pnt3{
 let delta=point-self.origin;
 Pnt3::new(delta.dot(self.x),delta.dot(self.y),delta.dot(self.z))
}}
struct PrimitiveProvenance{primitive:SolidPrimitive}
enum SolidProvenance{Primitive(PrimitiveProvenance),Frustum(ConicalFrustum,u64),Other}
"""]
leaf=next(root.rglob("🏗️solid-transaction/↔️solid-offset/🦀️.rs")).read_text()
recipe=leaf[leaf.index("fn unsupported()"):]
solid=next(root.rglob("🧱️primitives/🧊️analytic-solids/🦀️.rs")).read_text()
mass=solid[solid.index("    pub fn volume(self)"):solid.index("    /// 📦️ Computes the exact world-axis support")]
result=solid[solid.index("    pub fn signed_boundary_distance("):solid.index("    pub fn vertex_count(")]
for name in ("origin","min","max","point","local","offset"):
    for axis,index in zip("xyz",range(3)):
        recipe=recipe.replace(f"{name}.{axis}",f"{name}.0[{index}]")
        result=result.replace(f"{name}.{axis}",f"{name}.0[{index}]")
parts.extend(("impl SolidPrimitive{"+mass+result+"}",recipe))
frustum=next(root.rglob("📐️conical-frustum/🦀️.rs")).read_text().split("/// 🪣️",1)[1]
parts.append("/// 🪣️"+frustum)
distances=next(root.rglob("🧭️revolution-distance/🦀️.rs")).read_text().split("/// 🪣️",1)[1]
parts.append("mod revolved{use super::*;/// 🪣️"+distances+"}")
entry=next(root.rglob("↔️certified-offset/🦀️.rs")).read_text()
assert "self.preview_provenance(handle)" in entry and "source.offset_boundary(distance)?" in entry
parts.append(r"""
fn stock_frame(origin:[f64;3],angle:f64)->Frame3{
 let (s,c)=angle.sin_cos();
 Frame3{origin:Pnt3::from_array(origin),x:Vec3([c,0.,-s]),y:Vec3([0.,1.,0.]),z:Vec3([s,0.,c])}
}
fn stock(kind:u8,frame:Frame3,r:f64,h:f64,cut:f64)->SolidProvenance{
 let shape=match kind{
   0=>SolidPrimitive::Cylinder{frame,radius:r,height:h},
   1=>SolidPrimitive::Cone{frame,radius:r,height:h},
   _=>{
    let small=r*(1.-cut/h);
    let reversed=Frame3{origin:frame.origin+frame.z*cut,y:-frame.y,z:-frame.z,..frame};
    return SolidProvenance::Frustum(ConicalFrustum::new(reversed,[small,r],cut).unwrap(),0);
   }
 };
 SolidProvenance::Primitive(PrimitiveProvenance{primitive:shape})
}
fn check_trim(shape:ConicalFrustum){
 let mut uses=[[0;2];3];
 for (face,&count) in ConicalFrustum::RINGS.iter().enumerate(){
  let surface=shape.surface(face);
  for member in 0..count{
   let trim=shape.trim(face,member);
   let edge=shape.edge(trim.edge);
   uses[trim.edge][usize::from(trim.forward)]+=1;
   for fraction in [0.,0.27,0.9,1.]{
    let t=trim.range.0+fraction*(trim.range.1-trim.range.0);
    let Pnt2(u,v)=trim.curve.eval(t);
    assert!((surface.eval(u,v)-edge.curve.eval(t)).norm()<1e-11);
   }
  }
 }
 assert_eq!(uses,[[1;2];3]);
}
fn check_offset(source:SolidProvenance,delta:f64,reference:Frame3,levels:[f64;2],radii:[f64;2],measures:[f64;2],center:[f64;3],probes:&[([f64;3],u8)]){
 let actual=source.offset_boundary(delta).unwrap();
 let (bottom,top,found,area,volume, centroid)=match actual{
  SolidBoundary::Analytic(solid)=>{
   let (frame,radius,height,end_radius)=match solid{
    SolidPrimitive::Cylinder{frame,radius,height}=>(frame,radius,height,radius),
    SolidPrimitive::Cone{frame,radius,height}=>(frame,radius,height,0.),
    _=>unreachable!()
   };
   assert!((frame.z-reference.z).norm()<1e-12);
   (frame.origin,frame.origin+frame.z*height,[radius,end_radius],solid.area(),solid.volume(),solid.center_of_mass())
  }
  SolidBoundary::Frustum(shape)=>{
   check_trim(shape);
   assert!((shape.frame.z+reference.z).norm()<1e-12);
   (shape.rim(1).origin,shape.frame.origin,[shape.radii[1],shape.radii[0]],shape.area(),shape.volume,shape.center_of_mass())
  }
 };
 for (point,level) in [bottom,top].into_iter().zip(levels){
  assert!((point-(reference.origin+reference.z*level)).norm()<1e-11);
 }
 assert!(found.into_iter().zip(radii).all(|(a,b)|(a-b).abs()<1e-11));
 assert!((area-measures[0]).abs()<1e-10);
 assert!((volume-measures[1]).abs()<1e-10);
 assert!((centroid-Pnt3::from_array(center)).norm()<1e-10);
 for &(coords,expected) in probes{
  let point=reference.to_world(Pnt3::from_array(coords));
  let distance=match actual{
   SolidBoundary::Analytic(solid)=>solid.signed_boundary_distance(point),
   SolidBoundary::Frustum(shape)=>revolved::frustum_signed_distance(shape.frame,shape.radii,shape.height,point)
  };
  assert_eq!(if distance.abs()<1e-10{2}else if distance<0.{0}else{1},expected);
 }
}
#[test]fn nonfinite_offsets_leave_the_source_unchanged(){
 assert!(SolidProvenance::Other.offset_boundary(0.2).is_err());
 let frame=stock_frame([2.,-1.,4.],0.7);
 for kind in 0..3{
  for distance in [f64::INFINITY,f64::NEG_INFINITY,f64::NAN,f64::MAX]{
   assert!(stock(kind,frame,3.,5.,3.).offset_boundary(distance).is_err());
  }
 }
}
""")
def value(x):
    if isinstance(x,list):
        return "["+",".join(map(value,x))+"]"
    return repr(float(x))
def fields(row):
    spec=fixture["stock"]
    frame=f'stock_frame({value(spec["origin"])},{value(spec["rotateY"])})'
    kind=["cylinder","cone","frustum"].index(row["kind"])
    stock=f'stock({kind},{frame},{value(spec["radius"])},{value(spec["height"])},{value(row.get("cut",spec["cut"]))})'
    return frame,stock,value(row["distance"])
for i,row in enumerate(fixture["cases"]):
    frame,stock,amount=fields(row)
    probes="&["+",".join(f"({value(point)},{kind})" for point,kind in row["classify"])+"]"
    args=",".join((stock,amount,frame,value(row["levels"]),value(row["radii"]),value([row["area"],row["volume"]]),value(row["centroid"]),probes))
    parts.append(f'#[test]fn offset_{i}(){{check_offset({args});}}')
for i,row in enumerate(fixture["refused"]):
    _,stock,amount=fields(row)
    parts.append(f'#[test]fn refusal_{i}(){{assert!({stock}.offset_boundary({amount}).is_err());}}')
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    source=Path(folder)/"axial-offset.rs"
    exe=source.with_suffix(".exe" if os.name=="nt" else "")
    source.write_text("\n".join(parts))
    for args in (("rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(exe)),(str(exe),)):
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",*args],check=True)
