//! 🧬️ Shooting artifact schema — every field of the artifact with its state class.

use crate::artifacts::shooting::{ShootingAsset, ShootingCamera, ShootingEmblemChild, ShootingSavedCamera, ShootingSceneLighting, ShootingShot, ShootingSnapshot};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::write_svg_xml;
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;
use dsl::json;
use dsl::os_pack::json::Value;

//#region 🔖️Artifact
/// 🧬️ Full shooting artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub assets: Vec<ShootingAsset>,
    #[state(artifact)]
    pub saved_cameras: Vec<ShootingSavedCamera>,
    #[state(artifact)]
    pub scene: ShootingSceneLighting,
    #[state(artifact)]
    pub shots: Vec<ShootingShot>,
    #[state(artifact)]
    pub active_shot_id: String,
    #[state(artifact)]
    pub active_asset_id: String,
    /// 🕸️ Composed `s.stdio.semio.image` child mirror — see `ShootingSnapshot::emblem`'s doc comment.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.image")]
    pub emblem: Option<ShootingEmblemChild>,
    #[state(presence)]
    pub selected_shot_ids: Vec<String>,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(config)]
    pub default_shot_format: String,
    #[state(config)]
    pub default_shot_shape: String,
    #[state(config)]
    pub default_asset_format: String,
    #[state(config)]
    pub center_model: bool,
    #[state(config)]
    pub fit_revision: u32,
    #[state(config)]
    pub camera_draft_label: String,
    #[state(config)]
    pub camera: ShootingCamera,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ShootingArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA.into(),
            assets: Vec::new(),
            saved_cameras: Vec::new(),
            scene: ShootingSceneLighting::default(),
            shots: Vec::new(),
            active_shot_id: String::new(),
            active_asset_id: String::new(),
            emblem: None,
            selected_shot_ids: Vec::new(),
            active_utility_id: "move".into(),
            default_shot_format: "png".into(),
            default_shot_shape: "rectangle".into(),
            default_asset_format: "glb".into(),
            center_model: true,
            fit_revision: 0,
            camera_draft_label: String::new(),
            camera: ShootingCamera::default(),
            locale: "en-US".into(),
        }
    }
}

