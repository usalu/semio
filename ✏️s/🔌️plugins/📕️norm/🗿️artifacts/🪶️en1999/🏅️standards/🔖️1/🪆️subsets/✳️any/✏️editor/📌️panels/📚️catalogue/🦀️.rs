//! 📚️ EN 1999 play app panel — alloy/temper catalogue (Table 3.2) and examples.

use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::document::ClauseId;

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1999.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 📚️AlloyCatalogue
/// 🧪 EN 1999-1-1 Table 3.2 alloy/temper reference rows (designation, f_o, f_u, ρ_o,haz, ρ_u,haz).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![alloy_table_3_2()]
}

pub fn alloy_table_3_2() -> CatalogueTable {
    CatalogueTable {
        id: "table-3-2",
        title_en: "Aluminium alloys and tempers",
        title_de: "Aluminiumlegierungen und Zustände",
        clause: ClauseId::new("EN 1999-1-1", "§3.2", "3.2"),
        columns: vec![
            CatalogueColumn { id: "des", label_en: "Designation", label_de: "Bezeichnung", unit: None },
            CatalogueColumn { id: "fo", label_en: "f_o", label_de: "f_o", unit: Some("MPa") },
            CatalogueColumn { id: "fu", label_en: "f_u", label_de: "f_u", unit: Some("MPa") },
            CatalogueColumn { id: "rho_o", label_en: "ρ_o,haz", label_de: "ρ_o,WEZ", unit: None },
            CatalogueColumn { id: "rho_u", label_en: "ρ_u,haz", label_de: "ρ_u,WEZ", unit: None },
        ],
        rows: vec![
            alloy_row("aw6060-t6", 160.0, 215.0, 0.48, 0.56),
            alloy_row("aw6061-t6", 240.0, 290.0, 0.53, 0.62),
            alloy_row("aw6063-t6", 170.0, 215.0, 0.49, 0.56),
            alloy_row("aw6082-t6", 260.0, 310.0, 0.64, 0.73),
            alloy_row("aw5083-o", 125.0, 275.0, 1.0, 1.0),
            alloy_row("aw5083-h111", 125.0, 275.0, 1.0, 1.0),
        ],
    }
}

fn alloy_row(des: &str, fo: f64, fu: f64, rho_o: f64, rho_u: f64) -> CatalogueRow {
    CatalogueRow {
        id: des.into(),
        cells: vec![
            CatalogueCell::text(des),
            CatalogueCell::number(fo, 0),
            CatalogueCell::number(fu, 0),
            CatalogueCell::number(rho_o, 2),
            CatalogueCell::number(rho_u, 2),
        ],
    }
}
//#endregion 📚️AlloyCatalogue

//#region 🔖️Render
pub fn render(examples: Vec<semio_framework_plugin::ExampleSource>, locale: semio_framework_plugin::Locale, controller_id: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let tables = reference_tables();
    let windows = TreeWindows::unhosted();
    crate::app_surface::render_catalogue(&examples, &tables, locale, controller_id, &windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion ️Tests
