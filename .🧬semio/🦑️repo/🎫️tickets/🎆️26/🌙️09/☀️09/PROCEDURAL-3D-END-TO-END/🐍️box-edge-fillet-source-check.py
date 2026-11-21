"""Run the exact box fillet boundary and preview chart against the neutral authored edges."""
from pathlib import Path
import contextlib
import io
import json
import os
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
brep=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
output=io.StringIO()
with contextlib.redirect_stdout(output):
    runpy.run_path(str(ticket/"🐍️point-face-source-check.py"))
source=output.getvalue().split("mod orthogonal_fixture",1)[0]
source=source.replace("pub use quadratic_pcurve::QuadraticPcurve;","").replace("use polygon_face_publication::PolygonFacePublication;","")
source=source.replace("use polygon_wire_publication::{HelixWireRecipe,PolygonWirePublication,PolygonWireRecipe};","")
source=source.replace("enum Surface{Plane{frame:Frame3},Other}","enum Surface{Plane{frame:Frame3},Cylinder{frame:Frame3,radius:f64},Other}")
source=source.replace("enum SolidBoundary{Bore(BoreBoundary),Orthogonal(OrthogonalBoundary)}","")
primitive=(brep/"🛠️operations/🧱️primitives/🦀️.rs").read_text()
box=primitive[primitive.index("pub struct BoxGeometry;"):]
box=box[:box.index("\n}",box.index("    pub fn frame("))+2]
source=source.replace(" fn vertex(&self,_:usize)->Pnt3 {unreachable!()}",
""" fn vertex(&self,index:usize)->Pnt3 {
  let Self::Box{origin,width,depth,height}=*self else{unreachable!()};
  origin+BoxGeometry::corners(width,depth,height)[index]
 }""")
source=source.replace(" fn edge(&self,_:usize)->PrimitiveEdge {unreachable!()}",
""" fn edge_count(&self)->usize{BoxGeometry::EDGES.len()}
 fn edge(&self,i:usize)->PrimitiveEdge {
  let vertices=BoxGeometry::EDGES[i];
  let [a,b]=vertices.map(|j|self.vertex(j));
  PrimitiveEdge{vertices,range:(0.,1.),curve:Curve3::Line{origin:a,dir:b-a}}
 }""")
recipe=(brep/"⚙️engine/🧠️native-registry/🏗️solid-transaction/🧰️box-finish/🧰️fillet-profile/🦀️.rs").read_text()
recipe=recipe[recipe.index("/// 📐️"):].replace("pub(super) fn fillet_box","fn fillet_box").replace("pub(super) fn fillet_placed","fn fillet_placed")
prism_path=brep/"🛠️operations/📐️circular-prism"
prism=(prism_path/"🦀️.rs").read_text().split("#[derive(Clone,Copy)]",1)[1]
clip=next(brep.rglob("🧮️boolean-recipe/✂️disk-rectangle/🦀️.rs")).read_text().split("use std::f64::consts::TAU;",1)[1]
prism="#[derive(Clone,Copy)]"+prism+"mod disk_rectangle{use super::*;"+clip+"}"
preview=(brep/"⚙️engine/📦️mesh-io/🖼️primitive-preview/📐️circular-prism-sampling/🦀️.rs").read_text().split("\n",4)[4]
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🛠️brep-feature/🔣️.json"))
fixture=json.loads(fixture.read_text())
rows=[dict(r,edges=[r["edge"]],invalidRadius=max(r["size"])) for r in fixture["retainedBoxEdgeFillet"]]
rows.extend(fixture["retainedMultiBoxFillet"])

def array(xs):
    return "["+",".join(array(x) if isinstance(x,list) else str(float(x)) for x in xs)+"]"

