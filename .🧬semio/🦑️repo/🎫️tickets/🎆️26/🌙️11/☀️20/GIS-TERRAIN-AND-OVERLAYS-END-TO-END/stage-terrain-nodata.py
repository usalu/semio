"""🏔️ Stage terrain NoData source changes without writing to the live input tree."""

from pathlib import Path
import difflib
import json
import re

TICKET = Path(__file__).resolve().parent
GIS = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any")
SURFACE = Path("🧰️framework/🔨️modules/🗺️surface/🏔️terrain")
CODEC = GIS / "🚪️io/📝️text/🏔️ascii-grid/🦀️.rs"
MASK = SURFACE / "🕳️missing-samples/🦀️.rs"
changes = {}


def read(path):
    """📖 Read from the staged transformations or the current live snapshot."""
    return changes.get(path) if path in changes else path.read_text(encoding="utf-8")


def edit(path, before, after, count=1):
    """🪡 Refuse edits whose anchors moved."""
    source = read(path)
    assert source.count(before) == count, (path, repr(before), source.count(before), count)
    changes[path] = source.replace(before, after)


changes[MASK] = '''//! 🕳️ Encodes canonical absent sample ordinals for rectangular terrain grids.

/// 📦️ Largest encoded mask at the maximum supported grid size.
pub const MAX_MASK_BYTES: usize = (65_usize * 65).div_ceil(8);
pub const MAX_MASK_HEX_BYTES: usize = MAX_MASK_BYTES * 2;

const HEX: &[u8; 16] = b"0123456789abcdef";

fn nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

fn byte_at(hex: &str, index: usize) -> Option<u8> {
    let pair = hex.as_bytes().get(2 * index..2 * index + 2)?;
    Some((nibble(pair[0])? << 4) | nibble(pair[1])?)
}

/// 🧩 Stores a bounded south-first bitset without an allocation during per-row import.
pub struct MissingSamples {
    bytes: [u8; MAX_MASK_BYTES],
    sample_count: usize,
}

impl MissingSamples {
    /// ➕️ Creates an entirely present bounded grid.
    pub fn new(sample_count: usize) -> Option<Self> {
        (sample_count <= 65 * 65).then_some(Self { bytes: [0; MAX_MASK_BYTES], sample_count })
    }

    /// 🪫 Marks one south-first sample ordinal as missing.
    pub fn mark_missing(&mut self, ordinal: usize) {
        assert!(ordinal < self.sample_count);
        self.bytes[ordinal / 8] |= 1 << (ordinal % 8);
    }

    fn bytes(&self) -> &[u8] { &self.bytes[..self.sample_count.div_ceil(8)] }

    /// 🔢 Reports the exact length of the canonical wire value, zero when all samples are present.
    pub fn encoded_len(&self) -> usize {
        if self.bytes().iter().any(|&byte| byte != 0) { 2 * self.bytes().len() } else { 0 }
    }

    /// 🟰 Compares to a canonical mask without constructing a second retained value.
    pub fn same_encoding(&self, value: Option<&str>) -> bool {
        match value {
            None => self.encoded_len() == 0,
            Some(hex) => self.encoded_len() == hex.len()
                && self.bytes().iter().enumerate().all(|(index, &value)| byte_at(hex, index) == Some(value)),
        }
    }

    /// 💾️ Returns the optional canonical lowercase hexadecimal mask.
    pub fn encode(&self) -> Option<String> {
        let length = self.encoded_len();
        if length == 0 { return None; }
        let mut result = String::with_capacity(length);
        for &byte in self.bytes() {
            result.push(char::from(HEX[(byte >> 4) as usize]));
            result.push(char::from(HEX[(byte & 15) as usize]));
        }
        Some(result)
    }
}

/// 🎯️ Queries the validity of an ordinal in a checked encoding.
pub fn is_missing(hex: Option<&str>, ordinal: usize) -> bool {
    hex.and_then(|hex| byte_at(hex, ordinal / 8)).is_some_and(|byte| byte & (1 << (ordinal % 8)) != 0)
}

/// ✅️ Checks mask spelling, extent, padding and canonical zero payloads.
pub fn is_valid(hex: Option<&str>, samples: impl ExactSizeIterator<Item = i32>) -> bool {
    let Some(hex) = hex else { return true; };
    let count = samples.len();
    let bytes = count.div_ceil(8);
    if bytes == 0 || bytes > MAX_MASK_BYTES || hex.len() != 2 * bytes { return false; }
    let mut any_missing = false;
    let mut current = 0;
    for (ordinal, sample) in samples.enumerate() {
        if ordinal % 8 == 0 {
            let Some(byte) = byte_at(hex, ordinal / 8) else { return false; };
            current = byte;
            any_missing |= byte != 0;
        }
        if current & (1 << (ordinal % 8)) != 0 && sample != 0 { return false; }
    }
    any_missing && (count % 8 == 0 || current >> (count % 8) == 0)
}
'''

