//! 🧮️ Validates a native baseline scan in bounded turns and optionally lends one coefficient block per turn.
use super::{decode_block, BitReader, HuffLookup, JpgByteSource, JpgError};

const MAX_PIXELS: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BaselineJpgMetadata {
    pub width: u32,
    pub height: u32,
    pub components: u8,
    pub color_transform: Option<u32>,
}

pub struct BaselineJpgBlock<'a> {
    pub component: usize,
    pub block_x: usize,
    pub block_y: usize,
    pub sampling_x: usize,
    pub sampling_y: usize,
    pub maximum_sampling_x: usize,
    pub maximum_sampling_y: usize,
    pub zigzag: &'a [i32; 64],
    pub quantization: &'a [u8; 64],
}

#[derive(Clone, Copy, Default)]
struct Component {
    id: u8,
    h: usize,
    v: usize,
    quant: usize,
}

#[derive(Clone, Copy, Default)]
struct ScanComponent {
    index: usize,
    dc: usize,
    ac: usize,
}

struct EntropyTable {
    first: [u32; 16],
    offset: [u16; 16],
    counts: [u8; 16],
    values: [u8; 256],
}

impl HuffLookup for EntropyTable {
    fn max_len(&self) -> u8 { 16 }

    fn symbol(&self, length: u8, code: u16) -> Option<u8> {
        let index = usize::from(length - 1);
        let relative = u32::from(code).checked_sub(self.first[index])?;
        (relative < u32::from(self.counts[index]))
            .then(|| self.values[usize::from(self.offset[index]) + relative as usize])
    }
}

enum Phase {
    Start,
    Header,
    Tables { end: usize, huffman: bool },
    Scan,
    Restart,
    End,
    Complete,
}

pub struct BaselineJpgCursor {
    length: usize,
    phase: Phase,
    offset: usize,
    acc: u32,
    nbits: u32,
    metadata: Option<BaselineJpgMetadata>,
    components: [Component; 3],
    scan: [ScanComponent; 3],
    quant: [Option<[u8; 64]>; 4],
    huffman: [Option<EntropyTable>; 8],
    adobe: Option<u8>,
    jfif: bool,
    restart_interval: usize,
    next_restart: u8,
    predictors: [i32; 3],
    hmax: usize,
    vmax: usize,
    columns: usize,
    mcus: usize,
    mcu: usize,
    lane: usize,
    block: usize,
}

impl BaselineJpgCursor {
    /// 🌱️ Sets up fixed state without reading or copying any source bytes.
    pub fn new(length: usize) -> Self {
        Self {
            length, phase: Phase::Start, offset: 0, acc: 0, nbits: 0,
            metadata: None, components: Default::default(), scan: Default::default(),
            quant: [None; 4], huffman: std::array::from_fn(|_| None),
            adobe: None, jfif: false, restart_interval: 0, next_restart: 0,
            predictors: [0; 3], hmax: 0, vmax: 0, columns: 0, mcus: 0, mcu: 0, lane: 0, block: 0,
        }
    }

    /// 📍️ Returns the compressed input position for progress reporting.
    pub fn position(&self) -> usize { self.offset }

    /// ✅️ Returns the metadata only after validation of the entire single scan.
    pub fn complete(&self) -> Option<BaselineJpgMetadata> {
        matches!(self.phase, Phase::Complete).then_some(self.metadata).flatten()
    }

