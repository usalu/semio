"""Check exact clipped rectangular bores and the production axial preview charts."""
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
source=output.getvalue().replace("enum Surface{Plane{frame:Frame3},Other}","enum Surface{Plane{frame:Frame3},Cylinder{frame:Frame3,radius:f64},Other}")
primitive=next(brep.rglob("🧊️analytic-solids/🦀️.rs")).read_text()
helpers=primitive[primitive.index("const TURN:"):primitive.index("impl SolidPrimitive")]
for name,ty,params in [("vertex","Pnt3","index"),("edge","PrimitiveEdge","edge"),("surface","Surface","face"),("trim","PrimitiveTrim","face,member")]:
    body=primitive[primitive.index("pub fn "+name+"("):]
    begin=body.index("            Self::Cylinder")
    end=body.index("            Self::Cone",begin)
    arm=body[begin:end]
    stub=f" fn {name}(&self,"+",".join("_:usize" for _ in params.split(","))+f")->"+ty+" {unreachable!()}"
    scope="let linear=|a,b,range|line([a,b],self.vertex(a),self.vertex(b),range);" if name=="edge" else ""
    extra=""
    if name=="surface":
        extra="Self::Box{origin,height,..}=>{let frame=Frame3{origin,..Frame3::WORLD};Surface::Plane{frame:if face==0{reversed_frame(frame)}else{frame_at(frame,height)}}},"
    typed=",".join(p+":usize" for p in params.split(","))
    assert source.count(stub)==1
    source=source.replace(stub,f" fn {name}(&self,{typed})->{ty} {{{scope}match *self{{{arm}{extra}_=>unreachable!()}}}}")

transaction=next(brep.rglob("🏗️solid-transaction/🦀️.rs")).read_text()
methods=[]
for name,next_name in [("face_count","ring_owner"),("ring_owner","inner_ring"),("inner_ring","face_flipped"),("face_flipped","shell_count"),("shell_count","shell_face_count"),("shell_face_count","vertex")]:
    text=transaction[transaction.index("    fn "+name+"("):transaction.index("    fn "+next_name+"(")]
    text="\n".join(row for row in text.splitlines() if "Self::Spherical" not in row)
    if name=="face_count":
        text=text.replace("if let Self::Annular(shape)=self {shape.face_count()} else ","")
    elif name=="ring_owner":
        text=text.replace("if let Self::Annular(shape)=self {annular_boundary::face_index(*shape,ring)}\n        else ","")
    else:
        text="\n".join(row for row in text.splitlines() if "Self::Annular" not in row)
    methods.append(text)
