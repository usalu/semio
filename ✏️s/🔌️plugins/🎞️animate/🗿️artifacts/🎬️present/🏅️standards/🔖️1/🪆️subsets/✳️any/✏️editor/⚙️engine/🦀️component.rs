//! ⚙️ Animate present app engine — the app's own stateful host over the artifact's pure
//! `PresentSnapshot` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: relocated wholesale
//! from the deleted artifact-tree `⚙️engine` — an artifact is a schema + io, never an engine; behaviour
//! belongs to the app that edits it). Hosts the static-site compiler (`compiler`, real filesystem
//! writes), the scene-based presentation document types (`slide`), and headless video export
//! (`🔖️VideoExport`) — plus, as sibling `<topic>/🦀️component.rs` files mirroring this taxonomy node's
//! own subdirs, the Manim-class animation core (`⏱️rate`, `🎛️config`, `🎞️animation`, `📷️camera`,
//! `🎬️scene`, `📐️geometry`, `🔤️text`) and the headless video renderer (`🎥️video`). Both were their own
//! plugin-level crates before an earlier migration; neither has a dependent outside this app, so per
//! that migration's placement rule they stay folded in here rather than becoming a plugin-level
//! `🫀️core`. The former `PresentEngine` struct (a `PresentArtifact`+`PresentSnapshot` pair with only
//! `new`/`into_snapshot`) had zero external references and no `ArtifactEngine` trait impl anywhere in
//! the plugin — deleted outright, not rehomed, per this ticket's classification rule for the norm case.

pub mod compiler {
    //! 🌐️ Headless static-site compiler for animate present decks.

    use crate::editor::animate::engine::config::config::{AnimateConfig, QualityPreset};
    use crate::editor::animate::engine::video::{render_scene, scene_for_hash, OutputFormat};
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
    /// `🌐️index.html` is built as a real `HtmlSnapshot` (typed element tree) and serialized through
    /// stdio's real HTML5 engine — the hand-rolled `format!("<!DOCTYPE html>...")` string emitter this
    /// replaced is deleted outright (`styles.css`/`player.js`/`manifest.json`/`deck.json` are plain
    /// CSS/JS/JSON sidecars, not HTML — no ad-hoc HTML codec logic lived at those sites, so they stay
    /// unchanged `fs::write`s).
    pub fn compile_present_site(deck: &PresentSnapshot, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir).map_err(|error| PresentCompileError::new(error.to_string()))?;
        let deck_json = serde_json::to_string_pretty(deck).map_err(|error| PresentCompileError::new(format!("deck json: {error}")))?;
        fs::write(output_dir.join("deck.json"), &deck_json).map_err(|error| PresentCompileError::new(error.to_string()))?;
        let index_snapshot = index_html_snapshot(&deck_json);
        let index_text = semio_s_plugin_stdio::artifacts::html::standards::v5::subsets::any::schema::snapshot::write_html_document(&index_snapshot);
        fs::write(output_dir.join("🌐️index.html"), index_text).map_err(|error| PresentCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("styles.css"), styles_css()).map_err(|error| PresentCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("manifest.json"), serde_json::to_string_pretty(&site_manifest(deck)).map_err(|error| PresentCompileError::new(error.to_string()))?).map_err(|error| PresentCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("player.js"), player_boot_js()).map_err(|error| PresentCompileError::new(error.to_string()))?;
        Ok(())
    }

    fn site_manifest(deck: &PresentSnapshot) -> serde_json::Value {
        let (_, tiles) = crate::artifacts::present::present_working_scene(deck);
        json!({
            "schema": "animate.present.site",
            "deckSchema": deck.schema,
            "title": tiles.first().map_or("Animate Present", |tile| tile.name.as_str()),
            "tileCount": tiles.len(),
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

    /// 🌐️ Builds `🌐️index.html`'s real `HtmlSnapshot` — deck JSON lands verbatim inside the
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
                el("title", Vec::new(), vec![HtmlNode::Text { text: "Animate Present".into() }]),
                el("link", vec![HtmlAttr::new("rel", "stylesheet"), HtmlAttr::new("href", "styles.css")], Vec::new()),
                el("link", vec![HtmlAttr::new("rel", "manifest"), HtmlAttr::new("href", "manifest.json")], Vec::new()),
            ],
        );
        let deck_script_children = if deck_json.is_empty() { Vec::new() } else { vec![HtmlNode::RawText { parent_kind: RawTextKind::Script, text: deck_json.to_string() }] };
        let main = el(
            "main",
            vec![HtmlAttr::new("id", "animate-present-root"), HtmlAttr::new("data-deck-schema", "animate.present.deck")],
            vec![
                el("canvas", vec![HtmlAttr::new("id", "animate-present-canvas"), HtmlAttr::new("width", "1280"), HtmlAttr::new("height", "720")], Vec::new()),
                el("script", vec![HtmlAttr::new("id", "animate-present-deck"), HtmlAttr::new("type", "text/dsl")], deck_script_children),
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
        use crate::artifacts::present::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};

        #[test]
        fn compile_present_site_writes_static_bundle() {
            let deck = default_present_snapshot();
            let (source, _) = crate::artifacts::present::present_working_scene(&deck);
            let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
            let deck = crate::artifacts::present::present_snapshot_with_tiles(&source, &tiles);
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
            assert_eq!(crate::artifacts::present::present_working_scene(&deck_file).1.len(), 4);
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

    use crate::editor::animate::engine::scene::section::Section;
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

//#region 🔖️Error
/// 🎬️ Errors from headless video export (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
/// split from the former engine-tree `PresentError`, which mixed this app-tier video-export concern
/// with a schema-tier envelope-replay concern — see `crate::artifacts::present::schema::PresentError`
/// for that half, kept where the artifact's own `materialize_present_projection_json` can reach it
/// without an artifact-depends-on-app violation).
#[derive(Debug, thiserror::Error)]
pub enum PresentVideoExportError {
    /// 🎬️ The scene had no scene hashes to render.
    #[error("presentation has no scene hashes to export")]
    NoSceneHashes,
    /// 🎥️ A per-scene render/compile failed.
    #[error(transparent)]
    Compile(#[from] PresentCompileError),
}
//#endregion 🔖️Error

//#region 🔖️VideoExport
/// 🎬️ Renders every unique `scene_hash` referenced by a {@link PresentScene}.
pub fn export_video_from_scene(scene: &PresentScene, output_dir: &std::path::Path) -> Result<Vec<SceneAssetBundle>, PresentVideoExportError> {
    let hashes = scene.scene_hashes();
    if hashes.is_empty() {
        return Err(PresentVideoExportError::NoSceneHashes);
    }
    hashes.into_iter().map(|hash| compile_scene_to_assets(&hash, output_dir).map_err(PresentVideoExportError::from)).collect()
}
//#endregion 🔖️VideoExport
