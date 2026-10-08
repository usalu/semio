//! 🔺️ BitmapDiff — a sparse structural delta over `BitmapSnapshot`: the input buffer and the palette carry ORDERED edit ops (a coordinate-preserving resize, rectangular writes,
//! single-pixel cells; palette insert/remove/recolour), pins carry id-keyed row deltas. `absorb` composes the op lists in order and cancels or merges adjacent pairs that compose
//! exactly; `inverse` replays the ops over the base to restore every overwritten value.

use crate::schema::snapshot::{pin_key, read_region, resized_buffer, BitmapColor, BitmapInput, BitmapOutputSpec, BitmapOverlappingModel, BitmapPinnedPixel, BitmapSnapshot};
use crate::standards::v1::subsets::any::schema::snapshot::write_region;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 🔑️ A row of an id-keyed list kept in canonical order: `row_key` addresses it, `insert_at` is where a new row lands.
pub trait BitmapRow: Clone + PartialEq {
    fn row_key(&self) -> String;
    fn insert_at(items: &[Self], row: &Self) -> usize;
}

/// 🩹 Field-sparse patch over one row: names the fields it sets with the values they take.
pub trait BitmapPatch: Clone + Default + PartialEq {
    type Row;
    fn patched(&self, row: &Self::Row) -> Self::Row;
    fn between(from: &Self::Row, to: &Self::Row) -> Self;
    fn inverse(&self, base: &Self::Row) -> Self;
    fn absorb(&mut self, later: Self);
    fn is_empty(&self) -> bool;
}

/// 🩹 One patched row, addressed by its identity.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapRowPatch<Q> {
    pub id: String,
    pub patch: Q,
}

/// 📂 Id-keyed row delta: removed identities, added rows (landing at their canonical position) and per-row field patches.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapRows<T, Q> {
    #[value(default)]
    pub removed: Vec<String>,
    #[value(default)]
    pub added: Vec<T>,
    #[value(default)]
    pub patched: Vec<BitmapRowPatch<Q>>,
}

impl<T, Q> Default for BitmapRows<T, Q> {
    fn default() -> Self {
        Self { removed: Vec::new(), added: Vec::new(), patched: Vec::new() }
    }
}

fn rejection(code: &'static str, message: &'static str, lane: &str, id: &str) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at([lane, id])
}

