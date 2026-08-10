//! ⚙️ Animate present artifact — headless compute (constitutional: engine). Also hosts the plugin's
//! `register()` entrypoint (moved from the old bundle crate's `📦️glue.rs`, called from the plugin-root
//! `📦️glue.rs`'s `semio_plugin!{}` `setup:` field), and — as sibling `🦀️<topic>.rs` files, per the
//! taxonomy's allowance for big engines — the Manim-class animation core (`animate`, only ever used
//! by this app's own engine and by `animate_video`) and the headless video renderer (`animate_video`,
//! only ever used by this engine's `compiler` submodule below). Both were their own plugin-level crates
//! before this migration; neither has a dependent outside this one artifact/app, so per the plan's
//! simpler placement rule they fold in here rather than becoming a plugin-level `🫀️core`.

pub mod compiler {
    //! 🌐️ Headless static-site compiler for animate present decks.

    use crate::artifacts::present::engine::animate::{AnimateConfig, QualityPreset};
    use crate::artifacts::present::engine::animate_video::{render_scene, scene_for_hash, OutputFormat};
    use crate::artifacts::present::PresentSnapshot;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// 🚨️ Static-site compilation failure.
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

    /// 📦️ Rendered scene clip paths for present sites and plugin export.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SceneAssetBundle {
        pub scene_hash: String,
        pub mp4: Option<PathBuf>,
        pub last_frame: Option<PathBuf>,
        pub subtitles: Option<PathBuf>,
        pub sections: Option<PathBuf>,
    }

    /// 🎬️ Renders one animate scene hash into `output_dir/scenes/{hash}`.
    pub fn compile_scene_to_assets(scene_hash: &str, output_dir: &Path) -> Result<SceneAssetBundle> {
        let scene_dir = output_dir.join("scenes").join(scene_hash);
        fs::create_dir_all(&scene_dir).map_err(|error| PresentCompileError::new(error.to_string()))?;
        let config = AnimateConfig::from_quality(QualityPreset::Medium).with_output_dir(&scene_dir).with_media_dir(scene_dir.join("media")).with_subtitles_path(scene_dir.join("scene.srt"));
        let scene = scene_for_hash(config.clone(), scene_hash);
        let outputs = render_scene(scene, &config, &[OutputFormat::Mp4, OutputFormat::LastFrame]).map_err(|error| PresentCompileError::new(error.to_string()))?;
        Ok(SceneAssetBundle { scene_hash: scene_hash.into(), mp4: outputs.mp4, last_frame: outputs.last_frame, subtitles: Some(scene_dir.join("scene.srt")), sections: outputs.sections })
    }

    /// 📦️ Writes `🌐️index.html`, `styles.css`, `manifest.json`, and embedded deck JSON for a wgpu-ready site.
    pub fn compile_present_site(deck: &PresentSnapshot, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir).map_err(|error| PresentCompileError::new(error.to_string()))?;
        let deck_json = serde_json::to_string_pretty(deck).map_err(|error| PresentCompileError::new(format!("deck json: {error}")))?;
        fs::write(output_dir.join("deck.json"), &deck_json).map_err(|error| PresentCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("🌐️index.html"), index_html(&deck_json)).map_err(|error| PresentCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("styles.css"), styles_css()).map_err(|error| PresentCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("manifest.json"), serde_json::to_string_pretty(&site_manifest(deck)).map_err(|error| PresentCompileError::new(error.to_string()))?).map_err(|error| PresentCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("player.js"), player_boot_js()).map_err(|error| PresentCompileError::new(error.to_string()))?;
        Ok(())
    }

    fn site_manifest(deck: &PresentSnapshot) -> serde_json::Value {
        json!({
            "schema": "animate.present.site",
            "deckSchema": deck.schema,
            "title": deck.tiles.first().map_or("Animate Present", |tile| tile.name.as_str()),
            "tileCount": deck.tiles.len(),
            "player": {
                "kind": "wgpu",
                "wasm": "/animate/plugin/wasm/animate_plugin_bg.wasm",
                "js": "/animate/plugin/wasm/semio_s_plugin_animate.js",
                "boot": "/animate/plugin/wasm/🟨️boot.js"
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
        <script id="animate-present-deck" type="text/dsl">{escaped}</script>
      </main>
      <script type="module" src="/animate/plugin/wasm/semio_s_plugin_animate.js"></script>
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
        use crate::artifacts::present::default_present_snapshot;
        use crate::artifacts::present::engine::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};

        #[test]
        fn compile_present_site_writes_static_bundle() {
            let deck = default_present_snapshot();
            let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
            let deck = PresentSnapshot { tiles, ..deck };
            let output = std::env::temp_dir().join(format!("animate-present-{}", std::process::id()));
            let _ = fs::remove_dir_all(&output);
            compile_present_site(&deck, &output).expect("compile site");
            let index = fs::read_to_string(output.join("🌐️index.html")).expect("🌐️index.html");
            assert!(index.contains("animate.present.deck"));
            assert!(index.contains("semio_s_plugin_animate.js"));
            let player = fs::read_to_string(output.join("player.js")).expect("player.js");
            assert!(player.contains("sceneClips"));
            let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(output.join("manifest.json")).expect("manifest")).expect("json");
            assert_eq!(manifest.get("schema").and_then(|v| v.as_str()), Some("animate.present.site"));
            assert_eq!(manifest.pointer("/player/wasm").and_then(|v| v.as_str()), Some("/animate/plugin/wasm/animate_plugin_bg.wasm"));
            let deck_file: PresentSnapshot = serde_json::from_str(&fs::read_to_string(output.join("deck.json")).expect("deck.json")).expect("deck");
            assert_eq!(deck_file.tiles.len(), 4);
            let _ = fs::remove_dir_all(&output);
        }

        #[test]
        fn compile_scene_to_assets_writes_mp4() {
            let output = std::env::temp_dir().join(format!("animate-scene-assets-{}", std::process::id()));
            let _ = fs::remove_dir_all(&output);
            let bundle = compile_scene_to_assets("demo123", &output).expect("compile scene");
            assert_eq!(bundle.scene_hash, "demo123");
            assert!(bundle.mp4.as_ref().is_some_and(|path| path.exists()));
            let _ = fs::remove_dir_all(&output);
        }
    }
}

