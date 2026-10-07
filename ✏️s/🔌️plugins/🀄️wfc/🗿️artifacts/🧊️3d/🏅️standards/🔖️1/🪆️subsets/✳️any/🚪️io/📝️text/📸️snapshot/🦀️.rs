//! 📜️ `wfc3d` artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `*Dsl` types below are LOCAL structural twins of the schema tree's own records, not derives
//! on those records directly, for one measured reason: `Tile::media` is a `TileMedia3d` whose
//! `MeshChild` variant carries a `store::ArtifactChild<SemioMeshSnapshot>` — a foreign type this
//! crate can implement neither `dsl::DslField` nor `dsl::DslRecord` for. The twin carries the field
//! as `dsl::DslValue`, the engine's own schema-less literal, and bridges at the boundary through
//! `semio_framework_value::ToValue::to_value`/`semio_framework_value::FromValue::from_value`, which are defined for every `ToValue`/`FromValue`
//! type. The remaining three records are twinned for symmetry, so one file states this subset's
//! whole text grammar instead of scattering `#[dsl]` attributes across a schema file that must stay
//! representation-free.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::snapshot::{GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};

//#region 🔖️Examples
/// 📄️ The three authored problem specs this subset ships, in their own `.wfc3d` DSL.
pub const WFC3D_EXAMPLE_CORRIDOR_TEXT: &str = include_str!("../../../📚️examples/🚪️two-room-corridor/🖼️assets/🚪️two-room-corridor/🗣️.dsl.semio");
pub const WFC3D_EXAMPLE_FACADE_TEXT: &str = include_str!("../../../📚️examples/🧱️wall-roof-facade-strip/🖼️assets/🧱️wall-roof-facade-strip/🗣️.dsl.semio");
pub const WFC3D_EXAMPLE_TOWER_TEXT: &str = include_str!("../../../📚️examples/🗼️tower-stack/🖼️assets/🗼️tower-stack/🗣️.dsl.semio");
//#endregion 🔖️Examples

