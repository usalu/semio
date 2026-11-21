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
source=output.getvalue().replace("#[derive(Clone,Copy,PartialEq)] struct Vec3", "#[derive(Clone,Copy,PartialEq,Default)] struct Vec3").replace("enum SolidPrimitive{Box", "enum SolidPrimitive{Sphere{frame:Frame3,radius:f64},Cone{frame:Frame3,radius:f64,height:f64},Box")
source=source.replace("fn volume(&self)->f64 {match *self{Self::Box", "fn volume(&self)->f64 {match *self{Self::Sphere{radius,..}=>4.*std::f64::consts::PI*radius.powi(3)/3.,Self::Cone{radius,height,..}=>std::f64::consts::PI*radius.powi(2)*height/3.,Self::Box")
polygon=(brep/"📸️representation/🔖️contract/✂️box-section/🦀️.rs").read_text()
polygon=polygon[polygon.index("const MAX_PROFILE:"):polygon.index("/// 🎟️ Intersects one certified stock edge")]
owner=next(brep.rglob("📐️conic-section/🦀️.rs")).read_text()
owner=owner[owner.index("pub fn conic_section("):]
open_conic=next(brep.rglob("📈️open-conic/🦀️.rs")).read_text().split("\n",2)[2]
owner="mod open_conic{"+open_conic+"}"+owner

owner=owner.replace("ValueError::literal(ValueRefusalKind::InvalidValue,", "ValueError(").replace("ValueError::literal(ValueRefusalKind::UnsupportedOwner,", "ValueError(")
fixture=next(Path("✏️s/🔌️plugins/🌀️procedural").rglob("🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json"))
rows=json.loads(fixture.read_text())["retainedConicSection"]

def array(xs):
    return "["+",".join(array(x) if isinstance(x,list) else str(float(x)) for x in xs)+"]"

checks=r"""
use conic_face_draft::{Conic,ConicBoundary,SectionArc};
fn section_case(source:SolidPrimitive,plane:EVec3,normal:EVec3,center:EVec3,radii:[f64;2],area:f64,refusals:[(EVec3,EVec3);2]){
 let (frame,rim,boundary)=conic_section(&source,pnt(plane),pnt(normal)).unwrap();
 assert_eq!(boundary.members(),1);
 assert!((frame.origin-pnt(center)).norm()<1e-12);
 assert!((frame.z-pnt(normal).normalized().unwrap()).norm()<1e-12);
 let actual=rim.radii().unwrap();
 assert!((actual[0]-radii[0]).abs()<1e-12);
 assert!((actual[1]-radii[1]).abs()<1e-12);
 assert!((std::f64::consts::PI*actual[0]*actual[1]-area).abs()<1e-12);
 for step in 0..64{
  let t=f64::from(step)*std::f64::consts::TAU/64.;
  let point=frame.origin+frame.x*(actual[0]*t.cos())+frame.y*(actual[1]*t.sin());
  assert!((point-pnt(plane)).dot(frame.z).abs()<1e-12);
  match source{
   SolidPrimitive::Sphere{frame,radius}=>assert!(((point-frame.origin).norm()-radius).abs()<1e-12),
   SolidPrimitive::Cylinder{frame,radius,height}=>{
    let relative=point-frame.origin;
    let offset=relative.dot(frame.z);
    assert!(offset>0.&&offset<height);
    assert!(((relative-frame.z*offset).norm()-radius).abs()<1e-12);
   }
   _=>unreachable!()
  }
 }
 for (origin,normal) in refusals{
  assert!(conic_section(&source,pnt(origin),pnt(normal)).is_err());
 }
 let (_,reversed,_)=conic_section(&source,pnt(plane),pnt(normal)*-1.).unwrap();
 assert_eq!(reversed.radii().unwrap(),actual);
}
#[test] fn exact_closed_sections_match_neutral_corpus(){
"""
for row in rows:
    frame=f'Frame3::from_normal(pnt({array(row["origin"])}),Vec3::Z).unwrap()'
    radius=float(row["radius"])
    if row["kind"]=="cylinder":
        ctor=f'SolidPrimitive::Cylinder{{frame:{frame},radius:{radius},height:{float(row["height"])}}}'
    else:
        ctor=f'SolidPrimitive::Sphere{{frame:{frame},radius:{radius}}}'
    refusals="["+",".join(f'({array(test["origin"])},{array(test["normal"])})' for test in row["refusals"])+"]"
    args=",".join(array(row[key]) for key in ["planeOrigin","normal","center","radii"])
    checks+=f'section_case({ctor},{args},{float(row["area"])},{refusals});\n'
