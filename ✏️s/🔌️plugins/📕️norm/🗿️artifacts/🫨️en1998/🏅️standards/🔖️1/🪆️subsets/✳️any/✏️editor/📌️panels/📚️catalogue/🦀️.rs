//! 📚️ EN 1998 play app panel — examples plus DIN EN 1998-1/NA Tables NA.1 and NA.4.

use semio_s_plugin_norm_app_surface::{spec_trace_table, CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use semio_s_artifact_norm_en1998_inferences::spec_trace::TABLE as SPEC_TRACE;
use semio_s_plugin_norm_compliance::ClauseId;
use semio_s_artifact_norm_en1998_schema::na_de::{self, GroundCombo};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1998.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    semio_s_plugin_norm_app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

/// 📚️ Normative reference tables — same NA.1 / NA.4 consts `evaluate()` reads via `na_de`.
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![table_na1_plateau(), table_na4_ground_combos(), spec_trace_table(&SPEC_TRACE)]
}

fn table_na1_plateau() -> CatalogueTable {
    CatalogueTable {
        id: "table-na-1-plateau-acceleration",
        title_en: "Reference acceleration a_gR from plateau acceleration S_aP,R (DE NA:2023-11)",
        title_de: "Bezugsbeschleunigung a_gR aus Plateau-Beschleunigung S_aP,R (DE NA:2023-11)",
        clause: ClauseId::new("DIN EN 1998-1/NA", "NA.3.2.1", "S_aP,R"),
        columns: vec![
            CatalogueColumn { id: "sapr", label_en: "S_aP,R", label_de: "S_aP,R", unit: Some("m/s²") },
            CatalogueColumn { id: "agr", label_en: "a_gR = S_aP,R / 2.5", label_de: "a_gR = S_aP,R / 2,5", unit: Some("m/s²") },
        ],
        rows: [0.5, 1.0, 1.5, 2.0, 2.5]
            .into_iter()
            .map(|s_ap_r| CatalogueRow {
                id: format!("s-ap-r-{s_ap_r}"),
                cells: vec![CatalogueCell::number(s_ap_r, 2), CatalogueCell::number(na_de::a_gr_from_s_ap_r(s_ap_r), 2)],
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
pub fn render(examples: Vec<semio_framework_plugin::ExampleSource>, locale: semio_framework_ui_locale::Locale, controller_id: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_s_plugin_norm_app_surface::render_catalogue(&examples, &reference_tables(), locale, controller_id, &semio_framework_plugin::TreeWindows::unhosted())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
