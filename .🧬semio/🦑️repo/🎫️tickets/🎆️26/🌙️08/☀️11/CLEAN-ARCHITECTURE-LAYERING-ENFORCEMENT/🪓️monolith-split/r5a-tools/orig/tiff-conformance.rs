//! 🛡️ Baseline conformance of owned image interpretation.
use crate::schema::snapshot::*;
use semio_framework_diagnostic::{Diagnostic,FaultCode,FaultScope,Severity,TextSpan};
pub const CODE_DEGENERATE_RASTER:&str="stdio.tiff.baseline.degenerate-raster";
pub const CODE_NO_IFD:&str="stdio.tiff.baseline.no-ifd";
pub const CODE_UNSUPPORTED_PHOTOMETRIC:&str="stdio.tiff.baseline.unsupported-photometric";
pub const CODE_UNSUPPORTED_BITS_PER_SAMPLE:&str="stdio.tiff.baseline.unsupported-bits-per-sample";
fn soft(code:&'static str,message:String)->Diagnostic{Diagnostic{code:FaultCode::new(code),severity:Severity::Warning,span:TextSpan::at(1,1),message,expected:None,scope:FaultScope::default()}}
pub fn check_tiff_baseline_conformance(snapshot:&TiffSnapshot)->Vec<Diagnostic>{check_tiff_baseline_conformance_with(snapshot,&mut|_,_|Ok(())).expect("unbounded owned conformance")}
pub fn check_tiff_baseline_conformance_with(snapshot:&TiffSnapshot,checkpoint:&mut dyn FnMut(usize,usize)->Result<(),semio_framework_value::ValueError>)->Result<Vec<Diagnostic>,semio_framework_value::ValueError>{
 checkpoint(0,0)?;let mut out=Vec::new();let Some(ifd)=snapshot.ifds.first() else{out.push(soft(CODE_NO_IFD,"no owned image page".into()));return Ok(out)};
 if ifd.integer(TAG_IMAGE_WIDTH).unwrap_or(0)==0||ifd.integer(TAG_IMAGE_LENGTH).unwrap_or(0)==0||ifd.blocks.is_empty(){out.push(soft(CODE_DEGENERATE_RASTER,"image page has no logical raster extent".into()))}
 let total=ifd.entries.len();for(index,tag)in ifd.entries.iter().enumerate(){if index%256==0{checkpoint(index,total)?}if tag.tag==TAG_PHOTOMETRIC&&tag.values.first_u32().is_some_and(|v|v>3){out.push(soft(CODE_UNSUPPORTED_PHOTOMETRIC,"photometric interpretation is outside baseline 0..3".into()))}if tag.tag==TAG_BITS_PER_SAMPLE{let values=ifd.integers(TAG_BITS_PER_SAMPLE);if values.iter().any(|n|!matches!(n,1|4|8)){out.push(soft(CODE_UNSUPPORTED_BITS_PER_SAMPLE,"sample precision is outside baseline 1/4/8".into()))}}}checkpoint(total,total)?;Ok(out)
}
