//! 🔺️ The existing typed keyed-row delta replaces pins in their authored sequence.
use crate::diff::{BitmapDiff, BitmapPinnedDelta, BitmapPinnedInsertion};
use crate::schema::snapshot::{pin_key, BitmapPinnedPixel, BitmapSnapshot};

pub fn replacement_diff(pins: &[BitmapPinnedPixel], base: &BitmapSnapshot, complete: bool) -> protocol::MutationOutcome<BitmapDiff> {
    let cells = base.output.width as usize * base.output.height as usize;
    if complete && pins.len() != cells {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A pinned solution must cover every output cell.", ["pinned".to_string()]);
    }
    let mut seen = std::collections::BTreeSet::new();
    for (index, pin) in pins.iter().enumerate() {
        if pin.x >= base.output.width || pin.y >= base.output.height || pin.color as usize >= base.input.palette.len() || !seen.insert((pin.x, pin.y)) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "A pin assignment must name unique valid output cells and palette colours.", [pin_key(pin.x, pin.y)]);
        }
        if complete && (pin.x as usize != index % base.output.width as usize || pin.y as usize != index / base.output.width as usize) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "A pinned solution must use row-major order.", ["pinned".to_string()]);
        }
    }
    if base.pinned == pins {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The output already carries this pin assignment.");
    }
    let indices: Vec<usize> = (0..base.pinned.len()).collect();
    let mut pinned = BitmapPinnedDelta::removals(&base.pinned, &indices);
    pinned.inserted = pins.iter().enumerate().map(|(index, pin)| BitmapPinnedInsertion { index: index.try_into().expect("bitmap pin count fits its index"), row: *pin }).collect();
    protocol::MutationOutcome::new(BitmapDiff { pinned, ..Default::default() })
}

pub fn diff(payload: &super::PinSolution, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    replacement_diff(&payload.pins, base, true)
}
