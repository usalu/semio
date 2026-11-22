//! 🔍️ Terrain data inspection for the mounted GIS editor.

use crate::GisTerrainSnapshot;
use semio_framework_surface::terrain::missing_samples::is_missing;
use semio_framework_value::DslValue;
use semio_plugin::{BuiltNode, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, ViewModel, tree_item_desc, ui_node_list};
use semio_ui_locale::LocalizedLabel;

pub const BODY: &str = "gis3d.terrain.data";

/// 🧭️ Registers an inspection tab for stored elevation and GIS features.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native("Terrain Data", "Geländedaten"),
        group: PanelGroup::Details,
        body_key: Some(BODY.into()),
        children: Vec::new(),
    }
}

fn bounded_text(value: &str) -> &str {
    &value[..value.floor_char_boundary(240)]
}

/// 📊️ Renders the current document's grid, sample statistics and editable feature IDs.
pub fn render(document: &GisTerrainSnapshot, view: &ViewModel) -> UiAssemblyResult<BuiltNode> {
    let localize = |en, de| LocalizedLabel::native(en, de).resolve(view.terminology, view.locale).to_owned();
    let item = |id, en, de, value| tree_item_desc(id, localize(en, de), Some(value));
    let label = |text: String| semio_ui_contract::Label::try_from(text).map_err(|_| semio_plugin::ui_assembly_error(BODY));
    let mut data = PanelTreeBuilder::new(BODY)?;
    if let Some(grid) = document.elevation_source.as_ref().filter(|grid| grid.is_valid()) {
        let [west, south, east, north] = grid.bounds;
        let mask = grid.missing_samples.as_deref();
        let (minimum, maximum, sum, present) = grid.rows_millimetres.iter().flatten().copied().enumerate()
            .filter(|(ordinal, _)| !is_missing(mask, *ordinal))
            .fold((i32::MAX, i32::MIN, 0_i64, 0_usize), |(low, high, sum, present), (_, height)| {
                (low.min(height), high.max(height), sum + i64::from(height), present + 1)
            });
        let dimensions = (grid.rows_millimetres[0].len(), grid.rows_millimetres.len());
        let absent = dimensions.0 * dimensions.1 - present;
        let mut items = Vec::with_capacity(6);
        items.push(item("grid-size", "Grid Size", "Rastergröße", format!("{} × {}", dimensions.0, dimensions.1)));
        if present > 0 {
            let minimum = f64::from(minimum) / 1000.0;
            let maximum = f64::from(maximum) / 1000.0;
            let mean = sum as f64 / (1000.0 * present as f64);
            items.push(item("grid-range", "Elevation Range", "Höhenbereich", format!("{minimum}–{maximum} m")));
            items.push(item("grid-mean", "Mean Elevation", "Mittlere Höhe", format!("{mean:.3} m")));
        }
        if absent > 0 {
            items.push(item("grid-missing-count", "Missing Samples", "Fehlende Rasterpunkte", absent.to_string()));
        }
        items.extend([
            item("grid-longitude", "Longitude Bounds", "Längengrade", format!("{west:.6}° → {east:.6}°")),
            item("grid-latitude", "Latitude Bounds", "Breitengrade", format!("{south:.6}° → {north:.6}°")),
        ]);
        let items = ui_node_list(items)?;
        data = data.section("elevation", Some(label(localize("Elevation", "Höhe"))?), true, items)?;
    } else {
        let items = ui_node_list([item("grid-empty", "Elevation Grid", "Höhenraster", localize("No grid loaded", "Kein Raster geladen"))])?;
        data = data.section("elevation", None, true, items)?;
    }
    if let Some(map) = &document.imported_map {
        for (id, en, de, features) in [
            ("positions", "Positions", "Positionen", &map.positions),
            ("routes", "Routes", "Routen", &map.routes),
            ("regions", "Regions", "Regionen", &map.regions),
        ] {
            let items = ui_node_list(features.iter().filter_map(|feature| {
                let feature_id = feature.get("id")?.as_str()?;
                let name = feature.get("name").or_else(|| feature.get("label")).and_then(DslValue::as_str);
                Some((bounded_text(feature_id), name.filter(|name| *name != feature_id).map(bounded_text)))
            }).take(16).enumerate().map(|(index, (feature_id, description))| {
                tree_item_desc(format!("{id}-{index}"), feature_id, description.map(str::to_owned))
            }))?;
            let heading = format!("{} ({})", localize(en, de), features.len());
            data = data.section(id, Some(label(heading)?), true, items)?;
        }
    }
    data.build()
}
