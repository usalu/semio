use semio_s_artifact_norm_iso16757_model::part_2::*;
use std::collections::BTreeMap;
use semio_s_artifact_norm_iso16757_model::part_2::solid;
fn prim(k:&str,p:&[(&str,f64)])->GeometryNode{GeometryNode::Primitive{kind:k.into(),parameters:p.iter().map(|(a,b)|(a.to_string(),*b)).collect::<BTreeMap<_,_>>()}}
fn bx(w:f64,h:f64,d:f64)->GeometryNode{prim("box",&[("width",w),("height",h),("depth",d)])}
fn cy(r:f64,h:f64)->GeometryNode{prim("cylinder",&[("radius",r),("height",h)])}
fn sp(r:f64)->GeometryNode{prim("sphere",&[("radius",r)])}
fn tf(t:[f64;3],r:[f64;3],c:GeometryNode)->GeometryNode{GeometryNode::Transform{translation:t,rotation_deg:r,child:Box::new(c)}}
fn bo(o:BooleanOperator,c:Vec<GeometryNode>)->GeometryNode{GeometryNode::Boolean{operator:o,children:c}}
fn main(){
 let cat=GeometryCatalogue::default(); use BooleanOperator::*;
 let cases:Vec<(&str,GeometryNode)>=vec![
 ("box-1x2x3",bx(1.,2.,3.)),("cylinder-r1-h2",cy(1.,2.)),("sphere-r1.5",sp(1.5)),
 ("box-2x1x1-rot45z",tf([0.;3],[0.,0.,45.],bx(2.,1.,1.))),
 ("union-apart",bo(Union,vec![bx(1.,1.,1.),tf([5.;3],[0.;3],bx(1.,1.,1.))])),
 ("inter-offset",bo(Intersection,vec![bx(1.,1.,1.),tf([0.5;3],[0.;3],bx(1.,1.,1.))])),
 ("diff-half",bo(Difference,vec![bx(1.,1.,1.),tf([0.;3],[0.;3],bx(0.5,1.,1.))])),
 ("union-spheres-overlap",bo(Union,vec![sp(1.),tf([1.,0.,0.],[0.;3],sp(1.))])),
 ("cyl-minus-box-hole",bo(Difference,vec![cy(1.,2.),tf([0.5,-0.5,0.5],[0.;3],bx(1.,3.,1.))])),
 ("rot-boxes-union",bo(Union,vec![bx(2.,1.,1.),tf([1.,0.5,0.],[0.,0.,45.],bx(2.,1.,1.))])),
 ("cyl-z90-inter-sphere",bo(Intersection,vec![tf([1.,0.,0.],[0.,0.,90.],cy(0.5,2.)),sp(1.)])),("cyl-x90",tf([0.;3],[90.,0.,0.],cy(0.5,3.))),
 ];
 for (n,g) in cases{let v=solid::solid_volume(&g,&cat).unwrap();let b=solid::bounding_box(&g,&cat).unwrap();
  println!("{n}\t{:.12}\t{:.2e}\t{}\t{:?} {:?}",v.volume_m3,v.error_m3,v.exact,b.min,b.max);}
}