checks+="}\n"

checks+=r"""
fn axial_case(origin:EVec3,radius:f64,height:f64,plane:EVec3,normals:[EVec3;2],corners:[EVec3;4],area:f64,misses:[EVec3;2]){
 let frame=Frame3::from_normal(pnt(origin),Vec3::Z).unwrap();
 let source=SolidPrimitive::Cylinder{frame,radius,height};
 for n in normals{
  let normal=pnt(n);
  let points=axial_cylinder_section(&source,pnt(plane),normal).unwrap().unwrap();
  let signed=(points[1]-points[0]).cross(points[3]-points[0]).dot(normal);
  assert!((signed-area).abs()<1e-12);
  assert!(corners.into_iter().all(|q|points.into_iter().any(|p|(p-pnt(q)).norm()<1e-12)));
  for miss in misses{assert!(axial_cylinder_section(&source,pnt(miss),normal).is_err());}
  let mut registry=BrepRegistry::default();
  let mut face=registry.plan_polygon_face_points(4).unwrap();
  let input=points.map(|p|p.to_array());
  for _ in 0..256{
   if face.topology_ready(){break;}
   advance(&mut registry,&mut face,&input).unwrap();
  }
  assert!(face.topology_ready());
  let face_id=registry.body.faces.0.iter().position(Option::is_some).unwrap();
  let native=registry.body.faces.get(face_id).unwrap();
  let Surface::Plane{frame}=registry.body.surfaces.get(native.surface).unwrap()else{panic!("expected a plane");};
  assert!((normal-if native.flipped{-frame.z}else{frame.z}).norm()<1e-12);
  close(&mut registry,&mut face);
  assert!(registry.body.faces.0.iter().all(Option::is_none));
  assert!(registry.body.edges.0.iter().all(Option::is_none));
 }
 assert!(axial_cylinder_section(&source,pnt(plane),Vec3::Z).unwrap().is_none());
}
#[test]fn axial_section_builds_and_cancels_signed_line_faces(){
"""
for row in json.loads(fixture.read_text())["retainedAxialCylinderSection"]:
    args=[array(row["origin"]),str(float(row["radius"])),str(float(row["height"])),
          *(array(row[key]) for key in ["planeOrigin","normals","corners"]),
          str(float(row["area"])),array(row["refusedPlaneOrigins"])]
    checks+="axial_case("+",".join(args)+");\n"
checks+="}\n"

