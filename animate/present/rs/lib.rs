//! 🎞️ Animate present deck document + typed VCS on `vcs`.

pub mod present {
    //! 🎞️ Scene-based presentation documents and static site compiler.

    pub mod compiler {
        //! 🌐 Headless static-site compiler for animate present decks.

        use crate::PresentDeck;
        use animate_core::{AnimateConfig, QualityPreset};
        use animate_video::{render_scene, scene_for_hash, OutputFormat};
        use serde::{Deserialize, Serialize};
        use serde_json::json;
        use std::fs;
        use std::path::{Path, PathBuf};

        /// 🚨 Static-site compilation failure.
        #[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
        #[error("{message}")]
        pub struct PresentCompileError {
            pub message: String,
        }

        impl PresentCompileError {
            fn new(message: impl Into<String>) -> Self {
                Self { message: message.into() }
            }
        }

        pub type Result<T> = std::result::Result<T, PresentCompileError>;

        /// 📦 Rendered scene clip paths for present sites and plugin export.
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct SceneAssetBundle {
            pub scene_hash: String,
            pub mp4: Option<PathBuf>,
            pub last_frame: Option<PathBuf>,
            pub subtitles: Option<PathBuf>,
            pub sections: Option<PathBuf>,
        }

        /// 🎬 Renders one animate scene hash into `output_dir/scenes/{hash}`.
        pub fn compile_scene_to_assets(scene_hash: &str, output_dir: &Path) -> Result<SceneAssetBundle> {
            let scene_dir = output_dir.join("scenes").join(scene_hash);
            fs::create_dir_all(&scene_dir).map_err(|error| PresentCompileError::new(error.to_string()))?;
            let config = AnimateConfig::from_quality(QualityPreset::Medium).with_output_dir(&scene_dir).with_media_dir(scene_dir.join("media")).with_subtitles_path(scene_dir.join("scene.srt"));
            let scene = scene_for_hash(config.clone(), scene_hash);
            let outputs = render_scene(scene, config, &[OutputFormat::Mp4, OutputFormat::LastFrame]).map_err(|error| PresentCompileError::new(error.to_string()))?;
            Ok(SceneAssetBundle { scene_hash: scene_hash.into(), mp4: outputs.mp4, last_frame: outputs.last_frame, subtitles: Some(scene_dir.join("scene.srt")), sections: outputs.sections })
        }

        /// 📦 Writes `index.html`, `styles.css`, `manifest.json`, and embedded deck JSON for a wgpu-ready site.
        pub fn compile_present_site(deck: &PresentDeck, output_dir: &Path) -> Result<()> {
            fs::create_dir_all(output_dir).map_err(|error| PresentCompileError::new(error.to_string()))?;
            let deck_json = serde_json::to_string_pretty(deck).map_err(|error| PresentCompileError::new(format!("deck json: {error}")))?;
            fs::write(output_dir.join("deck.json"), &deck_json).map_err(|error| PresentCompileError::new(error.to_string()))?;
            fs::write(output_dir.join("index.html"), index_html(&deck_json)).map_err(|error| PresentCompileError::new(error.to_string()))?;
            fs::write(output_dir.join("styles.css"), styles_css()).map_err(|error| PresentCompileError::new(error.to_string()))?;
            fs::write(output_dir.join("manifest.json"), serde_json::to_string_pretty(&site_manifest(deck)).map_err(|error| PresentCompileError::new(error.to_string()))?).map_err(|error| PresentCompileError::new(error.to_string()))?;
            fs::write(output_dir.join("player.js"), player_boot_js()).map_err(|error| PresentCompileError::new(error.to_string()))?;
            Ok(())
        }

        fn site_manifest(deck: &PresentDeck) -> serde_json::Value {
            json!({
                "schema": "animate.present.site",
                "deckSchema": deck.schema,
                "title": deck.tiles.first().map_or("Animate Present", |tile| tile.name.as_str()),
                "tileCount": deck.tiles.len(),
                "player": {
                    "kind": "wgpu",
                    "wasm": "/animate/plugin/wasm/animate_plugin_bg.wasm",
                    "js": "/animate/plugin/wasm/animate_plugin.js",
                    "boot": "/animate/plugin/wasm/boot.js"
                },
                "assets": {
                    "deck": "deck.json",
                    "styles": "styles.css",
                    "player": "player.js",
                    "scenes": "scenes"
                }
            })
        }

        fn index_html(deck_json: &str) -> String {
            let escaped = deck_json.replace('&', "&amp;").replace('<', "&lt;");
            format!(
                r#"<!DOCTYPE html>
        <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <title>Animate Present</title>
          <link rel="stylesheet" href="styles.css" />
          <link rel="manifest" href="manifest.json" />
        </head>
        <body>
          <main id="animate-present-root" data-deck-schema="animate.present.deck">
            <canvas id="animate-present-canvas" width="1280" height="720"></canvas>
            <script id="animate-present-deck" type="application/json">{escaped}</script>
          </main>
          <script type="module" src="/animate/plugin/wasm/animate_plugin.js"></script>
          <script type="module" src="player.js"></script>
        </body>
        </html>
        "#
            )
        }

        fn styles_css() -> &'static str {
            r#"html, body {
          margin: 0;
          height: 100%;
          background: #0b0d12;
          color: #f4f6fb;
          font-family: system-ui, sans-serif;
        }

