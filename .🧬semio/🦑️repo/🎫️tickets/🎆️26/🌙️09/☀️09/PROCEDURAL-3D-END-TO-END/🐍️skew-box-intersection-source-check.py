"""🔀️ Verify exact skew Box Intersect, cap ownership and typed refusal through the retained recipe."""
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
kernel=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
fixture=json.loads((flow/"🔣️original-skew-box-intersection.json").read_text())
parts=runpy.run_path(str(ticket/"🐍️coaxial-bore-source-check.py"))["parts"]
parts=[part.replace("#[derive(Clone,Copy)] struct Pnt2", "#[derive(Clone,Copy,PartialEq)] struct Pnt2") for part in parts]
boundary=next(kernel.rglob("✂️clip-boundary/🦀️.rs")).read_text().split("pub(super) fn edge",1)[1]
parts.append("mod clip_boundary{use super::*;pub(super) fn edge"+boundary+"}")

classification = next(kernel.rglob("🧭️planar-classify/🦀️.rs")).read_text(encoding="utf8").split("const MAX_EDGES", 1)[1]
classification = classification.replace("Pnt3::default()", "pnt([0.;3])").replace("semio_framework_value::ValueError", "()")
section_source = next(kernel.rglob("✂️planar-section/🦀️.rs")).read_text(encoding="utf8").split("const MAX_FACES", 1)[1]
section_source = section_source.replace("Pnt3::default()", "pnt([0.;3])")
mass_owner=next(kernel.rglob("🔎️analysis/📦️planar-volume/🦀️.rs"))
mass=mass_owner.read_text(encoding="utf8").split("#[derive(Clone, Copy)]\nstruct Ring",1)[1].split("/// 🎟️ Advances one funded scalar measurement",1)[0]
mass=mass.replace("ValueError::literal(ValueRefusalKind::InvalidValue, ", "ValueError(").replace("ValueError::literal(ValueRefusalKind::UnsupportedOwner, ", "ValueError(")
centroid_code=(mass_owner.parent/"⚖️centroid/🦀️.rs").read_text(encoding="utf8").split("#[derive(Default)]",1)[1]
nearest_dir=mass_owner.parent.parent/"🎯️planar-closest"
solid_source=(nearest_dir/"🧺️solid/🦀️.rs").read_text(encoding="utf8").split("pub struct SolidClosest",1)[1]
solid_source="mod solid { use super::*;pub struct SolidClosest"+solid_source+"}"
nearest_source=(nearest_dir/"🦀️.rs").read_text(encoding="utf8").split("struct Plane",1)[1]
nearest_source=nearest_source.replace('#[path = "🧺️solid/🦀️.rs"]\nmod solid;', solid_source)
nearest_source=nearest_source.replace("ValueError::literal(ValueRefusalKind::InvalidValue, ", "ValueError(")
nearest_source=nearest_source.replace("ValueError::literal(ValueRefusalKind::UnsupportedOwner, ", "ValueError(")
nearest_source=nearest_source.replace('ValueError::invariant("', 'ValueError("')
predicates=(kernel/"📸️representation/➡️vector/⚖️predicates/🦀️.rs").read_text(encoding="utf8")
predicates=predicates.split("use super::",1)[1].split("/// 🎯️ Orientation of four 3D points",1)[0]
parts.append(r"""
mod planar_classify {
use super::*;
use crate::point_hull::HullBoundary;
type EdgeId=usize;
type SolidId=usize;
type ShellId=usize;
type VertexId=usize;
type FaceId=usize;
type LoopId=usize;
type CoedgeId=usize;
#[derive(Clone,Copy,Debug,PartialEq)]
enum PointClassification { Inside, Outside, OnBoundary }
struct Arena<T>(Vec<T>);
impl<T> Arena<T> {
 fn get(&self,id:usize)->Option<&T>{self.0.get(id)}
 fn next_remove_depth_demand(&self,id:usize)->Result<usize,()>{Ok(1+id/5)}
 fn slot_count(&self)->usize{self.0.len()}
}
struct Solid{outer:usize,inners:Vec<usize>}
struct Shell{faces:Vec<usize>}
struct Face{surface:usize,outer:Option<usize>,inners:Vec<usize>,flipped:bool}
struct Loop{face:usize,first:usize}
struct Coedge{edge:usize,loop_id:usize,next:usize,forward:bool,pcurve:Option<usize>,prange:(f64,f64)}
struct Edge{v0:usize,v1:usize,curve:usize,range:(f64,f64)}
struct Vertex{position:Pnt3}
pub struct Body{
 solids:Arena<Solid>,shells:Arena<Shell>,faces:Arena<Face>,surfaces:Arena<Surface>,
 loops:Arena<Loop>,coedges:Arena<Coedge>,edges:Arena<Edge>,curves3:Arena<Curve3>,curves2:Arena<Curve2>,vertices:Arena<Vertex>,
}
impl Body{
 fn coedge_endpoints(&self,id:CoedgeId)->Option<(VertexId,VertexId)>{
  let coedge=self.coedges.get(id)?;
  let edge=self.edges.get(coedge.edge)?;
  Some(if coedge.forward{(edge.v0,edge.v1)}else{(edge.v1,edge.v0)})
 }
}
mod mass_query{
 use super::*;
 use std::mem::size_of;
 #[derive(Default)]
 pub(super) struct RetirementDemand{pub copy_bytes:usize,pub depth:usize}
 impl From<()> for ValueError{fn from(_:())->Self{ValueError("test topology depth")}}
 #[derive(Clone,Copy,Default)]
 pub(super) struct SphericalBand;
 impl SphericalBand{
 pub(super) fn project(&self,_:&Surface,_:Pnt3)->Result<Pnt3,ValueError>{unreachable!("linear proof")}
 pub(super) fn row(&mut self,_:usize,_:&Body,_:&Surface,_:&Coedge,_:&Edge,_:&Curve3)->Result<Option<f64>,ValueError>{unreachable!("planar proof")}}
 fn area_correction(_:&Curve3,_:&Curve2,_:(f64,f64),_:[Pnt3;2],_ : &Frame3)->Result<f64,ValueError>{unreachable!("linear proof")}
 mod centroid{use super::*;#[derive(Default)]"""+centroid_code+r"""}
 use centroid::Centroid;
 #[derive(Clone, Copy)]
 struct Ring"""+mass+r"""
 pub(super) fn axis_range(body:&Body,source:PlanarMeasureSource)->[Pnt3;2]{
  let mut job=PlanarMeasure::new(source,false).with_axis_bounds();
  (0..10_000).find_map(|_| {
   let demand=job.next_demand(body).unwrap();
   assert!(demand.copy_bytes<=2*size_of::<Pnt3>()&&demand.depth>0);
   job.step(body).unwrap()
  }).expect("exact planar bounds settle");
  job.axis_bounds().unwrap()
 }
 pub(super) fn integrated(body:&Body,source:PlanarMeasureSource)->(f64,Pnt3){
  let mut job=PlanarMeasure::new(source,false).with_centroid();
  assert!(job.centroid().is_none());
  let volume=(0..10_000).find_map(|_|{
   let demand=job.next_demand(body).unwrap();
   assert!(demand.copy_bytes<=2*size_of::<Pnt3>()&&demand.depth>0);
   job.step(body).unwrap()
  }).expect("exact planar traversal settles");
  (volume,job.centroid().unwrap())
 }
}
"""+"const MAX_EDGES"+classification+r"""
mod nearest_query{
 use super::*;
 use mass_query::{PlanarMeasureSource,RetirementDemand,SphericalBand};
 use std::mem::size_of;
 pub(super) mod predicates { use super::"""+predicates+r"""}
 use predicates::ray_event;
 mod arc {
  use super::*;
  pub(super) fn project(_:&Curve3,_:&Curve2,_:(f64,f64),_:[Pnt3;2],_:&Foot)->Result<(bool,bool,Pnt3),ValueError>{unreachable!("linear proof")}
 }
 fn original_linear_distance(_:(&Curve3,Option<(f64,f64)>),_:(&Curve3,Option<(f64,f64)>))->Result<f64,ValueError>{unreachable!("point proof")}
 fn plane_probe(_:(&Curve3,Option<(f64,f64)>),_:Pnt3,_:Vec3)->Result<Option<(Pnt3,f64)>,ValueError>{unreachable!("point proof")}
 struct Plane"""+nearest_source+r"""
}
mod section_query { use super::*;
 use super::nearest_query::predicates::{Orient,orient2d};
 use super::nearest_query::predicates::ray_event;
"""+"const MAX_FACES"+section_source+r"""}
use section_query::{PlanarSection,planar_section};
fn make_body(shape:&HullBoundary)->Body{
 let mut faces=Vec::new();
 let mut loops=Vec::new();
 let mut coedges=Vec::new();
 let shell_faces=(0..shape.shell_count()).scan(0,|first,index|{
  let next=*first+shape.shell_face_count(index);
  let range=*first..next;
  *first=next;
  Some(Shell{faces:range.collect()})
 }).collect();
 for (face,&count) in shape.ring_sizes().iter().enumerate(){
  faces.push(Face{surface:face,outer:Some(face),inners:Vec::new(),flipped:false});
  let first=coedges.len();
  loops.push(Loop{face,first});
  for member in 0..count{
   let trim=shape.trim(face,member);
   coedges.push(Coedge{edge:trim.edge,loop_id:face,next:first+(member+1)%count,forward:trim.forward,pcurve:None,prange:trim.range});
  }
 }
 Body {
  solids:Arena(vec![Solid{outer:0,inners:(1..shape.shell_count()).collect()}]),
  shells:Arena(shell_faces),faces:Arena(faces),
  surfaces:Arena((0..shape.ring_sizes().len()).map(|i|shape.surface(i)).collect()),
  loops:Arena(loops),coedges:Arena(coedges),
  edges:Arena((0..shape.edge_count()).map(|i|{let spec=shape.edge(i);let [v0,v1]=spec.vertices;Edge{v0,v1,curve:i,range:spec.range}}).collect()),
  curves3:Arena((0..shape.edge_count()).map(|i|shape.edge(i).curve).collect()),
  curves2:Arena(Vec::new()),
  vertices:Arena((0..shape.vertex_count()).map(|i|Vertex{position:shape.vertex(i)}).collect()),
 }
}
fn body_of_hulls(mut shapes:impl Iterator<Item=HullBoundary>)->Body{
 let mut body=make_body(&shapes.next().unwrap());
 for shape in shapes{
  let mut next=make_body(&shape);
  let (so,sh,fa,lo,co,ed,ve)=(body.solids.0.len(),body.shells.0.len(),body.faces.0.len(),body.loops.0.len(),
    body.coedges.0.len(),body.edges.0.len(),body.vertices.0.len());
  for solid in &mut next.solids.0{solid.outer+=sh;for shell in &mut solid.inners{*shell+=sh;}}
  for shell in &mut next.shells.0{for face in &mut shell.faces{*face+=fa;}}
  for face in &mut next.faces.0{face.surface+=fa;face.outer=face.outer.map(|id|id+lo);}
  for ring in &mut next.loops.0{ring.face+=fa;ring.first+=co;}
  for edge in &mut next.coedges.0{edge.edge+=ed;edge.loop_id+=lo;edge.next+=co;}
  for edge in &mut next.edges.0{edge.v0+=ve;edge.v1+=ve;edge.curve+=ed;}
  body.solids.0.extend(next.solids.0);body.shells.0.extend(next.shells.0);body.faces.0.extend(next.faces.0);
  body.loops.0.extend(next.loops.0);body.coedges.0.extend(next.coedges.0);body.edges.0.extend(next.edges.0);
  body.curves3.0.extend(next.curves3.0);body.vertices.0.extend(next.vertices.0);body.surfaces.0.extend(next.surfaces.0);
  assert_eq!(body.solids.0.len(),so+1);
 }
 body
}
pub fn check_centroid(shapes:impl Iterator<Item=HullBoundary>,expected:EVec3,volume:f64){
 let body=body_of_hulls(shapes);
 use mass_query::PlanarMeasureSource;
 let count=body.solids.0.len();
 let source=if count==1{PlanarMeasureSource::Solid(0)}else{
  PlanarMeasureSource::compound(&(0..count).collect::<Vec<_>>()).unwrap()
 };
 let (value,point)=mass_query::integrated(&body,source);
 assert!((value-volume).abs()<1e-10);
 assert!((point-pnt(expected)).norm()<1e-7);
}
pub fn check_compound_bounds(shapes:impl Iterator<Item=HullBoundary>,expected:[EVec3;2]){
 let body=body_of_hulls(shapes);
 let members=(0..body.solids.0.len()).collect::<Vec<_>>();
 let source=mass_query::PlanarMeasureSource::compound(&members).unwrap();
 for (point,expected) in mass_query::axis_range(&body,source).into_iter().zip(expected){
  assert!(point.distance(pnt(expected))<1e-9);
 }
}
pub fn check_compound_nearest(shapes:impl Iterator<Item=HullBoundary>,probes:&[(EVec3,EVec3)]){
 let body=body_of_hulls(shapes);
 let members=(0..body.solids.0.len()).collect::<Vec<_>>();
 let source=mass_query::PlanarMeasureSource::compound(&members).unwrap();
 for &(probe,expected) in probes {
  let mut job=nearest_query::SolidClosest::new(source,pnt(probe));
  let point=(0..4096).find_map(|_|{
   let demand=job.next_demand(&body).unwrap();
   assert!(demand.depth>0&&demand.copy_bytes<=128);
   job.step(&body).unwrap()
  }).expect("original face traversal settles");
  assert!(point.distance(pnt(expected))<2e-7,"{:?} != {expected:?}",point.to_array());
 }
}
pub fn check_compound_classification(shapes:impl Iterator<Item=HullBoundary>,probes:&[(EVec3,u8)]){
 let body=body_of_hulls(shapes);
 let members=(0..body.solids.0.len()).collect::<Vec<_>>();
 assert!(planar_compound_classification_depth(&body,&members).unwrap().unwrap()>0);
 for &(point,expected) in probes{
  let point=pnt(point);
  let actual=classify_planar_compound(&body,&members,point,1e-6).unwrap();
  assert_eq!(actual,[PointClassification::Inside,PointClassification::Outside,PointClassification::OnBoundary][expected as usize]);
 }
}
pub fn check_void_section(shape:&HullBoundary,origin:EVec3,normal:EVec3,areas:[f64;2]){
 let body=make_body(shape);
 let PlanarSection::Region(region)=planar_section(&body,0,origin,normal,1e-6).unwrap().unwrap()else{panic!("expected nested section contours");};
 let (outer,holes)=region.into_rings();
 let anchor=pnt(origin);
 let axis=pnt(normal).normalized().unwrap();
 let rings=std::iter::once(outer).chain(holes.into_iter().flatten()).collect::<Vec<_>>();
 assert_eq!(rings.len(),areas.len());
 for (ring,want) in rings.iter().zip(areas){
  let area=(0..ring.count()).map(|i|{
   let a=pnt(ring.point(i).unwrap())-anchor;
   let b=pnt(ring.point((i+1)%ring.count()).unwrap())-anchor;
   assert!(a.dot(axis).abs()<1e-10);
   a.cross(b).dot(axis)
  }).sum::<f64>()/2.;
  assert!((area-want).abs()<1e-6,"{area} != {want}");
 }
}
pub fn check_section(shape:&HullBoundary,origin:EVec3,normal:EVec3,area:f64,concave:bool)->Vec<Pnt3>{
 let body=make_body(shape);
 let polygon=match planar_section(&body,0,origin,normal,1e-6).unwrap().unwrap(){
  PlanarSection::Convex(mut section)=>{
   while section.has_edge(){section.advance_edge().unwrap();}
   section.finish().unwrap()
  }
  PlanarSection::Polygon(section)=>section,
  PlanarSection::Region(_) | PlanarSection::Regions(_) => panic!("expected one section boundary"),
 };
 let origin=pnt(origin);
 let normal=pnt(normal).normalized().unwrap();
 let points=(0..polygon.count()).map(|i|pnt(polygon.point(i).unwrap())).collect::<Vec<_>>();
 assert!(points.len()>=4);
 assert!(points.iter().all(|&point|(point-origin).dot(normal).abs()<1e-10));
 let measured=(0..points.len()).map(|i|(points[i]-origin).cross(points[(i+1)%points.len()]-origin).dot(normal)).sum::<f64>()/2.;
 assert!((measured-area).abs()<1e-6,"{measured} != {area}");
 if concave{
  assert!((0..points.len()).any(|i|{
   let [a,b,c]=[i,(i+1)%points.len(),(i+2)%points.len()].map(|i|points[i]);
   (b-a).cross(c-b).dot(normal)< -1e-8
  }));
 }
 points
}
pub fn check(shape:&HullBoundary,probes:&[(EVec3,u8)]){
 let mut body=make_body(shape);
 assert!(planar_classification_depth(&body,0).unwrap()>1);
 for &(xyz,expected) in probes{
  let actual=classify_planar_solid(&body,0,pnt(xyz),1e-6);
  assert_eq!(actual,Some([PointClassification::Inside,PointClassification::Outside,PointClassification::OnBoundary][usize::from(expected)]),"{xyz:?}");
 }
 body.coedges.0[0].forward=!body.coedges.0[0].forward;
 assert!(classify_planar_solid(&body,0,pnt([0.,0.,0.]),1e-6).is_none());
}
}
""")

