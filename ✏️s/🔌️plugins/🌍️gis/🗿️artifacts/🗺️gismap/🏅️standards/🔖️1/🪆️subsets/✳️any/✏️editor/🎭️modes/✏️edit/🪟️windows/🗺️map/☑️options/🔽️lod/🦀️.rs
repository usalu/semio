//! 🔽️ Map window option — the level-of-detail tier select.
//!
//! 🧭️ Single owner of the LOD vocabulary: the tier list is derived once here from the tiled-map
//! surface crate's scale table, and reused by the inspector panel's matching field and by the
//! manifest's `setLodMode` arg schema.

use crate::editor::gis2d::maphost;
use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use crate::editor::gis2d::gis2d_window_action;
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
use crate::GisMapSnapshot;
use semio_framework_plugin::{ActionArgOption, LocalizedLabel, MeasureSelectItem, WindowMeasure};
use semio_framework_surface::tiled_map::{gis_map_lod_scale_json, GIS_MAP_LOD_MODE_AUTOMATIC};
use serde_json::Value;

//#region 🔖️Vocabulary
pub const GIS2D_LOD_MODE_MEASURE_ID: &str = "gis2d-play-window.lod-mode";

/// 🔭️ When LOD mode is automatic, the zoom-resolved band id for the config camera (same source as `MapHost::current_lod_json`).
fn resolved_automatic_lod_id(document: &GisMapSnapshot, cfg: &MapWindowConfig) -> Option<String> {
    if cfg.lod_mode != GIS_MAP_LOD_MODE_AUTOMATIC {
        return None;
    }
    let host = maphost::map_host_from(document, cfg);
    serde_json::from_str::<Value>(&host.current_lod_json())
        .ok()
        .and_then(|value| value.get("id").and_then(Value::as_str).map(str::to_string))
}

fn lod_automatic_label(labels: &Gis2dPlayLabels, document: &GisMapSnapshot, cfg: &MapWindowConfig) -> String {
    let base = labels.lod_automatic.as_str();
    resolved_automatic_lod_id(document, cfg).map_or_else(|| base.to_string(), |lod_id| format!("{base} ({lod_id})"))
}

/// 🔽️ The automatic tier plus every LOD scale tier from the map descriptor, as `(value, label)` rows.
pub fn lod_select_entries(labels: &Gis2dPlayLabels, document: &GisMapSnapshot, cfg: &MapWindowConfig) -> Vec<(String, String)> {
    std::iter::once((GIS_MAP_LOD_MODE_AUTOMATIC.into(), lod_automatic_label(labels, document, cfg)))
        .chain(serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
            let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
            let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            Some((id, name))
        }))
        .collect()
}

/// 🔽️ The static LOD-mode choices for the palette arg schema: the automatic mode plus each LOD scale
/// tier from the map descriptor, labelled in the app's base locale (localization is applied by overlay).
pub fn lod_arg_options() -> Vec<ActionArgOption> {
    std::iter::once(ActionArgOption::new(GIS_MAP_LOD_MODE_AUTOMATIC, LocalizedLabel::native("Automatic", "Automatisch")))
        .chain(serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
            let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
            let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            Some(ActionArgOption::new(id, LocalizedLabel::data(name)))
        }))
        .collect()
}
//#endregion 🔖️Vocabulary

//#region 🔖️Option
pub fn measure(document: &GisMapSnapshot, cfg: &MapWindowConfig, labels: &Gis2dPlayLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: GIS2D_LOD_MODE_MEASURE_ID.into(),
        label: Some(labels.lod_mode.into()),
        value: cfg.lod_mode.clone(),
        items: lod_select_entries(labels, document, cfg).into_iter().map(|(value, label)| MeasureSelectItem { id: value.clone(), value, label }).collect(),
        on_change: gis2d_window_action("setLodMode", None),
    }
}
//#endregion 🔖️Option

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