edit(CODEC, "use std::fmt::Write;", """use semio_framework_surface::terrain::missing_samples::MissingSamples;
use std::fmt::Write;""")
edit(CODEC, "Samples, NoData, Missing", "Samples, Missing")
edit(CODEC, "    no_data: f64,", "    no_data: Decimal,\n    pub missing_samples: MissingSamples,")
edit(CODEC, "            bounds, dimensions, no_data, offset:", """            bounds, dimensions, no_data,
            missing_samples: MissingSamples::new(dimensions[0] * dimensions[1]).ok_or(Error::Dimensions)?,
            offset:""")
edit(CODEC, """        for _ in 0..self.dimensions[0] {
            let token = tokens.take().ok_or(Error::Samples)?;
            if token.len() <= 64 && finite(token).is_some_and(|sample| sample == self.no_data) {
                return Err(Error::NoData);
            }
            row.push(parse_millimetres(token).ok_or(Error::Samples)?);
        }""", """        let south_first_start = (self.remaining_rows - 1) * self.dimensions[0];
        for column in 0..self.dimensions[0] {
            let decimal = Decimal::parse(tokens.take().ok_or(Error::Samples)?).ok_or(Error::Samples)?;
            if decimal == self.no_data {
                self.missing_samples.mark_missing(south_first_start + column);
                row.push(0);
            } else {
                row.push(decimal.millimetres().ok_or(Error::Samples)?);
            }
        }""")
edit(CODEC, "[usize; 2], f64), Error>", "[usize; 2], Decimal), Error>")
edit(CODEC, 'let no_data = finite(no_data.unwrap_or("-9999")).ok_or(Error::Header)?;', 'let no_data = Decimal::parse(no_data.unwrap_or("-9999")).ok_or(Error::Header)?;')
edit(CODEC, "Ok(ElevationSource { rows_millimetres, bounds: reader.bounds })", "Ok(ElevationSource { rows_millimetres, bounds: reader.bounds, missing_samples: reader.missing_samples.encode() })")
codec = read(CODEC)
decimal = '''#[derive(Clone, Copy, Eq, PartialEq)]
struct Decimal {
    digits: [u8; 64],
    length: usize,
    exponent: i64,
    negative: bool,
}

impl Decimal {
    fn parse(token: &str) -> Option<Self> {
        if token.len() > 64 { return None; }
        let negative = token.starts_with('-');
        let unsigned = token.strip_prefix(['-', '+']).unwrap_or(token);
        let (mantissa, exponent) = match unsigned.split_once(['e', 'E']) {
            Some((mantissa, exponent)) => (mantissa, exponent.parse::<i32>().ok()?),
            None => (unsigned, 0),
        };
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        if whole.is_empty() && fraction.is_empty() { return None; }
        let mut digits = [0; 64];
        let mut length = 0;
        for digit in whole.bytes().chain(fraction.bytes()) {
            if !digit.is_ascii_digit() { return None; }
            if digit == b'0' && length == 0 { continue; }
            digits[length] = digit - b'0';
            length += 1;
        }
        if length == 0 { return Some(Self { digits, length, exponent: 0, negative: false }); }
        let mut exponent = i64::from(exponent) - fraction.len() as i64;
        while digits[length - 1] == 0 {
            length -= 1;
            exponent += 1;
        }
        Some(Self { digits, length, exponent, negative })
    }

    fn millimetres(self) -> Option<i32> {
        if self.length == 0 { return Some(0); }
        let scale = u32::try_from(self.exponent + 3).ok()?;
        if self.length as u64 + u64::from(scale) > 10 { return None; }
        let significant = self.digits[..self.length].iter().fold(0_i64, |value, &digit| value * 10 + i64::from(digit));
        let magnitude = significant * 10_i64.pow(scale);
        i32::try_from(if self.negative { -magnitude } else { magnitude }).ok()
    }
}

#[cfg(test)]
fn parse_millimetres(token: &str) -> Option<i32> {
    Decimal::parse(token)?.millimetres()
}
'''
changes[CODEC] = codec[:codec.index("fn parse_millimetres")] + decimal + codec[codec.index("\nfn write_metres"):]
edit(CODEC, "pub const MAX_HEADER_BYTES", 'const EXPORTED_NODATA: &str = "-9999999";\n\npub const MAX_HEADER_BYTES')
edit(CODEC, "NODATA_VALUE -2147483.649\\n\").expect", "NODATA_VALUE {EXPORTED_NODATA}\\n\").expect")
edit(CODEC, """pub fn write_row(output: &mut String, row: &[i32]) {
    for (index, &cell) in row.iter().enumerate() {
        if index != 0 {
            output.push(' ');
        }
        write_metres(output, cell);
    }""", """pub fn write_row(output: &mut String, source: &ElevationSource, south_first: usize) {
    let row = &source.rows_millimetres[south_first];
    for (column, &cell) in row.iter().enumerate() {
        if column != 0 {
            output.push(' ');
        }
        if semio_framework_surface::terrain::missing_samples::is_missing(source.missing_samples.as_deref(), south_first * row.len() + column) {
            output.push_str(EXPORTED_NODATA);
        } else {
            write_metres(output, cell);
        }
    }""")
