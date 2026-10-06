//! 🖼️ BitmapSnapshot — the classic overlapping-model wave function collapse PROBLEM: an INPUT
//! bitmap (palette + owned row-major palette indices), an OUTPUT specification (size and
//! periodicity), the OVERLAPPING MODEL parameters (pattern size `N`, D4 symmetry expansion,
//! periodic input, optional ground colour), and per-pixel PINS the solve must respect. The solved
//! output bitmap, the contradiction verdict and the entropy map are never stored here: they are an
//! INFERENCE (`../💡️inferences/🦀️.rs`) over this spec, exactly as the assembly artifact's own
//! solve is — only the PROBLEM is authored, the SOLUTION is derived.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Ids
pub const WFC_BITMAP_DOCUMENT_SCHEMA: &str = "s.wfc.bitmap";

/// 🎨️ The largest palette a document may carry — one byte per pixel is the whole point of the
/// owned index buffer, so index 255 is the ceiling and 256 entries the count.
pub const BITMAP_MAX_PALETTE: usize = 256;

/// 🖼️ The largest input/output edge length. `512 × 512` is one quarter of a mebibyte of indices,
/// which is what a single `set-input-pixels` region write may carry before the guest's contiguous
/// allocation ceiling (64 KiB, project memory `guest-contiguous-request-ceiling`) is the binding
/// constraint rather than this one.
pub const BITMAP_MAX_EDGE: u32 = 512;
//#endregion 🔖️Ids



//#region 🔖️Color
/// 🎨️ One straight-alpha sRGB palette entry, 0–255 per channel — integers, not floats, so a
/// committed fixture never depends on a float print convention.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl BitmapColor {
    pub const fn opaque(r: u32, g: u32, b: u32) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// 🎨️ The `[r, g, b, a]` 0..1 float quadruple the canvas host's own `fill.color` record takes.
    pub fn to_unit_rgba(self) -> [f64; 4] {
        [f64::from(self.r) / 255.0, f64::from(self.g) / 255.0, f64::from(self.b) / 255.0, f64::from(self.a) / 255.0]
    }
}
//#endregion 🔖️Color

//#region 🔖️Input
/// 🖼️ Authored row-major palette indices, exactly one owned octet per pixel.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapInput {
    pub width: u32,
    pub height: u32,
    pub palette: Vec<BitmapColor>,
    #[value(with = "semio_framework_value::bytes")]
    pub pixels: Vec<u8>,
}

impl Default for BitmapInput {
    fn default() -> Self {
        Self { width: 1, height: 1, palette: vec![BitmapColor::opaque(0, 0, 0)], pixels: vec![0] }
    }
}

impl BitmapInput {
    /// 🖼️ The owned index buffer, or `None` when its length does not match exactly
    /// `width * height` bytes.
    pub fn indices(&self) -> Option<Vec<u8>> {
        let decoded = self.pixels.clone();
        if decoded.len() == (self.width as usize) * (self.height as usize) {
            Some(decoded)
        } else {
            None
        }
    }
}
//#endregion 🔖️Input

//#region 🔖️Output
/// 🧩️ What the solve must produce — a size and whether the output torus wraps. The pixels
/// themselves are inferred, never persisted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapOutputSpec {
    pub width: u32,
    pub height: u32,
    pub periodic: bool,
}

impl Default for BitmapOutputSpec {
    fn default() -> Self {
        Self { width: 1, height: 1, periodic: false }
    }
}
//#endregion 🔖️Output

//#region 🔖️Model
/// ⚙️ Overlapping-model parameters. `symmetry` is the classic `1..=8` D4 expansion count: the first
/// `symmetry` elements of `Transform2d::ALL` are applied to every extracted window before
/// deduplication, so `1` learns the sample verbatim and `8` learns it under the full dihedral
/// group. `ground`, when set, is a palette index every bottom-row output cell is forced to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapOverlappingModel {
    pub pattern_size: u32,
    pub symmetry: u32,
    pub periodic_input: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ground: Option<u32>,
}

impl Default for BitmapOverlappingModel {
    fn default() -> Self {
        Self { pattern_size: 2, symmetry: 1, periodic_input: true, ground: None }
    }
}

/// ⚙️ The inclusive pattern-size window this artifact admits — `N = 1` learns single pixels (no
/// overlap information at all) and `N > 5` explodes the pattern universe past what a bounded
/// inference step budget can compile.
pub const BITMAP_PATTERN_SIZE_RANGE: (u32, u32) = (2, 5);

/// ⚙️ The inclusive D4 expansion count.
pub const BITMAP_SYMMETRY_RANGE: (u32, u32) = (1, 8);
//#endregion 🔖️Model

