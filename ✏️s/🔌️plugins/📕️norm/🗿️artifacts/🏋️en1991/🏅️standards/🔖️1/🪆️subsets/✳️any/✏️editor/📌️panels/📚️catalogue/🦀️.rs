//! 📚️ EN 1991 play app panel — examples plus Table 6.1 / snow-zone / wind q_p reference tables.

use semio_s_plugin_norm_app_surface::{spec_trace_table, CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use semio_s_artifact_norm_en1991_checks::spec_trace::TABLE as SPEC_TRACE;
use semio_s_plugin_norm_compliance::{AnnexChoice, ClauseId};
use semio_s_artifact_norm_en1991_checks::{na_de, part_1_1, part_1_3, part_1_4};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1991.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    semio_s_plugin_norm_app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

/// 📚️ Normative reference tables shared with `evaluate()` (Table 6.1 / NA, snow zones, DE NA B.3 q_p).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![imposed_qk_de(), imposed_qk_en(), snow_zone_map(), wind_qp_na_b3(), spec_trace_table(&SPEC_TRACE)]
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
            CatalogueCell::number(part_1_1::imposed_qk_pa(category, annex).unwrap_or_default(), 0),
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
        rows: part_1_1::categories(AnnexChoice::De).iter().map(|c| imposed_row(c, AnnexChoice::De)).collect(),
    }
}

fn imposed_qk_en() -> CatalogueTable {
    CatalogueTable {
        id: "table-6-1-imposed-qk-en",
        title_en: "Imposed floor load q_k (EN)",
        title_de: "Nutzlast q_k auf Decken (EN)",
        clause: ClauseId::new("EN 1991-1-1", "Table 6.2", "6.3.1"),
        columns: imposed_columns(),
        rows: part_1_1::categories(AnnexChoice::En).iter().map(|c| imposed_row(c, AnnexChoice::En)).collect(),
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
                    CatalogueCell::number(part_1_3::ground_snow_pa(zone, alt).unwrap_or_default(), 0),
                ],
            })
            .collect(),
    }
}

fn wind_qp_na_b3() -> CatalogueTable {
    CatalogueTable {
        id: "na-profile-qp-inland",
        title_en: "Peak velocity pressure q_p (DE NA inland profile)",
        title_de: "Böengeschwindigkeitsdruck q_p (DE NA Binnenlandprofil)",
        clause: ClauseId::new("DIN EN 1991-1-4/NA", "NA.B.3", "Table B.3"),
        columns: vec![
            CatalogueColumn { id: "zone", label_en: "Wind zone", label_de: "Windzone", unit: None },
            CatalogueColumn { id: "z", label_en: "Height z", label_de: "Höhe z", unit: Some("m") },
            CatalogueColumn { id: "qp", label_en: "q_p", label_de: "q_p", unit: Some("Pa") },
        ],
        rows: [1_u8, 2, 3, 4]
            .into_iter()
            .map(|zone| CatalogueRow {
                id: format!("z{zone}-inland-10m"),
                cells: vec![
                    CatalogueCell::text(format!("{zone}")),
                    CatalogueCell::number(10.0, 0),
                    CatalogueCell::number(part_1_4::na_profile_pa(part_1_4::basic_velocity_pressure_pa(1.25, na_de::WindZone::from_u8(zone).map(na_de::WindZone::v_b0_m_s).unwrap_or_default()), 10.0, false), 0),
                ],
            })
            .collect(),
    }
}

//#region 🔖️Render
pub fn render(examples: Vec<semio_framework_plugin::ExampleSource>, locale: semio_framework_ui_locale::Locale, controller_id: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut tables = reference_tables();
    tables.push(semio_s_plugin_norm_app_surface::case_catalogue_table("en1991", &<semio_s_artifact_norm_en1991_checks::catalogue::En1991Cases as semio_s_plugin_norm_compliance::design_case::CaseCatalogue>::entries(), "", locale));
    semio_s_plugin_norm_app_surface::render_catalogue(&examples, &tables, locale, controller_id, &semio_framework_plugin::TreeWindows::unhosted())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