checks+=r"""
fn profile_case(kind:&str,origin:EVec3,radii:[f64;2],height:f64,plane:EVec3,normal:EVec3,corner_list:&[EVec3],area:f64,expected:Option<(EVec3,[f64;2])>){
 use conic_face_draft::ConicFacePublication;
 let frame=Frame3::from_normal(pnt(origin),Vec3::Z).unwrap();
 let (frame,conic,boundary)=match kind{
  "cone"=>conic_section(&SolidPrimitive::Cone{frame,radius:radii[0],height},pnt(plane),pnt(normal)),
  "cylinder"=>conic_section(&SolidPrimitive::Cylinder{frame,radius:radii[0],height},pnt(plane),pnt(normal)),
  _=>frustum_section(frame,radii,height,pnt(plane),pnt(normal)),
 }.unwrap();
 if let Some((center,axes))=expected{
  assert!((frame.origin-pnt(center)).norm()<1e-11);
  assert!(conic.radii().unwrap().into_iter().zip(axes).all(|(a,b)|(a-b).abs()<1e-11));
 }
 let n=corner_list.len().max(1);
 assert_eq!(boundary.members(),n);
 assert!((frame.z-pnt(normal).normalized().unwrap()).norm()<1e-12);
 let mut body=Body::default();
 let mut face=ConicFacePublication::new(1,frame,conic,boundary,false,1,[b'f';64]);
 for _ in 0..80{
  if face.topology_ready(){break;}
  let mut grant=face.topology_demand(&body).unwrap();
  assert_eq!(face.step(&mut body,&mut RetainedAllocationEffect(&grant)).unwrap().copied_items,0);
  grant.maximum_items=1;
  let receipt=face.step(&mut body,&mut RetainedAllocationEffect(&grant)).unwrap();
  assert!(receipt.fits(grant));
  assert_eq!(receipt.copied_items,1);
 }
 assert!(face.topology_ready());
 let face_id=face.face_id();
 let ring=body.loops.get(body.faces.get(face_id).unwrap().outer.unwrap()).unwrap();
 let mut cursor=ring.first;
 let mut observed=Vec::new();
 let mut twice_area=0.;
 for i in 0..n{
  let trim=body.coedges.get(cursor).unwrap();
  let edge=body.edges.get(trim.edge).unwrap();
  let next=body.coedges.get(trim.next).unwrap();
  assert_eq!(next.prev,cursor);
  assert_eq!(edge.v1,body.edges.get(next.edge).unwrap().v0);
  assert!(trim.forward&&edge.range==trim.prange);
  let curve=body.curves3.get(edge.curve).unwrap();
  let uv=body.curves2.get(trim.pcurve.unwrap()).unwrap();
  let (a,b)=edge.range;
  for fraction in [0.,0.3,0.7,1.]{
   let t=a+(b-a)*fraction;
   let p=curve.eval(t);
   let delta=p-pnt(origin);
   let q=uv.eval(t);
   assert!((p-(frame.origin+frame.x*q.x+frame.y*q.y)).norm()<1e-12);
   assert!((p-pnt(plane)).dot(frame.z).abs()<1e-12);
   assert!(delta.z>=-1e-12&&delta.z<=height+1e-12);
   if i%2==0{
    assert!(matches!(curve,Curve3::Ellipse{..}|Curve3::Circle{..}|Curve3::Nurbs{..}));
    let radius=radii[0]+(radii[1]-radii[0])*delta.z/height;
    assert!((delta.x.hypot(delta.y)-radius).abs()<1e-11);
   }else{
    assert!(matches!(curve,Curve3::Line{..}));
    assert!(delta.z.abs()<1e-12||(delta.z-height).abs()<1e-12);
   }
  }
  let start=uv.eval(a);
  let end=uv.eval(b);
  let cross=|p:Pnt2,q:Pnt2|p.x*q.y-p.y*q.x;
  twice_area+=if i%2!=0{cross(start,end)}else if let Some([a_radius,b_radius])=conic.radii(){
   a_radius*b_radius*(b-a)
  }else{
   let mut previous=start;
   (1..=8192).map(|index|{
    let current=uv.eval(a+(b-a)*f64::from(index)/8192.);
    let area=cross(previous,current);
    previous=current;area
   }).sum()
  };
  observed.push(body.vertices.get(edge.v0).unwrap().position);
  cursor=trim.next;
 }
 assert_eq!(cursor,ring.first);
 assert!(corner_list.iter().all(|p|observed.iter().any(|q|(*q-pnt(*p)).norm()<1e-12)));
 assert!((twice_area/2.-area).abs()<if conic.radii().is_some(){1e-10}else{1e-5});
 for _ in 0..80{
  if face.terminal_is_empty(){break;}
  let mut grant=face.close_demand(&body).unwrap();
  grant.maximum_items=1;
  assert!(face.close(&mut body,&mut RetainedAllocationEffect(&grant)).unwrap().receipt().fits(grant));
 }
 assert!(face.terminal_is_empty());
 assert_eq!(body.counts(),[0;8]);
}
#[test]fn capped_cylinder_section_keeps_native_trim_parameters_and_ring_incidence(){
"""
for row in json.loads(fixture.read_text())["retainedClippedCylinderSection"]:
    args=['"cylinder"',array(row["origin"]),array([row["radius"]]*2),str(float(row["height"])),array(row["planeOrigin"]),
          array(row["normal"]),"&"+array(row["capCorners"]),str(float(row["area"])),"None"]
    checks+="profile_case("+",".join(args)+");\n"
