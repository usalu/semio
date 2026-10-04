//! ⚙️ Animate presentation app engine — the app's own stateful host over the artifact's pure
//! `PresentationSnapshot` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: relocated wholesale
//! from the deleted artifact-tree `⚙️engine` — an artifact is a schema + io, never an engine; behaviour
//! belongs to the app that edits it). Hosts the static-site compiler (`compiler`, real filesystem
//! writes), the scene-based presentation document types (`🎞️slide`), and headless video export
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

    use crate::editor::animate::engine::config::config::{AnimateConfig, QualityPreset};
    use crate::editor::animate::engine::video::{render_scene, scene_for_hash, OutputFormat};
    use crate::PresentationSnapshot;
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
        let deck_value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(deck));
        let deck_json = semio_framework_pack_json::to_string_pretty(&deck_value);
        fs::write(output_dir.join("deck.json"), &deck_json).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        let index_snapshot = index_html_snapshot(&deck_json);
        let index_text = semio_s_artifact_stdio_html::standards::v5::subsets::any::schema::snapshot::write_html_document(&index_snapshot);
        fs::write(output_dir.join("🌐️.html"), index_text).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("styles.css"), styles_css()).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("manifest.json"), semio_framework_pack_json::to_string_pretty(&site_manifest(deck))).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        fs::write(output_dir.join("player.js"), player_boot_js()).map_err(|error| PresentationCompileError::new(error.to_string()))?;
        Ok(())
    }

    fn site_manifest(deck: &PresentationSnapshot) -> semio_framework_pack_json::Value {
        let (_, tiles) = crate::presentation_working_scene(deck);
        semio_framework_pack_json::object([
            ("schema".to_string(), semio_framework_pack_json::Value::from("animate.presentation.site")),
            ("deckSchema".to_string(), semio_framework_pack_json::Value::from(deck.schema.clone())),
            ("title".to_string(), semio_framework_pack_json::Value::from(tiles.first().map_or("Animate Presentation", |tile| tile.name.as_str()))),
            ("tileCount".to_string(), semio_framework_pack_json::Value::from(tiles.len())),
            (
                "player".to_string(),
                semio_framework_pack_json::object([
                    ("kind".to_string(), semio_framework_pack_json::Value::from("wgpu")),
                    ("wasm".to_string(), semio_framework_pack_json::Value::from("/animate/plugin/wasm/animate_plugin_bg.wasm")),
                    ("js".to_string(), semio_framework_pack_json::Value::from("/animate/plugin/wasm/semio_s_plugin_animate.js")),
                    ("boot".to_string(), semio_framework_pack_json::Value::from("/animate/plugin/wasm/🟨️boot.js")),
                ]),
            ),
            (
                "assets".to_string(),
                semio_framework_pack_json::object([
                    ("deck".to_string(), semio_framework_pack_json::Value::from("deck.json")),
                    ("styles".to_string(), semio_framework_pack_json::Value::from("styles.css")),
                    ("player".to_string(), semio_framework_pack_json::Value::from("player.js")),
                    ("scenes".to_string(), semio_framework_pack_json::Value::from("scenes")),
                ]),
            ),
        ])
    }

    /// 🌐️ Builds `🌐️.html`'s real `HtmlSnapshot` — deck JSON lands verbatim inside the
    /// `<script>` tag's `RawText` node (HTML5's RAWTEXT content model never entity-decodes script
    /// content, so this is MORE spec-correct than the deleted emitter's `&`/`<` string-replace,
    /// which would have literally corrupted any deck JSON string containing those characters once
    /// a real browser DOM read it back via `textContent`).
    fn index_html_snapshot(deck_json: &str) -> semio_s_artifact_stdio_html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot {
        use semio_s_artifact_stdio_html::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode, HtmlSnapshot, RawTextKind, STDIO_HTML_DOCUMENT_SCHEMA};

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
    include!("🧪️tests/🔬️compiler-unit/🦀️.rs");
}

pub mod slide {
    //! 🎭️ Scene-based presentation document types for slide/section timelines.

    use crate::editor::animate::engine::scene::section::Section;
    use crate::PresentationSnapshot;

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
    include!("🧪️tests/🔬️slide-unit/🦀️.rs");
}

pub use compiler::{compile_presentation_site, compile_scene_to_assets, PresentationCompileError, SceneAssetBundle};
pub use slide::{PresentationScene, PresentationSection, PresentationSlide, PRESENTATION_SCENE_SCHEMA};

