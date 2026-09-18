//! 🧱️ `Grid3dSnapshot` — the persisted WFC problem for a regular, NON-UNIFORM box grid: the grid
//! extent (`width`/`height`/`depth`) with one explicit size per column, row and layer
//! (`cellSizesX`/`cellSizesY`/`cellSizesZ`), the per-axis periodicity flags, the TILE catalogue
//! (mesh media, inline or as a composed `s.stdio.semio@v1/mesh` child), the directed six-neighbour
//! ADJACENCY RULES between tiles, the user PINS and the MASKED cells.
//!
//! 🚫️ The SOLVE is never stored here: it is an inference (`../💡️inferences/🦀️.rs`) over this spec,
//! exactly as the sibling wfc artifacts state it — only the PROBLEM is authored, the SOLUTION is
//! derived and republished into the preview window, never mutation-authored persisted state.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Ids
pub const WFC_GRID3D_DOCUMENT_SCHEMA: &str = "s.wfc.grid3d";
//#endregion 🔖️Ids

//#region 🔖️Color
/// 🎨️ Straight-alpha RGBA, every channel `0..=255`. Values outside that range are clamped by every
/// reader rather than refused, so a hand-authored document never fails to render.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Grid3dColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}
//#endregion 🔖️Color

//#region 🔖️Media
/// 🔺️ Inline triangle geometry in the TILE-SPACE unit box `0..1` on every axis — the renderer scales
/// it into whatever box the cell it lands in actually occupies, so one tile mesh serves every cell
/// size on a non-uniform grid. `positions` is xyz triples, `indices` is triangle corners.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Grid3dMesh {
    #[value(default)]
    pub positions: Vec<f64>,
    #[value(default)]
    pub indices: Vec<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Grid3dColor>,
}

/// 🗿️ A tile's 3D media: either geometry authored INLINE in this document, or a handle to a composed
/// `s.stdio.semio@v1/mesh` child the host hydrates. A handle whose child is not hydrated renders as
/// a unit-box placeholder rather than failing the surface (the raster/remodel "fail soft" rule).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum Grid3dTileMedia {
    Mesh { mesh: Grid3dMesh },
    MeshChild { child: store::ArtifactChild<SemioMeshSnapshot> },
}

impl Default for Grid3dTileMedia {
    fn default() -> Self {
        Grid3dTileMedia::Mesh { mesh: Grid3dMesh::default() }
    }
}

/// 🌉️ Hand `dsl::DslField` impl — `Grid3dTileMedia` is a `DslEnum` (`DslVariants` only) and
/// `Grid3dTile::media` is a REQUIRED, never-optional field that must stay a bare `Grid3dTileMedia`
/// (`s.process.process3d`'s `MeasureRecipe` precedent).
impl dsl::DslField for Grid3dTileMedia {
    fn shape() -> dsl::Shape {
        dsl::Shape::Statements(<Grid3dTileMedia as dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Statements(vec![<Grid3dTileMedia as dsl::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Statements(items) if items.len() == 1 => <Grid3dTileMedia as dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|error| error.message),
            other => Err(format!("expected exactly 1 tagged tile media value, found {other:?}")),
        }
    }
}

/// 🗿️ One placeable tile — the WFC pattern universe of this document. `weight` is the selection
/// bias the solver samples with; every mutation refuses a non-positive or non-finite weight.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Grid3dTile {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub weight: f64,
    #[dsl(statements, block)]
    pub media: Grid3dTileMedia,
}
//#endregion 🔖️Media

//#region 🔖️Rule
/// 🧭️ Which of the six face neighbours a rule speaks about: `B lies in <direction> of A`.
/// `LEFT`/`RIGHT` is −x/+x, `FRONT`/`BACK` is −y/+y, `BOTTOM`/`TOP` is −z/+z.
///
/// 🔤️ The tokens are spelled PER VARIANT, not through `rename_all`: `semio_framework_value_derive`
/// accepts only `camelCase`/`kebab-case`/`lowercase`/`snake_case` there and silently ignores any
/// other spelling, which would leave the wire on `Left`/`Right` while every schema leaf declares
/// `LEFT`/`RIGHT` (measured on the sibling `s.wfc.grid2d`, ticket 26/09/18/EXTRACT-WFC-PLUGIN).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ToValue, FromValue, dsl::DslScalar)]
pub enum Grid3dDirection {
    #[value(rename = "LEFT")]
    Left,
    #[default]
    #[value(rename = "RIGHT")]
    Right,
    #[value(rename = "FRONT")]
    Front,
    #[value(rename = "BACK")]
    Back,
    #[value(rename = "BOTTOM")]
    Bottom,
    #[value(rename = "TOP")]
    Top,
}

