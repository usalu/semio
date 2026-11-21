"""🐚️ Verify the production Shell stock recipe, annular topology and preview."""
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
kernel=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
fixture=json.loads((flow/"🔣️original-cylinder-shell.json").read_text())
parts=runpy.run_path(str(ticket/"🐍️coaxial-bore-source-check.py"))["parts"]
leaf=next(kernel.rglob("🏗️solid-transaction/🐚️shell-recipe/🦀️.rs")).read_text()
recipe=leaf[leaf.index("fn invalid("):leaf.index("pub(super) fn exterior(")]
parts.append("mod stock_shell{use super::*;"+recipe+"} use stock_shell::ShellStock;")
registry=next(kernel.rglob("🔀️certified-boolean/🐚️shell/🦀️.rs")).read_text()
assert all(check in registry for check in ("outer.faces.contains(id)","Surface::Plane { frame }","draft.stock.opening(normal)?","draft.stock.finish(draft.open_sides)?"))
parts.append(r"""
fn stock_frame(origin:EVec3,angle:f64)->Frame3{
 let (s,c)=angle.sin_cos();
 Frame3{origin:pnt(origin),x:pnt([c,0.,-s]),y:Vec3::Y,z:pnt([s,0.,c])}
}
fn case_shell(frame:Frame3,radius:f64,height:f64,thickness:f64,opened:u8,levels:[f64;2],volume:f64,area:f64,shells:usize,euler:i32,probes:&[(EVec3,u8)]){
 let stock=SolidPrimitive::Cylinder{frame,radius,height};
 let recipe=ShellStock::new(stock,thickness).unwrap();
 let mut mask=0;
 for index in 0..2{
  if opened&(1<<index)==0 {continue;}
  let normal=if index==0{-frame.z}else{frame.z};
  assert_eq!(recipe.opening(normal).unwrap(),index);
  mask|=1<<index;
 }
 let boundary=recipe.finish(mask).unwrap();
 assert_eq!(boundary.shell_count(),shells);
 let SolidBoundary::Annular(shape)=boundary else{panic!("cylindrical Shell must use analytic annular walls");};
 assert!(shape.full());
 assert!((shape.inner-(radius-thickness)).abs()<1e-12);
 let inner_levels=if shape.frame.z.dot(frame.z)>0.{[shape.floor(),shape.ceiling()]}else{[height-shape.ceiling(),height-shape.floor()]};
 assert!(inner_levels.into_iter().zip(levels).all(|(a,b)|(a-b).abs()<1e-12));
 assert!((shape.volume()-volume).abs()<1e-10);
 assert!((shape.area()-area).abs()<1e-10);
 for &([x,y,z],classification) in probes{
  let point=frame.origin+frame.x*x+frame.y*y+frame.z*z;
  let distance=shape.signed_boundary_distance(point);
  let actual=if distance.abs()<1e-10{2}else if distance<0.{0}else{1};
  assert_eq!(actual,classification);
 }
 let rings=annular_boundary::ring_sizes(shape);
 assert_eq!(shape.vertex_count() as i32-shape.edge_count() as i32+2*shape.face_count() as i32-rings.len() as i32,euler);
 let mut uses=[[0u8;2];6];
 for (ring,&count) in rings.iter().enumerate(){
  let face=annular_boundary::face_index(shape,ring);
  let surface=annular_boundary::surface(shape,face);
  let mut first=None;
  let mut last=None;
  for member in 0..count{
   let trim=annular_boundary::trim(shape,ring,member);
   let edge=annular_boundary::edge(shape,trim.edge);
   let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
   assert!(last.is_none()||last==Some(a));
   first.get_or_insert(a);
   last=Some(b);
   let direction=usize::from(trim.forward!=annular_boundary::face_flipped(shape,face));
   uses[trim.edge][direction]+=1;
   for fraction in [0.,0.3,0.72,1.]{
    let t=trim.range.0+(trim.range.1-trim.range.0)*fraction;
    let uv=trim.curve.eval(t);
    let point=match surface{
     Surface::Plane{frame}=>frame.origin+frame.x*uv.x+frame.y*uv.y,
     Surface::Cylinder{frame,radius}=>frame.origin+frame.z*uv.y+frame.x*(radius*uv.x.cos())+frame.y*(radius*uv.x.sin()),
     _=>unreachable!(),
    };
    assert!((point-edge.curve.eval(t)).norm()<1e-11);
   }
  }
  assert_eq!(first,last);
 }
 assert!(uses[..shape.edge_count()].iter().all(|uses|*uses==[1,1]));
 let mut preview=[0.;2];
 for face in 0..shape.face_count(){
  for slot in 0..annular_sampling::slots(shape,face,512){
   let tri=annular_sampling::triangle(shape,face,slot,512);
   let [a,b,c]=tri.each_ref().map(|v|v.point-frame.origin);
   let cross=(b-a).cross(c-a);
   let normal=tri[0].normal+tri[1].normal+tri[2].normal;
   preview[0]+=cross.norm()/2.;
   preview[1]+=a.dot(b.cross(c))*if cross.dot(normal)>0.{1./6.}else{-1./6.};
  }
 }
 assert!((preview[0]-area).abs()<0.005);
 assert!((preview[1]-volume).abs()<0.005);
}
fn invalid_shell(frame:Frame3,radius:f64,height:f64,thickness:f64,mask:u8){
 let stock=SolidPrimitive::Cylinder{frame,radius,height};
 assert!(ShellStock::new(stock,thickness).and_then(|recipe|recipe.finish(mask)).is_err());
}
#[test]fn curved_wall_is_not_a_cap(){
 let frame=stock_frame([2.,-1.,4.],0.7);
 let stock=SolidPrimitive::Cylinder{frame,radius:3.,height:5.};
 assert!(ShellStock::new(stock,0.75).unwrap().opening(frame.x).is_err());
}
""")
def value(x):
    if isinstance(x,list):
        return "["+",".join(map(value,x))+"]"
    return repr(float(x))
base=fixture["stock"]
frame=f'stock_frame({value(base["origin"])},{value(base["rotateY"])})'
def mask(row):
    return ("bottom" in row["open"]) + 2 * ("top" in row["open"])
for i,row in enumerate(fixture["cases"]):
    probes="&["+",".join(f"({value(point)},{kind})" for point,kind in row["classify"])+"]"
    args=",".join((frame,value(base["radius"]),value(row["height"]),value(fixture["thickness"]),str(mask(row)),value(row["levels"]),value(row["volume"]),value(row["area"]),str(row["shells"]),str(row["euler"]),probes))
    parts.append(f'#[test]fn shell_{i}(){{case_shell({args});}}')
for i,row in enumerate(fixture["refused"]):
    if "side" in row["open"] or "foreign" in row["open"]:
        continue
    args=",".join((frame,value(base["radius"]),value(row["height"]),value(row["thickness"]),str(mask(row))))
    parts.append(f'#[test]fn refusal_{i}(){{invalid_shell({args});}}')
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    source=Path(folder)/"cylinder-shell.rs"
    exe=source.with_suffix(".exe" if os.name=="nt" else "")
    source.write_text("\n".join(parts))
    for args in (("rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(exe)),(str(exe),)):
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",*args],check=True)
