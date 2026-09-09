//! 🗺️ GIS 2D play app — the shared `MapHost` projection.
//!
//! 🧭️ App level (not the `gismap` artifact engine) on purpose: it needs BOTH the document and the
//! app-only view state (`Gis2dConfig`), and an artifact must never depend on an app. Every
//! `🎮️commands/*` node that has to hit-test, frame or query the live map goes through here.

use crate::editor::gis2d::config::Gis2dConfig;
use crate::schema::gis_map_descriptor_json;
use crate::GisMapSnapshot;
use semio_framework_surface::tiled_map::MapHost;
use serde_json::Value;

//#region 🔖️MapHost
/// 🗺️ Builds a `MapHost` from the document content (derived descriptor JSON) plus the config's
/// camera/render/style/LOD/selection view state.
pub fn map_host_from(document: &GisMapSnapshot, cfg: &Gis2dConfig) -> MapHost {
    let mut host = MapHost::new();
    let descriptor = gis_map_descriptor_json(document);
    let _ = host.sync_map_json(&descriptor);
    if let Ok(camera) = serde_json::from_str::<Value>(&cfg.camera_json) {
        let x = camera.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let y = camera.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let zoom = camera.get("zoom").and_then(|value| value.as_f64()).unwrap_or(1.0);
        host.set_camera(x, y, zoom);
    }
    host.set_render_mode(&cfg.render_mode);
    host.set_vector_style(&cfg.vector_style);
    host.set_lod_mode(&cfg.lod_mode);
    // 🕹️ Feature selection is framework-owned config now (ticket
    // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `MapHost::set_selection_json` was
    // deleted along with it; the surface's `MapHost::sync_interaction(granularity, ids, hovered_id)`
    // replacement is driven from `InteractionState` by the renderer glue, not from here (this
    // function only ever had `Gis2dConfig`, never `InteractionView`).
    host
}
//#endregion 🔖️MapHost

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
