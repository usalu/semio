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
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::present::schema::diff::PresentDiff;
    use crate::artifacts::present::schema::mutations::PresentMutation;
    use crate::artifacts::present::schema::snapshot::PresentSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct PresentBuilderConstruction {
        snapshot: PresentSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PresentBuilderConstruction {
        type Snapshot = PresentSnapshot;
        type Mutation = PresentMutation;
        type Diff = PresentDiff;
        fn empty() -> Self { Self { snapshot: PresentSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PresentSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PresentSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <PresentDiff as protocol::MutationDiff<PresentSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::present::PresentSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct PresentParts {
        pub snapshot: Option<PresentSnapshot>,
    }

    pub struct PresentAnalyzerAnalysis;

    impl ArtifactAnalysis for PresentAnalyzerAnalysis {
        type Parts = PresentParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.present", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PresentParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PresentSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PresentSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PresentBuilderFacets {
        construction: derived_construction::PresentBuilderConstruction,
        analysis: derived_analysis::PresentAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PresentComposerComposition,
    }
    builder: PresentBuilder,
    analyzer: PresentAnalyzer,
    composer: PresentComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️Error
/// 🎞️ Errors from present deck envelope materialization (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: split from the former engine-tree
/// `PresentError`, which mixed this schema-tier envelope-replay concern with app-tier video-export
/// concerns — an artifact must never depend on an app, so the video-only variants
/// (`NoSceneHashes`/`Compile`) moved to `crate::apps::present::engine`'s own `PresentVideoExportError`
/// instead of being kept here).
#[derive(Debug, thiserror::Error)]
pub enum PresentError {
    /// 🧾️ The stored envelope JSON was malformed.
    #[error("deserialize envelope: {0}")]
    DeserializeEnvelope(#[from] serde_json::Error),
    /// 📐️ VCS replay failed while materializing the projection.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
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
            cells.push(SplitGridCell { key: format!("{}-r{row}-c{column}", spec.key_prefix), crop: crate::artifacts::present::FigureTileFrame { x: frame.x + column as f64 * crop_width, y: frame.y + row as f64 * crop_height, width: crop_width, height: crop_height } });
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
