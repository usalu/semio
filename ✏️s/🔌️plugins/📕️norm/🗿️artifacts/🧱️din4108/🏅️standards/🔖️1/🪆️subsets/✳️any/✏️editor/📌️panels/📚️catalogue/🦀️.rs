//! 📚️ DIN 4108 play app panel — examples plus design-λ, Table 3 R_min, and 4108-10 application classes.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::document::ClauseId;
use crate::artifact_schema::{part_2, part_4, part_10};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.din4108.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 🔖️Tables
/// 📚️ Normative reference tables beside examples (DIN 4108-4 λ, DIN 4108-2 Table 3, DIN 4108-10 Table 1).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![design_lambda_table(), table3_r_min_table(), application_property_table()]
}

fn design_lambda_table() -> CatalogueTable {
    CatalogueTable {
        id: "din4108-4-design-lambda",
        title_en: "Design thermal conductivity λ",
        title_de: "Bemessungswärmeleitfähigkeit λ",
        clause: ClauseId::new("DIN 4108-4", "Table 1", "λ"),
        columns: vec![
            CatalogueColumn { id: "material", label_en: "Material id", label_de: "Werkstoff-Id", unit: None },
            CatalogueColumn { id: "lambda", label_en: "λ_design", label_de: "λ_Bemessung", unit: Some("W/(m·K)") },
        ],
        rows: part_4::DESIGN_LAMBDA_ROWS
            .iter()
            .map(|(id, lambda)| CatalogueRow {
                id: (*id).into(),
                cells: vec![CatalogueCell::text(*id), CatalogueCell::number(*lambda, 3)],
            })
            .collect(),
    }
}

fn table3_r_min_table() -> CatalogueTable {
    CatalogueTable {
        id: "din4108-2-table3-r-min",
        title_en: "Minimum thermal resistance R_min",
        title_de: "Mindestwärmedurchlasswiderstand R_min",
        clause: ClauseId::new("DIN 4108-2", "Table 3", "R_min"),
        columns: vec![
            CatalogueColumn { id: "kind", label_en: "Element kind", label_de: "Bauteilart", unit: None },
            CatalogueColumn { id: "adjacent", label_en: "Adjacent", label_de: "Angrenzend", unit: None },
            CatalogueColumn { id: "mass", label_en: "Mass class", label_de: "Masseklasse", unit: None },
            CatalogueColumn { id: "r_min", label_en: "R_min", label_de: "R_min", unit: Some("m²K/W") },
        ],
        rows: part_2::TABLE3_R_MIN_ROWS
            .iter()
            .map(|(kind, adjacent, mass, r_min)| CatalogueRow {
                id: format!("{kind}:{adjacent}:{mass}"),
                cells: vec![
                    CatalogueCell::text(*kind),
                    CatalogueCell::text(*adjacent),
                    CatalogueCell::text(*mass),
                    CatalogueCell::number(*r_min, 2),
                ],
            })
            .collect(),
    }
}

fn application_property_table() -> CatalogueTable {
    CatalogueTable {
        id: "din4108-10-application-classes",
        title_en: "Application and property classes",
        title_de: "Anwendungsgebiete und Eigenschaftsklassen",
        clause: ClauseId::new("DIN 4108-10", "Table 1", "application"),
        columns: vec![
            CatalogueColumn { id: "app", label_en: "Application", label_de: "Anwendung", unit: None },
            CatalogueColumn { id: "compressive", label_en: "Min compressive", label_de: "Min. Druck", unit: None },
            CatalogueColumn { id: "water", label_en: "Min water", label_de: "Min. Wasser", unit: None },
            CatalogueColumn { id: "tensile", label_en: "Min tensile", label_de: "Min. Zug", unit: None },
            CatalogueColumn { id: "acoustic", label_en: "Min acoustic", label_de: "Min. Akustik", unit: None },
        ],
        rows: part_10::CATALOGUE_APPLICATION_TYPES
            .iter()
            .map(|app| {
                let (id, c, w, t, a) = part_10::application_property_row(app);
                CatalogueRow {
                    id: id.into(),
                    cells: vec![
                        CatalogueCell::text(id),
                        CatalogueCell::text(c),
                        CatalogueCell::text(w),
                        CatalogueCell::text(t),
                        CatalogueCell::text(a),
                    ],
                }
            })
            .collect(),
    }
}
//#endregion 🔖️Tables

//#region 🔖️Render
pub fn render(
    examples: Vec<semio_framework_plugin::ExampleSource>,
    locale: semio_framework_plugin::Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_catalogue(&examples, &reference_tables(), locale, controller_id, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
