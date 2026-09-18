//! 🔺️ BitmapDiff — a sparse, lane-addressed structural delta over `BitmapSnapshot`. Each mutation
//! triad's own `🔺️diff/🦀️.rs` builds one of these directly from `(payload, base)`; `absorb` is
//! structural (lane-wise, id-keyed for `pinned`), never re-derived from an applied snapshot.
//!
//! Two lanes are deliberately COARSER than an id-keyed collection delta, and the reason is the
//! document, not laziness. `palette` is positional: `add-palette-color`/`remove-palette-color`
//! insert and delete AT an index and renumber every pixel above it, so a "palette entry 3 changed"
//! delta would not describe what actually happened. `inputPixels` is the whole re-indexed buffer
//! for exactly those two renumbering mutations. Ordinary painting never touches either: it rides
//! `inputRegions`, a real sparse list of rectangular writes.

use crate::schema::snapshot::{pin_key, resized_buffer, write_region, BitmapColor, BitmapOverlappingModel, BitmapOutputSpec, BitmapPinnedPixel, BitmapSnapshot};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Region
/// 🖌️ One rectangular write into the input index buffer, base64 palette indices, row-major within
/// the rectangle. A stroke coalesces into exactly one of these, never one per sampled pixel.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapPixelRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub pixels: String,
}
//#endregion 🔖️Region

//#region 🔖️BitmapDiff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.wfc.bitmap")]
pub struct BitmapDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub seed: Option<u64>,
    #[state(artifact)]
    pub input_width: Option<u32>,
    #[state(artifact)]
    pub input_height: Option<u32>,
    #[state(artifact)]
    pub input_pixels: Option<String>,
    #[state(artifact)]
    pub input_regions: Vec<BitmapPixelRegion>,
    #[state(artifact)]
    pub palette: Option<Vec<BitmapColor>>,
    #[state(artifact)]
    pub output: Option<BitmapOutputSpec>,
    #[state(artifact)]
    pub model: Option<BitmapOverlappingModel>,
    #[state(artifact)]
    pub pinned_removed: Vec<String>,
    #[state(artifact)]
    pub pinned_upserted: Vec<(usize, BitmapPinnedPixel)>,
}
//#endregion 🔖️BitmapDiff

//#region 🔖️IdKeyedMerge
/// 🔀 Id-keyed upsert/remove merge for the `pinned` lane: `self` is base→mid, `other` is mid→after
/// — a later remove wins over an earlier upsert of the SAME key, a later upsert clears an earlier
/// remove of it.
fn merge_pins(self_removed: &[String], self_upserted: &[(usize, BitmapPinnedPixel)], other_removed: &[String], other_upserted: &[(usize, BitmapPinnedPixel)]) -> (Vec<String>, Vec<(usize, BitmapPinnedPixel)>) {
    let mut removed: BTreeMap<String, ()> = self_removed.iter().cloned().map(|key| (key, ())).collect();
    let mut upserted: BTreeMap<String, (usize, BitmapPinnedPixel)> = self_upserted.iter().map(|(index, pin)| (pin_key(pin.x, pin.y), (*index, *pin))).collect();
    for key in other_removed {
        upserted.remove(key);
        removed.insert(key.clone(), ());
    }
    for (index, pin) in other_upserted {
        let key = pin_key(pin.x, pin.y);
        removed.remove(&key);
        upserted.insert(key, (*index, *pin));
    }
    (removed.into_keys().collect(), upserted.into_values().collect())
}
//#endregion 🔖️IdKeyedMerge

