//! 📜️ `wfc3d` artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `*Dsl` types below are LOCAL structural twins of the schema tree's own records, not derives
//! on those records directly, for one measured reason: `Tile::media` is a `TileMedia3d` whose
//! `MeshChild` variant carries a `store::ArtifactChild<SemioMeshSnapshot>` — a foreign type this
//! crate can implement neither `dsl::DslField` nor `dsl::DslRecord` for. The twin carries the field
//! as `dsl::DslValue`, the engine's own schema-less literal, and bridges at the boundary through
//! `dsl::to_dsl_value`/`dsl::from_dsl_value`, which are defined for every `ToValue`/`FromValue`
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
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
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

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct SlotEdgeDsl {
    pub id: String,
    pub from_slot_id: String,
    pub to_slot_id: String,
    pub relation: String,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct TileDsl {
    pub id: String,
    pub label: Option<String>,
    pub weight: f64,
    pub media: dsl::DslValue,
}

impl Default for TileDsl {
    fn default() -> Self {
        Self { id: String::new(), label: None, weight: 1.0, media: dsl::DslValue::Null }
    }
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
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
    TileDsl { id: tile.id.clone(), label: tile.label.clone(), weight: tile.weight, media: dsl::to_dsl_value(&tile.media).unwrap_or(dsl::DslValue::Null) }
}

pub fn tile_from_dsl(tile: TileDsl) -> Result<Tile, store::TextError> {
    let media: TileMedia3d = match tile.media {
        dsl::DslValue::Null => TileMedia3d::default(),
        other => dsl::from_dsl_value(other).map_err(|error| store::TextError::new(format!("invalid tile media: {error}"), store::TextSpan::at(1, 1)))?,
    };
    Ok(Tile { id: tile.id, label: tile.label, weight: tile.weight, media })
}

pub fn rule_to_dsl(rule: &GraphRule) -> GraphRuleDsl {
    GraphRuleDsl { id: rule.id.clone(), tile_a_id: rule.tile_a_id.clone(), tile_b_id: rule.tile_b_id.clone(), relation: rule.relation.clone(), allowed: rule.allowed }
}

pub fn rule_from_dsl(rule: GraphRuleDsl) -> GraphRule {
    GraphRule { id: rule.id, tile_a_id: rule.tile_a_id, tile_b_id: rule.tile_b_id, relation: rule.relation, allowed: rule.allowed }
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(id = "wfc.wfc3d", layout = "lines")]
struct Wfc3dSnapshotDsl {
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
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Wfc3dSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

fn wfc3d_document_to_dsl(document: &Wfc3dSnapshot) -> Wfc3dSnapshotDsl {
    Wfc3dSnapshotDsl {
        schema: document.schema.clone(),
        seed: document.seed,
        slots: document.slots.iter().map(slot_to_dsl).collect(),
        edges: document.edges.iter().map(edge_to_dsl).collect(),
        tiles: document.tiles.iter().map(tile_to_dsl).collect(),
        rules: document.rules.iter().map(rule_to_dsl).collect(),
    }
}

fn wfc3d_document_from_dsl(parsed: Wfc3dSnapshotDsl) -> Result<Wfc3dSnapshot, store::TextError> {
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

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        wfc3d_document_from_dsl(<Wfc3dSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?)
    }

    fn print_dsl(&self) -> String {
        <Wfc3dSnapshotDsl as store::ArtifactDsl>::print_dsl(&wfc3d_document_to_dsl(self))
    }
}

impl store::ArtifactPack for Wfc3dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Wfc3dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&wfc3d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Wfc3dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        wfc3d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        <Wfc3dSnapshotDsl as store::ArtifactPack>::record_spec()
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 📖️ Parses `.wfc3d` DSL text into a `Wfc3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Wfc3dSnapshot, store::TextError> {
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
