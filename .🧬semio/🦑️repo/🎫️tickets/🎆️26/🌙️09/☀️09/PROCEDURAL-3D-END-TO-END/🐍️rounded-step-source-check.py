"""🟠️ Verify the actual retained rounded STEP writer and decoder against an independent reader."""
from contextlib import redirect_stdout
from io import StringIO
import base64
import hashlib
import io
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys
import tarfile
from tempfile import TemporaryDirectory
import urllib.request

ticket = Path(__file__).parent
kernel = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
step = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry")
fixture = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️original-box-all-edge-fillet.json").resolve()
output = StringIO()
with redirect_stdout(output):
    runpy.run_path(str(ticket/"🐍️point-face-source-check.py"))
original = output.getvalue()
core = original[original.index("type EVec3="):original.index("type EdgeId=usize;")]
curves = original[original.index("enum Curve3{"):original.index("enum Surface{")]
source = """
extern crate self as semio_framework_3d_brep_curve;
extern crate self as semio_framework_3d_brep_representation;
extern crate self as semio_framework_3d_brep_construction;
extern crate self as semio_framework_3d_brep_tolerance;
extern crate self as semio_framework_3d_brep_vector;
extern crate self as semio_framework_3d_brep_surface;
extern crate self as semio_framework_value;
"""
source += original[original.index("#[derive(Clone,Copy,PartialEq)] struct Vec3"):original.index("#[derive(Clone,Copy)] struct Pnt2")]
source += core.replace("type NativeVec3=Vec3;", "").replace(
    "fn to_array(self)->EVec3{evec(self)}", "fn from_array(a:EVec3)->Self{pnt(a)} fn to_array(self)->EVec3{evec(self)}")
