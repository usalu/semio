"""🐚️ Verify the exact transported orthogonal Shell topology and its world-space queries."""
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
kernel=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
fixture=json.loads((flow/"🔣️original-oriented-box-shell.json").read_text())
parts=runpy.run_path(str(ticket/"🐍️coaxial-bore-source-check.py"))["parts"]
leaf=next(kernel.rglob("🏗️solid-transaction/🐚️shell-recipe/🦀️.rs")).read_text()
recipe=leaf[leaf.index("fn invalid("):leaf.index("pub(super) fn exterior(")]
parts.append("mod stock_shell{use super::*;"+recipe+"} use stock_shell::ShellStock;")
registry=next(kernel.rglob("🔀️certified-boolean/🐚️shell/🦀️.rs")).read_text()
assert all(check in registry for check in ("outer.faces.contains(id)","source.rectangular().ok_or_else(unsupported)?","draft.stock.opening(normal)?","draft.stock.finish(draft.open_sides)?"))
query=next(kernel.rglob("📍️solid-distance/🦀️.rs")).read_text()
arm=next(line.strip() for line in query.splitlines() if "Self::PlacedOrthogonal(source,frame)" in line)
parts.append("enum PlacedProof{PlacedOrthogonal(OrthogonalCells,Frame3)} impl PlacedProof{fn distance(self,point:Pnt3)->f64{match self{"+arm+"}}}")
parts.append(r"""
fn stock_frame(origin:EVec3,angles:[f64;2],mirror:bool)->Frame3{
 let (s,c)=angles[0].sin_cos();
 let (sp,cp)=angles[1].sin_cos();
 let sign=if mirror{-1.}else{1.};
 Frame3{origin:pnt(origin),x:pnt([sign*c,sp*s,-cp*s]),y:pnt([0.,cp,sp]),z:pnt([sign*s,-sp*c,cp*c])}
}
fn stock_shell(frame:Frame3,size:EVec3,thickness:f64)->Result<ShellStock,ValueError>{
 ShellStock::oriented_box(OrientedBox{origin:frame.origin,axes:[frame.x,frame.y,frame.z],size},thickness)
}
fn shell(frame:Frame3,size:EVec3,thickness:f64,open:u8,area:f64,volume:f64,expected_bounds:[EVec3;2],shells:usize,euler:isize,probes:&[(EVec3,u8)]){
 let recipe=stock_shell(frame,size,thickness).unwrap();
 let basis=[frame.x,frame.y,frame.z];
 let mut mask=0;
 for i in 0..6{
  let direction=basis[i/2]*if i%2==0{-1.}else{1.};
  let normal=Frame3::from_normal(frame.origin,direction).unwrap().z;
  let actual=recipe.opening(normal).unwrap();
  assert_eq!(actual,i);
  if open&(1<<i)!=0{mask|=1<<actual;}
 }
 assert_eq!(mask,open);
 let SolidBoundary::Orthogonal(boundary)=recipe.finish(mask).unwrap()else{panic!("expected planar Shell");};
 assert_eq!(boundary.shell_sizes().len(),shells);
 let placement=boundary.placement().unwrap();
 assert!((placement.origin-frame.origin).norm()<1e-12);
 let shape=PlacedProof::PlacedOrthogonal(boundary.material(),placement);
 assert_eq!(boundary.vertex_count() as isize-boundary.edge_count() as isize+boundary.ring_sizes().len() as isize,euler);
 let reference=frame.to_world(pnt(size)*0.5);
 let mut uses=vec![[0;2];boundary.edge_count()];
 let mut total_area=0.;
 let mut volumes=Vec::new();
 let mut face=0;
 for &count in boundary.shell_sizes(){
  let mut shell_volume=0.;
  for i in face..face+count{
   let Surface::Plane{frame:plane}=boundary.surface(i)else{unreachable!()};
   assert_eq!(basis.into_iter().filter(|axis|axis.dot(plane.z).abs()>1e-10).count(),1);
   let mut polygon=[reference;4];
   let mut first=None;
   let mut last=None;
   for (member,point) in polygon.iter_mut().enumerate(){
    let trim=boundary.trim(i,member);
    let edge=boundary.edge(trim.edge);
    uses[trim.edge][usize::from(trim.forward)]+=1;
    let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
    assert!(last.is_none()||last==Some(a));
    first.get_or_insert(a);
    last=Some(b);
    *point=boundary.vertex(a);
    for t in [0.,0.37,1.]{
     let uv=trim.curve.eval(t);
     let point=plane.origin+plane.x*uv.x+plane.y*uv.y;
     assert!((point-edge.curve.eval(t)).norm()<1e-11);
    }
   }
   assert_eq!(last,first);
   for [a,b,c] in [[polygon[0],polygon[1],polygon[2]],[polygon[0],polygon[2],polygon[3]]]{
    let outward=(b-a).cross(c-a);
    assert!(outward.dot(plane.z)>0.);
    total_area+=outward.norm()/2.;
    shell_volume+=(a-reference).dot((b-reference).cross(c-reference))/6.;
   }
  }
  face+=count;
  volumes.push(shell_volume);
 }
 assert!(volumes[0]>0.&&volumes[1..].iter().all(|value|*value<0.));
 assert!((volumes.iter().sum::<f64>()-volume).abs()<1e-10);
 assert!((total_area-area).abs()<1e-10);
 assert!(uses.into_iter().all(|n|n==[1;2]));
 for axis in 0..3{
  let values=(0..boundary.vertex_count()).map(|i|boundary.vertex(i).to_array()[axis]);
  assert!((values.clone().fold(f64::INFINITY,f64::min)-expected_bounds[0][axis]).abs()<1e-11);
  assert!((values.fold(f64::NEG_INFINITY,f64::max)-expected_bounds[1][axis]).abs()<1e-11);
 }
 for &(point,expected) in probes{
  let distance=PlacedProof::PlacedOrthogonal(boundary.material(),placement).distance(frame.to_world(pnt(point)));
  let actual=if distance.abs()<1e-10{2}else if distance<0.{0}else{1};
  assert_eq!(actual,expected);
 }
 assert!(shape.distance(frame.to_world(pnt([-1.,-1.,-1.])))>0.);
}
#[test]fn collapsed_world_frames_refuse_exact_shell(){
 let frame=stock_frame([1e20,1e20,1e20],[0.37,0.52],true);
 assert!(stock_shell(frame,[6.,5.,4.],0.5).unwrap().finish(0).is_err());
}
""")
def value(x):
    if isinstance(x,list): return "["+",".join(map(value,x))+"]"
    return repr(float(x))