helpers=r"""
use std::f64::consts::{FRAC_PI_2,TAU};
use primitives::planar_pcurve;
struct SampleVertex {point:Pnt3,normal:Vec3}
impl Pnt3 {fn from_array(values:[f64;3])->Self{pnt(values)}}
impl Surface{fn eval(&self,u:f64,v:f64)->Pnt3{match *self{
 Self::Plane{frame}=>frame.origin+frame.x*u+frame.y*v,
 Self::Cylinder{frame,radius}=>frame.origin+frame.x*(radius*u.cos())+frame.y*(radius*u.sin())+frame.z*v,
 _=>unreachable!()
}}}
fn eval_uv(curve:&Curve2,t:f64)->[f64;2]{match curve{
 Curve2::Line{origin,dir}=>[origin.x+dir.x*t,origin.y+dir.y*t],
 Curve2::Ellipse{center,x_axis,major_radius,minor_radius}=>{
  [center.x+x_axis.x*major_radius*t.cos()-x_axis.y*minor_radius*t.sin(),
   center.y+x_axis.y*major_radius*t.cos()+x_axis.x*minor_radius*t.sin()]
 }
 _=>unreachable!()
}}
fn check(origin:[f64;3],size:[f64;3],edges:&[[[f64;3];2]],axis:usize,radius:f64,invalid:f64,volume:f64,area:f64){
 let [width,depth,height]=size;
 let stock=SolidPrimitive::Box{origin:pnt(origin),width,depth,height};
 let selected=edges.iter().map(|&edge|(0..12).find(|&i| {
  let a=stock.edge(i).vertices.map(|j|stock.vertex(j).to_array());
  a==edge||[a[1],a[0]]==edge
 }).unwrap()).collect::<Vec<_>>();
 let result=fillet_box(stock,&selected,radius).unwrap();
 assert_eq!(result.axis.to_array()[axis],1.);
 assert!((result.volume-volume).abs()<1e-10);
 assert!(fillet_box(stock,&selected,0.).is_none());
 assert!(fillet_box(stock,&selected,invalid).is_none());
 let perpendicular=(0..12).find(|&i| {
  let [a,b]=stock.edge(i).vertices.map(|j|stock.vertex(j).to_array());
  a[axis]==b[axis]
 }).unwrap();
 assert!(fillet_box(stock,&[selected[0],perpendicular],radius).is_none());
 assert!(fillet_box(stock,&[selected[0],selected[0]],radius).is_none());
 let mut reversed=selected.clone();reversed.reverse();
 let same=fillet_box(stock,&reversed,radius).unwrap();
 assert_eq!(result.count,4+selected.len());
 assert!((0..result.vertex_count()).all(|i|result.vertex(i)==same.vertex(i)));
 verify_prism(result,volume,area);
}
fn verify_prism(result:CircularPrism,volume:f64,area:f64){
 assert!((result.area()-area).abs()<1e-10);
 let mut uses=vec![(0,0);result.edge_count()];
 assert!((result.volume-volume).abs()<1e-10);
 let mut mesh_volume=0.;
 let mut mesh_area=0.;
 for (face,&count) in result.ring_sizes().iter().enumerate(){
  let surface=result.surface(face);
  let mut previous=None;
  let mut first=None;
  for member in 0..count{
   let trim=result.trim(face,member);
   if trim.forward{uses[trim.edge].0+=1;}else{uses[trim.edge].1+=1;}
   let edge=result.edge(trim.edge);
   let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
   if let Some(p)=previous{assert_eq!(p,a);}
   first.get_or_insert(a);previous=Some(b);
   for s in [0.,0.25,0.5,0.75,1.] {
    let t=trim.range.0+(trim.range.1-trim.range.0)*s;
    let [u,v]=eval_uv(&trim.curve,t);
    assert!((surface.eval(u,v)-edge.curve.eval(t)).norm()<1e-12);
   }
  }
  assert_eq!(previous,first);
  for slot in 0..fillet_sampling::slots(result,face,128) {
   let triangle=fillet_sampling::triangle(result,face,slot,128);
   let [a,b,c]=triangle.each_ref().map(|v|v.point);
   let cross=(b-a).cross(c-a);
   let normals=triangle[0].normal+triangle[1].normal+triangle[2].normal;
   assert!(cross.norm()>0.);
   mesh_area+=cross.norm()/2.;
   mesh_volume+=a.dot(b.cross(c))*if cross.dot(normals)<0.{-1./6.}else{1./6.};
  }
 }
 assert!(uses.into_iter().all(|uses|uses==(1,1)));
 assert!((mesh_area-area).abs()<0.005);
 assert!((mesh_volume-volume).abs()<0.005);
}
#[test]fn all_authored_boxes_close_with_exact_arc_trims_and_a_matching_preview(){
"""
for row in rows:
    helpers+=f"check({array(row['origin'])},{array(row['size'])},&{array(row['edges'])},{row['axis']},{row['radius']},{float(row['invalidRadius'])},{row['volume']},{row['area']});\n"