//#region 🔖️Apply
/// 🧬 Validates and applies one id-keyed indexed collection delta atomically — the same total
/// validation the assembly artifact's own diff performs, narrowed to this artifact's one such lane.
fn apply_pins(base: &[BitmapPinnedPixel], removed: &[String], upserted: &[(usize, BitmapPinnedPixel)]) -> protocol::MutationApplyResult<Vec<BitmapPinnedPixel>> {
    for (position, key) in removed.iter().enumerate() {
        if !base.iter().any(|pin| &pin_key(pin.x, pin.y) == key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed pin does not exist").at(["pinnedRemoved".to_string(), position.to_string()]));
        }
        if removed[..position].contains(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "pin is removed more than once").at(["pinnedRemoved".to_string(), position.to_string()]));
        }
    }
    for (position, (index, pin)) in upserted.iter().enumerate() {
        let key = pin_key(pin.x, pin.y);
        if removed.contains(&key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "pin cannot be removed and upserted").at(["pinnedUpserted".to_string(), position.to_string()]));
        }
        if upserted[..position].iter().any(|(_, prior)| pin_key(prior.x, prior.y) == key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "pin is upserted more than once").at(["pinnedUpserted".to_string(), position.to_string()]));
        }
        if let Some(existing) = base.iter().position(|candidate| pin_key(candidate.x, candidate.y) == key) {
            if *index != existing {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "replacement index does not match the existing pin").at(["pinnedUpserted".to_string(), position.to_string()]));
            }
        } else {
            let preceding = upserted[..position].iter().filter(|(_, prior)| !base.iter().any(|candidate| pin_key(candidate.x, candidate.y) == pin_key(prior.x, prior.y))).count();
            let available = base.len() - removed.len() + preceding;
            if *index > available {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("insertion index {index} exceeds length {available}")).at(["pinnedUpserted".to_string(), position.to_string()]));
            }
        }
    }
    let mut pins: Vec<BitmapPinnedPixel> = base.iter().copied().filter(|pin| !removed.contains(&pin_key(pin.x, pin.y))).collect();
    for (index, pin) in upserted {
        let key = pin_key(pin.x, pin.y);
        if let Some(existing) = pins.iter_mut().find(|candidate| pin_key(candidate.x, candidate.y) == key) {
            *existing = *pin;
        } else {
            pins.insert(*index, *pin);
        }
    }
    Ok(pins)
}

impl protocol::MutationDiff<BitmapSnapshot> for BitmapDiff {
    fn apply(&self, base: &BitmapSnapshot) -> protocol::MutationApplyResult<BitmapSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(seed) = self.seed {
            next.seed = seed;
        }
        let mut buffer = next.input.indices().ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.malformed-pixels", "the base input pixel buffer does not decode").at(["input", "pixels"]))?;
        if self.input_width.is_some() || self.input_height.is_some() {
            let width = self.input_width.unwrap_or(next.input.width);
            let height = self.input_height.unwrap_or(next.input.height);
            if width == 0 || height == 0 {
                return Err(protocol::MutationApplyError::new("mutation.apply.invariant", "an input bitmap may not have a zero edge").at(["input"]));
            }
            buffer = resized_buffer(&buffer, next.input.width, next.input.height, width, height);
            next.input.width = width;
            next.input.height = height;
        }
        if let Some(pixels) = &self.input_pixels {
            let decoded = crate::schema::snapshot::decode_base64(pixels).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.malformed-pixels", "the replacement input pixel buffer is not base64").at(["inputPixels"]))?;
            if decoded.len() != (next.input.width as usize) * (next.input.height as usize) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invariant", "the replacement input pixel buffer does not match the input size").at(["inputPixels"]));
            }
            buffer = decoded;
        }
        if let Some(palette) = &self.palette {
            if palette.is_empty() {
                return Err(protocol::MutationApplyError::new("mutation.apply.invariant", "a palette may not be empty").at(["palette"]));
            }
            next.input.palette = palette.clone();
        }
        for (position, region) in self.input_regions.iter().enumerate() {
            let pixels = crate::schema::snapshot::decode_base64(&region.pixels).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.malformed-pixels", "a region write is not base64").at(["inputRegions".to_string(), position.to_string()]))?;
            if !write_region(&mut buffer, next.input.width, next.input.height, region.x, region.y, region.width, region.height, &pixels) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invariant", "a region write falls outside the input bitmap").at(["inputRegions".to_string(), position.to_string()]));
            }
        }
        next.input.pixels = crate::schema::snapshot::encode_base64(&buffer);
        if let Some(output) = self.output {
            next.output = output;
        }
        if let Some(model) = self.model {
            next.model = model;
        }
        next.pinned = apply_pins(&next.pinned, &self.pinned_removed, &self.pinned_upserted).map_err(|error| error.under(["pinned"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.seed.is_some() {
            self.seed = other.seed;
        }
        if other.input_width.is_some() {
            self.input_width = other.input_width;
        }
        if other.input_height.is_some() {
            self.input_height = other.input_height;
        }
        if other.input_pixels.is_some() {
            self.input_pixels = other.input_pixels;
            self.input_regions = other.input_regions;
        } else {
            self.input_regions.extend(other.input_regions);
        }
        if other.palette.is_some() {
            self.palette = other.palette;
        }
        if other.output.is_some() {
            self.output = other.output;
        }
        if other.model.is_some() {
            self.model = other.model;
        }
        let (removed, upserted) = merge_pins(&self.pinned_removed, &self.pinned_upserted, &other.pinned_removed, &other.pinned_upserted);
        self.pinned_removed = removed;
        self.pinned_upserted = upserted;
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
