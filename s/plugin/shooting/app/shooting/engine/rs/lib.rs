//! ⚙️ Shooting app — headless compute (constitutional: engine).

use shooting::{default_camera_position, default_fov, empty_shooting_fixture, ShootingAsset, ShootingCamera, ShootingFixture, ShootingShot, SHOOTING_FIXTURE_SCHEMA};
use serde_json::Value;
use std::sync::atomic::{AtomicU32, Ordering};
use store::DocumentDsl;

//#region 🔖Constants
static SHOOTING_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖DocumentHelpers
pub fn next_shooting_id(prefix: &str) -> String {
    let next = SHOOTING_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

/// 📄 Parses the handcrafted DSL fixture once per call — used both for the in-plugin default document
/// and to bridge into the framework's still-JSON-only `App::example` surface below, so
/// `shooting_dsl::SHOOTING_EXAMPLE_TEXT` stays the single source of truth for the fixture.
pub fn default_fixture() -> ShootingFixture {
    ShootingFixture::parse_dsl(shooting_dsl::SHOOTING_EXAMPLE_TEXT).unwrap_or_else(|_| empty_shooting_fixture())
}

/// 🌉 JSON bridge for `semio_framework_plugin`'s `App::example` override, which hardcodes
/// `serde_json::from_str` on its `document_json` parameter (shared framework machinery, out of scope
/// for this DSL migration) — derives the JSON from the DSL fixture rather than keeping a second,
/// redundant JSON copy of it on disk.
pub fn default_fixture_json() -> String {
    serde_json::to_string(&default_fixture()).unwrap_or_default()
}

pub fn active_shot(fixture: &ShootingFixture) -> Option<&ShootingShot> {
    fixture
        .shots
        .iter()
        .find(|shot| shot.id == fixture.active_shot_id)
        .or_else(|| fixture.shots.first())
}

pub fn active_asset(fixture: &ShootingFixture) -> Option<&ShootingAsset> {
    fixture
        .assets
        .iter()
        .find(|asset| asset.id == fixture.active_asset_id)
        .or_else(|| fixture.assets.first())
}
//#endregion 🔖DocumentHelpers

//#region 🔖MediaExport
fn escape_svg_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// 🖼️ Renders the active shot as an SVG emblem — shot shape as the clip, the emblem override
/// or asset name as the payload — instead of a generic title card.
pub fn shooting_scene_svg(fixture: &ShootingFixture) -> (String, u32, u32) {
    let shot = active_shot(fixture);
    let asset = active_asset(fixture);
    let (width, height) = shot.map(|entry| (entry.width, entry.height)).unwrap_or((256, 256));
    let shape = shot.map(|entry| entry.shape.as_str()).unwrap_or("rectangle");
    let background = if fixture.scene.background.is_empty() { "#0f172a" } else { fixture.scene.background.as_str() };
    let clip = if shape == "ellipse" {
        format!(
            "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"{background}\"/>",
            cx = width as f64 / 2.0,
            cy = height as f64 / 2.0,
            rx = width as f64 / 2.0,
            ry = height as f64 / 2.0,
        )
    } else {
        format!("<rect width=\"100%\" height=\"100%\" fill=\"{background}\"/>")
    };
    let emblem = fixture
        .scene
        .emblem_base64
        .as_ref()
        .filter(|data| !data.is_empty())
        .map(|data| {
            format!(
                "<image href=\"data:image/png;base64,{data}\" x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" preserveAspectRatio=\"xMidYMid meet\"/>"
            )
        })
        .unwrap_or_default();
    let label = asset.map(|entry| entry.name.as_str()).unwrap_or("Untitled");
    let font_size = (height as f64 * 0.09).max(10.0);
    let text = format!(
        "<text x=\"50%\" y=\"{y}\" font-size=\"{font_size}\" fill=\"white\" text-anchor=\"middle\" font-family=\"sans-serif\">{label}</text>",
        y = height as f64 * 0.92,
        label = escape_svg_text(label),
    );
    semio_framework_os::wrap_svg(width, height, &format!("{clip}{emblem}{text}"))
}

pub fn shooting_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let fixture: ShootingFixture = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    Ok(shooting_scene_svg(&fixture))
}
//#endregion 🔖MediaExport

//#region 🔖MediaImport
/// 🎯 Frames a `ShootingCamera` around a DWG extent, reusing the default studio angle but
/// scaling distance to the drawing's bounding box; degenerates gracefully for an empty drawing.
fn shooting_camera_from_dwg_bounds(extmin: [f64; 3], extmax: [f64; 3]) -> ShootingCamera {
    let center = [
        (extmin[0] + extmax[0]) * 0.5,
        (extmin[1] + extmax[1]) * 0.5,
        (extmin[2] + extmax[2]) * 0.5,
    ];
    let span = [(extmax[0] - extmin[0]).abs(), (extmax[1] - extmin[1]).abs(), (extmax[2] - extmin[2]).abs()];
    let radius = span[0].max(span[1]).max(span[2]) * 0.5;
    let distance = if radius > 1e-6 { radius * 2.6 } else { 600.0 };
    let direction = default_camera_position();
    let direction_len = (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2]).sqrt().max(1e-6);
    let position = [
        center[0] + direction[0] / direction_len * distance,
        center[1] + direction[1] / direction_len * distance,
        center[2] + direction[2] / direction_len * distance,
    ];
    ShootingCamera { position, target: center, zoom: 1.0, fov: default_fov(), up: None, projection: None }
}

/// 📥 Tier C DWG import for `2d.shooting`: the format has no wall/obstacle concept, so this
/// always returns the default studio fixture with the camera reframed to the drawing extent —
/// never errors, including for a structurally empty `DwgDrawing`.
pub fn shooting_document_json_from_dwg(drawing: &semio_framework_plugin::DwgDrawing) -> Result<Value, String> {
    let mut fixture = default_fixture();
    fixture.camera = shooting_camera_from_dwg_bounds(drawing.extmin, drawing.extmax);
    serde_json::to_value(&fixture).map_err(|error| error.to_string())
}
//#endregion 🔖MediaImport

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_example_fixture_parses() {
        let fixture = default_fixture();
        assert_eq!(fixture.schema, SHOOTING_FIXTURE_SCHEMA);
        assert!(!fixture.shots.is_empty());
        assert!(!fixture.assets.is_empty());
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

    #[test]
    fn dwg_import_frames_camera_to_extent_and_stays_schema_valid() {
        let mut drawing = semio_framework_plugin::DwgDrawing::default();
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [100.0, 200.0, 0.0];
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors");
        let fixture: ShootingFixture = serde_json::from_value(document).expect("schema-valid fixture");
        assert_eq!(fixture.schema, SHOOTING_FIXTURE_SCHEMA);
        assert!(!fixture.shots.is_empty());
        assert_eq!(fixture.camera.target, [50.0, 100.0, 0.0]);
        assert_ne!(fixture.camera.position, ShootingCamera::default().position);
    }

    #[test]
    fn dwg_import_never_errors_on_empty_drawing() {
        let drawing = semio_framework_plugin::DwgDrawing::default();
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors on empty drawing");
        let fixture: ShootingFixture = serde_json::from_value(document).expect("schema-valid fixture");
        assert_eq!(fixture.schema, SHOOTING_FIXTURE_SCHEMA);
        assert_eq!(fixture.camera.target, [0.0, 0.0, 0.0]);
    }
}
//#endregion 🧪Tests