parts.append(r"""
fn oriented_stock(origin:EVec3,size:EVec3,angle:[f64;2],mirror:bool)->OrientedBox{
 let (s,c)=angle[0].sin_cos();
 let (sp,cp)=angle[1].sin_cos();
 let sign=if mirror{-1.}else{1.};
 OrientedBox{origin:pnt(origin),axes:[pnt([sign*c,sp*s,-cp*s]),pnt([0.,cp,sp]),pnt([sign*s,-sp*c,cp*c])],size}
}


fn check_overlapping_union(a:OrientedBox,b:OrientedBox,topology:[usize;3],area:f64,volume:f64,bounds:[EVec3;2]){
 for (left,right) in [(a,b),(b,a)]{
  let SolidBoundary::Hull(result)=BooleanShape::OrientedBox(left).combine(BooleanShape::OrientedBox(right),BooleanOp::Unite).unwrap()else{panic!("expected a sewn Union");};
  assert_eq!(result.shell_count(),1);
  assert_eq!([result.vertex_count(),result.edge_count(),result.ring_sizes().len()],topology);
  assert_eq!(result.vertex_count()+result.ring_sizes().len(),result.edge_count()+2);
  let mut uses=vec![[0;2];result.edge_count()];
  let (mut total_area,mut total_volume)=(0.,0.);
  for (face,&count) in result.ring_sizes().iter().enumerate(){
   let Surface::Plane{frame}=result.surface(face)else{unreachable!()};
   assert!([left,right].into_iter().any(|stock| (0..3).any(|i|{
    let axis=stock.axes[i];
    [(stock.origin,-axis),(stock.origin+axis*stock.size[i],axis)].into_iter().any(|(origin,normal)|
      (frame.z-normal).norm()<1e-11&&(frame.origin-origin).dot(normal).abs()<1e-11)
   })));
   let mut points=Vec::new();
   for member in 0..count{
    let trim=result.trim(face,member);
    uses[trim.edge][usize::from(trim.forward)]+=1;
    let edge=result.edge(trim.edge);
    points.push(result.vertex(edge.vertices[usize::from(!trim.forward)]));
   }
   let origin=points[0];
   for pair in points[1..].windows(2){
    let normal=(pair[0]-origin).cross(pair[1]-origin);
    assert!(normal.dot(frame.z)>=-1e-11);
    total_area+=normal.norm()/2.;
    total_volume+=(origin-left.origin).dot(normal)/6.;
   }
  }
  assert!(uses.iter().all(|uses|*uses==[1,1]));
  assert!((total_area-area).abs()<1e-10);
  assert!((total_volume-volume).abs()<1e-10);
  for (axis,[lo,hi]) in (0..3).map(|axis|(axis,[bounds[0][axis],bounds[1][axis]])){
   let points=(0..result.vertex_count()).map(|i|result.vertex(i).to_array()[axis]);
   assert!((points.clone().fold(f64::INFINITY,f64::min)-lo).abs()<1e-10);
   assert!((points.fold(f64::NEG_INFINITY,f64::max)-hi).abs()<1e-10);
  }
 }
}

fn check_recess(stock:OrientedBox,tool:OrientedBox,counts:[usize;3],area:f64,volume:f64,bounds:[EVec3;2],probes:&[(EVec3,u8)],section:Option<(EVec3,EVec3,f64)>,expected_walls:usize,expected_junctions:usize){
 let SolidBoundary::Hull(result)=cut_result(stock,tool).unwrap()else{panic!("expected sewn recess")};
 planar_classify::check(&result,probes);
 if let Some((origin,normal,area))=section{
  planar_classify::check_section(&result,origin,normal,area,true);
  planar_classify::check_section(&result,origin,normal.map(|component|-component),area,true);
 }
 assert_eq!(result.shell_count(),1);
 assert_eq!([result.vertex_count(),result.edge_count(),result.ring_sizes().len()],counts);
 assert_eq!(result.vertex_count()+result.ring_sizes().len(),result.edge_count()+2);
 let mut uses=vec![[0;2];result.edge_count()];
 let mut walls=Vec::new();
 let mut measured=[0.,0.];
 for (face,&count) in result.ring_sizes().iter().enumerate(){
  let Surface::Plane{frame}=result.surface(face)else{unreachable!()};
  let candidates=|source:OrientedBox,reverse:bool| (0..3).any(|i|{
   let axis=source.axes[i];
   [(source.origin,-axis),(source.origin+axis*source.size[i],axis)].into_iter().any(|(origin,normal)|{
    (frame.z-if reverse{-normal}else{normal}).norm()<1e-11&&(frame.origin-origin).dot(frame.z).abs()<1e-11
   })
  });
  if candidates(tool,true){walls.push(face);}
  else{assert!(candidates(stock,false));}
  let mut polygon=Vec::new();
  for member in 0..count{
   let trim=result.trim(face,member);
   let edge=result.edge(trim.edge);
   let next=result.trim(face,(member+1)%count);
   let following=result.edge(next.edge);
   uses[trim.edge][usize::from(trim.forward)]+=1;
   assert_eq!(edge.vertices[usize::from(trim.forward)],following.vertices[usize::from(!next.forward)]);
   polygon.push(result.vertex(edge.vertices[usize::from(!trim.forward)]));
   for t in [0.,0.37,1.]{
    let uv=trim.curve.eval(t);
    assert!((frame.origin+frame.x*uv.x+frame.y*uv.y-edge.curve.eval(t)).norm()<1e-11);
   }
  }
  let origin=polygon[0];
  for triangle in polygon[1..].windows(2){
   let outward=(triangle[0]-origin).cross(triangle[1]-origin);
   assert!(outward.dot(frame.z)>=-1e-11);
   measured[0]+=outward.norm()/2.;
   measured[1]+=(origin-stock.origin).dot(outward)/6.;
  }
 }
 assert_eq!(walls.len(),expected_walls);
 let wall_edges=|face:usize|(0..result.ring_sizes()[face]).map(|member|result.trim(face,member).edge).collect::<Vec<_>>();
 let mut junctions=0;
 for (index,&a) in walls.iter().enumerate(){
  for &b in &walls[index+1..]{
   let shared=wall_edges(a).iter().filter(|edge|wall_edges(b).contains(edge)).count();
   assert!(shared<=1);
   junctions+=shared;
  }
 }
 assert_eq!(junctions,expected_junctions);
 assert!(uses.iter().all(|row|*row==[1,1]));
 assert!((measured[0]-area).abs()<1e-10);
 assert!((measured[1]-volume).abs()<1e-10);
 for (axis,[lo,hi]) in (0..3).map(|axis|(axis,[bounds[0][axis],bounds[1][axis]])){
  let points=(0..result.vertex_count()).map(|index|result.vertex(index).to_array()[axis]);
  assert!((points.clone().fold(f64::INFINITY,f64::min)-lo).abs()<1e-10);
  assert!((points.fold(f64::NEG_INFINITY,f64::max)-hi).abs()<1e-10);
 }
}

fn check_skew(a:OrientedBox,b:OrientedBox,operation:BooleanOp,origin:EVec3,normal:EVec3,vertices:usize,area:f64,volume:f64,bounds:[EVec3;2],probes:&[(EVec3,u8)]){
 let swaps=if matches!(operation,BooleanOp::Cut){1}else{2};
 for (left,right) in [(a,b),(b,a)].into_iter().take(swaps){
  let SolidBoundary::ClippedBox(shape,proof)=BooleanShape::OrientedBox(left).combine(BooleanShape::OrientedBox(right),operation).unwrap()else{unreachable!()};
  let mut uses=[[0;2];15];
  let (mut measured_area,mut measured_volume)=(0.,0.);
  let cap=shape.ring_sizes().len()-1;
  assert_eq!(shape.ring_sizes()[cap],vertices);
  assert_eq!(shape.vertex_count()+shape.ring_sizes().len(),shape.edge_count()+2);
  for (face,&count) in shape.ring_sizes().iter().enumerate(){
   let frame=shape.frame(face);
   if face==cap{
    assert!((frame.z-pnt(normal)).norm()<1e-11);
    assert!((frame.origin-pnt(origin)).dot(frame.z).abs()<1e-11);
   }else{
    assert!(a.axes.into_iter().any(|axis|(axis.dot(frame.z).abs()-1.).abs()<1e-11));
   }
   let mut polygon=[pnt([0.;3]);6];
   for (i,point) in polygon.iter_mut().enumerate().take(count){
    let trim=clip_boundary::trim(&shape,face,i);
    let edge=clip_boundary::edge(&shape,trim.edge);
    uses[trim.edge][usize::from(trim.forward)]+=1;
    let vertex=edge.vertices[usize::from(!trim.forward)];
    *point=shape.vertex(vertex);
    let next=clip_boundary::trim(&shape,face,(i+1)%count);
    assert_eq!(edge.vertices[usize::from(trim.forward)],
               clip_boundary::edge(&shape,next.edge).vertices[usize::from(!next.forward)]);
    for t in [0.,0.37,1.]{
     let uv=trim.curve.eval(t);
     assert!((frame.origin+frame.x*uv.x+frame.y*uv.y-edge.curve.eval(t)).norm()<1e-11);
    }
   }
   for i in 1..count-1{
    let [p,q,r]=[polygon[0],polygon[i],polygon[i+1]];
    let outward=(q-p).cross(r-p);
    assert!(outward.dot(frame.z)>0.);
    measured_area+=outward.norm()/2.;
    measured_volume+=(p-pnt(origin)).dot((q-pnt(origin)).cross(r-pnt(origin)))/6.;
   }
  }
  assert!((measured_area-area).abs()<1e-10);
  assert!((measured_volume-volume).abs()<1e-10);
  assert!(uses[..shape.edge_count()].iter().all(|uses|*uses==[1,1]));
  for (actual,wanted) in proof.bounds().unwrap().map(Pnt3::to_array).into_iter().flatten().zip(bounds.into_iter().flatten()){
   assert!((actual-wanted).abs()<1e-10);
  }
  for &(xyz,wanted) in probes{
   let distance=proof.signed_boundary_distance(pnt(xyz)).unwrap();
   let actual=if distance.abs()<1e-10{2}else if distance<0.{0}else{1};
   assert_eq!(actual,wanted);
  }
 }
}
fn check_convex(a:OrientedBox,b:OrientedBox,counts:[usize;3],area:f64,volume:f64,bounds:[EVec3;2],section:(EVec3,EVec3,f64)){
 let supports=[a,b].into_iter().flat_map(|stock|{
  (0..3).flat_map(move |i|[(stock.origin,-stock.axes[i]),(stock.origin+stock.axes[i]*stock.size[i],stock.axes[i])])
 }).collect::<Vec<_>>();
 for (left,right) in [(a,b),(b,a)]{
  let SolidBoundary::Hull(result)=BooleanShape::OrientedBox(left).combine(BooleanShape::OrientedBox(right),BooleanOp::Intersect).unwrap()
   else{panic!("expected a multi-support convex result");};
  assert_eq!([result.vertex_count(),result.edge_count(),result.ring_sizes().len()],counts);
  planar_classify::check_section(&result,section.0,section.1,section.2,false);
  planar_classify::check_section(&result,section.0,section.1.map(|component|-component),section.2,false);
  for &(origin,normal) in &supports{
   assert!((0..result.vertex_count()).all(|index|(result.vertex(index)-origin).dot(normal)<=1e-10));
  }
  let mut used=[false;12];
  let mut uses=[[0;2];30];
  let (mut measured_volume,mut measured_area)=(0.,0.);
  for (face,&count) in result.ring_sizes().iter().enumerate(){
   let Surface::Plane{frame}=result.surface(face)else{unreachable!()};
   let source=supports.iter().position(|&(o,n)|(o-frame.origin).norm()<1e-12&&(n-frame.z).norm()<1e-12).unwrap();
   assert!(!std::mem::replace(&mut used[source],true));
   let mut polygon=[a.origin;11];
   let mut first=None;
   let mut last=None;
   for (member,point) in polygon.iter_mut().enumerate().take(count){
    let trim=result.trim(face,member);
    let edge=result.edge(trim.edge);
    uses[trim.edge][usize::from(trim.forward)]+=1;
    let [a,b]=if trim.forward{edge.vertices}else{[edge.vertices[1],edge.vertices[0]]};
    assert!(last.is_none()||last==Some(a));
    first.get_or_insert(a);
    last=Some(b);
    *point=result.vertex(a);
    for t in [0.,0.37,1.]{
     let uv=trim.curve.eval(t);
     assert!((frame.origin+frame.x*uv.x+frame.y*uv.y-edge.curve.eval(t)).norm()<1e-11);
    }
   }
   assert_eq!(first,last);
   for i in 1..count-1{
    let [x,y,z]=[polygon[0],polygon[i],polygon[i+1]];
    let outward=(y-x).cross(z-x);
    assert!(outward.dot(frame.z)>0.);
    measured_area+=outward.norm()/2.;
    measured_volume+=(x-a.origin).dot((y-a.origin).cross(z-a.origin))/6.;
   }
  }
  assert!((measured_volume-volume).abs()<1e-10);
  assert!((measured_area-area).abs()<1e-10);
  assert!(uses[..result.edge_count()].iter().all(|n|*n==[1;2]));
  for axis in 0..3{
   let values=(0..result.vertex_count()).map(|i|result.vertex(i).to_array()[axis]);
   assert!((values.clone().fold(f64::INFINITY,f64::min)-bounds[0][axis]).abs()<1e-10);
   assert!((values.fold(f64::NEG_INFINITY,f64::max)-bounds[1][axis]).abs()<1e-10);
  }
 }
}
fn cut_result(a:OrientedBox,b:OrientedBox)->Result<SolidBoundary,ValueError>{
 BooleanShape::OrientedBox(a).combine(BooleanShape::OrientedBox(b),BooleanOp::Cut)
}
fn check_cavity(a:OrientedBox,b:OrientedBox,area:f64,volume:f64,bounds:[EVec3;2],probes:&[(EVec3,u8)],section:(EVec3,EVec3,f64),void_section:(EVec3,EVec3,[f64;2])){
 let SolidBoundary::Hull(shape)=cut_result(a,b).unwrap()else{unreachable!()};
 planar_classify::check(&shape,probes);
 for axis in [void_section.1,void_section.1.map(|axis|-axis)]{
  planar_classify::check_void_section(&shape,void_section.0,axis,void_section.2);
 }
 planar_classify::check_section(&shape,section.0,section.1,section.2,false);
 planar_classify::check_section(&shape,section.0,section.1.map(|component|-component),section.2,false);
 assert_eq!((shape.vertex_count(),shape.edge_count(),shape.ring_sizes().len()),(16,24,12));
 assert_eq!(shape.shell_count(),2);
 assert_eq!([shape.shell_face_count(0),shape.shell_face_count(1)],[6,6]);
 let mut volumes=[0.;2];
 let mut measured_area=0.;
 let mut uses=[[0;2];24];
 for (face,&count) in shape.ring_sizes().iter().enumerate(){
  assert_eq!(count,4);
  let shell=usize::from(face>=6);
  let stock=[a,b][shell];
  let Surface::Plane{frame}=shape.surface(face)else{unreachable!()};
  let normals=stock.axes;
  let center=(0..3).fold(stock.origin,|point,i|point+normals[i]*(stock.size[i]/2.));
  assert_eq!((frame.origin-center).dot(frame.z).is_sign_positive(),shell==0);
  assert!(normals.into_iter().any(|axis|(axis.dot(frame.z).abs()-1.).abs()<1e-12));
  let mut polygon=[stock.origin;4];
  for (member,point) in polygon.iter_mut().enumerate(){
   let trim=shape.trim(face,member);
   let edge=shape.edge(trim.edge);
   uses[trim.edge][usize::from(trim.forward)]+=1;
   *point=shape.vertex(edge.vertices[usize::from(!trim.forward)]);
   let next=shape.trim(face,(member+1)%4);
   assert_eq!(edge.vertices[usize::from(trim.forward)],shape.edge(next.edge).vertices[usize::from(!next.forward)]);
   for t in [0.,0.39,1.]{
    let uv=trim.curve.eval(t);
    assert!((frame.origin+frame.x*uv.x+frame.y*uv.y-edge.curve.eval(t)).norm()<1e-11);
   }
  }
  for i in 1..3{
   let [u,v,w]=[polygon[0],polygon[i],polygon[i+1]];
   let cross=(v-u).cross(w-u);
   assert!(cross.dot(frame.z)>0.);
   volumes[shell]+=(u-a.origin).dot((v-a.origin).cross(w-a.origin))/6.;
   measured_area+=cross.norm()/2.;
  }
 }
 assert!(volumes[0]>0.&&volumes[1]<0.);
 assert!((volumes.iter().sum::<f64>()-volume).abs()<1e-10);
 assert!((measured_area-area).abs()<1e-10);
 assert!(uses.into_iter().all(|n|n==[1;2]));
 for axis in 0..3{
  let values=(0..shape.vertex_count()).map(|i|shape.vertex(i).to_array()[axis]);
  assert!((values.clone().fold(f64::INFINITY,f64::min)-bounds[0][axis]).abs()<1e-10);
  assert!((values.fold(f64::NEG_INFINITY,f64::max)-bounds[1][axis]).abs()<1e-10);
 }
}
fn check_unchanged(a:OrientedBox,b:OrientedBox){
 let SolidBoundary::BoxPrism(result,_)=cut_result(a,b).unwrap()else{unreachable!()};
 let SolidBoundary::BoxPrism(stock,_)=a.boundary().unwrap()else{unreachable!()};
 assert_eq!(result.vertex_count(),stock.vertex_count());
 for i in 0..stock.vertex_count(){
  assert!((result.vertex(i)-stock.vertex(i)).norm()<1e-12);
 }
}
fn check_compound_cut(a:OrientedBox,b:OrientedBox,expected:&[(f64,f64,i64)]){
 assert_eq!(cut_result(a,b).err().unwrap().0,"unsupported");
 let batch=BooleanShape::OrientedBox(a).disconnected(BooleanShape::OrientedBox(b),BooleanOp::Cut)
   .expect("exact placed Cut should return a compound");
 assert_eq!(batch.count(),expected.len());
 let mut metrics=Vec::new();
 for index in 0..batch.count(){
  let SolidBoundary::Hull(member)=batch.member(index).unwrap()else{panic!("expected authored planar component")};
  let chi=member.vertex_count() as i64-member.edge_count() as i64+member.ring_sizes().len() as i64;
  let mut edge_uses=vec![[0;2];member.edge_count()];
  let (mut area,mut volume)=(0.,0.);
  for (face,&count) in member.ring_sizes().iter().enumerate(){
   let Surface::Plane{frame}=member.surface(face)else{unreachable!()};
   let points=(0..count).map(|part| {
    let trim=member.trim(face,part);
    let edge=member.edge(trim.edge);
    edge_uses[trim.edge][usize::from(trim.forward)]+=1;
    let uv=trim.curve.eval(0.37);
    assert!((frame.origin+frame.x*uv.x+frame.y*uv.y-edge.curve.eval(0.37)).norm()<1e-10);
    member.vertex(edge.vertices[usize::from(!trim.forward)])
   }).collect::<Vec<_>>();
   for pair in points[1..].windows(2){
    area+=(pair[0]-points[0]).cross(pair[1]-points[0]).dot(frame.z)/2.;
    volume+=(points[0]-a.origin).dot((pair[0]-a.origin).cross(pair[1]-a.origin))/6.;
   }
  }
  assert!(edge_uses.iter().all(|uses|*uses==[1,1]));
  assert_eq!(member.shell_count(),1);
  metrics.push((volume,area,chi));
 }
 metrics.sort_unstable_by(|x,y|x.0.total_cmp(&y.0));
 for ((volume,area,chi),&(want_volume,want_area,want_chi)) in metrics.into_iter().zip(expected){
  assert!((volume-want_volume).abs()<1e-10);
  assert!((area-want_area).abs()<1e-10);
  assert_eq!(chi,want_chi);
 }
}
fn check_separated_union(a:OrientedBox,b:OrientedBox){
 check_face_axes_overlap(a,b);
 for (first,second) in [(a,b),(b,a)]{
  let batch=BooleanShape::OrientedBox(first).disconnected(BooleanShape::OrientedBox(second),BooleanOp::Unite).unwrap();
  assert_eq!(batch.count(),2);
  for (index,stock) in [first,second].into_iter().enumerate(){
   let SolidBoundary::BoxPrism(_,proof)=batch.member(index).unwrap()else{unreachable!()};
   let retained=proof.rectangular().unwrap();
   assert!(retained.origin==stock.origin);
   assert!(retained.axes==stock.axes);
   assert_eq!(retained.size,stock.size);
  }
  assert!(batch.member(2).is_none());
 }
 assert!(BooleanShape::OrientedBox(a).disconnected(BooleanShape::OrientedBox(a),BooleanOp::Unite).is_none());
}
fn check_refusal(a:OrientedBox,b:OrientedBox,kind:&str){
 for (left,right) in [(a,b),(b,a)]{
  assert_eq!(BooleanShape::OrientedBox(left).combine(BooleanShape::OrientedBox(right),BooleanOp::Intersect).err().unwrap().0,kind);
 }
}
fn check_face_axes_overlap(a:OrientedBox,b:OrientedBox){
 let interval=|stock:OrientedBox,axis:Vec3|{
  (0..8).map(|corner|{
   let point=(0..3).filter(|i|corner&(1<<i)!=0).fold(stock.origin,|p,i|p+stock.axes[i]*stock.size[i]);
   point.dot(axis)
  }).fold([f64::INFINITY,f64::NEG_INFINITY],|[lo,hi],n|[lo.min(n),hi.max(n)])
 };
 for axis in a.axes.into_iter().chain(b.axes){
  let [lo,hi]=interval(a,axis);
  let [start,end]=interval(b,axis);
  assert!(hi.min(end)-lo.max(start)>0.03);
 }
}
""")
def value(x):
    if isinstance(x,list): return "["+",".join(map(value,x))+"]"
    return repr(float(x))
