"""Check original coaxial Boolean classification and the exact annular boundary."""
import contextlib
import io
import json
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
brep=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
output=io.StringIO()
with contextlib.redirect_stdout(output):
    runpy.run_path(str(ticket/"🐍️point-face-source-check.py"))
source=output.getvalue()
source=source.replace("enum SolidPrimitive{Box","enum SolidPrimitive{Sphere{frame:Frame3,radius:f64},Box")
source=source.replace("fn volume(&self)->f64 {match *self{Self::Box","fn volume(&self)->f64 {match *self{Self::Sphere{radius,..}=>4.*std::f64::consts::PI*radius.powi(3)/3.,Self::Box")
source=source.replace("enum SolidBoundary{Bore(BoreBoundary),Orthogonal(OrthogonalBoundary)}",
    "enum SolidBoundary{Bore(BoreBoundary),Orthogonal(OrthogonalBoundary),Analytic(SolidPrimitive),Annular(AnnularRevolution),SphericalShell(SphericalShell),SphericalCaps(SphericalCaps),CircularPrism(CircularPrism),Prism(PrismBoundary),BoxPrism(PrismBoundary,BoxPrismSource),PerforatedBox(PerforatedPrism,BoxPrismSource),ClippedBox(ClippedBoxBoundary,ClippedBox),Hull(point_hull::HullBoundary)}")
source=source.replace("enum Surface{Plane{frame:Frame3},Other}","enum Surface{Plane{frame:Frame3},Cylinder{frame:Frame3,radius:f64},Sphere{frame:Frame3,radius:f64},Other}")
source=source.replace("} use material_cells::OrthogonalCells;", "} pub(super) use material_cells::OrthogonalCells;")
source += """use orthogonal_fixture::OrthogonalCells;
use primitives::planar_pcurve;
impl Pnt3{fn distance(self,b:Self)->f64{(self-b).norm()}fn from_array(a:EVec3)->Self{pnt(a)}fn lerp(self,b:Self,t:f64)->Self{self+(b-self)*t}}
"""
oriented_path=next(brep.rglob("🏗️solid-transaction/📦️oriented-box/🦀️.rs"))
oriented=oriented_path.read_text().split("const ANGULAR",1)[1]
oriented="const ANGULAR"+oriented.replace("semio_framework_3d_brep_construction::prism_boundary::MAX_PRISM_CORNERS","MAX_PRISM_CORNERS").replace("pub(super) fn contains","fn contains")
prism_source=next(brep.rglob("🗜️prism-boundary/🦀️.rs")).read_text().split("pub const MAX_PRISM_CORNERS",1)[1]
oriented="pub const MAX_PRISM_CORNERS"+prism_source+oriented
for path in ("📐️footprint/🦀️.rs","🔺️polygon-region/🦀️.rs"):
    leaf=next(brep.rglob(path)).read_text().split("\n",2)[2]
    module=Path(path).parent.name.split("️",1)[-1].replace("-","_")
    oriented+=f"mod {module}{{\n{leaf}}}\nuse {module}::*;\n"
perforated=next(brep.rglob("🪟️perforated-prism/🦀️.rs")).read_text().split("const CORNERS",1)[1]
oriented+="mod perforated_prism{use super::*;const CORNERS"+perforated+"} use perforated_prism::*;\n"
oriented=oriented.replace("pub(super) fn perforated_", "fn perforated_")
layout=next(brep.rglob("📐️box-finish-layout/🦀️.rs")).read_text().split("\n",2)[2]
clip=next(brep.rglob("✂️clipped-box/🦀️.rs")).read_text().split("const NODES",1)[1]
oriented+="mod box_finish_layout{"+layout+"} mod clipped_box{use super::*; const NODES"+clip+"} use clipped_box::*;"
section=next(brep.rglob("✂️box-section/🦀️.rs")).read_text().split("const MAX_PROFILE",1)[1]
oriented+="mod box_section{use super::*; const MAX_PROFILE"+section+"} use box_section::*;"
oriented=oriented.replace("pub(super) fn section", "fn section").replace("pub(super) fn placement", "fn placement").replace("pub(super) fn common_axis", "fn common_axis").replace("pub(super) fn footprint_region", "fn footprint_region")
oriented=oriented.replace("semio_framework_3d_brep_construction::skew_box_clip::","skew_box_clip::")
convex_path=next(brep.rglob("✂️intersect/📦️convex-box/🦀️.rs"))
convex=convex_path.read_text(encoding="utf8").split("pub const BOX_BOUNDARY_VERTICES",1)[1]
recess=(convex_path.parent/"🪚️recess/🦀️.rs").read_text(encoding="utf8").split("use super::*;",1)[1]
convex=convex.replace('#[path="🪚️recess/🦀️.rs"]\nmod recess;', "mod recess{use super::*;"+recess+"}")
shells=(convex_path.parent/"🐚️shells/🦀️.rs").read_text(encoding="utf8").split("use super::*;",1)[1]
convex=convex.replace('#[path="🐚️shells/🦀️.rs"]\nmod shells;', "mod shells{use super::*;"+shells+"}")
oriented+="mod convex_box{use super::*;pub const BOX_BOUNDARY_VERTICES"+convex+"}"
hull=next(brep.rglob("📦️point-hull/🦀️.rs")).read_text()
shared=hull[hull.index("const MAX_POINTS"):hull.index("fn projection(")]
normal=hull[hull.index("fn ordinary_normal("):hull.index("fn convex_ring(")]
topology=hull[hull.index("impl HullBoundary {"):hull.index("    fn scan_plane(")]
oriented+="mod point_hull{use super::*;use convex_box::{BoxSupportBoundary,BOX_BOUNDARY_VERTICES};type NativeVec3=Vec3;"+shared.replace("Pnt3::default()","pnt([0.;3])")+normal+topology+"}} use point_hull::HullBoundary;"
skew=next(brep.rglob("✂️skew-box-clip/🦀️.rs")).read_text().split("pub enum SkewBoxBoundary",1)[1]
oriented+="mod skew_box_clip{use super::*;use convex_box::*;pub(super) use convex_box::{BoxSupportPieces,box_recess_pieces,boxes_separated,contains_box};use point_hull::HullBoundary;pub enum SkewBoxBoundary"+skew+"}"
oriented+="""
impl Frame3{
 fn from_x_z(origin:Pnt3,x:Vec3,z:Vec3)->Option<Self>{
  let z=z.normalized()?;
  let y=z.cross(x).normalized()?;
  Some(Self{origin,x:y.cross(z),y,z})
 }
}
#[derive(Clone,Copy)]struct PrimitiveProvenance{primitive:SolidPrimitive,nurbs_cuboid:bool}
#[derive(Clone,Copy)]enum SolidProvenance{Primitive(PrimitiveProvenance),PlanarBoundary(Option<BoxPrismSource>)}
impl SolidPrimitive{fn affinely_transformed(self,map:&Affine3)->Option<Self>{
 let Self::Box{origin,width,depth,height}=self else{return None;};
 mapped_box(origin,[width,depth,height],map)
}}
"""
placement=next(brep.rglob("🧊️analytic-solids/🧭️placement/🦀️.rs")).read_text()
oriented+=placement[placement.index("fn mapped_box("):placement.index("impl SolidPrimitive")]
prism_path=brep/"🛠️operations/📐️circular-prism"
prism=(prism_path/"🦀️.rs").read_text().split("#[derive(Clone,Copy)]",1)[1]
clip=next(brep.rglob("🧮️boolean-recipe/✂️disk-rectangle/🦀️.rs")).read_text().split("use std::f64::consts::TAU;",1)[1]
prism="#[derive(Clone,Copy)]"+prism+"mod disk_rectangle{use super::*;"+clip+"}"
annular=next(brep.rglob("🧿️annular-revolution/🦀️.rs")).read_text()
annular=annular[annular.index("/// 🛞️"):].replace("super::revolution_distance","crate::revolution_distance")
metric=next(brep.rglob("🧭️revolution-distance/🦀️.rs")).read_text().split("/// 🪣️",1)[1]
annular+="mod revolution_distance{use super::*;use std::f64::consts::TAU;/// 🪣️"+metric+"}"
sphere=next(brep.rglob("🔵️spherical-shell/🦀️.rs")).read_text().split("\n",4)[4]
caps=next(brep.rglob("🌗️spherical-caps/🦀️.rs")).read_text()
caps=caps[caps.index("/// 🔗️"):]
measure=next(brep.rglob("🌗️spherical-caps/📐️measure/🦀️.rs")).read_text().split("\n",2)[2]
caps="mod measure{"+measure+"}"+caps
boundary=next(brep.rglob("🧿️annular-boundary/🦀️.rs")).read_text()
boundary=boundary[boundary.index("fn unsupported()"):]
two_steps=next(brep.rglob("🪜️double-shoulder/🦀️.rs")).read_text().split("\n",2)[2]
boundary="mod two_steps{"+two_steps+"}"+boundary
sampling=next(brep.rglob("🧿️annular-sampling/🦀️.rs")).read_text().split("\n",4)[4]
coaxial=next(brep.rglob("🥫️coaxial-cylinders/🦀️.rs")).read_text()
coaxial=coaxial[coaxial.index("struct CoaxialPair"):]
boolean=next(brep.rglob("🧮️boolean-recipe/🦀️.rs")).read_text()
boolean=boolean[boolean.index("#[derive(Clone, Copy)]"):]
boolean=boolean.replace("semio_framework_3d_brep_construction::skew_box_clip::","skew_box_clip::")
mixed=next(brep.rglob("🧱️box-cylinder/🦀️.rs")).read_text().split("\n",2)[2]
mixed=mixed.replace("use semio_framework_3d_brep_vector::Vec3;", "")
parallel=next(brep.rglob("🥫️parallel-cylinders/🦀️.rs")).read_text().split("fn unit_segment",1)[1]
parallel="fn unit_segment"+parallel
sphere_box=next(brep.rglob("🌗️sphere-box/🦀️.rs")).read_text().split("pub(super) fn combine",1)[1]
for classifier in brep.rglob("🌐️sphere-cylinder/🦀️.rs"):
    sphere_cylinder=classifier.read_text().split("use super::*;",1)[1]
    boolean="mod sphere_cylinder{use super::*;"+sphere_cylinder+"}"+boolean