checks+="}\n"

checks+='\n#[test] fn retained_elliptic_cone_and_frustum_sections_preserve_exact_contours(){\n'
for row in json.loads(fixture.read_text())["retainedConicalSections"]:
    args=[json.dumps(row["kind"]),array(row["origin"]),array(row["radii"]),str(float(row["height"])),
          array(row["planeOrigin"]),array(row["normal"]),"&"+array(row["capCorners"]),str(float(row["area"])),
          "Some(("+array(row["center"])+","+array(row["sectionRadii"])+"))"]
    checks+="profile_case("+",".join(args)+");\n"
checks+=r"""
 let frame=Frame3::from_normal(Pnt3::new(0.,0.,0.),Vec3::Z).unwrap();
 let cone=SolidPrimitive::Cone{frame,radius:4.,height:8.};
 assert!(conic_section(&cone,Pnt3::new(0.,0.,8.),Vec3::Z).is_err());
 assert!(conic_section(&cone,Pnt3::new(0.,0.,8.),Pnt3::new(2.,0.,1.)).is_err());
 assert!(frustum_section(frame,[1.25,3.25],5.,Pnt3::new(0.,0.,6.),Vec3::Z).is_err());
}
"""

checks+='\n#[test] fn retained_open_conics_store_exact_rational_quadratic_curves_and_trims(){\n'
for key in ["retainedParabolicSections","retainedHyperbolicSections"]:
    for row in json.loads(fixture.read_text())[key]:
        args=[json.dumps(row["kind"]),array(row["origin"]),array(row["radii"]),str(float(row["height"])),
              array(row["planeOrigin"]),array(row["normal"]),"&"+array(row["capCorners"]),str(float(row["area"])),"None"]
        checks+="profile_case("+",".join(args)+");\n"
checks+="}\n"

checks+=r"""
#[test] fn quadratic_cancellation_releases_only_funded_original_arrays(){
 use conic_face_draft::ConicFacePublication;
 let source=SolidPrimitive::Cone{frame:Frame3::WORLD,radius:4.,height:8.};
 let (frame,support,boundary)=conic_section(&source,Pnt3::new(0.,0.,3.),Pnt3::new(3.,0.,1.)).unwrap();
 let mut draft=ConicFacePublication::new(1,frame,support,boundary,false,1,[b'c';64]);
 let mut body=Body::default();
 let mut obtained=0;
 for _ in 0..80{
  let mut quote=draft.topology_demand(&body).unwrap();
  assert!(quote.copy_bytes<=72);
  quote.maximum_items=1;
  let receipt=draft.step(&mut body,&mut RetainedAllocationEffect(&quote)).unwrap();
  obtained+=receipt.retained_capacity_bytes;
  if obtained>0{break;}
 }
 assert_eq!(obtained,48);
 assert!(body.curves3.0.iter().all(Option::is_none));
 let mut denied=draft.close_demand(&body).unwrap();
 assert_eq!(denied.release_bytes,obtained);
 denied.maximum_items=1;
 denied.release_bytes=obtained-1;
 assert_eq!(draft.close(&mut body,&mut RetainedAllocationEffect(&denied)).unwrap().receipt().copied_items,0);
 let mut returned=0;
 for _ in 0..80{
  if draft.terminal_is_empty(){break;}
  let mut quote=draft.close_demand(&body).unwrap();
  quote.maximum_items=1;
  let progress=draft.close(&mut body,&mut RetainedAllocationEffect(&quote)).unwrap().receipt();
  assert!(progress.fits(quote));
  returned+=progress.released_bytes;
 }
 assert!(draft.terminal_is_empty());
 assert_eq!(returned,obtained);
 assert_eq!(body.counts(),[0;8]);
}
"""