def stock(row):
    return f'oriented_stock({value(row["origin"])},{value(row["size"])},{value(row["angles"])},{str(row["mirror"]).lower()})'
for i,row in enumerate(fixture["cases"]):
    probes="&["+",".join(f"({value(xyz)},{kind})" for xyz,kind in row["classify"])+"]"
    args=",".join((stock(row["stock"]),stock(row["tool"]),"BooleanOp::Intersect",value(row["capOrigin"]),value(row["capNormal"]),str(row["capVertices"]),value(row["area"]),value(row["volume"]),value(row["bounds"]),probes))
    parts.append(f'#[test]fn skew_{i}(){{check_skew({args});}}')
for i,row in enumerate(fixture["convexCases"]):
    counts="["+",".join(str(row[k]) for k in ("vertices","edges","faces"))+"]"
    sec=row["section"]
    cross_section="("+",".join(value(sec[k]) for k in ("origin","normal","area"))+")"
    args=",".join((stock(row["stock"]),stock(row["tool"]),counts,value(row["area"]),value(row["volume"]),value(row["bounds"]),cross_section))
    parts.append(f'#[test]fn multi_support_{i}(){{check_convex({args});}}')
for family,kind in [("empty","empty"),("unsupported","unsupported")]:
    for i,row in enumerate(fixture[family]):
        a,b=stock(row["stock"]),stock(row["tool"])
        witness=f'check_face_axes_overlap({a},{b});' if row["name"].startswith("edge-cross") else ""
        parts.append(f'#[test]fn {family}_{i}(){{{witness}check_refusal({a},{b},"{kind}");}}')