helpers+="}\n"
boolean_fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").rglob("🔗️brep-boolean/🔣️.json"))
for index,row in enumerate(json.loads(boolean_fixture.read_text())["cases"]):
    if "retainedArcPrism" not in row or not row["name"].startswith("box-cylinder "):
        continue
    box_recipe,cylinder_recipe=sorted(row["inputs"].values(),key=lambda operand:operand["of"]["recipe"])
    lo=box_recipe["by"]
    hi=[a+b for a,b in zip(lo,box_recipe["of"]["size"])]
    center=cylinder_recipe["by"]
    span=[max(lo[2],center[2]),min(hi[2],center[2]+cylinder_recipe["of"]["height"])]
    bounds=[lo[:2],hi[:2]]
    output=row["outputs"]["shape"]
    helpers+=f'''
#[test]fn clipped_prism_{index}_retains_exact_curves_and_oriented_preview(){{
 let result=disk_rectangle::clip_disk({array(center[:2])},{float(cylinder_recipe["of"]["radius"])},{array(bounds)},{array(span)}).unwrap();
 assert_eq!(result.count,{row["retainedArcPrism"]["members"]});
 assert_eq!((0..result.count).filter(|&side|result.has_arc(side)).count(),{row["retainedArcPrism"]["arcs"]});
 verify_prism(result,{output["volume"]},{output["area"]});
}}
'''
oriented_path=next(brep.rglob("🏗️solid-transaction/📦️oriented-box/🦀️.rs"))
oriented_stock=oriented_path.read_text().split("pub struct OrientedBox {",1)[1].split("}",1)[0]
placement=oriented_path.read_text().split("    pub(super) fn placement(",1)[1].split("    pub fn boundary(",1)[0]
finish_path=next(brep.rglob("🏗️solid-transaction/🧰️box-finish/🦀️.rs"))
oriented_method="    fn canonical_edge"+finish_path.read_text().split("    fn canonical_edge",1)[1].replace("fillet_profile::fillet_placed","fillet_placed")
oriented="struct OrientedBox{"+oriented_stock+"} impl OrientedBox{fn placement("+placement+oriented_method
registry=next(brep.rglob("🧊️solid-publication/🦀️.rs")).read_text()
assert all(check in registry for check in ("source.rectangular()","record.label.0.checked_sub","index<edge_count","stock.fillet(&edges","stock.chamfer(edges[0],distance)"))
prism_linear=next(brep.rglob("🗜️prism-boundary/🦀️.rs")).read_text().split("pub const MAX_PRISM_CORNERS",1)[1]
prism_linear="mod linear_prism{use super::*;type NativeVec3=Vec3;pub const MAX_PRISM_CORNERS"+prism_linear+"}"
chamfer=next(brep.rglob("🧰️box-edge-chamfer/🦀️.rs")).read_text().split("pub fn box_edge_chamfer(",1)[1]
chamfer="use linear_prism::{PrismBoundary,MAX_PRISM_CORNERS};fn box_edge_chamfer("+chamfer.replace("pub(super) fn chamfer_placed","fn chamfer_placed")
helpers+=r"""
enum SolidBoundary{CircularPrism(CircularPrism),Prism(PrismBoundary)}
impl Frame3{
 fn from_x_z(origin:Pnt3,x:Vec3,z:Vec3)->Option<Self>{
  let z=z.normalized()?;
  let y=z.cross(x).normalized()?;
  Some(Self{origin,x:y.cross(z),y,z})
 }
}
fn authored_box(origin:EVec3,size:EVec3,angle:f64,pitch:f64,mirror:bool)->(OrientedBox,linear_prism::PrismBoundary){
 let (s,c)=angle.sin_cos();
 let (sp,cp)=pitch.sin_cos();
 let sign=if mirror{-1.}else{1.};
 let axes=[pnt([sign*c,sp*s,-cp*s]),pnt([0.,cp,sp]),pnt([sign*s,-sp*c,cp*c])];
 let origin=pnt(origin);
 let mut corners=[origin;linear_prism::MAX_PRISM_CORNERS];
 for (i,local) in BoxGeometry::corners(size[0],size[1],size[2]).into_iter().take(4).enumerate(){
  corners[i]=origin+axes[0]*local.x+axes[1]*local.y;
 }
 let boundary=linear_prism::PrismBoundary::from_planar_ring(corners,4,axes[2]*size[2],axes[0].cross(axes[1])).unwrap();
 (OrientedBox{origin,axes,size},boundary)
}
fn selected_stock_edges(stock:&OrientedBox,source:&linear_prism::PrismBoundary,edges:&[[EVec3;2]])->Vec<usize>{
 let [x,y,z]=stock.axes;
 let lift=|[u,v,w]:EVec3|stock.origin+x*u+y*v+z*w;
 edges.iter().map(|pair|{
  let expected=pair.map(lift);
  (0..source.edge_count()).find(|&index|{
   let [a,b]=source.edge(index).vertices.map(|vertex|source.vertex(vertex));
   let [p,q]=expected;
   ((a-p).norm()<1e-11&&(b-q).norm()<1e-11)||((a-q).norm()<1e-11&&(b-p).norm()<1e-11)
  }).unwrap()
 }).collect()
}
fn check_oriented(origin:EVec3,size:EVec3,angle:f64,pitch:f64,mirror:bool,axis:usize,radius:f64,edges:&[[EVec3;2]],volume:f64,area:f64){
 let (stock,source)=authored_box(origin,size,angle,pitch,mirror);
 let ordinals=selected_stock_edges(&stock,&source,edges);
 let OrientedBox{origin,axes,size}=stock;
 let stock=OrientedBox{origin,axes,size};
 let SolidBoundary::CircularPrism(result)=stock.fillet(&ordinals,radius).unwrap()else{unreachable!()};
 assert!((result.axis-axes[axis]).norm()<1e-12);
 assert!((result.length-size[axis]).abs()<1e-12);
 let mut centers=Vec::new();
 for face in 2..result.ring_sizes().len(){
  if let Surface::Cylinder{frame,radius:r}=result.surface(face){
   assert!((r-radius).abs()<1e-12);
   assert!((frame.z-axes[axis]).norm()<1e-12);
   centers.push(frame.origin);
  }
 }
 assert_eq!(centers.len(),edges.len());
 for [mut cap,_] in edges.iter().copied(){
  cap[axis]=0.;
  for j in (0..3).filter(|&j|j!=axis){cap[j]+=if cap[j]==0.{radius}else{-radius};}
  let expected=origin+axes[0]*cap[0]+axes[1]*cap[1]+axes[2]*cap[2];
  assert!(centers.iter().any(|c|(*c-expected).norm()<1e-11));
 }
 verify_prism(result,volume,area);
}
fn check_chamfer(origin:EVec3,size:EVec3,angle:f64,pitch:f64,mirror:bool,axis:usize,distance:f64,edge:[EVec3;2],volume:f64,area:f64){
 let (stock,source)=authored_box(origin,size,angle,pitch,mirror);
 let ordinal=selected_stock_edges(&stock,&source,&[edge])[0];
 let OrientedBox{origin,axes,size:_}=stock;
 let SolidBoundary::Prism(result)=stock.chamfer(ordinal,distance).unwrap()else{unreachable!()};
 assert_eq!((result.vertex_count(),result.edge_count()),(10,15));
 assert_eq!(result.ring_sizes(),[5,5,4,4,4,4,4]);
 let corner=origin+axes[0]*edge[0][0]+axes[1]*edge[0][1]+axes[2]*edge[0][2];
 let outward=(0..3).filter(|&i|i!=axis).fold(pnt([0.;3]),|n,i|n+axes[i]*if edge[0][i]==0.{-1.}else{1.});
 let outward=outward.normalized().unwrap();
 let mut bevels=0;
 let mut uses=[[0;2];15];
 let (mut measured_area,mut measured_volume)=(0.,0.);
 for (face,&count) in result.ring_sizes().iter().enumerate(){
  let surface=result.surface(face);
  let Surface::Plane{frame}=surface else{unreachable!()};
  if (frame.z-outward).norm()<1e-11{
   bevels+=1;
   let offset=(corner-frame.origin).dot(frame.z);
   assert!((offset-distance*std::f64::consts::FRAC_1_SQRT_2).abs()<1e-11);
  }
  let mut polygon=Vec::new();
  let mut first=None;
  let mut last=None;
  for member in 0..count{
   let trim=result.trim(face,member);
   let edge=result.edge(trim.edge);
   uses[trim.edge][usize::from(trim.forward)]+=1;
   let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
   assert!(last.is_none()||last==Some(a));
   first.get_or_insert(a);
   last=Some(b);
   polygon.push(result.vertex(a)-origin);
   for t in [0.,0.27,0.9,1.]{
    let [u,v]=eval_uv(&trim.curve,t);
    assert!((surface.eval(u,v)-edge.curve.eval(t)).norm()<1e-11);
   }
  }
  assert_eq!(first,last);
  for i in 1..count-1{
   let [a,b,c]=[polygon[0],polygon[i],polygon[i+1]];
   let vector=(b-a).cross(c-a);
   assert!(vector.dot(frame.z)>0.);
   measured_area+=vector.norm()/2.;
   measured_volume+=a.dot(b.cross(c))/6.;
  }
 }
 assert_eq!(bevels,1);
 assert!(uses.into_iter().all(|use_count|use_count==[1;2]));
 assert!((measured_area-area).abs()<1e-10);
 assert!((measured_volume-volume).abs()<1e-10);
}
fn reject_chamfer(origin:EVec3,size:EVec3,angle:f64,amount:f64,edge:[EVec3;2]){
 let (stock,source)=authored_box(origin,size,angle,0.,false);
 let index=selected_stock_edges(&stock,&source,&[edge])[0];
 assert!(stock.chamfer(index,amount).is_none());
}
fn reject_oriented(origin:EVec3,size:EVec3,angle:f64,radius:f64,edges:&[[EVec3;2]]){
 let (stock,source)=authored_box(origin,size,angle,0.,false);
 let ordinals=selected_stock_edges(&stock,&source,edges);
 assert!(stock.fillet(&ordinals,radius).is_none());
}
"""
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
placed_cases=json.loads((flow/"🔣️original-oriented-fillet.json").read_text())
stock=placed_cases["stock"]
xyz=f'{array(stock["origin"])},{array(stock["size"])},{stock["rotateY"]}'
for i,row in enumerate(placed_cases["cases"]):
    mirror=str(row["mirror"]).lower()
    helpers+=f'#[test]fn oriented_fillet_{i}(){{check_oriented({xyz},{float(row.get("rotateX",0))},{mirror},{row["axis"]},{placed_cases["radius"]},&{array(row["edges"])},{row["volume"]},{row["area"]});}}'
