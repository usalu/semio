//! 🔺️ BitmapDiff — a sparse structural delta over `BitmapSnapshot`: the input buffer and the palette carry ORDERED edit ops (a coordinate-preserving resize, rectangular writes,
//! single-pixel cells; palette insert/remove/recolour), pins carry id-keyed row deltas. `absorb` composes the op lists in order and cancels or merges adjacent pairs that compose
//! exactly; `inverse` replays the ops over the base to restore every overwritten value.

use crate::schema::snapshot::{pin_key, read_region, resized_buffer, BitmapColor, BitmapInput, BitmapOutputSpec, BitmapOverlappingModel, BitmapPinnedPixel, BitmapSnapshot};
use crate::standards::v1::subsets::any::schema::snapshot::write_region;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 📍️ The canonical position a new row of a list lands at in the after list.
pub trait BitmapRow: Clone + PartialEq {
    fn insert_at(items: &[Self], row: &Self) -> usize;
}
//#endregion 🔖️Rows

//#region 🔖️Optionals
//#endregion 🔖️Optionals

//#region 🔖️PatchMacro
/// 🩹 Declares a field-sparse patch struct for `$row` and its `protocol::list_delta::RowPatch` impl: `plain` fields set a value, `optional` fields set or clear an `Option` through their wrapper.
macro_rules! wfc_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { plain { $($field:ident : $ty:ty),* } optional { $($ofield:ident : $wrap:ident),* } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
        #[value(rename_all = "camelCase", default)]
        pub struct $name {
            $(pub $field: Option<$ty>,)*
            $(pub $ofield: Option<$wrap>,)*
        }

        impl protocol::list_delta::RowPatch<$row> for $name {
            fn commit_into(&self, row: &mut $row, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$field { row.$field = value.clone(); })*
                $(if let Some(value) = &self.$ofield { row.$ofield = value.value.clone(); })*
                Ok(())
            }
            fn inverse(&self, row: &$row) -> Self {
                Self { $($field: self.$field.as_ref().map(|_| row.$field.clone()),)* $($ofield: self.$ofield.as_ref().map(|_| $wrap { value: row.$ofield.clone() }),)* }
            }
            fn absorb(&mut self, later: Self) {
                $(if later.$field.is_some() { self.$field = later.$field; })*
                $(if later.$ofield.is_some() { self.$ofield = later.$ofield; })*
            }
            fn is_empty(&self) -> bool {
                true $(&& self.$field.is_none())* $(&& self.$ofield.is_none())*
            }
        }
    };
}
//#endregion 🔖️PatchMacro

//#region 🔖️RowTypes
impl BitmapRow for BitmapPinnedPixel {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| (item.y, item.x) > (row.y, row.x)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📌️ Field patch over a pinned pixel (its `x:y` key is the row identity).
    BitmapPinnedPatch for BitmapPinnedPixel { plain { color: u32 } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the pinned pixels.
    pub BitmapPinnedDelta { removal: BitmapPinnedRemoval, insertion: BitmapPinnedInsertion, relocation: BitmapPinnedRelocation, modification: BitmapPinnedModification, row: BitmapPinnedPixel, patch: BitmapPinnedPatch, list: Vec<BitmapPinnedPixel>, key: String = |row| pin_key(row.x, row.y), values_only }
}
//#endregion 🔖️RowTypes

//#region 🔖️Region
/// 🖌️ One rectangular write into the input index buffer, palette indices, row-major within the rectangle. A stroke
/// coalesces into exactly one of these, never one per sampled pixel.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapPixelRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    #[value(with = "semio_framework_value::bytes")]
    pub pixels: Vec<u8>,
}

/// 🔲 One input pixel and the palette index it takes.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapPixelCell {
    pub x: u32,
    pub y: u32,
    pub value: u32,
}
//#endregion 🔖️Region

//#region 🔖️Ops
/// 📐️ A bitmap size in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapSize {
    pub width: u32,
    pub height: u32,
}

