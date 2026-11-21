//! 📐️ Operations on the authored elevation samples of a GIS Terrain artifact.

use crate::schema::snapshot::ElevationSource;
use semio_framework_value::{ValueError, ValueRefusalKind};

pub const LAST_SAMPLE: u32 = 32;
pub const MAX_STROKE_SAMPLES: usize = 64;
const GRID_WIDTH: u64 = LAST_SAMPLE as u64 + 1;
const STAMP_WORK: u64 = GRID_WIDTH * GRID_WIDTH * 10;
pub const MAX_STROKE_WORK: u64 = MAX_STROKE_SAMPLES as u64 * STAMP_WORK + 1;
pub const MIN_ELEVATION: i32 = i16::MIN as i32;
pub const MAX_ELEVATION: i32 = i16::MAX as i32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ElevationEditMode {
    SetSample,
    RaiseLower,
    Flatten,
    Smooth,
    Offset,
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
        if [self.row, self.column, self.radius].into_iter().any(|index| index > LAST_SAMPLE)
            || !(MIN_ELEVATION..=MAX_ELEVATION).contains(&self.value) {
            return Err(invalid("gis-terrain-elevation-edit-out-of-range"));
        }
        if self.mode == ElevationEditMode::Smooth && !(0..=100).contains(&self.value) {
            return Err(invalid("gis-terrain-smoothing-strength-out-of-range"));
        }
        Ok(self)
    }

    /// 🛰️ Applies one edit in raw metres with quadratic falloff for circular brushes.
    pub fn apply(self, source: &ElevationSource) -> Result<ElevationSource, ValueError> {
        source.validate()?;
        let edit = self.validate()?;
        let mut next = source.clone();
        edit.write(source, &mut next)?;
        Ok(next)
    }

    /// 🖌️ Applies ordered brush stamps to one private grid, with a second grid for smoothing.
    pub fn apply_stroke(self, source: &ElevationSource, samples: &[[u32; 2]]) -> Result<ElevationSource, ValueError> {
        self.apply_stroke_with(source, samples, || true)
    }

    /// ⏱️ Allows one mounted command to cancel its still-private samples between stamps.
    pub fn apply_stroke_with(self, source: &ElevationSource, samples: &[[u32; 2]], mut active: impl FnMut() -> bool) -> Result<ElevationSource, ValueError> {
        source.validate()?;
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

    /// 🧮️ Charges each visited sample and up to nine smoothing neighbors.
    pub fn stroke_work(self, samples: &[[u32; 2]]) -> Result<u64, ValueError> {
        Ok(self.validate_stroke(samples)? as u64 * STAMP_WORK)
    }

    fn stroke_spacing(self) -> u32 {
        ((self.radius + 1) / 2).max(1)
    }

    fn stroke_distance(from: [u32; 2], to: [u32; 2]) -> u32 {
        from[0].abs_diff(to[0]).max(from[1].abs_diff(to[1]))
    }

    fn write(self, source: &ElevationSource, next: &mut ElevationSource) -> Result<(), ValueError> {
        let edit = self;
        let limit = ((edit.radius + 1) * (edit.radius + 1)) as i32;
        for (y, row) in next.rows.iter_mut().enumerate() {
            for (x, height) in row.iter_mut().enumerate() {
                let dx = x as i32 - edit.column as i32;
                let dy = y as i32 - edit.row as i32;
                let distance = dx * dx + dy * dy;
                let weight = limit - distance;
                let current = i32::from(source.rows[y][x]);
                let updated = match edit.mode {
                    ElevationEditMode::SetSample if distance == 0 => edit.value,
                    ElevationEditMode::Offset => current + edit.value,
                    ElevationEditMode::RaiseLower if weight > 0 => current + rounded_ratio(edit.value * weight, limit),
                    ElevationEditMode::Flatten if weight > 0 => current + rounded_ratio((edit.value - current) * weight, limit),
                    ElevationEditMode::Smooth if weight > 0 => current + smooth_delta(&source.rows, x, y, current, edit.value * weight, limit),
                    _ => current,
                };
                *height = i16::try_from(updated).map_err(|_| invalid("gis-terrain-elevation-sample-overflow"))?;
            }
        }
        Ok(())
    }
}

fn rounded_ratio(numerator: i32, denominator: i32) -> i32 {
    (numerator.abs() + denominator / 2) / denominator * numerator.signum()
}

fn smooth_delta(rows: &[Vec<i16>], x: usize, y: usize, current: i32, weight: i32, limit: i32) -> i32 {
    let mut sum = 0;
    let mut count = 0;
    for row in &rows[y.saturating_sub(1)..=(y + 1).min(rows.len() - 1)] {
        for &height in &row[x.saturating_sub(1)..=(x + 1).min(row.len() - 1)] {
            sum += i32::from(height);
            count += 1;
        }
    }
    let numerator = i64::from(sum - count * current) * i64::from(weight);
    let denominator = i64::from(count * limit * 100);
    ((numerator.abs() + denominator / 2) / denominator * numerator.signum()) as i32
}