source += original[original.index("#[derive(Clone,Copy)]struct Tol"):original.index("struct Vertex{position:Pnt3,label:PersistentLabel,tol:Tol}")]
source += "#[derive(Clone,Copy)] struct Pnt2{x:f64,y:f64} impl Pnt2{fn new(x:f64,y:f64)->Self{Self{x,y}}}\n"
source += curves
source += r"""
mod matrix{pub(super) use super::Frame3;}
impl Frame3{
 fn from_x_z(origin:Pnt3,x:Vec3,z:Vec3)->Option<Self>{let z=z.normalized()?;let y=z.cross(x).normalized()?;Some(Self{origin,x:y.cross(z),y,z})}
 fn to_world(self,p:Pnt3)->Pnt3{self.origin+self.x*p.x+self.y*p.y+self.z*p.z}
}
impl PartialEq for Frame3{fn eq(&self,r:&Self)->bool{self.origin==r.origin&&self.x==r.x&&self.y==r.y&&self.z==r.z}}
enum Surface{Plane{frame:Frame3},Cylinder{frame:Frame3,radius:f64},Sphere{frame:Frame3,radius:f64},Nurbs{}}
mod arena{
 pub(super) type FaceId=usize;pub(super) type CoedgeId=usize;pub(super) type SolidId=usize;
 pub(super) type LoopId=usize;pub(super) type EdgeId=usize;pub(super) type ShellId=usize;
 pub(super) trait ArenaId {fn raw_index(self)->u32;}
 impl ArenaId for usize {fn raw_index(self)->u32{self as u32}}
}
use arena::*;
struct Edge{curve:usize,range:(f64,f64),v0:usize,v1:usize,tol:Tol}
struct Vertex{position:Pnt3}
struct Face{surface:usize,outer:Option<usize>,inners:Vec<usize>,flipped:bool}
struct Loop{first:usize,face:usize}
struct Coedge{loop_id:usize,edge:usize,forward:bool,next:usize,pcurve:Option<usize>,prange:(f64,f64)}
struct Solid{inners:Vec<usize>,outer:usize}
struct Shell{faces:Vec<usize>}
struct Store<T>(Vec<T>);
impl<T> Default for Store<T>{fn default()->Self{Self(Vec::new())}}
impl<T> Store<T>{
 fn get(&self,id:usize)->Option<&T>{self.0.get(id)}
 fn next_remove_depth_demand(&self,id:usize)->Result<usize,ValueError>{self.get(id).map(|_|1).ok_or(ValueError("empty slot"))}
 fn push(&mut self,x:T)->usize{let id=self.0.len();self.0.push(x);id}
}
#[derive(Default)]
struct Body{solids:Store<Solid>,shells:Store<Shell>,faces:Store<Face>,loops:Store<Loop>,
 coedges:Store<Coedge>,edges:Store<Edge>,vertices:Store<Vertex>,surfaces:Store<Surface>,
 curves3:Store<Curve3>,curves2:Store<Curve2>}
mod topology{pub(super) use super::Body;}
mod primitives {
 use super::*;
 #[derive(Clone,Copy)]
 pub(super) enum SolidPrimitive{Box{origin:Pnt3,width:f64,depth:f64,height:f64},Cylinder{frame:Frame3,radius:f64,height:f64}}
 pub(super) fn planar_pcurve(curve:&Curve3,frame:Frame3)->Curve2{
   let Curve3::Line{origin,dir}=*curve else{unreachable!()};
   let map=|v:Vec3|Pnt2::new(v.dot(frame.x),v.dot(frame.y));
   Curve2::Line{origin:map(origin-frame.origin),dir:map(dir)}
 }
}
mod prism_boundary{pub(super) struct PrismBoundary;}
struct PrimitiveEdge{vertices:[usize;2],curve:Curve3,range:(f64,f64)}
struct PrimitiveTrim{edge:usize,forward:bool,curve:Curve2,range:(f64,f64)}
#[derive(Default,Clone,Copy)]
struct RetainedCloneProgress{copied_items:usize,copied_bytes:usize,retained_capacity_bytes:usize,released_bytes:usize}
impl RetainedCloneProgress {
 fn fits(self,g:RetirementDemand)->bool{self.copied_items<=1&&self.copied_bytes<=g.copy_bytes&&self.retained_capacity_bytes<=g.capacity_bytes&&self.released_bytes<=g.release_bytes}
}
#[derive(Default,Clone,Copy)]
struct RetirementDemand{copy_bytes:usize,capacity_bytes:usize,release_bytes:usize,depth:usize}
#[derive(Debug)]
enum ValueRefusalKind{InvalidValue,UnsupportedOwner,OwnershipLimit}
#[derive(Debug)]
struct ValueError(&'static str);
impl ValueError{
 fn literal(_:ValueRefusalKind,text:&'static str)->Self{Self(text)}
 fn invariant(text:&'static str)->Self{Self(text)}
 fn with_retained_progress(self,_:RetainedCloneProgress)->Self{self}
}
struct RetainedAllocationEffect<'a>{allow:bool,grant:&'a RetirementDemand,performed:bool}
impl RetainedAllocationEffect<'_>{
 fn grant(&self)->RetirementDemand{*self.grant}
 fn performed(&mut self,n:usize)->Result<bool,ValueError>{assert!(!self.performed&&n==1);self.performed=self.allow;Ok(self.allow)}
}
mod retained_clone{use super::*;pub(super) fn retained_clone_grant_admits(g:RetirementDemand,d:RetirementDemand)->bool{
 g.copy_bytes>=d.copy_bytes&&g.capacity_bytes>=d.capacity_bytes&&g.release_bytes>=d.release_bytes&&g.depth>=d.depth
}}
"""
for folder, module in [("📐️box-finish-layout","box_finish_layout"),("📦️rounded-box","rounded_box"),("🟠️rounded-box-boundary","boundary")]:
    path = next(kernel.rglob(folder+"/🦀️.rs")).resolve()
    source += f'#[path = {json.dumps(str(path),ensure_ascii=False)}] mod {module};\n'
source += "use rounded_box::RoundedBox; mod contract{pub use crate::{box_finish_layout,rounded_box::RoundedBox};}\n"
source = source.replace("] mod box_finish_layout;","] pub mod box_finish_layout;")

