//! 🗺️ Decodes and resamples a bounded Esri ASCII Grid onto the terrain’s WGS84 sample lattice.

use crate::schema::snapshot::ElevationSource;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error { FileSize, Header, Bounds, Samples, NoData, Missing, CellShape }
use std::fmt::Write;

const SIDE: usize = 33;
const HEADERS: [&str; 8] = [
    "ncols", "nrows", "xllcorner", "yllcorner", "xllcenter", "yllcenter", "cellsize", "nodata_value",
];

/// 📏️ Preserves the outer sample centres and interpolates on the terrain’s shared sample lattice.
pub fn decode(text: &str, maximum_bytes: usize) -> Result<ElevationSource, Error> {
    if text.is_empty() || text.len() > maximum_bytes {
        return Err(Error::FileSize);
    }
    let mut tokens = text.trim_start_matches('\u{feff}').split_ascii_whitespace().peekable();
    let mut header = [None; HEADERS.len()];
    while let Some(key) = tokens.peek().and_then(|token| HEADERS.iter().position(|key| token.eq_ignore_ascii_case(key))) {
        tokens.next();
        if header[key].is_some() {
            return Err(Error::Header);
        }
        header[key] = Some(tokens.next().ok_or(Error::Header)?);
    }
    let [columns, rows, west, south, west_centre, south_centre, size, no_data] = header;
    let dimension = |token: Option<&str>| token.and_then(|value| value.parse::<usize>().ok()).filter(|&value| value > 0).ok_or(Error::Header);
    let number = |token: &str| token.parse::<f64>().ok().filter(|value| value.is_finite());
    let (columns, rows) = (dimension(columns)?, dimension(rows)?);
    columns.checked_mul(rows).filter(|&count| count <= text.len() / 2 + 1).ok_or(Error::Samples)?;
    let size = size.and_then(number).filter(|&value| value > 0.0).ok_or(Error::Header)?;
    let (x, y, centre_offset) = match (west, south, west_centre, south_centre) {
        (Some(x), Some(y), None, None) => (x, y, size * 0.5),
        (None, None, Some(x), Some(y)) => (x, y, 0.0),
        _ => return Err(Error::Header),
    };
    let [west, east] = sample_bounds(number(x).ok_or(Error::Header)? + centre_offset, columns, size);
    let [south, north] = sample_bounds(number(y).ok_or(Error::Header)? + centre_offset, rows, size);
    if !(-180.0..east).contains(&west) || east > 180.0 || !(-90.0..north).contains(&south) || north > 90.0 {
        return Err(Error::Bounds);
    }
    let no_data = number(no_data.unwrap_or("-9999")).ok_or(Error::Header)?;
    let horizontal = lattice(columns);
    let vertical = lattice(rows);
    let mut previous_row = [0.0; SIDE];
    let mut elevations = Vec::with_capacity(SIDE);
    for row in 0..rows {
        let needed = vertical.get(elevations.len()).is_some_and(|&(upper, lower, _)| upper == row || lower == row);
        let mut projected_row = [0.0; SIDE];
        let mut next_column = 0;
        let mut previous_cell = 0.0;
        for column in 0..columns {
            let sample = tokens.next().and_then(number).ok_or(Error::Samples)?;
            if sample == no_data {
                return Err(Error::NoData);
            }
            if !(f64::from(i16::MIN)..=f64::from(i16::MAX)).contains(&sample) {
                return Err(Error::Samples);
            }
            if needed {
                while let Some(&(left, _, weight)) = horizontal.get(next_column).filter(|&&(_, right, _)| right == column) {
                    let first = if left == column { sample } else { previous_cell };
                    projected_row[next_column] = interpolate(first, sample, weight);
                    next_column += 1;
                }
                previous_cell = sample;
            }
        }
        if needed {
            while let Some(&(upper, _, weight)) = vertical.get(elevations.len()).filter(|&&(_, lower, _)| lower == row) {
                let first = if upper == row { &projected_row } else { &previous_row };
                elevations.push(first.iter().zip(projected_row).map(|(&north, south)| interpolate(north, south, weight).round() as i16).collect());
            }
            previous_row = projected_row;
        }
    }
    if tokens.next().is_some() {
        return Err(Error::Samples);
    }
    elevations.reverse();
    Ok(ElevationSource { bounds: [west, south, east, north], rows: elevations })
}

/// 🧾️ Encodes exact sample positions in Esri’s cell-centre convention when the cells are square.
pub fn encode(source: &ElevationSource) -> Result<String, Error> {
    if !source.is_valid() {
        return Err(Error::Bounds);
    }
    let [west, south, east, north] = source.bounds;
    let longitude_step = (east - west) / (SIDE - 1) as f64;
    let latitude_step = (north - south) / (SIDE - 1) as f64;
    let east_check = west + longitude_step * (SIDE - 1) as f64;
    let north_check = south + longitude_step * (SIDE - 1) as f64;
    if longitude_step <= 0.0 || latitude_step <= 0.0
        || (longitude_step - latitude_step).abs() > longitude_step.max(latitude_step) * 1e-9
        || !(-180.0..=180.0).contains(&east_check) || !(-90.0..=90.0).contains(&north_check) {
        return Err(Error::CellShape);
    }
    let mut grid = String::with_capacity(8_192);
    write!(&mut grid, "NCOLS {SIDE}\nNROWS {SIDE}\nXLLCENTER {west}\nYLLCENTER {south}\nCELLSIZE {longitude_step}\nNODATA_VALUE -32769\n").expect("ASCII Grid header string");
    for row in source.rows.iter().rev() {
        let mut cells = row.iter();
        let first = cells.next().expect("validated ASCII Grid row");
        write!(&mut grid, "{first}").expect("ASCII Grid cell string");
        for cell in cells {
            write!(&mut grid, " {cell}").expect("ASCII Grid cell string");
        }
        grid.push('\n');
    }
    Ok(grid)
}