edit(CODEC, """    for row in source.rows_millimetres.iter().rev() {
        write_row(&mut grid, row);
    }""", """    for row in (0..rows).rev() {
        write_row(&mut grid, source, row);
    }""")
edit(CODEC, """        assert_eq!(decode(&ascii.replace("nodata_value -2147483.649", "nodata_value -2147483.648"), CAP), Err(Error::NoData));""", """        let missing = decode(&ascii.replace("nodata_value -2147483.649", "nodata_value -2147483.648"), CAP).unwrap();
        assert!(missing.rows_millimetres.iter().flatten().all(|&height| height == 0));
        let mask = missing.missing_samples.as_deref().unwrap();
        assert_eq!(mask.len(), 1058);
        assert!(mask.starts_with("ffffffff") && mask.ends_with("ff01"));
        assert_eq!(decode(&encode(&missing).unwrap(), CAP), Ok(missing));""")

surface = SURFACE / "🦀️.rs"
edit(surface, "/// 🏔️ Rectangular WGS84 sample-centre bounds", '#[path = "🕳️missing-samples/🦀️.rs"]\npub mod missing_samples;\n\n/// 🏔️ Rectangular WGS84 sample-centre bounds')
edit(surface, "    pub rows_millimetres: Vec<Vec<i32>>,\n}", """    pub rows_millimetres: Vec<Vec<i32>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub missing_samples: Option<String>,
}""")
edit(surface, "            && self.dimensions().is_some()", """            && self.dimensions().is_some_and(|[columns, rows]| {
                missing_samples::is_valid(self.missing_samples.as_deref(), (columns * rows) as usize, self.rows_millimetres.iter().flatten().copied())
            })""")
changes[MASK] = changes[MASK].replace(
    "pub fn is_valid(hex: Option<&str>, samples: impl ExactSizeIterator<Item = i32>) -> bool {",
    "pub fn is_valid(hex: Option<&str>, count: usize, mut samples: impl Iterator<Item = i32>) -> bool {",
).replace("    let count = samples.len();\n", "").replace(
    "    for (ordinal, sample) in samples.enumerate() {",
    "    for ordinal in 0..count {\n        let Some(sample) = samples.next() else { return false; };",
).replace(
    "    any_missing && (count % 8",
    "    samples.next().is_none() && any_missing && (count % 8",
)
snapshot = GIS / "🧬️schema/📸️snapshot/🦀️.rs"
edit(snapshot, "use semio_framework_value::FromValue;", """use semio_framework_surface::terrain::missing_samples::{self, MAX_MASK_HEX_BYTES};
use semio_framework_value::FromValue;""")
edit(snapshot, '"elevation source requires ordered WGS84 bounds and a rectangular grid with 2 to 65 samples per axis"', '"elevation source requires ordered WGS84 bounds, a rectangular grid with 2 to 65 samples per axis and canonical missing samples"')
edit(snapshot, "    pub bounds: [f64; 4],\n}", """    pub bounds: [f64; 4],
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub missing_samples: Option<String>,
}""")
edit(snapshot, "            && self.rows_millimetres == other.rows_millimetres", """            && self.rows_millimetres == other.rows_millimetres
            && self.missing_samples == other.missing_samples""")
edit(snapshot, "    rows_millimetres: Vec<Vec<i32>>,\n}", """    rows_millimetres: Vec<Vec<i32>>,
    #[value(default)]
    missing_samples: DecodedMissingSamples,
}""")
edit(snapshot, "ElevationSource { bounds: self.bounds, rows_millimetres: self.rows_millimetres }", "ElevationSource { bounds: self.bounds, rows_millimetres: self.rows_millimetres, missing_samples: self.missing_samples.0 }")
edit(snapshot, '        let members = value.as_object().filter(|members| members.len() == 2).ok_or_else(elevation_shape_error)?;\n        if !members.iter().any(|(name, _)| name == "bounds") {', '''        let members = value.as_object().ok_or_else(elevation_shape_error)?;
        if !(2..=3).contains(&members.len()) || !members.iter().any(|(name, _)| name == "bounds") {''')
