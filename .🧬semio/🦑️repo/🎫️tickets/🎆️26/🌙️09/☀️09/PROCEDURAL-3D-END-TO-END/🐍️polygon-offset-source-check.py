"""Compile and exercise the retained production polygon offset cursor."""
import contextlib
import io
import json
from pathlib import Path
import runpy

here = Path(__file__).parent
text = io.StringIO()
with contextlib.redirect_stdout(text):
    runpy.run_path(str(here / "🐍️point-face-source-check.py"))
print(text.getvalue().replace("HelixWireRecipe,PolygonWirePublication","PolygonWirePublication"))
print(r"""
fn start_source(points:&[EVec3])->(BrepRegistry,String){
 let mut k=BrepRegistry::default();
 let mut p=k.plan_polygon_face_points(points.len()).unwrap();
 for _ in 0..500{
  if p.topology_ready(){break;}
  advance(&mut k,&mut p,points).unwrap();
 }
 let handle=p.handle().to_owned();
 let grant=RetirementDemand{maximum_items:1,..Default::default()};
 assert!(k.publish_polygon_face(&mut p,handle.clone(),grant).is_ok());
 close(&mut k,&mut p);
 (k,handle)
}

fn face_id(k:&BrepRegistry,key:&str)->FaceId {
 let Entity::Face(id)=k.live.0[key] else{panic!("expected face")};
 id
}
fn ring(k:&BrepRegistry,key:&str,count:usize)->Vec<(usize,EVec3)> {
 let face=k.body.faces.get(face_id(k,key)).unwrap();
 let first=k.body.loops.get(face.outer.unwrap()).unwrap().first;
 let mut cursor=first;
 let mut out=vec![];
 for _ in 0..count{
  let c=k.body.coedges.get(cursor).unwrap();
  let e=k.body.edges.get(c.edge).unwrap();
  let v=k.body.vertices.get(if c.forward{e.v0}else{e.v1}).unwrap();
  out.push((cursor,evec(v.position)));
  cursor=c.next;
 }
 assert_eq!(cursor,first);
 out
}

fn offset_step(k:&mut BrepRegistry,p:&mut PolygonFacePublication,source:&str){
 assert!(p.next_point_index().is_none());
 let mut grant=k.polygon_face_topology_demand(p,Some(source)).unwrap();
 assert!(grant.copy_bytes<=128);
 let idle=k.step_polygon_face_topology(p,Some(source),None,&mut RetainedAllocationEffect(&grant)).unwrap();
 assert_eq!(idle.copied_items,0);
 grant.maximum_items=1;
 let work=k.step_polygon_face_topology(p,Some(source),None,&mut RetainedAllocationEffect(&grant)).unwrap();
 assert_eq!(work.copied_items,1);
 assert!(work.fits(grant));
}

fn verify_offset(points:&[EVec3],normal:EVec3,distance:f64,expected:&[EVec3],flip:bool){
 let (mut k,source)=start_source(points);
 let original=k.body.counts();
 let old_ring=ring(&k,&source,points.len());
 let id=face_id(&k,&source);
 k.body.faces.get_mut(id).unwrap().flipped=flip;
 if flip {
   for &(coedge,_) in &old_ring {
     let c=k.body.coedges.get_mut(coedge).unwrap();
     c.forward=false;
     let edge=k.body.edges.get_mut(c.edge).unwrap();
     std::mem::swap(&mut edge.v0,&mut edge.v1);
   }
 }
 let mut draft=k.plan_polygon_face_offset(&source,distance).unwrap().unwrap();
 for _ in 0..500{
  if draft.topology_ready(){break;}
  assert_eq!(k.live.0.len(),1);
  offset_step(&mut k,&mut draft,&source);
 }
 assert!(draft.topology_ready());
 let key=draft.handle().to_owned();
 let grant=RetirementDemand{maximum_items:1,..Default::default()};
 assert!(k.publish_polygon_face(&mut draft,key.clone(),grant).is_ok());
 close(&mut k,&mut draft);
 assert_eq!(k.unpublished_topology,0);
 assert_eq!(k.live.0.len(),2);
 assert_eq!(k.body.counts(),original.map(|n|n*2));
 assert_eq!(ring(&k,&source,points.len()),old_ring);
 let result=k.body.faces.get(face_id(&k,&key)).unwrap();
 assert_eq!(result.flipped,flip);
 let Surface::Plane{frame}=k.body.surfaces.get(result.surface).unwrap() else{panic!("expected plane")};
 assert!((frame.z-pnt(normal)).norm()<1e-12);
 let source_position=ring(&k,&source,points.len());
 for (((_,actual),reference),(_,before)) in ring(&k,&key,points.len()).into_iter().zip(expected).zip(source_position){
   let target=if flip { pnt(before)-pnt(normal)*distance } else { pnt(*reference) };
   assert!((pnt(actual)-target).norm()<1e-12);
 }
}

#[test]fn translated_face_preserves_winding_and_original_source(){
""")

fixture=next(Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json"))
data=json.loads(fixture.read_text())["polygonOffset"]
def vec(xs):
    return "[" + ",".join(str(float(x)) for x in xs) + "]"
def pts(rows):
    return "[" + ",".join(vec(row) for row in rows) + "]"
for row in data:
    print(f'verify_offset(&{pts(row["points"])},{vec(row["normal"])},{row["distance"]},&{pts(row["offsetPoints"])},false);')
    print(f'verify_offset(&{pts(row["points"])},{vec(row["normal"])},{row["distance"]},&{pts(row["offsetPoints"])},true);')

print(r"""
}
#[test]fn offset_cancel_or_revision_change_keeps_the_only_source(){
 for changed in [false,true]{
  let (mut k,source)=start_source(&CONCAVE);
  let before=k.body.counts();
  let loop_before=ring(&k,&source,CONCAVE.len());
  let mut draft=k.plan_polygon_face_offset(&source,0.5).unwrap().unwrap();
  for _ in 0..500{
   if k.body.coedges.len()>before[6]{break;}
   offset_step(&mut k,&mut draft,&source);
  }
  assert!(k.body.coedges.len()>before[6]);
  if changed{
   k.shape_mutation_revision+=1;
   assert!(k.polygon_face_topology_demand(&draft,Some(&source)).is_err());
  }
  close(&mut k,&mut draft);
  assert_eq!(k.unpublished_topology,0);
  assert_eq!(k.live.0.len(),1);
  assert_eq!(k.body.counts(),before);
  assert_eq!(ring(&k,&source,CONCAVE.len()),loop_before);
 }
}
""")
