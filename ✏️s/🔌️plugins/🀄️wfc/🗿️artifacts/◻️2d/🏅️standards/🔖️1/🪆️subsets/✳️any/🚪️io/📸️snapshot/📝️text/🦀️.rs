//! 📜️ WFC 2D artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `*Dsl` types below are LOCAL structural twins of the schema tree's own records rather than
//! derives on those records directly, for one measured reason: `Wfc2dTile::media` is a
//! data-carrying enum (`Wfc2dTileMedia`), which the record grammar has no field shape for. The twin
//! carries it as `dsl::DslValue` — the engine's own schema-less literal — and bridges at the
//! boundary through `dsl::to_dsl_value`/`dsl::from_dsl_value`, which are defined for every
//! `ToValue`/`FromValue` type. The remaining records are twinned for symmetry, so ONE file states
//! this subset's whole text grammar instead of scattering `#[dsl]` attributes across a schema file
//! that must stay representation-free (assembly's `params: SemioValue` precedent).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::snapshot::{Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile, Wfc2dTileMedia, WFC_2D_DOCUMENT_SCHEMA};

//#region 🔖️DslMirror
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct Wfc2dSlotDsl {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[dsl(key = "pinned")]
    pub pinned_tile_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct Wfc2dSlotEdgeDsl {
    pub id: String,
    pub from_slot_id: String,
    pub to_slot_id: String,
    pub relation: String,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct Wfc2dTileDsl {
    pub id: String,
    pub label: Option<String>,
    pub weight: f64,
    pub media: dsl::DslValue,
}

impl Default for Wfc2dTileDsl {
    fn default() -> Self {
        Self { id: String::new(), label: None, weight: 1.0, media: dsl::DslValue::Null }
    }
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct Wfc2dRuleDsl {
    pub id: String,
    pub tile_a_id: String,
    pub tile_b_id: String,
    pub relation: Option<String>,
    pub allowed: bool,
}

pub fn slot_to_dsl(slot: &Wfc2dSlot) -> Wfc2dSlotDsl {
    Wfc2dSlotDsl { id: slot.id.clone(), x: slot.x, y: slot.y, width: slot.width, height: slot.height, pinned_tile_id: slot.pinned_tile_id.clone() }
}

pub fn slot_from_dsl(slot: Wfc2dSlotDsl) -> Wfc2dSlot {
    Wfc2dSlot { id: slot.id, x: slot.x, y: slot.y, width: slot.width, height: slot.height, pinned_tile_id: slot.pinned_tile_id }
}

pub fn edge_to_dsl(edge: &Wfc2dSlotEdge) -> Wfc2dSlotEdgeDsl {
    Wfc2dSlotEdgeDsl { id: edge.id.clone(), from_slot_id: edge.from_slot_id.clone(), to_slot_id: edge.to_slot_id.clone(), relation: edge.relation.clone() }
}

pub fn edge_from_dsl(edge: Wfc2dSlotEdgeDsl) -> Wfc2dSlotEdge {
    Wfc2dSlotEdge { id: edge.id, from_slot_id: edge.from_slot_id, to_slot_id: edge.to_slot_id, relation: edge.relation }
}

/// 🖼️ `media` is the one field that cannot be a derive: see this file's own docstring. A value that
/// refuses to project is `Null`, never a silent drop — `Wfc2dTileMedia::Empty` IS this field's
/// default, so the twin says exactly what the record says.
pub fn tile_to_dsl(tile: &Wfc2dTile) -> Wfc2dTileDsl {
    Wfc2dTileDsl { id: tile.id.clone(), label: tile.label.clone(), weight: tile.weight, media: dsl::to_dsl_value(&tile.media).unwrap_or(dsl::DslValue::Null) }
}

pub fn tile_from_dsl(tile: Wfc2dTileDsl) -> Result<Wfc2dTile, store::TextError> {
    let media: Wfc2dTileMedia = match tile.media {
        dsl::DslValue::Null => Wfc2dTileMedia::default(),
        other => dsl::from_dsl_value(other).map_err(|error| store::TextError::new(format!("invalid tile media: {error}"), store::TextSpan::at(1, 1)))?,
    };
    Ok(Wfc2dTile { id: tile.id, label: tile.label, weight: tile.weight, media })
}

pub fn rule_to_dsl(rule: &Wfc2dRule) -> Wfc2dRuleDsl {
    Wfc2dRuleDsl { id: rule.id.clone(), tile_a_id: rule.tile_a_id.clone(), tile_b_id: rule.tile_b_id.clone(), relation: rule.relation.clone(), allowed: rule.allowed }
}

pub fn rule_from_dsl(rule: Wfc2dRuleDsl) -> Wfc2dRule {
    Wfc2dRule { id: rule.id, tile_a_id: rule.tile_a_id, tile_b_id: rule.tile_b_id, relation: rule.relation, allowed: rule.allowed }
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(id = "wfc.wfc2d", layout = "lines")]
struct Wfc2dSnapshotDsl {
    schema: String,
    seed: u64,
    #[dsl(table)]
    slots: Vec<Wfc2dSlotDsl>,
    #[dsl(table)]
    edges: Vec<Wfc2dSlotEdgeDsl>,
    #[dsl(table)]
    tiles: Vec<Wfc2dTileDsl>,
    #[dsl(table)]
    rules: Vec<Wfc2dRuleDsl>,
}

impl Default for Wfc2dSnapshotDsl {
    fn default() -> Self {
        Self { schema: WFC_2D_DOCUMENT_SCHEMA.into(), seed: 0, slots: Vec::new(), edges: Vec::new(), tiles: Vec::new(), rules: Vec::new() }
    }
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` — the derive stopped emitting these traits, so every
/// artifact states its own envelope discipline.
impl store::ArtifactDsl for Wfc2dSnapshotDsl {
    const EXTENSION: &'static str = "wfc2d";
    fn envelope_id() -> &'static str {
        "wfc.wfc2d"
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

impl store::ArtifactPack for Wfc2dSnapshotDsl {
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

fn wfc2d_document_to_dsl(document: &Wfc2dSnapshot) -> Wfc2dSnapshotDsl {
    Wfc2dSnapshotDsl {
        schema: document.schema.clone(),
        seed: document.seed,
        slots: document.slots.iter().map(slot_to_dsl).collect(),
        edges: document.edges.iter().map(edge_to_dsl).collect(),
        tiles: document.tiles.iter().map(tile_to_dsl).collect(),
        rules: document.rules.iter().map(rule_to_dsl).collect(),
    }
}

fn wfc2d_document_from_dsl(parsed: Wfc2dSnapshotDsl) -> Result<Wfc2dSnapshot, store::TextError> {
    Ok(Wfc2dSnapshot {
        schema: parsed.schema,
        seed: parsed.seed,
        slots: parsed.slots.into_iter().map(slot_from_dsl).collect(),
        edges: parsed.edges.into_iter().map(edge_from_dsl).collect(),
        tiles: parsed.tiles.into_iter().map(tile_from_dsl).collect::<Result<Vec<_>, _>>()?,
        rules: parsed.rules.into_iter().map(rule_from_dsl).collect(),
    })
}

impl store::ArtifactDsl for Wfc2dSnapshot {
    const EXTENSION: &'static str = "wfc2d";
    fn envelope_id() -> &'static str {
        "wfc.wfc2d"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        wfc2d_document_from_dsl(<Wfc2dSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?)
    }

    fn print_dsl(&self) -> String {
        <Wfc2dSnapshotDsl as store::ArtifactDsl>::print_dsl(&wfc2d_document_to_dsl(self))
    }
}

impl store::ArtifactPack for Wfc2dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Wfc2dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&wfc2d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Wfc2dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        wfc2d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        <Wfc2dSnapshotDsl as store::ArtifactPack>::record_spec()
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 📖️ Parses `.wfc2d` DSL text into a `Wfc2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Wfc2dSnapshot, store::TextError> {
    <Wfc2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Wfc2dSnapshot` back to `.wfc2d` DSL text.
pub fn print_dsl(document: &Wfc2dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Wfc2dSnapshotText = String;
//#endregion 🚚️Carrier