boolean="mod box_cylinder{"+mixed+"}mod parallel_cylinders{use super::*;"+parallel+"}mod sphere_box{use super::*;pub(super) fn combine"+sphere_box+"}"+boolean
spherical_boundary=next(brep.rglob("🌗️spherical-cap-boundary/🦀️.rs")).read_text().split("pub(super) fn surface",1)[1]
spherical_boundary="mod spherical_boundary{use super::*;pub(super) fn surface"+spherical_boundary.replace("semio_framework_3d_brep_construction::primitives::","primitives::")+"}"


boolean=boolean.replace("error.kind != ValueRefusalKind::UnsupportedOwner",'error.0 != "unsupported"')
boolean=boolean.replace("error.kind == ValueRefusalKind::InvalidValue",'error.0 == "empty"')
transaction=next(brep.rglob("🏗️solid-transaction/🦀️.rs")).read_text()
begin=transaction.index("    fn shell_count(")
shell_source=(next(brep.rglob("🐚️shell-recipe/🦀️.rs"))).read_text()
rings=shell_source.split("pub(super) fn rings",1)[1].split("pub(super) fn vertex",1)[0]
shell_methods="mod shell_recipe{use super::*;fn exterior(shape:SphericalShell)->SphericalShellBoundary{shape.boundary(false)}pub(super) fn rings"+rings.replace("SolidPrimitive::Box","SphericalShellBoundary::Box")+"}"+ "impl SolidBoundary{fn face_count(&self)->usize{match self{Self::Annular(p)=>p.face_count(),_=>unreachable!()}}"+transaction[begin:transaction.index("    fn vertex(",begin)]+"}"

fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").rglob("🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json"))
def array(xs):
    return "["+",".join(str(float(x)) for x in xs)+"]"