impl<T: BitmapRow, Q: BitmapPatch<Row = T>> BitmapRows<T, Q> {
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.added.is_empty() && self.patched.is_empty()
    }

    /// 🔑️ The list after the delta: removals first, then canonical-position insertions, then field patches; unknown targets and duplicated identities are rejected.
    pub fn apply(&self, items: &[T]) -> protocol::MutationApplyResult<Vec<T>> {
        let mut next = items.to_vec();
        for (position, id) in self.removed.iter().enumerate() {
            if self.removed[..position].contains(id) {
                return Err(rejection("mutation.apply.duplicate-target", "item is removed more than once", "removed", id));
            }
            let at = next.iter().position(|item| item.row_key() == *id).ok_or_else(|| rejection("mutation.apply.missing-target", "removed item does not exist", "removed", id))?;
            next.remove(at);
        }
        for (position, row) in self.added.iter().enumerate() {
            let key = row.row_key();
            if self.added[..position].iter().any(|prior| prior.row_key() == key) || next.iter().any(|item| item.row_key() == key) {
                return Err(rejection("mutation.apply.duplicate-target", "added item identity already exists", "added", &key));
            }
            let at = T::insert_at(&next, row);
            next.insert(at, row.clone());
        }
        for (position, entry) in self.patched.iter().enumerate() {
            if self.patched[..position].iter().any(|prior| prior.id == entry.id) {
                return Err(rejection("mutation.apply.duplicate-target", "item is patched more than once", "patched", &entry.id));
            }
            let at = next.iter().position(|item| item.row_key() == entry.id).ok_or_else(|| rejection("mutation.apply.missing-target", "patched item does not exist", "patched", &entry.id))?;
            let patched = entry.patch.patched(&next[at]);
            next[at] = patched;
        }
        Ok(next)
    }

    /// ➕️ Composes a later delta: create∘delete cancels, patch∘delete leaves the deletion, patch∘patch coalesces, delete∘create replaces.
    pub fn absorb(&mut self, later: Self) {
        for id in later.removed {
            if let Some(position) = self.added.iter().position(|row| row.row_key() == id) {
                self.added.remove(position);
            } else {
                self.patched.retain(|entry| entry.id != id);
                self.removed.push(id);
            }
        }
        self.added.extend(later.added);
        for entry in later.patched {
            if let Some(row) = self.added.iter_mut().find(|row| row.row_key() == entry.id) {
                *row = entry.patch.patched(row);
            } else if let Some(existing) = self.patched.iter_mut().find(|existing| existing.id == entry.id) {
                existing.patch.absorb(entry.patch);
            } else {
                self.patched.push(entry);
            }
        }
    }

    /// 🔁️ The delta that, applied after this one, restores `base` rows and positions exactly.
    pub fn inverse(&self, base: &[T]) -> Self {
        let find = |id: &str| base.iter().find(|row| row.row_key() == id);
        let removed: Vec<String> = self.added.iter().map(T::row_key).collect();
        let added: Vec<T> = self.removed.iter().filter_map(|id| find(id).cloned()).collect();
        let patched: Vec<BitmapRowPatch<Q>> = self
            .patched
            .iter()
            .filter(|entry| !self.removed.contains(&entry.id) && !self.added.iter().any(|row| row.row_key() == entry.id))
            .filter_map(|entry| find(&entry.id).map(|row| BitmapRowPatch { id: entry.id.clone(), patch: entry.patch.inverse(row) }))
            .filter(|entry| !entry.patch.is_empty())
            .collect();
        Self { removed, added, patched }
    }

    /// 🧭️ The delta that turns `from` into `to`.
    pub fn between(from: &[T], to: &[T]) -> Self {
        let removed: Vec<String> = from.iter().map(T::row_key).filter(|key| !to.iter().any(|row| row.row_key() == *key)).collect();
        let added: Vec<T> = to.iter().filter(|row| !from.iter().any(|other| other.row_key() == row.row_key())).cloned().collect();
        let patched: Vec<BitmapRowPatch<Q>> = from
            .iter()
            .filter_map(|row| to.iter().find(|other| other.row_key() == row.row_key()).filter(|other| *other != row).map(|other| BitmapRowPatch { id: row.row_key(), patch: Q::between(row, other) }))
            .collect();
        Self { removed, added, patched }
    }
}
//#endregion 🔖️Rows

//#region 🔖️Optionals
//#endregion 🔖️Optionals