//#region 🔖️Error
/// 🎬️ Why a presentation deck or scene cannot become a host video program (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
/// split from the former engine-tree `PresentationError`; the schema-tier half is
/// `crate::standards::v1::subsets::any::schema::PresentationError`).
#[derive(Debug, PartialEq, Eq)]
pub enum PresentationVideoExportError {
    /// 🎬️ The scene had no scene hashes to render.
    NoSceneHashes,
    /// 📄️ The deck's source is not a picture a host canvas can draw (a PDF page, or no source at all).
    SourceKind { kind: String },
    /// 🚦️ The program would be refused by every host (`VideoRenderProgram::validate`).
    Program(semio_framework::kernel::VideoRenderProgramError),
}

impl PresentationVideoExportError {
    /// 🔤️ The fault code a command answers this error with.
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoSceneHashes => "animate.video.export.no-scenes",
            Self::SourceKind { .. } => "animate.video.export.source-kind",
            Self::Program(_) => "animate.video.export.program",
        }
    }
}

impl std::fmt::Display for PresentationVideoExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSceneHashes => formatter.write_str("presentation has no scene hashes to export"),
            Self::SourceKind { kind } => write!(formatter, "the deck's source ({kind:?}) is not a picture a video can show"),
            Self::Program(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for PresentationVideoExportError {}
//#endregion 🔖️Error

//#region 🔖️VideoExport
/// ⏱️ How long the opening overview (the whole source with every tile outlined) plays.
pub const DECK_VIDEO_OVERVIEW_SECONDS: f64 = 3.0;

/// ⏱️ How long each tile's slide plays.
pub const DECK_VIDEO_SLIDE_SECONDS: f64 = 2.0;

/// 📐️ The share of the picture's shorter edge kept free around every slide.
pub const DECK_VIDEO_MARGIN: f64 = 0.05;

/// ✏️ The stroke width, in device pixels, of a tile outline on the overview.
pub const DECK_VIDEO_OUTLINE_PIXELS: f64 = 3.0;

/// 🎨️ The tile outline colour on the overview (straight RGBA).
pub const DECK_VIDEO_OUTLINE_COLOR: [f64; 4] = [1.0, 0.78, 0.2, 1.0];

/// 🎞️ The host video program of a tile deck at the Medium preset: an overview of the whole source picture with every tile
/// outlined, then one slide per tile showing the tile's crop of the picture, each fitted into the frame (aspect kept, a
/// margin around it). A still slide is one scene however long it plays, so the program stays a few kilobytes and the
/// host encodes one picture per slide. The picture is drawn by the host from `source.src`; nothing here decodes it.
pub fn video_render_program_from_deck(deck: &crate::PresentationSnapshot) -> Result<semio_framework::kernel::VideoRenderProgram, PresentationVideoExportError> {
    use crate::editor::animate::engine::config::config::{AnimateConfig, QualityPreset};
    use semio_framework::kernel::{VideoRenderImage, VideoRenderOp, VideoRenderPath, VideoRenderProgram, VideoRenderRun, VideoRenderScene, VIDEO_RENDER_PROGRAM_SCHEMA};
    let (source, tiles) = crate::presentation_working_scene(deck);
    if source.src.trim().is_empty() || source.kind == "pdf" {
        return Err(PresentationVideoExportError::SourceKind { kind: source.kind.clone() });
    }
    let config = AnimateConfig::from_quality(QualityPreset::Medium);
    let (width, height) = (f64::from(config.width), f64::from(config.height));
    let fps = config.frame_rate.round().max(1.0) as u32;
    let frames = |seconds: f64| (seconds * f64::from(fps)).round().max(1.0) as u32;
    let frame = &source.frame;
    let aspect = source.source_aspect.filter(|aspect| aspect.is_finite() && *aspect > 0.0).unwrap_or(if frame.height > 0.0 { frame.width / frame.height } else { 1.0 });
    let normalised = |value: f64, origin: f64, extent: f64| if extent > 0.0 { ((value - origin) / extent).clamp(0.0, 1.0) } else { 0.0 };
    let margin = width.min(height) * DECK_VIDEO_MARGIN;
    let fit = |crop: [f64; 4]| {
        let picture_aspect = crop[2] * aspect / crop[3].max(f64::EPSILON);
        let (room_width, room_height) = (width - 2.0 * margin, height - 2.0 * margin);
        let (fit_width, fit_height) = if picture_aspect >= room_width / room_height { (room_width, room_width / picture_aspect) } else { (room_height * picture_aspect, room_height) };
        [fit_width, 0.0, 0.0, fit_height, (width - fit_width) / 2.0, (height - fit_height) / 2.0]
    };
    let crops: Vec<[f64; 4]> = tiles
        .iter()
        .map(|tile| {
            let (x, y) = (normalised(tile.crop.x, frame.x, frame.width), normalised(tile.crop.y, frame.y, frame.height));
            let (right, bottom) = (normalised(tile.crop.x + tile.crop.width, frame.x, frame.width), normalised(tile.crop.y + tile.crop.height, frame.y, frame.height));
            [x, y, right - x, bottom - y]
        })
        .filter(|crop| crop[2] > 0.0 && crop[3] > 0.0)
        .collect();
    let overview = fit([0.0, 0.0, 1.0, 1.0]);
    let outline = |crop: &[f64; 4]| {
        let (left, top) = (overview[4] + crop[0] * overview[0], overview[5] + crop[1] * overview[3]);
        let (right, bottom) = (left + crop[2] * overview[0], top + crop[3] * overview[3]);
        VideoRenderPath { verbs: "MLLLZ".into(), points: vec![left, top, right, top, right, bottom, left, bottom] }
    };
    let identity = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    let mut overview_ops = vec![VideoRenderOp::Image { image: 0, crop: [0.0, 0.0, 1.0, 1.0], transform: overview, opacity: 1.0 }];
    overview_ops.extend((0..crops.len()).map(|index| VideoRenderOp::Stroke { path: index as u32, transform: identity, color: DECK_VIDEO_OUTLINE_COLOR, width: DECK_VIDEO_OUTLINE_PIXELS }));
    let mut scenes = vec![VideoRenderScene { ops: overview_ops }];
    scenes.extend(crops.iter().map(|crop| VideoRenderScene { ops: vec![VideoRenderOp::Image { image: 0, crop: *crop, transform: fit(*crop), opacity: 1.0 }] }));
    let mut timeline = vec![VideoRenderRun { scene: 0, frames: frames(DECK_VIDEO_OVERVIEW_SECONDS) }];
    timeline.extend((1..scenes.len()).map(|scene| VideoRenderRun { scene: scene as u32, frames: frames(DECK_VIDEO_SLIDE_SECONDS) }));
    let program = VideoRenderProgram {
        schema: VIDEO_RENDER_PROGRAM_SCHEMA.into(),
        width: config.width,
        height: config.height,
        fps,
        background: config.background.map(|value| if value.is_nan() { 0.0 } else { value.clamp(0.0, 1.0) }),
        paths: crops.iter().map(outline).collect(),
        images: vec![VideoRenderImage { url: source.src.clone() }],
        scenes,
        timeline,
    };
    program.validate().map_err(PresentationVideoExportError::Program)?;
    Ok(program)
}

/// 🎬️ The host video program of every unique `scene_hash` a {@link PresentationScene} references, in slide order,
/// at the Medium preset: each hash's scene is played and captured target-neutrally (`video::capture_scene`) and every
/// captured frame joins one program (`video::VideoProgramBuilder`). The host renders + encodes it
/// (`Effect::VideoRenderExport`); nothing here touches a GPU or a file system.
pub fn video_render_program_from_scene(scene: &PresentationScene) -> Result<semio_framework::kernel::VideoRenderProgram, PresentationVideoExportError> {
    use crate::editor::animate::engine::config::config::{AnimateConfig, QualityPreset};
    let hashes = scene.scene_hashes();
    if hashes.is_empty() {
        return Err(PresentationVideoExportError::NoSceneHashes);
    }
    let config = AnimateConfig::from_quality(QualityPreset::Medium);
    let mut builder = crate::editor::animate::engine::video::VideoProgramBuilder::new(&config);
    for hash in hashes {
        let capture = crate::editor::animate::engine::video::capture_scene(crate::editor::animate::engine::video::scene_for_hash(config.clone(), &hash), &config);
        for frame in &capture.captures {
            builder.push_capture(frame, &capture.camera, &config);
        }
    }
    let program = builder.finish();
    program.validate().map_err(PresentationVideoExportError::Program)?;
    Ok(program)
}

/// 📛️ A download file name from `title`: its ASCII letters and digits as a lowercase slug (`animate-presentation` when
/// nothing is left).
pub fn video_filename_for_title(title: &str) -> String {
    let slug: String = title.chars().map(|character| if character.is_ascii_alphanumeric() { character.to_ascii_lowercase() } else { '-' }).collect();
    let slug = slug.split('-').filter(|part| !part.is_empty()).collect::<Vec<_>>().join("-");
    format!("{}.mp4", if slug.is_empty() { "animate-presentation" } else { slug.as_str() })
}

/// 📛️ The downloaded file name of a deck's video: its source picture's file stem (`/…/🏘️habitat-67.png` → `habitat-67.mp4`).
pub fn video_filename_for_deck(deck: &crate::PresentationSnapshot) -> String {
    let (source, _) = crate::presentation_working_scene(deck);
    let stem = if source.src.starts_with("data:") { "" } else { source.src.rsplit('/').next().unwrap_or("").rsplit_once('.').map_or(source.src.rsplit('/').next().unwrap_or(""), |(stem, _)| stem) };
    video_filename_for_title(stem)
}
//#endregion 🔖️VideoExport
