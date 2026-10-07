//! 🧬️ Shooting artifact schema — every field of the artifact with its state class.

use crate::{ShootingEmblemChild, ShootingSnapshot};
use schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};

use semio_s_artifact_stdio_svg::SvgSnapshot;

//#region 🔖️Artifact
/// 🧬️ shooting document artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
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
    #[child(kind = "s.stdio.semio")]
    pub emblem: Option<ShootingEmblemChild>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ShootingArtifact {
    fn default() -> Self {
        Self { schema: crate::SHOOTING_DOCUMENT_SCHEMA.into(), assets: Vec::new(), saved_cameras: Vec::new(), scene: ShootingSceneLighting::default(), shots: Vec::new(), active_shot_id: String::new(), active_asset_id: String::new(), emblem: None }
    }
}

impl ShootingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> ShootingSnapshot {
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

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: ShootingSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            assets: snapshot.assets,
            saved_cameras: snapshot.saved_cameras,
            scene: snapshot.scene,
            shots: snapshot.shots,
            active_shot_id: snapshot.active_shot_id,
            active_asset_id: snapshot.active_asset_id,
            emblem: snapshot.emblem,

        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: ShootingSnapshot) {
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
pub fn next_shooting_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let next = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{prefix}-{next}")
}





/// 📸️ The active shot — falls back to the first shot when `active_shot_id` names nothing (an empty
/// document, or a stale id left over after a delete).
pub fn active_shot(snapshot: &ShootingSnapshot) -> Option<&ShootingShot> {
    snapshot.shots.iter().find(|shot| shot.id == snapshot.active_shot_id).or_else(|| snapshot.shots.first())
}

/// 📦️ The active asset — same fallback rule as `active_shot`.
pub fn active_asset(snapshot: &ShootingSnapshot) -> Option<&ShootingAsset> {
    snapshot.assets.iter().find(|asset| asset.id == snapshot.active_asset_id).or_else(|| snapshot.assets.first())
}

/// 🌫️ A background of `""`/`"transparent"` means "let the surface show through" — shared by the scene
/// window's environment JSON and the icon-render request below (two consumers).
pub fn is_transparent_shooting_background(background: &str) -> bool {
    background.is_empty() || background == "transparent"
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaExport
/// 🌉️ Builds a real `SemioDrawingSnapshot` (canvas + one named `DrawStyle` per painted primitive
/// and a single "scene" `DrawLayer`) from the active shot/asset/scene — replaces the old hand-rolled
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
pub(crate) fn shooting_scene_to_semio_drawing(snapshot: &ShootingSnapshot) -> (SemioDrawingSnapshot, u32, u32) {
    let shot = active_shot(snapshot);
    let asset = active_asset(snapshot);
    let (width, height) = shot.map_or((256, 256), |entry| (entry.width, entry.height));
    let shape = shot.map_or("rectangle", |entry| entry.shape.as_str());
    let shot_background = shot.and_then(|entry| entry.background.clone()).unwrap_or_else(|| snapshot.scene.background.clone());
    let label = asset.map_or("Untitled", |entry| entry.name.as_str());

    let mut children = Vec::new();
    let mut styles = vec![DrawStyle { name: "label".into(), fill: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None }];
    // 🖼️ The shot's `shape` is the frame every export carries, INDEPENDENT of whether that frame is
    // filled: a transparent shot still has a rectangle/ellipse outline, and the scene material's own
    // `stroke` colour is what draws it. So the shape is ALWAYS one real `Path` child (the only
    // drawable `DrawNode` — see `shooting_shape_path_segments`) and only its FILL is conditional:
    // an opaque `background` fills it, a transparent one fills it with a fully transparent colour
    // (never a missing `fill`, which SVG would render as the default opaque black).
    let canvas_background = (!is_transparent_shooting_background(&shot_background)).then(|| shooting_hex_color_to_rgba(&shot_background).unwrap_or(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }));
    children.push(DrawNode::Path { segments: shooting_shape_path_segments(shape, width as f64, height as f64), style: Some("shape".into()) });
    styles.insert(
        0,
        DrawStyle {
            name: "shape".into(),
            fill: Some(canvas_background.unwrap_or(SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })),
            stroke: shooting_hex_color_to_rgba(&snapshot.scene.material.stroke),
            stroke_width: None,
            opacity: None,
        },
    );
    if let Some(bytes) = crate::shooting_emblem_bytes(snapshot).filter(|bytes| !bytes.is_empty()) {
        children.push(DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: width as f64, height: height as f64, mime: "image/png".into(), bytes });
    }
    children.push(DrawNode::Text { value: label.to_string(), at: SemioPoint2 { x: width as f64 / 2.0, y: height as f64 * 0.92 }, style: Some("label".into()) });

    let drawing = SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: width as f64, height: height as f64, background: canvas_background },
        styles,
        layers: vec![DrawLayer { id: "scene".into(), name: "scene".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    };
    (drawing, width, height)
}

