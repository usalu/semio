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
    use crate::standards::v1::subsets::any::schema::part_1_1::CATALOGUE_ALLOY_ROWS;
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
        rows: CATALOGUE_ALLOY_ROWS.iter().map(|(des, props)| CatalogueRow {
            id: (*des).into(),
            cells: vec![
                CatalogueCell::text(*des),
                CatalogueCell::number(props.f_o_pa / 1e6, 0),
                CatalogueCell::number(props.f_u_pa / 1e6, 0),
                CatalogueCell::number(props.rho_o_haz, 2),
                CatalogueCell::number(props.rho_u_haz, 2),
            ],
        }).collect(),
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
