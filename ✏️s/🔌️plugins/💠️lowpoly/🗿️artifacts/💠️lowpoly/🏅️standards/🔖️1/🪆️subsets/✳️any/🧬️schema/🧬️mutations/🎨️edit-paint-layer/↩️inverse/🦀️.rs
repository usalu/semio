//! ↩️ Restores exact pre-edit octets after validating canonical layer and run bounds.
use super::EditPaintLayer;
use semio_s_artifact_lowpoly_lowpoly_model::{PixelRun,snapshot::LowpolySnapshot,paint_texture::LOWPOLY_PAINT_TEXTURE_BYTES};
pub fn inverse<Op:From<EditPaintLayer>>(payload:&EditPaintLayer,base:&LowpolySnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{
 let Some(layer)=base.objects.iter().find(|object|object.id==payload.object_id).and_then(|object|object.paint_layers.get(payload.layer_index))else{return Ok(Vec::new())};layer.validate_pixels()?;
 let mut inverse_runs=Vec::with_capacity(payload.runs.len());for run in &payload.runs{let start=run.offset as usize;let end=start.checked_add(run.bytes.len()).filter(|end|*end<=LOWPOLY_PAINT_TEXTURE_BYTES).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"paint run exceeds canonical texture extent"))?;let bytes=if layer.pixels.is_empty(){vec![255;run.bytes.len()]}else{layer.pixels[start..end].to_vec()};inverse_runs.push(PixelRun{offset:run.offset,bytes});}
 Ok(vec![Op::from(EditPaintLayer{object_id:payload.object_id.clone(),layer_index:payload.layer_index,runs:inverse_runs})])
}