for index,positive in enumerate(fixture["overlappingUnion"]):
    sewn=[stock(positive["stock"]),stock(positive["tool"]),"["+",".join(map(str,positive["nativeTopology"]))+"]",value(positive["area"]),value(positive["volume"]),value(positive["bounds"])]
    parts.append(f"#[test]fn overlapping_skew_union_{index}(){{check_overlapping_union("+",".join(sewn)+");}")
union=fixture["separatedUnion"]
operands=fixture["empty"][union["reference"]]
parts.append(f'#[test]fn skew_union_is_a_compound_of_exact_placements(){{check_separated_union({stock(operands["stock"])},{stock(operands["tool"])});}}')
wide=next(kernel.rglob("📈️bounded-polynomial/🦀️.rs")).read_text(encoding="utf8").split("/// 🧭️",1)[1]
parts.append("mod bounded_polynomial{/// 🧭️"+wide+"}")
lin=next(kernel.rglob("📏️linear-distance/🦀️.rs")).read_text(encoding="utf8").split("type Vector",1)[1]
parts.append("mod linear_distance{use super::*;use crate::bounded_polynomial::Wide;type Vector"+lin+"}")
box=next(kernel.rglob("📦️box-distance/🦀️.rs")).read_text(encoding="utf8").split("type Placement",1)[1]
parts.append("mod distance_boxes{use super::*;use super::linear_distance::original_linear_distance;type Placement"+box+"}")
sat=next(kernel.rglob("📦️convex-box/🦀️.rs")).read_text(encoding="utf8").split("pub fn support_interval",1)[1].split("fn supports(",1)[0]
parts.append("mod distance_sat{use super::*;type BoxPlacement=(Pnt3,[Vec3;3],[f64;3]);pub fn support_interval"+sat+"}")
parts.append(r"""
fn box_gap(a:OrientedBox,b:OrientedBox)->f64{
 let (a,b)=(a.placement(),b.placement());
 if distance_sat::boxes_separated(a,b,0.0).unwrap(){distance_boxes::separated_box_distance(a,b).unwrap()}else{0.0}
}
""")
for family in ("cases","convexCases","empty"):
    for i,row in enumerate(fixture[family]):
        if "gap" not in row:continue
        a,b=stock(row["stock"]),stock(row["tool"])
        gap=value(row["gap"])
        parts.append(f'#[test]fn {family.lower()}_gap_{i}(){{assert!((box_gap({a},{b})-{gap}).abs()<1e-7);assert!((box_gap({b},{a})-{gap}).abs()<1e-7);}}')
