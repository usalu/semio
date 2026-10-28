//! 📐️ Native page/frame coordinates become physical STEP millimetres with a page Y flip.
use super::*;
pub(super) const MM:f64=25.4/72.0;
pub(super) fn position(b:&LayoutBounds,page_height:f64,x:f64,y:f64)->[f64;3]{let(sin,cos)=b.rotation.sin_cos();let dx=x-b.width*0.5;let dy=y-b.height*0.5;[(b.x+b.width*0.5+cos*dx-sin*dy)*MM,(page_height-b.y-b.height*0.5-sin*dx-cos*dy)*MM,0.0]}
pub(super) fn outline(b:&LayoutBounds,height:f64)->[[f64;3];5]{[[0.0,0.0],[b.width,0.0],[b.width,b.height],[0.0,b.height],[0.0,0.0]].map(|[x,y]|position(b,height,x,y))}
/// 🔲️ Recovers native centre rotation from a closed orthogonal XY rectangle in points.
pub(super) fn rectangle_bounds(points:&[[f64;3]],min_x:f64,max_y:f64)->Option<LayoutBounds>{if points.len()!=5||points.iter().flatten().any(|value|!value.is_finite()){return None;}let close=|a:[f64;3],b:[f64;3]|a.into_iter().zip(b).all(|(a,b)|(a-b).abs()<1e-7);if !close(points[0],points[4])||points.iter().any(|point|(point[2]-points[0][2]).abs()>1e-7){return None;}let mut a=points[1];let mut c=points[3];let mut u=[a[0]-points[0][0],a[1]-points[0][1]];let mut v=[c[0]-points[0][0],c[1]-points[0][1]];if u[0]*v[1]-u[1]*v[0]>0.0{std::mem::swap(&mut a,&mut c);std::mem::swap(&mut u,&mut v);}let width=u[0].hypot(u[1]);let height=v[0].hypot(v[1]);if width<1e-9||height<1e-9||(u[0]*v[0]+u[1]*v[1]).abs()>width*height*1e-7||(points[2][0]-points[0][0]-u[0]-v[0]).abs()>1e-7||(points[2][1]-points[0][1]-u[1]-v[1]).abs()>1e-7{return None;}Some(LayoutBounds{x:(points[0][0]+points[2][0])*0.5-min_x-width*0.5,y:max_y-(points[0][1]+points[2][1])*0.5-height*0.5,width,height,rotation:(-u[1]).atan2(u[0])})}
/// 🔤️ Recovers an editable glyph-height frame with its baseline at the standard text origin.
pub(super) fn annotation_bounds(position:[f64;3],x_axis:[f64;3],y_axis:[f64;3],width:f64,height:f64,min_x:f64,max_y:f64)->LayoutBounds{let x=position[0]+x_axis[0]*width*0.5+y_axis[0]*height*0.5;let y=position[1]+x_axis[1]*width*0.5+y_axis[1]*height*0.5;LayoutBounds{x:x-min_x-width*0.5,y:max_y-y-height*0.5,width,height,rotation:-x_axis[1].atan2(x_axis[0])}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