edit(snapshot, "        for row in samples {\n            checkpoint()?;", '''        if let Some(mask) = value.get("missingSamples") {
            let DslValue::String(mask) = mask else { return Err(elevation_shape_error()); };
            if mask.len() != (columns * samples.len()).div_ceil(8) * 2 { return Err(elevation_shape_error()); }
        }
        for row in samples {
            checkpoint()?;''')
edit(snapshot, "owned::{NativeReceiveTryMap, NativeTryMappedReceiver, tree::NativeValueSource},", "owned::{NativeMappedReceiver, NativeReceiveMap, NativeReceiveTryMap, NativeTryMappedReceiver, tree::NativeValueSource},")
edit(snapshot, "impl NativeReceiveTryMap<ElevationSourceFields> for ElevationSource {", '''#[doc(hidden)]
#[derive(Default, semio_framework_value::RetireOwned)]
pub struct DecodedMissingSamples(Option<String>);

impl FromValue for DecodedMissingSamples {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        String::from_value(value).map(Self::from_native)
    }

    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        String::from_value_controlled(value, control).map(Self::from_native)
    }

    fn retire_decoded(self) { Option::<String>::retire_decoded(self.0); }
}

impl NativeReceiveMap<String> for DecodedMissingSamples {
    fn from_native(value: String) -> Self { Self(Some(value)) }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for DecodedMissingSamples {
    type Receiver = NativeMappedReceiver<I, String, Self>;

    fn receiver() -> Self::Receiver { Self::Receiver::new() }
}

impl NativeReceiveTryMap<ElevationSourceFields> for ElevationSource {''')
edit(snapshot, "Self::validate_parts(value.bounds, &value.rows_millimetres)", "Self::validate_parts(value.bounds, &value.rows_millimetres, value.missing_samples.0.as_deref())")
edit(snapshot, "            bounds: self.bounds, rows_millimetres: self.rows_millimetres,", "            bounds: self.bounds, rows_millimetres: self.rows_millimetres, missing_samples: DecodedMissingSamples(self.missing_samples),")
edit(snapshot, "Self::validate_parts(self.bounds, &self.rows_millimetres)", "Self::validate_parts(self.bounds, &self.rows_millimetres, self.missing_samples.as_deref())")
edit(snapshot, "fn validate_parts(bounds: [f64; 4], rows: &[Vec<i32>])", "fn validate_parts(bounds: [f64; 4], rows: &[Vec<i32>], mask: Option<&str>)")
edit(snapshot, "Self::valid_parts(bounds, rows).then_some", "Self::valid_parts(bounds, rows, mask).then_some")
edit(snapshot, "Self::valid_parts(self.bounds, &self.rows_millimetres)", "Self::valid_parts(self.bounds, &self.rows_millimetres, self.missing_samples.as_deref())")
edit(snapshot, "fn valid_parts([west, south, east, north]: [f64; 4], rows: &[Vec<i32>])", "fn valid_parts([west, south, east, north]: [f64; 4], rows: &[Vec<i32>], mask: Option<&str>)")
edit(snapshot, "            && Self::dimensions_of(rows).is_some()", """            && Self::dimensions_of(rows).is_some_and(|(width, height)| {
                mask.is_none_or(|text| text.len() <= MAX_MASK_HEX_BYTES)
                    && missing_samples::is_valid(mask, width * height, rows.iter().flatten().copied())
            })""")
edit(snapshot, "            bounds: self.bounds, rows_millimetres: self.rows_millimetres.clone(),", "            bounds: self.bounds, rows_millimetres: self.rows_millimetres.clone(), missing_samples: self.missing_samples.clone(),")

import_command = GIS / "✏️editor/🎮️commands/📥️import-elevation"
work = import_command / "⏩️work/🦀️.rs"
edit(work, "source.bounds.map(f64::to_bits) == bounds.map(f64::to_bits) && source.rows_millimetres.iter().eq(rows.iter().rev())", """source.bounds.map(f64::to_bits) == bounds.map(f64::to_bits)
                && reader.missing_samples.same_encoding(source.missing_samples.as_deref())
                && source.rows_millimetres.iter().eq(rows.iter().rev())""")
edit(work, """        let copy_bytes = size_of::<Step>() + height * size_of::<Vec<i32>>() * 2;
        let capacity_bytes = size_of::<GisTerrainMutation>();""", """        let mask_bytes = reader.missing_samples.encoded_len();
        let copy_bytes = size_of::<Step>() + height * size_of::<Vec<i32>>() * 2 + mask_bytes;
        let capacity_bytes = size_of::<GisTerrainMutation>() + mask_bytes;""")
