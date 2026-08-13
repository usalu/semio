//! 🧬️ Layout artifact schema — every field of the artifact with its state class.

use crate::artifacts::layout::{
    CharacterStyle, GridSettings, ImageLink, LayoutDropPreviewState, LayoutDrawingChild, Page, ParagraphStyle, ParentPage, Spread,
    TextStory, LAYOUT_DOCUMENT_SCHEMA,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full layout artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub grid: GridSettings,
    #[state(artifact)]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[state(artifact)]
    pub character_styles: Vec<CharacterStyle>,
    #[state(artifact)]
    pub stories: Vec<TextStory>,
    #[state(artifact)]
    pub links: Vec<ImageLink>,
    #[state(artifact)]
    pub parent_pages: Vec<ParentPage>,
    #[state(artifact)]
    pub spreads: Vec<Spread>,
    #[state(artifact)]
    pub pages: Vec<Page>,
    #[state(artifact)]
    pub print_target: Option<String>,
    #[state(artifact)]
    pub data_fields_json: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.drawing")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_drawing: Option<LayoutDrawingChild>,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
    #[state(presence)]
    pub selected_ids: Vec<String>,
    #[state(config)]
    pub active_page_id: String,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera_x: f64,
    #[state(config)]
    pub camera_y: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(config)]
    pub preview_camera_x: f64,
    #[state(config)]
    pub preview_camera_y: f64,
    #[state(config)]
    pub preview_camera_zoom: f64,
    #[state(config)]
    pub drop_preview: LayoutDropPreviewState,
    #[state(config)]
    pub locale: String,
    #[state(artifact)]
    pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LayoutArtifact {
    fn default() -> Self {
        Self {
            schema: LAYOUT_DOCUMENT_SCHEMA.into(),
            name: String::new(),
            grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
            data_fields_json: None,
            background_drawing: None,
            referenced_model: None,
            selected_ids: Vec::new(),
            active_page_id: "page-1".into(),
            engagement_input: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            preview_camera_x: 0.0,
            preview_camera_y: 0.0,
            preview_camera_zoom: 1.0,
            drop_preview: LayoutDropPreviewState::default(),
            locale: "en-US".into(),
            hovered_id: None,
        }
    }
}