impl ShootingArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> ShootingSnapshot {
        ShootingSnapshot {
            schema: self.schema.clone(),
            assets: self.assets.clone(),
            saved_cameras: self.saved_cameras.clone(),
            scene: self.scene.clone(),
            shots: self.shots.clone(),
            active_shot_id: self.active_shot_id.clone(),
            active_asset_id: self.active_asset_id.clone(),
            emblem: self.emblem.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: ShootingSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            assets: snapshot.assets,
            saved_cameras: snapshot.saved_cameras,
            scene: snapshot.scene,
            shots: snapshot.shots,
            active_shot_id: snapshot.active_shot_id,
            active_asset_id: snapshot.active_asset_id,
            emblem: snapshot.emblem,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: ShootingSnapshot) {
        self.schema = snapshot.schema;
        self.assets = snapshot.assets;
        self.saved_cameras = snapshot.saved_cameras;
        self.scene = snapshot.scene;
        self.shots = snapshot.shots;
        self.active_shot_id = snapshot.active_shot_id;
        self.active_asset_id = snapshot.active_asset_id;
        self.emblem = snapshot.emblem;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️DocumentHelpers
/// 🔢️ Mints a fresh, process-unique id (`"{prefix}-{n}"`) — shared by every mutation that creates a
/// new shot/asset/saved-camera record.
pub async fn next_shooting_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let next = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{prefix}-{next}")
}

/// 📄️ Parses the handcrafted DSL fixture once per call — used both for the in-plugin default document
/// and to bridge into the framework's still-JSON-only `App::example` surface, so
/// `crate::artifacts::shooting::dsl::SHOOTING_EXAMPLE_TEXT` stays the single source of truth for the
/// snapshot.
pub async fn default_snapshot() -> ShootingSnapshot {
    crate::artifacts::shooting::dsl::parse_dsl(crate::artifacts::shooting::dsl::SHOOTING_EXAMPLE_TEXT).unwrap_or_else(|_| crate::artifacts::shooting::empty_shooting_snapshot())
}

/// 🌉️ JSON bridge for `semio_framework_plugin`'s `App::example` override, which hardcodes
/// `serde_json::from_str` on its `document_json` parameter (shared framework machinery, out of scope
/// for this migration) — derives the JSON from the DSL fixture rather than keeping a second, redundant
/// JSON copy of it on disk.
pub async fn default_snapshot_json() -> String {
    dsl::os_pack::json::to_json_string(&default_snapshot())
}

/// 📸️ The active shot — falls back to the first shot when `active_shot_id` names nothing (an empty
/// document, or a stale id left over after a delete).
pub async fn active_shot(snapshot: &ShootingSnapshot) -> Option<&ShootingShot> {
    snapshot.shots.iter().find(|shot| shot.id == snapshot.active_shot_id).or_else(|| snapshot.shots.first())
}

/// 📦️ The active asset — same fallback rule as `active_shot`.
pub async fn active_asset(snapshot: &ShootingSnapshot) -> Option<&ShootingAsset> {
    snapshot.assets.iter().find(|asset| asset.id == snapshot.active_asset_id).or_else(|| snapshot.assets.first())
}

/// 🌫️ A background of `""`/`"transparent"` means "let the surface show through" — shared by the scene
/// window's environment JSON and the icon-render request below (two consumers).
pub async fn is_transparent_shooting_background(background: &str) -> bool {
    background.is_empty() || background == "transparent"
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaExport
/// 🌉️ Builds a real `SemioDrawingSnapshot` (canvas + one named `DrawStyle` per painted primitive
/// + a single "scene" `DrawLayer`) from the active shot/asset/scene — replaces the old hand-rolled
/// SVG string builder. The shot `shape` becomes a real `Path` (a rectangle as four `Line`
/// segments, an ellipse as two `Arc` segments — `DrawNode` has no native ellipse/rect primitive,
/// `Path` is the recursive scene graph's only drawable shape, see `shooting_shape_path_segments`);
/// the emblem override (if any) becomes a real `Image` node (base64-decoded to raw bytes — the
/// drawing subset's own svg export leaf re-encodes them, this never touches SVG text directly);
/// the active asset's name becomes a `Text` node.
///
/// Honest lossy points versus the old hand-rolled SVG (queued as `stdio_gaps`, not worked around
/// here — see ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT
/// w5b report): (1) `DrawNode::Text` carries no font-size/text-anchor/font-family field, so the
/// label now renders left/top-anchored at the browser's default size instead of centered/bold
/// like before; (2) `DrawCanvas.background` is captured here for round-trip fidelity but the
/// `s.stdio.semio/v1/drawing` → svg export leaf never reads it — the background is therefore
/// painted as an explicit filled `Path` layer child instead, which the export leaf DOES lower
/// into real SVG markup.
async fn shooting_scene_to_semio_drawing(snapshot: &ShootingSnapshot) -> (SemioDrawingSnapshot, u32, u32) {
    let shot = active_shot(snapshot);
    let asset = active_asset(snapshot);
    let (width, height) = shot.map_or((256, 256), |entry| (entry.width, entry.height));
    let shape = shot.map_or("rectangle", |entry| entry.shape.as_str());
    let background_hex = if snapshot.scene.background.is_empty() { "#0f172a" } else { snapshot.scene.background.as_str() };
    let background_rgba = shooting_hex_color_to_rgba(background_hex).unwrap_or(SemioRgba { r: 0.058_824, g: 0.090_196, b: 0.164_706, a: 1.0 });
    let label = asset.map_or("Untitled", |entry| entry.name.as_str());

    let mut children = vec![DrawNode::Path { segments: shooting_shape_path_segments(shape, width as f64, height as f64), style: Some("background".into()) }];
    if let Some(bytes) = crate::artifacts::shooting::shooting_emblem_bytes(snapshot).filter(|bytes| !bytes.is_empty()) {
        children.push(DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: width as f64, height: height as f64, mime: "image/png".into(), bytes });
    }
    children.push(DrawNode::Text { value: label.to_string(), at: SemioPoint2 { x: width as f64 / 2.0, y: height as f64 * 0.92 }, style: Some("label".into()) });

    let drawing = SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: width as f64, height: height as f64, background: Some(background_rgba) },
        styles: vec![
            DrawStyle { name: "background".into(), fill: Some(background_rgba), stroke: None, stroke_width: None, opacity: None },
            DrawStyle { name: "label".into(), fill: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None },
        ],
        layers: vec![DrawLayer { id: "scene".into(), name: "scene".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    };
    (drawing, width, height)
}

/// ✏️ The shot `shape` field's only two real values, lowered to real path geometry: `"ellipse"`
/// draws a full ellipse via the standard two-`A`rc-command SVG technique (`M` to the right vertex,
/// two semicircular arcs back to the same point); anything else (`"rectangle"` in practice) draws
/// the four canvas edges as `L`ine segments.
async fn shooting_shape_path_segments(shape: &str, width: f64, height: f64) -> Vec<PathSegment> {
    if shape == "ellipse" {
        let (cx, cy, rx, ry) = (width / 2.0, height / 2.0, width / 2.0, height / 2.0);
        vec![
            PathSegment::MoveTo { to: SemioPoint2 { x: cx + rx, y: cy } },
            PathSegment::ArcTo { rx, ry, x_rotation: 0.0, large_arc: true, sweep: false, to: SemioPoint2 { x: cx - rx, y: cy } },
            PathSegment::ArcTo { rx, ry, x_rotation: 0.0, large_arc: true, sweep: false, to: SemioPoint2 { x: cx + rx, y: cy } },
            PathSegment::Close,
        ]
    } else {
        vec![
            PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } },
            PathSegment::LineTo { to: SemioPoint2 { x: width, y: 0.0 } },
            PathSegment::LineTo { to: SemioPoint2 { x: width, y: height } },
            PathSegment::LineTo { to: SemioPoint2 { x: 0.0, y: height } },
            PathSegment::Close,
        ]
    }
}