/// 🖼️ One write into the input index buffer, in the coordinates of the buffer AFTER the patch's resize: a rectangular write or a sparse list of single pixels. Writes layer in list order.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "op", rename_all = "camelCase")]
pub enum BitmapInputWrite {
    Region { region: BitmapPixelRegion },
    Cells { cells: Vec<BitmapPixelCell> },
}

/// 🖼️ The input patch: an optional coordinate-preserving resize of the base buffer (crop, or pad with index 0), then the writes in their after-resize coordinates.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct BitmapInputPatch {
    pub size: Option<BitmapSize>,
    pub writes: Vec<BitmapInputWrite>,
}

/// ➕️ One palette colour that enters at `index` of the AFTER palette.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapPaletteInsertion {
    pub index: u32,
    pub color: BitmapColor,
}

/// 🎨️ One surviving palette colour recoloured; `index` is its position in the BASE palette.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapPaletteRecolor {
    pub index: u32,
    pub color: BitmapColor,
}

/// 🎨️ The positional palette delta: `removed` are BASE indices, `inserted` carry AFTER indices, `recolored` are BASE indices of surviving colours. Pixels that point at shifted entries are renumbered by explicit input cells and pin patches, never implicitly.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct BitmapPaletteDelta {
    pub removed: Vec<u32>,
    pub inserted: Vec<BitmapPaletteInsertion>,
    pub recolored: Vec<BitmapPaletteRecolor>,
}

/// 🔢️ The position of `index` once every coordinate in `excluded` is skipped.
fn rank(excluded: &[usize], index: usize) -> usize {
    index - excluded.iter().filter(|skipped| **skipped < index).count()
}

/// 🔢️ The `nth` (0-based) coordinate that is not in `excluded`.
fn nth_free(excluded: &[usize], nth: usize) -> usize {
    (0..).filter(|candidate| !excluded.contains(candidate)).nth(nth).unwrap_or(nth)
}

fn coordinates(indices: impl Iterator<Item = u32>) -> Vec<usize> {
    indices.map(|index| index as usize).collect()
}

fn invariant(message: &'static str, lane: &str, position: usize) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.invariant", message).at([lane.to_string(), position.to_string()])
}

