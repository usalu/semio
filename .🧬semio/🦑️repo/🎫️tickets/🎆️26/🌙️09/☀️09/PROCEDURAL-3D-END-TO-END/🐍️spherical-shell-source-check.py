"""Exercise the certified sphere wall, its real trim topology and production sampling."""
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
source=output.getvalue().replace("enum SolidPrimitive{Box","enum SolidPrimitive{Sphere{frame:Frame3,radius:f64},Box")
source=source.replace("fn volume(&self)->f64 {match *self{Self::Box","fn volume(&self)->f64 {match *self{Self::Sphere{radius,..}=>4.*std::f64::consts::PI*radius.powi(3)/3.,Self::Box")
source=source.replace("enum Surface{Plane{frame:Frame3},Other}","enum Surface{Plane{frame:Frame3},Sphere{frame:Frame3,radius:f64},Cone{frame:Frame3,half_angle:f64},Cylinder{frame:Frame3},Torus{frame:Frame3},Other}")
source=source.replace("enum SolidBoundary{Bore(BoreBoundary),Orthogonal(OrthogonalBoundary)}",
    "enum SolidBoundary{Bore(BoreBoundary),Orthogonal(OrthogonalBoundary),SphericalShell(SphericalShell),SphericalCaps(SphericalCaps)}")
sphere=(brep/"📸️representation/🔖️contract/🔵️spherical-shell/🦀️.rs").read_text().split("\n",4)[4]
caps=next(brep.rglob("🌗️spherical-caps/🦀️.rs")).read_text()
caps=caps[caps.index("/// 🔗️"):]
measure=next(brep.rglob("🌗️spherical-caps/📐️measure/🦀️.rs")).read_text().split("\n",2)[2]
caps="mod measure{"+measure+"}"+caps
cap_boundary=next(brep.rglob("🌗️spherical-cap-boundary/🦀️.rs")).read_text()
cap_boundary=cap_boundary[cap_boundary.index("pub(super) fn trim"):]
primitive=next(brep.rglob("🧊️analytic-solids/🦀️.rs")).read_text()
helpers=primitive[primitive.index("const TURN:"):primitive.index("fn cap(")]
variants=[
    ("vertex", "i", "Pnt3", "index","pub fn vertex(", "Self::Cylinder"),
    ("edge", "i", "PrimitiveEdge", "edge","pub fn edge(", "Self::Cylinder"),
    ("surface", "i", "Surface", "face","pub fn surface(", "Self::Cylinder"),
    ("trim", "i,j", "PrimitiveTrim", "face,member","pub fn trim(", "Self::Cylinder"),
]
for name,fake_params,ty,params,method,stop in variants:
    base=primitive[primitive.index(method):]
    begin=base.index("            Self::Sphere")
    end=base.index("            "+stop,begin)
    arm=base[begin:end]
    fake=f" fn {name}(&self,"+",".join("_:usize" for p in fake_params.split(","))+f")->"+ty+" {unreachable!()}"
    params=",".join(p+":usize" for p in params.split(","))
    assert source.count(fake)==1,fake
    scopes="let linear = |a, b, range| line([a, b], self.vertex(a), self.vertex(b), range);" if name=="edge" else ""
    source=source.replace(fake,f" fn {name}(&self,{params})->{ty} {{{scopes}match *self{{{arm}_=>unreachable!()}}}}")

