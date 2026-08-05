//! 🗣️ Animate present app — the single `app_labels!` block plus the locale resolver every taxonomy
//! node reaches for. Deliberately ONE block for the whole app (never split per window/panel): the
//! macro's value is that every locale×terminology combination is compile-checked in one place.

use crate::apps::present::config::PresentConfig;
use semio_framework_plugin::{Locale, Terminology};

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the animate present tile-play app; one field per label makes every locale×terminology combination compile-checked. `PresentConfig` carries no terminology axis, so `reuse_*` mirrors `native_*` throughout.
    pub struct AnimatePresentLabels {
        tiles_section: native_en "Tiles", native_de "Kacheln", reuse_en "Tiles", reuse_de "Kacheln";
        no_tiles: native_en "(no tiles — seed a grid)", native_de "(keine Kacheln — Raster erzeugen)", reuse_en "(no tiles — seed a grid)", reuse_de "(keine Kacheln — Raster erzeugen)";
        details_select_tile: native_en "Select a tile in the canvas or workbench document.", native_de "Wählen Sie eine Kachel in der Leinwand oder im Werkbankdokument aus.", reuse_en "Select a tile in the canvas or workbench document.", reuse_de "Wählen Sie eine Kachel in der Leinwand oder im Werkbankdokument aus.";
        details_tile_not_found: native_en "Selected tile not found.", native_de "Ausgewählte Kachel nicht gefunden.", reuse_en "Selected tile not found.", reuse_de "Ausgewählte Kachel nicht gefunden.";
        field_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        field_id: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        selected_suffix: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        delete_tile: native_en "Delete tile", native_de "Kachel löschen", reuse_en "Delete tile", reuse_de "Kachel löschen";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
        group_crop: native_en "Crop", native_de "Zuschnitt", reuse_en "Crop", reuse_de "Zuschnitt";
        field_x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        field_y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        field_width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        field_height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        group_identity: native_en "Identity", native_de "Identität", reuse_en "Identity", reuse_de "Identität";
        catalogue_tile_templates: native_en "Tile templates", native_de "Kachelvorlagen", reuse_en "Tile templates", reuse_de "Kachelvorlagen";
        catalogue_seed_desc: native_en "Seed morph tiles from figure templates.", native_de "Morph-Kacheln aus Abbildungsvorlagen erzeugen.", reuse_en "Seed morph tiles from figure templates.", reuse_de "Morph-Kacheln aus Abbildungsvorlagen erzeugen.";
        catalogue_seed_2x2: native_en "Split 2×2 grid", native_de "2×2-Raster teilen", reuse_en "Split 2×2 grid", reuse_de "2×2-Raster teilen";
        catalogue_seed_3x5: native_en "Split 3×5 catalogue grid", native_de "3×5-Katalograster teilen", reuse_en "Split 3×5 catalogue grid", reuse_de "3×5-Katalograster teilen";
        catalogue_add_tile: native_en "Add single tile", native_de "Einzelne Kachel hinzufügen", reuse_en "Add single tile", reuse_de "Einzelne Kachel hinzufügen";
        catalogue_clear_tiles: native_en "Clear tiles", native_de "Kacheln leeren", reuse_en "Clear tiles", reuse_de "Kacheln leeren";
        catalogue_figure_templates: native_en "Figure templates", native_de "Abbildungsvorlagen", reuse_en "Figure templates", reuse_de "Abbildungsvorlagen";
        catalogue_use_figure: native_en "Use catalogue figure", native_de "Katalogabbildung verwenden", reuse_en "Use catalogue figure", reuse_de "Katalogabbildung verwenden";
        catalogue_active_source: native_en "Active source", native_de "Aktive Quelle", reuse_en "Active source", reuse_de "Aktive Quelle";
        catalogue_media_kind: native_en "Media kind", native_de "Medientyp", reuse_en "Media kind", reuse_de "Medientyp";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ B1: resolves the active label set from `cfg.locale` (was the host-pushed `ViewState.locale`);
/// unknown/absent locales fall back to native English. `PresentConfig` carries no terminology axis,
/// so this app is always `Terminology::Native` — mirrors `sequence_ui`'s identical pair.
pub fn animate_present_locale(config: &PresentConfig) -> Locale {
    if config.locale.starts_with("de") {
        Locale::De
    } else {
        Locale::En
    }
}

pub fn animate_present_labels(config: &PresentConfig) -> &'static AnimatePresentLabels {
    AnimatePresentLabels::labels(animate_present_locale(config), Terminology::Native)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(animate_present_labels(&PresentConfig::default()).tiles_section.as_str(), "Tiles");
        assert_eq!(animate_present_labels(&PresentConfig { locale: "de-DE".into(), ..PresentConfig::default() }).tiles_section.as_str(), "Kacheln");
    }
}
//#endregion 🧪️Tests