checks+=r"""
fn apex_case(origin:EVec3,radii:[f64;2],height:f64,plane:EVec3,normal:EVec3,corners:&[EVec3],area:f64){
 let frame=Frame3::from_normal(pnt(origin),Vec3::Z).unwrap();
 let polygon=apex_section(frame,radii,height,None,pnt(plane),pnt(normal)).unwrap().unwrap();
 assert_eq!(polygon.count(),corners.len());
 assert!(apex_section(frame,radii,height,None,pnt(plane),Vec3::Z).is_err());
 let slope=(radii[1]-radii[0])/height;
 assert!(apex_section(frame,radii,height,None,pnt(plane),Pnt3::new(1.,0.,slope.abs())).is_err());
 if radii[1]>0. {
  let owner_slope=(radii[1]-radii[0]).atan2(height).tan();
  let native_apex=frame.origin-frame.z*(radii[0]/owner_slope);
  assert!(apex_section(frame,radii,height,Some(native_apex),native_apex,pnt(normal)).unwrap().is_some());
 }
 assert!(apex_section(frame,radii,height,None,pnt(plane)+Vec3::X,pnt(normal)).unwrap().is_none());
 let points=(0..polygon.count()).map(|i|polygon.point(i).unwrap()).collect::<Vec<_>>();
 for (&actual,&expected) in points.iter().zip(corners){
  assert!((pnt(actual)-pnt(expected)).norm()<1e-12);
 }
 let mut matching_edges=0;
 let mut ordered_area=0.;
 let reference=pnt(points[0]);
 let n=pnt(normal).normalized().unwrap();
 for i in 0..points.len(){
  let a=pnt(points[i]);let b=pnt(points[(i+1)%points.len()]);
  ordered_area+=(a-reference).cross(b-reference).dot(n)/2.;
  if a.z!=b.z {
   matching_edges+=1;
   let p=(a+b)*0.5-pnt(origin);
   let r=radii[0]+(radii[1]-radii[0])*p.z/height;
   assert!((p.x.hypot(p.y)-r).abs()<1e-12);
  }
 }
 assert_eq!(matching_edges,2);
 assert!((ordered_area-area).abs()<1e-10);
 let mut registry=BrepRegistry::default();
 let mut face=registry.plan_polygon_face_points(points.len()).unwrap();
 for _ in 0..256{
  if face.topology_ready(){break;}
  advance(&mut registry,&mut face,&points).unwrap();
 }
 assert!(face.topology_ready());
 for curve in registry.body.curves3.0.iter().flatten(){assert!(matches!(curve,Curve3::Line{..}));}
 close(&mut registry,&mut face);
 assert_eq!(registry.body.counts(),[0;8]);
}
#[test]fn apex_sections_keep_exact_straight_generators_and_grants(){
"""

for row in json.loads(fixture.read_text())["retainedApexSections"]:
    args=[array(row["origin"]),array(row["radii"]),str(float(row["height"])),array(row["planeOrigin"]),
          array(row["normal"]),"&"+array(row["points"]),str(float(row["area"]))]
    checks+="apex_case("+",".join(args)+");\n"
checks+="}\n"