edit(work, "let source = ElevationSource { rows_millimetres: std::mem::take(rows), bounds };", """let source = ElevationSource { rows_millimetres: std::mem::take(rows), bounds, missing_samples: reader.missing_samples.encode() };""")
edit(work, "cx.consume_fuel(height as u64);", "cx.consume_fuel((height + mask_bytes) as u64);")
command = import_command / "🦀️.rs"
s = read(command)
changes[command] = re.sub(r"^            Self::NoData => .*?\n", "", s, flags=re.M)
edit(command, "LocalizedLabel); 8]", "LocalizedLabel); 7]")
edit(command, "Error::NoData, ", "")
ascii_export = GIS / "✏️editor/🎮️commands/🌍️geojson/🦀️.rs"
edit(ascii_export, "ascii_grid::write_row(&mut fragment, row);", "ascii_grid::write_row(&mut fragment, source, source.rows_millimetres.len() - 1 - owners.rows_written);")

elevation_media = GIS / "✏️editor/📤️media-export/🏔️elevation/🦀️.rs"
edit(elevation_media, "            ArtifactCanonicalJsonNode::I64(value) => JsonWriteNode::Number(Number::Int(value)),", """            ArtifactCanonicalJsonNode::I64(value) => JsonWriteNode::Number(Number::Int(value)),
            ArtifactCanonicalJsonNode::String(value) | ArtifactCanonicalJsonNode::Text(ArtifactCanonicalJsonText::Contiguous(value)) => JsonWriteNode::String(value),""")

schema = GIS / "🧬️schema/🗺️imported-map/🏔️elevation"
edit(schema / "🔣️.json", '    "rowsMillimetres": {', '''    "missingSamples": {
      "description": "Lowercase hexadecimal bytes, south-to-north and west-to-east, least significant bit first. Set bits mark missing samples with a zero height payload. Omit for a fully present grid.",
      "type": "string",
      "pattern": "^(?=[0-9a-f]*[1-9a-f])(?:[0-9a-f]{2})+(?![\\\\s\\\\S])",
      "minLength": 2,
      "maxLength": 1058
    },
    "rowsMillimetres": {''')
edit(schema / "🔗️.graphql", "  rowsMillimetres: [[Int!]!]!\n", "  rowsMillimetres: [[Int!]!]!\n  missingSamples: String\n", 2)
edit(schema / "🛰️.proto", "  repeated ElevationRow rows_millimetres = 2;", "  repeated ElevationRow rows_millimetres = 2;\n  optional string missing_samples = 3;")
ts_schema = schema / "🟦️.ts"
edit(ts_schema, "  readonly rowsMillimetres: readonly (readonly number[])[];", "  readonly rowsMillimetres: readonly (readonly number[])[];\n  readonly missingSamples?: string;")
edit(ts_schema, 'const { bounds, rowsMillimetres } = parseSchemaRecord(input, ["bounds", "rowsMillimetres"]', 'const record = parseSchemaRecord(input, ["bounds", "rowsMillimetres", "missingSamples"]')
edit(ts_schema, '  if (!Array.isArray(bounds)', '  const { bounds, rowsMillimetres, missingSamples } = record;\n  if (!Array.isArray(bounds)')
edit(ts_schema, "  return { bounds: [west, south, east, north], rowsMillimetres };", '''  const source = { bounds: [west, south, east, north] as const, rowsMillimetres };
  if (!Object.hasOwn(record, "missingSamples")) return source;
  const count = rowsMillimetres.length * first.length;
  const byteCount = Math.ceil(count / 8);
  if (typeof missingSamples !== "string" || missingSamples.length !== 2 * byteCount || /[^0-9a-f]/.test(missingSamples) || !/[1-9a-f]/.test(missingSamples)) {
    throw Error("Elevation source requires a canonical missing sample mask.");
  }
  for (let byteIndex = 0; byteIndex < byteCount; byteIndex++) {
    const byte = Number.parseInt(missingSamples.slice(byteIndex * 2, byteIndex * 2 + 2), 16);
    for (let bit = 0; bit < 8; bit++) {
      if ((byte & (1 << bit)) === 0) continue;
      const ordinal = 8 * byteIndex + bit;
      if (ordinal >= count || rowsMillimetres[Math.floor(ordinal / first.length)][ordinal % first.length] !== 0) {
        throw Error("Missing elevation samples require zero payload and padding.");
      }
    }
  }
  return { ...source, missingSamples };''')

