//! 🛬️ Receives the tagged Bitmap operation and advances base64 and column conversions under the original caller.

use super::{BitmapOperationDsl, COMPONENT_PROTOCOL_SEMIO, operation_from_dsl};
use crate::schema::mutations::{
    BitmapMutation, paint_input_stroke::{BitmapStrokePoint, PaintInputStroke},
    pin_solution::PinSolution, restore_pins::RestorePins, set_input_pixels::SetInputPixels,
};
use crate::schema::snapshot::BitmapPinnedPixel;
use dsl::original_receiving::converted::OriginalConvertedOperation;
use semio_framework_value::{
    NativeDecodeControl, RetainedAllocationEffect, RetainedCloneProgress, RetireOwned,
    RetirementDemand, ValueError, ValueRefusalKind, bytes::decode_standard_quad,
};
use std::mem::{size_of, take};

dsl::original_tagged_operation_receiving!(BitmapOperationDsl, COMPONENT_PROTOCOL_SEMIO);

#[derive(Default, RetireOwned)]
pub struct Conversion {
    pixels: Vec<u8>,
    points: Vec<BitmapStrokePoint>,
    pins: Vec<BitmapPinnedPixel>,
    source_offset: usize,
}

fn invalid_pixels() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Bitmap pixels require canonical base64")
}

fn decoded_word(bytes: &[u8], offset: usize) -> Result<([u8; 3], usize), ValueError> {
    let quad = bytes.get(offset..).and_then(|tail| tail.first_chunk::<4>()).ok_or_else(invalid_pixels)?;
    decode_standard_quad(*quad, offset, offset + 4 == bytes.len()).map_err(|_| invalid_pixels())
}

fn pixel_count(encoded: &str) -> Result<usize, ValueError> {
    if encoded.is_empty() {
        return Ok(0);
    }
    if !encoded.len().is_multiple_of(4) {
        return Err(invalid_pixels());
    }
    let last = encoded.len() - 4;
    let (_, tail) = decoded_word(encoded.as_bytes(), last)?;
    Ok(last / 4 * 3 + tail)
}

fn pin_count(xs: &[u32], ys: &[u32], colors: &[u32]) -> Result<usize, ValueError> {
    if xs.len() != ys.len() || xs.len() != colors.len() || xs.len() > 512 * 512 {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "pin batch columns differ or exceed the schema capacity"));
    }
    Ok(xs.len())
}

fn rows_demand<T>(rows: &Vec<T>, total: usize) -> Result<RetirementDemand, ValueError> {
    let capacity_bytes = if rows.capacity() == 0 {
        total.checked_mul(size_of::<T>()).ok_or_else(|| ValueError::invariant("Bitmap conversion row capacity overflow"))?
    } else {
        0
    };
    let copy_bytes = if capacity_bytes != 0 {
        size_of::<Vec<T>>()
    } else if rows.len() < total {
        size_of::<T>()
    } else {
        size_of::<Option<BitmapMutation>>()
    };
    Ok(RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() })
}

fn reserve<T>(rows: &mut Vec<T>, count: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<usize, ValueError> {
    *effect.actual() = Some(0);
    rows.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "Bitmap conversion row allocation refused"))?;
    let capacity = rows.capacity() * size_of::<T>();
    *effect.actual() = Some(capacity);
    Ok(capacity)
}

impl OriginalConvertedOperation for BitmapMutation {
    type Mirror = BitmapOperationDsl;
    type Conversion = Conversion;

    fn conversion_demand(state: &Conversion, input: &Option<BitmapOperationDsl>, _: usize) -> Result<RetirementDemand, ValueError> {
        use BitmapOperationDsl::*;
        match input.as_ref() {
            Some(SetInputPixels { pixels, .. }) => {
                let mut demand = rows_demand(&state.pixels, pixel_count(pixels)?)?;
                if demand.capacity_bytes == 0 && state.source_offset < pixels.len() {
                    demand.copy_bytes = decoded_word(pixels.as_bytes(), state.source_offset)?.1;
                }
                Ok(demand)
            }
            Some(PaintInputStroke { xs, ys, .. }) => rows_demand(&state.points, xs.len().min(ys.len())),
            Some(PinSolution { xs, ys, colors } | RestorePins { xs, ys, colors }) => rows_demand(&state.pins, pin_count(xs, ys, colors)?),
            Some(_) => Ok(RetirementDemand {
                copy_bytes: size_of::<Option<BitmapOperationDsl>>() + size_of::<Option<Self>>(), depth: 1,
                ..Default::default()
            }),
            None => Err(ValueError::invariant("Bitmap conversion input is absent")),
        }
    }

    fn convert_step(
        state: &mut Conversion, input: &mut Option<BitmapOperationDsl>, output: &mut Option<Self>, body: usize,
        _: &mut NativeDecodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        let demand = Self::conversion_demand(state, input, body)?;
        if !effect.performed(1)? {
            return Ok(Default::default());
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        match input.as_ref().expect("validated Bitmap conversion input") {
            BitmapOperationDsl::SetInputPixels { x, y, width, height, pixels } => {
                if demand.capacity_bytes != 0 {
                    progress.retained_capacity_bytes = reserve(&mut state.pixels, pixel_count(pixels)?, effect)?;
                } else if state.source_offset < pixels.len() {
                    let (bytes, length) = decoded_word(pixels.as_bytes(), state.source_offset)?;
                    state.pixels.extend_from_slice(&bytes[..length]);
                    state.source_offset += 4;
                } else {
                    *output = Some(Self::SetInputPixels(SetInputPixels { x: *x, y: *y, width: *width, height: *height, pixels: take(&mut state.pixels) }));
                }
            }
            BitmapOperationDsl::PaintInputStroke { color, xs, ys } => {
                let length = xs.len().min(ys.len());
                let index = state.points.len();
                if demand.capacity_bytes != 0 {
                    progress.retained_capacity_bytes = reserve(&mut state.points, length, effect)?;
                } else if index < length {
                    state.points.push(BitmapStrokePoint { x: xs[index], y: ys[index] });
                } else {
                    *output = Some(Self::PaintInputStroke(PaintInputStroke { color: *color, points: take(&mut state.points) }));
                }
            }
            BitmapOperationDsl::PinSolution { xs, ys, colors } | BitmapOperationDsl::RestorePins { xs, ys, colors } => {
                let index = state.pins.len();
                if demand.capacity_bytes != 0 {
                    progress.retained_capacity_bytes = reserve(&mut state.pins, xs.len(), effect)?;
                } else if index < xs.len() {
                    state.pins.push(BitmapPinnedPixel { x: xs[index], y: ys[index], color: colors[index] });
                } else {
                    let pins = take(&mut state.pins);
                    *output = Some(if matches!(input, Some(BitmapOperationDsl::PinSolution { .. })) {
                        Self::PinSolution(PinSolution { pins })
                    } else {
                        Self::RestorePins(RestorePins { pins })
                    });
                }
            }
            _ => *output = Some(operation_from_dsl(input.take().expect("validated Bitmap conversion input"))?),
        }
        Ok(progress)
    }
}

dsl::original_converted_operation_receiving!(BitmapMutation);

