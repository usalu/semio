//! 📜️ `s.wfc.grid2d` — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `*Dsl` types below are LOCAL structural twins of the schema tree's own records rather than
//! derives on those records, for one measured reason: `WfcTile2d::media` is a tagged ENUM whose
//! `Image` arm carries a `store::ArtifactChild<SemioImageSnapshot>`, so a single `dsl::DslRecord`
//! derive cannot state it. The twin carries the field as `dsl::DslValue` — the engine's own
//! schema-less literal — and bridges at the boundary through `dsl::to_dsl_value`/
//! `dsl::from_dsl_value`, which exist for every `ToValue`/`FromValue` type. The remaining records
//! are twinned for symmetry so ONE file states this subset's whole text grammar.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcDirection2d, WfcPinnedCell2d, WfcTile2d, WfcTileMedia2d, WFC_GRID2D_DOCUMENT_SCHEMA};

//#region 🔖️Direction
/// 🧭️ The wire token of one direction — the same `SCREAMING_SNAKE_CASE` spelling the JSON Schema
/// and the GraphQL enum declare, so text, JSON and pack never disagree.
pub fn direction_token(direction: WfcDirection2d) -> &'static str {
    match direction {
        WfcDirection2d::Left => "LEFT",
        WfcDirection2d::Right => "RIGHT",
        WfcDirection2d::Top => "TOP",
        WfcDirection2d::Bottom => "BOTTOM",
    }
}

pub fn direction_from_token(token: &str) -> Result<WfcDirection2d, store::TextError> {
    match token {
        "LEFT" => Ok(WfcDirection2d::Left),
        "RIGHT" => Ok(WfcDirection2d::Right),
        "TOP" => Ok(WfcDirection2d::Top),
        "BOTTOM" => Ok(WfcDirection2d::Bottom),
        other => Err(store::TextError::new(format!("unknown grid2d direction '{other}'"), store::TextSpan::at(1, 1))),
    }
}
//#endregion 🔖️Direction

//#region 🔖️DslMirror
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct WfcTile2dDsl {
    pub id: String,
    pub label: Option<String>,
    pub weight: f64,
    pub media: dsl::DslValue,
}