difference=json.loads((flow/"🔣️original-skew-box-difference.json").read_text())
def cut_operands(row):
    reference=row.get("reference")
    if reference:
        family,index=reference
        resolved=(difference if row.get("reverse") else fixture)[family][index]
    else:
        resolved=row
    a,b=stock(resolved["stock"]),stock(resolved["tool"])
    return (b,a,resolved) if row.get("reverse") else (a,b,resolved)
parts.append(r"""
fn recess_center(a:OrientedBox,b:OrientedBox,expected:EVec3,volume:f64){
 let SolidBoundary::Hull(result)=cut_result(a,b).unwrap()else{unreachable!()};
 planar_classify::check_centroid([result].into_iter(),expected,volume);
}
fn compound_hulls(a:OrientedBox,b:OrientedBox)->impl Iterator<Item=HullBoundary>{
 let batch=BooleanShape::OrientedBox(a).disconnected(BooleanShape::OrientedBox(b),BooleanOp::Cut).unwrap();
 (0..batch.count()).map(move |index|{
  let SolidBoundary::Hull(member)=batch.member(index).unwrap()else{unreachable!()};
  member
 })
}
fn compound_center(a:OrientedBox,b:OrientedBox,expected:EVec3,volume:f64){
 planar_classify::check_centroid(compound_hulls(a,b),expected,volume);
}
""")
for i,row in enumerate(difference["cases"]):
    a,b,resolved=cut_operands(row)
    if row.get("connectedVoidSection"):
        origin,normal,areas=(value(row["connectedVoidSection"][key]) for key in ("origin","normal","ringAreas"))
        parts.append(f'#[test]fn connected_section_{i}(){{let SolidBoundary::Hull(shape)=cut_result({a},{b}).unwrap()else{{panic!("original connected shell")}};for axis in [{normal},{normal}.map(|axis|-axis)]{{planar_classify::check_void_section(&shape,{origin},axis,{areas});}}}}')
    if row["kind"]=="clip":
        probes="&["+",".join(f"({value(xyz)},{kind if kind==2 else 1-kind})" for xyz,kind in resolved["classify"])+"]"
        args=",".join((a,b,"BooleanOp::Cut",value(resolved["capOrigin"]),value([-x for x in resolved["capNormal"]]),
                       str(resolved["capVertices"]),value(row["area"]),value(row["volume"]),value(row["bounds"]),probes))
        body=f"check_skew({args});"
    elif row["kind"]=="cavity":
        probes="&["+",".join(f"({value(xyz)},{kind})" for xyz,kind in row["classify"])+"]"
        sec=row["section"]
        section="("+",".join(value(sec[k]) for k in ("origin","normal","area"))+")"
        void=row["voidSection"]
        void_section="("+",".join(value(void[k]) for k in ("origin","normal","ringAreas"))+")"
        body=f'check_cavity({a},{b},{value(row["area"])},{value(row["volume"])},{value(row["bounds"])},{probes},{section},{void_section});'
    elif row["kind"]=="recess":
        counts="["+",".join(map(str,row["nativeTopology"]))+"]"
        probes="&["+",".join(f"({value(xyz)},{kind})" for xyz,kind in row["classify"])+"]"
        sec=row.get("section")
        section="Some(("+",".join(value(sec[k]) for k in ("origin","normal","area"))+"))" if sec else "None"
        walls=len(row.get("cutterFaces",[0,2]))
        junctions=row.get("wallJunctions",walls*(walls-1)//2)
        body=f'check_recess({a},{b},{counts},{value(row["area"])},{value(row["volume"])},{value(row["bounds"])},{probes},{section},{walls},{junctions});'
    else:
        body=f"check_unchanged({a},{b});"
    parts.append(f'#[test]fn cut_{i}(){{{body}}}')
    if "center" in row:
        parts.append(f'#[test]fn cut_center_{i}(){{recess_center({a},{b},{value(row["center"])},{value(row["volume"])});}}')
    for key in ("vertexSection","edgeSection","coincidentFloor"):
        if key not in row: continue
        fact=row[key]; origin=value(fact["origin"]); normal=value(fact["normal"]); area=value(fact["area"])
        anchors="["+",".join(value(point) for point in fact.get("vertices",[fact["origin"]]))+"]"
        parts.append(f'#[test]fn cut_{key.lower()}_{i}(){{let SolidBoundary::Hull(body)=cut_result({a},{b}).unwrap()else{{unreachable!()}};for normal in [{normal},{normal}.map(|value|-value)]{{let ring=planar_classify::check_section(&body,{origin},normal,{area},{str(key!="coincidentFloor").lower()});for anchor in {anchors}{{assert!(ring.contains(&pnt(anchor)));}}}}}}')
for i,row in enumerate(difference["compounds"]):
    a,b,_=cut_operands(row)
    metrics="&["+",".join(f'({value(c["volume"])},{value(c["area"])},{c["euler"]})' for c in row["components"])+"]"
    parts.append(f'#[test]fn cut_compound_{i}(){{check_compound_cut({a},{b},{metrics});}}')
    parts.append(f'#[test]fn compound_center_{i}(){{compound_center({a},{b},{value(row["center"])},{value(row["volume"])});}}')
    probes="&["+",".join(f"({value(xyz)},{kind})" for xyz,kind in row["classify"])+"]"
    parts.append(f'#[test]fn compound_classify_{i}(){{planar_classify::check_compound_classification(compound_hulls({a},{b}),{probes});}}')
    parts.append(f'#[test]fn compound_bounds_{i}(){{planar_classify::check_compound_bounds(compound_hulls({a},{b}),{value(row["bounds"])});}}')
    section=row["handledSection"]
    origin,normal,area=(value(section[key]) for key in ("origin","normal","area"))
    parts.append(f'#[test]fn compound_section_{i}(){{let shape=compound_hulls({a},{b}).find(|shape|shape.ring_sizes().len()+shape.vertex_count()==shape.edge_count()).unwrap();for normal in [{normal},{normal}.map(|value|-value)]{{planar_classify::check_section(&shape,{origin},normal,{area},true);}}}}')
    nearest="&["+",".join(f"({value(p['probe'])},{value(p['point'])})" for p in row["nearest"])+"]"
    parts.append(f'#[test]fn compound_nearest_{i}(){{planar_classify::check_compound_nearest(compound_hulls({a},{b}),{nearest});}}')
for i,row in enumerate(difference["refused"]):
    a,b,_=cut_operands(row)
    parts.append(f'#[test]fn cut_refused_{i}(){{assert_eq!(cut_result({a},{b}).err().unwrap().0,"{row["kind"]}");}}')
nearest_owner=mass_owner.parent.parent/"🎯️nearest-analytic"
primitive_nearest=(nearest_owner/"🦀️.rs").read_text(encoding="utf8").split("pub fn primitive_nearest",1)[1].split("        SolidPrimitive::Torus",1)[0]
derived=(nearest_owner/"🐚️derived-solids/🦀️.rs").read_text(encoding="utf8").split("fn nearest(",1)[1]
parts.append(r"""
mod derived_nearest {
use super::*;
fn invalid()->ValueError{ValueError("invalid analytic point")}
fn check(point:Pnt3)->Result<(),ValueError>{point.to_array().into_iter().all(f64::is_finite).then_some(()).ok_or_else(invalid)}
fn surface_nearest(surface:&Surface,target:Pnt3)->Result<([f64;2],Pnt3,f64),ValueError>{
 let Surface::Sphere{frame,radius}=*surface else{unreachable!("derived boundary fixture uses a sphere")};
 let p=frame.to_local(target);
 let u=p.y.atan2(p.x);
 let v=p.z.atan2(p.x.hypot(p.y));
 let (su,cu)=u.sin_cos();
 let (sv,cv)=v.sin_cos();
 let point=frame.origin+(frame.x*cu+frame.y*su)*(radius*cv)+frame.z*(radius*sv);
 Ok(([u,v],point,point.distance(target)))
}
pub fn primitive_nearest"""+primitive_nearest+r"""_=>unreachable!("derived boundary fixture uses a sphere or box"),
    }
}
fn nearest("""+derived+"}")
derived_fixture=json.loads((mass_owner.parent.parent/"🧪️tests/🧫️fixtures/derived-solid-nearest.json").read_text(encoding="utf8"))
for name,row in derived_fixture.items():
    frame=lambda origin:f'Frame3{{origin:pnt({value(origin)}),..Frame3::WORLD}}'
    shape={
      "sphereShell":lambda:f'SphericalShell::from_spheres([{frame(row["outer"])},{frame(row["inner"])}],[{value(row["outerRadius"])},{value(row["innerRadius"])}],1e-6).unwrap()',
      "boxShell":lambda:f'SphericalShell::from_box(pnt({value(row["origin"])}),{value(row["size"])},{frame(row["inner"])},{value(row["innerRadius"])},1e-6).unwrap()',
      "band":lambda:f'SphericalCaps::from_band(Frame3::WORLD,{value(row["radius"])},{value(row["span"])},1e-6).unwrap()',
      "frustum":lambda:f'Frame3::WORLD,{value(row["radii"])},{value(row["height"])}',
    }[name]()
    method={"sphereShell":"spherical_shell","boxShell":"spherical_shell","band":"spherical_caps","frustum":"frustum"}[name]
    parts.append(f'#[test]fn derived_{name.lower()}_bounds(){{let range=derived_nearest::{method}_bounds({shape});for (actual,wanted) in range.into_iter().zip({value(row["bounds"])}){{assert!(actual.distance(pnt(wanted))<1e-10);}}}}')
    if "placed" in row:
        q=row["placed"]; x,y,z=(value(v) for v in q["axes"]); origin=value(q["origin"]); radii=value(row["radii"]); height=value(row["height"]); bounds=value(q["bounds"])
        parts.append(f'#[test]fn derived_placed_frustum_bounds(){{let f=Frame3{{origin:pnt({origin}),x:pnt({x}),y:pnt({y}),z:pnt({z})}};for (actual,wanted) in derived_nearest::frustum_bounds(f,{radii},{height}).into_iter().zip({bounds}){{assert!(actual.distance(pnt(wanted))<1e-10);}}}}')
    for i,probe in enumerate(row["queries"]):
        parts.append(f'#[test]fn derived_{name.lower()}_{i}(){{let p=derived_nearest::{method}_nearest({shape},pnt({value(probe["target"])})).unwrap();assert!(p.distance(pnt({value(probe["point"])}))<1e-10);}}')
if os.environ.get("SEMIO_TRACE_DISCONNECTED_CUT"):
    diagnostics={
        "*result.faces.get_mut(result.face_count)?":
            '*result.faces.get_mut(result.face_count).or_else(||{eprintln!("[DEBUG] face bound {}",result.face_count);None})?',
        "*result.points.get_mut(result.point_count)?":
            '*result.points.get_mut(result.point_count).or_else(||{eprintln!("[DEBUG] point bound {}",result.point_count);None})?',
        "*ring.get_mut(count)?=i as u8;":
            '*ring.get_mut(count).or_else(||{eprintln!("[DEBUG] ring bound {count}");None})?=i as u8;',
        "face(support,i,complete)??":
            'face(support,i,complete)?.or_else(||{eprintln!("[DEBUG] no wall {i}");None})?',
        "return None;}\n        perimeter+=length;":
            'eprintln!("[DEBUG] nonconvex patch {slot}");return None;}\n        perimeter+=length;',
    }
    if os.environ.get("SEMIO_TRACE_WIDE_CUT"):
        diagnostics["const BOUNDARY_FACES:usize=21;"]="const BOUNDARY_FACES:usize=42;"
        diagnostics["pub const BOX_BOUNDARY_VERTICES:usize=32;"]="pub const BOX_BOUNDARY_VERTICES:usize=64;"
    for old,new in diagnostics.items():
        parts=[part.replace(old,new) for part in parts]
    pair=fixture["convexCases"][1]
    a,b=stock(pair["stock"]),stock(pair["tool"])
    parts.append(f'#[test]fn trace_disconnected_cut(){{let a={a};let b={b};let plan=convex_box::box_recess_faces(a.placement(),b.placement(),1e-6);eprintln!("[DEBUG] cut plan {{:?}}",plan.map(|plan|(plan.point_count,plan.faces().len())));}}')
parts = [part.replace("semio_framework_3d_brep_construction::primitives::SectionPolygon", "crate::SectionPolygon").replace(
    "semio_framework_3d_brep_queries::planar_section::MAX_SECTION_REGIONS", "planar_classify::section_query::MAX_SECTION_REGIONS",
).replace("mod section_query {", "pub(super) mod section_query {") for part in parts]
parts.append(r"""
impl Live {
    fn insert_vacant_batch_reserved(&mut self, rows: &mut [Option<(String, Entity)>], grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if grant.maximum_items == 0 { return Err(ValueError("unfunded original index batch")); }
        for (i, row) in rows.iter().enumerate() {
            let (key, _) = row.as_ref().expect("batch row");
            if self.0.contains_key(key) || rows[..i].iter().flatten().any(|(name, _)| name == key) { return Err(ValueError("invalid original index batch")); }
        }
        for row in rows { let (key, entity) = row.take().unwrap(); self.0.insert(key, entity); }
        Ok(RetainedCloneProgress { copied_items: 1, ..Default::default() })
    }
}
""")
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    source=Path(folder)/"skew-box.rs"
    exe=source.with_suffix(".exe" if os.name=="nt" else "")
    source.write_text("\n".join(parts))
    for args in (("rustc","--edition=2024","--test","-Awarnings",str(source),"-o",str(exe)),(str(exe),"--nocapture")):
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",*args],check=True)