bounds=next(brep.rglob("🧮️boolean-recipe/🦀️.rs")).read_text()
bounds=bounds[bounds.index("#[derive(Clone, Copy)]"):bounds.index("fn exact_box_boolean")]
shell=next(brep.rglob("🐚️shell-recipe/🦀️.rs")).read_text()
shell=shell[shell.index("fn invalid("):]
shell=shell.replace("ValueError::literal(ValueRefusalKind::InvalidValue,message)","ValueError(message)")
shell=shell.replace("ValueError::literal(ValueRefusalKind::UnsupportedOwner,","ValueError(")
transaction=next(brep.rglob("🏗️solid-transaction/🦀️.rs")).read_text()
h=transaction.index("    fn shell_count(")
shell_count=transaction[h:transaction.index("    fn vertex(",h)]
shell_count="\n".join(row for row in shell_count.splitlines() if "Self::Annular" not in row)
boundary="impl SolidBoundary{fn face_count(&self)->usize{2}"
boundary+=shell_count+"\nfn face_flipped(&self,face:usize)->bool{matches!(self,Self::SphericalShell(_))&&face==1||matches!(self,Self::SphericalCaps(shape) if shape.is_inward(face))}\n"
for name,_,ty,_,_,_ in variants:
    start=transaction.index(f"    fn {name}(&self, i: usize")
    match=transaction.index("Self::SphericalShell(p)=>",start)
    end=transaction.index(", Self::Hull",match)
    arm=transaction[match:end]
    params="i:usize,member:usize" if name=="trim" else "i:usize"
    boundary+=f"fn {name}(&self,{params})->{ty}{{match self{{{arm},_=>unreachable!()}}}}"
boundary+="}"

preview=(brep/"⚙️engine/📦️mesh-io/🖼️primitive-preview/🦀️.rs").read_text()
sphere_recipe=next(l.strip() for l in preview.splitlines() if l.strip().startswith("let sphere=|inside|"))
preview=preview[preview.index("    fn triangle(&self)"):preview.index("    fn reference_count")]
start=preview.index("=> {",preview.index("Chart::Wrapped { first"))
start=preview.index("{",start)
depth=1
for end in range(start+1,len(preview)):
    depth+=(preview[end]=="{")-(preview[end]=="}")
    if depth==0:break
sample=preview[start:end+1]
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").rglob("🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json"))

