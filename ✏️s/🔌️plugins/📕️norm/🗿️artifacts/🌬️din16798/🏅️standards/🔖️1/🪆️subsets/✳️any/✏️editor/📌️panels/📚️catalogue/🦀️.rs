//! 📚️ DIN EN 16798 play app panel — examples plus SFP, draught, and Annex B reference tables.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::artifact_schema::{
    annex_params::AnnexParams,
    part_1::{co2_above_outdoor_ppm, draught_limit_percent, outdoor_air_per_area_l_s_m2, outdoor_air_per_person_l_s},
    part_3::sfp_bound,
    ComfortCategory, PollutionClass,
};
use crate::document::ClauseId;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.din16798.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 🔖️Tables
/// 📚️ Normative reference tables beside examples (Table 10 SFP, ISO 7730 DR, Annex B.6/B.7, annex CO₂).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![sfp_class_bounds(), draught_rate_limits(), table_b6_co2(), table_b6_b7_outdoor_air(), annex_co2_absolute()]
}

fn sfp_class_bounds() -> CatalogueTable {
    CatalogueTable {
        id: "sfp-class-bounds",
        title_en: "SFP class bounds",
        title_de: "SFP-Klassengrenzen",
        clause: ClauseId::new("EN 16798-3", "Table 10", "SFP"),
        columns: vec![
            CatalogueColumn { id: "class", label_en: "Class", label_de: "Klasse", unit: None },
            CatalogueColumn { id: "bound", label_en: "Max SFP", label_de: "Max. SFP", unit: Some("W/(m³/s)") },
        ],
        rows: (0u8..5)
            .map(|class| CatalogueRow {
                id: class.to_string(),
                cells: vec![CatalogueCell::text(format!("SFP {class}")), CatalogueCell::number(sfp_bound(class), 0)],
            })
            .collect(),
    }
}

fn draught_rate_limits() -> CatalogueTable {
    CatalogueTable {
        id: "draught-rate-limits",
        title_en: "Draught rate DR limits",
        title_de: "Zugluftraten-Grenzwerte DR",
        clause: ClauseId::new("EN 16798-1", "§7.2 / ISO 7730", "DR"),
        columns: vec![
            CatalogueColumn { id: "cat", label_en: "Category", label_de: "Kategorie", unit: None },
            CatalogueColumn { id: "dr", label_en: "DR max", label_de: "DR max", unit: Some("%") },
        ],
        rows: [ComfortCategory::I, ComfortCategory::II, ComfortCategory::III, ComfortCategory::IV]
            .into_iter()
            .map(|cat| {
                let label = match cat {
                    ComfortCategory::I => "I",
                    ComfortCategory::II => "II",
                    ComfortCategory::III => "III",
                    ComfortCategory::IV => "IV",
                };
                CatalogueRow {
                    id: label.into(),
                    cells: vec![CatalogueCell::text(label), CatalogueCell::number(draught_limit_percent(cat), 0)],
                }
            })
            .collect(),
    }
}

fn table_b6_co2() -> CatalogueTable {
    CatalogueTable {
        id: "table-b6-co2",
        title_en: "CO₂ above outdoor (Table B.6)",
        title_de: "CO₂ über Außenluft (Tabelle B.6)",
        clause: ClauseId::new("EN 16798-1", "Annex B", "B.6"),
        columns: vec![
            CatalogueColumn { id: "cat", label_en: "Category", label_de: "Kategorie", unit: None },
            CatalogueColumn { id: "delta", label_en: "ΔCO₂", label_de: "ΔCO₂", unit: Some("ppm") },
        ],
        rows: [ComfortCategory::I, ComfortCategory::II, ComfortCategory::III, ComfortCategory::IV]
            .into_iter()
            .map(|cat| {
                let label = match cat {
                    ComfortCategory::I => "I",
                    ComfortCategory::II => "II",
                    ComfortCategory::III => "III",
                    ComfortCategory::IV => "IV",
                };
                CatalogueRow {
                    id: label.into(),
                    cells: vec![CatalogueCell::text(label), CatalogueCell::number(co2_above_outdoor_ppm(cat), 0)],
                }
            })
            .collect(),
    }
}