checks=r"""
use std::f64::consts::{FRAC_PI_2,PI,TAU};
struct SampleVertex{point:Pnt3,normal:Vec3}
fn cylinder(origin:EVec3,radius:f64,height:f64)->SolidPrimitive{
 let frame=Frame3::from_normal(pnt(origin),Vec3::Z).unwrap();
 SolidPrimitive::Cylinder{frame,radius,height}
}
fn check(a:SolidPrimitive,b:SolidPrimitive,expected_volume:f64,expected_area:f64,centroid:EVec3,fuse:bool){
 let op=if fuse{BooleanOp::Unite}else{BooleanOp::Cut};
 let result=BooleanShape::Analytic(a).combine(BooleanShape::Analytic(b),op).unwrap();
 let shells=result.shell_count();
 let faces=(0..shells).map(|shell|result.shell_face_count(shell)).collect::<Vec<_>>();
 let SolidBoundary::Annular(shape)=result else{panic!("expected material annulus");};
 assert_eq!(faces,if shape.has_void_shell(){vec![3,3]}else{vec![shape.face_count()]});
 assert!(shape.full());
 assert_eq!(shape.has_step(),fuse);
 assert!((shape.volume()-expected_volume).abs()<1e-10);
 assert!((shape.area()-expected_area).abs()<1e-10);
 assert!((shape.center_of_mass()-pnt(centroid)).norm()<1e-10);
 let mut uses=vec![(0,0);shape.edge_count()];
 let mut edge_shells=vec![0u8;shape.edge_count()];
 let mut area=0.;
 for (ring,count) in annular_boundary::ring_sizes(shape).iter().copied().enumerate(){
  let face=annular_boundary::face_index(shape,ring);
  let surface=annular_boundary::surface(shape,face);
  if let Surface::Cylinder{frame,radius}=surface{
   assert_eq!(annular_boundary::face_flipped(shape,face),!fuse&&radius==shape.inner);
   if radius==shape.inner{
    let (SolidPrimitive::Cylinder{radius:ra,..},SolidPrimitive::Cylinder{radius:rb,..})=(a,b)else{unreachable!();};
    let SolidPrimitive::Cylinder{frame:tool,..}= (if fuse&&ra<rb{a}else{b}) else{unreachable!();};
    let shift=frame.origin-tool.origin;
    assert!((shift-tool.z*shift.dot(tool.z)).norm()<1e-12);
   }
   let depth=if shape.has_two_steps(){shape.level(face)-shape.level(face-1)}
       else if face==3{shape.ceiling()-shape.floor()}else{shape.level(2)};
   area+=TAU*radius*depth;
  }else if shape.has_two_steps(){
   if ring<7{
    let radius_sq=if matches!(face,2|4){shape.outer.powi(2)-shape.inner.powi(2)}else{shape.inner.powi(2)};
    area+=std::f64::consts::PI*radius_sq;
   }
  }else if ring<4||shape.has_floor()&&ring==4||shape.has_void_shell()||fuse&&ring==5{
   let area_without_hole=if face>=4{shape.inner.powi(2)}else{shape.outer.powi(2)};
   let hole=if !shape.has_void_shell()&&(face==2||face==0&&!shape.has_floor()&&!fuse){shape.inner.powi(2)}else{0.};
   area+=std::f64::consts::PI*(area_without_hole-hole);
  }
  let mut first=None;
  let mut last=None;
  for member in 0..count {
   let trim=annular_boundary::trim(shape,ring,member);
   let edge=annular_boundary::edge(shape,trim.edge);
   edge_shells[trim.edge] |= if shape.has_void_shell()&&face>=3{2}else{1};
   for fraction in [0.,0.23,0.75,1.] {
    let t=trim.range.0+(trim.range.1-trim.range.0)*fraction;
    let uv=trim.curve.eval(t);
    let p=match surface{
     Surface::Plane{frame}=>frame.origin+frame.x*uv.x+frame.y*uv.y,
     Surface::Cylinder{frame,radius}=>frame.origin+frame.z*uv.y+frame.x*(radius*uv.x.cos())+frame.y*(radius*uv.x.sin()),
     _=>unreachable!()
    };
    assert!((p-edge.curve.eval(t)).norm()<1e-12);
   }
   let [v,w]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
   if let Some(previous)=last{assert_eq!(previous,v);}
   first.get_or_insert(v);last=Some(w);
   if trim.forward!=annular_boundary::face_flipped(shape,face){uses[trim.edge].0+=1;}else{uses[trim.edge].1+=1;}
  }
  assert_eq!(last,first);
 }
 assert!((area-expected_area).abs()<1e-10);
 let mut preview_area=0.;
 let mut preview_volume=[0.,0.];
 for face in 0..shape.face_count(){
  for slot in 0..annular_sampling::slots(shape,face,256){
   let tri=annular_sampling::triangle(shape,face,slot,256);
   let [a,b,c]=tri.each_ref().map(|v|v.point);
   let normal=tri[0].normal+tri[1].normal+tri[2].normal;
   let cross=(b-a).cross(c-a);
   preview_area+=cross.norm()/2.;
   preview_volume[usize::from(shape.has_void_shell()&&face>=3)]+=a.dot(b.cross(c))*if cross.dot(normal)>0.{1./6.}else{-1./6.};
  }
 }
 assert!((preview_area-expected_area).abs()<0.02);
 assert!((preview_volume.iter().sum::<f64>()-expected_volume).abs()<0.02);
 if shape.has_void_shell(){
  let absolute_void=std::f64::consts::PI*shape.inner.powi(2)*(shape.ceiling()-shape.floor());
  assert!((preview_volume[1]+absolute_void).abs()<0.02&&preview_volume[0]>0.);
  assert_eq!(edge_shells.into_iter().filter(|&m|m==2).count(),3);
 }else{
  assert_eq!(preview_volume[1],0.);
 }
 assert!(uses.into_iter().all(|uses|uses==(1,1)));
 if fuse {
  let SolidBoundary::Annular(swapped)=BooleanShape::Analytic(b).combine(BooleanShape::Analytic(a),op).unwrap()else{panic!("expected commutative fuse");};
  assert_eq!(shape.volume(),swapped.volume());
  let extent=if shape.has_two_steps(){shape.level(3)-shape.level(2)}else{shape.level(2)};
  let neck=shape.height-extent;
  let separate=SolidPrimitive::Cylinder{frame:Frame3{origin:shape.frame.origin+shape.frame.z*(extent+1.),..shape.frame},radius:shape.inner,height:neck};
  let broad=SolidPrimitive::Cylinder{frame:shape.frame,radius:shape.outer,height:extent};
  assert!(BooleanShape::Analytic(broad).combine(BooleanShape::Analytic(separate),op).is_err());
  let both_sides=SolidPrimitive::Cylinder{frame:Frame3{origin:shape.frame.origin-shape.frame.z,..shape.frame},radius:shape.inner,height:extent+2.};
  let SolidBoundary::Annular(both)=BooleanShape::Analytic(broad).combine(BooleanShape::Analytic(both_sides),op).unwrap()else{panic!("expected two shoulders");};
  assert!(both.has_two_steps());
  return;
 }
 let reverse=Frame3{origin:shape.frame.origin+shape.frame.z*shape.height,y:-shape.frame.y,z:-shape.frame.z,..shape.frame};
 let reversed_tool=SolidPrimitive::Cylinder{frame:reverse,radius:shape.inner,height:shape.height};
 assert!(matches!(BooleanShape::Analytic(a).combine(BooleanShape::Analytic(reversed_tool),BooleanOp::Cut),Ok(SolidBoundary::Annular(_))));
 let cavity=SolidPrimitive::Cylinder{frame:Frame3{origin:shape.frame.origin+shape.frame.z*(shape.height*0.25),..shape.frame},radius:shape.inner,height:shape.height*0.5};
 let SolidBoundary::Annular(enclosed)=BooleanShape::Analytic(a).combine(BooleanShape::Analytic(cavity),BooleanOp::Cut).unwrap()else{panic!("expected a closed void");};
 assert!(enclosed.has_void_shell());
 for tool_radius in [shape.outer,shape.outer*2.]{
  let disconnected=SolidPrimitive::Cylinder{frame:Frame3{origin:shape.frame.origin+shape.frame.z*(shape.height*0.25),..shape.frame},radius:tool_radius,height:shape.height*0.5};
  assert!(BooleanShape::Analytic(a).combine(BooleanShape::Analytic(disconnected),BooleanOp::Cut).is_err());
 }
 let eccentric=SolidPrimitive::Cylinder{frame:Frame3{origin:shape.frame.origin+shape.frame.x*0.01,..shape.frame},radius:shape.inner,height:shape.height};
 assert!(matches!(BooleanShape::Analytic(a).combine(BooleanShape::Analytic(eccentric),BooleanOp::Cut),Ok(SolidBoundary::Annular(_))));
 assert!(matches!(BooleanShape::Analytic(a).combine(BooleanShape::Analytic(b),BooleanOp::Intersect),Ok(SolidBoundary::Analytic(_))));
}
#[test]fn coaxial_booleans_preserve_exact_revolved_topology(){
"""
for row in json.loads(fixture.read_text())["cases"]:
    if not row["name"].startswith(("coaxial ","eccentric-cylinder ")) and "eccentricShoulders" not in row:
        continue
    solids=[row["inputs"][side] for side in ["a","b"]]
    operands=[f'cylinder({array(s["by"])},{float(s["of"]["radius"])},{float(s["of"]["height"])})' for s in solids]
    expected=[float(row["outputs"]["shape"][key]) for key in ["volume","area"]]
    fuse=str(row["kind"].endswith(".fuse")).lower()
    centroid=array(row["outputs"]["shape"]["centroid"])
    checks+=f'check({operands[0]},{operands[1]},{expected[0]},{expected[1]},{centroid},{fuse});\n'