fn sample_bounds(origin: f64, cells: usize, size: f64) -> [f64; 2] {
    if cells == 1 {
        [origin - size * 0.5, origin + size * 0.5]
    } else {
        [origin, origin + (cells - 1) as f64 * size]
    }
}

fn lattice(cells: usize) -> [(usize, usize, f64); SIDE] {
    std::array::from_fn(|index| {
        let position = index as f64 * (cells - 1) as f64 / (SIDE - 1) as f64;
        let first = position as usize;
        (first, (position.ceil() as usize).min(cells - 1), position.fract())
    })
}

fn interpolate(first: f64, second: f64, fraction: f64) -> f64 {
    first + (second - first) * fraction
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_test_fixtures::fixture_text;

    const MAX_FILE_BYTES: usize = 24 * 1024;
    fn decode_fixture(text: &str) -> Result<ElevationSource, Error> { decode(text, MAX_FILE_BYTES) }

    static SLOPE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🔼️slope.asc"));
    static EXPECTED: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🔣️.json"));

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RasterFixture {
        bounds: [f64; 4],
        side: usize,
        checkpoints: Vec<(usize, usize, i16)>,
        checksum: i32,
        squared_checksum: i64,
    }

    #[test]
    fn esri_ascii_grid_retains_sample_extent_with_south_first_artifact_rows() {
        let fixture: RasterFixture = serde_json::from_str(*EXPECTED).unwrap();
        let source = decode_fixture(*SLOPE).unwrap();
        assert_eq!(source.bounds, fixture.bounds);
        assert!(source.is_valid());
        assert_eq!(source.rows.len(), fixture.side);
        for (row, column, height) in fixture.checkpoints {
            assert_eq!(source.rows[row][column], height);
        }
        assert_eq!(source.rows.iter().flatten().copied().map(i32::from).sum::<i32>(), fixture.checksum);
        assert_eq!(source.rows.iter().flatten().map(|&height| i64::from(height).pow(2)).sum::<i64>(), fixture.squared_checksum);
        let centres = SLOPE.replace("XLLCORNER 6.5", "xllcenter 6.625").replace("YLLCORNER 44.5", "yllcenter 44.625");
        assert_eq!(decode_fixture(&format!("\u{feff}{}", centres.replace('\n', "\r\n"))), Ok(source));
    }

    #[test]
    fn esri_ascii_grid_retains_cell_width_on_singleton_axes() {
        let single = "ncols 1 nrows 1 xllcorner 6.5 yllcorner 44.5 cellsize 0.25 -6.5";
        let source = decode_fixture(single).unwrap();
        assert_eq!(source.bounds, [6.5, 44.5, 6.75, 44.75]);
        assert!(source.rows.iter().flatten().all(|&height| height == -7));
    }

    #[test]
    fn esri_ascii_grid_refuses_invalid_extent_missing_data_and_malformed_samples() {
        for (file, expected) in [
            (SLOPE.replace("6.5", "700000"), Error::Bounds),
            (SLOPE.replace("XLLCORNER", "XLLCENTER"), Error::Header),
            (SLOPE.replace("NROWS 3", "NROWS 3 NROWS 2"), Error::Header),
            (SLOPE.replace("NROWS 3", "NROWS 0"), Error::Header),
            (SLOPE.replace("-32", "-9999"), Error::NoData),
            (SLOPE.replace("NODATA_VALUE -9999", "").replace("-32", "-9999"), Error::NoData),
            (SLOPE.replace("-32", "NaN"), Error::Samples),
            (SLOPE.replace("-32", "32768"), Error::Samples),
            (SLOPE.replace("-32", ""), Error::Samples),
            (format!("{} 17", *SLOPE), Error::Samples),
            (" ".repeat(MAX_FILE_BYTES + 1), Error::FileSize),
        ] {
            assert_eq!(decode_fixture(&file), Err(expected));
        }
    }

    #[test]
    fn esri_ascii_export_preserves_centres_and_all_signed_heights() {
        let mut source = decode_fixture(*SLOPE).unwrap();
        source.rows[0][0] = i16::MIN;
        source.rows[0][1] = -9999;
        source.rows[0][2] = i16::MAX;
        let encoded = encode(&source).unwrap();
        assert!(encoded.starts_with("NCOLS 33\nNROWS 33\nXLLCENTER 6.625\nYLLCENTER 44.625\nCELLSIZE 0.015625\nNODATA_VALUE -32769\n"));
        assert_eq!(decode_fixture(&encoded), Ok(source.clone()));
        source.bounds[2] += 0.25;
        assert_eq!(encode(&source), Err(Error::CellShape));
    }
}