impl LayoutArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::layout::LayoutSnapshot {
        crate::artifacts::layout::LayoutSnapshot {
            schema: self.schema.clone(),
            name: self.name.clone(),
            grid: self.grid.clone(),
            paragraph_styles: self.paragraph_styles.clone(),
            character_styles: self.character_styles.clone(),
            stories: self.stories.clone(),
            links: self.links.clone(),
            parent_pages: self.parent_pages.clone(),
            spreads: self.spreads.clone(),
            pages: self.pages.clone(),
            print_target: self.print_target.clone(),
            data_fields_json: self.data_fields_json.clone(),
            background_drawing: self.background_drawing.clone(),
            referenced_model: self.referenced_model.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::layout::LayoutSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            name: snapshot.name,
            grid: snapshot.grid,
            paragraph_styles: snapshot.paragraph_styles,
            character_styles: snapshot.character_styles,
            stories: snapshot.stories,
            links: snapshot.links,
            parent_pages: snapshot.parent_pages,
            spreads: snapshot.spreads,
            pages: snapshot.pages,
            print_target: snapshot.print_target,
            data_fields_json: snapshot.data_fields_json,
            background_drawing: snapshot.background_drawing,
            referenced_model: snapshot.referenced_model,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::layout::LayoutSnapshot) {
        self.schema = snapshot.schema;
        self.name = snapshot.name;
        self.grid = snapshot.grid;
        self.paragraph_styles = snapshot.paragraph_styles;
        self.character_styles = snapshot.character_styles;
        self.stories = snapshot.stories;
        self.links = snapshot.links;
        self.parent_pages = snapshot.parent_pages;
        self.spreads = snapshot.spreads;
        self.pages = snapshot.pages;
        self.print_target = snapshot.print_target;
        self.data_fields_json = snapshot.data_fields_json;
        self.background_drawing = snapshot.background_drawing;
        self.referenced_model = snapshot.referenced_model;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.layout.layout` — twenty handcrafted schema leaves.
pub fn layout_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.layout.layout",
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
    use crate::artifacts::layout::{LayoutDiff, LayoutMutation, LayoutSnapshot};

    #[derive(Clone, Debug)]
    pub struct LayoutBuilderConstruction {
        snapshot: LayoutSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for LayoutBuilderConstruction {
        type Snapshot = LayoutSnapshot;
        type Mutation = LayoutMutation;
        type Diff = LayoutDiff;
        fn empty() -> Self {
            Self {
                snapshot: crate::artifacts::layout::schema::default_document(),
                diagnostics: Vec::new(),
            }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<LayoutSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<LayoutSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <LayoutDiff as protocol::MutationDiff<LayoutSnapshot>>::apply(&diff, &self.snapshot);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <LayoutDiff as protocol::MutationDiff<LayoutSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::layout::LayoutSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct LayoutParts {
        pub snapshot: Option<LayoutSnapshot>,
    }

    pub struct LayoutAnalyzerAnalysis;

    impl ArtifactAnalysis for LayoutAnalyzerAnalysis {
        type Parts = LayoutParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.layout", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = LayoutParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <LayoutSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 📄️Document
/// 📄️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — pure over `LayoutSnapshot`/`Page`, no engine state, no app type.
pub fn parse_layout_document(json: &str) -> Result<crate::artifacts::layout::LayoutSnapshot, crate::artifacts::layout::io::LayoutError> {
    let doc: crate::artifacts::layout::LayoutSnapshot = serde_json::from_str(json)?;
    if doc.schema != LAYOUT_DOCUMENT_SCHEMA {
        return Err(crate::artifacts::layout::io::LayoutError::UnexpectedSchema(doc.schema));
    }
    Ok(doc)
}

pub struct ResolvedFrame {
    pub frame: crate::artifacts::layout::Frame,
    pub inherited: bool,
}

pub fn resolve_page<'a>(doc: &'a crate::artifacts::layout::LayoutSnapshot, page: &'a Page) -> Vec<ResolvedFrame> {
    let mut frames = Vec::new();
    if let Some(parent_id) = &page.parent_page_id {
        if let Some(parent) = doc.parent_pages.iter().find(|p| p.id == *parent_id) {
            for frame in &parent.frames {
                let overridden = page.overrides.iter().any(|o| o.object_id == frame.id());
                frames.push(ResolvedFrame { frame: frame.clone(), inherited: !overridden });
            }
        }
    }
    for frame in &page.frames {
        frames.push(ResolvedFrame { frame: frame.clone(), inherited: false });
    }
    frames
}
//#endregion 📄️Document

//#region 🔖️DocumentHelpers
/// 📄️ The bundled sample fixture, parsed once — the source of truth for `LayoutPlayApp::initial_snapshot`
/// and the app manifest's `.example(...)` document. Relocated from the deleted `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub fn default_document() -> crate::artifacts::layout::LayoutSnapshot {
    build_demo_layout_snapshot()
}

fn build_demo_layout_snapshot() -> crate::artifacts::layout::LayoutSnapshot {
    crate::artifacts::layout::LayoutSnapshot {
        schema: LAYOUT_DOCUMENT_SCHEMA.into(),
        name: "Demo".into(),
        grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: true },
        paragraph_styles: vec![ParagraphStyle {
            id: "paragraph.body".into(),
            name: "Body".into(),
            font_family: "Layout Sans".into(),
            font_size: 12.0,
            font_weight: 400,
            leading: 14.4,
            tracking: 0.0,
            alignment: "left".into(),
        }],
        character_styles: Vec::new(),
        stories: vec![TextStory {
            id: "story-1".into(),
            content: "Hello layout".into(),
            style_runs: Vec::new(),
        }],
        links: vec![ImageLink {
            id: "link-missing".into(),
            path: "assets/missing.png".into(),
            hash: "sha256:missing".into(),
            width: 100,
            height: 100,
            dpi: 300,
            color_profile: None,
            state: Some("missing".into()),
            proxy_data_url: None,
        }],
        parent_pages: vec![ParentPage {
            id: "parent-1".into(),
            name: "Master".into(),
            width: 400.0,
            height: 500.0,
            layer_ids: vec!["layer-parent".into()],
            layers: vec![crate::artifacts::layout::Layer {
                id: "layer-parent".into(),
                name: "Master".into(),
                visible: true,
                locked: false,
                object_ids: vec!["frame-inherited".into()],
            }],
            frames: vec![crate::artifacts::layout::Frame::Rect {
                id: "frame-inherited".into(),
                layer_id: "layer-parent".into(),
                bounds: crate::artifacts::layout::LayoutBounds { x: 50.0, y: 50.0, width: 100.0, height: 80.0, rotation: 0.0 },
                locked: None,
                visible: None,
                fill: None,
                stroke: Some([0.4, 0.5, 0.7, 0.8]),
            }],
        }],
        spreads: vec![Spread { id: "spread-1".into(), name: "Spread 1".into(), page_ids: vec!["page-1".into(), "page-2".into()] }],
        pages: vec![
            Page {
                id: "page-1".into(),
                name: "Page 1".into(),
                spread_id: "spread-1".into(),
                parent_page_id: Some("parent-1".into()),
                width: 400.0,
                height: 500.0,
                margins: crate::artifacts::layout::PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                columns: crate::artifacts::layout::PageColumns { count: 1, gutter: 0.0 },
                guides: Vec::new(),
                layer_ids: vec!["layer-1".into()],
                layers: vec![crate::artifacts::layout::Layer {
                    id: "layer-1".into(),
                    name: "Content".into(),
                    visible: true,
                    locked: false,
                    object_ids: vec!["frame-text-1".into(), "frame-image-1".into(), "frame-1".into()],
                }],
                frames: vec![
                    crate::artifacts::layout::Frame::Text {
                        id: "frame-text-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: crate::artifacts::layout::LayoutBounds { x: 156.0, y: 220.0, width: 80.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        story_id: "story-1".into(),
                        thread_next: None,
                        columns: 1,
                        inset: crate::artifacts::layout::LayoutRect { x: 0.0, y: 0.0, width: 80.0, height: 40.0 },
                        wrap_mode: "box".into(),
                    },
                    crate::artifacts::layout::Frame::Image {
                        id: "frame-image-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: crate::artifacts::layout::LayoutBounds { x: 136.0, y: 435.0, width: 60.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        link_id: "link-missing".into(),
                    },
                    crate::artifacts::layout::Frame::Rect {
                        id: "frame-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: crate::artifacts::layout::LayoutBounds { x: 10.0, y: 10.0, width: 40.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        fill: Some([1.0, 1.0, 1.0, 1.0]),
                        stroke: None,
                    },
                ],
                overrides: Vec::new(),
            },
            Page {
                id: "page-2".into(),
                name: "Page 2".into(),
                spread_id: "spread-1".into(),
                parent_page_id: None,
                width: 400.0,
                height: 500.0,
                margins: crate::artifacts::layout::PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                columns: crate::artifacts::layout::PageColumns { count: 1, gutter: 0.0 },
                guides: Vec::new(),
                layer_ids: Vec::new(),
                layers: Vec::new(),
                frames: Vec::new(),
                overrides: Vec::new(),
            },
        ],
        print_target: None,
        data_fields_json: None,
        background_drawing: None,
        referenced_model: None,
    }
}

/// 🌉️ JSON bridge for `semio_framework_plugin::App::example`, which hardcodes `serde_json::from_str`
/// on its `document_json` parameter (shared framework machinery, out of scope for this DSL migration) —
/// derives the JSON from the DSL fixture rather than keeping a second, redundant JSON copy of it on disk.
pub fn layout_sample_document_json() -> String {
    serde_json::to_string(&default_document()).unwrap_or_default()
}

/// 🎨️ Formats an optional RGBA color as a comma-separated text field value; two consumers
/// (`📌️panels/🔍️inspection` reads it, `🎮️commands/✏️author` parses it back via `text_to_rgba`).
pub fn rgba_to_text(color: &Option<[f32; 4]>) -> String {
    color.map(|channels| channels.iter().map(|channel| channel.to_string()).collect::<Vec<_>>().join(", ")).unwrap_or_default()
}

/// 🎨️ Parses a comma-separated `r, g, b, a` text field value back into an RGBA color, or `None` if it
/// does not have exactly four numeric components.
pub fn text_to_rgba(text: &str) -> Option<[f32; 4]> {
    let parts: Vec<f32> = text.split(',').filter_map(|part| part.trim().parse::<f32>().ok()).collect();
    (parts.len() == 4).then(|| [parts[0], parts[1], parts[2], parts[3]])
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️DocumentTests
#[cfg(test)]
mod document_tests {
    use super::*;

    fn rect_frame(id: &str, visible: Option<bool>) -> crate::artifacts::layout::Frame {
        crate::artifacts::layout::Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: crate::artifacts::layout::LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: None, visible, fill: None, stroke: None }
    }

    fn base_doc() -> crate::artifacts::layout::LayoutSnapshot {
        crate::artifacts::layout::LayoutSnapshot {
            schema: LAYOUT_DOCUMENT_SCHEMA.into(),
            name: "t".into(),
            grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
            data_fields_json: None,
            background_drawing: None,
            referenced_model: None,
        }
    }

    #[test]
    fn resolve_page_marks_overridden_parent_frames_and_ignores_missing_parent() {
        let mut doc = base_doc();
        doc.parent_pages.push(crate::artifacts::layout::ParentPage {
            id: "parent-1".into(),
            name: "Master".into(),
            width: 100.0,
            height: 100.0,
            layer_ids: vec!["layer-1".into()],
            layers: Vec::new(),
            frames: vec![rect_frame("frame-a", None), rect_frame("frame-b", None)],
        });

        let page_with_parent = Page {
            id: "page-1".into(),
            name: "P1".into(),
            spread_id: "spread-1".into(),
            parent_page_id: Some("parent-1".into()),
            width: 100.0,
            height: 100.0,
            margins: crate::artifacts::layout::PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
            columns: crate::artifacts::layout::PageColumns { count: 1, gutter: 0.0 },
            guides: Vec::new(),
            layer_ids: Vec::new(),
            layers: Vec::new(),
            frames: Vec::new(),
            overrides: vec![crate::artifacts::layout::PageOverride { object_id: "frame-a".into(), bounds: None, visible: None, locked: None }],
        };
        let resolved = resolve_page(&doc, &page_with_parent);
        assert_eq!(resolved.len(), 2);
        let a = resolved.iter().find(|r| r.frame.id() == "frame-a").expect("frame-a resolved");
        assert!(!a.inherited, "overridden parent frame must not be marked inherited");
        let b = resolved.iter().find(|r| r.frame.id() == "frame-b").expect("frame-b resolved");
        assert!(b.inherited, "non-overridden parent frame stays inherited");

        let mut page_missing_parent = page_with_parent.clone();
        page_missing_parent.parent_page_id = Some("no-such-parent".into());
        assert!(resolve_page(&doc, &page_missing_parent).is_empty());

        let mut page_no_parent = page_with_parent;
        page_no_parent.parent_page_id = None;
        page_no_parent.frames = vec![rect_frame("frame-own", None)];
        let own_only = resolve_page(&doc, &page_no_parent);
        assert_eq!(own_only.len(), 1);
        assert!(!own_only[0].inherited);
    }

    #[test]
    fn parse_layout_document_rejects_wrong_schema_and_invalid_json() {
        let wrong_schema = r#"{"schema":"other.schema","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[]}"#;
        let error = parse_layout_document(wrong_schema).expect_err("wrong schema must fail");
        assert!(matches!(error, crate::artifacts::layout::io::LayoutError::UnexpectedSchema(schema) if schema == "other.schema"));

        let invalid_json = "not json";
        let error = parse_layout_document(invalid_json).expect_err("invalid json must fail");
        assert!(matches!(error, crate::artifacts::layout::io::LayoutError::Json(_)));
    }

    #[test]
    fn rgba_text_round_trips() {
        assert_eq!(rgba_to_text(&Some([0.1, 0.2, 0.3, 1.0])), "0.1, 0.2, 0.3, 1");
        assert_eq!(rgba_to_text(&None), "");
        assert_eq!(text_to_rgba("0.5, 0.4, 0.3, 1"), Some([0.5, 0.4, 0.3, 1.0]));
        assert_eq!(text_to_rgba("not, a, color"), None);
    }
}
//#endregion 🧪️DocumentTests

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec LayoutBuilderFacets {
        construction: derived_construction::LayoutBuilderConstruction,
        analysis: derived_analysis::LayoutAnalyzerAnalysis,
        composition: super::super::io::derived_composition::LayoutComposerComposition,
    }
    builder: LayoutBuilder,
    analyzer: LayoutAnalyzer,
    composer: LayoutComposer,
);
//#endregion 🧬️DerivedArtifactFacets
