"""✂️ Verify the retained clipped-box BRep, ordered caps and exact convex distance."""
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
flow=Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit")
kernel=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
fixture=json.loads((flow/"🔣️original-oblique-box-split.json").read_text())
parts=runpy.run_path(str(ticket/"🐍️coaxial-bore-source-check.py"))["parts"]
source=(ticket/"🐍️rotated-box-intersection-source-check.py").read_text()
parts.append(source[source.index("fn rotation_z("):source.index("fn check_boundary(")])
boundary=next(kernel.rglob("✂️clip-boundary/🦀️.rs")).read_text().split("pub(super) fn edge",1)[1]
parts.append("mod clip_boundary{use super::*;pub(super) fn edge"+boundary+"}")
parts.append(r"""
fn rotate_and_split(size:EVec3,base:EVec3,angle:f64,anchor:EVec3,normal:EVec3,expected:[(f64,f64,[EVec3;2]);2]){
 let rotation=rotation_z(angle);
 let origin=pnt(base)+rotation.apply_vector(pnt(anchor));
 let normal=rotation.apply_vector(pnt(normal)).normalized().unwrap();
 let source=rotated_box(size,angle,base);
 let oriented=match source {
  BooleanShape::OrientedBox(source)=>source,
  BooleanShape::Analytic(shape)=>OrientedBox::from_primitive(shape).unwrap(),
  _=>unreachable!(),
 };
 let mut previous=None;
 for sign in [1.,-1.]{
  let n=normal*sign;
  let pair=oriented.split(origin.to_array(),n.to_array()).unwrap();
  let [a,b]=pair.map(|part|match part{SolidBoundary::ClippedBox(shape,proof)=>(shape,proof),_=>unreachable!()});
  let mut reflected_vertices=Vec::new();
  let mut total=0.;
""")
parts.append(r"""
  let halves=[(a.0,a.1,b.1),(b.0,b.1,a.1)];
  for (index,(shape,proof,opposite)) in halves.into_iter().enumerate(){
   let (expected_volume,expected_area,expected_bounds)=expected[if sign>0.{index}else{1-index}];
   let actual=proof.bounds().unwrap().map(Pnt3::to_array);
   for (obtained,wanted) in actual.into_iter().flatten().zip(expected_bounds.into_iter().flatten()){
    assert!((obtained-wanted).abs()<1e-10);
   }
   let center=(0..shape.vertex_count()).fold(pnt([0.;3]),|sum,v|sum+shape.vertex(v))*(1./shape.vertex_count() as f64);
   assert!(proof.signed_boundary_distance(center).unwrap()<0. && opposite.signed_boundary_distance(center).unwrap()>0.);
   let orientation=if index==0{1.}else{-1.};
   for i in 0..shape.vertex_count(){assert!((shape.vertex(i)-origin).dot(n)*orientation>=-1e-10);}
   let mut incidence=[[0;2];15];
   let (mut volume,mut area)=(0.,0.);
   let (mut cap,mut cap_normal)=(Vec::new(),pnt([0.;3]));
   let cap_index=shape.ring_sizes().len()-1;
   for (face,&count) in shape.ring_sizes().iter().enumerate(){
    let frame=shape.frame(face);
    let mut polygon=[pnt([0.;3]);6];
    for i in 0..count{
     let trim=clip_boundary::trim(&shape,face,i);
     let edge=clip_boundary::edge(&shape,trim.edge);
     let (first,last)=(edge.vertices[usize::from(!trim.forward)],edge.vertices[usize::from(trim.forward)]);
     incidence[trim.edge][usize::from(trim.forward)]+=1;
     polygon[i]=shape.vertex(first);
     let next=clip_boundary::trim(&shape,face,(i+1)%count);
     assert_eq!(last,clip_boundary::edge(&shape,next.edge).vertices[usize::from(!next.forward)]);
     for t in [0.,0.39,1.]{
      let uv=trim.curve.eval(t);
      assert!((frame.origin+frame.x*uv.x+frame.y*uv.y-edge.curve.eval(t)).norm()<1e-11);
     }
    }
    for i in 1..count-1{
     let [a,b,c]=[polygon[0],polygon[i],polygon[i+1]];
     let normal=(b-a).cross(c-a);
     assert!(normal.dot(frame.z)>0.);
     area+=normal.norm()/2.;
     volume+=(a-center).dot((b-center).cross(c-center))/6.;
    }
    if face==cap_index{
     cap=polygon[..count].iter().map(|p|{
      assert!((*p-origin).dot(n).abs()<1e-11);
      assert!(proof.signed_boundary_distance(*p).unwrap().abs()<1e-10);
      p.to_array()
     }).collect();
     cap.sort_unstable_by(|a,b|a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])).then(a[2].total_cmp(&b[2])));
     cap_normal=frame.z;
    }
   }
   assert!((cap_normal+n*orientation).norm()<1e-12);
   assert!(incidence[..shape.edge_count()].iter().all(|uses|*uses==[1,1]));
   assert_eq!(shape.vertex_count()+shape.ring_sizes().len(),shape.edge_count()+2);
   assert!((volume-expected_volume).abs()<1e-10);
   assert!((area-expected_area).abs()<1e-10);
   total+=volume;
   reflected_vertices.push(cap);
  }
  assert_eq!(reflected_vertices[0],reflected_vertices[1]);
  if let Some(vertices)=&previous {assert_eq!(vertices,&reflected_vertices[0]);}
  previous=Some(reflected_vertices.remove(0));
  assert!((total-size.into_iter().product::<f64>()).abs()<1e-10);
 }
}
#[test]fn exact_oblique_box_split_keeps_two_outward_support_shells(){
""")
def array(items):
    if isinstance(items,list):
        return "["+",".join(array(item) for item in items)+"]"
    return repr(float(items))
for row in fixture["cases"]:
    facts=",".join(f'({float(side["volume"])},{float(side["area"])},{array(side["bounds"])})' for side in row["halves"])
    parts.append(f'rotate_and_split({array(row["size"])},{array(row["origin"])},{row["rotationDegrees"]}_f64.to_radians(),{array(row["point"])},{array(row["normal"])},[{facts}]);')
parts.append(r"""
}
#[test]fn exact_convex_clipping_refuses_contact_and_measures_projected_edges(){
 let source=OrientedBox::from_primitive(box_shape([0.;3],[2.;3])).unwrap();
""")
for row in fixture["refused"]:
    parts.append(f'assert!(source.split({array(row["point"])},{array(row["normal"])}).is_err());')
parts.append(r"""
 let [_,SolidBoundary::ClippedBox(_,piece)]=source.split([0.4;3],[1.;3]).unwrap()else{unreachable!();};
 let exact=piece.signed_boundary_distance(pnt([2.;3])).unwrap();
 assert!((exact-1.6*3_f64.sqrt()).abs()<1e-12);
 let exact=piece.signed_boundary_distance(pnt([1.4,1.6,0.2])).unwrap();
 assert!((exact-1.66_f64.sqrt()).abs()<1e-12);
}
""")
generated=ticket/"🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as folder:
    rust=Path(folder)/"oblique-box-split.rs"
    rust.write_text("\n".join(parts))
    exe=rust.with_suffix(".exe" if os.name=="nt" else "")
    for args in (("rustc","--edition=2024","--test","-Awarnings",str(rust),"-o",str(exe)),(str(exe),)):
        subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20",*args],check=True)