impl Grid3dDirection {
    /// 🧭️ The six directions in the order the stencil relations are declared in.
    pub const ALL: [Grid3dDirection; 6] = [Grid3dDirection::Right, Grid3dDirection::Left, Grid3dDirection::Back, Grid3dDirection::Front, Grid3dDirection::Top, Grid3dDirection::Bottom];

    /// 🧭️ Index into `Stencil3d::Face6`'s own offset order `[+x, −x, +y, −y, +z, −z]`.
    pub fn stencil_index(self) -> usize {
        match self {
            Grid3dDirection::Right => 0,
            Grid3dDirection::Left => 1,
            Grid3dDirection::Back => 2,
            Grid3dDirection::Front => 3,
            Grid3dDirection::Top => 4,
            Grid3dDirection::Bottom => 5,
        }
    }

    pub fn opposite(self) -> Grid3dDirection {
        match self {
            Grid3dDirection::Left => Grid3dDirection::Right,
            Grid3dDirection::Right => Grid3dDirection::Left,
            Grid3dDirection::Front => Grid3dDirection::Back,
            Grid3dDirection::Back => Grid3dDirection::Front,
            Grid3dDirection::Bottom => Grid3dDirection::Top,
            Grid3dDirection::Top => Grid3dDirection::Bottom,
        }
    }

    /// 🔤️ The wire token this direction carries — the ONE spelling every schema leaf, fixture and
    /// oracle declares.
    pub fn label(self) -> &'static str {
        match self {
            Grid3dDirection::Left => "LEFT",
            Grid3dDirection::Right => "RIGHT",
            Grid3dDirection::Front => "FRONT",
            Grid3dDirection::Back => "BACK",
            Grid3dDirection::Bottom => "BOTTOM",
            Grid3dDirection::Top => "TOP",
        }
    }
}

/// 📏️ Which per-axis size array a `change-cell-sizes` mutation addresses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum Grid3dAxis {
    #[default]
    X,
    Y,
    Z,
}

impl Grid3dAxis {
    /// 📏️ The extent this axis' size array must match.
    pub fn extent(self, snapshot: &Grid3dSnapshot) -> u32 {
        match self {
            Grid3dAxis::X => snapshot.width,
            Grid3dAxis::Y => snapshot.height,
            Grid3dAxis::Z => snapshot.depth,
        }
    }

    /// 📏️ This axis' current size array.
    pub fn sizes(self, snapshot: &Grid3dSnapshot) -> &Vec<f64> {
        match self {
            Grid3dAxis::X => &snapshot.cell_sizes_x,
            Grid3dAxis::Y => &snapshot.cell_sizes_y,
            Grid3dAxis::Z => &snapshot.cell_sizes_z,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Grid3dAxis::X => "x",
            Grid3dAxis::Y => "y",
            Grid3dAxis::Z => "z",
        }
    }
}

/// ⛓️ One directed adjacency rule. `allowed = true` admits `B` in `direction` of `A` (and, by the
/// stencil's declared inverse, `A` in the opposite direction of `B`); `allowed = false` DENIES the
/// pair outright, and a deny always wins over any allow. A tile pair no rule mentions for a
/// direction is NOT allowed — the rule set is a closed allow-list, never a deny-list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Grid3dRule {
    pub id: String,
    pub tile_a_id: String,
    pub tile_b_id: String,
    pub direction: Grid3dDirection,
    pub allowed: bool,
}
//#endregion 🔖️Rule

//#region 🔖️Cells
/// 📌️ A hard pre-assignment the solver must respect — a domain restriction feeding the solve, never
/// overwritten by it.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Grid3dPinnedCell {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub tile_id: String,
}

