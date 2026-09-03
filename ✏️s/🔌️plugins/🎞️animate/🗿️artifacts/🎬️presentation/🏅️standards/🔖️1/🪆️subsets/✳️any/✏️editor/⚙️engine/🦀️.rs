//! ⚙️ Animate presentation app engine — the app's own stateful host over the artifact's pure
//! `PresentationSnapshot` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: relocated wholesale
//! from the deleted artifact-tree `⚙️engine` — an artifact is a schema + io, never an engine; behaviour
//! belongs to the app that edits it). Hosts the static-site compiler (`compiler`, real filesystem
//! writes), the scene-based presentation document types (`slide`), and headless video export
//! (`🔖️VideoExport`) — plus, as sibling `<topic>/🦀️.rs` files mirroring this taxonomy node's
//! own subdirs, the Manim-class animation core (`⏱️rate`, `🎛️config`, `🎞️animation`, `📷️camera`,
//! `🎬️scene`, `📐️geometry`, `🔤️text`) and the headless video renderer (`🎥️video`). Both were their own
//! plugin-level crates before an earlier migration; neither has a dependent outside this app, so per
//! that migration's placement rule they stay folded in here rather than becoming a plugin-level
//! `🫀️core`. The former `PresentationEngine` struct (a `PresentationArtifact`+`PresentationSnapshot` pair with only
//! `new`/`into_snapshot`) had zero external references and no `ArtifactEngine` trait impl anywhere in
//! the plugin — deleted outright, not rehomed, per this ticket's classification rule for the norm case.

pub mod compiler {
    //! 🌐️ Headless static-site compiler for animate presentation decks.

    use crate::artifacts::presentation::PresentationSnapshot;
    use crate::editor::animate::engine::config::config::{AnimateConfig, QualityPreset};
    use crate::editor::animate::engine::video::{render_scene, scene_for_hash, OutputFormat};
    use std::fs;
    use std::path::{Path, PathBuf};

