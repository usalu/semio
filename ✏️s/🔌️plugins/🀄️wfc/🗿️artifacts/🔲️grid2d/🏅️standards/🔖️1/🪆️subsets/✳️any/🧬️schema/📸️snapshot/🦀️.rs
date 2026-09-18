//! 🔲️ `Grid2dSnapshot` — the persisted WFC problem for a regular rectangular grid: the grid extent
//! and cell size, the periodicity of each axis, the TILE catalogue (each tile carrying its own 2D
//! media, drawn scaled into one cell), the directional ADJACENCY RULES between tile pairs, the
//! PINNED cells (hard pre-assignments) and the MASKED cells (holes the solve never fills). The
//! solved assignment is NEVER stored here — it is an inference over this spec
//! (`../💡️inferences/🦀️.rs`), exactly as `s.wfc.wfc2d`/`s.wfc.grid3d` do.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

//#region 🔖️Ids
pub const WFC_GRID2D_DOCUMENT_SCHEMA: &str = "s.wfc.grid2d";
//#endregion 🔖️Ids

//#region 🔖️Media
/// 🎨️ One 8-bit-per-channel colour — the single colour vocabulary every `wfc` artifact defines
/// locally (never imported across artifact crates: app-to-app coupling is audited and forbidden).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WfcColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

/// 📍️ One point in a tile's own `0..1` unit space — the tile is drawn scaled into the cell rect, so
/// media coordinates are resolution independent.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WfcPoint2 {
    pub x: f64,
    pub y: f64,
}

/// ✏️ One SVG-flavoured path command (`M`/`L`/`Q`/`C`/`Z`), mirroring `🖍️draw`'s own `PathSegment`
/// vocabulary field-for-field WITHOUT depending on that crate — a deliberate local copy, since an
/// artifact crate may never import another app plugin's crate.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum WfcPathSegment {
    MoveTo { to: WfcPoint2 },
    LineTo { to: WfcPoint2 },
    QuadTo { ctrl: WfcPoint2, to: WfcPoint2 },
    CubicTo { ctrl1: WfcPoint2, ctrl2: WfcPoint2, to: WfcPoint2 },
    Close,
}

/// 🖊️ One filled/stroked outline inside a vector tile.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WfcVectorPath {
    #[value(default)]
    pub segments: Vec<WfcPathSegment>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<WfcColor>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<WfcColor>,
    pub stroke_width: f64,
}

/// 🖼️ What one tile LOOKS like inside its cell: an inline indexed bitmap, an inline vector drawing,
/// or a composed `s.stdio.semio@v1/image` child. `Bitmap.pixels` is base64 of one palette index per
/// pixel, row-major — a compact, wire-safe carrier that keeps a 64×64 tile far under the retained
/// surface budget (`format: "base64"`, never `contentEncoding`: the owned JSON-Schema validator
/// refuses that keyword).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum WfcTileMedia2d {
    Bitmap { width: u32, height: u32, palette: Vec<WfcColor>, pixels: String },
    Vector { paths: Vec<WfcVectorPath> },
    Image { child: store::ArtifactChild<SemioImageSnapshot> },
}

impl Default for WfcTileMedia2d {
    fn default() -> Self {
        Self::Vector { paths: Vec::new() }
    }
}

/// 🀄️ One authored tile — the WFC pattern universe is exactly this list, in canonical id order.
/// `weight` is the sampling bias (`wfc_engine::weights::WeightTable` input); a non-positive weight
/// is refused by `create-tile`/`change-tile-weight`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WfcTile2d {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub weight: f64,
    pub media: WfcTileMedia2d,
}

/// 🧭️ Where tile B lies relative to tile A. `LEFT`/`RIGHT` are the `x` axis, `TOP`/`BOTTOM` the `y`
/// axis with `y` growing DOWNWARD (`TOP` = `y - 1`), matching the grid's row-major cell order.
/// 🔠️ The wire spelling is stated per variant, not through `rename_all`: the value derive supports
/// only `camelCase`/`kebab-case`/`lowercase`/`snake_case` and SILENTLY ignores anything else, which
/// would leave the wire on PascalCase while the JSON Schema and the GraphQL enum declare
/// `SCREAMING_SNAKE_CASE`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ToValue, FromValue)]
pub enum WfcDirection2d {
    #[default]
    #[value(rename = "LEFT")]
    Left,
    #[value(rename = "RIGHT")]
    Right,
    #[value(rename = "TOP")]
    Top,
    #[value(rename = "BOTTOM")]
    Bottom,
}

impl WfcDirection2d {
    /// 🧭️ The `(dx, dy)` step this direction takes from A to B.
    pub fn offset(self) -> (i32, i32) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Top => (0, -1),
            Self::Bottom => (0, 1),
        }
    }

    /// 🔁️ The direction that carries B back to A.
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
        }
    }

    pub const ALL: [Self; 4] = [Self::Left, Self::Right, Self::Top, Self::Bottom];
}

/// ⛓️ One directional adjacency constraint: `allowed = true` permits tile B in `direction` of tile
/// A. **A pair with no rule for a direction defaults to `allowed = false`** — the rule set is the
/// complete whitelist, so an empty rule set collapses to an unsatisfiable problem the moment the
/// grid has two cells. `allowed = false` rows are authored so an editor can carry an explicit
/// refusal (and so a rule can be toggled without losing its id).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WfcAdjacencyRule2d {
    pub id: String,
    pub tile_a_id: String,
    pub tile_b_id: String,
    pub direction: WfcDirection2d,
    pub allowed: bool,
}