/// 🚫️ A cell excluded from the topology entirely — it gets no arcs and is never assigned, which is
/// how a non-box shape is carved out of the regular grid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Grid3dCell {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
//#endregion 🔖️Cells

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "wfc.grid3d", layout = "lines")]
#[artifact_schema(id = "s.wfc.grid3d")]
pub struct Grid3dSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🎲️ Deterministic solve seed — PERSISTED, authored only via `change-seed`, never ambient: the
    /// solve inference's `DepHash` caching is sound only when `compute` is a pure function of
    /// snapshot content, seed included.
    #[state(artifact)]
    pub seed: u64,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    pub depth: u32,
    /// 📏️ One size per column/row/layer — `cellSizesX.len() == width` and so on is an invariant every
    /// mutation upholds, so cell `n`'s world extent is the cumulative sum of the sizes before it.
    #[state(artifact)]
    #[value(default)]
    pub cell_sizes_x: Vec<f64>,
    #[state(artifact)]
    #[value(default)]
    pub cell_sizes_y: Vec<f64>,
    #[state(artifact)]
    #[value(default)]
    pub cell_sizes_z: Vec<f64>,
    #[state(artifact)]
    pub periodic_x: bool,
    #[state(artifact)]
    pub periodic_y: bool,
    #[state(artifact)]
    pub periodic_z: bool,
    #[state(artifact)]
    #[value(default)]
    #[dsl(table)]
    pub tiles: Vec<Grid3dTile>,
    #[state(artifact)]
    #[value(default)]
    #[dsl(table)]
    pub rules: Vec<Grid3dRule>,
    #[state(artifact)]
    #[value(default)]
    #[dsl(table)]
    pub pinned: Vec<Grid3dPinnedCell>,
    #[state(artifact)]
    #[value(default)]
    #[dsl(table)]
    pub masked: Vec<Grid3dCell>,
}

impl Default for Grid3dSnapshot {
    fn default() -> Self {
        Self {
            schema: WFC_GRID3D_DOCUMENT_SCHEMA.into(),
            seed: 0,
            width: 1,
            height: 1,
            depth: 1,
            cell_sizes_x: vec![1.0],
            cell_sizes_y: vec![1.0],
            cell_sizes_z: vec![1.0],
            periodic_x: false,
            periodic_y: false,
            periodic_z: false,
            tiles: Vec::new(),
            rules: Vec::new(),
            pinned: Vec::new(),
            masked: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` — the derive stopped emitting these traits, so every
/// artifact states its own envelope discipline.
impl store::ArtifactDsl for Grid3dSnapshot {
    const EXTENSION: &'static str = "wfcgrid3d";
    fn envelope_id() -> &'static str {
        "wfc.grid3d"
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

impl store::ArtifactPack for Grid3dSnapshot {
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
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Addressing
/// 📐️ The canonical sort key of one pinned/masked cell — the ordering every collection mutation
/// inserts at, so a delete and its inverse create round-trip at the SAME index.
pub fn cell_key(x: u32, y: u32, z: u32) -> String {
    format!("{x}:{y}:{z}")
}

pub fn tile_index(snapshot: &Grid3dSnapshot, id: &str) -> Option<usize> {
    snapshot.tiles.iter().position(|tile| tile.id == id)
}

pub fn rule_index(snapshot: &Grid3dSnapshot, id: &str) -> Option<usize> {
    snapshot.rules.iter().position(|rule| rule.id == id)
}

pub fn pinned_index(snapshot: &Grid3dSnapshot, x: u32, y: u32, z: u32) -> Option<usize> {
    snapshot.pinned.iter().position(|cell| (cell.x, cell.y, cell.z) == (x, y, z))
}

pub fn masked_index(snapshot: &Grid3dSnapshot, x: u32, y: u32, z: u32) -> Option<usize> {
    snapshot.masked.iter().position(|cell| (cell.x, cell.y, cell.z) == (x, y, z))
}

/// 📐️ Cell `index`'s lower world coordinate on one axis — the cumulative sum of every size before
/// it, which is what makes the grid non-uniform without any framework-level grid helper.
pub fn axis_offset(sizes: &[f64], index: usize) -> f64 {
    sizes.iter().take(index).sum()
}

/// 📐️ Cell `index`'s own size on one axis, defaulting to `1.0` for an index the array does not
/// reach (a document authored short).
pub fn axis_size(sizes: &[f64], index: usize) -> f64 {
    sizes.get(index).copied().filter(|size| size.is_finite() && *size > 0.0).unwrap_or(1.0)
}

/// 📐️ Whether a cell coordinate is inside the declared grid extent.
pub fn cell_in_grid(snapshot: &Grid3dSnapshot, x: u32, y: u32, z: u32) -> bool {
    x < snapshot.width && y < snapshot.height && z < snapshot.depth
}

/// 📏️ One axis' size array resized to `extent`: truncated, or extended with the last authored size
/// (`1.0` when there is none). The one place `resize-grid` derives the new arrays from.
pub fn resized_axis(sizes: &[f64], extent: u32) -> Vec<f64> {
    let extent = extent as usize;
    let fill = sizes.last().copied().filter(|size| size.is_finite() && *size > 0.0).unwrap_or(1.0);
    let mut next: Vec<f64> = sizes.iter().take(extent).copied().collect();
    while next.len() < extent {
        next.push(fill);
    }
    next
}
//#endregion 🔖️Addressing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's text form speaks, named as the schema names the export.
pub type Grid3dSnapshotText = String;
//#endregion 🚚️Carrier