/// 🎨️ `"#rrggbb"`/`"#rrggbbaa"` (the only two hex shapes the shooting document ever stores in
/// `scene.background`) into a `SemioRgba`. `None` for anything else (an empty string is handled
/// by the caller's own default-color fallback before this ever runs).
async fn shooting_hex_color_to_rgba(hex: &str) -> Option<SemioRgba> {
    let trimmed = hex.trim().trim_start_matches('#');
    let byte = |s: &str| u8::from_str_radix(s, 16).ok().map(|v| v as f32 / 255.0);
    match trimmed.len() {
        6 => Some(SemioRgba { r: byte(&trimmed[0..2])?, g: byte(&trimmed[2..4])?, b: byte(&trimmed[4..6])?, a: 1.0 }),
        8 => Some(SemioRgba { r: byte(&trimmed[0..2])?, g: byte(&trimmed[2..4])?, b: byte(&trimmed[4..6])?, a: byte(&trimmed[6..8])? }),
        _ => None,
    }
}

/// 🔌️ Runs the `s.stdio.semio/v1/drawing` composer registration exactly once per process —
/// idempotent (`register_composer_entries`/`register_document_codec` both overwrite on
/// re-registration, neither panics), so this is safe to call regardless of whether the hosting
/// OS/plugin runtime already ran stdio's own boot-time `plugin()` registration first.
async fn shooting_ensure_semio_drawing_bridge_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io::register);
}

/// 🌉️ `SemioDrawingSnapshot` → real SVG text, entirely through stdio's registered
/// `s.stdio.semio/v1/drawing` → `s.stdio.svg/1.1/*` composer entry (`io_dispatch`) — never a
/// hand-rolled SVG string. Returns the exporter's own `write_svg_xml` output (raw `<svg>…</svg>`
/// markup, no semio envelope preamble) so callers can hand it straight to
/// `rasterize_svg_to_png_base64`/embed it in an `<img>`, exactly like the old `wrap_svg` output did.
async fn shooting_drawing_to_svg_text(drawing: &SemioDrawingSnapshot) -> Result<String, String> {
    shooting_ensure_semio_drawing_bridge_registered();
    let key = semio_framework_plugin::IoKey {
        artifact_kind: "s.stdio.semio".into(),
        standard: "v1".into(),
        subset: "drawing".into(),
        direction: semio_framework_plugin::IoDirection::Export,
        format_kind: "s.stdio.svg".into(),
        format_standard: "1.1".into(),
        format_subset: "*".into(),
    };
    let source = semio_framework_plugin::ErasedComposeSource {
        dialect: semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("drawing") },
        payload: semio_framework_plugin::IoPayload::Binary(<SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(drawing)),
    };
    let composed = semio_framework_plugin::resolve_ready(semio_framework_plugin::io_dispatch(&key, std::slice::from_ref(&source))).map_err(|error| error.message)?;
    let bytes = match composed.payload {
        semio_framework_plugin::IoPayload::Binary(bytes) => bytes,
        semio_framework_plugin::IoPayload::Text(_) => return Err("s.stdio.semio/v1/drawing -> s.stdio.svg dispatch returned Text, expected Binary (ArtifactPack)".into()),
    };
    let svg_snapshot = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| error.to_string())?;
    Ok(write_svg_xml(&svg_snapshot.doc))
}

/// 🖼️ Renders the active shot as a real SVG scene — shot shape as a filled background path, the
/// emblem override (if any) as an embedded raster image, and the asset name as a text label — via
/// the `s.stdio.semio/v1/drawing` → svg stdio bridge (`shooting_scene_to_semio_drawing` +
/// `shooting_drawing_to_svg_text`), never hand-rolled SVG string formatting.
pub async fn shooting_scene_svg(snapshot: &ShootingSnapshot) -> Result<(String, u32, u32), String> {
    let (drawing, width, height) = shooting_scene_to_semio_drawing(snapshot);
    let svg = shooting_drawing_to_svg_text(&drawing)?;
    Ok((svg, width, height))
}