checks+="}\n"
checks+=r"""
fn rings_are_nested(shape:AnnularRevolution,outer:usize,inner:usize){
 for side in 0..8{
  let u=TAU*f64::from(side)/8.;
  let v=TAU*f64::from(side+1)/8.;
  let a=shape.point(outer,u);
  let b=shape.point(outer,v);
  for i in 0..8{
   let point=shape.point(inner,TAU*f64::from(i)/8.);
   assert!((b-a).cross(point-a).dot(shape.frame.z)>0.);
  }
 }
}
#[test]fn horizontal_eccentric_shoulders_keep_antiparallel_axes_and_coarse_incidence(){
 let wide=SolidPrimitive::Cylinder{frame:Frame3::from_normal(pnt([0.,0.,0.]),Vec3::X).unwrap(),radius:3.,height:4.};
 let narrow=SolidPrimitive::Cylinder{frame:Frame3::from_normal(pnt([6.,0.5,0.25]),-Vec3::X).unwrap(),radius:1.,height:8.};
 check(narrow,wide,40.*PI,50.*PI,[2.,0.05,0.025],true);
 let almost=cylinder([0.,1.9999,-1.],1.,7.);
 let stock=cylinder([0.,0.,0.],3.,4.);
 let union=|tool|BooleanShape::Analytic(stock).combine(BooleanShape::Analytic(tool),BooleanOp::Unite);
 let SolidBoundary::Annular(shape)=union(almost).unwrap()else{unreachable!();};
 assert!(shape.has_two_steps());
 rings_are_nested(shape,2,1);
 rings_are_nested(shape,3,4);
 assert!(union(cylinder([0.,2.,-1.],1.,7.)).is_err());
 assert!(union(cylinder([0.,1.9,4.01],1.,1.)).is_err());
}
#[test]fn eccentric_bore_cap_samples_stay_nested_with_a_known_seam(){
 let stock=cylinder([0.,0.,0.],3.,4.);
 let shift=1.9999;
 let reversed=Frame3::from_normal(pnt([0.6*shift,0.8*shift,4.]),-Vec3::Z).unwrap();
 let tool=SolidPrimitive::Cylinder{frame:reversed,radius:1.,height:4.};
 let SolidBoundary::Annular(shape)=BooleanShape::Analytic(stock).combine(BooleanShape::Analytic(tool),BooleanOp::Cut).unwrap()
 else{panic!("expected eccentric bore with the antiparallel cutter");};
 let offset=shape.inner_frame().origin-shape.frame.origin;
 assert!((offset-shape.frame.x*shift).norm()<1e-12&&shape.frame.z==Vec3::Z);
 rings_are_nested(shape,1,0);
 let touching=cylinder([2.,0.,0.],1.,4.);
 assert!(BooleanShape::Analytic(stock).combine(BooleanShape::Analytic(touching),BooleanOp::Cut).is_err());
}
fn sphere_stock(origin:EVec3,radius:f64)->SolidPrimitive{
 let frame=Frame3::from_normal(pnt(origin),Vec3::Z).unwrap();
 SolidPrimitive::Sphere{frame,radius}
}
fn sphere_case(first:SolidPrimitive,last:SolidPrimitive,volume:Option<f64>,measure:Option<(EVec3,f64)>){
 let stock=BooleanShape::Analytic(first);
 let tool=BooleanShape::Analytic(last);
 let plan=stock.combine(tool,BooleanOp::Cut);
 let SolidPrimitive::Sphere{frame,radius}=first else{unreachable!()};
 let SolidPrimitive::Sphere{frame:other,radius:tool_radius}=last else{unreachable!()};
 match (plan,volume){
  (Err(error),None)=>assert_eq!(error.0,"empty"),
  (Ok(SolidBoundary::Analytic(SolidPrimitive::Sphere{frame:unchanged,radius:retained})),Some(exact))=>{
   assert!(frame.origin!=other.origin);
   assert!(unchanged.origin==frame.origin&&retained==radius);
   assert!((first.volume()-exact).abs()<1e-10);
   assert_eq!(stock.combine(tool,BooleanOp::Intersect).err().unwrap().0,"empty");
   assert!(stock.combine(tool,BooleanOp::Unite).is_err());
  }
  (Ok(SolidBoundary::SphericalShell(shell)),Some(exact))=>{
   assert!(matches!(shell.boundary(false),SphericalShellBoundary::Sphere{frame:outer,..} if outer.origin==frame.origin));
   assert!(matches!(shell.boundary(true),SphericalShellBoundary::Sphere{frame:inner,..} if inner.origin==other.origin));
   if let Some((center,area))=measure{
    assert!((shell.center_of_mass()-pnt(center)).norm()<1e-12);
    assert!((shell.area()-area).abs()<1e-10);
   }
   for operation in [BooleanOp::Unite,BooleanOp::Intersect]{
    let result=stock.combine(tool,operation).unwrap();
    let SolidBoundary::Analytic(SolidPrimitive::Sphere{frame:exact,radius:bound})=result else{panic!("expected contained sphere");};
    if matches!(operation,BooleanOp::Unite){assert!(exact.origin==frame.origin&&bound==radius);}
    else{assert!(exact.origin==other.origin&&bound==tool_radius);}
   }
   assert_eq!(shell.inner,tool_radius);
   assert!(matches!(shell.boundary(false),SphericalShellBoundary::Sphere{radius:outer,..} if outer==radius));
   assert!((shell.volume()-exact).abs()<1e-10);
  }
  _=>panic!("unexpected certified sphere subtraction result"),
 }
}
#[test] fn sphere_cut_preserves_concentric_radii_and_disjoint_material(){
"""
for row in json.loads(fixture.read_text())["cases"]:
    if not row["name"].startswith(("concentric sphere cut","eccentric sphere cut","disjoint sphere cut")):continue
    operands=[row["inputs"][side] for side in ["a","b"]]
    inputs=[f'sphere_stock({array(entry["by"])},{float(entry["of"]["radius"])})' for entry in operands]
    result="None" if "fault" in row else f'Some({float(row["outputs"]["shape"]["volume"])})'
    measure="None"
    if "centerOfMass" in row:
        measure=f'Some(({array(row["centerOfMass"])},{row["outputs"]["shape"]["area"]}))'
    checks+=f'sphere_case({inputs[0]},{inputs[1]},{result},{measure});\n'
