//! 📐️ Operations on the authored elevation samples of a GIS Terrain artifact.

use crate::schema::snapshot::{ELEVATION_MAX_SIDE, ElevationSource};
use semio_framework_surface::terrain::missing_samples::{is_missing, MissingSamples};
use semio_framework_value::{ValueError, ValueRefusalKind};

/// 📏️ Converts an authored metre value when its binary representation identifies an exact millimetre.
pub fn metres_to_millimetres(metres: f64) -> Option<i32> {
    let scaled = (metres * 1000.0).round();
    if !(f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&scaled) {
        return None;
    }
    let millimetres = scaled as i32;
    (f64::from(millimetres) / 1000.0 == metres).then_some(millimetres)
}

pub const LAST_SAMPLE: u32 = ELEVATION_MAX_SIDE as u32 - 1;
pub const MAX_STROKE_SAMPLES: usize = 64;
const GRID_WIDTH: u64 = LAST_SAMPLE as u64 + 1;
const STAMP_WORK: u64 = GRID_WIDTH * GRID_WIDTH * 10;
pub const MAX_STROKE_WORK: u64 = MAX_STROKE_SAMPLES as u64 * STAMP_WORK + 1;
pub const MIN_ELEVATION: i32 = i32::MIN;
pub const MAX_ELEVATION: i32 = i32::MAX;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ElevationEditMode {
    SetSample,
    RaiseLower,
    Flatten,
    Smooth,
    Offset,
    FillMissing,
}

#[derive(Clone, Copy)]
pub struct ElevationEdit {
    pub mode: ElevationEditMode,
    pub column: u32,
    pub row: u32,
    pub radius: u32,
    pub value: i32,
}

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

impl ElevationEdit {
    /// 🎟️ Checks the command's sample coordinates, radius, and value before editing.
    pub fn validate(self) -> Result<Self, ValueError> {
        if [self.row, self.column, self.radius].into_iter().any(|index| index > LAST_SAMPLE) {
            return Err(invalid("gis-terrain-elevation-edit-out-of-range"));
        }
        if self.mode == ElevationEditMode::Smooth && !(0..=100).contains(&self.value) {
            return Err(invalid("gis-terrain-smoothing-strength-out-of-range"));
        }
        Ok(self)
    }

    /// 🛰️ Applies one edit in signed millimetres with quadratic falloff for circular brushes.
    pub fn apply(self, source: &ElevationSource) -> Result<ElevationSource, ValueError> {
        source.validate()?;
        let edit = self.validate()?;
        let (width, height) = source.dimensions().ok_or_else(|| invalid("gis-terrain-elevation-edit-out-of-range"))?;
        if edit.mode != ElevationEditMode::Offset && (edit.column as usize >= width || edit.row as usize >= height) {
            return Err(invalid("gis-terrain-elevation-edit-out-of-range"));
        }
        if edit.mode == ElevationEditMode::SetSample
            && is_missing(source.missing_samples.as_deref(), edit.row as usize * width + edit.column as usize)
        {
            return Err(invalid("gis-terrain-elevation-sample-is-missing"));
        }
        let mut next = source.clone();
        if edit.mode == ElevationEditMode::FillMissing {
            edit.fill_missing(&mut next)?;
        } else {
            edit.write(source, &mut next)?;
        }
        Ok(next)
    }

    /// 🖌️ Applies ordered brush stamps to one private grid, with a second grid for smoothing.
    pub fn apply_stroke(self, source: &ElevationSource, samples: &[[u32; 2]]) -> Result<ElevationSource, ValueError> {
        self.apply_stroke_with(source, samples, || true)
    }

    /// ⏱️ Allows one mounted command to cancel its still-private samples between stamps.
    pub fn apply_stroke_with(self, source: &ElevationSource, samples: &[[u32; 2]], mut active: impl FnMut() -> bool) -> Result<ElevationSource, ValueError> {
        source.validate()?;
        self.validate_stroke_source(source, samples)?;
        let stamps = self.stroke_stamps(samples)?;
        let mut current = source.clone();
        let mut next = source.clone();
        for [column, row] in stamps {
            if !active() {
                return Err(invalid("gis-terrain-elevation-stroke-cancelled"));
            }
            Self { column, row, ..self }.write(&current, &mut next)?;
            std::mem::swap(&mut current, &mut next);
        }
        Ok(current)
    }

    /// 📏️ Expands contiguous picked sample anchors at no more than half-radius spacing.
    pub fn stroke_stamps(self, samples: &[[u32; 2]]) -> Result<Vec<[u32; 2]>, ValueError> {
        let count = self.validate_stroke(samples)?;
        let spacing = self.stroke_spacing();
        let mut stamps = Vec::with_capacity(count);
        stamps.push([self.column, self.row]);
        for pair in samples.windows(2) {
            let [from, to] = [pair[0], pair[1]];
            let steps = Self::stroke_distance(from, to).div_ceil(spacing);
            for step in 1..=steps {
                let sample = |axis: usize| (from[axis] * (steps - step) + to[axis] * step + steps / 2) / steps;
                stamps.push([sample(0), sample(1)]);
            }
        }
        Ok(stamps)
    }