/// 📌️ A cell the author has pre-assigned — a hard pin the solve must respect.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WfcPinnedCell2d {
    pub x: u32,
    pub y: u32,
    pub tile_id: String,
}

/// 🕳️ A cell that is not part of the problem at all (a hole in the grid).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WfcCell2d {
    pub x: u32,
    pub y: u32,
}
//#endregion 🔖️Media

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.wfc.grid2d")]
pub struct Grid2dSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🎲 Deterministic solve seed — PERSISTED, authored only via `change-seed`, never ambient, so
    /// the solve inference's `DepHash` caching stays sound.
    #[state(artifact)]
    pub seed: u64,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    pub cell_width: f64,
    #[state(artifact)]
    pub cell_height: f64,
    #[state(artifact)]
    pub periodic_x: bool,
    #[state(artifact)]
    pub periodic_y: bool,
    #[state(artifact)]
    #[value(default)]
    pub tiles: Vec<WfcTile2d>,
    #[state(artifact)]
    #[value(default)]
    pub rules: Vec<WfcAdjacencyRule2d>,
    #[state(artifact)]
    #[value(default)]
    pub pinned: Vec<WfcPinnedCell2d>,
    #[state(artifact)]
    #[value(default)]
    pub masked: Vec<WfcCell2d>,
}

impl Default for Grid2dSnapshot {
    fn default() -> Self {
        Self {
            schema: WFC_GRID2D_DOCUMENT_SCHEMA.into(),
            seed: 0,
            width: 1,
            height: 1,
            cell_width: 1.0,
            cell_height: 1.0,
            periodic_x: false,
            periodic_y: false,
            tiles: Vec::new(),
            rules: Vec::new(),
            pinned: Vec::new(),
            masked: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Addressing
/// 🔑️ The canonical sort key of a pinned/masked cell — row-major, so a cell list is always ordered
/// exactly the way the grid itself is walked.
pub fn cell_key(x: u32, y: u32) -> (u32, u32) {
    (y, x)
}

pub fn tile_index(snapshot: &Grid2dSnapshot, id: &str) -> Option<usize> {
    snapshot.tiles.iter().position(|tile| tile.id == id)
}

pub fn rule_index(snapshot: &Grid2dSnapshot, id: &str) -> Option<usize> {
    snapshot.rules.iter().position(|rule| rule.id == id)
}

pub fn pinned_index(snapshot: &Grid2dSnapshot, x: u32, y: u32) -> Option<usize> {
    snapshot.pinned.iter().position(|cell| cell.x == x && cell.y == y)
}

pub fn masked_index(snapshot: &Grid2dSnapshot, x: u32, y: u32) -> Option<usize> {
    snapshot.masked.iter().position(|cell| cell.x == x && cell.y == y)
}

/// 🧭️ Whether `(x, y)` is inside the authored grid extent.
pub fn in_bounds(snapshot: &Grid2dSnapshot, x: u32, y: u32) -> bool {
    x < snapshot.width && y < snapshot.height
}

/// 📐️ The world rect one cell occupies, at the authored cell size.
pub fn cell_rect(snapshot: &Grid2dSnapshot, x: u32, y: u32) -> (f64, f64, f64, f64) {
    (f64::from(x) * snapshot.cell_width, f64::from(y) * snapshot.cell_height, snapshot.cell_width, snapshot.cell_height)
}
//#endregion 🔖️Addressing

//#region 🔖️PaletteStream
const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 🔤️ Encodes one palette index per pixel, row-major, as the base64 stream `Bitmap.pixels` carries.
/// Dependency-free by repo rule — no runtime library is pulled in for a 24-character transform.
pub fn encode_palette_indices(indices: &[u8]) -> String {
    let mut out = String::with_capacity(indices.len().div_ceil(3) * 4);
    for chunk in indices.chunks(3) {
        let taken = chunk.len();
        let mut block = 0u32;
        for (offset, byte) in chunk.iter().enumerate() {
            block |= u32::from(*byte) << (16 - 8 * offset);
        }
        for slot in 0..=taken {
            out.push(char::from(BASE64_ALPHABET[((block >> (18 - 6 * slot)) & 0x3f) as usize]));
        }
        for _ in taken..3 {
            out.push('=');
        }
    }
    out
}

/// 🔤️ The exact inverse of [`encode_palette_indices`]. A malformed stream yields the prefix it
/// could read, never a panic — a tile is media, not a protocol.
pub fn decode_palette_indices(pixels: &str) -> Vec<u8> {
    fn value(byte: u8) -> Option<u8> {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let clean: Vec<u8> = pixels.bytes().filter(|byte| *byte != b'=' && !byte.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        let Some(values) = chunk.iter().map(|byte| value(*byte)).collect::<Option<Vec<u8>>>() else {
            return out;
        };
        let taken = values.len();
        let combined = values.iter().fold(0u32, |accumulator, value| (accumulator << 6) | u32::from(*value)) << ((4 - taken) * 6);
        out.push((combined >> 16) as u8);
        if taken > 2 {
            out.push((combined >> 8) as u8);
        }
        if taken > 3 {
            out.push(combined as u8);
        }
    }
    out
}
//#endregion 🔖️PaletteStream

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