for i,row in enumerate(placed_cases["refused"]):
    if not row.get("foreign"):
        helpers+=f'#[test]fn oriented_refusal_{i}(){{reject_oriented({xyz},{float(row["radius"])},&{array(row["edges"])});}}'
placed_chamfer=json.loads((flow/"🔣️original-oriented-chamfer.json").read_text())
for i,row in enumerate(placed_chamfer["cases"]):
    mirror=str(row["mirror"]).lower()
    helpers+=f'#[test]fn oriented_chamfer_{i}(){{check_chamfer({xyz},{float(row.get("rotateX",0))},{mirror},{row["axis"]},{placed_chamfer["distance"]},{array(row["edges"][0])},{row["volume"]},{row["area"]});}}'
for i,row in enumerate(placed_chamfer["refused"]):
    if not row.get("foreign") and len(row["edges"])==1:
        helpers+=f'#[test]fn chamfer_refusal_{i}(){{reject_chamfer({xyz},{float(row["distance"])},{array(row["edges"][0])});}}'
sampling="mod fillet_sampling{use super::{CircularPrism,FRAC_PI_2,Surface,SampleVertex as Vertex};"+preview+"}"
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    src=Path(folder)/"fillet.rs"
    src.write_text("\n".join([source,box,recipe,prism,prism_linear,chamfer,oriented,sampling,helpers]))
    exe=src.with_suffix(".exe" if os.name=="nt" else "")
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024","--test","-Awarnings",str(src),"-o",str(exe)],check=True)
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",str(exe)],check=True)
