//! 🧊️ `Wfc3dSnapshot` — the persisted WFC PROBLEM for an arbitrary 3d slot graph: SLOTS (boxed
//! positions in space, the solver variables), EDGES (the adjacency graph constraints propagate
//! over, each carrying a named `relation`), TILES (the placeable catalogue, each with a weight and
//! a 3d medium) and RULES (which tile pair a relation admits). The SOLVE is never stored here: it
//! is an inference over this spec (`../💡️inferences/🦀️.rs`), never mutation-authored state.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Ids
pub const WFC3D_DOCUMENT_SCHEMA: &str = "s.wfc.wfc3d";
//#endregion 🔖️Ids

//#region 🔖️Media
/// 🎨️ One straight 8-bit RGBA colour — the shared colour vocabulary every `wfc` artifact declares
/// locally (no cross-artifact crate import for a four-byte record).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// 🥽️ What one tile LOOKS like in three dimensions — either geometry authored inline in tile space
/// (a unit box, `positions` as xyz triples indexed by `indices`), or a handle into a composed
/// `s.stdio.semio@v1/mesh` child (`remodel`'s precedent). Inline geometry keeps a small catalogue
/// self-contained; a child keeps a large one out of the document.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum TileMedia3d {
    Mesh {
        #[value(default)]
        positions: Vec<f64>,
        #[value(default)]
        indices: Vec<u32>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        color: Option<Color>,
    },
    MeshChild {
        child: store::ArtifactChild<SemioMeshSnapshot>,
    },
}

impl Default for TileMedia3d {
    fn default() -> Self {
        Self::Mesh { positions: Vec::new(), indices: Vec::new(), color: None }
    }
}

/// 🀄️ One placeable tile — `weight` is the selection bias the solver's `WeightTable` reads, `media`
/// is what the preview draws where the tile lands.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Tile {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub weight: f64,
    pub media: TileMedia3d,
}
//#endregion 🔖️Media

//#region 🔖️Slot
/// 📍 One position in the graph — a solver variable. `x`/`y`/`z` place its box's MINIMUM corner and
/// `width`/`height`/`depth` its extent, so tile media authored in the unit box `0..1` is placed at the
/// slot's origin and scaled by its extent. The solved tile assignment lives in the inference result,
/// never here.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Slot3d {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    /// 🔒 Optional user-pinned tile id — a hard pre-assignment the solve must respect.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pinned_tile_id: Option<String>,
}

/// 🔗 One adjacency EDGE between two slots. `relation` names the constraint channel: every distinct
/// string becomes its own `RelationId` in the compiled model, so "above"/"beside" can admit
/// different tile pairs over the same topology.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlotEdge {
    pub id: String,
    pub from_slot_id: String,
    pub to_slot_id: String,
    pub relation: String,
}
//#endregion 🔖️Slot

//#region 🔖️Rule
/// ⛓️ One adjacency rule over an UNORDERED tile pair. The rules ARE the compatibility table: they
/// compile onto an empty model, so a pair no rule mentions is FORBIDDEN. `allowed: true` admits the
/// pair, `allowed: false` forbids it and always wins over an admitting rule, and `relation: None`
/// states the rule for every relation at once.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct GraphRule {
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
#[artifact_schema(id = "s.wfc.wfc3d")]
pub struct Wfc3dSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🎲 Deterministic solve seed — PERSISTED, authored only via `change-seed`, never ambient: the
    /// solve inference's `DepHash` caching is sound only while `compute` is a pure function of
    /// snapshot content, seed included.
    #[state(artifact)]
    pub seed: u64,
    #[state(artifact)]
    #[value(default)]
    pub slots: Vec<Slot3d>,
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<SlotEdge>,
    #[state(artifact)]
    #[value(default)]
    pub tiles: Vec<Tile>,
    #[state(artifact)]
    #[value(default)]
    pub rules: Vec<GraphRule>,
}

impl Default for Wfc3dSnapshot {
    fn default() -> Self {
        Self { schema: WFC3D_DOCUMENT_SCHEMA.into(), seed: 0, slots: Vec::new(), edges: Vec::new(), tiles: Vec::new(), rules: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Addressing
pub fn slot_index(snapshot: &Wfc3dSnapshot, id: &str) -> Option<usize> {
    snapshot.slots.iter().position(|slot| slot.id == id)
}
pub fn edge_index(snapshot: &Wfc3dSnapshot, id: &str) -> Option<usize> {
    snapshot.edges.iter().position(|edge| edge.id == id)
}
pub fn tile_index(snapshot: &Wfc3dSnapshot, id: &str) -> Option<usize> {
    snapshot.tiles.iter().position(|tile| tile.id == id)
}
pub fn rule_index(snapshot: &Wfc3dSnapshot, id: &str) -> Option<usize> {
    snapshot.rules.iter().position(|rule| rule.id == id)
}

/// 🔤️ Where an id-keyed member belongs in its collection's canonical (lexicographic by id) order —
/// the insertion index every `create-*` builder uses so a collection stays sorted and a removal is
/// point-invertible back to exactly the position it left.
pub fn canonical_insertion_index(ids: impl IntoIterator<Item = String>, id: &str) -> usize {
    ids.into_iter().filter(|existing| existing.as_str() < id).count()
}

/// 🔤️ The canonical insertion index for one new slot id.
pub fn canonical_slot_index(snapshot: &Wfc3dSnapshot, id: &str) -> usize {
    canonical_insertion_index(snapshot.slots.iter().map(|slot| slot.id.clone()), id)
}

/// 🔤️ The canonical insertion index for one new edge id.
pub fn canonical_edge_index(snapshot: &Wfc3dSnapshot, id: &str) -> usize {
    canonical_insertion_index(snapshot.edges.iter().map(|edge| edge.id.clone()), id)
}

/// 🔤️ The canonical insertion index for one new tile id.
pub fn canonical_tile_index(snapshot: &Wfc3dSnapshot, id: &str) -> usize {
    canonical_insertion_index(snapshot.tiles.iter().map(|tile| tile.id.clone()), id)
}

/// 🔤️ The canonical insertion index for one new rule id.
pub fn canonical_rule_index(snapshot: &Wfc3dSnapshot, id: &str) -> usize {
    canonical_insertion_index(snapshot.rules.iter().map(|rule| rule.id.clone()), id)
}
//#endregion 🔖️Addressing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
