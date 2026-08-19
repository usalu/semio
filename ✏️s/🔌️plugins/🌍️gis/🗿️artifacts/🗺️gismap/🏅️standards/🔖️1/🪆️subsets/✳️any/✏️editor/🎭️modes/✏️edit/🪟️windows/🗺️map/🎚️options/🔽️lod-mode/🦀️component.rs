//! 🔽️ Map window option — the level-of-detail tier select.
//!
//! 🧭️ Single owner of the LOD vocabulary: the tier list is derived once here from the tiled-map
//! surface crate's scale table, and reused by the inspector panel's matching field and by the
//! manifest's `setLodMode` arg schema.

use crate::editor::gis2d::config::Gis2dConfig;
use crate::editor::gis2d::gis2d_action;
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
use framework_surface::tiled_map::{gis_map_lod_scale_json, GIS_MAP_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::{ActionArgOption, LocalizedLabel, MeasureSelectItem, WindowMeasure};
use serde_json::Value;

//#region 🔖️Vocabulary
pub const GIS2D_LOD_MODE_MEASURE_ID: &str = "gis2d-play-window.lod-mode";

/// 🔽️ The automatic tier plus every LOD scale tier from the map descriptor, as `(value, label)` rows.
pub async fn lod_select_entries(labels: &Gis2dPlayLabels) -> Vec<(String, String)> {
    std::iter::once((GIS_MAP_LOD_MODE_AUTOMATIC.into(), labels.lod_automatic.into()))
        .chain(serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
            let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
            let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            Some((id, name))
        }))
        .collect()
}

/// 🔽️ The static LOD-mode choices for the palette arg schema: the automatic mode plus each LOD scale
/// tier from the map descriptor, labelled in the app's base locale (localization is applied by overlay).
pub async fn lod_arg_options() -> Vec<ActionArgOption> {
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
pub async fn measure(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: GIS2D_LOD_MODE_MEASURE_ID.into(),
        label: Some(labels.lod_mode.into()),
        value: cfg.lod_mode.clone(),
        items: lod_select_entries(labels).into_iter().map(|(value, label)| MeasureSelectItem { id: value.clone(), value, label }).collect(),
        on_change: gis2d_action("setLodMode", None),
    }
}
//#endregion 🔖️Option

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::terminology::gis2d_labels;

    #[test]
    async fn the_tier_list_always_starts_with_the_automatic_mode() {
        let config = Gis2dConfig::default();
        let entries = lod_select_entries(gis2d_labels(&config));
        assert_eq!(entries[0].0, GIS_MAP_LOD_MODE_AUTOMATIC);
        assert_eq!(lod_arg_options().len(), entries.len(), "the palette arg schema and the window select share one vocabulary");
    }

    #[test]
    async fn the_measure_mirrors_the_config_value() {
        let config = Gis2dConfig::default();
        let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&config)) else {
            panic!("lod mode is a select measure");
        };
        assert_eq!(value, GIS_MAP_LOD_MODE_AUTOMATIC);
        assert!(!items.is_empty());
    }
}
//#endregion 🧪️Tests
