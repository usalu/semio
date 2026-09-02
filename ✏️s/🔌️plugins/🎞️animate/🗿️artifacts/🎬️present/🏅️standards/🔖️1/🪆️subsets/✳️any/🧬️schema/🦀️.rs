//! 🧬️ Present artifact schema — every field of the artifact with its state class.

use crate::artifacts::present::{AnimationChild, PresentationChild, PRESENT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full present artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present")]
pub struct PresentArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.presentation")]
    pub presentation: PresentationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.animation")]
    pub animation: AnimationChild,
    #[state(presence)]
    pub selected_ids: Vec<String>,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PresentArtifact {
    fn default() -> Self {
        Self {
            schema: PRESENT_DOCUMENT_SCHEMA.into(),
            presentation: crate::artifacts::present::presentation_child_handle_and_cache(&crate::artifacts::present::default_figure_tile_source(), &[]),
            animation: crate::artifacts::present::animation_child_handle(),
            selected_ids: Vec::new(),
            engagement_input: String::new(),
            locale: "en-US".into(),
        }
    }
}

impl PresentArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::present::PresentSnapshot {
        crate::artifacts::present::PresentSnapshot { schema: self.schema.clone(), presentation: self.presentation.clone(), animation: self.animation.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::present::PresentSnapshot) -> Self {
        Self { schema: snapshot.schema, presentation: snapshot.presentation, animation: snapshot.animation, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::present::PresentSnapshot) {
        self.schema = snapshot.schema;
        self.presentation = snapshot.presentation;
        self.animation = snapshot.animation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.animate.present` — twenty handcrafted schema leaves.
pub fn present_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.animate.present",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️Construction
/// 🏗️ Replaces the deleted `derive_artifact_facets!`-generated `PresentBuilder`/`PresentAnalyzer`/
/// `PresentComposer` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §2) — the
/// generic construction/mutation-application path every trivial subset (no custom analysis/
/// composition logic beyond ordinary `Mutation`/`MutationDiff` algebra) now uses. Never referenced
/// by `ArtifactInferrer` (orphan-rule violation, see `🚪️io/💡️inferences/🦀️.rs`'s own
/// `PresentInferrer` marker) — kept only as the documented replacement anchor, mirroring
/// `🎬️sequence`'s identical shape.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<crate::artifacts::present::PresentSnapshot, crate::artifacts::present::PresentMutation>;
//#endregion 🏗️Construction

//#region 🔖️Error
/// 🎞️ Errors from present deck envelope materialization (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: split from the former engine-tree
/// `PresentError`, which mixed this schema-tier envelope-replay concern with app-tier video-export
/// concerns — an artifact must never depend on an app, so the video-only variants
/// (`NoSceneHashes`/`Compile`) moved to `crate::editor::animate::engine`'s own `PresentVideoExportError`
/// instead of being kept here).
#[derive(Debug)]
pub enum PresentError {
    /// 🧾️ The stored envelope JSON was malformed.
    DeserializeEnvelope(serde_json::Error),
    /// 🧬️ Whole-buffer envelope ingress was rejected in favor of the persistent fixed-page decoder.
    EnvelopeIngress(store::ArtifactEnvelopeWholeBufferIngressError),
    /// 📐️ VCS replay failed while materializing the projection.
    Vcs(vcs::VcsError),
}

impl std::fmt::Display for PresentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeserializeEnvelope(error) => write!(formatter, "deserialize envelope: {error}"),
            Self::EnvelopeIngress(error) => write!(formatter, "{error}"),
            Self::Vcs(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for PresentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DeserializeEnvelope(error) => Some(error),
            Self::EnvelopeIngress(error) => Some(error),
            Self::Vcs(error) => std::error::Error::source(error),
        }
    }
}

impl From<serde_json::Error> for PresentError {
    fn from(error: serde_json::Error) -> Self {
        Self::DeserializeEnvelope(error)
    }
}

impl From<store::ArtifactEnvelopeWholeBufferIngressError> for PresentError {
    fn from(error: store::ArtifactEnvelopeWholeBufferIngressError) -> Self {
        Self::EnvelopeIngress(error)
    }
}

impl From<vcs::VcsError> for PresentError {
    fn from(error: vcs::VcsError) -> Self {
        Self::Vcs(error)
    }
}
//#endregion 🔖️Error

//#region 🔖️DocumentHelpers
/// 📄️ Empty presentation deck — the wasm VCS bridge's default projection for a fresh envelope.
pub fn empty_present_snapshot() -> crate::artifacts::present::PresentSnapshot {
    crate::artifacts::present::present_snapshot_with_tiles(&crate::artifacts::present::default_figure_tile_source(), &[])
}

//#region 🎞️TilePlay
/// 🌱️ Relocated verbatim from the former artifact-tree `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every function here is a pure `(&Snapshot-
/// adjacent structs) -> value` transform with no `&mut self`, no app coupling, and no I/O — reachable
/// from BOTH schema-tier DSL/grammar hooks (`📸️snapshot/📝️text`, `🧬️mutations/📝️text`) and app-tier
/// commands, so it belongs at this shared schema tier rather than in either dependent.
pub const NORMALIZED_RECT_MIN_FRACTION: f64 = 0.02;

#[derive(Clone, Copy)]
pub struct SplitFigureGridSpec<'a> {
    pub rows: u32,
    pub columns: u32,
    pub frame: &'a crate::artifacts::present::FigureTileFrame,
    pub gap: f64,
    pub key_prefix: &'a str,
}

pub struct SplitGridCell {
    pub key: String,
    pub crop: crate::artifacts::present::FigureTileFrame,
}

#[derive(Clone, Copy)]
pub struct FigureTileGridSeedSpec<'a> {
    pub source: &'a crate::artifacts::present::FigureTileSource,
    pub rows: u32,
    pub columns: u32,
    pub gap: f64,
    pub key_prefix: &'a str,
}

pub fn clamp_normalized_fraction(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

pub fn clamp_tile_crop(crop: &crate::artifacts::present::FigureTileFrame) -> crate::artifacts::present::FigureTileFrame {
    let width = crop.width.max(NORMALIZED_RECT_MIN_FRACTION);
    let height = crop.height.max(NORMALIZED_RECT_MIN_FRACTION);
    let x = clamp_normalized_fraction(crop.x.min(1.0 - width));
    let y = clamp_normalized_fraction(crop.y.min(1.0 - height));
    crate::artifacts::present::FigureTileFrame { x, y, width, height }
}

pub fn parse_grid_engagement(text: &str) -> Option<(u32, u32)> {
    let trimmed = text.trim();
    let lower = trimmed.to_lowercase();
    let normalized = lower.replace('×', "x");
    let parts: Vec<&str> = normalized.split('x').map(str::trim).collect();
    if parts.len() != 2 {
        return None;
    }
    let rows: u32 = parts[0].parse().ok()?;
    let columns: u32 = parts[1].parse().ok()?;
    if rows < 1 || columns < 1 {
        return None;
    }
    Some((rows, columns))
}

pub fn split_figure_grid(spec: SplitFigureGridSpec<'_>) -> Vec<SplitGridCell> {
    let rows = spec.rows.max(1);
    let columns = spec.columns.max(1);
    let gap = spec.gap;
    let frame = spec.frame;
    let cell_width = (frame.width - gap * (columns as f64 - 1.0)) / columns as f64;
    let cell_height = (frame.height - gap * (rows as f64 - 1.0)) / rows as f64;
    let crop_width = frame.width / columns as f64;
    let crop_height = frame.height / rows as f64;
    let mut cells = Vec::new();
    for row in 0..rows {
        for column in 0..columns {
            cells.push(SplitGridCell {
                key: format!("{}-r{row}-c{column}", spec.key_prefix),
                crop: crate::artifacts::present::FigureTileFrame { x: frame.x + column as f64 * crop_width, y: frame.y + row as f64 * crop_height, width: crop_width, height: crop_height },
            });
        }
    }
    let _ = (cell_width, cell_height);
    cells
}

pub fn populate_tile_drafts_from_grid(spec: FigureTileGridSeedSpec<'_>) -> Vec<crate::artifacts::present::FigureTileDraft> {
    split_figure_grid(SplitFigureGridSpec { rows: spec.rows, columns: spec.columns, frame: &spec.source.frame, gap: spec.gap, key_prefix: spec.key_prefix })
        .into_iter()
        .map(|cell| crate::artifacts::present::FigureTileDraft { id: cell.key.clone(), name: cell.key, crop: cell.crop })
        .collect()
}

pub fn build_tile_morph_prompt(source: &crate::artifacts::present::FigureTileSource, drafts: &[crate::artifacts::present::FigureTileDraft]) -> String {
    fn format_frame(frame: &crate::artifacts::present::FigureTileFrame) -> String {
        format!("{{ x: {:.6}, y: {:.6}, width: {:.6}, height: {:.6} }}", frame.x, frame.y, frame.width, frame.height)
    }
    let kind = if source.kind.is_empty() { "figure" } else { source.kind.as_str() };
    let mut lines = vec![
        "Wire a one-to-many morph for animate present deck tiles using the parameters below.".into(),
        String::new(),
        "## Source media".into(),
        format!("- kind: {kind}"),
        format!("- src: {}", serde_json::to_string(&source.src).unwrap_or_else(|_| "\"\"".into())),
    ];
    if let Some(aspect) = source.source_aspect {
        lines.push(format!("- sourceAspect: {aspect}"));
    }
    if kind == "pdf" {
        if let Some(page) = source.pdf_page {
            lines.push(format!("- pdfPage: {page}"));
        }
    }
    lines.push(format!("- frame: {}", format_frame(&source.frame)));
    lines.push(String::new());
    lines.push("## Tiles (normalized source crops; overlap allowed)".into());
    for draft in drafts {
        lines.push(format!("- {} ({}): crop {}", draft.name, draft.id, format_frame(&draft.crop)));
    }
    let embodiment_hint = match kind {
        "video" => "Use video embodiments for tile participants and the source clip.",
        "pdf" => "Use pdf embodiments for tile participants and the source document page.",
        _ => "Register one participant per tile with a tile figure embodiment using each crop above.",
    };
    lines.push(String::new());
    lines.push("## Task".into());
    lines.push(format!("1. {embodiment_hint}"));
    lines.push("2. On the source slide, place the full media with morphTo slots pointing at each tile participant.".into());
    lines.push("3. Use reveal.js auto-animate; morph from the actual disposition including ephemeral modifications.".into());
    lines.join("\n")
}
//#endregion 🎞️TilePlay
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_seed_produces_tiles() {
        let source = crate::artifacts::present::default_figure_tile_source();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 3, columns: 5, gap: 0.0, key_prefix: "tile" });
        assert_eq!(tiles.len(), 15);
        assert_eq!(tiles[0].id, "tile-r0-c0");
    }

    #[test]
    fn parse_grid_engagement_accepts_cross() {
        assert_eq!(parse_grid_engagement("3×5"), Some((3, 5)));
        assert_eq!(parse_grid_engagement("2x2"), Some((2, 2)));
    }

    #[test]
    fn morph_prompt_lists_tiles() {
        let source = crate::artifacts::present::default_figure_tile_source();
        let tiles = vec![crate::artifacts::present::FigureTileDraft { id: "t1".into(), name: "t1".into(), crop: crate::artifacts::present::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } }];
        let prompt = build_tile_morph_prompt(&source, &tiles);
        assert!(prompt.contains("t1"));
        assert!(prompt.contains("Source media"));
    }
}
//#endregion 🧪️Tests