sqlite = GIS / "🚪️io/🪶️sqlite/📸️snapshot"
edit(sqlite / "🗄️.sql", "column_count BETWEEN 2 AND 65));", "column_count BETWEEN 2 AND 65), missing_samples TEXT CHECK(missing_samples IS NULL OR length(missing_samples) BETWEEN 2 AND 1058));")
edit(sqlite / "📤️projection/🦀️.rs", "   &[west, south, east, north, Cell::Integer(columns as i64)],", "   &[west, south, east, north, Cell::Integer(columns as i64), source.missing_samples.as_deref().map_or(Cell::Null, Cell::Text)],")
edit(sqlite / "📥️reconstruction/🦀️.rs", "SqliteSnapshotPhase, artifact::FloatRow", "SqliteSnapshotPhase, SqliteValue, artifact::FloatRow")
edit(sqlite / "📥️reconstruction/🦀️.rs", " let source = ElevationSource { bounds, rows_millimetres: elevations };", '''
 let missing_samples = match source.values.get(6) {
  Some(SqliteValue::Null) => None,
  _ => {
   let text = source.text(6)?;
   if text.len() != (columns * height).div_ceil(8) * 2 { return Err(invalid("GIS elevation mask size differs")); }
   Some(owner.text(text)?)
  }
 };
 let source = ElevationSource { bounds, rows_millimetres: elevations, missing_samples };'''.lstrip("\n"))
edit(sqlite / "🦀️.rs", "   bound.add(512)?;", """   bound.add(512)?;
   if let Some(mask) = &source.missing_samples { bound.add(mask.len())?; }""")
edit(sqlite / "🟦️.ts", "const { bounds, rowsMillimetres }", "const { bounds, rowsMillimetres, missingSamples }")
edit(sqlite / "🟦️.ts", "[...bounds.map(encodeElevationBound), BigInt(columns)]", "[...bounds.map(encodeElevationBound), BigInt(columns), missingSamples ?? null]")
edit(sqlite / "🟦️.ts", "4,5,6,4],tables=", "4,5,7,4],tables=")
edit(sqlite / "🟦️.ts", "  result.elevationSource = parseElevationSource({ bounds, rowsMillimetres });", """  const missingSamples = source.values[6] === null ? undefined : text(source, 6);
  result.elevationSource = parseElevationSource(missingSamples === undefined ? { bounds, rowsMillimetres } : { bounds, rowsMillimetres, missingSamples });""")

row_admission = GIS / "🧬️schema/📸️snapshot/🧮️row-admission/🦀️.rs"
edit(row_admission, "   for sample_row in samples {", '''   match source.get(2) {
    None | Some(FieldValue::Absent) => (),
    Some(FieldValue::Text(mask)) if mask.len() == (columns * samples.len()).div_ceil(8) * 2 => (),
    _ => return Err(invalid("terrain native elevation mask size differs")),
   }
   for sample_row in samples {''')

text_snapshot = GIS / "🚪️io/📝️text/📸️snapshot/🦀️.rs"
edit(text_snapshot, "bounds: grid.bounds, rows_millimetres: grid.rows_millimetres.clone() });", "bounds: grid.bounds, rows_millimetres: grid.rows_millimetres.clone(), missing_samples: grid.missing_samples.clone() });")

for root, name in [(GIS, "ElevationSource"), (SURFACE, "TerrainElevationGrid")]:
    for path in root.rglob("*.rs"):
        if not path.is_file() or path in (snapshot, surface, CODEC, work, text_snapshot, sqlite / "📥️reconstruction/🦀️.rs"):
            continue
        text = read(path)
        pattern = rf"(?<![a-zA-Z]){name} \{{(\s*)(?=(?:bounds|rows_millimetres)\b)"
        updated = re.sub(pattern, rf"{name} {{\1missing_samples: None,\1", text)
        if updated != text:
            changes[path] = updated

fixture_name = "🕳️nodata.json"
fixture_path = import_command / "🧪️tests/🧫️fixtures" / fixture_name
changes[fixture_path] = (TICKET / "terrain-nodata.json").read_text(encoding="utf-8")
edit(CODEC, "    #[test]\n    fn exact_signed_millimetres_and_bounded_text()", """    #[test]
    fn nodata_preserves_exact_ordinal_topology_and_signed_samples() {
        let fixture = fixture_text!("../../../../✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🕳️nodata.json");
        let fixture: serde_json::Value = serde_json::from_str(fixture).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let source = decode(case["asciiGrid"].as_str().unwrap(), CAP).unwrap();
            let expected: ElevationSource = semio_framework_pack_json::from_json_str(
                &case["source"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
            ).unwrap();
            assert_eq!(source, expected, "{}", case["name"]);
            assert_eq!(decode(&encode(&source).unwrap(), CAP), Ok(source));
        }
    }

    #[test]
    fn exact_signed_millimetres_and_bounded_text()""")