        #animate-present-root {
          display: grid;
          place-items: center;
          min-height: 100%;
        }

        #animate-present-canvas {
          width: min(100vw, 1280px);
          height: auto;
          aspect-ratio: 16 / 9;
          border: 1px solid #2a3140;
          border-radius: 8px;
          background: #11151d;
        }
        "#
        }

        fn player_boot_js() -> &'static str {
            r#"const root = document.getElementById("animate-present-root");
        const canvas = document.getElementById("animate-present-canvas");
        const deckNode = document.getElementById("animate-present-deck");
        const deck = deckNode ? JSON.parse(deckNode.textContent || "{}") : {};

        function collectSceneClips(node, clips = {}) {
          if (!node || typeof node !== "object") {
            return clips;
          }
          const metadata = node.metadata;
          if (metadata && typeof metadata.sceneHash === "string" && metadata.sceneHash.length > 0) {
            clips[metadata.sceneHash] = `scenes/${metadata.sceneHash}/scene.mp4`;
          }
          if (Array.isArray(node.slides)) {
            for (const slide of node.slides) {
              collectSceneClips(slide, clips);
            }
          }
          if (Array.isArray(node.sections)) {
            for (const section of node.sections) {
              collectSceneClips(section, clips);
            }
          }
          if (Array.isArray(node.chapters)) {
            for (const chapter of node.chapters) {
              collectSceneClips(chapter, clips);
            }
          }
          if (Array.isArray(node.sequences)) {
            for (const sequence of node.sequences) {
              collectSceneClips(sequence, clips);
            }
          }
          if (Array.isArray(node.thoughts)) {
            for (const thought of node.thoughts) {
              collectSceneClips(thought, clips);
            }
          }
          if (node.arrangement) {
            collectSceneClips(node.arrangement, clips);
          }
          if (node.sceneHash) {
            clips[node.sceneHash] = `scenes/${node.sceneHash}/scene.mp4`;
          }
          return clips;
        }

        async function bootAnimatePresentPlayer() {
          const wasmUrl = "/animate/plugin/wasm/animate_plugin_bg.wasm";
          const init = globalThis.AnimatePluginInit || globalThis.default;
          const sceneClips = collectSceneClips(deck);
          if (typeof init !== "function") {
            console.warn("[animate-present] wasm player waiting for animate plugin", { wasmUrl, deck, sceneClips });
            return;
          }
          await init({ canvas, deck, appId: "animate-present-play", sceneClips });
        }

        bootAnimatePresentPlayer().catch((error) => {
          console.error("[animate-present] player boot failed", error);
        });
        "#
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::{default_present_deck, populate_tile_drafts_from_grid, FigureTileGridSeedSpec};

            #[test]
            fn compile_present_site_writes_static_bundle() {
                let deck = default_present_deck();
                let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
                let deck = PresentDeck { tiles, ..deck };
                let output = std::env::temp_dir().join(format!("animate-present-{}", std::process::id()));
                let _ = std::fs::remove_dir_all(&output);
                compile_present_site(&deck, &output).expect("compile site");
                let index = std::fs::read_to_string(output.join("index.html")).expect("index.html");
                assert!(index.contains("animate.present.deck"));
                assert!(index.contains("animate_plugin.js"));
                let player = std::fs::read_to_string(output.join("player.js")).expect("player.js");
                assert!(player.contains("sceneClips"));
                let manifest: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(output.join("manifest.json")).expect("manifest")).expect("json");
                assert_eq!(manifest.get("schema").and_then(|v| v.as_str()), Some("animate.present.site"));
                assert_eq!(manifest.pointer("/player/wasm").and_then(|v| v.as_str()), Some("/animate/plugin/wasm/animate_plugin_bg.wasm"));
                let deck_file: PresentDeck = serde_json::from_str(&std::fs::read_to_string(output.join("deck.json")).expect("deck.json")).expect("deck");
                assert_eq!(deck_file.tiles.len(), 4);
                let _ = std::fs::remove_dir_all(&output);
            }

            #[test]
            fn compile_scene_to_assets_writes_mp4() {
                let output = std::env::temp_dir().join(format!("animate-scene-assets-{}", std::process::id()));
                let _ = std::fs::remove_dir_all(&output);
                let bundle = compile_scene_to_assets("demo123", &output).expect("compile scene");
                assert_eq!(bundle.scene_hash, "demo123");
                assert!(bundle.mp4.as_ref().is_some_and(|path| path.exists()));
                let _ = std::fs::remove_dir_all(&output);
            }
        }
    }

    pub mod slide {
        //! 🎭 Scene-based presentation document types for slide/section timelines.

        use animate_core::Section;
        use serde::{Deserialize, Serialize};

        pub const PRESENT_SCENE_SCHEMA: &str = "animate.present.scene";

        /// 🖼️ One slide within a presentation section — may reference a compiled animate scene hash.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct PresentSlide {
            pub id: String,
            pub title: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub scene_hash: Option<String>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub timeline_sections: Vec<Section>,
        }

        /// 📚 Vertical column of slides (reveal.js sequence analogue).
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct PresentSection {
            pub id: String,
            pub title: String,
            pub slides: Vec<PresentSlide>,
        }

        /// 🎬 Full scene-based presentation document — sections of slides plus optional tile deck overlay.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct PresentScene {
            pub schema: String,
            pub title: String,
            pub sections: Vec<PresentSection>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub deck: Option<crate::PresentDeck>,
        }

        impl PresentScene {
            pub fn empty(title: impl Into<String>) -> Self {
                Self { schema: PRESENT_SCENE_SCHEMA.into(), title: title.into(), sections: Vec::new(), deck: None }
            }

            pub fn slide_count(&self) -> usize {
                self.sections.iter().map(|section| section.slides.len()).sum()
            }

            /// 🎬 Collects unique scene hashes referenced by slides.
            pub fn scene_hashes(&self) -> Vec<String> {
                let mut hashes = Vec::new();
                for section in &self.sections {
                    for slide in &section.slides {
                        if let Some(hash) = &slide.scene_hash {
                            if !hashes.iter().any(|existing| existing == hash) {
                                hashes.push(hash.clone());
                            }
                        }
                    }
                }
                hashes
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn present_scene_counts_slides() {
                let scene = PresentScene {
                    schema: PRESENT_SCENE_SCHEMA.into(),
                    title: "Demo".into(),
                    sections: vec![PresentSection {
                        id: "s1".into(),
                        title: "Intro".into(),
                        slides: vec![
                            PresentSlide { id: "a".into(), title: "A".into(), scene_hash: None, timeline_sections: Vec::new() },
                            PresentSlide { id: "b".into(), title: "B".into(), scene_hash: Some("abc123".into()), timeline_sections: vec![Section::new("main", 0.0, 5.0)] },
                        ],
                    }],
                    deck: None,
                };
                assert_eq!(scene.slide_count(), 2);
                assert_eq!(scene.scene_hashes(), vec!["abc123".to_string()]);
            }
        }
    }

    pub use compiler::{compile_present_site, compile_scene_to_assets, PresentCompileError, SceneAssetBundle};
    pub use slide::{PresentScene, PresentSection, PresentSlide, PRESENT_SCENE_SCHEMA};
}