checks+="}"
checks+=r"""
#[test] fn touching_spherical_wall_is_refused() {
 let a=BooleanShape::Analytic(sphere_stock([2.,-3.,4.],3.5));
 for x in [4.25,4.25-0.25*Tol::DEFAULT.value(),4.25+0.25*Tol::DEFAULT.value()]{
  let b=BooleanShape::Analytic(sphere_stock([x,-3.,4.],1.25));
  assert_eq!(a.combine(b,BooleanOp::Cut).err().unwrap().0,"unsupported");
 }
}
fn overlap_case(first:SolidPrimitive,last:SolidPrimitive,operation:BooleanOp,volume:f64){
 let result=BooleanShape::Analytic(first).combine(BooleanShape::Analytic(last),operation).unwrap();
 let SolidBoundary::SphericalCaps(shape)=result else{panic!("missing retained spherical-cap pair")};
 assert!((shape.volume()-volume).abs()<1e-10);
 assert_eq!(shape.is_inward(1),matches!(operation,BooleanOp::Cut));
 if !matches!(operation,BooleanOp::Cut){
  let reverse=BooleanShape::Analytic(last).combine(BooleanShape::Analytic(first),operation).unwrap();
  let SolidBoundary::SphericalCaps(other)=reverse else{panic!("missing symmetric spherical-cap pair")};
  assert!((other.volume()-volume).abs()<1e-10);
 }
}
#[test]fn certified_sphere_boolean_dispatches_each_cap_role(){
"""

for row in json.loads(fixture.read_text())["cases"]:
    if not row["name"].startswith("overlapping sphere"):continue
    operands=[row["inputs"][side] for side in ["a","b"]]
    inputs=[f'sphere_stock({array(entry["by"])},{float(entry["of"]["radius"])})' for entry in operands]
    modes={"fuse":"Unite","intersect":"Intersect","cut":"Cut"}
    mode=modes[row["kind"].split(".")[-1]]
    checks+=f'overlap_case({inputs[0]},{inputs[1]},BooleanOp::{mode},{row["outputs"]["shape"]["volume"]});\n'
checks+="}\n"
checks+=r"""
fn check_mixed(a:SolidPrimitive,b:SolidPrimitive,op:BooleanOp,expected:Option<([EVec3;2],f64,bool)>){
 let first=BooleanShape::Analytic(a);
 let last=BooleanShape::Analytic(b);
 let result=first.combine(last,op);
 let Some((bounds,volume,is_box))=expected else{
  assert_eq!(result.err().unwrap().0,"empty");
  return;
 };
 let SolidBoundary::Analytic(shape)=result.unwrap()else{panic!("expected retained mixed primitive");};
 let limits=match shape{
  SolidPrimitive::Box{origin,width,depth,height}=>{
   assert!(is_box);
   [origin.to_array(),(origin+Pnt3::new(width,depth,height)).to_array()]
  }
  SolidPrimitive::Cylinder{frame,radius,height}=>{
   assert!(!is_box);
   let [x,y,z]=frame.origin.to_array();
   let far=z+frame.z.z*height;
   [[x-radius,y-radius,z.min(far)],[x+radius,y+radius,z.max(far)]]
  }
  _=>unreachable!()
 };
 assert_eq!(limits,bounds);
 assert!((shape.volume()-volume).abs()<1e-10);
}
fn box_shape(origin:EVec3,dimensions:EVec3)->SolidPrimitive{
 let [width,depth,height]=dimensions;
 SolidPrimitive::Box{origin:pnt(origin),width,depth,height}
}
#[test]fn mixed_box_and_cylinder_operations_preserve_certified_profiles(){
"""

def mixed_operand(entry):
    spec=entry["of"]
    if spec["recipe"]=="box":
        return f'box_shape({array(entry["by"])},{array(spec["size"])})'
    return f'cylinder({array(entry["by"])},{float(spec["radius"])},{float(spec["height"])})'
for row in json.loads(fixture.read_text())["cases"]:
    if not row["name"].startswith("box-cylinder ") or "retainedArcPrism" in row:continue
    a,b=[mixed_operand(row["inputs"][side]) for side in ["a","b"]]
    mode={"intersect":"Intersect","fuse":"Unite","cut":"Cut"}[row["kind"].split(".")[-1]]
    value="None"
    if "outputs" in row:
        bounds="["+",".join(array(side) for side in row["outputs"]["shape"]["bbox"])+"]"
        is_box=str(row["retainedProfile"]=="box").lower()
        value=f'Some(({bounds},{float(row["outputs"]["shape"]["volume"])},{is_box}))'
    checks+=f'check_mixed({a},{b},BooleanOp::{mode},{value});\n'
checks+=r"""
 let a=box_shape([0.,0.,0.],[6.,5.,4.]);
 let reversed=Frame3{origin:Pnt3::new(3.,2.5,2.),y:-Vec3::Y,z:-Vec3::Z,..Frame3::WORLD};
 let b=SolidPrimitive::Cylinder{frame:reversed,radius:1.,height:3.};
 let result=BooleanShape::Analytic(b).combine(BooleanShape::Analytic(a),BooleanOp::Intersect).unwrap();
 let SolidBoundary::Analytic(SolidPrimitive::Cylinder{frame,height,..})=result else{panic!("expected reversed cylinder");};
 assert!(frame.origin==reversed.origin&&frame.z==reversed.z&&height==2.);
 let stock=cylinder([3.,2.5,0.],1.,10.);
 let slab=box_shape([0.,0.,3.],[6.,5.,2.]);
 assert_eq!(BooleanShape::Analytic(stock).combine(BooleanShape::Analytic(slab),BooleanOp::Cut).err().unwrap().0,"unsupported");
 for [base,height] in [[2.5,2.5],[-1.,2.5],[1.,1.75]]{
  let tool=cylinder([2.25,3.,base],0.75,height);
  assert!(matches!(BooleanShape::Analytic(a).combine(BooleanShape::Analytic(tool),BooleanOp::Cut),Ok(SolidBoundary::Bore(_))));
 }
 let through=cylinder([3.,2.5,-1.],1.,6.);
 assert!(matches!(BooleanShape::Analytic(a).combine(BooleanShape::Analytic(through),BooleanOp::Cut),Ok(SolidBoundary::Bore(_))));
 let crossing=cylinder([0.,2.,0.],1.,3.);
 let crossing_backwards=SolidPrimitive::Cylinder{
  frame:Frame3{origin:Pnt3::new(0.,2.,3.),y:-Vec3::Y,z:-Vec3::Z,..Frame3::WORLD},
  radius:1.,height:3.
 };
 for (first,second) in [(a,crossing),(crossing,a),(a,crossing_backwards)]{
  let result=BooleanShape::Analytic(first).combine(BooleanShape::Analytic(second),BooleanOp::Intersect).unwrap();
  let SolidBoundary::CircularPrism(profile)=result else{panic!("expected one certified circular prism");};
  assert_eq!(profile.count,2);
  assert!((profile.volume-1.5*PI).abs()<1e-12);
 }
 let tangent=cylinder([-1.,2.,0.],1.,3.);
 assert_eq!(BooleanShape::Analytic(a).combine(BooleanShape::Analytic(tangent),BooleanOp::Intersect).err().unwrap().0,"empty");
}
"""