/// 🌉️ `shooting_scene_svg` over an already-deserialized document `Value`.
pub async fn shooting_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let dsl_value: dsl::DslValue = dsl::os_pack::json::to_dsl_value(value);
    let snapshot: ShootingSnapshot = dsl::FromValue::from_value(dsl_value).map_err(|error| error.to_string())?;
    shooting_scene_svg(&snapshot)
}

/// 🖼️ Builds the icon-render host request JSON for `shot`/`asset` under `fixture`'s scene lighting —
/// consumed both by the icon window's `render()` and by the `exportActiveShot`/`exportAllShots` shell
/// commands (`🎮️commands/🖨️export`), two consumers.
pub async fn shooting_icon_render_request_json(snapshot: &ShootingSnapshot, shot: &ShootingShot, asset: &ShootingAsset, fallback_camera: &ShootingCamera) -> String {
    let vec3 = |v: [f64; 3]| Value::from(v.iter().map(|c| Value::from(*c)).collect::<Vec<Value>>());
    let camera = crate::artifacts::shooting::shooting_resolve_shot_camera(snapshot, shot, fallback_camera);
    let scene = &snapshot.scene;
    let mut camera_value = json!({
        "position": vec3(camera.position),
        "target": vec3(camera.target),
        "zoom": camera.zoom,
        "fov": camera.fov,
    });
    if let (Some(object), Some(up)) = (camera_value.as_object_mut(), camera.up) {
        object.insert("up", vec3(up));
    }
    let mut value = json!({
        "assetUrl": asset.url.as_str(),
        "camera": camera_value,
        "lights": {
            "ambientIntensity": scene.ambient.intensity,
            "ambientColor": scene.ambient.color.as_str(),
            "sunAzimuth": scene.sun.azimuth,
            "sunElevation": scene.sun.elevation,
            "sunIntensity": scene.sun.intensity,
            "sunColor": scene.sun.color.as_str(),
        },
        "width": shot.width,
        "height": shot.height,
        "format": shot.format.as_str(),
        "shape": if shot.shape == "ellipse" { "ellipse" } else { "rectangle" },
        "shadowEnabled": scene.shadow.enabled,
        "material": {
            "color": scene.material.color.as_str(),
            "metalness": scene.material.metalness,
            "roughness": scene.material.roughness,
            "emissive": scene.material.emissive.as_str(),
            "emissiveIntensity": scene.material.emissive_intensity,
        },
    });
    if let Some(object) = value.as_object_mut() {
        let background = shot.background.clone().unwrap_or_else(|| scene.background.clone());
        if !is_transparent_shooting_background(&background) {
            object.insert("background", json!(background.as_str()));
        }
    }
    value.to_string()
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 📥️ Tier C DWG import for `2d.shooting`: the format has no wall/obstacle concept, so this always
/// returns the default studio fixture — never errors, including for a structurally empty `DwgDrawing`.
/// The camera is session-only runtime state now (never a document field — see `ShootingConfig::camera`
/// in the app's `🦀️config.rs`), and `register_dwg_import_handler`'s callback signature
/// (`&DwgDrawing -> Result<Value, String>`) has no channel back into that runtime state, so this no
/// longer reframes the camera to the drawing extent (dropped, not moved — see the ticket notes).
pub async fn shooting_document_json_from_dwg(_drawing: &semio_s_plugin_stdio::artifacts::dwg::DwgDrawing) -> Result<Value, String> {
    Ok(dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&default_snapshot())))
}
//#endregion 🔖️MediaImport

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.shooting.shooting` — twenty handcrafted schema leaves.
pub async fn shooting_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.shooting.shooting",
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
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::shooting::schema::diff::ShootingDiff;
    use crate::artifacts::shooting::schema::mutations::ShootingMutation;
    use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct ShootingBuilderConstruction {
        snapshot: ShootingSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ShootingBuilderConstruction {
        type Snapshot = ShootingSnapshot;
        type Mutation = ShootingMutation;
        type Diff = ShootingDiff;
        async fn empty() -> Self {
            Self { snapshot: ShootingSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ShootingSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ShootingSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <ShootingMutation as protocol::Mutation<ShootingSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <ShootingDiff as protocol::MutationDiff<ShootingSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::shooting::ShootingSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct ShootingParts {
        pub snapshot: Option<ShootingSnapshot>,
    }

    pub struct ShootingAnalyzerAnalysis;

    impl ArtifactAnalysis for ShootingAnalyzerAnalysis {
        type Parts = ShootingParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.shooting.shooting", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ShootingParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <ShootingSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec ShootingBuilderFacets {
        construction: ShootingBuilderConstruction,
        analysis: ShootingAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ShootingComposerComposition,
    }
    builder: ShootingBuilder,
    analyzer: ShootingAnalyzer,
    composer: ShootingComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA;

    #[semio_framework_async_macros::async_test]
    async fn default_example_fixture_parses() {
        let snapshot = default_snapshot();
        assert_eq!(snapshot.schema, SHOOTING_DOCUMENT_SCHEMA);
        assert!(!snapshot.shots.is_empty());
        assert!(!snapshot.assets.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn scene_svg_embeds_active_asset_name_and_shot_shape() {
        let snapshot = default_snapshot();
        let (svg, width, height) = shooting_scene_svg(&snapshot).expect("scene svg via the semio/drawing stdio bridge");
        let shot = active_shot(&snapshot).expect("default fixture shot");
        let asset = active_asset(&snapshot).expect("default fixture asset");
        assert_eq!((width, height), (shot.width, shot.height));
        assert!(svg.contains(&asset.name), "svg emblem includes active asset name");
        // 🌉️ The shape now lowers through the semio/drawing bridge as a real `<path d="...">`
        // (drawing's own svg export leaf has no `<rect>`/`<ellipse>` element, only `<path>`);
        // ellipse shots draw via an SVG `A`rc command, rectangle shots via straight lines only.
        assert!(svg.contains("<path"), "shape renders as a real <path> element, not a raw <rect>/<ellipse>");
        let has_arc_command = svg.contains(" A ");
        assert_eq!(has_arc_command, shot.shape == "ellipse", "ellipse shots draw an SVG arc command, rectangle shots never do");
    }

    /// 🌉️ Exercises the ellipse branch the default fixture (shape "rectangle") never hits —
    /// confirms `shooting_shape_path_segments` really emits the two-arc ellipse technique.
    #[semio_framework_async_macros::async_test]
    async fn ellipse_shot_shape_renders_via_svg_arc_commands() {
        let mut snapshot = default_snapshot();
        let shot_id = snapshot.active_shot_id.clone();
        for shot in snapshot.shots.iter_mut() {
            if shot.id == shot_id {
                shot.shape = "ellipse".into();
            }
        }
        let (svg, _width, _height) = shooting_scene_svg(&snapshot).expect("ellipse scene svg");
        assert!(svg.contains(" A "), "ellipse shape draws via SVG arc commands: {svg}");
        assert!(!svg.contains("<rect") && !svg.contains("<ellipse"), "no raw <rect>/<ellipse> element, only <path>");
    }

    #[semio_framework_async_macros::async_test]
    async fn export_svg_uses_scene_render_not_title_card() {
        let snapshot = default_snapshot();
        let document = serde_json::to_value(&snapshot).unwrap();
        let (svg, _width, _height) = shooting_document_json_to_svg(&document).expect("export svg");
        let asset = active_asset(&snapshot).expect("default fixture asset");
        assert!(svg.contains(&asset.name));
        assert!(!svg.contains("Shooting"), "export renders the real scene, not the generic title card");
    }

    /// 🎥️ The camera used to be reframed to the DWG extent here; now that it's session-only runtime
    /// state (never a document field), the import hook has no channel back into it — this asserts the
    /// surviving intent: import still succeeds and stays schema-valid for a non-trivial extent.
    #[semio_framework_async_macros::async_test]
    async fn dwg_import_stays_schema_valid_for_a_non_trivial_extent() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing { extmin: [0.0, 0.0, 0.0], extmax: [100.0, 200.0, 0.0], ..Default::default() };
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors");
        let snapshot: ShootingSnapshot = serde_json::from_value(document).expect("schema-valid snapshot");
        assert_eq!(snapshot.schema, SHOOTING_DOCUMENT_SCHEMA);
        assert!(!snapshot.shots.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn dwg_import_never_errors_on_empty_drawing() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors on empty drawing");
        let snapshot: ShootingSnapshot = serde_json::from_value(document).expect("schema-valid fixture");
        assert_eq!(snapshot.schema, SHOOTING_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn transparent_background_predicate_covers_empty_and_literal_transparent() {
        assert!(is_transparent_shooting_background(""));
        assert!(is_transparent_shooting_background("transparent"));
        assert!(!is_transparent_shooting_background("#000000"));
    }
}
//#endregion 🧪️Tests