impl BitmapPaletteDelta {
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.recolored.is_empty()
    }

    /// ➕️ The delta that inserts `color` at `index` of the after palette.
    pub fn insertion(index: u32, color: BitmapColor) -> Self {
        Self { inserted: vec![BitmapPaletteInsertion { index, color }], ..Self::default() }
    }

    /// ➖️ The delta that removes the colour at `index` of the base palette.
    pub fn removal(index: u32) -> Self {
        Self { removed: vec![index], ..Self::default() }
    }

    /// 🎨️ The delta that recolours the colour at `index` of the base palette.
    pub fn recoloring(index: u32, color: BitmapColor) -> Self {
        Self { recolored: vec![BitmapPaletteRecolor { index, color }], ..Self::default() }
    }

    /// 🔑️ The palette after the delta: removed base indices leave, inserted colours take their after slots, surviving colours fill the rest in base order (recoloured where named).
    pub fn colors_after(&self, base: &[BitmapColor]) -> protocol::MutationApplyResult<Vec<BitmapColor>> {
        for (position, index) in self.removed.iter().enumerate() {
            if *index as usize >= base.len() || self.removed[..position].contains(index) {
                return Err(invariant("a palette removal names an index outside the palette or twice", "palette.removed", position));
            }
        }
        for (position, recolor) in self.recolored.iter().enumerate() {
            if recolor.index as usize >= base.len() || self.removed.contains(&recolor.index) || self.recolored[..position].iter().any(|prior| prior.index == recolor.index) {
                return Err(invariant("a palette recolour names an index outside the palette, a removed colour or twice", "palette.recolored", position));
            }
        }
        let after_len = (base.len() + self.inserted.len()).checked_sub(self.removed.len()).ok_or_else(|| invariant("more palette colours leave than the palette holds", "palette.removed", 0))?;
        for (position, entry) in self.inserted.iter().enumerate() {
            if entry.index as usize >= after_len || self.inserted[..position].iter().any(|prior| prior.index == entry.index) {
                return Err(invariant("a palette insertion lies past the end of the after palette or twice", "palette.inserted", position));
            }
        }
        let mut survivors = base.iter().enumerate().filter(|(index, _)| !self.removed.contains(&(*index as u32))).map(|(index, color)| self.recolored.iter().find(|recolor| recolor.index as usize == index).map_or(*color, |recolor| recolor.color));
        (0..after_len)
            .map(|slot| match self.inserted.iter().find(|entry| entry.index as usize == slot) {
                Some(entry) => Ok(entry.color),
                None => survivors.next().ok_or_else(|| invariant("the after palette has more free slots than surviving colours", "palette.inserted", slot)),
            })
            .collect()
    }

    /// 🔁️ The negative delta over `base`, read colour by colour; nothing is simulated.
    pub fn inverse(&self, base: &[BitmapColor]) -> Self {
        let inserted_at = coordinates(self.inserted.iter().map(|entry| entry.index));
        let removed_at = coordinates(self.removed.iter().copied());
        Self {
            removed: self.inserted.iter().map(|entry| entry.index).collect(),
            inserted: self.removed.iter().filter_map(|index| base.get(*index as usize).map(|color| BitmapPaletteInsertion { index: *index, color: *color })).collect(),
            recolored: self.recolored.iter().filter_map(|recolor| base.get(recolor.index as usize).map(|color| BitmapPaletteRecolor { index: nth_free(&inserted_at, rank(&removed_at, recolor.index as usize)) as u32, color: *color })).collect(),
        }
    }

    /// ➕️ Composes `self` (base→mid) with `later` (mid→after) into base→after by pure index arithmetic.
    pub fn composed(&self, later: &Self) -> Self {
        let mid_inserted = coordinates(self.inserted.iter().map(|entry| entry.index));
        let left_base = coordinates(self.removed.iter().copied());
        let left_mid = coordinates(later.removed.iter().copied());
        let after_inserted = coordinates(later.inserted.iter().map(|entry| entry.index));
        let after_of_mid = |mid: u32| nth_free(&after_inserted, rank(&left_mid, mid as usize)) as u32;
        let base_of_mid = |mid: u32| nth_free(&left_base, rank(&mid_inserted, mid as usize)) as u32;
        let first_inserted = |mid: u32| self.inserted.iter().any(|entry| entry.index == mid);
        let removed_base: Vec<u32> = later.removed.iter().filter(|mid| !first_inserted(**mid)).map(|mid| base_of_mid(*mid)).collect();
        let recolored_base: Vec<BitmapPaletteRecolor> = later.recolored.iter().filter(|recolor| !first_inserted(recolor.index)).map(|recolor| BitmapPaletteRecolor { index: base_of_mid(recolor.index), color: recolor.color }).collect();
        let inserted = self
            .inserted
            .iter()
            .filter(|entry| !later.removed.contains(&entry.index))
            .map(|entry| BitmapPaletteInsertion { index: after_of_mid(entry.index), color: later.recolored.iter().find(|recolor| recolor.index == entry.index).map_or(entry.color, |recolor| recolor.color) })
            .chain(later.inserted.iter().cloned())
            .collect();
        let recolored = self
            .recolored
            .iter()
            .filter(|recolor| !removed_base.contains(&recolor.index) && !recolored_base.iter().any(|later_recolor| later_recolor.index == recolor.index))
            .cloned()
            .chain(recolored_base.iter().cloned())
            .collect();
        Self { removed: self.removed.iter().copied().chain(removed_base).collect(), inserted, recolored }
    }
}

