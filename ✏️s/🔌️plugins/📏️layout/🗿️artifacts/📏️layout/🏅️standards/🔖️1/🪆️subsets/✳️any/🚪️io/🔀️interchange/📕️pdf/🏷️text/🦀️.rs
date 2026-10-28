//! 🏷️ An explicit application marker identifies positioned invisible text authored for native recovery.
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;
pub const PDF_LAYOUT_TEXT_TAG:&str="SemioLayoutText";
#[derive(Debug)]
pub(super) struct MarkedTextScope<'a>{depth:usize,marker_depth:Option<usize>,styles:[Option<&'a[PdfDictEntry]>;256],paint:u8}
impl<'a> Default for MarkedTextScope<'a>{fn default()->Self{Self{depth:0,marker_depth:None,styles:[None;256],paint:0}}}
impl<'a> MarkedTextScope<'a>{
 pub(super) fn begin(&mut self,tag:&str)->Result<(),String>{self.begin_with_style(tag,None)}
 pub(super) fn begin_with_style(&mut self,tag:&str,entries:Option<&'a[PdfDictEntry]>)->Result<(),String>{if self.depth>=256{return Err("layout.pdf.marked-content-depth".into());}let inherited=if self.depth==0{None}else{self.styles[self.depth-1]};self.styles[self.depth]=if tag==PDF_LAYOUT_TEXT_TAG{entries.or(inherited)}else{inherited};self.depth+=1;if tag==PDF_LAYOUT_TEXT_TAG&&self.marker_depth.is_none(){self.marker_depth=Some(self.depth);self.paint=0;}Ok(())}
 pub(super) fn end(&mut self)->Result<(),String>{if self.depth==0{return Err("layout.pdf.marked-content-stack".into());}if self.marker_depth==Some(self.depth){self.marker_depth=None;self.paint=0;}self.depth-=1;self.styles[self.depth]=None;Ok(())}
 pub(super) fn active(&self)->bool{self.marker_depth.is_some()}
 pub(super) fn entries(&self)->Option<&'a[PdfDictEntry]>{if self.active(){self.styles[self.depth-1]}else{None}}
 pub(super) fn record_promotion(&mut self,editable:bool){if self.active(){self.paint=if editable&&self.paint!=2{1}else{2};}}
 pub(super) fn owns_paint(&self)->bool{self.active()&&self.paint==1}
 pub(super) fn finish(&self)->Result<(),String>{if self.depth==0{Ok(())}else{Err("layout.pdf.marked-content-stack".into())}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// 📐 Only uniform orthogonal authored text matrices map to native font shape.
pub(super) fn editable_matrix(m:[f64;6],scale:f64,authored:bool)->bool{let[a,b,c,d,_,_]=m;let sx=a.hypot(b);let sy=c.hypot(d);m.iter().all(|x|x.is_finite())&&scale.is_finite()&&scale>0.0&&sx>0.0&&sy>0.0&&a*d-b*c>0.0&&(a*c+b*d).abs()<=1e-8*sx*sy&&(sx-sy).abs()<=1e-8*sx.max(sy)&&(authored||(scale-1.0).abs()<=1e-8)}
