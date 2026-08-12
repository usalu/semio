//! ⚙️ Shooting artifact — headless compute over the `ShootingSnapshot` projection (constitutional:
//! engine). The rule for what lands here rather than next to a single caller: a helper with MORE THAN
//! ONE consumer across the taxonomy tree lives here; a helper with exactly one consumer lives in that
//! consumer's own component file.

use crate::artifacts::shooting::{empty_shooting_snapshot, ShootingAsset, ShootingCamera, ShootingSnapshot, ShootingShot};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::write_svg_xml;
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;
use serde_json::{json, Value};

//#region 🔖️Constants
//#endregion 🔖️Constants

//#region 🔖️Register
/// 🗂️ Registers the SVG/DWG media handlers and the document codec for the shooting app under
/// `SHOOTING_DOCUMENT_SCHEMA` so `framework/sync`'s folder endpoints and any other schema-keyed caller
/// can print/parse/export shooting documents. Called from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::shooting::composer::register();

    register_artifact_schema();
    register_pilot_languages();
    crate::apps::shooting::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::shooting::ShootingPlayApp>(crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.document",
        extension: Some("shooting"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::shooting::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::shooting::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::shooting::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::shooting::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("shooting.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::shooting::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::shooting::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::shooting::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::shooting::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("shooting.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::shooting::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::shooting::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("shooting.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::shooting::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::shooting::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("shooting.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::shooting::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::shooting::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("shooting.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifacts::shooting::artifact_kind` already declares (schema/media type/presentation fields
/// copied verbatim); the sole app-specific port is `photos:out` (see `shooting_photos_out_port` below)
/// — the implicit document in/out ports cover the rest.
///
/// ⚠️ `export_formats`/`import_formats` stay empty: `AppIo` (unlike `ArtifactKindSpec`) carries no
/// `export_stdio_kinds`/`import_stdio_kinds` string peer to hold the real `["s.stdio.svg",
/// "s.stdio.png"]` list, and its field type (a `Vec` of the framework's closed media-format enum) is
/// framework-owned (`🧰️framework/🔨️modules/🛂️manifest`), out of this plugin's write scope. Confirmed
/// dead as of this migration —
/// `app.io.export_formats`/`import_formats` have no framework reader (`app.io.all_ports()`/
/// `document_schema`/`artifact.component_kind` are the only fields anything consumes) — so emptying
/// them drops no live behavior. `crate::artifacts::shooting::artifact_kind()`'s `export_stdio_kinds`/
/// `import_stdio_kinds` remain the live source of truth for this artifact's real format list.
pub fn shooting_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: "shooting.scene".into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        ports: vec![shooting_photos_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.shooting".into(), name: "2D Shooting".into(), dimension: "2d".into(), component_kind: "shooting".into() },
    }
}

/// 🔌️ `photos:out` — the shooting document's captured photo(s), as `2d.image` raster media (workflow
/// port surface; WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe).
/// `Many`/optional: a shooting document may carry several shots, and downstream consumers (e.g.
/// remodel's `photos:in`) may connect before any shot exists.
pub fn shooting_photos_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "photos:out".into(),
        label: "Photos".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Exports the active shot's rendered scene as a `2d.image` `Media` payload for the `photos:out`
/// port — reuses the same SVG-then-rasterize pipeline (`shooting_scene_svg` +
/// `rasterize_svg_to_png_base64`) as the `exportActiveShot`/PNG shell action, so there is exactly one
/// photo renderer.
pub fn shooting_photo_media(snapshot: &ShootingSnapshot) -> Result<semio_framework_plugin::Media, semio_framework_plugin::MediaError> {
    let (svg, width, height) = shooting_scene_svg(snapshot).map_err(|error| semio_framework_plugin::MediaError::Payload("photos:out".into(), error))?;
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| semio_framework_plugin::MediaError::Payload("photos:out".into(), error))?;
    Ok(semio_framework_plugin::Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 },
    })
}
//#endregion 🔖️Io

//#region 🔖️DocumentHelpers
pub fn next_shooting_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let next = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{prefix}-{next}")
}

