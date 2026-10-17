//! 🚪️ IO stdio.jpg (jfif-1.01/🧾️document) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
pub use crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::encode_options::{JpgEncodeOptions,JpgEncodeComponent};

// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the
// real baseline-sequential (SOF0) JPEG codec — Huffman entropy decoding, dequantization,
// integer-ish separable IDCT, YCbCr→RGB with nearest-neighbor chroma upsampling — relocated here
// verbatim (destination rule 2: codecs → `🚪️io/`; rule 6: pure format algorithms with no
// snapshot dependency stay WITH the codec here rather than promoting to a module engine, since
// they're JPEG-specific, not artifact-independent). Progressive/arithmetic/lossless SOFn variants
// are explicit `JpgError::Unsupported`, never decoded as garbage. `JpgEngine` (zero construction
// sites) and the dead `register`/`register_pilot_languages`/`register_artifact_inferences`/
// `register_schema_specs` cluster (superseded by `declaration()` in the artifact root, zero real
// callers) were deleted outright, not relocated. `blank_jpg_snapshot`/`demo_jpg_snapshot` moved to
// `../🧬️schema` (pure helpers over the document type).
use crate::schema::snapshot::{JfifDensityUnits,JfifThumbnail,JpgSegment};
use crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::observations::{JpgFrameComponent,JpgFrameHeader,JpgHuffmanClass,JpgHuffmanTable,JpgQuantTable,JpgScanComponent,JpgNativeObservations,JpgDecodedDocument};
use crate::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};
use std::collections::HashMap;

#[path = "🧮️transforms-and-tables/🦀️.rs"]
mod transforms_and_tables;
pub use transforms_and_tables::*;

#[path = "🔡️huffman-and-bit-io/🦀️.rs"]
mod huffman_and_bit_io;
use huffman_and_bit_io::*;

#[path = "✍️encoder/🦀️.rs"]
mod encoder;
pub use encoder::*;

#[path = "🔎️header-and-decode-entry/🦀️.rs"]
mod header_and_decode_entry;
pub use header_and_decode_entry::*;

#[path = "🪜️step-decoder/🦀️.rs"]
mod step_decoder;
pub use step_decoder::*;

#[path = "🎹️facets/🦀️.rs"]
pub mod facets;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests





/// 🆕️ Constructs the canonical decoded white image at native admission.
pub fn blank_jpg_snapshot() -> JpgSnapshot {
    use crate::standards::v_jfif_1_01::subsets::document::io::{decode_jpg, encode_jpg};
    let seed = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: crate::schema::snapshot::JpgImage { width: 1,height: 1,pixels: vec![255, 255, 255, 255],..crate::schema::snapshot::JpgImage::default() } };
    encode_jpg(&seed, &JpgEncodeOptions::default()).and_then(|bytes| decode_jpg(&bytes)).expect("blank_jpg_snapshot: the 1×1 seed round-trips through the real codec")
}

/// 🧾️ Inspects exact native header observations even when raster decoding refuses that profile.
pub fn inspect_jpg_native_header(data:&dyn JpgByteSource)->Result<JpgNativeObservations,JpgError>{
 let header=parse_jpg_header(data,true)?;
 Ok(JpgNativeObservations{frame:header.frame,sof_marker:header.sof_marker,arithmetic:header.arithmetic,arithmetic_conditioning:header.arithmetic_conditioning,scan_components:header.scan_components,scan_parameters:header.scan_parameters,quant_tables:header.quant_tables,huffman_tables:header.huffman_tables,restart_interval:header.restart_interval})
}

/// 🚦️ Inspects native facts within an input budget and checkpoints every 256 byte visits.
pub fn inspect_jpg_native_header_controlled(data:&dyn JpgByteSource,maximum_input_bytes:usize,on_progress:&mut dyn FnMut(usize)->bool)->Result<JpgNativeObservations,semio_framework_value::ValueError>{
 use std::cell::{Cell,RefCell};
 use semio_framework_value::{ValueError,ValueRefusalKind};
 if data.len()>maximum_input_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"JPEG native input exceeds its admission budget"));}
 struct Source<'a>{data:&'a dyn JpgByteSource,on_progress:RefCell<&'a mut dyn FnMut(usize)->bool>,visited:Cell<usize>,stopped:Cell<bool>}
 impl JpgByteSource for Source<'_>{
  fn len(&self)->usize{self.data.len()}
  fn byte(&self,index:usize)->Option<u8>{
   let visited=self.visited.get();
   if self.stopped.get()||(visited%256==0&&!(self.on_progress.borrow_mut())(visited)){self.stopped.set(true);return None;}
   self.visited.set(visited.saturating_add(1));self.data.byte(index)
  }
 }
 let source=Source{data,on_progress:RefCell::new(on_progress),visited:Cell::new(0),stopped:Cell::new(false)};
 let result=inspect_jpg_native_header(&source);
 if source.stopped.get(){return Err(ValueError::new(ValueRefusalKind::Canceled,"JPEG native header inspection canceled"));}
 if !(source.on_progress.borrow_mut())(source.visited.get()){return Err(ValueError::new(ValueRefusalKind::Canceled,"JPEG native header inspection canceled"));}
 result.map_err(Into::into)
}