checks=r"""
use std::f64::consts::{FRAC_PI_2,PI,TAU};
use shell_recipe::ShellStock;
impl Pnt3{fn from_array(a:EVec3)->Self{pnt(a)}}
impl Frame3{fn to_world_vector(&self,v:Vec3)->Vec3{self.x*v.x+self.y*v.y+self.z*v.z}}
impl Surface{
 fn eval(&self,u:f64,v:f64)->Pnt3{
  let Self::Sphere{frame,radius}=*self else{unreachable!();};
  frame.origin+frame.to_world_vector(Pnt3::new(u.cos()*v.cos(),u.sin()*v.cos(),v.sin()))*radius
 }
}
struct SampleVertex{point:Pnt3,normal:Vec3}
struct Sample{slot:usize,segments:usize}
impl Sample{
 fn sphere(&self,surface:Surface,direction:f64,params:Option<(SphericalCaps,usize)>)->Option<[SampleVertex;3]>{
 let (first,end,low_pole,high_pole)=params.map_or((-FRAC_PI_2,FRAC_PI_2,true,true),|(caps,face)|caps.band(face));
 let caps=params.map(|(shape,_)|shape);
 let rows=self.segments.div_ceil(2);
 let angle=|column:usize|TAU*column as f64/self.segments as f64;
"""
sample=sample.replace("Vertex","SampleVertex")
checks+=sample+"}}\n"
checks+="fn preview_sphere(shape:SphericalShell,inside:bool)->Surface{"+sphere_recipe+"sphere(inside).surface(0)}\n"
checks+=r"""
fn check(center:EVec3,inner_center:Option<EVec3>,radius:f64,thickness:f64,expected:[f64;2],refused:[f64;4]){
 let frame=Frame3::from_normal(pnt(center),Vec3::Z).unwrap();
 let input=SolidPrimitive::Sphere{frame,radius};
 for invalid in refused{assert!(ShellStock::new(input,invalid).is_err());}
 let draft=ShellStock::new(input,thickness).unwrap();
 assert!(draft.finish(1).is_err());
 let boundary=if let Some(inner_center)=inner_center{
  let inward=Frame3::from_normal(pnt(inner_center),Vec3::X).unwrap();
  SolidBoundary::SphericalShell(SphericalShell::from_spheres([frame,inward],[radius,radius-thickness],Tol::DEFAULT.value()).unwrap())
 }else{draft.finish(0).unwrap()};
 assert_eq!(boundary.shell_count(),2);
 assert_eq!(boundary.shell_face_count(0),1);
 assert_eq!(boundary.shell_face_count(1),1);
 let SolidBoundary::SphericalShell(shell)=boundary else{panic!("expected a spherical wall");};
 assert!((shell.volume()-expected[0]).abs()<1e-10);
 let mut volume=[0.,0.];
 let mut area=0.;
 for which in 0..2{
  let surface=boundary.surface(which);
  let direction=if boundary.face_flipped(which){-1.}else{1.};
  assert_eq!(direction,if which==0{1.}else{-1.});
  let first=which*2;
  assert!(boundary.vertex(first)!=boundary.vertex(first+1));
  let mut seam=Vec::new();
  for member in 0..4{
   let trim=boundary.trim(which,member);
   let edge=boundary.edge(trim.edge);
   assert!(edge.vertices.into_iter().all(|id|(first..first+2).contains(&id)));
   if trim.edge%3==0{seam.push(trim.forward);}
   for i in 0..5{
    let t=trim.range.0+(trim.range.1-trim.range.0)*i as f64/4.;
    let uv=trim.curve.eval(t);
    assert!((edge.curve.eval(t)-surface.eval(uv.x,uv.y)).norm()<1e-12);
   }
  }
  assert_eq!(seam,[true,false]);
  let resolution=256usize;
  for slot in 0..(2*resolution*resolution.div_ceil(2)){
   if let Some(samples)=(Sample{slot,segments:resolution}).sphere(preview_sphere(shell,which==1),direction,None){
    let support_center=if which==1{inner_center.unwrap_or(center)}else{center};
    assert!(samples.iter().all(|s|((s.point-pnt(support_center)).norm()-shell.radius(which==1)).abs()<1e-12));
    let [a,b,c]=samples.each_ref().map(|sample|sample.point-frame.origin);
    let dot=(b-a).cross(c-a).dot(samples[0].normal+samples[1].normal+samples[2].normal);
    volume[which]+=a.dot(b.cross(c))*dot.signum()/6.;
    area+=(b-a).cross(c-a).norm()/2.;
   }
  }
 }
 assert!(volume[0]>0.&&volume[1]<0.);
 assert!((volume[0]+volume[1]-expected[0]).abs()<0.05);
 assert!((area-expected[1]).abs()<0.05);
}
#[test] fn shell_owns_two_signed_spheres_and_exact_original_seams(){
"""
def array(xs):return "["+",".join(str(float(x)) for x in xs)+"]"
for row in json.loads(fixture.read_text())["retainedSphericalShell"]:
    checks+=f'check({array(row["center"])},None,{float(row["radius"])},{float(row["thickness"])},{array([row["volume"],row["area"]])},{array(row["refusedThicknesses"])});\n'
