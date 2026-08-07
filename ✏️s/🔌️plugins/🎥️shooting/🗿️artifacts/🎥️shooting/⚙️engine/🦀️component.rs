//! ⚙️ Shooting artifact — headless compute over the `ShootingFixture` projection (constitutional:
//! engine). The rule for what lands here rather than next to a single caller: a helper with MORE THAN
//! ONE consumer across the taxonomy tree lives here; a helper with exactly one consumer lives in that
//! consumer's own component file.

use crate::artifacts::shooting::{empty_shooting_fixture, ShootingAsset, ShootingCamera, ShootingFixture, ShootingShot};
use serde_json::{json, Value};

//#region 🔖️Constants
//#endregion 🔖️Constants

//#region 🔖️Register
/// 🗂️ Registers the SVG/DWG media handlers and the document codec for the shooting app under
/// `SHOOTING_FIXTURE_SCHEMA` so `framework/sync`'s folder endpoints and any other schema-keyed caller
/// can print/parse/export shooting documents. Called from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    register_pilot_languages();
    semio_framework_os::register_2d_export_handlers("2d.shooting", "shooting", shooting_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.shooting", shooting_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::shooting::ShootingPlayApp>(crate::artifacts::shooting::SHOOTING_FIXTURE_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.document",
        extension: Some("shooting"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::artifacts::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::artifacts::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::artifacts::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::artifacts::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("shooting.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::artifacts::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::artifacts::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::artifacts::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::artifacts::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("shooting.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::artifacts::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::artifacts::diff::COMPONENT_GRAMMAR_PATH),
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
        protocol: Some(crate::artifacts::artifacts::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::artifacts::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("shooting.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "shooting.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::artifacts::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::artifacts::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("shooting.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifacts::shooting::artifact_kind` already declares (schema/media type/export+import
/// formats/presentation fields copied verbatim); the sole app-specific port is `photos:out` (see
/// `shooting_photos_out_port` below) — the implicit document in/out ports cover the rest.
pub fn shooting_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: "shooting.scene".into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        ports: vec![shooting_photos_out_port()],
        export_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
        import_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
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
        multiplicity: semio_framework_core::PortMultiplicity::Many,
    }
}

/// 🖼️ Exports the active shot's rendered scene as a `2d.image` `Media` payload for the `photos:out`
/// port — reuses the same SVG-then-rasterize pipeline (`shooting_scene_svg` +
/// `rasterize_svg_to_png_base64`) as the `exportActiveShot`/PNG shell action, so there is exactly one
/// photo renderer.
pub fn shooting_photo_media(fixture: &ShootingFixture) -> Result<semio_framework_plugin::Media, semio_framework_plugin::MediaError> {
    let (svg, width, height) = shooting_scene_svg(fixture);
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| semio_framework_plugin::MediaError::Payload("photos:out".into(), error))?;
    Ok(semio_framework_plugin::Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 },
    })
}
//#endregion 🔖️Io

