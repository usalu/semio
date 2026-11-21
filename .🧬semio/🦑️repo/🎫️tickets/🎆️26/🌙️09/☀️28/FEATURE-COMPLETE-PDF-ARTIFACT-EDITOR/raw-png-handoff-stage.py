"""🧷 Stage checked paged reads in the current production PNG decoder without editing live build inputs."""

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[7]
PIXELS = ROOT / "🧰️framework/🔨️modules/🔲️pixels"
LIVE = PIXELS / "📷️png/📥️decode/🦀️.rs"
CANDIDATES = Path(__file__).resolve().parent / "🗑️generated" / "raw-png"


def replace(source, before, after):
    """🎯 Require one exact production anchor for every staged edit."""
    if source.count(before) != 1:
        raise ValueError(f"PNG source anchor drifted: {before}")
    return source.replace(before, after)


def stage_decoder(source):
    """📥 Add a second owned source to the existing raster reconstruction."""
    source = replace(source, "use semio_framework_value::RetainedAllocationEffect;",
        "use semio_framework_value::{RetainedAllocationEffect, paged::{ByteReadSource, PagedBytes}, retirement::{RetireOwned, controlled::ControlledRetirement}};")
    heading = "/// 🧩️ Reconstruction consumes one chunk byte, inflated byte, or expanded pixel per work unit."
    source = replace(source, heading, """/// 📚 An immutable PNG source supplied to the owned decode job.
pub trait PngSource: RetireOwned {
 fn byte_at(&self, offset: usize) -> Option<u8>;
 fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize;
}
impl PngSource for Vec<u8> {
 fn byte_at(&self, offset: usize) -> Option<u8> { self.get(offset).copied() }
 fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
  let source = self.get(offset..).unwrap_or_default();
  let length = source.len().min(output.len());
  output[..length].copy_from_slice(&source[..length]);
  length
 }
}
impl<const N: usize> PngSource for PagedBytes<N> {
 fn byte_at(&self, offset: usize) -> Option<u8> { ByteReadSource::byte_at(self, offset) }
 fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize { ByteReadSource::read_bytes(self, offset, output) }
}
fn fixed_bytes<const N: usize>(source: &impl PngSource, offset: usize) -> Result<[u8; N], PngDecodeError> {
 let mut bytes = [0; N];
 if source.read_bytes(offset, &mut bytes) != N { return Err(invalid("Truncated PNG source")); }
 Ok(bytes)
}

/// 📐 Limits for a PNG whose encoded bytes already have an owner.
#[derive(Clone, Copy)]
pub struct PngDecodeLimits { pub max_pixels: usize, pub max_bytes: usize, pub max_chunks: usize }
impl PngDecodeLimits {
 fn admits(self, length: usize) -> bool {
  (1..=16777216).contains(&self.max_pixels) && (8..=67108864).contains(&self.max_bytes)
   && (1..=65536).contains(&self.max_chunks) && (8..=self.max_bytes).contains(&length)
 }
}

""" + heading)
    source = replace(source, "pub struct PngDecodeJob{\n source:Vec<u8>,", "pub struct PngDecodeJob<S: RetireOwned = Vec<u8>>{\n source:S,")
    start = source.index("impl PngDecodeJob{")
    end = source.index("phase:\"chunks\"", start)
    source = source[:start] + """impl PngDecodeJob {
 pub fn new(input:PngDecodeInput)->Result<Self,(PngDecodeError,PngDecodeInput)>{
  let limits = PngDecodeLimits { max_pixels: input.max_pixels, max_bytes: input.max_bytes, max_chunks: input.max_chunks };
  if !limits.admits(input.data.len()) || fixed_bytes::<8>(&input.data, 0).ok() != Some([137,80,78,71,13,10,26,10]) {
   return Err((invalid("Invalid PNG input contract"), input));
  }
  let length = input.data.len();
  Ok(Self::new_admitted(input.data, length, limits))
 }
 pub fn source(&self)->&[u8]{&self.source}
}
impl<const N: usize> PngDecodeJob<PagedBytes<N>> {
 /// 📖 Borrows the exact owned encoded input pages for placement.
 pub fn paged_source(&self) -> &PagedBytes<N> { &self.source }
 /// 📦 Takes the admitted page owner without copying or flattening encoded bytes.
 pub fn admit_paged(owner: &mut ControlledRetirement<PagedBytes<N>>, limits: PngDecodeLimits) -> Result<Self, PngDecodeError> {
  let source = owner.original().ok_or(PngDecodeError::Incomplete)?;
  let length = source.len();
  if !limits.admits(length) || fixed_bytes::<8>(source, 0).ok() != Some([137,80,78,71,13,10,26,10]) {
   return Err(invalid("Invalid PNG input contract"));
  }
  let source = owner.take_original().ok_or(PngDecodeError::Incomplete)?;
  Ok(Self::new_admitted(source, length, limits))
 }
}
impl<S: PngSource> PngDecodeJob<S> {
 fn new_admitted(source: S, total_bytes: usize, limits: PngDecodeLimits) -> Self {
  Self{source,max_pixels:limits.max_pixels,max_chunks:limits.max_chunks,maximum_bytes:limits.max_bytes,total_bytes,""" + source[end:]
    source = replace(source, "partial_allocated:0,text:None})", "partial_allocated:0,text:None}")
    source = replace(source, "  let source=&self.source;let data=&source[c.start..c.end];",
        "  let source=&self.source;let length=c.end-c.start;")
    source = replace(source, 'if data.len()!=13{return Err(invalid("Invalid PNG header size"));}let width=be(data,0);',
        'if length!=13{return Err(invalid("Invalid PNG header size"));}let header_bytes=fixed_bytes::<13>(source,c.start)?;let data=&header_bytes[..];let width=be(data,0);')
    source = replace(source, 'matches!(color,0|4)||data.is_empty()||data.len()>768||data.len()%3!=0||color==3&&data.len()/3>1usize<<depth',
        'matches!(color,0|4)||length==0||length>768||length%3!=0||color==3&&length/3>1usize<<depth')
    source = replace(source, "self.palette_length=data.len()/3;for(i,value)in data.chunks_exact(3).enumerate(){self.palette[i]=[value[0],value[1],value[2]];}",
        "self.palette_length=length/3;for (index,value) in self.palette[..self.palette_length].iter_mut().enumerate(){*value=fixed_bytes::<3>(source,c.start+index*3)?;}")
    source = replace(source, 'if !self.had_palette||data.is_empty()||data.len()>self.palette_length{return Err(invalid("Invalid palette transparency"));}self.alpha_length=data.len();self.alpha[..data.len()].copy_from_slice(data);',
        'if !self.had_palette||length==0||length>self.palette_length{return Err(invalid("Invalid palette transparency"));}self.alpha_length=length;if source.read_bytes(c.start,&mut self.alpha[..length])!=length{return Err(invalid("Truncated PNG source"));}')
    source = replace(source, 'if data.len()!=if color==0{2}else{6}{return Err(invalid("Invalid sample transparency"));}let mut samples=[0u32;3];for (i,pair) in data.chunks_exact(2).enumerate(){samples[i]=u32::from(u16::from_be_bytes([pair[0],pair[1]]));',
        'if length!=if color==0{2}else{6}{return Err(invalid("Invalid sample transparency"));}let mut samples=[0u32;3];for (i,sample) in samples[..length/2].iter_mut().enumerate(){*sample=u32::from(u16::from_be_bytes(fixed_bytes::<2>(source,c.start+i*2)?));')
    source = replace(source, "if samples[i]>(1u32<<depth)-1", "if *sample>(1u32<<depth)-1")
    source = replace(source, "self.z_length+=data.len()", "self.z_length+=length")
    source = replace(source, "if !data.is_empty()||!self.had_idat", "if length!=0||!self.had_idat")
    source = replace(source, "let length=be(source,self.at)as usize;", "let framing=fixed_bytes::<8>(source,self.at)?;let length=be(&framing,0)as usize;")
    source = replace(source, "source[self.at+4..self.at+8].try_into().unwrap()", "framing[4..].try_into().unwrap()")
    source = replace(source, "&source[c.at..c.at+1]", '&[source.byte_at(c.at).ok_or_else(||invalid("Truncated PNG source"))?]')
    source = replace(source, "be(source,c.end)", "u32::from_be_bytes(fixed_bytes::<4>(source,c.end)?)")
    source = replace(source, "fn z_byte(&mut self)->Option<u8>{\n  let &(start,end)=self.ranges.get(self.range)?;",
        "fn z_byte(&mut self)->Result<Option<u8>,PngDecodeError>{\n  let Some(&(start,end))=self.ranges.get(self.range) else { return Ok(None); };")
    source = replace(source, "unwrap_or(0);return None;}\n  debug_assert!(self.range_at>=start);let value=self.source[self.range_at];self.range_at+=1;self.z_at+=1;Some(value)",
        'unwrap_or(0);return Ok(None);}\n  debug_assert!(self.range_at>=start);let value=self.source.byte_at(self.range_at).ok_or_else(||invalid("Truncated PNG source"))?;self.range_at+=1;self.z_at+=1;Ok(Some(value))')
    if source.count("self.z_byte()") != 3:
        raise ValueError("PNG IDAT cursor drifted")
    source = source.replace("self.z_byte()", "self.z_byte()?")
    start = source.index(" pub fn source(&self)->&[u8]{&self.source}", source.index("impl<S: PngSource>"))
    end = source.index(" pub fn decoded_properties", start)
    contiguous = source[start:end].split("\n", 1)[1]
    source = source[:start] + source[end:]
    anchor = '#[cfg(feature = "testkit")]'
    return replace(source, anchor, "impl PngDecodeJob {\n" + contiguous + "}\n\n" + anchor)


def main():
    """🧪 Emit the guarded decoder body and link to the original adjacent modules."""
    CANDIDATES.mkdir(parents=True, exist_ok=True)
    staged = stage_decoder(LIVE.read_text(encoding="utf8"))
    for relative in ("📝️text/🦀️.rs", "♻️close/🧪️tests/🦀️.rs"):
        staged = staged.replace(relative, (LIVE.parent / relative).as_posix())
    (CANDIDATES / "decode.rs").write_text(staged, encoding="utf8")


if __name__ == "__main__":
    main()
