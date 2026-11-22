"""🏔️ Prepare the exact rectangular Esri ASCII Grid codec changes."""

import difflib
import json
from pathlib import Path


TICKET = Path(__file__).resolve().parent
ARTIFACT = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any")
CODEC = ARTIFACT / "🚪️io/📝️text/🏔️ascii-grid/🦀️.rs"
FIXTURE = ARTIFACT / "✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/📏️fractional.json"
IMPORT = ARTIFACT / "✏️editor/🎮️commands/📥️import-elevation/🦀️.rs"
REPLACEMENT = r'''//! 🗺️ Exchanges a rectangular WGS84 Esri ASCII Grid in exact signed millimetres.

use crate::schema::snapshot::{ELEVATION_MAX_SIDE, ELEVATION_MIN_SIDE, ElevationSource};
use std::fmt::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error { FileSize, Header, Bounds, Dimensions, Samples, NoData, Missing, CellShape }

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
    no_data: f64,
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
            bounds, dimensions, no_data, offset: text.len() - tokens.0.len(), remaining_rows: dimensions[1],
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
        for _ in 0..self.dimensions[0] {
            let token = tokens.take().ok_or(Error::Samples)?;
            if token.len() <= 64 && finite(token).is_some_and(|sample| sample == self.no_data) {
                return Err(Error::NoData);
            }
            row.push(parse_millimetres(token).ok_or(Error::Samples)?);
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

fn read_header(tokens: &mut Tokens<'_>) -> Result<([f64; 4], [usize; 2], f64), Error> {
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
    let no_data = finite(no_data.unwrap_or("-9999")).ok_or(Error::Header)?;
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
    Ok(ElevationSource { rows_millimetres, bounds: reader.bounds })
}

fn parse_millimetres(token: &str) -> Option<i32> {
    if token.len() > 64 {
        return None;
    }
    let (negative, unsigned) = if let Some(unsigned) = token.strip_prefix('-') {
        (true, unsigned)
    } else {
        (false, token.strip_prefix('+').unwrap_or(token))
    };
    let (mantissa, exponent) = match unsigned.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (mantissa, exponent.parse::<i32>().ok()?),
        None => (unsigned, 0),
    };
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    if whole.is_empty() && fraction.is_empty() {
        return None;
    }
    let mut magnitude = 0_i64;
    let mut trailing_zeros = 0_i32;
    for digit in whole.bytes().chain(fraction.bytes()) {
        let value = i64::from(digit.checked_sub(b'0')?);
        if value > 9 {
            return None;
        }
        if value == 0 {
            if magnitude > 0 {
                trailing_zeros += 1;
            }
        } else {
            magnitude = magnitude.checked_mul(10_i64.checked_pow((trailing_zeros + 1) as u32)?)?.checked_add(value)?;
            trailing_zeros = 0;
        }
    }
    if magnitude == 0 {
        return Some(0);
    }
    let scale = exponent.checked_add(3)?.checked_sub(fraction.len() as i32)?.checked_add(trailing_zeros)?;
    let millimetres = magnitude.checked_mul(10_i64.checked_pow(u32::try_from(scale).ok()?)?)?;
    i32::try_from(if negative { -millimetres } else { millimetres }).ok()
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
    write!(&mut grid, "NCOLS {columns}\nNROWS {rows}\nXLLCENTER {west}\nYLLCENTER {south}\nCELLSIZE {longitude_step}\nNODATA_VALUE -2147483.649\n").expect("ASCII Grid header string");
    Ok(grid)
}

/// 🧮️ Appends a validated row of exact signed millimetre samples in west-first order.
pub fn write_row(output: &mut String, row: &[i32]) {
    for (index, &cell) in row.iter().enumerate() {
        if index != 0 {
            output.push(' ');
        }
        write_metres(output, cell);
    }
    output.push('\n');
}

/// 🧭️ Exports exact millimetre heights in Esri north-first row order.
pub fn encode(source: &ElevationSource) -> Result<String, Error> {
    let mut grid = encode_header(source)?;
    let (columns, rows) = source.dimensions().expect("validated ASCII Grid dimensions");
    grid.reserve(columns * rows * MAX_CELL_BYTES);
    for row in source.rows_millimetres.iter().rev() {
        write_row(&mut grid, row);
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
        assert_eq!(decode(&ascii.replace("nodata_value -2147483.649", "nodata_value -2147483.648"), CAP), Err(Error::NoData));
    }
}
'''


def patch_lines(path, target):
    """📐 Compare the proposed implementation against the exact live source."""
    previous = path.read_text(encoding="utf-8") if path.exists() else ""
    return difflib.unified_diff(
        previous.splitlines(keepends=True),
        target.splitlines(keepends=True),
        "a/" + path.as_posix() if path.exists() else "/dev/null",
        "b/" + path.as_posix(),
    )


fixture = json.loads((TICKET / "terrain-fractional-grid.json").read_text(encoding="utf-8"))
output = TICKET / "terrain-rectangular-ascii.patch"
with output.open("w", encoding="utf-8", newline="\n") as patch:
    patch.writelines(patch_lines(CODEC, REPLACEMENT))
    patch.writelines(patch_lines(FIXTURE, json.dumps(fixture, ensure_ascii=False, indent=2) + "\n"))
    source = IMPORT.read_text(encoding="utf-8")
    lines = source.splitlines(keepends=True)
    for index, line in enumerate(lines):
        if "Self::Samples =>" in line:
            lines[index] = (
                '            Self::Dimensions => ("gis.terrain.import.dimensions", "Choose an elevation grid with 2 to 65 samples on each axis.", "Wähle ein Höhenraster mit 2 bis 65 Rasterpunkten pro Achse."),\n'
                '            Self::Samples => ("gis.terrain.import.samples", "Supply the declared number of samples in metres with exact millimetre precision.", "Gib die angegebene Anzahl von Höhenwerten in Metern mit Millimetergenauigkeit an."),\n'
            )
    source = "".join(lines).replace("LocalizedLabel); 7]", "LocalizedLabel); 8]").replace(
        "Error::Header, Error::Bounds, Error::Samples", "Error::Header, Error::Bounds, Error::Dimensions, Error::Samples")
    source = source.replace("trait ErrorNotice {", "pub(super) trait ErrorNotice {")
    source = source.replace("mod ascii_grid {", "pub(super) mod ascii_grid {")
    source = source.replace("pub(super) use semio_s_artifact_gis_gisterrain_inferences", "pub(in super::super) use semio_s_artifact_gis_gisterrain_inferences")
    source = source.replace("{Error, encode};", "{Error, GridReader, MAX_CELL_BYTES, MAX_HEADER_BYTES, encode, encode_header, write_row};")
    patch.writelines(patch_lines(IMPORT, source))
print("[DEBUG] Prepared " + output.name)