//#region 🔖️DocumentHelpers
pub fn next_shooting_id(prefix: &str) -> String {
    let next = {
        let hex = blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex();
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("{prefix}-{next}")
}

/// 📄️ Parses the handcrafted DSL fixture once per call — used both for the in-plugin default document
/// and to bridge into the framework's still-JSON-only `App::example` surface below, so
/// `crate::artifacts::shooting::dsl::SHOOTING_EXAMPLE_TEXT` stays the single source of truth for the
/// fixture.
pub fn default_fixture() -> ShootingFixture {
    crate::artifacts::shooting::dsl::parse_dsl(crate::artifacts::shooting::dsl::SHOOTING_EXAMPLE_TEXT).unwrap_or_else(|_| empty_shooting_fixture())
}

/// 🌉️ JSON bridge for `semio_framework_plugin`'s `App::example` override, which hardcodes
/// `serde_json::from_str` on its `document_json` parameter (shared framework machinery, out of scope
/// for this migration) — derives the JSON from the DSL fixture rather than keeping a second, redundant
/// JSON copy of it on disk.
pub fn default_fixture_json() -> String {
    serde_json::to_string(&default_fixture()).unwrap_or_default()
}

pub fn active_shot(fixture: &ShootingFixture) -> Option<&ShootingShot> {
    fixture.shots.iter().find(|shot| shot.id == fixture.active_shot_id).or_else(|| fixture.shots.first())
}

pub fn active_asset(fixture: &ShootingFixture) -> Option<&ShootingAsset> {
    fixture.assets.iter().find(|asset| asset.id == fixture.active_asset_id).or_else(|| fixture.assets.first())
}

/// 🌫️ A background of `""`/`"transparent"` means "let the surface show through" — shared by the scene
/// window's environment JSON and the icon-render request below (two consumers).
pub fn is_transparent_shooting_background(background: &str) -> bool {
    background.is_empty() || background == "transparent"
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaExport
fn escape_svg_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// 🖼️ Renders the active shot as an SVG emblem — shot shape as the clip, the emblem override
/// or asset name as the payload — instead of a generic title card.
pub fn shooting_scene_svg(fixture: &ShootingFixture) -> (String, u32, u32) {
    let shot = active_shot(fixture);
    let asset = active_asset(fixture);
    let (width, height) = shot.map_or((256, 256), |entry| (entry.width, entry.height));
    let shape = shot.map_or("rectangle", |entry| entry.shape.as_str());
    let background = if fixture.scene.background.is_empty() { "#0f172a" } else { fixture.scene.background.as_str() };
    let clip = if shape == "ellipse" {
        format!("<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"{background}\"/>", cx = width as f64 / 2.0, cy = height as f64 / 2.0, rx = width as f64 / 2.0, ry = height as f64 / 2.0,)
    } else {
        format!("<rect width=\"100%\" height=\"100%\" fill=\"{background}\"/>")
    };
    let emblem = fixture
        .scene
        .emblem_base64
        .as_ref()
        .filter(|data| !data.is_empty())
        .map(|data| format!("<image href=\"data:image/png;base64,{data}\" x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" preserveAspectRatio=\"xMidYMid meet\"/>"))
        .unwrap_or_default();
    let label = asset.map_or("Untitled", |entry| entry.name.as_str());
    let font_size = (height as f64 * 0.09).max(10.0);
    let text = format!("<text x=\"50%\" y=\"{y}\" font-size=\"{font_size}\" fill=\"white\" text-anchor=\"middle\" font-family=\"sans-serif\">{label}</text>", y = height as f64 * 0.92, label = escape_svg_text(label),);
    semio_framework_os::wrap_svg(width, height, &format!("{clip}{emblem}{text}"))
}

pub fn shooting_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let fixture: ShootingFixture = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    Ok(shooting_scene_svg(&fixture))
}

/// 🖼️ Builds the icon-render host request JSON for `shot`/`asset` under `fixture`'s scene lighting —
/// consumed both by the icon window's `render()` and by the `exportActiveShot`/`exportAllShots` shell
/// commands (`🎮️commands/🖨️export`), two consumers.
pub fn shooting_icon_render_request_json(fixture: &ShootingFixture, shot: &ShootingShot, asset: &ShootingAsset, fallback_camera: &ShootingCamera) -> String {
    let camera = crate::artifacts::shooting::shooting_resolve_shot_camera(fixture, shot, fallback_camera);
    let scene = &fixture.scene;
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
    serde_json::to_value(default_fixture()).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaImport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::shooting::SHOOTING_FIXTURE_SCHEMA;

    #[test]
    fn default_example_fixture_parses() {
        let fixture = default_fixture();
        assert_eq!(fixture.schema, SHOOTING_FIXTURE_SCHEMA);
        assert!(!fixture.shots.is_empty());
        assert!(!fixture.assets.is_empty());
    }

    #[test]
    fn shooting_io_mirrors_the_declared_artifact_kind() {
        let io = shooting_io();
        assert_eq!(io.document_schema, "shooting.scene");
        assert_eq!(io.artifact.id, "2d.shooting");
        assert_eq!(io.export_formats.len(), 2);
        assert_eq!(io.import_formats.len(), 2);
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
        assert_eq!(port.multiplicity, semio_framework_core::PortMultiplicity::Many);
        assert_eq!(port.media_type.class, semio_framework_plugin::MediaClass::TwoD);
        assert_eq!(port.media_type.form, semio_framework_plugin::MediaForm::Raster);
    }

    /// 🖼️ `shooting_photo_media` renders the same scene as `exportActiveShot`'s PNG (base64, non-empty).
    #[test]
    fn shooting_photo_media_exports_a_raster_2d_image() {
        let fixture = default_fixture();
        let media = shooting_photo_media(&fixture).expect("photo export succeeds");
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
        let fixture = default_fixture();
        let (svg, width, height) = shooting_scene_svg(&fixture);
        let shot = active_shot(&fixture).expect("default fixture shot");
        let asset = active_asset(&fixture).expect("default fixture asset");
        assert_eq!((width, height), (shot.width, shot.height));
        assert!(svg.contains(&asset.name), "svg emblem includes active asset name");
        assert!(if shot.shape == "ellipse" { svg.contains("<ellipse") } else { svg.contains("<rect") });
    }

    #[test]
    fn export_svg_uses_scene_render_not_title_card() {
        let fixture = default_fixture();
        let document = serde_json::to_value(&fixture).unwrap();
        let (svg, _width, _height) = shooting_document_json_to_svg(&document).expect("export svg");
        let asset = active_asset(&fixture).expect("default fixture asset");
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
        let fixture: ShootingFixture = serde_json::from_value(document).expect("schema-valid fixture");
        assert_eq!(fixture.schema, SHOOTING_FIXTURE_SCHEMA);
        assert!(!fixture.shots.is_empty());
    }

    #[test]
    fn dwg_import_never_errors_on_empty_drawing() {
        let drawing = semio_framework_plugin::DwgDrawing::default();
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors on empty drawing");
        let fixture: ShootingFixture = serde_json::from_value(document).expect("schema-valid fixture");
        assert_eq!(fixture.schema, SHOOTING_FIXTURE_SCHEMA);
    }

    #[test]
    fn transparent_background_predicate_covers_empty_and_literal_transparent() {
        assert!(is_transparent_shooting_background(""));
        assert!(is_transparent_shooting_background("transparent"));
        assert!(!is_transparent_shooting_background("#000000"));
    }
}
//#endregion 🧪️Tests
