//! 🎨️ Canonical Lowpoly paint storage and compute extent.
pub const LOWPOLY_PAINT_TEXTURE_BYTES:usize=256*256*4;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PaintTextureError{pub actual_bytes:usize}
impl PaintTextureError{pub const CODE:&'static str="lowpoly.paint.invalid-texture";}
impl std::fmt::Display for PaintTextureError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}: {} bytes",Self::CODE,self.actual_bytes)}}
impl std::error::Error for PaintTextureError{}
impl From<PaintTextureError> for semio_framework_value::ValueError{fn from(error:PaintTextureError)->Self{Self::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())}}
pub fn validate_paint_pixels(pixels:&[u8])->Result<(),PaintTextureError>{if pixels.is_empty()||pixels.len()==LOWPOLY_PAINT_TEXTURE_BYTES{Ok(())}else{Err(PaintTextureError{actual_bytes:pixels.len()})}}
pub fn validate_compute_pixels(pixels:&[u8])->Result<(),PaintTextureError>{if pixels.len()==LOWPOLY_PAINT_TEXTURE_BYTES{Ok(())}else{Err(PaintTextureError{actual_bytes:pixels.len()})}}

pub fn validate_layer(layer:&crate::snapshot::LowpolyPaintLayer)->Result<(),semio_framework_value::ValueError>{layer.validate_pixels().map_err(Into::into)}
pub fn validate_layer_controlled(layer:&crate::snapshot::LowpolyPaintLayer,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_value::ValueError>{control.checkpoint()?;validate_layer(layer)}