//#region 🔖️DslMirror
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct Slot3dDsl {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    #[dsl(key = "pinned")]
    pub pinned_tile_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct SlotEdgeDsl {
    pub id: String,
    pub from_slot_id: String,
    pub to_slot_id: String,
    pub relation: String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct TileDsl {
    pub id: String,
    pub label: Option<String>,
    pub weight: f64,
    pub media: semio_framework_value::DslValue,
}

impl Default for TileDsl {
    fn default() -> Self {
        Self { id: String::new(), label: None, weight: 1.0, media: semio_framework_value::DslValue::Null }
    }
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct GraphRuleDsl {
    pub id: String,
    pub tile_a_id: String,
    pub tile_b_id: String,
    pub relation: Option<String>,
    pub allowed: bool,
}

pub fn slot_to_dsl(slot: &Slot3d) -> Slot3dDsl {
    Slot3dDsl { id: slot.id.clone(), x: slot.x, y: slot.y, z: slot.z, width: slot.width, height: slot.height, depth: slot.depth, pinned_tile_id: slot.pinned_tile_id.clone() }
}

pub fn slot_from_dsl(slot: Slot3dDsl) -> Slot3d {
    Slot3d { id: slot.id, x: slot.x, y: slot.y, z: slot.z, width: slot.width, height: slot.height, depth: slot.depth, pinned_tile_id: slot.pinned_tile_id }
}

pub fn edge_to_dsl(edge: &SlotEdge) -> SlotEdgeDsl {
    SlotEdgeDsl { id: edge.id.clone(), from_slot_id: edge.from_slot_id.clone(), to_slot_id: edge.to_slot_id.clone(), relation: edge.relation.clone() }
}

pub fn edge_from_dsl(edge: SlotEdgeDsl) -> SlotEdge {
    SlotEdge { id: edge.id, from_slot_id: edge.from_slot_id, to_slot_id: edge.to_slot_id, relation: edge.relation }
}

/// 🖼️ `media` is the one field that cannot be a derive: see this file's own docstring. A medium that
/// refuses to project is `Null`, never a silent drop.
pub fn tile_to_dsl(tile: &Tile) -> TileDsl {
    TileDsl { id: tile.id.clone(), label: tile.label.clone(), weight: tile.weight, media: semio_framework_value::ToValue::to_value(&tile.media) }
}

pub fn tile_from_dsl(tile: TileDsl) -> Result<Tile, semio_framework_diagnostic::TextError> {
    let media: TileMedia3d = match tile.media {
        semio_framework_value::DslValue::Null => TileMedia3d::default(),
        other => semio_framework_value::FromValue::from_value(other).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("invalid tile media: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?,
    };
    Ok(Tile { id: tile.id, label: tile.label, weight: tile.weight, media })
}

pub fn rule_to_dsl(rule: &GraphRule) -> GraphRuleDsl {
    GraphRuleDsl { id: rule.id.clone(), tile_a_id: rule.tile_a_id.clone(), tile_b_id: rule.tile_b_id.clone(), relation: rule.relation.clone(), allowed: rule.allowed }
}

pub fn rule_from_dsl(rule: GraphRuleDsl) -> GraphRule {
    GraphRule { id: rule.id, tile_a_id: rule.tile_a_id, tile_b_id: rule.tile_b_id, relation: rule.relation, allowed: rule.allowed }
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(id = "wfc.wfc3d", layout = "lines")]
pub(crate) struct Wfc3dSnapshotDsl {
    schema: String,
    seed: u64,
    #[dsl(table)]
    slots: Vec<Slot3dDsl>,
    #[dsl(table)]
    edges: Vec<SlotEdgeDsl>,
    #[dsl(table)]
    tiles: Vec<TileDsl>,
    #[dsl(table)]
    rules: Vec<GraphRuleDsl>,
}

impl Default for Wfc3dSnapshotDsl {
    fn default() -> Self {
        Self { schema: WFC3D_DOCUMENT_SCHEMA.into(), seed: 0, slots: Vec::new(), edges: Vec::new(), tiles: Vec::new(), rules: Vec::new() }
    }
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` — the derive no longer emits these traits, so every
/// artifact states its own envelope discipline.
impl store::ArtifactDsl for Wfc3dSnapshotDsl {
    const EXTENSION: &'static str = "wfc3d";
    fn envelope_id() -> &'static str {
        "wfc.wfc3d"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}



pub(crate) fn wfc3d_document_to_dsl(document: &Wfc3dSnapshot) -> Wfc3dSnapshotDsl {
    Wfc3dSnapshotDsl {
        schema: document.schema.clone(),
        seed: document.seed,
        slots: document.slots.iter().map(slot_to_dsl).collect(),
        edges: document.edges.iter().map(edge_to_dsl).collect(),
        tiles: document.tiles.iter().map(tile_to_dsl).collect(),
        rules: document.rules.iter().map(rule_to_dsl).collect(),
    }
}

pub(crate) fn wfc3d_document_from_dsl(parsed: Wfc3dSnapshotDsl) -> Result<Wfc3dSnapshot, semio_framework_diagnostic::TextError> {
    Ok(Wfc3dSnapshot {
        schema: parsed.schema,
        seed: parsed.seed,
        slots: parsed.slots.into_iter().map(slot_from_dsl).collect(),
        edges: parsed.edges.into_iter().map(edge_from_dsl).collect(),
        tiles: parsed.tiles.into_iter().map(tile_from_dsl).collect::<Result<Vec<_>, _>>()?,
        rules: parsed.rules.into_iter().map(rule_from_dsl).collect(),
    })
}

impl store::ArtifactDsl for Wfc3dSnapshot {
    const EXTENSION: &'static str = "wfc3d";
    fn envelope_id() -> &'static str {
        "wfc.wfc3d"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        wfc3d_document_from_dsl(<Wfc3dSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?)
    }

    fn print_dsl(&self) -> String {
        <Wfc3dSnapshotDsl as store::ArtifactDsl>::print_dsl(&wfc3d_document_to_dsl(self))
    }
}


//#endregion 🔖️HandcraftedArtifactCodecs

/// 📖️ Parses `.wfc3d` DSL text into a `Wfc3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Wfc3dSnapshot, semio_framework_diagnostic::TextError> {
    <Wfc3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Wfc3dSnapshot` back to `.wfc3d` DSL text.
pub fn print_dsl(document: &Wfc3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Wfc3dSnapshotText = String;
//#endregion 🚚️Carrier

#[path="🛬️native/🦀️.rs"]
mod controlled_native;
pub(crate)use controlled_native::{decode_sqlite_snapshot_native,encode_sqlite_snapshot_native};

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed;
use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
use crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight;
use crate::standards::v1::subsets::any::schema::mutations::connect_slots::connect_slots;
use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::standards::v1::subsets::any::schema::mutations::create_slot::create_slot;
use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
use crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule;
use crate::standards::v1::subsets::any::schema::mutations::delete_slot::delete_slot;
use crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::disconnect_slots;
use crate::standards::v1::subsets::any::schema::mutations::drag_slots::drag_slots;
use crate::standards::v1::subsets::any::schema::mutations::move_slot::move_slot;
use crate::standards::v1::subsets::any::schema::mutations::pin_slot::pin_slot;
use crate::standards::v1::subsets::any::schema::mutations::resize_slot::resize_slot;
use crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::{set_slot_positions, Wfc3dSlotPosition};
use crate::standards::v1::subsets::any::schema::mutations::unpin_slot::unpin_slot;

/// 🔁️ Decodes one snapshot through this subset's production JSON codec and re-encodes it — the subject half of the
/// case's `identity-round-trip` scenario.
pub fn wfc3d_snapshot_json_round_trip(text: &str) -> Result<String, String> {
    let snapshot: Wfc3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    Ok(semio_framework_pack_json::to_json_string(&snapshot))
}
}
pub use mutations_codec::*;
