//! 🗺️ Exchanges a rectangular WGS84 Esri ASCII Grid in exact signed millimetres.

use crate::schema::snapshot::{ELEVATION_MAX_SIDE, ELEVATION_MIN_SIDE, ElevationSource};
use semio_framework_surface::terrain::missing_samples::MissingSamples;
use std::fmt::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error { FileSize, Header, Bounds, Dimensions, Samples, Missing, CellShape }

const HEADERS: [&str; 8] = [
    "ncols", "nrows", "xllcorner", "yllcorner", "xllcenter", "yllcenter", "cellsize", "nodata_value",
];

#[derive(Clone, Copy)]
struct Tokens<'a>(&'a str);

impl<'a> Tokens<'a> {
    fn take(&mut self) -> Option<&'a str> {
        let remainder = self.0.trim_ascii_start();
        self.0 = remainder;
        if remainder.is_empty() {
            return None;
        }
        let end = remainder.find(|ch: char| ch.is_ascii_whitespace()).unwrap_or(remainder.len());
        let (token, rest) = remainder.split_at(end);
        self.0 = rest;
        Some(token)
    }
}

/// 🧭️ Holds the sample-centre extent and the position of the next north-first row.
pub struct GridReader {
    pub bounds: [f64; 4],
    pub dimensions: [usize; 2],
    no_data: Decimal,
    pub missing_samples: MissingSamples,
    offset: usize,
    remaining_rows: usize,
}

impl GridReader {
    /// 📋️ Admits the bounded text and reads its complete Esri header.
    pub fn new(text: &str, maximum_bytes: usize) -> Result<Self, Error> {
        if text.is_empty() || text.len() > maximum_bytes {
            return Err(Error::FileSize);
        }
        let mut tokens = Tokens(text.trim_start_matches('\u{feff}'));
        let (bounds, dimensions, no_data) = read_header(&mut tokens)?;
        if dimensions[0] * dimensions[1] > text.len() / 2 + 1 {
            return Err(Error::Samples);
        }
        Ok(Self {
            bounds, dimensions, no_data,
            missing_samples: MissingSamples::new(dimensions[0] * dimensions[1]).ok_or(Error::Dimensions)?,
            offset: text.len() - tokens.0.len(), remaining_rows: dimensions[1],
        })
    }

    /// 📍️ Reports the byte extent already consumed from the original source.
    pub fn offset(&self) -> usize { self.offset }

    /// 🧱️ Reads one exact-width north-first row and rejects trailing samples at the final row.
    pub fn next_row(&mut self, text: &str) -> Result<Option<Vec<i32>>, Error> {
        if self.remaining_rows == 0 {
            return Ok(None);
        }
        let mut tokens = Tokens(text.get(self.offset..).ok_or(Error::Samples)?);
        let mut row = Vec::with_capacity(self.dimensions[0]);
        let south_first_start = (self.remaining_rows - 1) * self.dimensions[0];
        for column in 0..self.dimensions[0] {
            let decimal = Decimal::parse(tokens.take().ok_or(Error::Samples)?).ok_or(Error::Samples)?;
            if decimal == self.no_data {
                self.missing_samples.mark_missing(south_first_start + column);
                row.push(0);
            } else {
                row.push(decimal.millimetres().ok_or(Error::Samples)?);
            }
        }
        if self.remaining_rows == 1 && tokens.take().is_some() {
            return Err(Error::Samples);
        }
        self.remaining_rows -= 1;
        self.offset = text.len() - tokens.0.len();
        Ok(Some(row))
    }
}

fn finite(token: &str) -> Option<f64> {
    token.parse::<f64>().ok().filter(|value| value.is_finite())
}