/// 📄️ Parses the handcrafted DSL fixture once per call — used both for the in-plugin default document
/// and to bridge into the framework's still-JSON-only `App::example` surface below, so
/// `crate::artifacts::shooting::dsl::SHOOTING_EXAMPLE_TEXT` stays the single source of truth for the
/// snapshot.
pub fn default_snapshot() -> ShootingSnapshot {
    crate::artifacts::shooting::dsl::parse_dsl(crate::artifacts::shooting::dsl::SHOOTING_EXAMPLE_TEXT).unwrap_or_else(|_| empty_shooting_snapshot())
}

/// 🌉️ JSON bridge for `semio_framework_plugin`'s `App::example` override, which hardcodes
/// `serde_json::from_str` on its `document_json` parameter (shared framework machinery, out of scope
/// for this migration) — derives the JSON from the DSL fixture rather than keeping a second, redundant
/// JSON copy of it on disk.
pub fn default_snapshot_json() -> String {
    serde_json::to_string(&default_snapshot()).unwrap_or_default()
}

pub fn active_shot(snapshot: &ShootingSnapshot) -> Option<&ShootingShot> {
    snapshot.shots.iter().find(|shot| shot.id == snapshot.active_shot_id).or_else(|| snapshot.shots.first())
}

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
fn shooting_scene_to_semio_drawing(snapshot: &ShootingSnapshot) -> (SemioDrawingSnapshot, u32, u32) {
    let shot = active_shot(snapshot);
    let asset = active_asset(snapshot);
    let (width, height) = shot.map_or((256, 256), |entry| (entry.width, entry.height));
    let shape = shot.map_or("rectangle", |entry| entry.shape.as_str());
    let background_hex = if snapshot.scene.background.is_empty() { "#0f172a" } else { snapshot.scene.background.as_str() };
    let background_rgba = shooting_hex_color_to_rgba(background_hex).unwrap_or(SemioRgba { r: 0.058_824, g: 0.090_196, b: 0.164_706, a: 1.0 });
    let label = asset.map_or("Untitled", |entry| entry.name.as_str());

    let mut children = vec![DrawNode::Path { segments: shooting_shape_path_segments(shape, width as f64, height as f64), style: Some("background".into()) }];
    if let Some(bytes) = snapshot.scene.emblem_base64.as_deref().filter(|data| !data.is_empty()).and_then(shooting_base64_decode) {
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

/// 🔤️ Minimal, dependency-free base64 decoder for `scene.emblem_base64` — mirrors the semio
/// drawing subset's own svg import leaf decoder (no shared crate; every leaf in this codebase
/// hand-rolls this exact algorithm rather than pull in an external dependency).
fn shooting_base64_decode(data: &str) -> Option<Vec<u8>> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let clean: Vec<u8> = data.bytes().filter(|&b| b != b'=' && !b.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        let vals: Vec<u8> = chunk.iter().map(|&b| val(b)).collect::<Option<Vec<u8>>>()?;
        let n = vals.len();
        let combined = vals.iter().fold(0u32, |acc, &v| (acc << 6) | v as u32) << ((4 - n) * 6);
        out.push((combined >> 16) as u8);
        if n > 2 { out.push((combined >> 8) as u8); }
        if n > 3 { out.push(combined as u8); }
    }
    Some(out)
}

/// 🔌️ Runs the `s.stdio.semio/v1/drawing` composer registration exactly once per process —
/// idempotent (`register_composer_entries`/`register_document_codec` both overwrite on
/// re-registration, neither panics), so this is safe to call regardless of whether the hosting
/// OS/plugin runtime already ran stdio's own boot-time `plugin()` registration first.
fn shooting_ensure_semio_drawing_bridge_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::composer::register);
}

