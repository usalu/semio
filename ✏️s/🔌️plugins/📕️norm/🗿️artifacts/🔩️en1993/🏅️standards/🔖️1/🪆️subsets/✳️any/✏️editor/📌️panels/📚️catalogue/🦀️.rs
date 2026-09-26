//! 📚️ EN 1993 play app panel — selectable examples plus partial-factor and section-property tables.

use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::document::ClauseId;
use crate::snapshot::rolled_heb_catalogue;
use crate::standards::v1::subsets::any::schema::{
    GAMMA_M0_DE, GAMMA_M0_EN, GAMMA_M1_DE, GAMMA_M1_EN, GAMMA_M2_DE, GAMMA_M2_EN, GAMMA_M3_DE, GAMMA_M3_EN, GAMMA_MF_DE, GAMMA_MF_EN,
};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1993.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

/// 📚️ Normative reference tables — same γ_M and rolled HEB properties `evaluate()` reads.
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![partial_factor_table(), section_property_table()]
}

fn partial_factor_table() -> CatalogueTable {
    CatalogueTable {
        id: "table-en1993-gamma-m",
        title_en: "Partial factors γ_M (EN / DE NA)",
        title_de: "Teilsicherheitsbeiwerte γ_M (EN / DE NA)",
        clause: ClauseId::new("EN 1993-1-1", "6", "6.1"),
        columns: vec![
            CatalogueColumn { id: "symbol", label_en: "Factor", label_de: "Beiwert", unit: None },
            CatalogueColumn { id: "en", label_en: "EN recommended", label_de: "EN empfohlen", unit: None },
            CatalogueColumn { id: "de", label_en: "DE national annex", label_de: "DE Nationaler Anhang", unit: None },
        ],
        rows: vec![
            CatalogueRow { id: "gammaM0".into(), cells: vec![CatalogueCell::text("γ_M0"), CatalogueCell::number(GAMMA_M0_EN, 2), CatalogueCell::number(GAMMA_M0_DE, 2)] },
            CatalogueRow { id: "gammaM1".into(), cells: vec![CatalogueCell::text("γ_M1"), CatalogueCell::number(GAMMA_M1_EN, 2), CatalogueCell::number(GAMMA_M1_DE, 2)] },
            CatalogueRow { id: "gammaM2".into(), cells: vec![CatalogueCell::text("γ_M2"), CatalogueCell::number(GAMMA_M2_EN, 2), CatalogueCell::number(GAMMA_M2_DE, 2)] },
            CatalogueRow { id: "gammaM3".into(), cells: vec![CatalogueCell::text("γ_M3"), CatalogueCell::number(GAMMA_M3_EN, 2), CatalogueCell::number(GAMMA_M3_DE, 2)] },
            CatalogueRow { id: "gammaMf".into(), cells: vec![CatalogueCell::text("γ_Mf"), CatalogueCell::number(GAMMA_MF_EN, 2), CatalogueCell::number(GAMMA_MF_DE, 2)] },
        ],
    }
}

fn section_property_table() -> CatalogueTable {
    CatalogueTable {
        id: "table-en1993-heb-sections",
        title_en: "Rolled HEB section properties",
        title_de: "Walzprofile HEB — Querschnittswerte",
        clause: ClauseId::new("EN 1993-1-1", "5", "Table 5.2"),
        columns: vec![
            CatalogueColumn { id: "designation", label_en: "Designation", label_de: "Bezeichnung", unit: None },
            CatalogueColumn { id: "area", label_en: "Area A", label_de: "Fläche A", unit: Some("cm²") },
            CatalogueColumn { id: "wply", label_en: "W_pl,y", label_de: "W_pl,y", unit: Some("cm³") },
            CatalogueColumn { id: "iy", label_en: "I_y", label_de: "I_y", unit: Some("cm⁴") },
        ],
        rows: rolled_heb_catalogue()
            .into_iter()
            .map(|s| CatalogueRow {
                id: s.id.clone(),
                cells: vec![
                    CatalogueCell::text(s.designation.clone()),
                    CatalogueCell::number(s.area * 1.0e4, 2),
                    CatalogueCell::number(s.w_pl_y * 1.0e6, 1),
                    CatalogueCell::number(s.iy * 1.0e8, 1),
                ],
            })
            .collect(),
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