fn read_header(tokens: &mut Tokens<'_>) -> Result<([f64; 4], [usize; 2], Decimal), Error> {
    let mut header = [None; HEADERS.len()];
    loop {
        let mut probe = *tokens;
        let Some(key) = probe.take().and_then(|token| HEADERS.iter().position(|key| token.eq_ignore_ascii_case(key))) else { break };
        if header[key].is_some() {
            return Err(Error::Header);
        }
        header[key] = Some(probe.take().ok_or(Error::Header)?);
        *tokens = probe;
    }
    let [columns, rows, west, south, west_centre, south_centre, size, no_data] = header;
    let dimension = |token: Option<&str>| {
        let value = token.and_then(|value| value.parse::<usize>().ok()).ok_or(Error::Header)?;
        (ELEVATION_MIN_SIDE..=ELEVATION_MAX_SIDE).contains(&value).then_some(value).ok_or(Error::Dimensions)
    };
    let (columns, rows) = (dimension(columns)?, dimension(rows)?);
    let size = size.and_then(finite).filter(|&value| value > 0.0).ok_or(Error::Header)?;
    let (x, y, centre_offset) = match (west, south, west_centre, south_centre) {
        (Some(x), Some(y), None, None) => (x, y, size * 0.5),
        (None, None, Some(x), Some(y)) => (x, y, 0.0),
        _ => return Err(Error::Header),
    };
    let west = finite(x).ok_or(Error::Header)? + centre_offset;
    let south = finite(y).ok_or(Error::Header)? + centre_offset;
    let east = west + (columns - 1) as f64 * size;
    let north = south + (rows - 1) as f64 * size;
    if !(-180.0..east).contains(&west) || east > 180.0 || !(-90.0..north).contains(&south) || north > 90.0 {
        return Err(Error::Bounds);
    }
    let no_data = Decimal::parse(no_data.unwrap_or("-9999")).ok_or(Error::Header)?;
    Ok(([west, south, east, north], [columns, rows], no_data))
}

/// 📏️ Preserves the sample-centre extent and reverses Esri file rows into canonical south-first rows.
pub fn decode(text: &str, maximum_bytes: usize) -> Result<ElevationSource, Error> {
    let mut reader = GridReader::new(text, maximum_bytes)?;
    let mut rows_millimetres = Vec::with_capacity(reader.dimensions[1]);
    while let Some(row) = reader.next_row(text)? {
        rows_millimetres.push(row);
    }
    rows_millimetres.reverse();
    Ok(ElevationSource { rows_millimetres, bounds: reader.bounds, missing_samples: reader.missing_samples.encode() })
}

#[derive(Clone, Copy, Eq, PartialEq)]
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

fn write_metres(output: &mut String, millimetres: i32) {
    let magnitude = millimetres.unsigned_abs();
    let whole = magnitude / 1000;
    let fraction = magnitude % 1000;
    if millimetres < 0 {
        output.push('-');
    }
    if fraction == 0 {
        write!(output, "{whole}").expect("ASCII Grid cell string");
    } else {
        write!(output, "{whole}.{fraction:03}").expect("ASCII Grid cell string");
    }
}

const EXPORTED_NODATA: &str = "-9999999";

pub const MAX_HEADER_BYTES: usize = 1152;
pub const MAX_CELL_BYTES: usize = 13;

/// 🧾️ Validates the grid extent and writes the shared square-cell Esri header.
pub fn encode_header(source: &ElevationSource) -> Result<String, Error> {
    let (columns, rows) = source.dimensions().ok_or(Error::Dimensions)?;
    if !source.is_valid() {
        return Err(Error::Bounds);
    }
    let [west, south, east, north] = source.bounds;
    let longitude_step = (east - west) / (columns - 1) as f64;
    let latitude_step = (north - south) / (rows - 1) as f64;
    let east_check = west + longitude_step * (columns - 1) as f64;
    let north_check = south + longitude_step * (rows - 1) as f64;
    if longitude_step <= 0.0 || latitude_step <= 0.0
        || (longitude_step - latitude_step).abs() > longitude_step.max(latitude_step) * 1e-9
        || !(-180.0..=180.0).contains(&east_check) || !(-90.0..=90.0).contains(&north_check) {
        return Err(Error::CellShape);
    }

    let mut grid = String::with_capacity(MAX_HEADER_BYTES);
    write!(&mut grid, "NCOLS {columns}\nNROWS {rows}\nXLLCENTER {west}\nYLLCENTER {south}\nCELLSIZE {longitude_step}\nNODATA_VALUE {EXPORTED_NODATA}\n").expect("ASCII Grid header string");
    Ok(grid)
}

