//! 🦴️ Shared structural overlay meshes in world coordinates, using distinct geometry as well as colour.
use crate::{ModelInference, Point3};
use semio_framework_plugin::MeshData;
use semio_framework_value::DslValue;

fn stroke(a: Point3, b: Point3, radius: f64, color: [f32;4]) -> MeshData {
    let axis=[b.x-a.x,b.y-a.y,b.z-a.z];let length=axis.iter().map(|v|v*v).sum::<f64>().sqrt();
    if length<1e-9 {return MeshData::default();}
    let direction=axis.map(|v|v/length);let seed=if direction[2].abs()<0.9 {[0.0,0.0,1.0]}else{[1.0,0.0,0.0]};
    let cross=|a:[f64;3],b:[f64;3]|[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
    let raw=cross(direction,seed);let size=raw.iter().map(|v|v*v).sum::<f64>().sqrt();let u=raw.map(|v|v/size);let v=cross(direction,u);
    let mut positions=Vec::new();let mut normals=Vec::new();let mut colors=Vec::new();
    for p in [a,b] {for (x,y) in [(1.0,1.0),(-1.0,1.0),(-1.0,-1.0),(1.0,-1.0)] {for i in 0..3 {positions.push(([p.x,p.y,p.z][i]+radius*(x*u[i]+y*v[i]))as f32);normals.push(((x*u[i]+y*v[i])/2f64.sqrt())as f32);}colors.extend(color);}}
    MeshData{positions,normals,colors,indices:vec![0,2,1,0,3,2,4,5,6,4,6,7,0,1,5,0,5,4,1,2,6,1,6,5,2,3,7,2,7,6,3,0,4,3,4,7],..MeshData::default()}
}
/// 👁️ Structural member lines, dashed rigid links, support tripods and force arrows; ids and labels remain visible to scene inspection.
pub fn overlay(inference:&ModelInference, visible:impl Fn(&str)->bool) -> (Vec<DslValue>,Vec<DslValue>) {
    let mut meshes=Vec::new();let mut instances=Vec::new();
    let mut add=|id:String,name:String,a:Point3,b:Point3,radius:f64,color:[f32;4]|{let mesh=stroke(a,b,radius,color);if mesh.indices.is_empty(){return;}meshes.push(DslValue::object([("id".into(),DslValue::String(id.clone())),("data".into(),semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::from(mesh)))]));instances.push(DslValue::object([("id".into(),DslValue::String(id.clone())),("mesh".into(),DslValue::String(id)),("name".into(),DslValue::String(name)),("position".into(),DslValue::Array(vec![DslValue::float(0.0);3]))]));};
    for(id,member)in &inference.analytical_members {if !visible(id){continue;}for(index,pair)in member.path.windows(2).enumerate(){add(format!("analysis/{id}/{index}"),id.clone(),pair[0],pair[1],0.015,[0.0,0.45,0.7,1.0]);}}
    for(id,link)in &inference.structural_analysis.rigid_links {if !visible(&link.member)||!visible(&link.other){continue;}for index in 0..5 {let point=|t:f64|Point3{x:link.start.x+(link.end.x-link.start.x)*t,y:link.start.y+(link.end.y-link.start.y)*t,z:link.start.z+(link.end.z-link.start.z)*t};add(format!("rigid/{id}/{index}"),id.clone(),point(index as f64/5.0),point((index as f64+0.5)/5.0),0.01,[0.8,0.45,0.0,1.0]);}}
    for(id,support)in &inference.structural_analysis.supports {if !visible(&support.authored.member){continue;}for(index,p)in support.points.iter().enumerate(){for leg in 0..3 {let angle=leg as f64*std::f64::consts::TAU/3.0;add(format!("support/{id}/{index}/{leg}"),support.authored.name.clone(),*p,Point3{x:p.x+0.2*angle.cos(),y:p.y+0.2*angle.sin(),z:p.z-0.25},0.025,[0.0,0.6,0.45,1.0]);}}}
    for(id,load)in &inference.structural_analysis.loads {if !visible(&load.authored.member){continue;}let f=&load.authored.force;let size=(f.x*f.x+f.y*f.y+f.z*f.z).sqrt();if size<1e-9 {continue;}for(index,p)in load.points.iter().enumerate(){let start=Point3{x:p.x-f.x/size*0.6,y:p.y-f.y/size*0.6,z:p.z-f.z/size*0.6};add(format!("load/{id}/{index}"),load.authored.name.clone(),start,*p,0.025,[0.8,0.3,0.0,1.0]);for side in [-1.0,1.0]{add(format!("load-head/{id}/{index}/{side}"),load.authored.name.clone(),*p,Point3{x:p.x-f.x/size*0.15+side*0.08,y:p.y-f.y/size*0.15,z:p.z-f.z/size*0.15},0.02,[0.8,0.3,0.0,1.0]);}}}
    (meshes,instances)
}
