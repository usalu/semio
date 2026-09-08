//! 🗣️ Animate presentation app — the single `app_labels!` block plus the locale resolver every taxonomy
//! node reaches for. Deliberately ONE block for the whole app (never split per window/panel): the
//! macro's value is that every locale×terminology combination is compile-checked in one place.

use crate::editor::animate::config::PresentationConfig;
use semio_framework_plugin::{AppLabels, Locale, Terminology};

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the animate presentation tile-play app; one field per label makes every locale×terminology combination compile-checked. `PresentationConfig` carries no terminology axis, so `reuse_*` mirrors `native_*` throughout.
    pub struct AnimatePresentationLabels {
        tiles_section: native_en "Tiles", native_de "Kacheln", reuse_en "Tiles", reuse_de "Kacheln";
        no_tiles: native_en "(no tiles — seed a grid)", native_de "(keine Kacheln — Raster erzeugen)", reuse_en "(no tiles — seed a grid)", reuse_de "(keine Kacheln — Raster erzeugen)";
        details_title: native_en "Inspection", native_de "Inspektion", reuse_en "Inspection", reuse_de "Inspektion";
        details_schema_field: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        details_tiles_field: native_en "Tiles", native_de "Kacheln", reuse_en "Tiles", reuse_de "Kacheln";
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
/// 🗣️ B1: resolves the active label set from `cfg.locale` (was the host-pushed `ViewModel.locale`);
/// unknown/absent locales fall back to native English. `PresentationConfig` carries no terminology axis,
/// so this app is always `Terminology::Native` — mirrors `sequence_ui`'s identical pair.
pub fn animate_presentation_locale(config: &PresentationConfig) -> Locale {
    if config.locale.starts_with("de") {
        Locale::De
    } else {
        Locale::En
    }
}

pub fn animate_presentation_labels(config: &PresentationConfig) -> &'static AnimatePresentationLabels {
    AnimatePresentationLabels::labels(animate_presentation_locale(config), Terminology::Native)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
