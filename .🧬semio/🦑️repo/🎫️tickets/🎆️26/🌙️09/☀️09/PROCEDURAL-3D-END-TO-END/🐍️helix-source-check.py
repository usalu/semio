"""Exercise the production helix sampler and retained wire publication under one-event grants."""
import contextlib
import io
import json
from pathlib import Path
import runpy

here = Path(__file__).parent
capture = io.StringIO()
with contextlib.redirect_stdout(capture):
    runpy.run_path(str(here / "🐍️point-face-source-check.py"))
preamble = capture.getvalue().split("fn advance(kernel:", 1)[0]
print(preamble.replace("use polygon_face_publication::PolygonFacePublication;",""))
print(r"""
fn plan(case: (EVec3,EVec3,f64,f64,f64)) -> HelixWireRecipe{
 let (origin,axis,radius,pitch,turns)=case;
 HelixWireRecipe::new(origin,axis,radius,pitch,turns).unwrap()
}
fn advance(kernel:&mut BrepRegistry,wire:&mut PolygonWirePublication){
 let mut d=kernel.polygon_wire_demand(wire,128).unwrap();
 assert!(d.copy_bytes<=128);
 let idle=kernel.step_polygon_wire_topology(wire,None,&mut RetainedAllocationEffect(&d)).unwrap();
 assert_eq!(idle.copied_items,0);
 d.maximum_items=1;
 let progress=kernel.step_polygon_wire_topology(wire,None,&mut RetainedAllocationEffect(&d)).unwrap();
 assert_eq!(progress.copied_items,1);
 assert!(progress.fits(d));
}
fn verify(case: (EVec3,EVec3,f64,f64,f64),owned:usize,total:f64,witness:&[(usize,EVec3)]){
 let recipe=plan(case);
 assert_eq!(recipe.sample_count(),owned+1);
 let mut kernel=BrepRegistry::default();
 let mut draft=kernel.plan_polygon_wire(PolygonWireRecipe::Helix(recipe)).unwrap();
 for _ in 0..500{
   if draft.topology_ready(){break;}
   assert!(kernel.live.0.is_empty());
   advance(&mut kernel,&mut draft);
 }
 let handle=draft.handle().to_owned();
 let grant=RetirementDemand{maximum_items:1,..Default::default()};
 assert!(kernel.publish_polygon_wire(&mut draft,handle.clone(),grant).is_ok());
 assert!(draft.terminal_is_empty());
 assert_eq!(kernel.live.0.len(),1);
 assert_eq!(kernel.unpublished_topology,0);
 let Entity::Wire(wire,_)=&kernel.live.0[&handle] else{panic!("expected wire")};
 assert!(!wire.closed);
 assert_eq!(wire.members.len(),owned);
 assert_eq!(wire.vertices.len(),owned+1);
 let nodes:Vec<_>=wire.vertices.iter().map(|&id|kernel.body.vertices.get(id).unwrap().position).collect();
 for (at,expected) in witness{
   assert!((nodes[*at]-pnt(*expected)).norm()<1e-12);
   assert!((nodes[*at]-pnt(recipe.sample(*at))).norm()<1e-12);
 }
 let length:f64=nodes.windows(2).map(|edge|(edge[1]-edge[0]).norm()).sum();
 assert!((length-total).abs()<1e-12);
}
#[test]fn published_wire_preserves_sampled_helix(){
""")

fixture = next(Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json"))
data=json.loads(fixture.read_text())["originalHelix"]
def vec(p):
    return "[" + ",".join(str(float(v)) for v in p) + "]"
for row in data:
    scalars=",".join(str(float(row[key])) for key in ("radius", "pitch", "turns"))
    witness=",".join(f"({i},{vec(p)})" for i,p in row["samples"])
    print(f'verify(({vec(row["origin"])},{vec(row["axis"])},{scalars}),{row["segments"]},{row["length"]},&[{witness}]);')

print(r"""
}
#[test]fn cancelling_helix_retires_the_private_topology(){
 let recipe=plan(([0.;3],[0.,0.,1.],1.,2.,1.));
 let mut kernel=BrepRegistry::default();
 let mut draft=kernel.plan_polygon_wire(PolygonWireRecipe::Helix(recipe)).unwrap();
 for _ in 0..70{
   if kernel.body.edges.len()>1{break;}
   advance(&mut kernel,&mut draft);
 }
 assert!(kernel.body.edges.len()>1);
 assert!(kernel.live.0.is_empty());
 for _ in 0..200{
   if draft.terminal_is_empty(){break;}
   let mut d=kernel.polygon_wire_close_demand(&draft).unwrap();
   d.maximum_items=1;
   let r=kernel.close_polygon_wire(&mut draft,&mut RetainedAllocationEffect(&d)).unwrap().receipt();
   assert_eq!(r.copied_items,1);assert!(r.fits(d));
 }
 assert!(draft.terminal_is_empty());
 assert_eq!(kernel.body.counts(),[0;8]);
 assert_eq!(kernel.unpublished_topology,0);
}
""")
