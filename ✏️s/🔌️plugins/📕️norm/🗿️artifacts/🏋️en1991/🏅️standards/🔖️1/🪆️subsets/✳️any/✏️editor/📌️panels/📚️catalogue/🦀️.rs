//! 📚️ EN 1991 play app panel — examples plus Table 6.1 / snow-zone / wind q_p reference tables.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::document::{AnnexChoice, ClauseId};
use crate::standards::v1::subsets::any::schema::{part_1_1, part_1_3, part_1_4};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1991.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

/// 📚️ Normative reference tables shared with `evaluate()` (Table 6.1 / NA, snow zones, DE NA B.3 q_p).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![imposed_qk_de(), imposed_qk_en(), snow_zone_map(), wind_qp_na_b3()]
}

fn imposed_columns() -> Vec<CatalogueColumn> {
    vec![
        CatalogueColumn { id: "category", label_en: "Category", label_de: "Kategorie", unit: None },
        CatalogueColumn { id: "qk", label_en: "q_k", label_de: "q_k", unit: Some("Pa") },
    ]
}

fn imposed_row(category: &str, annex: AnnexChoice) -> CatalogueRow {
    CatalogueRow {
        id: category.into(),
        cells: vec![
            CatalogueCell::text(category.to_string()),
            CatalogueCell::number(part_1_1::imposed_qk_pa(category, annex), 0),
        ],
    }
}

fn imposed_qk_de() -> CatalogueTable {
    CatalogueTable {
        id: "table-6-1-imposed-qk-de",
        title_en: "Imposed floor load q_k (DE NA)",
        title_de: "Nutzlast q_k auf Decken (DE NA)",
        clause: ClauseId::new("DIN EN 1991-1-1/NA", "Table 6.1DE", "6.3.1"),
        columns: imposed_columns(),
        rows: ["A1", "A2", "B1", "B2", "C1", "C2", "D1", "E1", "F", "G", "H", "I", "J", "K"]
            .into_iter()
            .map(|c| imposed_row(c, AnnexChoice::De))
            .collect(),
    }
}

fn imposed_qk_en() -> CatalogueTable {
    CatalogueTable {
        id: "table-6-1-imposed-qk-en",
        title_en: "Imposed floor load q_k (EN)",
        title_de: "Nutzlast q_k auf Decken (EN)",
        clause: ClauseId::new("EN 1991-1-1", "Table 6.2", "6.3.1"),
        columns: imposed_columns(),
        rows: ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K"]
            .into_iter()
            .map(|c| imposed_row(c, AnnexChoice::En))
            .collect(),
    }
}

fn snow_zone_map() -> CatalogueTable {
    CatalogueTable {
        id: "snow-zone-sk-de",
        title_en: "Ground snow load s_k by DE zone",
        title_de: "Schneelast s_k nach DE-Zone",
        clause: ClauseId::new("DIN EN 1991-1-3/NA", "NA.2.1", "snow map"),
        columns: vec![
            CatalogueColumn { id: "zone", label_en: "Snow zone", label_de: "Schneelastzone", unit: None },
            CatalogueColumn { id: "altitude", label_en: "Altitude", label_de: "Höhe", unit: Some("m") },
            CatalogueColumn { id: "sk", label_en: "s_k", label_de: "s_k", unit: Some("Pa") },
        ],
        rows: [("1", 0.0), ("1a", 0.0), ("2", 0.0), ("2a", 0.0), ("3", 0.0)]
            .into_iter()
            .map(|(zone, alt)| CatalogueRow {
                id: zone.into(),
                cells: vec![
                    CatalogueCell::text(zone.to_string()),
                    CatalogueCell::number(alt, 0),
                    CatalogueCell::number(part_1_3::ground_snow_pa(zone, alt), 0),
                ],
            })
            .collect(),
    }
}

fn wind_qp_na_b3() -> CatalogueTable {
    CatalogueTable {
        id: "na-b3-qp-terrain-2",
        title_en: "Peak velocity pressure q_p (DE NA B.3, terrain II)",
        title_de: "Böengeschwindigkeitsdruck q_p (DE NA B.3, Gelände II)",
        clause: ClauseId::new("DIN EN 1991-1-4/NA", "NA.B.3", "Table B.3"),
        columns: vec![
            CatalogueColumn { id: "zone", label_en: "Wind zone", label_de: "Windzone", unit: None },
            CatalogueColumn { id: "z", label_en: "Height z", label_de: "Höhe z", unit: Some("m") },
            CatalogueColumn { id: "qp", label_en: "q_p", label_de: "q_p", unit: Some("Pa") },
        ],
        rows: [1_u8, 2, 3, 4]
            .into_iter()
            .map(|zone| CatalogueRow {
                id: format!("z{zone}-t2-10m"),
                cells: vec![
                    CatalogueCell::text(format!("{zone}")),
                    CatalogueCell::number(10.0, 0),
                    CatalogueCell::number(part_1_4::na_b3_qp_pa(zone, 2, 10.0), 0),
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