checks+="}\n"
checks+=r"""
fn caps_case(centers:[EVec3;2],radii:[f64;2],positive:[bool;2],inward:bool,expected:[f64;2],rim:[f64;4]){
 let shape=SphericalCaps::new(pnt(centers[0]),pnt(centers[1]),radii,positive,inward,1e-7).unwrap();
 let boundary=SolidBoundary::SphericalCaps(shape);
 assert_eq!(boundary.shell_count(),1);
 assert_eq!(boundary.shell_face_count(0),2);
 assert!((shape.volume()-expected[0]).abs()<1e-10);
 let center=shape.rim_frame().origin;
 assert!((center-pnt([rim[0],rim[1],rim[2]])).norm()<1e-12);
 let (circle,turn,vertices)=shape.edge(0);
 assert_eq!(vertices,[0,0]);assert_eq!(turn,(0.,TAU));
 assert!(((circle.eval(0.)-center).norm()-rim[3]).abs()<1e-12);
 let mut directions=Vec::new();
 let mut volume=0.;
 let mut area=0.;
 for face in 0..2{
  let flipped=boundary.face_flipped(face);
  let direction=if flipped{-1.}else{1.};
  assert_eq!(flipped,face==1&&inward);
  for member in 0..4{
   let trim=boundary.trim(face,member);
   let edge=boundary.edge(trim.edge);
   assert_eq!(trim.range,edge.range);
   if trim.edge==0{directions.push(trim.forward!=flipped);}
   for i in 0..5{
    let t=trim.range.0+(trim.range.1-trim.range.0)*i as f64/4.;
    let uv=trim.curve.eval(t);
    assert!((edge.curve.eval(t)-boundary.surface(face).eval(uv.x,uv.y)).norm()<1e-12);
   }
  }
  let segments=256usize;
  for slot in 0..2*segments*segments.div_ceil(2){
   if let Some(samples)=(Sample{slot,segments}).sphere(boundary.surface(face),direction,Some((shape,face))){
    let [a,b,c]=samples.each_ref().map(|p|p.point-center);
    for sample in &samples{
     let rel=sample.point-pnt(centers[face]);
     assert!((rel.norm()-radii[face]).abs()<1e-12);
     assert!(rel.dot(sample.normal)*direction>0.);
    }
    let cross=(b-a).cross(c-a);
    let dot=cross.dot(samples[0].normal+samples[1].normal+samples[2].normal);
    volume+=a.dot(b.cross(c))*dot.signum()/6.;
    area+=cross.norm()/2.;
   }
  }
 }
 assert_eq!(directions.len(),2);
 assert_ne!(directions[0],directions[1]);
 assert!((volume-expected[0]).abs()<0.055);
 assert!((area-expected[1]).abs()<0.055);
}
#[test]fn exact_spherical_caps_share_signed_rim_and_sample_correctly(){
"""

boolean_fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").rglob("🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json"))
cases=json.loads(boolean_fixture.read_text())["cases"]
for row in cases:
    if not row["name"].startswith("overlapping sphere"):continue
    inputs=[row["inputs"][side] for side in ["a","b"]]
    centers="["+",".join(array(entry["by"]) for entry in inputs)+"]"
    radii=array([entry["of"]["radius"] for entry in inputs])
    mode=row["kind"].split(".")[-1]
    roles={"fuse":"[false,true]","intersect":"[true,false]","cut":"[false,false]"}[mode]
    values=array([row["outputs"]["shape"][key] for key in ["volume","area"]])
    checks+=f'caps_case({centers},{radii},{roles},{str(mode=="cut").lower()},{values},{array(row["rim"])});\n'
checks+="}\n"
row=next(item for item in cases if item["name"]=="eccentric sphere cut preserves the tool placement")
outside,inside=(row["inputs"][side] for side in ["a","b"])
outer,inner=(entry["of"]["radius"] for entry in [outside,inside])
expected=array([row["outputs"]["shape"][key] for key in ["volume","area"]])
checks+=f'#[test] fn eccentric_shell_samples_both_authored_frames(){{check({array(outside["by"])},Some({array(inside["by"])}),{outer},{outer-inner},{expected},{array([-.25,0.,outer,outer+1.])});}}\n'
with TemporaryDirectory(dir=ticket/"🗑️generated") as folder:
    src=Path(folder)/"sphere.rs"
    src.write_text("\n".join([source,sphere,caps,helpers,bounds,"mod shell_recipe{use super::*;"+shell+"}","mod spherical_cap_boundary{use super::*;"+cap_boundary+"}",boundary,checks]))
    exe=src.with_suffix("")
    subprocess.run(["rustup","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe)],check=True)