lens_sampling=next(brep.rglob("📐️circular-prism-sampling/🦀️.rs")).read_text().split("\n",4)[4]
lens_sampling="mod lens_sampling{use super::{CircularPrism,Surface,SampleVertex as Vertex};"+lens_sampling+"}"
checks+=r'''
fn intersect_cylinders(a:SolidPrimitive,b:SolidPrimitive)->Result<SolidBoundary,ValueError>{
 BooleanShape::Analytic(a).combine(BooleanShape::Analytic(b),BooleanOp::Intersect)
}
fn circular_preview_measure(shape:CircularPrism,segments:usize)->(f64,f64){
 let mut area=0.;
 let mut volume=0.;
 for face in 0..shape.ring_sizes().len(){
  for slot in 0..lens_sampling::slots(shape,face,segments){
   let p=lens_sampling::triangle(shape,face,slot,segments);
   let [a,b,c]=p.each_ref().map(|v|v.point);
   let cross=(b-a).cross(c-a);
   assert!(cross.norm()>0.);
   area+=cross.norm()/2.;
   let outward=p[0].normal+p[1].normal+p[2].normal;
   volume+=a.dot(b.cross(c))*if cross.dot(outward)<0.{-1./6.}else{1./6.};
  }
 }
 (area,volume)
}
fn check_circular_prism(a:SolidPrimitive,b:SolidPrimitive,op:BooleanOp,mass:[f64;2],radii:[f64;2]){
 for (order,(first,second)) in [(a,b),(b,a)].into_iter().enumerate(){
  if matches!(op,BooleanOp::Cut)&&order>0{break;}
  let SolidBoundary::CircularPrism(shape)=BooleanShape::Analytic(first).combine(BooleanShape::Analytic(second),op).unwrap()else{panic!("expected exact circular prism");};
  assert_eq!(shape.ring_sizes(),&[2,2,4,4]);
  assert!((shape.volume-mass[0]).abs()<1e-11);
  assert!((shape.area()-mass[1]).abs()<1e-11);
  if matches!(op,BooleanOp::Unite){assert!(shape.arc_span(0)>PI||shape.arc_span(1)>PI);}
  assert!(shape.has_arc(0)&&shape.has_arc(1));
  assert_eq!((0..4).filter(|&face|shape.face_flipped(face)).count(),usize::from(matches!(op,BooleanOp::Cut)));
  let mut uses=[(0,0);6];
  for (face,&n) in shape.ring_sizes().iter().enumerate(){
   let surface=shape.surface(face);
   if let Surface::Cylinder{radius,..}=surface{assert!(radii.contains(&radius));}
   let mut prior=None;
   let mut first=None;
   for i in 0..n{
    let trim=shape.trim(face,i);
    if trim.forward^shape.face_flipped(face){uses[trim.edge].0+=1;}else{uses[trim.edge].1+=1;}
    let edge=shape.edge(trim.edge);
    let [start,end]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
    if let Some(expected)=prior{assert_eq!(start,expected);}
    prior=Some(end);
    first.get_or_insert(start);
    for part in [0.,0.125,0.5,0.875,1.]{
     let t=trim.range.0+(trim.range.1-trim.range.0)*part;
     let uv=trim.curve.eval(t);
     let on_face=match surface{
      Surface::Plane{frame}=>frame.origin+frame.x*uv.x+frame.y*uv.y,
      Surface::Cylinder{frame,radius}=>frame.origin+frame.z*uv.y+frame.x*(radius*uv.x.cos())+frame.y*(radius*uv.x.sin()),
      _=>unreachable!(),
     };
     assert!((on_face-edge.curve.eval(t)).norm()<1e-11);
    }
   }
   assert_eq!(prior,first);
  }
  assert_eq!(uses,[(1,1);6]);
  let angle=2.*(1.-0.025/shape.maximum_radius()).clamp(-1.,1.).acos();
  let native_segments=((shape.maximum_arc_span()/angle).ceil() as usize).max(8);
  let (coarse_area,_)=circular_preview_measure(shape,native_segments);
  assert!((coarse_area-mass[1]).abs()<0.24_f64.max(0.025*mass[1]));
  let (area,preview_volume)=circular_preview_measure(shape,256);
  assert!((area-mass[1]).abs()<0.005, "preview area {area} expected {}",mass[1]);
  assert!((preview_volume-mass[0]).abs()<0.005);
 }
}
#[test]fn parallel_booleans_keep_exact_caps_and_oriented_preview(){
'''
for row in json.loads(fixture.read_text())["cases"]:
    if not row["name"].startswith("parallel-cylinder ") or "retainedArcPrism" not in row:
        continue
    recipes=[row["inputs"][key] for key in ["a","b"]]
    operands=[mixed_operand(entry) for entry in recipes]
    radii=[entry["of"]["radius"] for entry in recipes]
    shape=row["outputs"]["shape"]
    operation={"brep.boolean.fuse":"Unite","brep.boolean.intersect":"Intersect","brep.boolean.cut":"Cut"}[row["kind"]]
    checks+=f'check_circular_prism({operands[0]},{operands[1]},BooleanOp::{operation},{array([shape["volume"],shape["area"]])},{array(radii)});\n'