/// 🌉️ `SemioDrawingSnapshot` → real SVG text, entirely through stdio's registered
/// `s.stdio.semio/v1/drawing` → `s.stdio.svg/1.1/*` composer entry (`io_dispatch`) — never a
/// hand-rolled SVG string. Returns the exporter's own `write_svg_xml` output (raw `<svg>…</svg>`
/// markup, no semio envelope preamble) so callers can hand it straight to
/// `rasterize_svg_to_png_base64`/embed it in an `<img>`, exactly like the old `wrap_svg` output did.
fn shooting_drawing_to_svg_text(drawing: &SemioDrawingSnapshot) -> Result<String, String> {
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
    let composed = semio_framework_plugin::io_dispatch(&key, std::slice::from_ref(&source)).map_err(|error| error.message)?;
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
pub fn shooting_scene_svg(snapshot: &ShootingSnapshot) -> Result<(String, u32, u32), String> {
    let (drawing, width, height) = shooting_scene_to_semio_drawing(snapshot);
    let svg = shooting_drawing_to_svg_text(&drawing)?;
    Ok((svg, width, height))
}

pub fn shooting_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let snapshot: ShootingSnapshot = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    shooting_scene_svg(&snapshot)
}

/// 🖼️ Builds the icon-render host request JSON for `shot`/`asset` under `fixture`'s scene lighting —
/// consumed both by the icon window's `render()` and by the `exportActiveShot`/`exportAllShots` shell
/// commands (`🎮️commands/🖨️export`), two consumers.
pub fn shooting_icon_render_request_json(snapshot: &ShootingSnapshot, shot: &ShootingShot, asset: &ShootingAsset, fallback_camera: &ShootingCamera) -> String {
    let camera = crate::artifacts::shooting::shooting_resolve_shot_camera(snapshot, shot, fallback_camera);
    let scene = &snapshot.scene;
    let mut camera_value = json!({
        "position": camera.position,
        "target": camera.target,
        "zoom": camera.zoom,
        "fov": camera.fov,
    });
    if let (Some(object), Some(up)) = (camera_value.as_object_mut(), camera.up) {
        object.insert("up".into(), json!(up));
    }
    let mut value = json!({
        "assetUrl": asset.url,
        "camera": camera_value,
        "lights": {
            "ambientIntensity": scene.ambient.intensity,
            "ambientColor": scene.ambient.color,
            "sunAzimuth": scene.sun.azimuth,
            "sunElevation": scene.sun.elevation,
            "sunIntensity": scene.sun.intensity,
            "sunColor": scene.sun.color,
        },
        "width": shot.width,
        "height": shot.height,
        "format": shot.format,
        "shape": if shot.shape == "ellipse" { "ellipse" } else { "rectangle" },
        "shadowEnabled": scene.shadow.enabled,
        "material": {
            "color": scene.material.color,
            "metalness": scene.material.metalness,
            "roughness": scene.material.roughness,
            "emissive": scene.material.emissive,
            "emissiveIntensity": scene.material.emissive_intensity,
        },
    });
    if let Some(object) = value.as_object_mut() {
        let background = shot.background.clone().unwrap_or_else(|| scene.background.clone());
        if !is_transparent_shooting_background(&background) {
            object.insert("background".into(), json!(background));
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
pub fn shooting_document_json_from_dwg(_drawing: &semio_framework_plugin::DwgDrawing) -> Result<Value, String> {
    serde_json::to_value(default_snapshot()).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaImport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA;

    #[test]
    fn default_example_fixture_parses() {
        let snapshot = default_snapshot();
        assert_eq!(snapshot.schema, SHOOTING_DOCUMENT_SCHEMA);
        assert!(!snapshot.shots.is_empty());
        assert!(!snapshot.assets.is_empty());
    }

    #[test]
    fn shooting_io_mirrors_the_declared_artifact_kind() {
        let io = shooting_io();
        assert_eq!(io.document_schema, "shooting.scene");
        assert_eq!(io.artifact.id, "2d.shooting");
        // 🗂️ `AppIo` has no `export_stdio_kinds`/`import_stdio_kinds` string peer (see `shooting_io`'s
        // doc comment) — the real format list lives on `artifact_kind()` instead, asserted below.
        assert_eq!(io.export_formats.len(), 0);
        assert_eq!(io.import_formats.len(), 0);
        let kind = crate::artifacts::shooting::artifact_kind();
        assert_eq!(kind.export_stdio_kinds, kind.import_stdio_kinds);
        assert!(kind.export_stdio_kinds.contains(&"stdio.svg"));
        assert!(kind.export_stdio_kinds.contains(&"stdio.png"));
    }

    /// 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe:
    /// `photos:out` is declared, optional/`Many`, and pinned to the `2d.image` kind.
    #[test]
    fn shooting_io_declares_the_photos_out_port() {
        let io = shooting_io();
        let port = io.ports.iter().find(|port| port.id == "photos:out").expect("photos:out declared");
        assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(port.kind_id.as_deref(), Some("2d.image"));
        assert!(!port.required);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert_eq!(port.media_type.class, semio_framework_plugin::MediaClass::TwoD);
        assert_eq!(port.media_type.form, semio_framework_plugin::MediaForm::Raster);
    }

    /// 🖼️ `shooting_photo_media` renders the same scene as `exportActiveShot`'s PNG (base64, non-empty).
    #[test]
    fn shooting_photo_media_exports_a_raster_2d_image() {
        let snapshot = default_snapshot();
        let media = shooting_photo_media(&snapshot).expect("photo export succeeds");
        assert_eq!(media.media_type.class, semio_framework_plugin::MediaClass::TwoD);
        assert_eq!(media.media_type.form, semio_framework_plugin::MediaForm::Raster);
        match media.payload {
            semio_framework_plugin::MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "2d.image");
                assert!(!json.is_empty());
            }
            semio_framework_plugin::MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[test]
    fn scene_svg_embeds_active_asset_name_and_shot_shape() {
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
    #[test]
    fn ellipse_shot_shape_renders_via_svg_arc_commands() {
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

    #[test]
    fn export_svg_uses_scene_render_not_title_card() {
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
    #[test]
    fn dwg_import_stays_schema_valid_for_a_non_trivial_extent() {
        let drawing = semio_framework_plugin::DwgDrawing { extmin: [0.0, 0.0, 0.0], extmax: [100.0, 200.0, 0.0], ..Default::default() };
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors");
        let snapshot: ShootingSnapshot = serde_json::from_value(document).expect("schema-valid snapshot");
        assert_eq!(snapshot.schema, SHOOTING_DOCUMENT_SCHEMA);
        assert!(!snapshot.shots.is_empty());
    }

    #[test]
    fn dwg_import_never_errors_on_empty_drawing() {
        let drawing = semio_framework_plugin::DwgDrawing::default();
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors on empty drawing");
        let snapshot: ShootingSnapshot = serde_json::from_value(document).expect("schema-valid fixture");
        assert_eq!(snapshot.schema, SHOOTING_DOCUMENT_SCHEMA);
    }

    #[test]
    fn transparent_background_predicate_covers_empty_and_literal_transparent() {
        assert!(is_transparent_shooting_background(""));
        assert!(is_transparent_shooting_background("transparent"));
        assert!(!is_transparent_shooting_background("#000000"));
    }
}
//#endregion 🧪️Tests


//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent shooting artifact engine — owns the projection; every transition is a mutation.
pub struct ShootingEngine {
    artifact: crate::artifacts::shooting::schema::ShootingArtifact,
    snapshot: crate::artifacts::shooting::ShootingSnapshot,
}

impl ShootingEngine {
    /// 🏗️ Seeds the engine from a persisted snapshot.
    pub fn new(snapshot: crate::artifacts::shooting::ShootingSnapshot) -> Self {
        let artifact = crate::artifacts::shooting::schema::ShootingArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    /// 📸️ Consumes the engine and returns its persisted snapshot.
    pub fn into_snapshot(self) -> crate::artifacts::shooting::ShootingSnapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️SchemaRegistry
/// 📌️ Registers the twenty handcrafted schema leaves for `s.shooting.shooting`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::shooting::schema::shooting_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry
