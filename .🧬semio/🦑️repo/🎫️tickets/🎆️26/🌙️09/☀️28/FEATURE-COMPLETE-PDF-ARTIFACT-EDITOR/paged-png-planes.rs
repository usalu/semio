//! 📷️ Validates original PNG chunks and lends raw PDF color and alpha samples one byte at a time.

use semio_framework_deflate::{InflateOutcome, Inflater};
use semio_framework_value::paged::ByteReadSource;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PngRefusal {
    ChangedSource, InvalidSignature, InvalidChunk, InvalidCrc, UnsupportedImage,
    ImageLimit, InvalidEnd, InvalidDeflate, InvalidScanline, InvalidAdler, SampleTarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PngExtent {
    pub width: u32,
    pub height: u32,
    pub color_samples: u8,
    pub alpha: bool,
}

impl PngExtent {
    fn channels(self) -> usize { usize::from(self.color_samples) + usize::from(self.alpha) }
}

#[derive(Clone, Copy)]
struct Chunk {
    name: [u8; 4],
    body: usize,
    end: usize,
}

fn span<const N: usize>(source: &dyn ByteReadSource, index: usize) -> Option<[u8; N]> {
    let mut bytes = [0; N];
    (source.read_bytes(index, &mut bytes) == N).then_some(bytes)
}

fn chunk(source: &dyn ByteReadSource, offset: usize) -> Result<Chunk, PngRefusal> {
    let data = span::<8>(source, offset).ok_or(PngRefusal::InvalidChunk)?;
    let length = u32::from_be_bytes(data[..4].try_into().unwrap()) as usize;
    let name: [u8; 4] = data[4..].try_into().unwrap();
    let body = offset.checked_add(8).ok_or(PngRefusal::InvalidChunk)?;
    let end = body.checked_add(length).filter(|end| *end <= source.byte_len().saturating_sub(4)).ok_or(PngRefusal::InvalidChunk)?;
    if length > i32::MAX as usize || !name.iter().all(u8::is_ascii_alphabetic) || name[2].is_ascii_lowercase() {
        return Err(PngRefusal::InvalidChunk);
    }
    Ok(Chunk { name, body, end })
}

fn crc_byte(mut value: u32, byte: u8) -> u32 {
    value ^= u32::from(byte);
    for _ in 0..8 { value = (value >> 1) ^ (0xedb88320 & 0u32.wrapping_sub(value & 1)); }
    value
}

struct CrcChunk { chunk: Chunk, at: usize, crc: u32 }

pub struct PngIdatProof {
    input_length: usize,
    maximum_pixels: usize,
    maximum_chunks: u32,
    offset: usize,
    count: u32,
    live: Option<CrcChunk>,
    extent: Option<PngExtent>,
    first_data: Option<Chunk>,
    data_bytes: usize,
    after_data: bool,
    palette_seen: bool,
    ended: bool,
}

impl PngIdatProof {
    /// 🛂️ Opens a borrowed source proof within the selected image codec's independent bounds.
    pub fn new(input_length: usize, maximum_pixels: usize, maximum_chunks: u32) -> Self {
        Self { input_length, maximum_pixels, maximum_chunks, offset: 0, count: 0,
            live: None, extent: None, first_data: None, data_bytes: 0, after_data: false, palette_seen: false, ended: false }
    }

    /// 🔎️ Admits one header or checks at most one 4 KiB original chunk extent.
    pub fn step(&mut self, source: &dyn ByteReadSource, wanted: usize) -> Result<bool, PngRefusal> {
        if source.byte_len() != self.input_length { return Err(PngRefusal::ChangedSource); }
        if self.ended || wanted == 0 { return Ok(self.ended); }
        if self.offset == 0 {
            if span(source, 0) != Some([137, 80, 78, 71, 13, 10, 26, 10]) { return Err(PngRefusal::InvalidSignature); }
            self.offset = 8;
            return Ok(false);
        }
        let Some(live) = self.live.as_mut() else {
            self.count = self.count.checked_add(1).filter(|count| *count <= self.maximum_chunks).ok_or(PngRefusal::InvalidChunk)?;
            let chunk = chunk(source, self.offset)?;
            let crc = chunk.name.into_iter().fold(u32::MAX, crc_byte);
            self.live = Some(CrcChunk { chunk, at: chunk.body, crc });
            return Ok(false);
        };
        if live.at < live.chunk.end {
            let mut bytes = [0; 4096];
            let extent = (live.chunk.end - live.at).min(wanted).min(bytes.len());
            if source.read_bytes(live.at, &mut bytes[..extent]) != extent { return Err(PngRefusal::ChangedSource); }
            live.crc = bytes[..extent].iter().copied().fold(live.crc, crc_byte);
            live.at += extent;
            return Ok(false);
        }
        if span(source, live.chunk.end).map(u32::from_be_bytes) != Some(!live.crc) { return Err(PngRefusal::InvalidCrc); }
        let Chunk { name, body, end } = self.live.take().unwrap().chunk;
        self.offset = end + 4;
        if self.count == 1 && name != *b"IHDR" { return Err(PngRefusal::InvalidChunk); }
        self.after_data |= self.first_data.is_some() && name != *b"IDAT";
        match &name {
            b"IHDR" if self.count == 1 && end - body == 13 => {
                let header: [u8; 13] = span(source, body).ok_or(PngRefusal::InvalidChunk)?;
                let width = u32::from_be_bytes(header[..4].try_into().unwrap());
                let height = u32::from_be_bytes(header[4..8].try_into().unwrap());
                let (color_samples, alpha) = match (header[8], header[9]) {
                    (8, 0) => (1, false), (8, 2) => (3, false), (8, 4) => (1, true), (8, 6) => (3, true),
                    _ => return Err(PngRefusal::UnsupportedImage),
                };
                if header[10..] != [0, 0, 0] { return Err(PngRefusal::UnsupportedImage); }
                let pixels = width as u64 * height as u64;
                if pixels == 0 || pixels > self.maximum_pixels as u64 || pixels > usize::MAX as u64 / 4 {
                    return Err(PngRefusal::ImageLimit);
                }
                self.extent = Some(PngExtent { width, height, color_samples, alpha });
            }
            b"IDAT" if !self.after_data && self.extent.is_some() => {
                self.first_data.get_or_insert(Chunk { name, body, end });
                self.data_bytes = self.data_bytes.checked_add(end - body).ok_or(PngRefusal::ImageLimit)?;
            }
            b"IEND" if body == end && self.offset == self.input_length && self.data_bytes >= 6 => self.ended = true,
            b"IEND" => return Err(PngRefusal::InvalidEnd),
            b"iCCP" | b"tRNS" => return Err(PngRefusal::UnsupportedImage),
            b"PLTE" if !self.palette_seen && self.first_data.is_none() && self.extent.is_some_and(|extent| extent.color_samples == 3)
                && (3..=768).contains(&(end - body)) && (end - body).is_multiple_of(3) => self.palette_seen = true,
            _ if name[0].is_ascii_lowercase() => {}
            _ => return Err(PngRefusal::InvalidChunk),
        }
        Ok(self.ended)
    }

    pub fn samples(&self) -> Option<PngSampleCursor> {
        if !self.ended { return None; }
        let first = self.first_data?;
        Some(PngSampleCursor {
            input_length: self.input_length, extent: self.extent?, compressed: self.data_bytes,
            idat: first, input: first.body, consumed: 0, zlib: [0; 2], pending: None, inflater: None,
            deflate_done: false, tail: [0; 4], tail_size: 0, a: 1, b: 0,
            row: 0, sample: None, filter: 0, done: false,
        })
    }
}

pub trait PngPlaneTarget {
    /// 🧩️ Borrows a previously delivered raw channel without filling another scanline array.
    fn previous(&self, pixel: usize, channel: usize) -> Option<u8>;
    fn put(&mut self, pixel: usize, channel: usize, sample: u8) -> Result<(), PngRefusal>;
}

pub struct PngSampleCursor {
    input_length: usize,
    extent: PngExtent,
    compressed: usize,
    idat: Chunk,
    input: usize,
    consumed: usize,
    zlib: [u8; 2],
    pending: Option<u8>,
    inflater: Option<Inflater>,
    deflate_done: bool,
    tail: [u8; 4],
    tail_size: usize,
    a: u32,
    b: u32,
    row: usize,
    sample: Option<usize>,
    filter: u8,
    done: bool,
}

impl PngSampleCursor {
    pub fn extent(&self) -> PngExtent { self.extent }
    pub fn next_history_capacity(&self) -> Option<usize> { self.inflater.as_ref()?.next_retained_allocation_bytes() }
    pub fn history_capacity(&self) -> usize { self.inflater.as_ref().map_or(0, Inflater::retained_allocated_bytes) }

    pub fn next_output_alpha(&self) -> Option<bool> {
        if self.done || self.deflate_done { return None; }
        self.sample.map(|sample| sample % self.extent.channels() == usize::from(self.extent.color_samples))
    }


    pub fn close_history_step(&mut self, items: usize, release_bytes: usize) -> bool {
        let Some(inflater) = self.inflater.as_mut() else { return true; };
        if !inflater.retained_terminal_is_empty() { inflater.close_retained_step(items, release_bytes); }
        if inflater.retained_terminal_is_empty() { self.inflater = None; }
        self.inflater.is_none()
    }

    fn byte(&mut self, source: &dyn ByteReadSource) -> Result<Option<u8>, PngRefusal> {
        if self.input == self.idat.end {
            self.idat = chunk(source, self.idat.end + 4)?;
            if self.idat.name != *b"IDAT" { return Err(PngRefusal::ChangedSource); }
            self.input = self.idat.body;
            return Ok(None);
        }
        let [value] = span(source, self.input).ok_or(PngRefusal::ChangedSource)?;
        self.input += 1;
        self.consumed += 1;
        Ok(Some(value))
    }

    /// 🏁️ Advances one native inflate transition. Samples stay private until the exact Adler and row census agree.
    pub fn step(&mut self, source: &dyn ByteReadSource, output: &mut dyn PngPlaneTarget, capacity: usize) -> Result<bool, PngRefusal> {
        if source.byte_len() != self.input_length { return Err(PngRefusal::ChangedSource); }
        if self.done { return Ok(true); }
        if self.consumed < 2 {
            let index = self.consumed;
            if let Some(value) = self.byte(source)? { self.zlib[index] = value; }
            return Ok(false);
        }
        if self.inflater.is_none() {
            let [cmf, flg] = self.zlib;
            if cmf & 15 != 8 || cmf >> 4 > 7 || u16::from_be_bytes(self.zlib) % 31 != 0 || flg & 32 != 0 {
                return Err(PngRefusal::InvalidDeflate);
            }
            self.inflater = Some(Inflater::try_new_retained(1usize << ((cmf >> 4) + 8), 32768).map_err(|_| PngRefusal::InvalidDeflate)?);
            return Ok(false);
        }
        if self.deflate_done {
            if self.tail_size < 4 {
                if let Some(value) = self.byte(source)? {
                    self.tail[self.tail_size] = value;
                    self.tail_size += 1;
                }
                return Ok(false);
            }
            if self.consumed != self.compressed || u32::from_be_bytes(self.tail) != ((self.b << 16) | self.a)
                || self.row != self.extent.height as usize || self.sample.is_some() { return Err(PngRefusal::InvalidAdler); }
            self.done = true;
            return Ok(true);
        }
        let inflater = self.inflater.as_mut().unwrap();
        if let Some(required) = inflater.next_retained_allocation_bytes() {
            if capacity >= required { inflater.reserve_retained_history(capacity).map_err(|_| PngRefusal::InvalidDeflate)?; }
            return Ok(false);
        }
        if self.pending.is_none() && self.consumed < self.compressed - 4 {
            self.pending = self.byte(source)?;
            if self.pending.is_none() { return Ok(false); }
        }
        let inflater = self.inflater.as_mut().unwrap();
        match inflater.advance(&mut self.pending, self.consumed == self.compressed - 4).map_err(|_| PngRefusal::InvalidDeflate)? {
            InflateOutcome::Wrote(value) => self.sample(value, output)?,
            InflateOutcome::Done => {
                if self.pending.is_some() || self.consumed != self.compressed - 4 || inflater.terminal_unused_bytes() != Some(0) {
                    return Err(PngRefusal::InvalidDeflate);
                }
                self.deflate_done = true;
            }
            InflateOutcome::NeedInput => {}
        }
        Ok(false)
    }

    fn sample(&mut self, byte: u8, output: &mut dyn PngPlaneTarget) -> Result<(), PngRefusal> {
        self.a = (self.a + u32::from(byte)) % 65521;
        self.b = (self.b + self.a) % 65521;
        if self.row >= self.extent.height as usize { return Err(PngRefusal::InvalidScanline); }
        let Some(sample) = self.sample else {
            if byte > 4 { return Err(PngRefusal::InvalidScanline); }
            self.filter = byte;
            self.sample = Some(0);
            return Ok(());
        };
        let width = self.extent.width as usize;
        let channels = self.extent.channels();
        let (column, channel) = (sample / channels, sample % channels);
        let index = self.row * width + column;
        let previous = |active: bool, offset: usize| {
            if !active { return Ok(0); }
            output.previous(index - offset, channel).ok_or(PngRefusal::SampleTarget)
        };
        let left = previous(column != 0 && matches!(self.filter, 1 | 3 | 4), 1)?;
        let upper = previous(self.row != 0 && matches!(self.filter, 2 | 3 | 4), width)?;
        let corner = previous(self.row != 0 && column != 0 && self.filter == 4, width + 1)?;
        let prediction = match self.filter {
            0 => 0, 1 => left, 2 => upper, 3 => ((u16::from(left) + u16::from(upper)) / 2) as u8,
            _ => paeth(left, upper, corner),
        };
        output.put(index, channel, byte.wrapping_add(prediction))?;
        if sample + 1 == width * channels { self.sample = None; self.row += 1; }
        else { self.sample = Some(sample + 1); }
        Ok(())
    }
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (first, second, third) = (i16::from(a), i16::from(b), i16::from(c));
    let prediction = first + second - third;
    let (da, db, dc) = ((prediction - first).abs(), (prediction - second).abs(), (prediction - third).abs());
    if da <= db && da <= dc { a } else if db <= dc { b } else { c }
}