    /// 🚨️ Static-site compilation failure.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PresentationCompileError {
        pub message: String,
    }

    impl std::fmt::Display for PresentationCompileError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(&self.message)
        }
    }

    impl std::error::Error for PresentationCompileError {}

    impl PresentationCompileError {
        fn new(message: impl Into<String>) -> Self {
            Self { message: message.into() }
        }
    }

    pub type Result<T> = std::result::Result<T, PresentationCompileError>;

    /// 📦️ Rendered scene clip paths for presentation sites and plugin export.
    #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    pub struct SceneAssetBundle {
        pub scene_hash: String,
        pub mp4: Option<PathBuf>,
        pub last_frame: Option<PathBuf>,
        pub subtitles: Option<PathBuf>,
        pub sections: Option<PathBuf>,
    }

    /// 🎬️ Renders one animate scene hash into `output_dir/scenes/{hash}`.
    pub async fn compile_scene_to_assets(scene_hash: &str, output_dir: &Path) -> Result<SceneAssetBundle> {
        let scene_dir = output_dir.join("scenes").join(scene_hash);
        fs::create_dir_all(&scene_dir).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        let config = AnimateConfig::from_quality(QualityPreset::Medium).with_output_dir(&scene_dir).with_media_dir(scene_dir.join("media")).with_subtitles_path(scene_dir.join("scene.srt"));
        let scene = scene_for_hash(config.clone(), scene_hash);
        let outputs = render_scene(scene, &config, &[OutputFormat::Mp4, OutputFormat::LastFrame]).await.map_err(|error| PresentationCompileError::new(error.to_string()))?;
        Ok(SceneAssetBundle { scene_hash: scene_hash.into(), mp4: outputs.mp4, last_frame: outputs.last_frame, subtitles: Some(scene_dir.join("scene.srt")), sections: outputs.sections })
    }

    /// 📦️ Writes `🌐️.html`, `styles.css`, `manifest.json`, and embedded deck JSON for a wgpu-ready site.
    /// `🌐️.html` is built as a real `HtmlSnapshot` (typed element tree) and serialized through
    /// stdio's real HTML5 engine — the hand-rolled `format!("<!DOCTYPE html>...")` string emitter this
    /// replaced is deleted outright (`styles.css`/`player.js`/`manifest.json`/`deck.json` are plain
    /// CSS/JS/JSON sidecars, not HTML — no ad-hoc HTML codec logic lived at those sites, so they stay
    /// unchanged `fs::write`s).
    pub fn compile_presentation_site(deck: &PresentationSnapshot, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        let deck_value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(deck));
        let deck_json = dsl::os_pack::json::to_string_pretty(&deck_value);
        fs::write(output_dir.join("deck.json"), &deck_json).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        let index_snapshot = index_html_snapshot(&deck_json);
        let index_text = semio_s_plugin_stdio::artifacts::html::standards::v5::subsets::any::schema::snapshot::write_html_document(&index_snapshot);
        fs::write(output_dir.join("🌐️.html"), index_text).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("styles.css"), styles_css()).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("manifest.json"), dsl::os_pack::json::to_string_pretty(&site_manifest(deck))).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("player.js"), player_boot_js()).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        Ok(())
    }

    fn site_manifest(deck: &PresentationSnapshot) -> dsl::os_pack::json::Value {
        let (_, tiles) = crate::artifacts::presentation::presentation_working_scene(deck);
        dsl::os_pack::json::object([
            ("schema".to_string(), dsl::os_pack::json::Value::from("animate.presentation.site")),
            ("deckSchema".to_string(), dsl::os_pack::json::Value::from(deck.schema.clone())),
            ("title".to_string(), dsl::os_pack::json::Value::from(tiles.first().map_or("Animate Presentation", |tile| tile.name.as_str()))),
            ("tileCount".to_string(), dsl::os_pack::json::Value::from(tiles.len())),
            (
                "player".to_string(),
                dsl::os_pack::json::object([
                    ("kind".to_string(), dsl::os_pack::json::Value::from("wgpu")),
                    ("wasm".to_string(), dsl::os_pack::json::Value::from("/animate/plugin/wasm/animate_plugin_bg.wasm")),
                    ("js".to_string(), dsl::os_pack::json::Value::from("/animate/plugin/wasm/semio_s_plugin_animate.js")),
                    ("boot".to_string(), dsl::os_pack::json::Value::from("/animate/plugin/wasm/🟨️boot.js")),
                ]),
            ),
            (
                "assets".to_string(),
                dsl::os_pack::json::object([
                    ("deck".to_string(), dsl::os_pack::json::Value::from("deck.json")),
                    ("styles".to_string(), dsl::os_pack::json::Value::from("styles.css")),
                    ("player".to_string(), dsl::os_pack::json::Value::from("player.js")),
                    ("scenes".to_string(), dsl::os_pack::json::Value::from("scenes")),
                ]),
            ),
        ])
    }

    /// 🌐️ Builds `🌐️.html`'s real `HtmlSnapshot` — deck JSON lands verbatim inside the
    /// `<script>` tag's `RawText` node (HTML5's RAWTEXT content model never entity-decodes script
    /// content, so this is MORE spec-correct than the deleted emitter's `&`/`<` string-replace,
    /// which would have literally corrupted any deck JSON string containing those characters once
    /// a real browser DOM read it back via `textContent`).
    fn index_html_snapshot(deck_json: &str) -> semio_s_plugin_stdio::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot {
        use semio_s_plugin_stdio::artifacts::html::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode, HtmlSnapshot, RawTextKind, STDIO_HTML_DOCUMENT_SCHEMA};

        fn el(name: &str, attrs: Vec<HtmlAttr>, children: Vec<HtmlNode>) -> HtmlNode {
            HtmlNode::Element { name: name.into(), attributes: attrs, children }
        }
        fn module_script(src: &str) -> HtmlNode {
            el("script", vec![HtmlAttr::new("type", "module"), HtmlAttr::new("src", src)], Vec::new())
        }

        let head = el(
            "head",
            Vec::new(),
            vec![
                el("meta", vec![HtmlAttr::new("charset", "utf-8")], Vec::new()),
                el("meta", vec![HtmlAttr::new("name", "viewport"), HtmlAttr::new("content", "width=device-width, initial-scale=1")], Vec::new()),
                el("title", Vec::new(), vec![HtmlNode::Text { text: "Animate Presentation".into() }]),
                el("link", vec![HtmlAttr::new("rel", "stylesheet"), HtmlAttr::new("href", "styles.css")], Vec::new()),
                el("link", vec![HtmlAttr::new("rel", "manifest"), HtmlAttr::new("href", "manifest.json")], Vec::new()),
            ],
        );
        let deck_script_children = if deck_json.is_empty() { Vec::new() } else { vec![HtmlNode::RawText { parent_kind: RawTextKind::Script, text: deck_json.to_string() }] };
        let main = el(
            "main",
            vec![HtmlAttr::new("id", "animate-presentation-root"), HtmlAttr::new("data-deck-schema", "animate.presentation.deck")],
            vec![
                el("canvas", vec![HtmlAttr::new("id", "animate-presentation-canvas"), HtmlAttr::new("width", "1280"), HtmlAttr::new("height", "720")], Vec::new()),
                el("script", vec![HtmlAttr::new("id", "animate-presentation-deck"), HtmlAttr::new("type", "text/dsl")], deck_script_children),
            ],
        );
        let body = el("body", Vec::new(), vec![main, module_script("/animate/plugin/wasm/semio_s_plugin_animate.js"), module_script("player.js")]);
        let html = el("html", vec![HtmlAttr::new("lang", "en")], vec![head, body]);
        HtmlSnapshot { schema: STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype: Some("DOCTYPE html".into()), root: html }
    }

    fn styles_css() -> &'static str {
        r#"html, body {
      margin: 0;
      height: 100%;
      background: #0b0d12;
      color: #f4f6fb;
      font-family: system-ui, sans-serif;
    }

    #animate-presentation-root {
      display: grid;
      place-items: center;
      min-height: 100%;
    }

    #animate-presentation-canvas {
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
        r#"const root = document.getElementById("animate-presentation-root");
    const canvas = document.getElementById("animate-presentation-canvas");
    const deckNode = document.getElementById("animate-presentation-deck");
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

    async function bootAnimatePresentationPlayer() {
      const wasmUrl = "/animate/plugin/wasm/animate_plugin_bg.wasm";
      const init = globalThis.AnimatePluginInit || globalThis.default;
      const sceneClips = collectSceneClips(deck);
      if (typeof init !== "function") {
        console.warn("[animate-presentation] wasm player waiting for animate plugin", { wasmUrl, deck, sceneClips });
        return;
      }
      await init({ canvas, deck, appId: "animate-presentation-play", sceneClips });
    }

    bootAnimatePresentationPlayer().catch((error) => {
      console.error("[animate-presentation] player boot failed", error);
    });
    "#
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::presentation::default_presentation_snapshot;
        use crate::artifacts::presentation::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};

        #[semio_framework_async_macros::async_test]
        async fn compile_presentation_site_writes_static_bundle() {
            let deck = default_presentation_snapshot();
            let (source, _) = crate::artifacts::presentation::presentation_working_scene(&deck);
            let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
            let deck = crate::artifacts::presentation::presentation_snapshot_with_tiles(&source, &tiles);
            let output = std::env::temp_dir().join(format!("animate-presentation-{}", std::process::id()));
            let _ = fs::remove_dir_all(&output);
            compile_presentation_site(&deck, &output).expect("compile site");
            let index = fs::read_to_string(output.join("🌐️.html")).expect("🌐️.html");
            assert!(index.contains("animate.presentation.deck"));
            assert!(index.contains("semio_s_plugin_animate.js"));
            let player = fs::read_to_string(output.join("player.js")).expect("player.js");
            assert!(player.contains("sceneClips"));
            let manifest = dsl::os_pack::json::parse(&fs::read_to_string(output.join("manifest.json")).expect("manifest")).expect("json");
            assert_eq!(manifest.get("schema").and_then(|v| v.as_str()), Some("animate.presentation.site"));
            assert_eq!(manifest.pointer("/player/wasm").and_then(|v| v.as_str()), Some("/animate/plugin/wasm/animate_plugin_bg.wasm"));
            let deck_value = dsl::os_pack::json::parse(&fs::read_to_string(output.join("deck.json")).expect("deck.json")).expect("json");
            let deck_file: PresentationSnapshot = dsl::FromValue::from_value(dsl::os_pack::json::to_dsl_value(&deck_value)).expect("deck");
            assert_eq!(crate::artifacts::presentation::presentation_working_scene(&deck_file).1.len(), 4);
            let _ = fs::remove_dir_all(&output);
        }

        /// 🧪️ Native/host-only: `compile_scene_to_assets` renders real frames through
        /// `renderer::VelloRenderer`, which always reports "no adapter" on `wasm32-wasip2` by
        /// design — see `⚙️engine/🎥️video/🦀️.rs`'s `renderer` module.
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        #[semio_framework_async_macros::async_test]
        async fn compile_scene_to_assets_writes_mp4() {
            let output = std::env::temp_dir().join(format!("animate-scene-assets-{}", std::process::id()));
            let _ = fs::remove_dir_all(&output);
            let bundle = compile_scene_to_assets("demo123", &output).await.expect("compile scene");
            assert_eq!(bundle.scene_hash, "demo123");
            assert!(bundle.mp4.as_ref().is_some_and(|path| path.exists()));
            let _ = fs::remove_dir_all(&output);
        }
    }
}