lens_preview=next(brep.rglob("🕳️bore-sampling/🦀️.rs")).read_text()
lens_preview=lens_preview[lens_preview.index("const SECTIONS:"):lens_preview.index("#[cfg(test)]")]
preview=(brep/"⚙️engine/📦️mesh-io/🖼️primitive-preview/🦀️.rs").read_text()
begin=preview.index("fn segment_count(")
end=preview.index("\n}",begin)+2
segment_count=preview[begin:end]
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").rglob("🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json"))
checks=r"""
use std::f64::consts::{TAU,FRAC_PI_2};
impl Frame3{fn to_world(&self,p:Pnt3)->Pnt3{self.origin+self.x*p.x+self.y*p.y+self.z*p.z}}
impl Surface{fn eval(&self,u:f64,v:f64)->Pnt3{match *self{
 Self::Cylinder{frame,radius}=>frame.to_world(Pnt3::new(radius*u.cos(),radius*u.sin(),v)),
 Self::Plane{frame}=>frame.to_world(Pnt3::new(u,v,0.)),
 _=>unreachable!()
}}}
const MAX_SEGMENTS:usize=96;
fn invalid_preview()->ValueError{ValueError("invalid bore preview")}
struct SampleVertex{point:Pnt3,normal:Vec3}
fn area([a,b,c]:[SampleVertex;3])->f64{
 let cross=(b.point-a.point).cross(c.point-a.point);
 assert!(cross.dot(a.normal+b.normal+c.normal)>0.);
 cross.norm()/2.
}
fn check(size:EVec3,center:EVec3,radius:f64,height:f64,span:[f64;2],volume:f64,expected_area:f64){
 let [width,depth,h]=size;
 let box_=SolidPrimitive::Box{origin:Vec3::ZERO,width,depth,height:h};
 let cylinder=SolidPrimitive::Cylinder{frame:Frame3{origin:pnt(center),..Frame3::WORLD},radius,height};
 let b=BoreBoundary::new(box_,cylinder).unwrap();
 let reversed=SolidPrimitive::Cylinder{frame:Frame3{origin:pnt(center)+Vec3::Z*height,y:-Vec3::Y,z:-Vec3::Z,..Frame3::WORLD},radius,height};
 assert_eq!(BoreBoundary::new(box_,reversed).unwrap().preview_source(1).bore_span,span);
 let source=b.preview_source(1);
 let boundary=SolidBoundary::Bore(b);
 let SolidBoundary::Bore(ref bore)=boundary else{unreachable!();};
 assert_eq!(source.bore_span,span);
 assert!((source.volume-volume).abs()<1e-10);
 let floors=[!source.is_open(false),!source.is_open(true)];
 assert_eq!(boundary.face_count(),7+floors.into_iter().filter(|&floor|floor).count());
 if source.closed_bore(){
  assert_eq!(boundary.shell_count(),2);
  assert_eq!(boundary.shell_face_count(0),6);
  assert_eq!(boundary.shell_face_count(1),3);
 }else{assert_eq!(boundary.shell_count(),1);}
 let mut signs=[(false,false);2];
 for ring in [6,7,8]{
  let face=boundary.ring_owner(ring);
  for member in 0..if ring==6{4}else{1}{
   let trim=bore.trim(ring,member);
   let edge=bore.edge(trim.edge);
   for part in [0.,0.23,0.75,1.]{
    let t=trim.range.0+(trim.range.1-trim.range.0)*part;
    let uv=trim.curve.eval(t);
    assert!((bore.surface(face).eval(uv.x,uv.y)-edge.curve.eval(t)).norm()<1e-12);
   }
   if trim.edge==12||trim.edge==13{
    let role=trim.edge-12;
    if ring==6{signs[role].0=trim.forward!=boundary.face_flipped(face);}
    else{signs[role].1=trim.forward!=boundary.face_flipped(face);}
   }
  }
 }
 assert!(signs.into_iter().all(|(first,last)|first!=last));
 assert!(source.signed_boundary_distance([center[0],center[1],(span[0]+span[1])/2.])>0.);
 let sampler=bore_sampling::BoreSampling::new(source,0.002).unwrap();
 let mut surface=2.*(width+depth)*h;
 for top in [false,true]{
  let rim=span[usize::from(top)];
  if source.is_open(top){
   for slot in 0..2*sampler.segments(){surface+=area(sampler.cap_triangle(top,slot));}
  }else{
   surface+=width*depth;
   assert!(source.signed_boundary_distance([center[0],center[1],rim]).abs()<1e-12);
   let q=if top{(h+rim)/2.}else{rim/2.};
   assert!(source.signed_boundary_distance([center[0],center[1],q])<0.);
   for slot in 0..sampler.segments(){
    let tri=sampler.floor_triangle(top,slot);
    assert!(tri.iter().all(|v|v.point.z==rim&&v.normal.z==if top{-1.}else{1.}));
    assert!(tri.iter().any(|v|v.point==sampler.rim_point(top,slot)));
    surface+=area(tri);
   }
  }
 }
 for slot in 0..2*sampler.segments(){surface+=area(sampler.wall_triangle(slot));}
 assert!((surface-expected_area).abs()<0.02);
}
#[test]fn closed_and_blind_box_bores_keep_exact_uvs_and_paid_charts(){
"""
def array(xs):return "["+",".join(str(float(x)) for x in xs)+"]"
for row in json.loads(fixture.read_text())["cases"]:
    if not row["name"].startswith("box bore "):continue
    a,b=[row["inputs"][key] for key in ["a","b"]]
    args=[array(a["size"]),array(b["by"]),str(float(b["of"]["radius"])),str(float(b["of"]["height"])),array(row["boreSpan"]),
          str(float(row["outputs"]["shape"]["volume"])),str(float(row["outputs"]["shape"]["area"]))]
    checks+="check("+",".join(args)+");\n"
checks+="check([6.,5.,4.],[2.25,3.,-1.],0.75,6.,[0.,4.],120.-std::f64::consts::PI*2.25,148.+std::f64::consts::PI*4.875);}"
parts=[source,helpers,"impl SolidBoundary{fn ring_sizes(&self)->&[usize]{&[4,4,4,4,4,4,4,1,1]}"+''.join(methods)+"}",
      "mod bore_sampling{use super::*;use super::SampleVertex as Vertex;"+lens_preview+"}",segment_count,checks]
with TemporaryDirectory(dir=ticket/"🗑️generated") as folder:
    src=Path(folder)/"bore.rs"
    src.write_text("\n".join(parts))
    exe=src.with_suffix("")
    subprocess.run(["rustup","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe)],check=True)
