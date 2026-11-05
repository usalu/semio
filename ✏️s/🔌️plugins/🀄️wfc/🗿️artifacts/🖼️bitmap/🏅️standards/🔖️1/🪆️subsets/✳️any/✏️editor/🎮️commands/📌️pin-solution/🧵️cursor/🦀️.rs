//! 🧵️ Bounded canonical pin materialization, with one atomic output and paid backing allocations.

use super::*;
use crate::schema::snapshot::BitmapPinnedPixel;
use std::mem::size_of;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};

#[derive(semio_framework_value::RetireOwned)]
enum PinPhase { ReservePins, Pixels, Complete }

/// 🧵️ Owns only unpublished canonical mutations until every output pixel has been checked.
#[derive(semio_framework_value::RetireOwned)]
pub struct BitmapPinCursor {
    phase: PinPhase,
    cells: usize,
    cursor: usize,
    pins: Vec<BitmapPinnedPixel>,
    changed: bool,
}

impl BitmapPinCursor {
    pub fn new(cells: usize) -> Self { Self { phase: PinPhase::ReservePins, cells, cursor: 0, pins: Vec::new(), changed: false } }

    pub fn progress(&self) -> (usize, usize) { (if matches!(self.phase, PinPhase::Pixels | PinPhase::Complete) { self.cursor } else { 0 }, self.cells) }

    pub fn ready_to_publish(&self) -> bool { matches!(self.phase, PinPhase::Complete) }

    pub fn demands(&self) -> RetirementDemand {
        let capacity_bytes = match self.phase { PinPhase::ReservePins => self.cells * size_of::<BitmapPinnedPixel>(), _ => 0 };
        let copy_bytes = match self.phase { PinPhase::Pixels => BITMAP_PIN_WORK_UNITS_PER_TURN * size_of::<BitmapPinnedPixel>(), _ => 0 };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
    }

    pub fn advance(&mut self, snapshot: &BitmapSnapshot, pixels: &str, grant: RetainedCloneGrant) -> Result<(Option<Vec<BitmapPinnedPixel>>, RetainedCloneProgress), ValueError> {
        let demand = self.demands();
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_depth < demand.depth { return Ok((None, Default::default())); }
        let mut progress = RetainedCloneProgress::default();
        let limit = BITMAP_PIN_WORK_UNITS_PER_TURN.min(grant.maximum_items);
        match self.phase {
            PinPhase::ReservePins => {
                self.pins.try_reserve_exact(self.cells).map_err(|_| pin_capacity())?;
                progress.retained_capacity_bytes = self.pins.capacity() * size_of::<BitmapPinnedPixel>();
                progress.copied_items = 1;
                self.phase = PinPhase::Pixels;
            }
            PinPhase::Pixels => {
                let count = limit.min(self.cells - self.cursor);
                let mut colors = [0_u8; BITMAP_PIN_WORK_UNITS_PER_TURN];
                for (offset, color) in colors.iter_mut().take(count).enumerate() {
                    *color = pixel_at(pixels.as_bytes(), self.cursor + offset, self.cells).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "wfc.bitmap.solution.pixels-invalid"))?;
                    if *color as usize >= snapshot.input.palette.len() { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "wfc.bitmap.palette.unknown-color")); }
                }
                for color in colors.into_iter().take(count) {
                    if self.cursor == self.cells { self.phase = PinPhase::Complete; break; }
                    let pin = BitmapPinnedPixel { x: (self.cursor % snapshot.output.width as usize) as u32, y: (self.cursor / snapshot.output.width as usize) as u32, color: u32::from(color) };
                    self.changed |= snapshot.pinned.get(self.cursor) != Some(&pin);
                    self.pins.push(pin);
                    progress.copied_bytes += size_of::<BitmapPinnedPixel>();
                    self.cursor += 1;
                    progress.copied_items += 1;
                }
                if self.cursor == self.cells { self.phase = PinPhase::Complete; }
            }
            PinPhase::Complete => {
                progress.copied_items = 1;
                return Ok((Some(if self.changed || snapshot.pinned.len() != self.cells { std::mem::take(&mut self.pins) } else { Vec::new() }), progress));
            }
        }
        Ok((None, progress))
    }
}

fn pin_capacity() -> ValueError { ValueError::literal(ValueRefusalKind::OwnershipLimit, "bitmap pin backing allocation failed") }

pub(super) fn pixel_at(text: &[u8], index: usize, cells: usize) -> Option<u8> {
    use crate::standards::v1::subsets::any::io::text::snapshot::base64_value;
    let start = index / 3 * 4;
    let quartet = text.get(start..start + 4)?;
    let count = (cells - index / 3 * 3).min(3);
    let a = base64_value(quartet[0])?;
    let b = base64_value(quartet[1])?;
    let c = if count > 1 { base64_value(quartet[2])? } else { if quartet[2] != b'=' || b & 15 != 0 { return None } 0 };
    let d = if count > 2 { base64_value(quartet[3])? } else { if quartet[3] != b'=' || count == 2 && c & 3 != 0 { return None } 0 };
    let triple = a << 18 | b << 12 | c << 6 | d;
    Some((triple >> (16 - (index % 3) * 8)) as u8)
}
