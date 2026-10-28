//! 📐️ A qualified original page Drawing supplies the unchanged axis of one actual artboard mutation.
use super::super::PatchPage;
use super::source::OriginalPageDrawingSource;
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::RetainedCloneRef};
use semio_s_artifact_draw_drawing_model::DrawingArtboard;
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Page Drawing dimensions require finite positive original artboard values")}
pub fn from_original(command:RetainedCloneRef<'_,PatchPage>,source:&OriginalPageDrawingSource)->Result<DrawingArtboard,ValueError>{let canvas=source.canvas().ok_or_else(refusal)?;dimensions(&command.get().field,&command.get().value,canvas.width,canvas.height)}
fn dimensions(field:&str,value:&str,width:f64,height:f64)->Result<DrawingArtboard,ValueError>{let number=value.parse::<f64>().map_err(|_|refusal())?;let(width,height)=match field{"pageDrawingWidth"=>(number,height),"pageDrawingHeight"=>(width,number),_=>return Err(refusal())};if !width.is_finite()||!height.is_finite()||width<=0.0||height<=0.0{return Err(refusal())}Ok(DrawingArtboard{width,height})}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;