mutation_test = GIS / "🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"
edit(mutation_test, """    let inverse = replacement.inverse(&snapshot).unwrap();""", """    let mut only_mask = replaced.elevation_source.clone().unwrap();
    only_mask.missing_samples = Some("04".into());
    let mask_change = operation(Some(only_mask.clone()));
    store::os_store::test_support::assert_op_text_binary_equivalence(&mask_change);
    let masked = protocol::apply_diff_cold(mask_change.diff(&replaced).diff(), &replaced).unwrap();
    assert_eq!(masked.elevation_source.as_ref(), Some(&only_mask));
    store::os_store::test_support::assert_dsl_pack_equivalence(&masked);
    let unmask = mask_change.inverse(&replaced).unwrap();
    let unmasked = protocol::apply_diff_cold(unmask[0].diff(&masked).diff(), &masked).unwrap();
    assert_eq!(unmasked.elevation_source, replaced.elevation_source);
    let inverse = replacement.inverse(&snapshot).unwrap();""")

sqlite_test = sqlite / "🧪️tests/🏔️elevation/🟦️.test.ts"
edit(sqlite_test, 'const validate = new Ajv().compile(schema);', 'const validate = new Ajv().addKeyword("x-semio-ui").compile(schema);')
changes[sqlite_test] = read(sqlite_test) + '''
test("SQLite saves exact missing ordinals independently of valid zero and signed extrema", async () => {
  const elevationSource = parseElevationSource({
    bounds: [5, 50, 5.375, 50.25],
    rowsMillimetres: [[-2147483648, 0, 0, 2147483647], [-9999000, 20000, 30000, 40000], [1000, 2000, 0, 4000]],
    missingSamples: "0204",
  });
  expect(validate(elevationSource)).toBe(true);
  for (const missingSamples of [null, "", "0000", "020F", "02f4", "0104", "020400"]) {
    expect(() => parseElevationSource({ ...elevationSource, missingSamples })).toThrow();
  }
  const projected = await gisTerrainSnapshotToSqliteDatabase({ exaggeration: { bits: 0x3ff0000000000000n }, elevationSource });
  const oracle = Database.deserialize(await exportSqliteDatabase(projected));
  try {
    expect(oracle.query("SELECT missing_samples FROM gis_terrain_elevation_source").get()).toEqual({ missing_samples: "0204" });
    expect((await gisTerrainSnapshotFromSqliteDatabase(projected)).elevationSource).toEqual(elevationSource);
    oracle.exec("UPDATE gis_terrain_elevation_source SET missing_samples = '02f4'");
    await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow("padding");
  } finally {
    oracle.close();
  }
});
'''

edit(GIS / "✏️editor/🦀️.rs", "const GIS_ELEVATION_JSON_MAX_BYTES: usize = 8_192;", "const GIS_ELEVATION_JSON_MAX_BYTES: usize = 64 * 1024;")

patch = []
for path, staged in sorted(changes.items(), key=lambda item: str(item[0])):
    assert len(str(path.resolve()).encode()) <= 256, path
    previous = path.read_text(encoding="utf-8") if path.exists() else ""
    if staged == previous:
        continue
    patch.extend(difflib.unified_diff(previous.splitlines(keepends=True), staged.splitlines(keepends=True),
                                     f"a/{path}" if previous else "/dev/null", f"b/{path}", n=3))
(TICKET / "terrain-nodata-canonical-exchange.patch").write_text("".join(patch), encoding="utf-8")
print(f"Staged {len(changes)} paths")

generated = TICKET / "🗑️generated" / "nodata"
generated.mkdir(parents=True, exist_ok=True)
(generated / "ascii.rs").write_text(changes[CODEC], encoding="utf-8")
(generated / "mask.rs").write_text(changes[MASK], encoding="utf-8")
(generated / "schema.json").write_text(changes[schema / "🔣️.json"], encoding="utf-8")
(generated / "sqlite.sql").write_text(changes[sqlite / "🗄️.sql"], encoding="utf-8")
(generated / "source.ts").write_text(changes[ts_schema], encoding="utf-8")
fixture = json.loads(changes[fixture_path])
literal = lambda text: 'r###"' + text + '"###'
cases = []
for case in fixture["cases"]:
    expected = case["source"]
    matrix = "vec![" + ",".join("vec!" + str(row) for row in expected["rowsMillimetres"]) + "]"
    bounds = "[" + ",".join(str(float(value)) for value in expected["bounds"]) + "]"
    mask = 'Some("' + expected["missingSamples"] + '")' if "missingSamples" in expected else "None"
    cases.append(f"({literal(case['name'])},{literal(case['asciiGrid'])},{bounds},{matrix},{mask}),")