/// ✏️ The shot `shape` field's only two real values, lowered to real path geometry: `"ellipse"`
/// draws a full ellipse via the standard two-`A`rc-command SVG technique (`M` to the right vertex,
/// two semicircular arcs back to the same point); anything else (`"rectangle"` in practice) draws
/// the four canvas edges as `L`ine segments.
fn shooting_shape_path_segments(shape: &str, width: f64, height: f64) -> Vec<PathSegment> {
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
fn shooting_hex_color_to_rgba(hex: &str) -> Option<SemioRgba> {
    let trimmed = hex.trim().trim_start_matches('#');
    let byte = |s: &str| u8::from_str_radix(s, 16).ok().map(|v| v as f32 / 255.0);
    match trimmed.len() {
        6 => Some(SemioRgba { r: byte(&trimmed[0..2])?, g: byte(&trimmed[2..4])?, b: byte(&trimmed[4..6])?, a: 1.0 }),
        8 => Some(SemioRgba { r: byte(&trimmed[0..2])?, g: byte(&trimmed[2..4])?, b: byte(&trimmed[4..6])?, a: byte(&trimmed[6..8])? }),
        _ => None,
    }
}





/// 🖼️ Renders the active shot as a real SVG scene — shot shape as a filled background path, the
/// emblem override (if any) as an embedded raster image, and the asset name as a text label — via
/// the `s.stdio.semio/v1/drawing` → svg stdio bridge (`shooting_scene_to_semio_drawing` +
/// `shooting_drawing_to_svg_text`), never hand-rolled SVG string formatting.


/// 🌉️ `shooting_scene_svg` over an already-deserialized document `Value`.


/// 🖼️ Builds the icon-render host request JSON for `shot`/`asset` under `fixture`'s scene lighting —
/// consumed both by the icon window's `render()` and by the `exportActiveShot`/`exportAllShots` shell
/// commands (`🎮️commands/🖨️export`), two consumers. `fit` mirrors the scene window's centre-model
/// lane (`ShootingConfig::center_model`): the host re-targets the shot camera at the asset's bounding
/// sphere and backs off to frame it, so the icon shows what the centred scene shows.

//#endregion 🔖️MediaExport

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.shooting.shooting` — twenty handcrafted schema leaves.
pub fn shooting_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.shooting.shooting",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"), typescript: include_str!("🔺️diff/🟦️.ts"), graphql: include_str!("🔺️diff/🔗️.graphql"), json_schema: include_str!("🔺️diff/🔣️.json"), proto: include_str!("🔺️diff/🛰️.proto")
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
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

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
pub use crate::ShootingAsset;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::ShootingCamera;
pub use crate::ShootingSavedCamera;
pub use crate::ShootingSceneLighting;
pub use crate::ShootingShot;
//#endregion 🔁️Re-exports