pub mod slide {
    //! 🎭️ Scene-based presentation document types for slide/section timelines.

    use crate::artifacts::present::engine::animate::Section;
    use crate::artifacts::present::PresentSnapshot;
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

    /// 📚️ Vertical column of slides (reveal.js sequence analogue).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PresentSection {
        pub id: String,
        pub title: String,
        pub slides: Vec<PresentSlide>,
    }

    /// 🎬️ Full scene-based presentation document — sections of slides plus optional tile deck overlay.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PresentScene {
        pub schema: String,
        pub title: String,
        pub sections: Vec<PresentSection>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deck: Option<PresentSnapshot>,
    }

    impl PresentScene {
        pub fn empty(title: impl Into<String>) -> Self {
            Self { schema: PRESENT_SCENE_SCHEMA.into(), title: title.into(), sections: Vec::new(), deck: None }
        }

        pub fn slide_count(&self) -> usize {
            self.sections.iter().map(|section| section.slides.len()).sum()
        }

        /// 🎬️ Collects unique scene hashes referenced by slides.
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

use crate::artifacts::present::PRESENT_DOCUMENT_SCHEMA;

//#region 🔖️Register
/// 🔌️ Called by the plugin-root `📦️glue.rs`'s `semio_plugin!{}` `setup:` field.
pub fn register() {
    crate::artifacts::present::composer::register();

    register_pilot_languages();
    register_artifact_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::present::AnimatePresentPlayApp>(PRESENT_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "present.document",
        extension: Some("present"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::present::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::present::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::present::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::present::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("present.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "present.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::present::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::present::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::present::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::present::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("present.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "present.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::present::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::present::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("present.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "present.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::present::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::present::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("present.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "present.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::present::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::present::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("present.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_animate_present_app`'s
/// `.artifact_kind(...)` literal (schema/media type copied verbatim) plus the extra `frames:in` input
/// port (Wave-2 port recipe).
pub fn present_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        document_schema: PRESENT_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Presentation, form: semio_framework::MediaForm::Deck },
        ports: vec![semio_framework::MediaPortSpec {
            id: "frames:in".into(),
            label: "Frames".into(),
            direction: semio_framework::MediaPortDirection::In,
            media_type: semio_framework::MediaType { class: semio_framework::MediaClass::TwoD, form: semio_framework::MediaForm::Raster },
            kind_id: Some("2d.image".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: PRESENT_DOCUMENT_SCHEMA.into(), name: "Animate Present Deck".into(), dimension: "2d".into(), component_kind: "panel".into() },
    }
}

/// 🎞️ `frames:in` placement (Wave-2 port recipe) — `PresentSnapshot` models one shared background
/// `source` image with named crop-`tiles` over it; there is no per-tile independent raster payload in
/// this schema, so an incoming `2d.image` frame becomes a new tile positioned in a deterministic
/// contact-sheet grid (4 columns) rather than replacing `source` — exactly the surface `seedGrid`/
/// `addTile` (see the app's `🎮️commands/🀄️tile`/`🎮️commands/🌐️grid`) already let a user crop/arrange
/// candidate frames on. Pure: both functions depend only on the current tile COUNT, so repeated imports
/// land in distinct, stable cells without needing a live host/counter.
const FRAME_IMPORT_GRID_COLUMNS: usize = 4;

pub fn next_frame_tile_id(existing_tile_count: usize) -> String {
    format!("frame-{}", existing_tile_count + 1)
}

pub fn next_frame_tile_crop(existing_tile_count: usize) -> crate::artifacts::present::FigureTileFrame {
    let cell = 1.0 / FRAME_IMPORT_GRID_COLUMNS as f64;
    let column = existing_tile_count % FRAME_IMPORT_GRID_COLUMNS;
    let row = existing_tile_count / FRAME_IMPORT_GRID_COLUMNS;
    clamp_tile_crop(&crate::artifacts::present::FigureTileFrame { x: column as f64 * cell, y: (row as f64 * cell).min(1.0 - cell), width: cell, height: cell })
}
//#endregion 🔖️Io

//#region 🔖️Error
/// 🎞️ Errors from present deck video export and VCS envelope materialization.
#[derive(Debug, thiserror::Error)]
pub enum PresentError {
    /// 🎬️ The scene had no scene hashes to render.
    #[error("presentation has no scene hashes to export")]
    NoSceneHashes,
    /// 🎥️ A per-scene render/compile failed.
    #[error(transparent)]
    Compile(#[from] PresentCompileError),
    /// 🧾️ The stored envelope JSON was malformed.
    #[error("deserialize envelope: {0}")]
    DeserializeEnvelope(#[from] serde_json::Error),
    /// 📐️ VCS replay failed while materializing the projection.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
}
//#endregion 🔖️Error

//#region 🔖️Domain
/// 📄️ Empty presentation deck — the wasm VCS bridge's default projection for a fresh envelope.
pub fn empty_present_snapshot() -> crate::artifacts::present::PresentSnapshot {
    crate::artifacts::present::PresentSnapshot { schema: PRESENT_DOCUMENT_SCHEMA.into(), source: crate::artifacts::present::default_figure_tile_source(), tiles: Vec::new() }
}
//#endregion 🔖️Domain

//#region 🔖️TilePlay
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
//#endregion 🔖️TilePlay

//#region 🔖️VideoExport
/// 🎬️ Renders every unique `scene_hash` referenced by a {@link PresentScene}.
pub fn export_video_from_scene(scene: &PresentScene, output_dir: &std::path::Path) -> Result<Vec<SceneAssetBundle>, PresentError> {
    let hashes = scene.scene_hashes();
    if hashes.is_empty() {
        return Err(PresentError::NoSceneHashes);
    }
    hashes.into_iter().map(|hash| compile_scene_to_assets(&hash, output_dir).map_err(PresentError::from)).collect()
}
//#endregion 🔖️VideoExport

//#region 🔖️MediaCodec
/// 🖼️ Title-card SVG export for the app catalogue/thumbnail surface.
pub fn animate_present_document_json_to_svg(value: &serde_json::Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Animate Present", 1280, 720)
}

/// 📥️ Builds a degenerate-but-valid one-slide deck from a rasterized DWG drawing, for the DWG import path.
pub fn animate_present_document_json_from_dwg(drawing: &semio_framework::DwgDrawing) -> Result<serde_json::Value, String> {
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height)?;
    let frame = crate::artifacts::present::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    let deck = crate::artifacts::present::PresentSnapshot {
        schema: PRESENT_DOCUMENT_SCHEMA.into(),
        source: crate::artifacts::present::FigureTileSource { src: format!("data:image/png;base64,{png_base64}"), kind: "image".into(), frame: frame.clone(), source_aspect: Some(width as f64 / height.max(1) as f64), pdf_page: None },
        tiles: vec![crate::artifacts::present::FigureTileDraft { id: "imported-drawing".into(), name: "Imported Drawing".into(), crop: frame }],
    };
    serde_json::to_value(&deck).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaCodec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::default_figure_tile_source;
    use serde_json::json;

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
        let tiles = vec![crate::artifacts::present::FigureTileDraft { id: "t1".into(), name: "t1".into(), crop: crate::artifacts::present::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } }];
        let prompt = build_tile_morph_prompt(&source, &tiles);
        assert!(prompt.contains("t1"));
        assert!(prompt.contains("Source media"));
    }

    #[test]
    fn animate_present_document_json_to_svg_embeds_title() {
        let (svg, width, height) = animate_present_document_json_to_svg(&json!({ "title": "My Deck" })).expect("svg");
        assert!(svg.contains("My Deck"));
        assert_eq!((width, height), (1280, 720));
    }

    #[test]
    fn animate_present_document_json_to_svg_falls_back_to_app_label_without_title() {
        let (svg, _, _) = animate_present_document_json_to_svg(&json!({})).expect("svg fallback");
        assert!(svg.contains("Animate Present"));
    }

    #[test]
    fn from_dwg_builds_single_slide_deck_from_entity() {
        let drawing = semio_framework::DwgDrawing {
            layers: vec![semio_framework::DwgLayer::default()],
            entities: vec![semio_framework::DwgEntity {
                layer: 0,
                color: semio_framework::DwgColor::ByLayer,
                geometry: semio_framework::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
            }],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let document = animate_present_document_json_from_dwg(&drawing).expect("from_dwg");
        let deck: crate::artifacts::present::PresentSnapshot = serde_json::from_value(document).expect("deck");
        assert_eq!(deck.schema, PRESENT_DOCUMENT_SCHEMA);
        assert_eq!(deck.tiles.len(), 1);
        assert_eq!(deck.tiles[0].name, "Imported Drawing");
        assert!(deck.source.src.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn from_dwg_never_errors_on_empty_drawing() {
        let drawing = semio_framework::DwgDrawing::default();
        let document = animate_present_document_json_from_dwg(&drawing).expect("from_dwg on empty drawing");
        let deck: crate::artifacts::present::PresentSnapshot = serde_json::from_value(document).expect("deck");
        assert_eq!(deck.tiles.len(), 1);
    }

    //#region 🔖️IoTests
    #[test]
    fn present_io_declares_the_frames_in_port() {
        let io = present_io();
        assert_eq!(io.document_schema, PRESENT_DOCUMENT_SCHEMA);
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "frames:in");
        assert_eq!(port.kind_id.as_deref(), Some("2d.image"));
        assert_eq!(port.direction, semio_framework::MediaPortDirection::In);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert!(!port.required);
    }

    #[test]
    fn frame_import_placement_is_deterministic_and_non_overlapping() {
        let first = next_frame_tile_crop(0);
        let second = next_frame_tile_crop(1);
        assert_ne!(first, second);
        assert_eq!(next_frame_tile_id(0), "frame-1");
        assert_eq!(next_frame_tile_id(1), "frame-2");
        // 🧮️ Pure function of the count, not a mutating counter.
        assert_eq!(next_frame_tile_crop(0), first);
    }
    //#endregion 🔖️IoTests
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
/// 🧬️ UI-independent document engine — every transition is a `PresentMutation`.
pub struct PresentEngine {
    artifact: crate::artifacts::present::schema::PresentArtifact,
    snapshot: crate::artifacts::present::PresentSnapshot,
}

impl PresentEngine {
    pub fn new(snapshot: crate::artifacts::present::PresentSnapshot) -> Self {
        let artifact = crate::artifacts::present::schema::PresentArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::present::PresentSnapshot {
        self.snapshot
    }
}

impl protocol::ArtifactEngine for PresentEngine {
    type Artifact = crate::artifacts::present::schema::PresentArtifact;
    type Snapshot = crate::artifacts::present::PresentSnapshot;
    type Mutation = crate::artifacts::present::mutations::PresentMutation;
    type Diff = crate::artifacts::present::diff::PresentDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️SchemaRegistry
/// 📌️ Registers the fifteen handcrafted schema leaves for `s.animate.present`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::present::schema::present_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry

