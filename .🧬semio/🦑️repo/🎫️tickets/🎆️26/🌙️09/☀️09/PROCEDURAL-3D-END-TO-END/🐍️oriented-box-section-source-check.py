"""✂️ Verify the production retained box section cursor against the neutral section corpus."""
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
kernel=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
fixture=json.loads((flow/"🔣️original-box-section.json").read_text())
parts=runpy.run_path(str(ticket/"🐍️coaxial-bore-source-check.py"))["parts"]
source=(ticket/"🐍️rotated-box-intersection-source-check.py").read_text()
parts.append(source[source.index("fn rotation_z("):source.index("fn check_boundary(")])
registry=next(kernel.rglob("🪷️conic-face-publication/🦀️.rs")).read_text()
assert "source.box_section(origin,normal)?" in registry
dispatch=next(kernel.rglob("🏗️solid-transaction/🦀️.rs")).read_text()
start=dispatch.index("    pub fn box_section(")
end=dispatch.index("    /// ⚖️",start)
parts.append("enum SectionStock{Primitive(PrimitiveProvenance),PlanarBoundary(Option<BoxPrismSource>),ClippedBox(ClippedBox)}")
parts.append("impl SectionStock{"+dispatch[start:end]+"}")
parts.append(r"""
fn section_test_stock(size:EVec3,base:EVec3,angle:f64,clip:Option<(EVec3,EVec3)>) -> SectionStock {
 let shape=match rotated_box(size,angle,base){
  BooleanShape::Analytic(primitive) =>
   return SectionStock::Primitive(PrimitiveProvenance{primitive,nurbs_cuboid:false}),
  BooleanShape::OrientedBox(shape)=>shape,
  _=>unreachable!(),
 };
 if let Some((point,normal))=clip {
  let [positive,_]=shape.split(point,normal).unwrap();
  let SolidBoundary::ClippedBox(_,proof)=positive else {panic!("expected clipped proof");};
  SectionStock::ClippedBox(proof)
 } else {
  let SolidBoundary::BoxPrism(_,proof)=shape.boundary().unwrap() else {panic!("expected oriented box proof");};
  SectionStock::PlanarBoundary(Some(proof))
 }
}
fn original_oriented_box(size:EVec3,angle:f64,base:EVec3)->OrientedBox{
 match rotated_box(size,angle,base){
  BooleanShape::OrientedBox(stock)=>stock,
  BooleanShape::Analytic(shape)=>OrientedBox::from_primitive(shape).unwrap(),
  _=>unreachable!(),
 }
}
fn from_boolean(boundary:SolidBoundary)->SectionStock{
 let proof=match boundary{
  SolidBoundary::BoxPrism(_,proof)|SolidBoundary::PerforatedBox(_,proof)=>proof,
  _=>unreachable!(),
 };
 SectionStock::PlanarBoundary(Some(proof))
}
#[test] fn nonconvex_profiles_do_not_enter_the_convex_cursor(){
 let stock=original_oriented_box([4.,4.,2.],0.,[0.;3]);
 for origin in [[3.,1.,0.],[1.,1.,0.]]{
  let tool=original_oriented_box([2.,2.,2.],0.3,origin);
  let shape=from_boolean(stock.footprint_boolean(tool,RectangleOperation::Difference).unwrap());
  assert!(shape.box_section([0.,0.,1.],[0.,0.,1.]).unwrap().is_none());
 }
}
fn section_geometry(stock:SectionStock,origin:EVec3,normal:EVec3,n:usize,edges:usize,area:f64,bounds:[EVec3;2]){
 let normal=pnt(normal).normalized().unwrap();
 let mut draft=stock.box_section(origin,normal.to_array()).unwrap().unwrap();
 assert!(draft.finish().is_err());
 for _ in 0..edges {
  assert!(draft.has_edge());
  assert_eq!(draft.copied_coordinates(),6);
  draft.advance_edge().unwrap();
 }
 assert!(!draft.has_edge());
 assert_eq!(draft.copied_coordinates(),(2*n).max(12));
 let polygon=draft.finish().unwrap();
 assert_eq!(polygon.count(),n);
 assert!(polygon.point(n).is_none());
 let points=(0..n).map(|i|pnt(polygon.point(i).unwrap())).collect::<Vec<_>>();
 for point in &points{assert!((*point-pnt(origin)).dot(normal).abs()<1e-11);}
 let mut total=0.;
 for i in 1..n-1{
  let signed=(points[i]-points[0]).cross(points[i+1]-points[0]).dot(normal)/2.;
  assert!(signed>0.);
  total+=signed;
 }
 assert!((total-area).abs()<1e-10,"area {total}, expected {area}");
 for axis in 0..3{
  let samples=points.iter().map(|p|p.to_array()[axis]);
  assert!((samples.clone().fold(f64::INFINITY,f64::min)-bounds[0][axis]).abs()<1e-10);
  assert!((samples.fold(f64::NEG_INFINITY,f64::max)-bounds[1][axis]).abs()<1e-10);
 }
}
fn section_contact(stock:SectionStock,origin:EVec3,normal:EVec3){
 let Ok(Some(mut draft))=stock.box_section(origin,normal) else {
  assert_eq!(normal,[0.;3]);
  return;
 };
 while draft.has_edge(){draft.advance_edge().unwrap();}
 assert!(draft.finish().is_err());
}
""")
def value(x):
    if isinstance(x,list):
        return "["+",".join(map(value,x))+"]"
    return repr(float(x))
def call(row):
    clip=row.get("clip")
    clip_expr=f'Some(({value(clip["origin"])},{value(clip["normal"])}))' if clip else 'None'
    return f'section_test_stock({value(fixture["box"]["size"])},{value(fixture["box"]["origin"])},{value(row.get("rotation",0))}_f64.to_radians(),{clip_expr})'
for i,row in enumerate(fixture["cases"]):
    for sign,suffix in ((1,"positive"),(-1,"negative")):
        n=[v*sign for v in row["normal"]]
        edges=row["clip"]["edges"] if "clip" in row else 12
        parts.append(f'#[test] fn section_{i}_{suffix}(){{section_geometry({call(row)},{value(row["origin"])},{value(n)},{row["vertices"]},{edges},{value(row["area"])},{value(row["bounds"])});}}')
for i,row in enumerate(fixture["refused"]):
    parts.append(f'#[test] fn contact_{i}(){{section_contact({call(row)},{value(row["origin"])},{value(row["normal"])});}}')
fixture_booleans=json.loads((flow/"🔣️original-rotated-box-intersection.json").read_text())
def oriented(spec):
    return f'original_oriented_box({value(spec["size"])},{value(spec["angle"])},{value(spec["origin"])})'
row=fixture_booleans["cases"][0]
for index,section in enumerate(row["sections"]):
    stock=f'from_boolean({oriented(row["a"])}.intersect({oriented(row["b"])}).unwrap())'
    origin,normal=(value(section[key]) for key in ("origin","normal"))
    if "vertices" in section:
        check=f'section_geometry({stock},{origin},{normal},{section["vertices"]},24,{value(section["area"])},{value(section["bounds"])});'
    else:
        check=f'section_contact({stock},{origin},{normal});'
    parts.append(f'#[test] fn box_intersection_section_{index}(){{{check}}}')
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    source=Path(folder)/"oriented-box-section.rs"
    exe=source.with_suffix(".exe" if os.name=="nt" else "")
    source.write_text("\n".join(parts))
    for args in (("rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(exe)),(str(exe),)):
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",*args],check=True)