impl Default for WfcTile2dDsl {
    fn default() -> Self {
        Self { id: String::new(), label: None, weight: 1.0, media: dsl::DslValue::Null }
    }
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct WfcAdjacencyRule2dDsl {
    pub id: String,
    pub tile_a_id: String,
    pub tile_b_id: String,
    pub direction: String,
    pub allowed: bool,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct WfcPinnedCell2dDsl {
    pub x: u32,
    pub y: u32,
    pub tile_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct WfcCell2dDsl {
    pub x: u32,
    pub y: u32,
}

/// 🎨️ `media` is the one field that cannot be a derive: see this file's own docstring. Media that
/// refuses to project is `Null`, never a silent drop.
pub fn tile_to_dsl(tile: &WfcTile2d) -> WfcTile2dDsl {
    WfcTile2dDsl { id: tile.id.clone(), label: tile.label.clone(), weight: tile.weight, media: dsl::to_dsl_value(&tile.media).unwrap_or(dsl::DslValue::Null) }
}

pub fn tile_from_dsl(tile: WfcTile2dDsl) -> Result<WfcTile2d, store::TextError> {
    let media: WfcTileMedia2d = match tile.media {
        dsl::DslValue::Null => WfcTileMedia2d::default(),
        other => dsl::from_dsl_value(other).map_err(|error| store::TextError::new(format!("invalid tile media: {error}"), store::TextSpan::at(1, 1)))?,
    };
    Ok(WfcTile2d { id: tile.id, label: tile.label, weight: tile.weight, media })
}

pub fn rule_to_dsl(rule: &WfcAdjacencyRule2d) -> WfcAdjacencyRule2dDsl {
    WfcAdjacencyRule2dDsl { id: rule.id.clone(), tile_a_id: rule.tile_a_id.clone(), tile_b_id: rule.tile_b_id.clone(), direction: direction_token(rule.direction).to_string(), allowed: rule.allowed }
}

pub fn rule_from_dsl(rule: WfcAdjacencyRule2dDsl) -> Result<WfcAdjacencyRule2d, store::TextError> {
    Ok(WfcAdjacencyRule2d { id: rule.id, tile_a_id: rule.tile_a_id, tile_b_id: rule.tile_b_id, direction: direction_from_token(&rule.direction)?, allowed: rule.allowed })
}

pub fn pinned_to_dsl(cell: &WfcPinnedCell2d) -> WfcPinnedCell2dDsl {
    WfcPinnedCell2dDsl { x: cell.x, y: cell.y, tile_id: cell.tile_id.clone() }
}

pub fn pinned_from_dsl(cell: WfcPinnedCell2dDsl) -> WfcPinnedCell2d {
    WfcPinnedCell2d { x: cell.x, y: cell.y, tile_id: cell.tile_id }
}

pub fn cell_to_dsl(cell: &WfcCell2d) -> WfcCell2dDsl {
    WfcCell2dDsl { x: cell.x, y: cell.y }
}

pub fn cell_from_dsl(cell: &WfcCell2dDsl) -> WfcCell2d {
    WfcCell2d { x: cell.x, y: cell.y }
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(id = "wfc.grid2d", layout = "lines")]
struct Grid2dSnapshotDsl {
    schema: String,
    seed: u64,
    width: u32,
    height: u32,
    cell_width: f64,
    cell_height: f64,
    periodic_x: bool,
    periodic_y: bool,
    #[dsl(table)]
    tiles: Vec<WfcTile2dDsl>,
    #[dsl(table)]
    rules: Vec<WfcAdjacencyRule2dDsl>,
    #[dsl(table)]
    pinned: Vec<WfcPinnedCell2dDsl>,
    #[dsl(table)]
    masked: Vec<WfcCell2dDsl>,
}

impl Default for Grid2dSnapshotDsl {
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
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for Grid2dSnapshotDsl {
    const EXTENSION: &'static str = "wfcgrid2d";
    fn envelope_id() -> &'static str {
        "wfc.grid2d"
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

impl store::ArtifactPack for Grid2dSnapshotDsl {
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

fn grid2d_document_to_dsl(document: &Grid2dSnapshot) -> Grid2dSnapshotDsl {
    Grid2dSnapshotDsl {
        schema: document.schema.clone(),
        seed: document.seed,
        width: document.width,
        height: document.height,
        cell_width: document.cell_width,
        cell_height: document.cell_height,
        periodic_x: document.periodic_x,
        periodic_y: document.periodic_y,
        tiles: document.tiles.iter().map(tile_to_dsl).collect(),
        rules: document.rules.iter().map(rule_to_dsl).collect(),
        pinned: document.pinned.iter().map(pinned_to_dsl).collect(),
        masked: document.masked.iter().map(cell_to_dsl).collect(),
    }
}

fn grid2d_document_from_dsl(parsed: Grid2dSnapshotDsl) -> Result<Grid2dSnapshot, store::TextError> {
    Ok(Grid2dSnapshot {
        schema: parsed.schema,
        seed: parsed.seed,
        width: parsed.width,
        height: parsed.height,
        cell_width: parsed.cell_width,
        cell_height: parsed.cell_height,
        periodic_x: parsed.periodic_x,
        periodic_y: parsed.periodic_y,
        tiles: parsed.tiles.into_iter().map(tile_from_dsl).collect::<Result<Vec<_>, _>>()?,
        rules: parsed.rules.into_iter().map(rule_from_dsl).collect::<Result<Vec<_>, _>>()?,
        pinned: parsed.pinned.into_iter().map(pinned_from_dsl).collect(),
        masked: parsed.masked.iter().map(cell_from_dsl).collect(),
    })
}

impl store::ArtifactDsl for Grid2dSnapshot {
    const EXTENSION: &'static str = "wfcgrid2d";
    fn envelope_id() -> &'static str {
        "wfc.grid2d"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        grid2d_document_from_dsl(<Grid2dSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?)
    }

    fn print_dsl(&self) -> String {
        <Grid2dSnapshotDsl as store::ArtifactDsl>::print_dsl(&grid2d_document_to_dsl(self))
    }
}

impl store::ArtifactPack for Grid2dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Grid2dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&grid2d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Grid2dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        grid2d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 📖️ Parses `.wfcgrid2d` DSL text into a `Grid2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Grid2dSnapshot, store::TextError> {
    <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Grid2dSnapshot` back to `.wfcgrid2d` DSL text.
pub fn print_dsl(document: &Grid2dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Grid2dSnapshotText = String;
//#endregion 🚚️Carrier
