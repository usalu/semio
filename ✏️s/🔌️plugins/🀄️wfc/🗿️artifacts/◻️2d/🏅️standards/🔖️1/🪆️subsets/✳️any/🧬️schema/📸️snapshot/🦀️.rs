//! 🌊️ `Wfc2dSnapshot` — the persisted WFC problem for an ARBITRARY 2D slot graph: SLOTS (rectangles
//! at free `x`/`y` with a `width`/`height`, the solver's variables), EDGES (the explicit adjacency
//! graph propagation runs over — no grid stencil is assumed, and each edge names a RELATION class),
//! TILES (the placeable alphabet, each carrying its own inline 2D media and selection weight), and
//! RULES (per-tile-pair adjacency permissions, optionally scoped to one relation).
//!
//! The SOLVE is never stored here: it is an inference (`../💡️inferences/🦀️.rs`) over this spec.
//! Only the PROBLEM is authored; the assignment, the contradiction verdict and the entropy map are
//! all derived.
//!
//! Every id-keyed collection is kept in CANONICAL ASCENDING `id` ORDER: each mutation inserts at
//! `crate::schema::mutations::ordered_index`, never at the end, so a delete followed by its own
//! inverse restores a row's POSITION as well as its value (`📓️explore-artifact-taxonomy-template.md`
//! §3.3).

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

//#region 🔖️Ids
pub const WFC_2D_DOCUMENT_SCHEMA: &str = "s.wfc.wfc2d";
/// 🔗 The relation string an edge carries when the author names no particular adjacency class — the
/// single-relation case assembly's graph route always ran.
pub const WFC_2D_DEFAULT_RELATION: &str = "adjacent";
//#endregion 🔖️Ids

//#region 🔖️Color
/// 🎨 One straight-alpha sRGB colour, 0–255 per channel — the one colour vocabulary every media
/// variant shares, so a palette entry and a vector fill are the same type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}
//#endregion 🔖️Color

//#region 🔖️Media
/// ✏️ One path command in TILE SPACE (`0.0..=1.0` on both axes) — an SVG-flavoured subset, drawn
/// scaled into whatever rectangle the slot occupies.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue, Default)]
pub enum Wfc2dPathSegment {
    #[default]
    Close,
    Move {
        to: [f64; 2],
    },
    Line {
        to: [f64; 2],
    },
    Quad {
        ctrl: [f64; 2],
        to: [f64; 2],
    },
    Cubic {
        ctrl1: [f64; 2],
        ctrl2: [f64; 2],
        to: [f64; 2],
    },
}

/// 🖍️ One filled/stroked subpath of a vector tile.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dVectorPath {
    #[value(default)]
    pub segments: Vec<Wfc2dPathSegment>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<Wfc2dColor>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Wfc2dColor>,
    #[value(default)]
    pub stroke_width: f64,
}

/// 🖼️ What a tile actually LOOKS like. `Bitmap` carries palette-indexed pixels inline (row-major,
/// base64 of one byte per pixel); `Vector` carries tile-space paths inline; `Image` addresses an
/// `s.stdio.semio@v1/image` document instead.
///
/// 🚧️ `Image` is carried and round-trips, but this artifact declares NO `#[child]` slot for it: the
/// handle is reachable only through `tiles[].media`, which `ArtifactSchema`'s child enumeration
/// cannot see, so a host never hydrates it and the preview window draws an outline placeholder for
/// such a tile. Wiring composed-child hydration (genesis pack, `SemioMembers` on both surfaces) is a
/// real remaining increment, deliberately out of this slice — every bundled example uses `Vector`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, Default)]
pub enum Wfc2dTileMedia {
    #[default]
    Empty,
    Bitmap {
        width: u32,
        height: u32,
        palette: Vec<Wfc2dColor>,
        pixels: String,
    },
    Vector {
        paths: Vec<Wfc2dVectorPath>,
    },
    Image {
        child: store::ArtifactChild<SemioImageSnapshot>,
    },
}
//#endregion 🔖️Media

//#region 🔖️Tile
/// 🀄️ One placeable tile — the WFC pattern alphabet. `weight` is the selection bias the engine's
/// `WeightTable` consumes; `media` is what the preview window paints into a solved slot.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dTile {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub weight: f64,
    #[value(default)]
    pub media: Wfc2dTileMedia,
}
//#endregion 🔖️Tile

//#region 🔖️Slot
/// 📍 One position the solver must fill — a free rectangle, never a grid cell.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dSlot {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    /// 🔒 A hard pre-assignment the solve must respect — a domain restriction feeding the solver,
    /// never written back by it.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pinned_tile_id: Option<String>,
}

/// 🔗 One adjacency edge between two slots. `relation` names the adjacency CLASS (`"adjacent"`,
/// `"above"`, `"ring"`, …); each distinct string compiles to its own `RelationId` in the model, so
/// rules can be scoped per class.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dSlotEdge {
    pub id: String,
    pub from_slot_id: String,
    pub to_slot_id: String,
    pub relation: String,
}
//#endregion 🔖️Slot

//#region 🔖️Rule
/// ⛓️ One adjacency permission between two tile ids. `relation: None` means EVERY relation class;
/// `allowed: false` is a hard deny and always wins over any allow of the same pair.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dRule {
    pub id: String,
    pub tile_a_id: String,
    pub tile_b_id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    pub allowed: bool,
}
//#endregion 🔖️Rule

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.wfc.wfc2d")]
pub struct Wfc2dSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🎲 Deterministic solve seed — PERSISTED, authored only via `change-seed`, never ambient, so
    /// the solve inference's `DepHash` caching stays sound.
    #[state(artifact)]
    pub seed: u64,
    #[state(artifact)]
    #[value(default)]
    pub slots: Vec<Wfc2dSlot>,
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<Wfc2dSlotEdge>,
    #[state(artifact)]
    #[value(default)]
    pub tiles: Vec<Wfc2dTile>,
    #[state(artifact)]
    #[value(default)]
    pub rules: Vec<Wfc2dRule>,
}

impl Default for Wfc2dSnapshot {
    fn default() -> Self {
        Self { schema: WFC_2D_DOCUMENT_SCHEMA.into(), seed: 0, slots: Vec::new(), edges: Vec::new(), tiles: Vec::new(), rules: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Addressing
pub fn slot_index(snapshot: &Wfc2dSnapshot, id: &str) -> Option<usize> {
    snapshot.slots.iter().position(|slot| slot.id == id)
}
pub fn edge_index(snapshot: &Wfc2dSnapshot, id: &str) -> Option<usize> {
    snapshot.edges.iter().position(|edge| edge.id == id)
}
pub fn tile_index(snapshot: &Wfc2dSnapshot, id: &str) -> Option<usize> {
    snapshot.tiles.iter().position(|tile| tile.id == id)
}
pub fn rule_index(snapshot: &Wfc2dSnapshot, id: &str) -> Option<usize> {
    snapshot.rules.iter().position(|rule| rule.id == id)
}

/// 🔗 Every distinct edge relation string, ascending — the model's relation universe.
pub fn relations(snapshot: &Wfc2dSnapshot) -> Vec<String> {
    let mut names: Vec<String> = snapshot.edges.iter().map(|edge| edge.relation.clone()).collect();
    names.sort();
    names.dedup();
    names
}
//#endregion 🔖️Addressing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
