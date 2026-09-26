//! 📚️ EN 1998 play app panel — examples plus DIN EN 1998-1/NA Tables NA.1 and NA.4.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::document::ClauseId;
use crate::standards::v1::subsets::any::schema::na_de::{GroundCombo, SeismicZone};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1998.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

/// 📚️ Normative reference tables — same NA.1 / NA.4 consts `evaluate()` reads via `na_de`.
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![table_na1_zones(), table_na4_ground_combos()]
}

fn table_na1_zones() -> CatalogueTable {
    CatalogueTable {
        id: "table-na-1-seismic-zones",
        title_en: "Reference PGA a_gR by seismic zone (DE NA)",
        title_de: "Bezugsspitzenwert a_gR nach Erdbebenzone (DE NA)",
        clause: ClauseId::new("DIN EN 1998-1/NA", "NA.3.2.1", "Table NA.1"),
        columns: vec![
            CatalogueColumn { id: "zone", label_en: "Zone", label_de: "Zone", unit: None },
            CatalogueColumn { id: "agr", label_en: "a_gR", label_de: "a_gR", unit: Some("m/s²") },
        ],
        rows: [SeismicZone::Zone0, SeismicZone::Zone1, SeismicZone::Zone2, SeismicZone::Zone3]
            .into_iter()
            .map(|zone| {
                let id = format!("zone-{}", zone.as_u8());
                CatalogueRow {
                    id: id.clone(),
                    cells: vec![CatalogueCell::text(format!("Zone {}", zone.as_u8())), CatalogueCell::number(zone.a_gr(), 1)],
                }
            })
            .collect(),
    }
}

fn ground_combo_row(combo: GroundCombo, label: &str) -> CatalogueRow {
    let (s, tb, tc, td) = combo.spectrum_params();
    CatalogueRow {
        id: label.into(),
        cells: vec![
            CatalogueCell::text(label),
            CatalogueCell::number(s, 2),
            CatalogueCell::number(tb, 2),
            CatalogueCell::number(tc, 2),
            CatalogueCell::number(td, 1),
        ],
    }
}

fn table_na4_ground_combos() -> CatalogueTable {
    CatalogueTable {
        id: "table-na-4-ground-combos",
        title_en: "Spectrum parameters S, T_B, T_C, T_D (DE NA)",
        title_de: "Spektrumparameter S, T_B, T_C, T_D (DE NA)",
        clause: ClauseId::new("DIN EN 1998-1/NA", "NA.3.2.2.2", "Table NA.4"),
        columns: vec![
            CatalogueColumn { id: "combo", label_en: "Ground combination", label_de: "Untergrundkombination", unit: None },
            CatalogueColumn { id: "S", label_en: "S", label_de: "S", unit: None },
            CatalogueColumn { id: "TB", label_en: "T_B", label_de: "T_B", unit: Some("s") },
            CatalogueColumn { id: "TC", label_en: "T_C", label_de: "T_C", unit: Some("s") },
            CatalogueColumn { id: "TD", label_en: "T_D", label_de: "T_D", unit: Some("s") },
        ],
        rows: vec![
            ground_combo_row(GroundCombo::AR, "A-R"),
            ground_combo_row(GroundCombo::BR, "B-R"),
            ground_combo_row(GroundCombo::CR, "C-R"),
            ground_combo_row(GroundCombo::BT, "B-T"),
            ground_combo_row(GroundCombo::CT, "C-T"),
            ground_combo_row(GroundCombo::CS, "C-S"),
        ],
    }
}

//#region 🔖️Render
pub fn render(examples: Vec<semio_framework_plugin::ExampleSource>, locale: semio_framework_plugin::Locale, controller_id: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_catalogue(&examples, &reference_tables(), locale, controller_id, &semio_framework_plugin::TreeWindows::unhosted())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