fn table_b6_b7_outdoor_air() -> CatalogueTable {
    CatalogueTable {
        id: "table-b6-b7-outdoor-air",
        title_en: "Outdoor air rates (Tables B.6/B.7, low pollution)",
        title_de: "Außenluftraten (Tabellen B.6/B.7, geringe Belastung)",
        clause: ClauseId::new("EN 16798-1", "Annex B", "B.6/B.7"),
        columns: vec![
            CatalogueColumn { id: "cat", label_en: "Category", label_de: "Kategorie", unit: None },
            CatalogueColumn { id: "qp", label_en: "q_p", label_de: "q_p", unit: Some("L/(s·pers)") },
            CatalogueColumn { id: "qa", label_en: "q_a", label_de: "q_a", unit: Some("L/(s·m²)") },
        ],
        rows: [ComfortCategory::I, ComfortCategory::II, ComfortCategory::III, ComfortCategory::IV]
            .into_iter()
            .map(|cat| {
                let label = match cat {
                    ComfortCategory::I => "I",
                    ComfortCategory::II => "II",
                    ComfortCategory::III => "III",
                    ComfortCategory::IV => "IV",
                };
                CatalogueRow {
                    id: label.into(),
                    cells: vec![
                        CatalogueCell::text(label),
                        CatalogueCell::number(outdoor_air_per_person_l_s(cat), 1),
                        CatalogueCell::number(outdoor_air_per_area_l_s_m2(cat, PollutionClass::Low), 2),
                    ],
                }
            })
            .collect(),
    }
}

fn annex_co2_absolute() -> CatalogueTable {
    let en = AnnexParams::en();
    let de = AnnexParams::de();
    CatalogueTable {
        id: "annex-co2-absolute",
        title_en: "Absolute CO₂ limits (national annex)",
        title_de: "Absolute CO₂-Grenzwerte (nationaler Anhang)",
        clause: ClauseId::new("EN 16798-1", "Annex", "CO2"),
        columns: vec![
            CatalogueColumn { id: "usage", label_en: "Usage", label_de: "Nutzung", unit: None },
            CatalogueColumn { id: "en", label_en: "EN", label_de: "EN", unit: Some("ppm") },
            CatalogueColumn { id: "de", label_en: "DE NA", label_de: "DE NA", unit: Some("ppm") },
        ],
        rows: vec![
            CatalogueRow {
                id: "residential".into(),
                cells: vec![
                    CatalogueCell::text("Residential / Wohnen"),
                    CatalogueCell::number(en.co2_absolute_residential_ppm, 0),
                    CatalogueCell::number(de.co2_absolute_residential_ppm, 0),
                ],
            },
            CatalogueRow {
                id: "classroom".into(),
                cells: vec![
                    CatalogueCell::text("Classroom / Unterricht"),
                    CatalogueCell::number(en.co2_absolute_classroom_ppm, 0),
                    CatalogueCell::number(de.co2_absolute_classroom_ppm, 0),
                ],
            },
            CatalogueRow {
                id: "other".into(),
                cells: vec![
                    CatalogueCell::text("Other / Sonstige"),
                    CatalogueCell::number(en.co2_absolute_other_ppm, 0),
                    CatalogueCell::number(de.co2_absolute_other_ppm, 0),
                ],
            },
        ],
    }
}
//#endregion 🔖️Tables

//#region 🔖️Render
/// 📚️ Lists every declared example plus normative DIN EN 16798 reference tables.
pub fn render(
    examples: Vec<semio_framework_plugin::ExampleSource>,
    locale: semio_framework_plugin::Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let tables = reference_tables();
    crate::app_surface::render_catalogue(&examples, &tables, locale, controller_id, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