strip_fixture=json.loads((brep/"💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/spherical-strip-section.json").read_text())
strip_code=(brep/"💡️queries/✂️planar-section/🌗️spherical-strip/🦀️.rs").read_text().split("type CircularStrip", 1)[1]
strip_code="mod spherical_strip{use super::*;use std::f64::consts::TAU;type CircularStrip"+strip_code.replace("Pnt3::from_array(", "pnt(").replace("Vec3::from_array(", "pnt(")+"}"
checks+=r"""
fn spherical_case(authored_radius:f64,span:[f64;2],plane:EVec3,normal:EVec3,n:usize,area:f64){
 use conic_face_draft::ConicFacePublication;
 for normal in [normal,normal.map(|axis|-axis)]{
  let sphere=Frame3::from_normal(pnt([0.;3]),Vec3::Z).unwrap();
  let (frame,radius,arcs)=spherical_strip::spherical_strip_section(sphere,authored_radius,span,plane,normal,1e-6).unwrap();
  let boundary=arcs[0].map_or(ConicBoundary::FULL,|first|ConicBoundary::clipped(first,arcs[1]));
  let mut face=ConicFacePublication::new(1,frame,Conic::Circle(radius),boundary,false,1,[b'g';64]);
  let mut body=Body::default();
  for _ in 0..96{
   if face.topology_ready(){break;}
   let mut grant=face.topology_demand(&body).unwrap();grant.maximum_items=1;
   assert!(face.step(&mut body,&mut RetainedAllocationEffect(&grant)).unwrap().fits(grant));
  }
  assert!(face.topology_ready());
  assert_eq!(body.coedges.len(),n);
  let loop_id=body.faces.get(face.face_id()).unwrap().outer.unwrap();
  let first=body.loops.get(loop_id).unwrap().first;
  let mut current=first;
  let mut twice_area=0.;
  for _ in 0..n{
   let row=body.coedges.get(current).unwrap();
   assert_eq!(body.coedges.get(row.next).unwrap().prev,current);
   let edge=body.edges.get(row.edge).unwrap();
   let curve=body.curves3.get(edge.curve).unwrap();
   let trim=body.curves2.get(row.pcurve.unwrap()).unwrap();
   let [a,b]=[row.prange.0,row.prange.1].map(|parameter|trim.eval(parameter));
   for parameter in [row.prange.0,(row.prange.0+row.prange.1)*0.5,row.prange.1]{
    let point=curve.eval(parameter);
    let uv=trim.eval(parameter);
    assert!((point-(frame.origin+frame.x*uv.x+frame.y*uv.y)).norm()<1e-12);
    assert!(point.z>=span[0]-1e-10&&point.z<=span[1]+1e-10);
    if matches!(curve,Curve3::Circle{..}){assert!((point.norm()-authored_radius).abs()<1e-10);}
    else{assert!(span.into_iter().any(|bound|(point.z-bound).abs()<1e-10));}
   }
   twice_area+=if matches!(curve,Curve3::Circle{..}){radius*radius*(row.prange.1-row.prange.0)}else{a.x*b.y-a.y*b.x};
   current=row.next;
  }
  assert_eq!(current,first);
  assert!((twice_area/2.-area).abs()<1e-9);
  for _ in 0..96{
   if face.terminal_is_empty(){break;}
   let mut grant=face.close_demand(&body).unwrap();grant.maximum_items=1;
   assert!(face.close(&mut body,&mut RetainedAllocationEffect(&grant)).unwrap().receipt().fits(grant));
  }
  assert!(face.terminal_is_empty());
  assert_eq!(body.counts(),[0;8]);
 }
}
#[test]fn spherical_supports_publish_native_circle_arcs_and_closure_chords(){
"""
for row in strip_fixture["cases"]:
    args=[str(float(strip_fixture["sphereRadius"])),array(row["axial"]),array(row["plane"]),array(row["normal"]),
          str(max(1,2*row["arcs"])),str(float(row["area"]))]
    checks+="spherical_case("+",".join(args)+");\n"
checks+="}\n"

with TemporaryDirectory(dir=ticket/"🗑️generated") as folder:
    src=Path(folder)/"section.rs"
    src.write_text("\n".join([source,polygon,owner,strip_code,checks]),encoding="utf-8")
    exe=src.with_suffix(".exe" if os.name=="nt" else "")
    command=["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20"]
    subprocess.run(command+["rustc","--edition=2024","--test","-Awarnings",str(src),"-o",str(exe)],check=True)
    subprocess.run(command+[str(exe)],check=True)
