//! 🖍️ PDF current paths retain transformed cubic geometry without flattening.
use semio_framework_io_ports::{VectorPoint2,VectorSegment};
#[derive(Default)]
pub(super) struct PdfPath{segments:Vec<VectorSegment>,current:Option<VectorPoint2>,start:Option<VectorPoint2>}
impl PdfPath{
 fn point(m:[f64;6],x:f64,y:f64,height:f64,unit:f64)->Result<VectorPoint2,String>{let p=VectorPoint2{x:(m[0]*x+m[2]*y+m[4])*unit,y:height-(m[1]*x+m[3]*y+m[5])*unit};if p.x.is_finite()&&p.y.is_finite(){Ok(p)}else{Err("layout.pdf.path-non-finite".into())}}
 fn push(&mut self,s:VectorSegment)->Result<(),String>{if self.segments.len()>=16_384{return Err("layout.pdf.path-segment-limit".into());}self.segments.push(s);Ok(())}
 pub(super) fn move_to(&mut self,m:[f64;6],x:f64,y:f64,h:f64,u:f64)->Result<(),String>{let to=Self::point(m,x,y,h,u)?;self.push(VectorSegment::MoveTo{to})?;self.current=Some(to);self.start=Some(to);Ok(())}
 pub(super) fn line_to(&mut self,m:[f64;6],x:f64,y:f64,h:f64,u:f64)->Result<(),String>{if self.current.is_none(){return Err("layout.pdf.path-current-point".into());}let to=Self::point(m,x,y,h,u)?;self.push(VectorSegment::LineTo{to})?;self.current=Some(to);Ok(())}
 pub(super) fn cubic(&mut self,m:[f64;6],p:[f64;6],h:f64,u:f64)->Result<(),String>{if self.current.is_none(){return Err("layout.pdf.path-current-point".into());}let c1=Self::point(m,p[0],p[1],h,u)?;let c2=Self::point(m,p[2],p[3],h,u)?;let to=Self::point(m,p[4],p[5],h,u)?;self.push(VectorSegment::CubicTo{c1,c2,to})?;self.current=Some(to);Ok(())}
 pub(super) fn initial(&mut self,m:[f64;6],p:[f64;4],h:f64,u:f64)->Result<(),String>{let c1=self.current.ok_or("layout.pdf.path-current-point")?;let c2=Self::point(m,p[0],p[1],h,u)?;let to=Self::point(m,p[2],p[3],h,u)?;self.push(VectorSegment::CubicTo{c1,c2,to})?;self.current=Some(to);Ok(())}
 pub(super) fn final_curve(&mut self,m:[f64;6],p:[f64;4],h:f64,u:f64)->Result<(),String>{self.cubic(m,[p[0],p[1],p[2],p[3],p[2],p[3]],h,u)}
 pub(super) fn close(&mut self)->Result<(),String>{if self.current.is_some(){self.push(VectorSegment::Close)?;self.current=self.start;}Ok(())}
 pub(super) fn rectangle(&mut self,m:[f64;6],p:[f64;4],h:f64,u:f64)->Result<(),String>{let[x,y,w,height]=p;self.move_to(m,x,y,h,u)?;self.line_to(m,x+w,y,h,u)?;self.line_to(m,x+w,y+height,h,u)?;self.line_to(m,x,y+height,h,u)?;self.close()}
 pub(super) fn take(&mut self)->Vec<VectorSegment>{self.current=None;self.start=None;std::mem::take(&mut self.segments)}
 pub(super) fn is_empty(&self)->bool{self.segments.is_empty()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
