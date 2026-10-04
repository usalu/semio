//! 🧬️ GIS map viewer window-config schema — the free map camera one concrete `gis2d-view-map` window
//! retains. Source of record: `🔣️.json`.

/// 🧭️ The retained map camera, in the SAME three keys the react `TiledMapHost` sends under `camera`
/// (`MapCamera {x, y, zoom}`), so the wire camera needs no renaming on the way in or out.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct GisMapViewerCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for GisMapViewerCamera {
    /// 🌍️ The host's own default map camera. A fresh window never shows it: `render` publishes only a
    /// STORED camera, and an unstored window keeps the default scene camera the host fits to the world.
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

impl GisMapViewerCamera {
    /// ✅️ A camera is admissible when every coordinate is finite and the zoom is positive.
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.zoom.is_finite() && self.zoom > 0.0
    }

    /// 🎬️ The exact `TiledMapScene::camera_json` string this camera stands for.
    pub fn scene_camera_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(self))
    }
}

/// 🎚️ Persisted local state for ONE concrete `gis2d-view-map` viewer window: its camera, and nothing
/// else. Per WINDOW — two open map panes pan independently, and a read-only surface may not write the
/// artifact lane at all.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Copy, Debug, Default, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(layout = "lines")]
#[artifact(id = "gis.mapviewerwindowcfg")]
pub struct GisMapViewerWindowConfig {
    #[dsl(block)]
    pub camera: GisMapViewerCamera,
}