pub mod slide {
    //! 🎭️ Scene-based presentation document types for slide/section timelines.

    use crate::artifacts::presentation::PresentationSnapshot;
    use crate::editor::animate::engine::scene::section::Section;

    pub const PRESENTATION_SCENE_SCHEMA: &str = "animate.presentation.scene";

    /// 🖼️ One slide within a presentation section — may reference a compiled animate scene hash.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    pub struct PresentationSlide {
        pub id: String,
        pub title: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub scene_hash: Option<String>,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        pub timeline_sections: Vec<Section>,
    }

    /// 📚️ Vertical column of slides (reveal.js sequence analogue).
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    pub struct PresentationSection {
        pub id: String,
        pub title: String,
        pub slides: Vec<PresentationSlide>,
    }

    /// 🎬️ Full scene-based presentation document — sections of slides plus optional tile deck overlay.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    pub struct PresentationScene {
        pub schema: String,
        pub title: String,
        pub sections: Vec<PresentationSection>,
        #[value(skip_serializing_if = "Option::is_none")]
        pub deck: Option<PresentationSnapshot>,
    }

    impl PresentationScene {
        pub fn empty(title: impl Into<String>) -> Self {
            Self { schema: PRESENTATION_SCENE_SCHEMA.into(), title: title.into(), sections: Vec::new(), deck: None }
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

        #[semio_framework_async_macros::async_test]
        async fn presentation_scene_counts_slides() {
            let scene = PresentationScene {
                schema: PRESENTATION_SCENE_SCHEMA.into(),
                title: "Demo".into(),
                sections: vec![PresentationSection {
                    id: "s1".into(),
                    title: "Intro".into(),
                    slides: vec![
                        PresentationSlide { id: "a".into(), title: "A".into(), scene_hash: None, timeline_sections: Vec::new() },
                        PresentationSlide { id: "b".into(), title: "B".into(), scene_hash: Some("abc123".into()), timeline_sections: vec![Section::new("main", 0.0, 5.0)] },
                    ],
                }],
                deck: None,
            };
            assert_eq!(scene.slide_count(), 2);
            assert_eq!(scene.scene_hashes(), vec!["abc123".to_string()]);
        }
    }
}