    /// 🎟️ Admits the bounded number of brush stamps without allocating a working grid.
    pub fn validate_stroke(self, samples: &[[u32; 2]]) -> Result<usize, ValueError> {
        self.validate()?;
        if !matches!(self.mode, ElevationEditMode::RaiseLower | ElevationEditMode::Smooth)
            || samples.first() != Some(&[self.column, self.row])
            || samples.len() > MAX_STROKE_SAMPLES
            || samples.iter().flatten().any(|coordinate| *coordinate > LAST_SAMPLE)
        {
            return Err(invalid("gis-terrain-elevation-stroke-invalid"));
        }
        let spacing = self.stroke_spacing();
        let count = 1 + samples.windows(2).map(|pair| Self::stroke_distance(pair[0], pair[1]).div_ceil(spacing) as usize).sum::<usize>();
        if count > MAX_STROKE_SAMPLES {
            return Err(invalid("gis-terrain-elevation-stroke-limit"));
        }
        Ok(count)
    }

    fn validate_stroke_source(self, source: &ElevationSource, samples: &[[u32; 2]]) -> Result<(usize, usize), ValueError> {
        let (width, height) = source.dimensions().ok_or_else(|| invalid("gis-terrain-elevation-stroke-invalid"))?;
        if samples.iter().any(|[column, row]| *column as usize >= width || *row as usize >= height) {
            return Err(invalid("gis-terrain-elevation-stroke-invalid"));
        }
        Ok((width, height))
    }

    /// 🧮️ Quotes one visit per grid sample plus the maximum smoothing neighbors within each brush.
    pub fn stroke_work(self, source: &ElevationSource, samples: &[[u32; 2]]) -> Result<u64, ValueError> {
        let count = self.validate_stroke(samples)? as u64;
        let (width, height) = self.validate_stroke_source(source, samples)?;
        let cells = (width * height) as u64;
        let visits = if self.mode == ElevationEditMode::Smooth {
            let span = u64::from(self.radius) * 2 + 1;
            cells + 9 * cells.min(span * span)
        } else {
            cells
        };
        Ok(count * visits)
    }

    fn stroke_spacing(self) -> u32 {
        ((self.radius + 1) / 2).max(1)
    }

    fn stroke_distance(from: [u32; 2], to: [u32; 2]) -> u32 {
        from[0].abs_diff(to[0]).max(from[1].abs_diff(to[1]))
    }

    fn squared_distance(self, column: usize, row: usize) -> i64 {
        let dx = column as i64 - i64::from(self.column);
        let dy = row as i64 - i64::from(self.row);
        dx * dx + dy * dy
    }

    fn footprint_limit(self) -> i64 {
        i64::from((self.radius + 1) * (self.radius + 1))
    }

    fn fill_missing(self, next: &mut ElevationSource) -> Result<(), ValueError> {
        let Some(mask) = next.missing_samples.as_deref() else { return Ok(()); };
        let width = next.rows_millimetres[0].len();
        let mut missing = MissingSamples::new(width * next.rows_millimetres.len())
            .ok_or_else(|| invalid("gis-terrain-elevation-edit-out-of-range"))?;
        let limit = self.footprint_limit();
        for (y, row) in next.rows_millimetres.iter_mut().enumerate() {
            for (x, height) in row.iter_mut().enumerate() {
                let ordinal = y * width + x;
                if is_missing(Some(mask), ordinal) {
                    if self.squared_distance(x, y) < limit {
                        *height = self.value;
                    } else {
                        missing.mark_missing(ordinal);
                    }
                }
            }
        }
        next.missing_samples = missing.encode();
        Ok(())
    }

    fn write(self, source: &ElevationSource, next: &mut ElevationSource) -> Result<(), ValueError> {
        let edit = self;
        let limit = edit.footprint_limit();
        let value = i64::from(edit.value);
        let mask = source.missing_samples.as_deref();
        for (y, row) in next.rows_millimetres.iter_mut().enumerate() {
            let row_start = y * row.len();
            for (x, height) in row.iter_mut().enumerate() {
                if is_missing(mask, row_start + x) {
                    continue;
                }
                let distance = edit.squared_distance(x, y);
                let weight = limit - distance;
                let current = i64::from(source.rows_millimetres[y][x]);
                let updated = match edit.mode {
                    ElevationEditMode::SetSample if distance == 0 => value,
                    ElevationEditMode::Offset => current + value,
                    ElevationEditMode::RaiseLower if weight > 0 => current + rounded_ratio(value * weight, limit),
                    ElevationEditMode::Flatten if weight > 0 => current + rounded_ratio((value - current) * weight, limit),
                    ElevationEditMode::Smooth if weight > 0 => current + smooth_delta(&source.rows_millimetres, mask, x, y, current, value * weight, limit),
                    _ => current,
                };
                *height = i32::try_from(updated).map_err(|_| invalid("gis-terrain-elevation-sample-overflow"))?;
            }
        }
        Ok(())
    }
}

fn rounded_ratio(numerator: i64, denominator: i64) -> i64 {
    (numerator.abs() + denominator / 2) / denominator * numerator.signum()
}

fn smooth_delta(rows: &[Vec<i32>], mask: Option<&str>, x: usize, y: usize, current: i64, weight: i64, limit: i64) -> i64 {
    let mut sum = 0;
    let mut count = 0;
    for neighbor_y in y.saturating_sub(1)..=(y + 1).min(rows.len() - 1) {
        let row = &rows[neighbor_y];
        for neighbor_x in x.saturating_sub(1)..=(x + 1).min(row.len() - 1) {
            if !is_missing(mask, neighbor_y * row.len() + neighbor_x) {
                sum += i64::from(row[neighbor_x]);
                count += 1;
            }
        }
    }
    rounded_ratio((sum - count * current) * weight, count * limit * 100)
}