/// 🧮️ Appends a validated row of exact signed millimetre samples in west-first order.
pub fn write_row(output: &mut String, source: &ElevationSource, south_first: usize) {
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
    }
    output.push('\n');
}

/// 🧭️ Exports exact millimetre heights in Esri north-first row order.
pub fn encode(source: &ElevationSource) -> Result<String, Error> {
    let mut grid = encode_header(source)?;
    let (columns, rows) = source.dimensions().expect("validated ASCII Grid dimensions");
    grid.reserve(columns * rows * MAX_CELL_BYTES);
    for row in (0..rows).rev() {
        write_row(&mut grid, source, row);
    }
    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_test_fixtures::fixture_text;

    const CAP: usize = 64 * 1024;

    #[test]
    fn rectangular_fractional_samples_and_esri_row_order_round_trip() {
        let fixture = fixture_text!("../../../../✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/📏️fractional.json");
        let fixture: serde_json::Value = serde_json::from_str(fixture).unwrap();
        let raster = fixture["asciiGrid"].as_str().unwrap();
        let source = decode(raster, CAP).unwrap();
        let expected: Vec<Vec<i32>> = serde_json::from_value(fixture["elevation"]["millimetres"].clone()).unwrap();
        let bounds: [f64; 4] = serde_json::from_value(fixture["elevation"]["bounds"].clone()).unwrap();
        assert_eq!(source.rows_millimetres, expected);
        assert!(source.bounds.into_iter().zip(bounds).all(|(actual, expected)| (actual - expected).abs() < 1e-12));
        assert_eq!(source.dimensions(), Some((4, 3)));
        assert_eq!(decode(&encode(&source).unwrap(), CAP), Ok(source));
    }

    #[test]
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
    fn exact_signed_millimetres_and_bounded_text() {
        for (decimal, expected) in [
            ("2147483.647", Some(i32::MAX)), ("-2147483.648", Some(i32::MIN)),
            ("1.25e-1", Some(125)), ("0.00100", Some(1)), (".020", Some(20)),
            ("-0.0000", Some(0)), ("+3E-3", Some(3)),
            ("2147483.648", None), ("0.0009", None), ("1.12500000000000001", None),
            ("2.0e-400", None), ("NaN", None), ("e0", None), ("1.2.3", None),
        ] {
            assert_eq!(parse_millimetres(decimal), expected, "{decimal}");
        }
        let row = format!("{}\n", " -2147483.648".repeat(65));
        let ascii = format!("ncols 65 nrows 65 xllcorner 0 yllcorner 0 cellsize 0.125 nodata_value -2147483.649\n{}", row.repeat(65));
        let source = decode(&ascii, CAP).unwrap();
        assert_eq!(source.dimensions(), Some((65, 65)));
        assert_eq!(source.rows_millimetres[64][64], i32::MIN);
        assert!(encode(&source).unwrap().len() < CAP);
        assert_eq!(decode(&ascii.replace("ncols 65", "ncols 66"), CAP), Err(Error::Dimensions));
        assert_eq!(decode(&ascii.replace("ncols 65", "ncols 1"), CAP), Err(Error::Dimensions));
        let missing = decode(&ascii.replace("nodata_value -2147483.649", "nodata_value -2147483.648"), CAP).unwrap();
        assert!(missing.rows_millimetres.iter().flatten().all(|&height| height == 0));
        let mask = missing.missing_samples.as_deref().unwrap();
        assert_eq!(mask.len(), 1058);
        assert!(mask.starts_with("ffffffff") && mask.ends_with("ff01"));
        assert_eq!(decode(&encode(&missing).unwrap(), CAP), Ok(missing));
    }
}