writer = step/"📝️retained-write"
read = step/"📥️retained-read"
write_body = (writer/"🦀️.rs").read_text().replace("mod splines;","""mod splines {
 use super::*;
 pub(super) fn surface(_: &mut Piece,_:&Surface,_:u64,_:[u64;4])->Result<(),ValueError>{unreachable!()}
 pub(super) fn curve(_: &mut Piece,_:&Curve3,_:u64,_:[u64;2],_:(f64,f64),_:[Pnt3;2])->Result<(),ValueError>{unreachable!()}
}""".replace("use super::*;","use super::*;use crate::Pnt3;")).replace('#[path = "〰️splines.rs"]\n','')
write_body = write_body.replace('"🛢️cylinder.rs"',json.dumps(str((writer/"🛢️cylinder.rs").resolve()),ensure_ascii=False))
source += "mod writer{\n"+write_body+"}\n"
decoder = (ticket/"🐍️spline-step-reader-check.py").read_text()
exec(decoder[decoder.index("reader=(step/"):decoder.index('reader=reader.replace(\'#[path = "🟠️rounded-proof.rs"]')])
start = reader.index('#[path = r')
end = reader.index('use spline_proof::Chart;',start)
reader = reader[:start]+"""mod spline_proof{
 use super::*;
 #[derive(Clone,Copy,Default)] pub(super) struct Chart;
 impl Chart{
 pub(super) fn read(_: &StepSolidDecode,_:&str,_:u64)->Result<(Face,Self),ValueError>{unreachable!()}
 pub(super) fn certify(self,_:SolidPrimitive,_:&Face)->Result<(),ValueError>{unreachable!()}
 }
}
"""+reader[end:]
reader = reader.replace('"🟠️rounded-proof.rs"',json.dumps(str((read/"🟠️rounded-proof.rs").resolve()),ensure_ascii=False))
source += "mod reader{\n"+reader+"}\n"
source += r"""
fn body(shape:RoundedBox)->Body{
 let mut b=Body::default();
 b.solids.push(Solid{inners:Vec::new(),outer:0});
 b.shells.push(Shell{faces:(0..shape.face_count()).collect()});
 for id in 0..shape.vertex_count(){b.vertices.push(Vertex{position:shape.vertex(id)});}
 for i in 0..shape.edge_count(){
  let spec=boundary::edge(shape,i);
  let curve=b.curves3.push(spec.curve);
  b.edges.push(Edge{curve,range:spec.range,v0:spec.vertices[0],v1:spec.vertices[1],tol:Tol::DEFAULT});
 }
 for (i,&count) in shape.ring_sizes().iter().enumerate(){
  let surface=b.surfaces.push(boundary::surface(shape,i));
  let first=b.coedges.0.len();
  let ring=b.loops.push(Loop{first,face:i});
  b.faces.push(Face{surface,outer:Some(ring),inners:Vec::new(),flipped:false});
  for j in 0..count{
   let trim=boundary::trim(shape,i,j);
   let curve=b.curves2.push(trim.curve);
   b.coedges.push(Coedge{loop_id:ring,edge:trim.edge,forward:trim.forward,next:first+(j+1)%count,pcurve:Some(curve),prange:trim.range});
  }
 }
 b
}
fn decode(step:&str)->Result<reader::StepSolid,ValueError>{
 let mut decoder=reader::StepSolidDecode::new(step);
 for _ in 0..25000{
  if decoder.is_ready(){assert!(decoder.terminal_is_empty());return Ok(decoder.ready().unwrap());}
  let quote=decoder.demand();
  let receipt=decoder.step(step,&mut RetainedAllocationEffect{grant:&quote,allow:true,performed:false})?;
  assert_eq!(receipt.copied_items,1);
  assert_eq!(receipt.copied_bytes,quote.copy_bytes);
 }
 panic!("STEP decoder did not terminate");
}
fn case(index:usize,origin:[f64;3],size:[f64;3],radius:f64){
 let shape=RoundedBox::new(pnt(origin),size,radius,Tol::DEFAULT.value()).unwrap();
 let body=body(shape);
 let handle="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
 let source=writer::StepWriteSource{body:&body,solid:0,revision:1,handle,depth:1};
 let mut writer=writer::StepWrite::new(source);
 let first=writer.demand(source).unwrap();
 assert_eq!(writer.step(source,&mut RetainedAllocationEffect{grant:&first,allow:false,performed:false}).unwrap().copied_items,0);
 for _ in 0..25000{
  if writer.ready(){break;}
  let quote=writer.demand(source).unwrap();
  let receipt=writer.step(source,&mut RetainedAllocationEffect{grant:&quote,allow:true,performed:false}).unwrap();
  assert_eq!(receipt.copied_items,1);
  assert!(receipt.copied_bytes<=quote.copy_bytes&&receipt.retained_capacity_bytes<=quote.capacity_bytes);
 }
 let output=writer.take_output(source).unwrap();
 for &(name,count) in STEP_ENTITIES {
  let pattern=format!(" = {name}(");
  assert_eq!(output.matches(pattern.as_str()).count(),count);
 }
 let expected=reader::StepSolid::RoundedBox{origin,size,radius};
 assert_eq!(decode(&output).unwrap(),expected);
 let shell=output.lines().find(|line|line.starts_with("#19 = CLOSED_SHELL")).unwrap();
 let ring=shell.split_once(",(").unwrap().1.strip_suffix("));").unwrap();
 let mut faces=ring.split(',').collect::<Vec<_>>();
 faces.reverse();
 assert_eq!(decode(&output.replacen(ring,&faces.join(","),1)).unwrap(),expected);
 assert!(decode(&output.replacen(" = SPHERICAL_SURFACE(", " = ELLIPTICAL_SURFACE(",1)).is_err());
 let lost_arc=output.lines().find(|l|l.contains(" = CIRCLE(")).unwrap();
 assert!(decode(&output.replacen(lost_arc,&lost_arc.replace("CIRCLE","ELLIPSE"),1)).is_err());
 let target=std::env::args().nth(1).unwrap();
 std::fs::write(std::path::Path::new(&target).join(format!("{index}.step")),output).unwrap();
}
fn main() {
"""
def array(values):
    return "[" + ",".join(repr(float(value)) for value in values) + "]"