    /// 🪜️ Observes at most one marker, table, or coefficient block on the same immutable source.
    pub fn step(
        &mut self,
        source: &dyn JpgByteSource,
        block: &mut dyn FnMut(BaselineJpgBlock<'_>),
    ) -> Result<(), JpgError> {
        if source.len() != self.length {
            return Err(malformed("JPEG source length changed during admission"));
        }
        match self.phase {
            Phase::Start => {
                if pair(source, 0)? != 0xffd8 { return Err(malformed("missing JPEG SOI")); }
                self.offset = 2;
                self.phase = Phase::Header;
                Ok(())
            }
            Phase::Header => self.header(source),
            Phase::Tables { end, huffman } => self.table(source, end, huffman),
            Phase::Scan => self.entropy_block(source, block),
            Phase::Restart | Phase::End => self.scan_marker(source),
            Phase::Complete => Ok(()),
        }
    }

    fn header(&mut self, source: &dyn JpgByteSource) -> Result<(), JpgError> {
        let marker = pair(source, self.offset)?;
        if marker == 0xffff {
            self.offset += 1;
            return Ok(());
        }
        let [prefix, kind] = marker.to_be_bytes();
        if prefix != 0xff || !matches!(kind, 0xc0 | 0xc4 | 0xda | 0xdb | 0xdd | 0xe0..=0xef | 0xfe) {
            return Err(JpgError::Unsupported(format!("JPEG baseline header marker 0x{marker:04x}")));
        }
        let length = usize::from(pair(source, self.offset + 2)?);
        let start = self.offset + 4;
        let end = self.offset + 2 + length;
        if length < 2 || end > self.length { return Err(malformed("JPEG segment length overruns source")); }
        match kind {
            0xc0 => self.frame(source, start, end)?,
            0xc4 | 0xdb => {
                if start == end { return Err(malformed("empty JPEG table segment")); }
                self.offset = start;
                self.phase = Phase::Tables { end, huffman: kind == 0xc4 };
                return Ok(());
            }
            0xda => self.scan(source, start, end)?,
            0xdd => {
                if end - start != 2 { return Err(malformed("invalid JPEG restart interval length")); }
                self.restart_interval = usize::from(pair(source, start)?);
            }
            0xe0 if end - start >= 14 => self.jfif |= starts_with(source, start, b"JFIF\0"),
            0xee if end - start >= 12 && starts_with(source, start, b"Adobe") => {
                let transform = octet(source, start + 11)?;
                if self.adobe.is_some_and(|prior| prior != transform) {
                    return Err(malformed("conflicting JPEG Adobe color transforms"));
                }
                self.adobe = Some(transform);
            }
            _ => {}
        }
        self.offset = end;
        Ok(())
    }

    fn frame(&mut self, source: &dyn JpgByteSource, start: usize, end: usize) -> Result<(), JpgError> {
        if self.metadata.is_some() || end - start < 6 { return Err(malformed("invalid JPEG frame header")); }
        let precision = octet(source, start)?;
        let height = u32::from(pair(source, start + 1)?);
        let width = u32::from(pair(source, start + 3)?);
        let components = octet(source, start + 5)?;
        if precision != 8 || !matches!(components, 1 | 3) {
            return Err(JpgError::Unsupported("JPEG requires 8-bit grayscale or three-component baseline".into()));
        }
        if width == 0 || height == 0 || u64::from(width) * u64::from(height) > MAX_PIXELS
            || end - start != 6 + 3 * usize::from(components)
        {
            return Err(malformed("invalid JPEG frame dimensions or component count"));
        }
        let mut blocks = 0;
        for index in 0..usize::from(components) {
            let position = start + 6 + index * 3;
            let id = octet(source, position)?;
            let sampling = octet(source, position + 1)?;
            let mut h = usize::from(sampling >> 4);
            let mut v = usize::from(sampling & 15);
            let quant = usize::from(octet(source, position + 2)?);
            if !(1..=4).contains(&h) || !(1..=4).contains(&v) || quant >= self.quant.len()
                || self.components[..index].iter().any(|component| component.id == id)
            {
                return Err(malformed("invalid JPEG frame component"));
            }
            if components == 1 { (h, v) = (1, 1); }
            self.components[index] = Component { id, h, v, quant };
            self.hmax = self.hmax.max(h);
            self.vmax = self.vmax.max(v);
            blocks += h * v;
        }
        if blocks > 10 { return Err(malformed("JPEG MCU contains more than ten blocks")); }
        self.columns = (width as usize).div_ceil(8 * self.hmax);
        self.mcus = self.columns * (height as usize).div_ceil(8 * self.vmax);
        self.metadata = Some(BaselineJpgMetadata { width, height, components, color_transform: None });
        Ok(())
    }

    fn scan(&mut self, source: &dyn JpgByteSource, start: usize, end: usize) -> Result<(), JpgError> {
        let mut metadata = self.metadata.ok_or_else(|| malformed("JPEG scan precedes frame"))?;
        let n = usize::from(metadata.components);
        if end - start != 4 + 2 * n || octet(source, start)? != metadata.components
            || pair(source, end - 3)? != 63 || octet(source, end - 1)? != 0
        {
            return Err(JpgError::Unsupported("JPEG requires one complete baseline sequential scan".into()));
        }
        for lane in 0..n {
            let id = octet(source, start + 1 + lane * 2)?;
            let tables = octet(source, start + 2 + lane * 2)?;
            let index = self.components[..n].iter().position(|component| component.id == id)
                .ok_or_else(|| malformed("JPEG scan references an absent frame component"))?;
            let dc = usize::from(tables >> 4);
            let ac = usize::from(tables & 15);
            if self.scan[..lane].iter().any(|component| component.index == index)
                || dc > 3 || ac > 3 || self.huffman[dc].is_none() || self.huffman[4 + ac].is_none()
                || self.quant[self.components[index].quant].is_none()
            {
                return Err(malformed("invalid JPEG scan component or table reference"));
            }
            self.scan[lane] = ScanComponent { index, dc, ac: 4 + ac };
        }
        if let Some(transform) = self.adobe {
            if transform > 1 || n == 1 && transform != 0 {
                return Err(JpgError::Unsupported("unsupported JPEG Adobe color transform".into()));
            }
        }
        if n == 3 {
            let rgb_ids = self.components.map(|component| component.id) == *b"RGB";
            metadata.color_transform = Some(self.adobe.map(u32::from).unwrap_or(u32::from(self.jfif || !rgb_ids)));
        }
        self.metadata = Some(metadata);
        self.phase = Phase::Scan;
        Ok(())
    }

    fn table(&mut self, source: &dyn JpgByteSource, end: usize, huffman: bool) -> Result<(), JpgError> {
        let table_info = octet(source, self.offset)?;
        let id = usize::from(table_info & 15);
        let class = usize::from(table_info >> 4);
        if id > 3 || class > usize::from(huffman) { return Err(malformed("invalid JPEG table selector")); }
        let start = self.offset + 1;
        if huffman {
            if end - start < 16 { return Err(malformed("truncated JPEG entropy counts")); }
            let counts = fixed_bytes(source, start)?;
            let mut table = EntropyTable { first: [0; 16], offset: [0; 16], counts, values: [0; 256] };
            let (mut code, mut count) = (0u32, 0usize);
            for (index, length_count) in counts.into_iter().enumerate() {
                table.first[index] = code;
                table.offset[index] = count as u16;
                code += u32::from(length_count);
                count += usize::from(length_count);
                if code >= 1 << (index + 1) { return Err(malformed("oversubscribed JPEG entropy table")); }
                code <<= 1;
            }
            if count == 0 || count > 256 || count > end - start - 16 {
                return Err(malformed("invalid JPEG entropy table size"));
            }
            for index in 0..count {
                let symbol = octet(source, start + 16 + index)?;
                if class == 0 && symbol > 11 || class == 1
                    && (symbol & 15 > 10 || symbol & 15 == 0 && !matches!(symbol, 0 | 0xf0))
                {
                    return Err(malformed("invalid JPEG baseline entropy symbol"));
                }
                table.values[index] = symbol;
            }
            self.huffman[4 * class + id] = Some(table);
            self.offset = start + 16 + count;
        } else {
            if end - start < 64 { return Err(malformed("truncated JPEG quantization table")); }
            let table: [u8; 64] = fixed_bytes(source, start)?;
            if table.contains(&0) { return Err(malformed("zero JPEG quantization coefficient")); }
            self.quant[id] = Some(table);
            self.offset = start + 64;
        }
        if self.offset == end { self.phase = Phase::Header; }
        Ok(())
    }

    fn entropy_block(
        &mut self,
        source: &dyn JpgByteSource,
        output: &mut dyn FnMut(BaselineJpgBlock<'_>),
    ) -> Result<(), JpgError> {
        let lane = self.scan[self.lane];
        let component = self.components[lane.index];
        let mut input = BitReader { data: source, pos: self.offset, acc: self.acc, nbits: self.nbits };
        let dc = self.huffman[lane.dc].as_ref().expect("admitted JPEG DC table");
        let ac = self.huffman[lane.ac].as_ref().expect("admitted JPEG AC table");
        let zigzag = decode_block(&mut input, &mut self.predictors[lane.index], dc, ac)?;
        (self.offset, self.acc, self.nbits) = (input.pos, input.acc, input.nbits);
        output(BaselineJpgBlock {
            component: lane.index,
            block_x: self.mcu % self.columns * component.h + self.block % component.h,
            block_y: self.mcu / self.columns * component.v + self.block / component.h,
            sampling_x: component.h,
            sampling_y: component.v,
            maximum_sampling_x: self.hmax,
            maximum_sampling_y: self.vmax,
            zigzag: &zigzag,
            quantization: self.quant[component.quant].as_ref().expect("admitted JPEG quantization table"),
        });
        self.block += 1;
        if self.block == component.h * component.v {
            self.block = 0;
            self.lane += 1;
        }
        if self.lane == usize::from(self.metadata.expect("admitted JPEG frame").components) {
            self.lane = 0;
            self.mcu += 1;
            if self.mcu == self.mcus {
                self.phase = Phase::End;
            } else if self.restart_interval != 0 && self.mcu % self.restart_interval == 0 {
                self.phase = Phase::Restart;
            }
        }
        Ok(())
    }

    fn scan_marker(&mut self, source: &dyn JpgByteSource) -> Result<(), JpgError> {
        let padding = (1 << self.nbits) - 1;
        if self.acc & padding != padding { return Err(malformed("invalid JPEG entropy padding")); }
        let marker = pair(source, self.offset)?;
        if marker == 0xffff {
            self.offset += 1;
            return Ok(());
        }
        self.offset += 2;
        if matches!(self.phase, Phase::End) {
            if marker != 0xffd9 || self.offset != self.length {
                return Err(malformed("JPEG single scan requires a terminal EOI"));
            }
            self.phase = Phase::Complete;
        } else {
            if marker != 0xffd0 + u16::from(self.next_restart) {
                return Err(malformed("unexpected JPEG restart marker"));
            }
            self.next_restart = (self.next_restart + 1) % 8;
            self.predictors = [0; 3];
            (self.acc, self.nbits) = (0, 0);
            self.phase = Phase::Scan;
        }
        Ok(())
    }
}

fn malformed(message: &str) -> JpgError { JpgError::Malformed(message.into()) }

fn octet(source: &dyn JpgByteSource, position: usize) -> Result<u8, JpgError> {
    source.byte(position).ok_or_else(|| malformed("truncated JPEG input"))
}

fn fixed_bytes<const N: usize>(source: &dyn JpgByteSource, position: usize) -> Result<[u8; N], JpgError> {
    let mut bytes = [0; N];
    for (index, byte) in bytes.iter_mut().enumerate() { *byte = octet(source, position + index)?; }
    Ok(bytes)
}

fn pair(source: &dyn JpgByteSource, position: usize) -> Result<u16, JpgError> {
    fixed_bytes(source, position).map(u16::from_be_bytes)
}

fn starts_with(source: &dyn JpgByteSource, position: usize, signature: &[u8]) -> bool {
    signature.iter().enumerate().all(|(index, byte)| source.byte(position + index) == Some(*byte))
}