pub use present::{compile_present_site, compile_scene_to_assets, PresentCompileError, PresentScene, PresentSection, PresentSlide, SceneAssetBundle, PRESENT_SCENE_SCHEMA};

use serde::{Deserialize, Serialize};
#[cfg(any(test, target_arch = "wasm32"))]
use vcs::DocumentVcsCommand;
use vcs::{collection_diff_from_operation, create_document_vcs_envelope, invert_collection_operation, materialize_document_projection, CollectionDiff, CollectionOperation, DocumentVcsEnvelope, DocumentVcsStore, Identified, Operation, OperationDiff, Patchable};

pub const PRESENT_DECK_SCHEMA: &str = "animate.present.deck";

//#region 🔖Error
/// 🎞️ Errors from present deck video export and VCS envelope materialization.
#[derive(Debug, thiserror::Error)]
pub enum PresentError {
    /// 🎬 The scene had no scene hashes to render.
    #[error("presentation has no scene hashes to export")]
    NoSceneHashes,
    /// 🎥 A per-scene render/compile failed.
    #[error(transparent)]
    Compile(#[from] PresentCompileError),
    /// 🧾 The stored envelope JSON was malformed.
    #[error("deserialize envelope: {0}")]
    DeserializeEnvelope(#[from] serde_json::Error),
    /// 📐 VCS replay failed while materializing the projection.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
}
//#endregion 🔖Error

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileSource {
    pub src: String,
    pub kind: String,
    pub frame: FigureTileFrame,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_aspect: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_page: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraft {
    pub id: String,
    pub name: String,
    pub crop: FigureTileFrame,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentDeck {
    pub schema: String,
    pub source: FigureTileSource,
    pub tiles: Vec<FigureTileDraft>,
}

pub type PresentEnvelope = DocumentVcsEnvelope<PresentDeck, PresentOperation>;
pub type PresentStore = DocumentVcsStore<PresentDeck, PresentOperation>;

pub fn empty_present_deck() -> PresentDeck {
    PresentDeck { schema: PRESENT_DECK_SCHEMA.into(), source: default_figure_tile_source(), tiles: Vec::new() }
}

pub fn default_figure_tile_source() -> FigureTileSource {
    FigureTileSource { src: "/bauteilbörse.png".into(), kind: "figure".into(), frame: FigureTileFrame { x: 0.127, y: 0.1, width: 0.746, height: 0.75 }, source_aspect: Some(1222.0 / 896.0), pdf_page: None }
}

pub fn default_present_deck() -> PresentDeck {
    PresentDeck { schema: PRESENT_DECK_SCHEMA.into(), source: default_figure_tile_source(), tiles: Vec::new() }
}
//#endregion 🔖Domain

//#region 🔖TilePlay
pub const NORMALIZED_RECT_MIN_FRACTION: f64 = 0.02;

pub struct SplitFigureGridSpec<'a> {
    pub rows: u32,
    pub columns: u32,
    pub frame: &'a FigureTileFrame,
    pub gap: f64,
    pub key_prefix: &'a str,
}

pub struct SplitGridCell {
    pub key: String,
    pub crop: FigureTileFrame,
}

pub struct FigureTileGridSeedSpec<'a> {
    pub source: &'a FigureTileSource,
    pub rows: u32,
    pub columns: u32,
    pub gap: f64,
    pub key_prefix: &'a str,
}

pub fn clamp_normalized_fraction(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

pub fn clamp_tile_crop(crop: FigureTileFrame) -> FigureTileFrame {
    let width = crop.width.max(NORMALIZED_RECT_MIN_FRACTION);
    let height = crop.height.max(NORMALIZED_RECT_MIN_FRACTION);
    let x = clamp_normalized_fraction(crop.x.min(1.0 - width));
    let y = clamp_normalized_fraction(crop.y.min(1.0 - height));
    FigureTileFrame { x, y, width, height }
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
            cells.push(SplitGridCell { key: format!("{}-r{row}-c{column}", spec.key_prefix), crop: FigureTileFrame { x: frame.x + column as f64 * crop_width, y: frame.y + row as f64 * crop_height, width: crop_width, height: crop_height } });
        }
    }
    let _ = (cell_width, cell_height);
    cells
}

pub fn populate_tile_drafts_from_grid(spec: FigureTileGridSeedSpec<'_>) -> Vec<FigureTileDraft> {
    split_figure_grid(SplitFigureGridSpec { rows: spec.rows, columns: spec.columns, frame: &spec.source.frame, gap: spec.gap, key_prefix: spec.key_prefix })
        .into_iter()
        .map(|cell| FigureTileDraft { id: cell.key.clone(), name: cell.key, crop: cell.crop })
        .collect()
}

pub fn build_tile_morph_prompt(source: &FigureTileSource, drafts: &[FigureTileDraft]) -> String {
    fn format_frame(frame: &FigureTileFrame) -> String {
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

//#endregion 🔖TilePlay

//#region 🔖VideoExport
/// 🎬 Renders every unique `scene_hash` referenced by a {@link PresentScene}.
pub fn export_video_from_scene(scene: &PresentScene, output_dir: &std::path::Path) -> Result<Vec<SceneAssetBundle>, PresentError> {
    let hashes = scene.scene_hashes();
    if hashes.is_empty() {
        return Err(PresentError::NoSceneHashes);
    }
    hashes.into_iter().map(|hash| compile_scene_to_assets(&hash, output_dir).map_err(PresentError::from)).collect()
}
//#endregion 🔖VideoExport

//#region 🔖Operations
//#region 🔖CollectionSupport
impl Identified<String> for FigureTileDraft {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
    pub crop: Option<FigureTileFrame>,
}

impl Patchable<FigureTileDraftPatch> for FigureTileDraft {
    fn apply_patch(&mut self, patch: &FigureTileDraftPatch) -> FigureTileDraftPatch {
        let inverse = FigureTileDraftPatch { name: patch.name.as_ref().map(|_| self.name.clone()), crop: patch.crop.as_ref().map(|_| self.crop.clone()) };
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(crop) = &patch.crop {
            self.crop = crop.clone();
        }
        inverse
    }
}

fn apply_tile_diff(tiles: &mut Vec<FigureTileDraft>, diff: &CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>) {
    for id in &diff.removed {
        tiles.retain(|tile| tile.id != *id);
    }
    for patch in &diff.modified {
        if let Some(tile) = tiles.iter_mut().find(|tile| tile.id == patch.id) {
            tile.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        tiles.push(added.clone());
    }
}

fn absorb_tile_diff(target: &mut Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>, incoming: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}
//#endregion 🔖CollectionSupport

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum PresentOperation {
    Tiles(CollectionOperation<String, FigureTileDraft, FigureTileDraftPatch>),
    SetSource { source: FigureTileSource },
    SetTiles { tiles: Vec<FigureTileDraft> },
    SetDeck { deck: PresentDeck },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentDiff {
    pub deck: Option<PresentDeck>,
    pub source: Option<FigureTileSource>,
    pub tiles: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>,
    pub set_tiles: Option<Vec<FigureTileDraft>>,
}

impl OperationDiff<PresentDeck> for PresentDiff {
    fn apply(&self, projection: &PresentDeck) -> PresentDeck {
        if let Some(deck) = &self.deck {
            return deck.clone();
        }
        let mut next = projection.clone();
        if let Some(source) = &self.source {
            next.source = source.clone();
        }
        if let Some(tiles) = &self.set_tiles {
            next.tiles = tiles.clone();
        }
        if let Some(diff) = &self.tiles {
            apply_tile_diff(&mut next.tiles, diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.deck.is_some() {
            self.deck = other.deck;
            return;
        }
        if other.source.is_some() {
            self.source = other.source;
        }
        if other.set_tiles.is_some() {
            self.set_tiles = other.set_tiles;
        }
        absorb_tile_diff(&mut self.tiles, other.tiles);
    }
}

impl Operation<PresentDeck> for PresentOperation {
    type Diff = PresentDiff;

    fn diff(&self, projection: &PresentDeck) -> PresentDiff {
        match self {
            PresentOperation::Tiles(operation) => PresentDiff { tiles: Some(collection_diff_from_operation(&projection.tiles, operation)), ..Default::default() },
            PresentOperation::SetSource { source } => PresentDiff { source: Some(source.clone()), ..Default::default() },
            PresentOperation::SetTiles { tiles } => PresentDiff { set_tiles: Some(tiles.clone()), ..Default::default() },
            PresentOperation::SetDeck { deck } => PresentDiff { deck: Some(deck.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &PresentDeck) -> Vec<Self> {
        match self {
            PresentOperation::Tiles(operation) => vec![PresentOperation::Tiles(invert_collection_operation(&projection.tiles, operation))],
            PresentOperation::SetSource { .. } => vec![PresentOperation::SetSource { source: projection.source.clone() }],
            PresentOperation::SetTiles { .. } => vec![PresentOperation::SetTiles { tiles: projection.tiles.clone() }],
            PresentOperation::SetDeck { .. } => vec![PresentOperation::SetDeck { deck: projection.clone() }],
        }
    }
}
//#endregion 🔖Operations

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, recursive-descent parser and printer shared by `PresentDeck`'s `.present` DSL
/// (`🔖Dsl`) and `PresentOperation`'s compact single-line op-log encoding (`🔖OpText`) — both share the
/// same `keyword key=value ... { nested }` line grammar (mirrors `imperative_text` in
/// `imperative/core/rs/lib.rs`). A `key=value` token collapses key and bareword value into one word;
/// `key=` alone means the value is a separate following token (a quoted string or a `{ }` block).
/// Whitespace (including newlines) is never significant to the parser — `print_dsl` inserts
/// newlines/indentation purely for readability, `print_op` renders the identical grammar on one line.
/// See {@link vcs::DocumentDsl} and {@link vcs::OpText}.
mod present_text {
    use super::{FigureTileDraft, FigureTileDraftPatch, FigureTileFrame, FigureTileSource, PresentDeck, PresentOperation};
    use vcs::{CollectionOperation, TextError, TextSpan};

    //#region Lexer
    #[derive(Clone, Debug, PartialEq)]
    enum Tok {
        Word(String),
        Str(String),
        LBrace,
        RBrace,
        Eof,
    }

    #[derive(Clone, Debug)]
    struct Lexed {
        tok: Tok,
        span: TextSpan,
    }

    /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`{`/`}`/`"`, so `=` and
    /// `,` are ordinary word characters — `key=value` and `0.1,0.2,0.3,0.4` each collapse into one token.
    fn lex(input: &str) -> Result<Vec<Lexed>, TextError> {
        let chars: Vec<char> = input.chars().collect();
        let mut out = Vec::new();
        let mut i = 0usize;
        let mut line = 1u32;
        let mut col = 1u32;
        while i < chars.len() {
            match chars[i] {
                ' ' | '\t' | '\r' => {
                    i += 1;
                    col += 1;
                }
                '\n' => {
                    i += 1;
                    line += 1;
                    col = 1;
                }
                '{' => {
                    out.push(Lexed { tok: Tok::LBrace, span: TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(Lexed { tok: Tok::RBrace, span: TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '"' => {
                    let (start_line, start_col) = (line, col);
                    i += 1;
                    col += 1;
                    let mut s = String::new();
                    let mut closed = false;
                    while i < chars.len() {
                        let ch = chars[i];
                        if ch == '\\' && i + 1 < chars.len() {
                            match chars[i + 1] {
                                'n' => s.push('\n'),
                                '"' => s.push('"'),
                                '\\' => s.push('\\'),
                                other => {
                                    s.push('\\');
                                    s.push(other);
                                }
                            }
                            i += 2;
                            col += 2;
                        } else if ch == '"' {
                            i += 1;
                            col += 1;
                            closed = true;
                            break;
                        } else if ch == '\n' {
                            s.push(ch);
                            i += 1;
                            line += 1;
                            col = 1;
                        } else {
                            s.push(ch);
                            i += 1;
                            col += 1;
                        }
                    }
                    if !closed {
                        return Err(TextError::new("unterminated string literal", TextSpan::at(start_line, start_col)));
                    }
                    out.push(Lexed { tok: Tok::Str(s), span: TextSpan::at(start_line, start_col) });
                }
                _ => {
                    let (start_line, start_col, start) = (line, col, i);
                    while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '{' | '}' | '"') {
                        i += 1;
                        col += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    out.push(Lexed { tok: Tok::Word(word), span: TextSpan::at(start_line, start_col) });
                }
            }
        }
        out.push(Lexed { tok: Tok::Eof, span: TextSpan::at(line, col) });
        Ok(out)
    }
    //#endregion Lexer

    //#region Parser
    struct Parser {
        toks: Vec<Lexed>,
        pos: usize,
    }

    impl Parser {
        fn peek(&self) -> &Tok {
            &self.toks[self.pos].tok
        }

        fn span(&self) -> TextSpan {
            self.toks[self.pos].span
        }

        fn bump(&mut self) -> Tok {
            let tok = self.toks[self.pos].tok.clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn at_rbrace(&self) -> bool {
            matches!(self.peek(), Tok::RBrace)
        }

        fn at_keyword(&self, keyword: &str) -> bool {
            matches!(self.peek(), Tok::Word(w) if w == keyword)
        }

        fn expect_word(&mut self) -> Result<String, TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Word(w) => Ok(w),
                other => Err(TextError::expected(format!("expected a word, found {other:?}"), span, "word")),
            }
        }

        fn expect_keyword(&mut self, keyword: &str) -> Result<(), TextError> {
            let span = self.span();
            let word = self.expect_word()?;
            if word != keyword {
                return Err(TextError::expected(format!("expected '{keyword}', found '{word}'"), span, keyword.to_string()));
            }
            Ok(())
        }

        fn expect_lbrace(&mut self) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                Tok::LBrace => Ok(()),
                other => Err(TextError::expected(format!("expected '{{', found {other:?}"), span, "{")),
            }
        }

        fn expect_rbrace(&mut self) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                Tok::RBrace => Ok(()),
                other => Err(TextError::expected(format!("expected '}}', found {other:?}"), span, "}")),
            }
        }

        fn expect_eof(&mut self) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Eof => Ok(()),
                other => Err(TextError::expected(format!("expected end of input, found {other:?}"), span, "eof")),
            }
        }

        fn expect_str(&mut self) -> Result<String, TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Str(s) => Ok(s),
                other => Err(TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
            }
        }

        /// 🗝️ Consumes a `key=`/`key=value` word token whose key must equal `key`, returning the
        /// inline suffix — empty when the value is a separate following token (a quoted string),
        /// non-empty when it is a bareword value collapsed into the same token.
        fn expect_kv(&mut self, key: &str) -> Result<String, TextError> {
            let span = self.span();
            let word = self.expect_word()?;
            let (found, rest) = word
                .split_once('=')
                .ok_or_else(|| TextError::expected(format!("expected '{key}=...', found '{word}'"), span, format!("{key}=...")))?;
            if found != key {
                return Err(TextError::expected(format!("expected '{key}=...', found '{found}=...'"), span, format!("{key}=...")));
            }
            Ok(rest.to_string())
        }

        fn expect_kv_str(&mut self, key: &str) -> Result<String, TextError> {
            let span = self.span();
            let rest = self.expect_kv(key)?;
            if !rest.is_empty() {
                return Err(TextError::expected(format!("field '{key}' must be a quoted string"), span, "string"));
            }
            self.expect_str()
        }

        /// 🕳️ Like {@link expect_kv_str} but `-` (bareword) means `None` — vcs's own sentinel for an
        /// absent optional text field.
        fn expect_kv_opt_str(&mut self, key: &str) -> Result<Option<String>, TextError> {
            let span = self.span();
            let rest = self.expect_kv(key)?;
            if rest.is_empty() {
                return Ok(Some(self.expect_str()?));
            }
            if rest == "-" {
                return Ok(None);
            }
            Err(TextError::expected(format!("field '{key}' must be a quoted string or '-'"), span, "string|-"))
        }

        fn expect_kv_word(&mut self, key: &str) -> Result<String, TextError> {
            let span = self.span();
            let rest = self.expect_kv(key)?;
            if rest.is_empty() {
                return Err(TextError::expected(format!("field '{key}' must not be quoted"), span, "word"));
            }
            Ok(rest)
        }

        fn expect_kv_usize(&mut self, key: &str) -> Result<usize, TextError> {
            let span = self.span();
            let word = self.expect_kv_word(key)?;
            word.parse::<usize>().map_err(|_| TextError::expected(format!("field '{key}' must be a non-negative integer"), span, "usize"))
        }

        fn expect_kv_opt_f64(&mut self, key: &str) -> Result<Option<f64>, TextError> {
            let span = self.span();
            let word = self.expect_kv_word(key)?;
            if word == "-" {
                return Ok(None);
            }
            word.parse::<f64>().map(Some).map_err(|_| TextError::expected(format!("field '{key}' must be a number or '-'"), span, "number|-"))
        }

        fn expect_kv_opt_u32(&mut self, key: &str) -> Result<Option<u32>, TextError> {
            let span = self.span();
            let word = self.expect_kv_word(key)?;
            if word == "-" {
                return Ok(None);
            }
            word.parse::<u32>().map(Some).map_err(|_| TextError::expected(format!("field '{key}' must be an integer or '-'"), span, "integer|-"))
        }

        /// 📐 Consumes a `key=x,y,width,height` frame token (see {@link parse_frame_token}).
        fn expect_kv_frame(&mut self, key: &str) -> Result<FigureTileFrame, TextError> {
            let span = self.span();
            let word = self.expect_kv_word(key)?;
            parse_frame_token(&word, span)
        }

        fn expect_kv_opt_frame(&mut self, key: &str) -> Result<Option<FigureTileFrame>, TextError> {
            let span = self.span();
            let word = self.expect_kv_word(key)?;
            if word == "-" {
                return Ok(None);
            }
            Ok(Some(parse_frame_token(&word, span)?))
        }
    }
    //#endregion Parser

    //#region Primitives
    fn quote(value: &str) -> String {
        let mut out = String::with_capacity(value.len() + 2);
        out.push('"');
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out.push('"');
        out
    }

    /// 📐 Parses a `x,y,width,height` frame token (see {@link print_frame}).
    fn parse_frame_token(token: &str, span: TextSpan) -> Result<FigureTileFrame, TextError> {
        let parts: Vec<&str> = token.split(',').collect();
        if parts.len() != 4 {
            return Err(TextError::expected(format!("expected 'x,y,width,height', got '{token}'"), span, "x,y,width,height"));
        }
        let parse = |value: &str| value.parse::<f64>().map_err(|_| TextError::expected(format!("expected a number, got '{value}'"), span, "number"));
        Ok(FigureTileFrame { x: parse(parts[0])?, y: parse(parts[1])?, width: parse(parts[2])?, height: parse(parts[3])? })
    }

    /// 📤 Prints a `FigureTileFrame` as one whitespace-free `x,y,width,height` token.
    fn print_frame(frame: &FigureTileFrame) -> String {
        format!("{},{},{},{}", frame.x, frame.y, frame.width, frame.height)
    }

    fn indent(depth: usize) -> String {
        "  ".repeat(depth)
    }

    /// 🧱 Wraps already-rendered `items` in `{ }`, one per line indented at `depth + 1` when `pretty`,
    /// or space-joined on one line otherwise — mirrors `imperative_text::wrap_body`.
    fn wrap_body(items: &[String], depth: usize, pretty: bool) -> String {
        if pretty {
            let inner_pad = indent(depth + 1);
            let outer_pad = indent(depth);
            let body: String = items.iter().map(|item| format!("{inner_pad}{item}\n")).collect();
            format!("{{\n{body}{outer_pad}}}")
        } else {
            format!("{{ {} }}", items.join(" "))
        }
    }
    //#endregion Primitives

    //#region Source
    fn parse_source(p: &mut Parser) -> Result<FigureTileSource, TextError> {
        p.expect_keyword("source")?;
        let src = p.expect_kv_str("src")?;
        let kind = p.expect_kv_str("kind")?;
        let frame = p.expect_kv_frame("frame")?;
        let source_aspect = p.expect_kv_opt_f64("aspect")?;
        let pdf_page = p.expect_kv_opt_u32("pdfPage")?;
        Ok(FigureTileSource { src, kind, frame, source_aspect, pdf_page })
    }

    fn print_source(source: &FigureTileSource) -> String {
        format!(
            "source src={} kind={} frame={} aspect={} pdfPage={}",
            quote(&source.src),
            quote(&source.kind),
            print_frame(&source.frame),
            source.source_aspect.map(|value| value.to_string()).unwrap_or_else(|| "-".to_string()),
            source.pdf_page.map(|value| value.to_string()).unwrap_or_else(|| "-".to_string()),
        )
    }
    //#endregion Source

    //#region Tile
    fn parse_tile(p: &mut Parser) -> Result<FigureTileDraft, TextError> {
        p.expect_keyword("tile")?;
        let id = p.expect_kv_str("id")?;
        let name = p.expect_kv_str("name")?;
        let crop = p.expect_kv_frame("crop")?;
        Ok(FigureTileDraft { id, name, crop })
    }

    fn print_tile(tile: &FigureTileDraft) -> String {
        format!("tile id={} name={} crop={}", quote(&tile.id), quote(&tile.name), print_frame(&tile.crop))
    }

    fn parse_tiles_block(p: &mut Parser) -> Result<Vec<FigureTileDraft>, TextError> {
        p.expect_lbrace()?;
        let mut tiles = Vec::new();
        while !p.at_rbrace() {
            tiles.push(parse_tile(p)?);
        }
        p.expect_rbrace()?;
        Ok(tiles)
    }

    fn parse_tile_patch(p: &mut Parser) -> Result<FigureTileDraftPatch, TextError> {
        let name = p.expect_kv_opt_str("name")?;
        let crop = p.expect_kv_opt_frame("crop")?;
        Ok(FigureTileDraftPatch { name, crop })
    }

    fn print_tile_patch(patch: &FigureTileDraftPatch) -> String {
        format!(
            "name={} crop={}",
            patch.name.as_deref().map(quote).unwrap_or_else(|| "-".to_string()),
            patch.crop.as_ref().map(print_frame).unwrap_or_else(|| "-".to_string()),
        )
    }
    //#endregion Tile

    //#region Document
    /// 📥 Parses the shared `schema=... ` + `source ...` + `tiles { ... }` body reused by both the full
    /// `.present` DSL document ({@link parse_document}) and `PresentOperation::SetDeck`'s op-text.
    fn parse_deck_body(p: &mut Parser) -> Result<PresentDeck, TextError> {
        let schema = p.expect_kv_str("schema")?;
        let source = parse_source(p)?;
        let tiles = if p.at_keyword("tiles") {
            p.bump();
            parse_tiles_block(p)?
        } else {
            Vec::new()
        };
        Ok(PresentDeck { schema, source, tiles })
    }

    fn print_deck_body(deck: &PresentDeck, pretty: bool) -> String {
        let items: Vec<String> = deck.tiles.iter().map(print_tile).collect();
        let parts = vec![format!("schema={}", quote(&deck.schema)), print_source(&deck.source), format!("tiles {}", wrap_body(&items, 0, pretty))];
        parts.join(if pretty { "\n" } else { " " })
    }

    /// 📥 Parses a full `.present` document: `present schema=... \n source ... \n tiles { ... }` (see
    /// {@link print_document}).
    pub(crate) fn parse_document(text: &str) -> Result<PresentDeck, TextError> {
        let toks = lex(text)?;
        let mut p = Parser { toks, pos: 0 };
        p.expect_keyword("present")?;
        let deck = parse_deck_body(&mut p)?;
        p.expect_eof()?;
        Ok(deck)
    }

    /// 📤 Prints a `PresentDeck` back to its `.present` DSL form (see {@link parse_document}).
    pub(crate) fn print_document(deck: &PresentDeck) -> String {
        format!("present {}", print_deck_body(deck, true))
    }
    //#endregion Document

    //#region Operation
    /// 📥 Parses a single one-line `PresentOperation`: `tiles-add|tiles-remove|tiles-move|tiles-patch`
    /// (mirroring `vcs::CollectionOperation`) or `set-source|set-tiles|set-deck` (see {@link print_operation}).
    pub(crate) fn parse_operation(line: &str) -> Result<PresentOperation, TextError> {
        let toks = lex(line)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        let keyword = p.expect_word()?;
        let operation = match keyword.as_str() {
            "tiles-add" => {
                let index = p.expect_kv_usize("index")?;
                let item = parse_tile(&mut p)?;
                PresentOperation::Tiles(CollectionOperation::Add { index, item })
            }
            "tiles-remove" => {
                let id = p.expect_kv_str("id")?;
                PresentOperation::Tiles(CollectionOperation::Remove { id })
            }
            "tiles-move" => {
                let id = p.expect_kv_str("id")?;
                let to_index = p.expect_kv_usize("to")?;
                PresentOperation::Tiles(CollectionOperation::Move { id, to_index })
            }
            "tiles-patch" => {
                let id = p.expect_kv_str("id")?;
                let patch = parse_tile_patch(&mut p)?;
                PresentOperation::Tiles(CollectionOperation::Patch { id, patch })
            }
            "set-source" => PresentOperation::SetSource { source: parse_source(&mut p)? },
            "set-tiles" => PresentOperation::SetTiles { tiles: parse_tiles_block(&mut p)? },
            "set-deck" => PresentOperation::SetDeck { deck: parse_deck_body(&mut p)? },
            other => {
                return Err(TextError::expected(
                    format!("unknown present operation '{other}'"),
                    span,
                    "tiles-add|tiles-remove|tiles-move|tiles-patch|set-source|set-tiles|set-deck",
                ))
            }
        };
        p.expect_eof()?;
        Ok(operation)
    }

    /// 📤 Renders one `PresentOperation` as a single line (see {@link parse_operation}).
    pub(crate) fn print_operation(operation: &PresentOperation) -> String {
        match operation {
            PresentOperation::Tiles(CollectionOperation::Add { index, item }) => format!("tiles-add index={index} {}", print_tile(item)),
            PresentOperation::Tiles(CollectionOperation::Remove { id }) => format!("tiles-remove id={}", quote(id)),
            PresentOperation::Tiles(CollectionOperation::Move { id, to_index }) => format!("tiles-move id={} to={to_index}", quote(id)),
            PresentOperation::Tiles(CollectionOperation::Patch { id, patch }) => format!("tiles-patch id={} {}", quote(id), print_tile_patch(patch)),
            PresentOperation::SetSource { source } => format!("set-source {}", print_source(source)),
            PresentOperation::SetTiles { tiles } => {
                let items: Vec<String> = tiles.iter().map(print_tile).collect();
                format!("set-tiles {}", wrap_body(&items, 0, false))
            }
            PresentOperation::SetDeck { deck } => format!("set-deck {}", print_deck_body(deck, false)),
        }
    }
    //#endregion Operation
}

/// 📜 `.present` textual document: `present schema=...` then `source ...` then `tiles { ... }` — see
/// {@link present_text}.
impl vcs::DocumentDsl for PresentDeck {
    const EXTENSION: &'static str = "present";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        present_text::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        present_text::print_document(self)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
/// ⚡ One-line op-text for every `PresentOperation` variant (see {@link present_text}).
impl vcs::OpText for PresentOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        present_text::parse_operation(line)
    }

    fn print_op(&self) -> String {
        present_text::print_operation(self)
    }
}
//#endregion 🔖OpText

//#region 🔖VcsEnvelope
/// @emoji 📦 Creates an empty typed VCS envelope for a presentation deck document.
pub fn create_present_envelope(id: &str) -> PresentEnvelope {
    create_document_vcs_envelope(PRESENT_DECK_SCHEMA, id, empty_present_deck(), None)
}

/// @emoji 📐 Replays every stored edit in `envelope_json` and returns the materialized deck projection.
pub fn materialize_present_projection_json(envelope_json: &str) -> Result<PresentDeck, PresentError> {
    let envelope: PresentEnvelope = serde_json::from_str(envelope_json)?;
    let edit_ids: Vec<String> = envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
    Ok(materialize_document_projection(&envelope, &edit_ids)?)
}
//#endregion 🔖VcsEnvelope

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct PresentDocumentVcs {
        store: RefCell<PresentStore>,
    }

    #[wasm_bindgen(js_name = createPresentEnvelopeJson)]
    pub fn create_present_envelope_json(id: &str) -> Result<String, JsValue> {
        serde_json::to_string(&create_present_envelope(id)).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = materializePresentProjectionJson)]
    pub fn materialize_present_projection_json_wasm(envelope_json: &str) -> Result<String, JsValue> {
        let deck = materialize_present_projection_json(envelope_json).map_err(|error| JsValue::from_str(&error.to_string()))?;
        serde_json::to_string(&deck).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen]
    impl PresentDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<PresentDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: PresentEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    PresentStore::new(envelope)
                }
                None => PresentStore::new(create_document_vcs_envelope(PRESENT_DECK_SCHEMA, "animate-present", empty_present_deck(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::test_support;

    #[test]
    fn envelope_helpers_round_trip() {
        let envelope = create_present_envelope("deck-1");
        let json = serde_json::to_string(&envelope).expect("serialize");
        let deck = materialize_present_projection_json(&json).expect("materialize");
        assert_eq!(deck.schema, PRESENT_DECK_SCHEMA);
        assert!(deck.tiles.is_empty());
    }

    #[test]
    fn grid_seed_produces_tiles() {
        let source = default_figure_tile_source();
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
        let source = default_figure_tile_source();
        let tiles = vec![FigureTileDraft { id: "t1".into(), name: "t1".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } }];
        let prompt = build_tile_morph_prompt(&source, &tiles);
        assert!(prompt.contains("t1"));
        assert!(prompt.contains("Source media"));
    }

    fn round_trip(deck: &PresentDeck, operation: &PresentOperation) -> PresentDeck {
        let forward = vcs::apply_operation(deck, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(deck) {
            restored = vcs::apply_operation(&restored, &back);
        }
        assert_eq!(&restored, deck, "backwards() must exactly restore the pre-operation deck");
        forward
    }

    #[test]
    fn set_tiles_and_clear_round_trip() {
        let deck = default_present_deck();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let seeded = round_trip(&deck, &PresentOperation::SetTiles { tiles: tiles.clone() });
        assert_eq!(seeded.tiles.len(), 4);
        let cleared = round_trip(&seeded, &PresentOperation::SetTiles { tiles: Vec::new() });
        assert!(cleared.tiles.is_empty());
    }

    #[test]
    fn tile_add_patch_remove_round_trip() {
        let deck = default_present_deck();
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        let added = round_trip(&deck, &PresentOperation::Tiles(CollectionOperation::Add { index: 0, item: tile }));
        assert_eq!(added.tiles.len(), 1);
        let renamed = round_trip(&added, &PresentOperation::Tiles(CollectionOperation::Patch { id: "t1".into(), patch: FigureTileDraftPatch { name: Some("Renamed".into()), crop: None } }));
        assert_eq!(renamed.tiles[0].name, "Renamed");
        let recropped = round_trip(&renamed, &PresentOperation::Tiles(CollectionOperation::Patch { id: "t1".into(), patch: FigureTileDraftPatch { name: None, crop: Some(FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 }) } }));
        assert_eq!(recropped.tiles[0].crop.width, 0.4);
        let removed = round_trip(&recropped, &PresentOperation::Tiles(CollectionOperation::Remove { id: "t1".into() }));
        assert!(removed.tiles.is_empty());
    }

    #[test]
    fn present_deck_materializes() {
        let mut store = PresentStore::new(create_document_vcs_envelope(PRESENT_DECK_SCHEMA, "animate-present", empty_present_deck(), None));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![PresentOperation::Tiles(CollectionOperation::Add { index: 0, item: FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } } })],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").tiles.len(), 1);
    }

    #[test]
    fn present_deck_schema_is_animate_present() {
        assert_eq!(default_present_deck().schema, PRESENT_DECK_SCHEMA);
    }

    //#region 🔖DslTests
    #[test]
    fn dsl_round_trip_default_present_deck() {
        test_support::assert_dsl_round_trip(&default_present_deck());
    }

    #[test]
    fn dsl_round_trip_present_deck_with_tiles() {
        let deck = default_present_deck();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let deck = PresentDeck { tiles, ..deck };
        test_support::assert_dsl_round_trip(&deck);
    }
    //#endregion 🔖DslTests

    //#region 🔖OpTextTests
    #[test]
    fn op_text_round_trip_tiles_add() {
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Add { index: 0, item: tile }));
    }

    #[test]
    fn op_text_round_trip_tiles_remove() {
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Remove { id: "t1".into() }));
    }

    #[test]
    fn op_text_round_trip_tiles_move() {
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Move { id: "t1".into(), to_index: 2 }));
    }

    #[test]
    fn op_text_round_trip_tiles_patch_full() {
        let patch = FigureTileDraftPatch { name: Some("Renamed".into()), crop: Some(FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 }) };
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Patch { id: "t1".into(), patch }));
    }

    #[test]
    fn op_text_round_trip_tiles_patch_empty() {
        let patch = FigureTileDraftPatch { name: None, crop: None };
        test_support::assert_op_line_round_trip(&PresentOperation::Tiles(CollectionOperation::Patch { id: "t1".into(), patch }));
    }

    #[test]
    fn op_text_round_trip_set_source() {
        test_support::assert_op_line_round_trip(&PresentOperation::SetSource { source: default_figure_tile_source() });
    }

    #[test]
    fn op_text_round_trip_set_tiles() {
        let source = default_figure_tile_source();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        test_support::assert_op_line_round_trip(&PresentOperation::SetTiles { tiles });
    }

    #[test]
    fn op_text_round_trip_set_deck() {
        test_support::assert_op_line_round_trip(&PresentOperation::SetDeck { deck: default_present_deck() });
    }
    //#endregion 🔖OpTextTests

    //#region 🔖DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = PresentStore::new(create_document_vcs_envelope(PRESENT_DECK_SCHEMA, "animate-present", default_present_deck(), None));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![PresentOperation::Tiles(CollectionOperation::Add { index: 0, item: FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } } })],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
//#endregion 🧪Tests