harness = '''extern crate self as semio_framework_surface;
#[path = "mask.rs"]
pub mod mask;
pub mod terrain { pub use crate::mask as missing_samples; }
#[path = "ascii.rs"]
mod ascii;

mod schema {
    pub mod snapshot {
        pub const ELEVATION_MAX_SIDE: usize = 65;
        pub const ELEVATION_MIN_SIDE: usize = 2;

        #[derive(Debug, PartialEq)]
        pub struct ElevationSource {
            pub bounds: [f64; 4],
            pub rows_millimetres: Vec<Vec<i32>>,
            pub missing_samples: Option<String>,
        }

        impl ElevationSource {
            pub fn dimensions(&self) -> Option<(usize, usize)> {
                let height = self.rows_millimetres.len();
                let width = self.rows_millimetres.first()?.len();
                (self.rows_millimetres.iter().all(|row| row.len() == width)).then_some((width, height))
            }
            pub fn is_valid(&self) -> bool {
                self.dimensions().is_some_and(|(width, height)| {
                    crate::mask::is_valid(self.missing_samples.as_deref(), width * height, self.rows_millimetres.iter().flatten().copied())
                })
            }
        }
    }
}

fn main() {
    let output = std::env::args().nth(1).expect("generated output directory");
    let cases: [(&str, &str, [f64; 4], Vec<Vec<i32>>, Option<&str>); 5] = [
        @CASES@
    ];
    for (name, text, bounds, samples, mask) in cases {
        let grid = ascii::decode(text, 65_536).unwrap();
        assert_eq!(grid.bounds, bounds, "{name}");
        assert_eq!(grid.rows_millimetres, samples, "{name}");
        assert_eq!(grid.missing_samples.as_deref(), mask, "{name}");
        assert!(grid.is_valid(), "{name}");
        let formatted = ascii::encode(&grid).unwrap();
        assert_eq!(ascii::decode(&formatted, 65_536), Ok(grid), "{name}");
        std::fs::write(std::path::Path::new(&output).join(format!("{name}.asc")), formatted).unwrap();
    }

    let rows = format!("{}\\n", "-7 ".repeat(65)).repeat(65);
    let grid = ascii::decode(&format!("ncols 65 nrows 65 xllcenter 5 yllcenter 50 cellsize .125 nodata_value -7\\n{rows}"), 65_536).unwrap();
    let mask = grid.missing_samples.as_deref().unwrap();
    assert_eq!(mask.len(), 1058);
    assert!(mask.starts_with("ff") && mask.ends_with("ff01"));
    assert!(grid.is_valid());
    assert_eq!(ascii::decode(&ascii::encode(&grid).unwrap(), 65_536), Ok(grid));

    for malformed in ["", "00", "1000", "0F", "10", "ff", "zz"] {
        assert!(!mask::is_valid(Some(malformed), 4, [0, 0, 0, 0].into_iter()));
    }
    assert!(!mask::is_valid(Some("02f4"), 12, [0; 12].into_iter()));
    assert!(!mask::is_valid(Some("01"), 4, [1, 0, 0, 0].into_iter()));
    println!("Staged Rust codec passed 5 fixtures, full mask size, and canonical refusal cases");
}
'''
(generated / "harness.rs").write_text(harness.replace("@CASES@", "\n        ".join(cases)), encoding="utf-8")

sql_ts = changes[sqlite / "🟦️.ts"].replace('../../../🧬️schema/🗺️imported-map/🏔️elevation/🟦️.ts', './source.ts')
sql_ts = sql_ts.replace('./🗄️.sql', './sqlite.sql')
(generated / "sqlite.ts").write_text(sql_ts, encoding="utf-8")
(generated / "fixtures.json").write_text(changes[fixture_path], encoding="utf-8")
(generated / "test.ts").write_text('''
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework/io/sqlite-snapshot";
import { gisTerrainSnapshotFromSqliteDatabase, gisTerrainSnapshotToSqliteDatabase } from "./sqlite";
import { parseElevationSource } from "./source";
import fixture from "./fixtures.json";
import schema from "./schema.json";

const validate = new Ajv().addKeyword("x-semio-ui").compile(schema);
test("NoData shapes round trip through the staged TypeScript and SQLite sources", async () => {
    for (const { source } of fixture.cases) {
        expect(validate(source)).toBe(true);
        const parsed = parseElevationSource(source);
        const projected = await gisTerrainSnapshotToSqliteDatabase({ exaggeration: { bits: 0x3ff0000000000000n }, elevationSource: parsed });
        expect((await gisTerrainSnapshotFromSqliteDatabase(projected)).elevationSource).toEqual(parsed);
        const oracle = Database.deserialize(await exportSqliteDatabase(projected));
        try {
            expect(oracle.query("SELECT missing_samples FROM gis_terrain_elevation_source").get()).toEqual({ missing_samples: parsed.missingSamples ?? null });
            expect(oracle.query("SELECT elevation_millimetres AS height FROM gis_terrain_elevation_sample ORDER BY ordinal").all().map(row => row.height)).toEqual(parsed.rowsMillimetres.flat());
            if (parsed.missingSamples === "0204") {
                oracle.exec("UPDATE gis_terrain_elevation_source SET missing_samples='02f4'");
                await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow("padding");
            }
        } finally {
            oracle.close();
        }
    }
});
''', encoding="utf-8")