/// 🔁️ The strips of the `from` buffer that a resize to `to` crops away: the right strip over the full old height, then the bottom strip over the surviving width.
fn cropped_strips(buffer: &[u8], from_width: u32, from_height: u32, to_width: u32, to_height: u32) -> Vec<BitmapInputWrite> {
    let right = (to_width < from_width).then(|| read_region(buffer, from_width, from_height, to_width, 0, from_width - to_width, from_height).map(|pixels| BitmapInputWrite::Region { region: BitmapPixelRegion { x: to_width, y: 0, width: from_width - to_width, height: from_height, pixels } })).flatten();
    let width = to_width.min(from_width);
    let bottom = (to_height < from_height).then(|| read_region(buffer, from_width, from_height, 0, to_height, width, from_height - to_height).map(|pixels| BitmapInputWrite::Region { region: BitmapPixelRegion { x: 0, y: to_height, width, height: from_height - to_height, pixels } })).flatten();
    right.into_iter().chain(bottom).collect()
}

/// 🧱️ The zero fill of the area `to` has beyond `from`: the right strip over the full new height, then the bottom strip over the shared width.
fn zero_fill(from: BitmapSize, to: BitmapSize) -> Vec<BitmapInputWrite> {
    let zeros = |x: u32, y: u32, width: u32, height: u32| BitmapInputWrite::Region { region: BitmapPixelRegion { x, y, width, height, pixels: vec![0; (width as usize) * (height as usize)] } };
    let right = (to.width > from.width).then(|| zeros(from.width, 0, to.width - from.width, to.height));
    let bottom = (to.height > from.height && from.width.min(to.width) > 0).then(|| zeros(0, from.height, from.width.min(to.width), to.height - from.height));
    right.into_iter().chain(bottom).collect()
}

/// ✂️ One write clipped to a `width`×`height` buffer; `None` when nothing of it remains.
fn clipped_write(write: &BitmapInputWrite, width: u32, height: u32) -> Option<BitmapInputWrite> {
    match write {
        BitmapInputWrite::Region { region } => {
            let (clip_width, clip_height) = (region.width.min(width.saturating_sub(region.x)), region.height.min(height.saturating_sub(region.y)));
            (clip_width > 0 && clip_height > 0).then(|| {
                let pixels = (0..clip_height).flat_map(|row| region.pixels[(row * region.width) as usize..(row * region.width + clip_width) as usize].iter().copied()).collect();
                BitmapInputWrite::Region { region: BitmapPixelRegion { x: region.x, y: region.y, width: clip_width, height: clip_height, pixels } }
            })
        }
        BitmapInputWrite::Cells { cells } => {
            let kept: Vec<BitmapPixelCell> = cells.iter().filter(|cell| cell.x < width && cell.y < height).cloned().collect();
            (!kept.is_empty()).then_some(BitmapInputWrite::Cells { cells: kept })
        }
    }
}

/// ➕️ Composes `later` onto `first` (base→mid→after): the later resize wins, earlier writes are clipped to it, a regrown area is zero-filled, and the later writes layer on top.
fn composed_input(first: &BitmapInputPatch, later: BitmapInputPatch) -> BitmapInputPatch {
    let kept: Vec<BitmapInputWrite> = match later.size {
        Some(size) => first.writes.iter().filter_map(|write| clipped_write(write, size.width, size.height)).collect(),
        None => first.writes.clone(),
    };
    let fill = match (first.size, later.size) {
        (Some(from), Some(to)) => zero_fill(from, to),
        _ => Vec::new(),
    };
    BitmapInputPatch { size: later.size.or(first.size), writes: fill.into_iter().chain(kept).chain(later.writes).collect() }
}