//#region 🔖️Pinned
/// 📌️ One output cell pre-assigned to a palette colour — a hard domain restriction the solve must
/// respect, never something the solve writes back.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapPinnedPixel {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

/// 📌️ The canonical id-key of one pin — pins carry no author-visible id, so the diff machinery
/// keys them by their own coordinates, which is exactly what makes a pin point-invertible.
pub fn pin_key(x: u32, y: u32) -> String {
    format!("{x}:{y}")
}
//#endregion 🔖️Pinned

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.wfc.bitmap")]
pub struct BitmapSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🎲 Deterministic solve seed — PERSISTED, authored only via `change-seed`, never ambient: the
    /// solve inference's `DepHash` caching is sound only because `compute` is a pure function of
    /// snapshot content, seed included.
    #[state(artifact)]
    pub seed: u64,
    #[state(artifact)]
    pub input: BitmapInput,
    #[state(artifact)]
    pub output: BitmapOutputSpec,
    #[state(artifact)]
    pub model: BitmapOverlappingModel,
    #[state(artifact)]
    #[value(default)]
    pub pinned: Vec<BitmapPinnedPixel>,
}

impl Default for BitmapSnapshot {
    fn default() -> Self {
        Self {
            schema: WFC_BITMAP_DOCUMENT_SCHEMA.into(),
            seed: 0,
            input: BitmapInput::default(),
            output: BitmapOutputSpec::default(),
            model: BitmapOverlappingModel::default(),
            pinned: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Addressing
pub fn pin_index(snapshot: &BitmapSnapshot, x: u32, y: u32) -> Option<usize> {
    snapshot.pinned.iter().position(|pin| pin.x == x && pin.y == y)
}

/// 📌️ Where a pin at `(x, y)` belongs in the canonical row-major pin order. Pins are kept sorted
/// so `pin-pixel`'s insert and `unpin-pixel`'s remove round-trip at the same index — the
/// "retained rows must be point-invertible" law, which an append-shaped insert violates.
pub fn ordered_pin_index(pinned: &[BitmapPinnedPixel], x: u32, y: u32) -> usize {
    pinned.iter().position(|pin| (pin.y, pin.x) > (y, x)).unwrap_or(pinned.len())
}

/// 🎨️ Every palette index the input pixels or the pins actually reference.
pub fn used_palette_indices(snapshot: &BitmapSnapshot) -> Vec<u32> {
    let mut used: Vec<u32> = Vec::new();
    if let Some(indices) = snapshot.input.indices() {
        for index in indices {
            let value = u32::from(index);
            if !used.contains(&value) {
                used.push(value);
            }
        }
    }
    for pin in &snapshot.pinned {
        if !used.contains(&pin.color) {
            used.push(pin.color);
        }
    }
    used.sort_unstable();
    used
}



/// 🖌️ Reads one rectangular region of palette indices out of a decoded buffer — the shape an
/// inverse `set-input-pixels` carries.
pub fn read_region(buffer: &[u8], buffer_width: u32, buffer_height: u32, x: u32, y: u32, width: u32, height: u32) -> Option<Vec<u8>> {
    if width == 0 || height == 0 || x.saturating_add(width) > buffer_width || y.saturating_add(height) > buffer_height {
        return None;
    }
    let mut out = Vec::with_capacity((width as usize) * (height as usize));
    for row in 0..height as usize {
        let source = ((y as usize + row) * buffer_width as usize) + x as usize;
        out.extend_from_slice(&buffer[source..source + width as usize]);
    }
    Some(out)
}

/// 📐️ Resizes an index buffer, keeping the overlapping top-left region and padding with index `0`.
pub fn resized_buffer(buffer: &[u8], from_width: u32, from_height: u32, to_width: u32, to_height: u32) -> Vec<u8> {
    let mut out = vec![0u8; (to_width as usize) * (to_height as usize)];
    for y in 0..from_height.min(to_height) as usize {
        for x in 0..from_width.min(to_width) as usize {
            out[y * to_width as usize + x] = buffer[y * from_width as usize + x];
        }
    }
    out
}
//#endregion 🔖️Addressing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests




pub fn write_region(buffer: &mut [u8], buffer_width: u32, buffer_height: u32, x: u32, y: u32, width: u32, height: u32, region: &[u8]) -> bool {
    if width == 0 || height == 0 {
        return false;
    }
    if x.saturating_add(width) > buffer_width || y.saturating_add(height) > buffer_height {
        return false;
    }
    if region.len() != (width as usize) * (height as usize) {
        return false;
    }
    for row in 0..height as usize {
        let destination = ((y as usize + row) * buffer_width as usize) + x as usize;
        buffer[destination..destination + width as usize].copy_from_slice(&region[row * width as usize..(row + 1) * width as usize]);
    }
    true
}