//#region 🔖️PatchMacro
/// 🩹 Declares a field-sparse patch struct for `$row` and its [`BitmapPatch`] impl: `plain` fields set a value, `optional` fields set or clear an `Option` through their wrapper.
macro_rules! wfc_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { plain { $($field:ident : $ty:ty),* } optional { $($ofield:ident : $wrap:ident),* } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
        #[value(rename_all = "camelCase", default)]
        pub struct $name {
            $(pub $field: Option<$ty>,)*
            $(pub $ofield: Option<$wrap>,)*
        }

        impl BitmapPatch for $name {
            type Row = $row;
            fn patched(&self, row: &$row) -> $row {
                #[allow(unused_mut)]
                let mut next = row.clone();
                $(if let Some(value) = &self.$field { next.$field = value.clone(); })*
                $(if let Some(value) = &self.$ofield { next.$ofield = value.value.clone(); })*
                next
            }
            fn between(from: &$row, to: &$row) -> Self {
                Self { $($field: (from.$field != to.$field).then(|| to.$field.clone()),)* $($ofield: (from.$ofield != to.$ofield).then(|| $wrap { value: to.$ofield.clone() }),)* }
            }
            fn inverse(&self, base: &$row) -> Self {
                Self { $($field: self.$field.as_ref().map(|_| base.$field.clone()),)* $($ofield: self.$ofield.as_ref().map(|_| $wrap { value: base.$ofield.clone() }),)* }
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
    fn row_key(&self) -> String {
        pin_key(self.x, self.y)
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| (item.y, item.x) > (row.y, row.x)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📌️ Field patch over a pinned pixel (its `x:y` key is the row identity).
    BitmapPinnedPatch for BitmapPinnedPixel { plain { color: u32 } optional {  } });

/// 📂 Row delta over the pinned pixels.
pub type BitmapPinnedDelta = BitmapRows<BitmapPinnedPixel, BitmapPinnedPatch>;
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
/// 🖼️ One ordered edit of the input index buffer: a coordinate-preserving resize, a rectangular write, or a sparse list of single pixels.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "op", rename_all = "camelCase")]
pub enum BitmapInputOp {
    Resize { width: u32, height: u32 },
    Region { region: BitmapPixelRegion },
    Cells { cells: Vec<BitmapPixelCell> },
}

/// 🎨️ One ordered edit of the palette: insert at an index, remove an index, or recolour an index. Pixels that point at shifted entries are renumbered by explicit [`BitmapInputOp::Cells`] and pin patches, never implicitly.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "op", rename_all = "camelCase")]
pub enum BitmapPaletteOp {
    Insert { index: u32, color: BitmapColor },
    Remove { index: u32 },
    Recolor { index: u32, color: BitmapColor },
}

/// ➕️ Appends an input op, merging it into an adjacent op of the same kind when the pair composes exactly.
fn push_input_op(ops: &mut Vec<BitmapInputOp>, op: BitmapInputOp) {
    match (ops.last_mut(), op) {
        (Some(BitmapInputOp::Cells { cells: earlier }), BitmapInputOp::Cells { cells }) => {
            for cell in cells {
                match earlier.iter_mut().find(|existing| (existing.x, existing.y) == (cell.x, cell.y)) {
                    Some(existing) => existing.value = cell.value,
                    None => earlier.push(cell),
                }
            }
            earlier.sort_by_key(|cell| (cell.y, cell.x));
        }
        (Some(BitmapInputOp::Region { region: earlier }), BitmapInputOp::Region { region }) if (earlier.x, earlier.y, earlier.width, earlier.height) == (region.x, region.y, region.width, region.height) => *earlier = region,
        (_, op) => ops.push(op),
    }
}

/// ➕️ Appends a palette op, cancelling or merging it with an adjacent op on the same index when the pair composes exactly.
fn push_palette_op(ops: &mut Vec<BitmapPaletteOp>, op: BitmapPaletteOp) {
    match (ops.last().cloned(), op) {
        (Some(BitmapPaletteOp::Insert { index: earlier, .. }), BitmapPaletteOp::Remove { index }) if earlier == index => {
            ops.pop();
        }
        (Some(BitmapPaletteOp::Insert { index: earlier, .. }), BitmapPaletteOp::Recolor { index, color }) if earlier == index => *ops.last_mut().expect("last op exists") = BitmapPaletteOp::Insert { index, color },
        (Some(BitmapPaletteOp::Recolor { index: earlier, .. }), BitmapPaletteOp::Recolor { index, color }) if earlier == index => *ops.last_mut().expect("last op exists") = BitmapPaletteOp::Recolor { index, color },
        (Some(BitmapPaletteOp::Recolor { index: earlier, .. }), BitmapPaletteOp::Remove { index }) if earlier == index => *ops.last_mut().expect("last op exists") = BitmapPaletteOp::Remove { index },
        (_, op) => ops.push(op),
    }
}

/// 🔁️ The strips of the `from` buffer that a resize to `to` crops away: the right strip over the full old height, then the bottom strip over the surviving width.
fn cropped_strips(buffer: &[u8], from_width: u32, from_height: u32, to_width: u32, to_height: u32) -> Vec<BitmapInputOp> {
    let mut strips = Vec::new();
    if to_width < from_width {
        if let Some(pixels) = read_region(buffer, from_width, from_height, to_width, 0, from_width - to_width, from_height) {
            strips.push(BitmapInputOp::Region { region: BitmapPixelRegion { x: to_width, y: 0, width: from_width - to_width, height: from_height, pixels } });
        }
    }
    if to_height < from_height {
        let width = to_width.min(from_width);
        if let Some(pixels) = read_region(buffer, from_width, from_height, 0, to_height, width, from_height - to_height) {
            strips.push(BitmapInputOp::Region { region: BitmapPixelRegion { x: 0, y: to_height, width, height: from_height - to_height, pixels } });
        }
    }
    strips
}

/// 🔁️ The ordered input ops that undo `ops` applied to `base`: each op's restoring ops, in reverse op order.
fn inverse_input_ops(ops: &[BitmapInputOp], base: &BitmapInput) -> Vec<BitmapInputOp> {
    let (mut width, mut height, mut buffer) = (base.width, base.height, base.pixels.clone());
    let mut undo: Vec<Vec<BitmapInputOp>> = Vec::new();
    for op in ops {
        match op {
            BitmapInputOp::Resize { width: to_width, height: to_height } => {
                let mut restore = vec![BitmapInputOp::Resize { width, height }];
                restore.extend(cropped_strips(&buffer, width, height, *to_width, *to_height));
                undo.push(restore);
                buffer = resized_buffer(&buffer, width, height, *to_width, *to_height);
                (width, height) = (*to_width, *to_height);
            }
            BitmapInputOp::Region { region } => {
                if let Some(pixels) = read_region(&buffer, width, height, region.x, region.y, region.width, region.height) {
                    undo.push(vec![BitmapInputOp::Region { region: BitmapPixelRegion { x: region.x, y: region.y, width: region.width, height: region.height, pixels } }]);
                }
                write_region(&mut buffer, width, height, region.x, region.y, region.width, region.height, &region.pixels);
            }
            BitmapInputOp::Cells { cells } => {
                let prior: Vec<BitmapPixelCell> = cells.iter().filter(|cell| cell.x < width && cell.y < height).map(|cell| BitmapPixelCell { x: cell.x, y: cell.y, value: u32::from(buffer[(cell.y * width + cell.x) as usize]) }).collect();
                for cell in cells.iter().filter(|cell| cell.x < width && cell.y < height) {
                    buffer[(cell.y * width + cell.x) as usize] = cell.value as u8;
                }
                undo.push(vec![BitmapInputOp::Cells { cells: prior }]);
            }
        }
    }
    let mut inverse = Vec::new();
    for op in undo.into_iter().rev().flatten() {
        push_input_op(&mut inverse, op);
    }
    inverse
}

/// 🔁️ The ordered palette ops that undo `ops` applied to `base`.
fn inverse_palette_ops(ops: &[BitmapPaletteOp], base: &[BitmapColor]) -> Vec<BitmapPaletteOp> {
    let mut palette = base.to_vec();
    let mut undo = Vec::new();
    for op in ops {
        match op {
            BitmapPaletteOp::Insert { index, color } => {
                undo.push(BitmapPaletteOp::Remove { index: *index });
                palette.insert((*index as usize).min(palette.len()), *color);
            }
            BitmapPaletteOp::Remove { index } => {
                if (*index as usize) < palette.len() {
                    undo.push(BitmapPaletteOp::Insert { index: *index, color: palette.remove(*index as usize) });
                }
            }
            BitmapPaletteOp::Recolor { index, color } => {
                if let Some(slot) = palette.get_mut(*index as usize) {
                    undo.push(BitmapPaletteOp::Recolor { index: *index, color: *slot });
                    *slot = *color;
                }
            }
        }
    }
    let mut inverse = Vec::new();
    for op in undo.into_iter().rev() {
        push_palette_op(&mut inverse, op);
    }
    inverse
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
    pub input_ops: Vec<BitmapInputOp>,
    #[state(artifact)]
    pub palette_ops: Vec<BitmapPaletteOp>,
    #[state(artifact)]
    pub output: Option<BitmapOutputSpec>,
    #[state(artifact)]
    pub model: Option<BitmapOverlappingModel>,
    #[state(artifact)]
    pub pinned: BitmapPinnedDelta,
}
//#endregion 🔖️Diff

//#region 🔖️Apply
fn invariant(message: &'static str, lane: &str, position: usize) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.invariant", message).at([lane.to_string(), position.to_string()])
}

impl protocol::MutationDiff<BitmapSnapshot> for BitmapDiff {
    fn apply(&self, base: &BitmapSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<BitmapSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        if let Some(seed) = self.seed {
            next.seed = seed;
        }
        let (mut width, mut height) = (base.input.width, base.input.height);
        let mut buffer = base.input.indices().ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.malformed-pixels", "the base input pixel buffer does not decode").at(["input", "pixels"]))?;
        for (position, op) in self.input_ops.iter().enumerate() {
            match op {
                BitmapInputOp::Resize { width: to_width, height: to_height } => {
                    if *to_width == 0 || *to_height == 0 {
                        return Err(invariant("an input bitmap may not have a zero edge", "inputOps", position));
                    }
                    buffer = resized_buffer(&buffer, width, height, *to_width, *to_height);
                    (width, height) = (*to_width, *to_height);
                }
                BitmapInputOp::Region { region } => {
                    if !write_region(&mut buffer, width, height, region.x, region.y, region.width, region.height, &region.pixels) {
                        return Err(invariant("a region write falls outside the input bitmap", "inputOps", position));
                    }
                }
                BitmapInputOp::Cells { cells } => {
                    for cell in cells {
                        if cell.x >= width || cell.y >= height || cell.value > 255 {
                            return Err(invariant("a pixel write falls outside the input bitmap", "inputOps", position));
                        }
                        buffer[(cell.y * width + cell.x) as usize] = cell.value as u8;
                    }
                }
            }
        }
        let mut palette = base.input.palette.clone();
        for (position, op) in self.palette_ops.iter().enumerate() {
            match op {
                BitmapPaletteOp::Insert { index, color } if (*index as usize) <= palette.len() => palette.insert(*index as usize, *color),
                BitmapPaletteOp::Remove { index } if (*index as usize) < palette.len() => {
                    palette.remove(*index as usize);
                }
                BitmapPaletteOp::Recolor { index, color } if (*index as usize) < palette.len() => palette[*index as usize] = *color,
                _ => return Err(invariant("a palette edit names an index outside the palette", "paletteOps", position)),
            }
        }
        if palette.is_empty() {
            return Err(protocol::MutationApplyError::new("mutation.apply.invariant", "a palette may not be empty").at(["paletteOps"]));
        }
        next.input = BitmapInput { width, height, palette, pixels: buffer };
        if let Some(output) = self.output {
            next.output = output;
        }
        if let Some(model) = self.model {
            next.model = model;
        }
        next.pinned = self.pinned.apply(&base.pinned).map_err(|error| error.under(["pinned"]))?;
        Ok(next)
    }

    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        if later.seed.is_some() {
            self.seed = later.seed;
        }
        for op in later.input_ops {
            push_input_op(&mut self.input_ops, op);
        }
        for op in later.palette_ops {
            push_palette_op(&mut self.palette_ops, op);
        }
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
            input_ops: inverse_input_ops(&self.input_ops, &base.input),
            palette_ops: inverse_palette_ops(&self.palette_ops, &base.input.palette),
            output: self.output.map(|_| base.output),
            model: self.model.map(|_| base.model),
            pinned: self.pinned.inverse(&base.pinned),
        }
    }

    fn between(base: &BitmapSnapshot, other: &BitmapSnapshot) -> Self {
        let mut input_ops = Vec::new();
        let (mut width, mut height) = (base.input.width, base.input.height);
        let mut buffer = base.input.pixels.clone();
        if (other.input.width, other.input.height) != (width, height) {
            input_ops.push(BitmapInputOp::Resize { width: other.input.width, height: other.input.height });
            buffer = resized_buffer(&buffer, width, height, other.input.width, other.input.height);
            (width, height) = (other.input.width, other.input.height);
        }
        let cells: Vec<BitmapPixelCell> = other
            .input
            .pixels
            .iter()
            .enumerate()
            .filter(|(at, value)| buffer.get(*at) != Some(*value))
            .map(|(at, value)| BitmapPixelCell { x: (at as u32) % width.max(1), y: (at as u32) / width.max(1), value: u32::from(*value) })
            .collect();
        if !cells.is_empty() {
            input_ops.push(BitmapInputOp::Cells { cells });
        }
        let (from, to) = (&base.input.palette, &other.input.palette);
        let mut palette_ops: Vec<BitmapPaletteOp> = (0..from.len().min(to.len())).filter(|index| from[*index] != to[*index]).map(|index| BitmapPaletteOp::Recolor { index: index as u32, color: to[index] }).collect();
        palette_ops.extend((to.len()..from.len()).rev().map(|index| BitmapPaletteOp::Remove { index: index as u32 }));
        palette_ops.extend((from.len()..to.len()).map(|index| BitmapPaletteOp::Insert { index: index as u32, color: to[index] }));
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            seed: (base.seed != other.seed).then_some(other.seed),
            input_ops,
            palette_ops,
            output: (base.output != other.output).then_some(other.output),
            model: (base.model != other.model).then_some(other.model),
            pinned: BitmapRows::between(&base.pinned, &other.pinned),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.seed.is_none() && self.input_ops.is_empty() && self.palette_ops.is_empty() && self.output.is_none() && self.model.is_none() && self.pinned.is_empty()
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