checks+=r'''}
#[test]fn parallel_lens_preserves_opposite_axes_containment_and_empty_contacts(){
 let a=cylinder([0.,0.,0.],2.,4.);
 let inside=cylinder([0.5,0.25,1.],1.,6.);
 let SolidBoundary::Analytic(SolidPrimitive::Cylinder{frame,radius,height})=intersect_cylinders(a,inside).unwrap()else{panic!("expected clipped smaller cylinder");};
 assert!((frame.origin-pnt([0.5,0.25,1.])).norm()<1e-12&&radius==1.&&height==3.);
 let touching=cylinder([3.,0.,1.],1.,5.);
 assert_eq!(intersect_cylinders(a,touching).err().unwrap().0,"empty");
 let above=cylinder([1.,0.,4.],1.,1.);
 assert_eq!(intersect_cylinders(a,above).err().unwrap().0,"empty");
 let frame=Frame3::from_normal(pnt([1.,1.,0.]),Vec3::X).unwrap();
 let side=SolidPrimitive::Cylinder{frame,radius:1.,height:2.};
 assert_eq!(intersect_cylinders(a,side).err().unwrap().0,"unsupported");
 let forward=SolidPrimitive::Cylinder{frame:Frame3::from_normal(pnt([0.,0.,0.]),Vec3::X).unwrap(),radius:1.,height:4.};
 let reverse=SolidPrimitive::Cylinder{frame:Frame3::from_normal(pnt([3.,1.,0.]),-Vec3::X).unwrap(),radius:1.,height:2.};
 let area=2.*PI/3.-3_f64.sqrt()/2.;
 let perimeter=4.*PI/3.;
 check_circular_prism(forward,reverse,BooleanOp::Intersect,[area*2.,2.*(area+perimeter)],[1.,1.]);
 let union=|a,b|BooleanShape::Analytic(a).combine(BooleanShape::Analytic(b),BooleanOp::Unite);
 assert!(matches!(union(a,inside),Ok(SolidBoundary::Annular(shape)) if shape.has_step()));
 assert!(union(a,cylinder([2.,0.,1.],1.,4.)).is_err());
 let contained=cylinder([0.5,0.25,1.],1.,2.);
 assert!(matches!(union(a,contained),Ok(SolidBoundary::Analytic(SolidPrimitive::Cylinder{radius:2.,height:4.,..}))));
 let touching=cylinder([3.,0.,0.],1.,4.);
 assert_eq!(union(a,touching).err().unwrap().0,"unsupported");
 let opposite=SolidPrimitive::Cylinder{frame:Frame3::from_normal(pnt([4.,1.,0.]),-Vec3::X).unwrap(),radius:1.,height:4.};
 let area=2.*PI-area;
 let perimeter=4.*PI-perimeter;
 check_circular_prism(forward,opposite,BooleanOp::Unite,[area*4.,2.*area+4.*perimeter],[1.,1.]);
 let notch=SolidPrimitive::Cylinder{frame:Frame3::from_normal(pnt([5.,1.25,0.]),-Vec3::X).unwrap(),radius:1.,height:6.};
 let chord=0.625_f64;
 let half=(1.-chord*chord).sqrt();
 let angle=half.atan2(chord);
 let area=PI-2.*(angle-chord*half);
 check_circular_prism(forward,notch,BooleanOp::Cut,[area*4.,2.*area+8.*PI],[1.,1.]);
 let subtract=|a,b|BooleanShape::Analytic(a).combine(BooleanShape::Analytic(b),BooleanOp::Cut);
 assert!(matches!(subtract(a,touching),Ok(SolidBoundary::Analytic(SolidPrimitive::Cylinder{radius:2.,height:4.,..}))));
 assert!(matches!(subtract(a,inside),Ok(SolidBoundary::Annular(_))));
 let off_axis_wide=cylinder([0.5,0.,-1.],4.,3.);
 let SolidBoundary::Analytic(SolidPrimitive::Cylinder{frame,height,..})=subtract(a,off_axis_wide).unwrap()else{panic!("expected trimmed stock");};
 assert!(frame.origin.z==2.&&height==2.);
}
'''

checks+=r'''
fn check_sphere_clip(a:SolidPrimitive,b:SolidPrimitive,op:BooleanOp,measure:[f64;2],centroid:EVec3,disks:&[(EVec3,EVec3,f64)]){
 let SolidBoundary::SphericalCaps(s)=BooleanShape::Analytic(a).combine(BooleanShape::Analytic(b),op).unwrap()
 else{panic!("expected spherical cap and original plane");};
 assert!((s.volume()-measure[0]).abs()<1e-12&&(s.area()-measure[1]).abs()<1e-12);
 assert!((s.center_of_mass()-pnt(centroid)).norm()<1e-12);
 assert_eq!(s.ring_sizes(),if disks.len()==1{&[4,1][..]}else{&[4,1,1][..]});
 assert_eq!((s.vertex_count(),s.edge_count()),(2,3));
 for (i,&(disk,normal,expected_radius)) in disks.iter().enumerate(){
  let Surface::Plane{frame:plane}=spherical_boundary::surface(s,i+1)else{unreachable!();};
  assert!((plane.origin-pnt(disk)).norm()<1e-12&&(plane.z-pnt(normal)).norm()<1e-12);
  let (r3,range,_)=s.edge(i*2);
  let Curve3::Circle{radius,..}=r3 else{unreachable!();};
  assert_eq!(range,(0.,TAU));
  assert!((radius-expected_radius).abs()<1e-12);
  for j in 0..64 {
   let u=TAU*f64::from(j)/64.;
   let point=s.rim_point_at(i==1,u);
   assert!((point-r3.eval(u)).norm()<1e-12);
   assert!((point-plane.origin).dot(plane.z).abs()<1e-12);
  }
 }
 let mut uses=[(0,0);3];
 for (face,&n) in s.ring_sizes().iter().enumerate(){
  let support=spherical_boundary::surface(s,face);
  let mut first=None;
  let mut prior=None;
  for i in 0..n{
   let trim=spherical_boundary::trim(s,face,i);
   let (curve,range,[u,v])=s.edge(trim.edge);
   let [a,b]=if trim.forward{uses[trim.edge].0+=1;[u,v]}else{uses[trim.edge].1+=1;[v,u]};
   if let Some(end)=prior{assert_eq!(a,end);}
   first.get_or_insert(a);
   prior=Some(b);
   assert_eq!(range,trim.range);
   for part in [0.,0.125,0.5,0.875,1.]{
    let t=range.0+(range.1-range.0)*part;
    let uv=trim.curve.eval(t);
    let point=match support {
     Surface::Plane{frame}=>frame.origin+frame.x*uv.x+frame.y*uv.y,
     Surface::Sphere{frame,radius}=>frame.origin+(frame.x*(uv.x.cos()*uv.y.cos())+frame.y*(uv.x.sin()*uv.y.cos())+frame.z*uv.y.sin())*radius,
     _=>unreachable!(),
    };
    assert!((point-curve.eval(t)).norm()<1e-11);
   }
  }
  assert_eq!(first,prior);
 }
 assert_eq!((uses[0],uses[1]),((1,1),(1,1)));
 if disks.len()==2{assert_eq!(uses[2],(1,1));}
}
#[test]fn sphere_original_recipe_keeps_exact_spherical_and_planar_trims(){
'''
for row in json.loads(fixture.read_text())["cases"]:
    if "sphericalDisk" in row:
        disks=[row["sphericalDisk"]]
        centroid=row["sphericalDisk"]["centroid"]
    elif "sphericalSlab" in row:
        disks=row["sphericalSlab"]["disks"]
        centroid=row["sphericalSlab"]["centroid"]
    else:
        continue
    choices=[]
    for side in ("a","b"):
        item=row["inputs"][side]
        recipe=item["of"]
        if recipe["recipe"]=="sphere":
            choices.append(f'sphere_stock({array(item["by"])},{float(recipe["radius"])})')
        elif recipe["recipe"]=="cylinder":
            choices.append(f'cylinder({array(item["by"])},{float(recipe["radius"])},{float(recipe["height"])})')
        else:
            choices.append(f'box_shape({array(item["by"])},{array(recipe["size"])})')
    output=row["outputs"]["shape"]
    op="Cut" if row["kind"].endswith(".cut") else "Intersect"
    measure=array([output["volume"],output["area"]])
    expected=",".join(f'({array(disk["center"])},{array(disk["normal"])},{disk["radius"]})' for disk in disks)
    checks+=f'check_sphere_clip({choices[0]},{choices[1]},BooleanOp::{op},{measure},{array(centroid)},&[{expected}]);\n'