pub use compiler::{compile_presentation_site, compile_scene_to_assets, PresentationCompileError, SceneAssetBundle};
pub use slide::{PresentationScene, PresentationSection, PresentationSlide, PRESENTATION_SCENE_SCHEMA};

//#region 🔖️Error
/// 🎬️ Errors from headless video export (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
/// split from the former engine-tree `PresentationError`, which mixed this app-tier video-export concern
/// with a schema-tier envelope-replay concern — see `crate::artifacts::presentation::schema::PresentationError`
/// for that half, kept where the artifact's own `materialize_presentation_projection_json` can reach it
/// without an artifact-depends-on-app violation).
#[derive(Debug)]
pub enum PresentationVideoExportError {
    /// 🎬️ The scene had no scene hashes to render.
    NoSceneHashes,
    /// 🎥️ A per-scene render/compile failed.
    Compile(PresentationCompileError),
}

impl std::fmt::Display for PresentationVideoExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSceneHashes => formatter.write_str("presentation has no scene hashes to export"),
            Self::Compile(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for PresentationVideoExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NoSceneHashes => None,
            Self::Compile(error) => std::error::Error::source(error),
        }
    }
}

impl From<PresentationCompileError> for PresentationVideoExportError {
    fn from(error: PresentationCompileError) -> Self {
        Self::Compile(error)
    }
}
//#endregion 🔖️Error

//#region 🔖️VideoExport
/// 🎬️ Renders every unique `scene_hash` referenced by a {@link PresentationScene}.
pub async fn export_video_from_scene(scene: &PresentationScene, output_dir: &std::path::Path) -> Result<Vec<SceneAssetBundle>, PresentationVideoExportError> {
    let hashes = scene.scene_hashes();
    if hashes.is_empty() {
        return Err(PresentationVideoExportError::NoSceneHashes);
    }
    let mut bundles = Vec::with_capacity(hashes.len());
    for hash in hashes {
        bundles.push(compile_scene_to_assets(&hash, output_dir).await.map_err(PresentationVideoExportError::from)?);
    }
    Ok(bundles)
}
//#endregion 🔖️VideoExport
