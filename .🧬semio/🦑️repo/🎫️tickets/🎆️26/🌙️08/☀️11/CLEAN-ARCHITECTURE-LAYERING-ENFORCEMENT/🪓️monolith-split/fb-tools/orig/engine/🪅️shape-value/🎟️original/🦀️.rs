//! 🎟️ Original analytic shape translation preserves actual root labels and the original source counter.
use super::{ShapeValue,ShapeRoot,Curve3,Surface,Body,LabelSource};
use semio_framework_3d_brep_vector::{matrix::Frame3, Pnt3};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
impl ShapeValue{
 /// ↔️ Quotes one fixed analytic root atom without traversing any body or free-form payload.
 pub fn analytic_translation_demand()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Self>(),depth:1,..Default::default()}}
 /// 🧳️ Retains the original source on every refusal and returns one true translated analytic root.
 pub fn translate_analytic_original(&self,offset:[f64;3],grant:RetainedCloneGrant)->Result<(Option<Self>,RetainedCloneProgress),ValueError>{
  let demand=Self::analytic_translation_demand();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok((None,Default::default()))}if offset.iter().any(|value|!value.is_finite()){return Err(invalid())}
  let body=&self.body;if [body.vertices.slot_count(),body.edges.slot_count(),body.coedges.slot_count(),body.loops.slot_count(),body.faces.slot_count(),body.shells.slot_count(),body.solids.slot_count(),body.curves3.slot_count(),body.curves2.slot_count(),body.surfaces.slot_count()].iter().any(|count|*count!=0){return Err(unsupported())}
  let root=match &self.root{
   ShapeRoot::Curve{curve,label}=>{let curve=match curve{Curve3::Line{origin,dir}=>{if [dir.x,dir.y,dir.z].iter().any(|value|!value.is_finite()){return Err(invalid())}Curve3::Line{origin:point(*origin,offset)?,dir:*dir}},Curve3::Circle{frame,radius}=>Curve3::Circle{frame:translated_frame(frame,offset)?,radius:positive(*radius)?},Curve3::Ellipse{frame,major_radius,minor_radius}=>Curve3::Ellipse{frame:translated_frame(frame,offset)?,major_radius:positive(*major_radius)?,minor_radius:positive(*minor_radius)?},_=>return Err(unsupported())};ShapeRoot::Curve{curve,label:*label}},
   ShapeRoot::Surface{surface,label}=>{let surface=match surface{Surface::Plane{frame}=>Surface::Plane{frame:translated_frame(frame,offset)?},Surface::Cylinder{frame,radius}=>Surface::Cylinder{frame:translated_frame(frame,offset)?,radius:positive(*radius)?},Surface::Cone{frame,half_angle}=>{if !half_angle.is_finite(){return Err(invalid())}Surface::Cone{frame:translated_frame(frame,offset)?,half_angle:*half_angle}},Surface::Sphere{frame,radius}=>Surface::Sphere{frame:translated_frame(frame,offset)?,radius:positive(*radius)?},Surface::Torus{frame,major_radius,minor_radius}=>Surface::Torus{frame:translated_frame(frame,offset)?,major_radius:positive(*major_radius)?,minor_radius:positive(*minor_radius)?},_=>return Err(unsupported())};ShapeRoot::Surface{surface,label:*label}},
   _=>return Err(unsupported()),
  };
  let mut body=Body::new();body.labels=LabelSource::from_next(self.body.labels.next());Ok((Some(Self{root,body}),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original analytic translation requires finite shape coordinates and parameters")}
fn unsupported()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original BRep translation has no funded arena or rational-root producer")}
fn positive(value:f64)->Result<f64,ValueError>{if !value.is_finite()||value<=0.0{return Err(invalid())}Ok(value)}
fn point(original:Pnt3,offset:[f64;3])->Result<Pnt3,ValueError>{let point=Pnt3::new(original.x+offset[0],original.y+offset[1],original.z+offset[2]);if [point.x,point.y,point.z].iter().any(|value|!value.is_finite()){return Err(invalid())}Ok(point)}
fn translated_frame(original:&Frame3,offset:[f64;3])->Result<Frame3,ValueError>{if [original.x.x,original.x.y,original.x.z,original.y.x,original.y.y,original.y.z,original.z.x,original.z.y,original.z.z].iter().any(|value|!value.is_finite()){return Err(invalid())}let mut frame=*original;frame.origin=point(original.origin,offset)?;Ok(frame)}