checks+=r'''
 let sphere=sphere_stock([0.,0.,0.],2.);
 let contact=box_shape([2.,-3.,-3.],[2.,6.,6.]);
 let wraps=box_shape([-3.,-3.,-3.],[6.,6.,6.]);
 let two=box_shape([0.2,0.3,-3.],[4.,5.,6.]);
 let combine=|left,right,op|BooleanShape::Analytic(left).combine(BooleanShape::Analytic(right),op);
 assert_eq!(combine(sphere,contact,BooleanOp::Intersect).err().unwrap().0,"empty");
 assert_eq!(combine(sphere,wraps,BooleanOp::Cut).err().unwrap().0,"empty");
 assert_eq!(combine(sphere,two,BooleanOp::Intersect).err().unwrap().0,"unsupported");
 let slab=box_shape([-3.,-1.,-3.],[6.,2.,6.]);
 assert_eq!(combine(sphere,slab,BooleanOp::Cut).err().unwrap().0,"unsupported");
 assert!(matches!(combine(sphere,contact,BooleanOp::Cut),Ok(SolidBoundary::Analytic(_))));
 assert!(matches!(combine(wraps,sphere,BooleanOp::Intersect),Ok(SolidBoundary::Analytic(SolidPrimitive::Sphere{..}))));
}
'''
checks+=r'''
#[test]fn sphere_cylinder_original_recipe_checks_relative_axes_and_refuses_side_contacts(){
 let sphere=sphere_stock([2.,-4.,6.],2.);
 let apply=|left,right,op|BooleanShape::Analytic(left).combine(BooleanShape::Analytic(right),op);
 let horizontal=SolidPrimitive::Cylinder{frame:Frame3::from_normal(pnt([3.,-3.75,6.]),-Vec3::X).unwrap(),radius:2.5,height:2.};
 check_sphere_clip(horizontal,sphere,BooleanOp::Intersect,[23.038346126325145,43.9822971502571],[2.,-4.,6.],
    &[([3.,-4.,6.],[1.,0.,0.],3_f64.sqrt()),([1.,-4.,6.],[-1.,0.,0.],3_f64.sqrt())]);
 let axis=[0.6,0.,0.8];
 let pitched=SolidPrimitive::Cylinder{frame:Frame3::from_normal(pnt([1.4,-3.75,5.2]),pnt(axis)).unwrap(),radius:2.5,height:2.};
 check_sphere_clip(sphere,pitched,BooleanOp::Intersect,[23.038346126325145,43.9822971502571],[2.,-4.,6.],
    &[([1.4,-4.,5.2],[-0.6,0.,-0.8],3_f64.sqrt()),([2.6,-4.,6.8],axis,3_f64.sqrt())]);
 let tangent=cylinder([5.,-4.,4.],1.,4.);
 assert_eq!(apply(tangent,sphere,BooleanOp::Intersect).err().unwrap().0,"empty");
 let remote=cylinder([10.,-4.,4.],1.,4.);
 assert_eq!(apply(remote,sphere,BooleanOp::Intersect).err().unwrap().0,"empty");
 assert!(matches!(apply(remote,sphere,BooleanOp::Cut),Ok(SolidBoundary::Analytic(SolidPrimitive::Cylinder{..}))));
 let narrow=cylinder([2.,-4.,5.],0.5,2.);
 assert!(matches!(apply(narrow,sphere,BooleanOp::Intersect),Ok(SolidBoundary::Analytic(SolidPrimitive::Cylinder{..}))));
 assert!(matches!(apply(narrow,sphere,BooleanOp::Unite),Ok(SolidBoundary::Analytic(SolidPrimitive::Sphere{..}))));
 let wide=cylinder([2.,-4.,3.],3.,6.);
 assert!(matches!(apply(wide,sphere,BooleanOp::Intersect),Ok(SolidBoundary::Analytic(SolidPrimitive::Sphere{..}))));
 assert_eq!(apply(sphere,wide,BooleanOp::Cut).err().unwrap().0,"empty");
 let side=cylinder([2.,-4.,4.],1.,4.);
 assert_eq!(apply(sphere,side,BooleanOp::Intersect).err().unwrap().0,"unsupported");
 let slab=cylinder([2.,-4.,5.],2.5,2.);
 assert_eq!(apply(sphere,slab,BooleanOp::Cut).err().unwrap().0,"unsupported");
}
'''
sector_fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").rglob("🏗️brep-solid/🔣️.json"))
checks+=r'''
fn exact_sector_metrics(origin:EVec3,axis:EVec3,radial:EVec3,sweep:f64,dims:EVec3,expected:[f64;2],centroid:EVec3){
 let mut origin=pnt(origin);
 let original=pnt(axis);
 let x=pnt(radial);
 let z=original*sweep.signum();
 if sweep<0.{origin=origin+original*dims[2];}
 let shape=AnnularRevolution::new(Frame3{origin,x,y:z.cross(x),z},dims[0],dims[1],dims[2],sweep.abs(),Tol::DEFAULT.value()).unwrap();
 assert!((shape.volume()-expected[0]).abs()<1e-10);
 assert!((shape.area()-expected[1]).abs()<1e-10);
 assert!((shape.center_of_mass()-pnt(centroid)).norm()<1e-10);
}
#[test]fn annular_sector_metrics_match_closed_ring_and_signed_world_fixtures(){
'''
for row in json.loads(sector_fixture.read_text())["retainedMeridionalRevolve"]:
    frame=",".join(array(row[key]) for key in ["origin","axis","radial"])
    dims=array([row["inner"],row["outer"],row["height"]])
    measures=array([row["volume"],row["area"]])
    checks+=f'exact_sector_metrics({frame},{row["angle"]},{dims},{measures},{array(row["centroid"])});\n'
checks+="}\n"
parts=[source,oriented,prism,sphere,caps,annular,spherical_boundary,lens_sampling,"mod annular_boundary{use super::*;use primitives::planar_pcurve;"+boundary+"}","mod annular_sampling{use super::{AnnularRevolution,SampleVertex as Vertex};"+sampling+"}",coaxial,boolean,shell_methods,checks]
if __name__=="__main__":
    generated=ticket/"🗑️generated"
    generated.mkdir(exist_ok=True)
    with TemporaryDirectory(dir=generated) as folder:
        src=Path(folder)/"bore.rs"
        src.write_text("\n".join(parts))
        exe=src.with_suffix("")
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(src),"-o",str(exe)],check=True)
        subprocess.run([str(exe)],check=True)