/// 🔁️ The input patch that undoes `patch` over `base`: the cropped strips of a shrinking resize plus the base values under every write, each read straight from the base buffer.
fn inverse_input_patch(patch: &BitmapInputPatch, base: &BitmapInput) -> BitmapInputPatch {
    let Some(buffer) = base.indices() else { return BitmapInputPatch::default() };
    let (width, height) = (base.width, base.height);
    let strips = patch.size.map(|size| cropped_strips(&buffer, width, height, size.width, size.height)).unwrap_or_default();
    let restored = patch.writes.iter().filter_map(|write| clipped_write(write, width, height)).filter_map(|write| match write {
        BitmapInputWrite::Region { region } => read_region(&buffer, width, height, region.x, region.y, region.width, region.height).map(|pixels| BitmapInputWrite::Region { region: BitmapPixelRegion { pixels, ..region } }),
        BitmapInputWrite::Cells { cells } => Some(BitmapInputWrite::Cells { cells: cells.into_iter().map(|cell| BitmapPixelCell { value: u32::from(buffer[(cell.y * width + cell.x) as usize]), ..cell }).collect() }),
    });
    BitmapInputPatch { size: patch.size.map(|_| BitmapSize { width, height }), writes: strips.into_iter().chain(restored).collect() }
}
//#endregion 🔖️Ops

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.wfc.bitmap")]
pub struct BitmapDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub seed: Option<u64>,
    #[state(artifact)]
    pub input: BitmapInputPatch,
    #[state(artifact)]
    pub palette: BitmapPaletteDelta,
    #[state(artifact)]
    pub output: Option<BitmapOutputSpec>,
    #[state(artifact)]
    pub model: Option<BitmapOverlappingModel>,
    #[state(artifact)]
    pub pinned: BitmapPinnedDelta,
}
//#endregion 🔖️Diff

//#region 🔖️Apply
impl protocol::MutationDiff<BitmapSnapshot> for BitmapDiff {
    fn apply(&self, base: &BitmapSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<BitmapSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        if let Some(seed) = self.seed {
            next.seed = seed;
        }
        let base_buffer = base.input.indices().ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.malformed-pixels", "the base input pixel buffer does not decode").at(["input", "pixels"]))?;
        let (width, height) = self.input.size.map_or((base.input.width, base.input.height), |size| (size.width, size.height));
        if width == 0 || height == 0 {
            return Err(invariant("an input bitmap may not have a zero edge", "input.size", 0));
        }
        let mut buffer = resized_buffer(&base_buffer, base.input.width, base.input.height, width, height);
        for (position, write) in self.input.writes.iter().enumerate() {
            match write {
                BitmapInputWrite::Region { region } => {
                    if !write_region(&mut buffer, width, height, region.x, region.y, region.width, region.height, &region.pixels) {
                        return Err(invariant("a region write falls outside the input bitmap", "input.writes", position));
                    }
                }
                BitmapInputWrite::Cells { cells } => {
                    for cell in cells {
                        if cell.x >= width || cell.y >= height || cell.value > 255 {
                            return Err(invariant("a pixel write falls outside the input bitmap", "input.writes", position));
                        }
                        buffer[(cell.y * width + cell.x) as usize] = cell.value as u8;
                    }
                }
            }
        }
        let palette = self.palette.colors_after(&base.input.palette)?;
        if palette.is_empty() {
            return Err(protocol::MutationApplyError::new("mutation.apply.invariant", "a palette may not be empty").at(["palette"]));
        }
        next.input = BitmapInput { width, height, palette, pixels: buffer };
        if let Some(output) = self.output {
            next.output = output;
        }
        if let Some(model) = self.model {
            next.model = model;
        }
        next.pinned = self.pinned.commit_onto(&base.pinned, capability).map_err(|error| error.under(["pinned"]))?;
        Ok(next)
    }

    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        if later.seed.is_some() {
            self.seed = later.seed;
        }
        self.input = composed_input(&self.input, later.input);
        self.palette = self.palette.composed(&later.palette);
        if later.output.is_some() {
            self.output = later.output;
        }
        if later.model.is_some() {
            self.model = later.model;
        }
        self.pinned.absorb(later.pinned);
    }
}

impl protocol::DiffAlgebra<BitmapSnapshot> for BitmapDiff {
    fn inverse(&self, base: &BitmapSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            seed: self.seed.map(|_| base.seed),
            input: inverse_input_patch(&self.input, &base.input),
            palette: self.palette.inverse(&base.input.palette),
            output: self.output.map(|_| base.output),
            model: self.model.map(|_| base.model),
            pinned: self.pinned.inverse(&base.pinned),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.seed.is_none() && self.input.size.is_none() && self.input.writes.is_empty() && self.palette.is_empty() && self.output.is_none() && self.model.is_none() && self.pinned.is_empty()
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
