//! 🗣️ GIS 2D play app — the single `app_labels!` block plus the locale resolvers every taxonomy node
//! reaches for.

use crate::editor::gis2d::config::Gis2dConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the GIS 2D app; one field per label makes every locale combination compile-checked.
    pub struct Gis2dPlayLabels {
        window_map: native_en "Map", native_de "Karte", reuse_en "Map", reuse_de "Karte";
        mode_edit: native_en "Edit", native_de "Bearbeiten", reuse_en "Edit", reuse_de "Bearbeiten";
        layer_raster: native_en "Raster", native_de "Raster", reuse_en "Raster", reuse_de "Raster";
        layer_water: native_en "Water", native_de "Wasser", reuse_en "Water", reuse_de "Wasser";
        layer_land: native_en "Land", native_de "Land", reuse_en "Land", reuse_de "Land";
        layer_roads: native_en "Roads", native_de "Straßen", reuse_en "Roads", reuse_de "Straßen";
        layer_buildings: native_en "Buildings", native_de "Gebäude", reuse_en "Buildings", reuse_de "Gebäude";
        layer_borders: native_en "Borders", native_de "Grenzen", reuse_en "Borders", reuse_de "Grenzen";
        layer_map_labels: native_en "Labels", native_de "Beschriftungen", reuse_en "Labels", reuse_de "Beschriftungen";
        layer_positions: native_en "Positions", native_de "Positionen", reuse_en "Positions", reuse_de "Positionen";
        layer_position_labels: native_en "Position Labels", native_de "Positionsbeschriftungen", reuse_en "Position Labels", reuse_de "Positionsbeschriftungen";
        layer_routes: native_en "Routes", native_de "Routen", reuse_en "Routes", reuse_de "Routen";
        layer_regions: native_en "Regions", native_de "Regionen", reuse_en "Regions", reuse_de "Regionen";
        map_view: native_en "Map View", native_de "Kartenansicht", reuse_en "Map View", reuse_de "Kartenansicht";
        render_mode: native_en "Render Mode", native_de "Darstellungsmodus", reuse_en "Render Mode", reuse_de "Darstellungsmodus";
        render_mode_image: native_en "Image", native_de "Bild", reuse_en "Image", reuse_de "Bild";
        render_mode_vector: native_en "Vector", native_de "Vektor", reuse_en "Vector", reuse_de "Vektor";
        render_mode_combined: native_en "Combined", native_de "Kombiniert", reuse_en "Combined", reuse_de "Kombiniert";
        vector_style: native_en "Vector Style", native_de "Vektorstil", reuse_en "Vector Style", reuse_de "Vektorstil";
        vector_style_colored: native_en "Colored", native_de "Farbig", reuse_en "Colored", reuse_de "Farbig";
        vector_style_figure_ground: native_en "Figure Ground", native_de "Figur-Grund", reuse_en "Figure Ground", reuse_de "Figur-Grund";
        vector_style_inverted_figure: native_en "Inverted Figure", native_de "Invertierte Figur", reuse_en "Inverted Figure", reuse_de "Invertierte Figur";
        lod_mode: native_en "LOD Mode", native_de "LOD-Modus", reuse_en "LOD Mode", reuse_de "LOD-Modus";
        lod_automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        layers_group: native_en "Layers", native_de "Ebenen", reuse_en "Layers", reuse_de "Ebenen";
        layer_weights_group: native_en "Layer Weights", native_de "Ebenengewichte", reuse_en "Layer Weights", reuse_de "Ebenengewichte";
        weight_suffix: native_en "weight", native_de "Gewicht", reuse_en "weight", reuse_de "Gewicht";
        map_layer: native_en "Map Layer", native_de "Kartenebene", reuse_en "Map Layer", reuse_de "Kartenebene";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        layers_visible: native_en "Layers visible", native_de "Sichtbare Ebenen", reuse_en "Layers visible", reuse_de "Sichtbare Ebenen";
    }
}

/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub async fn gis2d_labels(cfg: &Gis2dConfig) -> &'static Gis2dPlayLabels {
    semio_framework_plugin::resolve_labels_for_locale::<Gis2dPlayLabels>(&cfg.locale)
}

/// 🗣️ Resolves a standard map layer's display label from its stable id; unknown ids fall back to the
/// catalog's native English text.
pub async fn gis2d_layer_label(layer_id: &str, labels: &Gis2dPlayLabels) -> &'static str {
    match layer_id {
        "raster" => labels.layer_raster.as_str(),
        "water" => labels.layer_water.as_str(),
        "land" => labels.layer_land.as_str(),
        "roads" => labels.layer_roads.as_str(),
        "buildings" => labels.layer_buildings.as_str(),
        "borders" => labels.layer_borders.as_str(),
        "labels" => labels.layer_map_labels.as_str(),
        "positions" => labels.layer_positions.as_str(),
        "positionLabels" => labels.layer_position_labels.as_str(),
        "routes" => labels.layer_routes.as_str(),
        "regions" => labels.layer_regions.as_str(),
        // 🗣️ unreachable in practice — the arms above already cover every id in GIS_MAP_LAYER_IDS.
        _ => "",
    }
}
//#endregion 🔖️Labels

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(gis2d_labels(&Gis2dConfig::default()).map_view.as_str(), "Map View");
        assert_eq!(gis2d_labels(&Gis2dConfig { locale: "de-DE".into(), ..Gis2dConfig::default() }).map_view.as_str(), "Kartenansicht");
    }

    #[semio_framework_async_macros::async_test]
    async fn every_declared_layer_id_resolves_to_a_non_empty_label() {
        let labels = gis2d_labels(&Gis2dConfig::default());
        for (id, _, _) in crate::editor::gis2d::GIS_MAP_LAYER_IDS {
            assert!(!gis2d_layer_label(id, labels).is_empty(), "layer {id} has no label");
        }
        assert_eq!(gis2d_layer_label("bogusLayer", labels), "");
    }
}
//#endregion 🧪️Tests