corpus=json.loads(fixture.read_text())
for index,row in enumerate(corpus["cases"]):
    source += f'case({index},{array(row["origin"])},{array(row["size"])},{row["radius"]});\n'
entities=",".join(f'("{name}",{count})' for name,count in corpus["step"]["entities"])
source += f"}}\nconst STEP_ENTITIES:&[(&str,usize)]=&[{entities}];\n"
generated= ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    scratch=Path(folder)
    rust=scratch/"rounded-step.rs"
    rust.write_text(source)
    exe=rust.with_suffix(".exe" if os.name=="nt" else "")
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",
        "rustc","--edition=2024","-Awarnings",str(rust),"-o",str(exe)],check=True)
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",str(exe),folder],check=True)
    if "--occt" in sys.argv:
        with urllib.request.urlopen("https://registry.npmjs.org/occt-import-js/-/occt-import-js-0.0.23.tgz",timeout=20) as response:
            data=response.read(12*1024*1024)
        assert base64.b64encode(hashlib.sha512(data).digest()).decode("ascii") == "RFfYQXYFX5C1mB1Aywm0ShcUKzXOr/VzTnlzhBSDJOR6YCAPt1HYCzeXWg1vwwjn/cUxwqRNhhtf1dlewoZYCQ=="
        with tarfile.open(fileobj=io.BytesIO(data),mode="r:gz") as files:
            for name in ("occt-import-js.js","occt-import-js.wasm"):
                (scratch/name.replace(".js",".cjs")).write_bytes(files.extractfile("package/dist/"+name).read())
        subprocess.run(["repo","tools","exec","--light","node","--",
            str(ticket/"🟨️rounded-step-occt.mjs"),folder,str(fixture)],check=True)
