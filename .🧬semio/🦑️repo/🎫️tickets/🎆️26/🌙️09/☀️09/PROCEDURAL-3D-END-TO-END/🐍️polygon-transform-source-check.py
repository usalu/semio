"""Exercise the production affine planar polygon and line-pcurve stages."""
import contextlib
import io
import json
from pathlib import Path
import runpy

here=Path(__file__).parent
raw=io.StringIO()
with contextlib.redirect_stdout(raw):
    runpy.run_path(str(here/"🐍️polygon-offset-source-check.py"))
print(raw.getvalue())
print(r"""
fn verify_map(points:&[EVec3],linear:[EVec3;3],delta:EVec3,expected:&[EVec3],normal:EVec3,flipped:bool,area:f64){
 let (mut k,source)=start_source(points);
 let source_ring=ring(&k,&source,points.len());
 let transform=AnalyticTransform(Affine3{linear:linear.map(pnt),delta:pnt(delta)});
 let mut draft=k.plan_polygon_face_transform(&source,transform).unwrap().unwrap();
 for _ in 0..500{
  if draft.topology_ready(){break;}
  assert_eq!(k.live.0.len(),1);
  offset_step(&mut k,&mut draft,&source);
 }
 assert!(draft.topology_ready());
 let target=draft.handle().to_owned();
 let grant=RetirementDemand{maximum_items:1,..Default::default()};
 assert!(k.publish_polygon_face(&mut draft,target.clone(),grant).is_ok());
 close(&mut k,&mut draft);
 assert_eq!(k.unpublished_topology,0);
 assert_eq!(k.live.0.len(),2);
 assert_eq!(ring(&k,&source,points.len()),source_ring);
 let result=k.body.faces.get(face_id(&k,&target)).unwrap();
 assert_eq!(result.flipped,flipped);
 let Surface::Plane{frame}=k.body.surfaces.get(result.surface).unwrap() else{panic!("expected plane")};
 let oriented=frame.z.normalized().unwrap()*if flipped{-1.}else{1.};
 assert!((oriented-pnt(normal).normalized().unwrap()).norm()<1e-12);
 let target_ring=ring(&k,&target,points.len());
 let computed_area=(pnt(target_ring[1].1)-pnt(target_ring[0].1)).cross(pnt(target_ring[2].1)-pnt(target_ring[0].1)).norm()*0.5;
 assert!((computed_area-area).abs()<1e-12);
 for ((id,actual),reference) in target_ring.into_iter().zip(expected) {
   assert!((pnt(actual)-pnt(*reference)).norm()<1e-12);
   let coedge=k.body.coedges.get(id).unwrap();
   let edge=k.body.edges.get(coedge.edge).unwrap();
   let Curve3::Line{origin,dir}=k.body.curves3.get(edge.curve).unwrap() else{panic!("expected line")};
   let Curve2::Line{origin:uv,dir:duv}=k.body.curves2.get(coedge.pcurve.unwrap()).unwrap();
   for t in [0.,0.5,1.]{
     let planar=frame.origin+frame.x*(uv.x+duv.x*t)+frame.y*(uv.y+duv.y*t);
     assert!((planar-(*origin+*dir*t)).norm()<1e-12);
   }
 }
}

#[test]fn exact_affine_planar_faces(){
""")
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json"))
data=json.loads(fixture.read_text())["polygonTransforms"]
matrices=[
    ([[1,0,0],[0,1,0],[0,0,1]],[2,-3,0.5]),
    ([[0,-1,0],[1,0,0],[0,0,1]],[0,0,0]),
    ([[1,0,0],[0,0,-1],[0,1,0]],[0,2,0]),
    ([[2,0,0],[0,3,0],[0,0,0.5]],[-1,-2,0.5]),
    ([[-1,0,0],[0,1,0],[0,0,1]],[4,0,0]),
    ([[1,0,0],[0,1,0],[0,0,1]],[0,0,0]),
]
def vec(rows):
   return "["+",".join(str(float(x)) for x in rows)+"]"
def pts(rows):
   return "["+",".join(vec(row) for row in rows)+"]"
for row,(matrix,delta) in zip(data["cases"],matrices):
   flipped=str(row["flipped"]).lower()
   print(f'verify_map(&{pts(data["source"])},{pts(matrix)},{vec(delta)},&{pts(row["vertices"])},{vec(row["normal"])},{flipped},{row["area"]});')
print(r"""
}
#[test]fn cancel_scaled_face_after_coedge_keeps_source(){
 let points=[[1.,1.,1.],[3.,1.,2.],[1.,3.,2.]];
 let (mut k,source)=start_source(&points);
 let before=k.body.counts();
 let transform=AnalyticTransform(Affine3{
   linear:[Vec3::new(2.,0.,0.),Vec3::new(0.,3.,0.),Vec3::new(0.,0.,0.5)],
   delta:Vec3::ZERO,
 });
 let mut draft=k.plan_polygon_face_transform(&source,transform).unwrap().unwrap();
 for _ in 0..500{
   if k.body.coedges.len()>before[6]{break;}
   offset_step(&mut k,&mut draft,&source);
 }
 assert!(k.body.coedges.len()>before[6]);
 close(&mut k,&mut draft);
 assert_eq!(k.body.counts(),before);
 assert_eq!(k.live.0.len(),1);
 assert_eq!(k.unpublished_topology,0);
}
""")