def mask(row):
    return sum(1<<i for i,side in enumerate(["-x","+x","-y","+y","-z","+z"]) if side in row["open"])
spec=fixture["stock"]
def frame(row):
    return f'stock_frame({value(spec["origin"])},{value([spec["rotateY"],row.get("rotateX",0.37)])},{str(row.get("mirror",True)).lower()})'
size=value(spec["size"])
for i,row in enumerate(fixture["cases"]):
    probes="&["+",".join(f"({value(point)},{expected})" for point,expected in row["classify"])+"]"
    args=",".join((frame(row),size,value(fixture["thickness"]),str(mask(row)),value(row["area"]),value(row["volume"]),value(row["bounds"]),str(row["shells"]),str(row["euler"]),probes))
    parts.append(f'#[test]fn shell_{i}(){{shell({args});}}')
for i,row in enumerate(fixture["refused"]):
    if not row.get("foreign"):
        parts.append(f'#[test]fn refusal_{i}(){{assert!(stock_shell({frame(row)},{size},{value(row["thickness"])}).and_then(|shape|shape.finish({mask(row)})).is_err());}}')
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    source=Path(folder)/"oriented-shell.rs"
    exe=source.with_suffix(".exe" if os.name=="nt" else "")
    source.write_text("\n".join(parts))
    for args in (("rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(exe)),(str(exe),)):
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",*args],check=True)
