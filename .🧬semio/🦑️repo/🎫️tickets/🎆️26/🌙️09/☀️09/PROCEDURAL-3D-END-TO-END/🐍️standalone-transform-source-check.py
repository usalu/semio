"""Exercise the exact production rational conic recipe and its funded publication buffers."""
import contextlib
import io
import json
from pathlib import Path
import runpy

here=Path(__file__).parent
root=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
capture=io.StringIO()
with contextlib.redirect_stdout(capture):
    runpy.run_path(str(here/"🐍️curve-approx-source-check.py"))
preamble=capture.getvalue().split("use std::mem::size_of;",1)[0]
preamble=preamble.replace("enum Curve3{Nurbs", "enum Curve3{Circle{frame:Frame3,radius:f64},Ellipse{frame:Frame3,major_radius:f64,minor_radius:f64},Nurbs")
print(preamble.replace(", homogeneous_jet_at_span_original",""))
print(r"""
#[derive(Clone,Copy)]struct Frame3{origin:Pnt3,x:Vec3,y:Vec3,z:Vec3}
impl Frame3 {
 fn to_world(self,p:Pnt3)->Pnt3 {self.origin+self.x*p.x+self.y*p.y+self.z*p.z}
}
#[derive(Clone,Copy)]struct Affine3{linear:[f64;3],origin:Pnt3}
impl Affine3{
 fn apply_point(self,p:Pnt3)->Pnt3 {
   let p=p-self.origin;
   self.origin+Pnt3::new(p.x*self.linear[0],p.y*self.linear[1],p.z*self.linear[2])
 }
}
impl Curve3{
 fn domain(&self)->(f64,f64){(0.,std::f64::consts::TAU)}
""")
source = (root/"📸️representation/➰️curve/🦀️.rs").read_text()
a=source.index("    fn transformed_via_nurbs(")
b=source.index("// #region 🔖️Tests",a)
print(source[a:b])
a=source.index("pub struct ArcSplineRecipe")
b=source.index("fn arc_to_nurbs_with_span",a)
print("#[derive(Clone,Copy)]")
print(source[a:b])
print("mod arc{")
arc = (root/"⚙️engine/🧠️native-registry/🧮️spline-construction/🌙️arc/🦀️.rs").read_text()
print(arc.split("\n",2)[2].replace("#[derive(semio_framework_value::RetireOwned)]",""))
print("}")
print(r"""
fn verify(frame:Frame3,radii:[f64;2],linear:[f64;3],origin:Pnt3,expected:&[(f64,EVec3)]) {
 let map=Affine3{linear,origin};
 let source=Curve3::Ellipse{frame,major_radius:radii[0],minor_radius:radii[1]};
 let recipe=source.affine_conic_recipe(map).unwrap();
 let mut builder=arc::ArcSplineBuild::new(recipe,PersistentLabel(5));
 let mut result=None;
 for _ in 0..20_000 {
   let mut grant=builder.demand();
   assert!(grant.copy_bytes<=128);
   let (idle,data)=builder.step(&mut RetainedAllocationEffect(&grant)).unwrap();
   assert_eq!(idle.copied_items,0);
   assert!(data.is_none());
   grant.maximum_items=1;
   let (work,done)=builder.step(&mut RetainedAllocationEffect(&grant)).unwrap();
   assert!(work.fits(grant));
   assert_eq!(work.copied_items,1);
   if done.is_some(){result=done;break;}
 }
 let Curve3::Nurbs{knots,controls,weights}=result.unwrap() else{panic!("expected funded spline")};
 let Curve3::Nurbs{knots:cold_knots,controls:cold,weights:cold_weights}=source.transformed_via_nurbs(&map) else{panic!("expected cold spline")};
 assert_eq!(knots.knots,cold_knots.knots);
 assert_eq!(weights,cold_weights);
 assert_eq!(controls.len(),cold.len());
 for (a,b) in controls.iter().zip(cold){assert!(a.distance(b)<1e-14);}
 let curve=NurbsCurve3{knots,controls,weights};
 for (parameter,expected) in expected{
   let point=curve_ops::eval_nurbs_curve3(&curve,*parameter);
   assert!(point.distance(pnt(*expected))<1e-8);
 }
 for i in 0..=100{
   let t=i as f64/100.*std::f64::consts::TAU;
   let oracle=map.apply_point(frame.to_world(Pnt3::new(radii[0]*t.cos(),radii[1]*t.sin(),0.)));
   assert!(curve_ops::eval_nurbs_curve3(&curve,t).distance(oracle)<1e-8);
 }
}
fn pnt(a:EVec3)->Pnt3{Pnt3::from_array(a)}
#[test]fn funded_skew_conic_matches_cold_rational_arc(){
""")
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🎯️brep-evaluate/🔣️.json"))
data=json.loads(fixture.read_text())["standaloneAffineTransforms"]
row=next(case for case in data if case["kind"]=="ellipse")
def vec(v):
    return "["+",".join(str(float(x)) for x in v)+"]"
witness=",".join(f"({t[0]},{vec(p)})" for t,p in row["witnesses"])
print(f'let frame=Frame3{{origin:pnt({vec(row["center"])}),x:pnt({vec(row["x"])}),y:pnt({vec(row["y"])}),z:pnt({vec(row["normal"])}).normalized().unwrap()}};')
print(f'verify(frame,{vec(row["radii"])},{vec(row["vector"])},pnt({vec(row["origin"])}),&[{witness}]);')
print("}")
