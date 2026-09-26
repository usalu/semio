//! 📚️ DIN V 18599 play app panel — examples plus GEG / DIN V 18599 reference tables.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::artifact_schema::{
    din_v_18599_10_dhw_specific, din_v_18599_10_fan_hours, din_v_18599_10_lighting_hours, din_v_18599_12_tabelle5_qp_specific, din_v_18599_4_lighting_power_density, geg_anlage2_ht_prime_limits, geg_anlage2_reference_u, geg_anlage3_mean_u,
    geg_anlage4_primary_energy_factors,
};
use crate::document::ClauseId;
use crate::MonthlyClimate;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.din18599.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 🔖️Tables
/// 📚️ Normative reference tables beside examples (GEG Anlage 2–4, DIN V 18599-4/-10/-12, Potsdam TRY).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![
        primary_energy_factors(),
        ht_prime_limits(),
        reference_and_mean_u(),
        lighting_power_density(),
        usage_profile_hours_dhw(),
        tabular_qp_specific(),
        potsdam_monthly_climate(),
    ]
}

fn primary_energy_factors() -> CatalogueTable {
    use geg_anlage4_primary_energy_factors as f;
    CatalogueTable {
        id: "geg-anlage4-fp",
        title_en: "Primary energy factors f_P",
        title_de: "Primärenergiefaktoren f_P",
        clause: ClauseId::new("GEG", "Anlage 4", "f_P"),
        columns: vec![
            CatalogueColumn { id: "carrier", label_en: "Energy carrier", label_de: "Energieträger", unit: None },
            CatalogueColumn { id: "fp", label_en: "f_P", label_de: "f_P", unit: None },
        ],
        rows: vec![
            CatalogueRow { id: "natural_gas".into(), cells: vec![CatalogueCell::text("Natural gas / Erdgas"), CatalogueCell::number(f::NATURAL_GAS, 1)] },
            CatalogueRow { id: "heating_oil".into(), cells: vec![CatalogueCell::text("Heating oil / Heizöl"), CatalogueCell::number(f::HEATING_OIL, 1)] },
            CatalogueRow { id: "electricity".into(), cells: vec![CatalogueCell::text("Grid electricity / Netzstrom"), CatalogueCell::number(f::ELECTRICITY_GRID, 1)] },
            CatalogueRow { id: "district_heating".into(), cells: vec![CatalogueCell::text("District heating / Fernwärme"), CatalogueCell::number(f::DISTRICT_HEATING, 1)] },
            CatalogueRow { id: "biomass".into(), cells: vec![CatalogueCell::text("Biomass / Biomasse"), CatalogueCell::number(f::BIOMASS, 1)] },
        ],
    }
}

fn ht_prime_limits() -> CatalogueTable {
    use geg_anlage2_ht_prime_limits as lim;
    CatalogueTable {
        id: "geg-anlage2-ht-prime",
        title_en: "H′T limit values",
        title_de: "Grenzwerte H′T",
        clause: ClauseId::new("GEG", "Anlage 2", "H'T"),
        columns: vec![
            CatalogueColumn { id: "case", label_en: "Building case", label_de: "Gebäudefall", unit: None },
            CatalogueColumn { id: "limit", label_en: "H′T max", label_de: "H′T max", unit: Some("W/(m²·K)") },
        ],
        rows: vec![
            CatalogueRow { id: "detached-an-le-350".into(), cells: vec![CatalogueCell::text("Detached, A_N ≤ 350 m²"), CatalogueCell::number(lim::DETACHED_AN_LE_350, 2)] },
            CatalogueRow { id: "detached-an-gt-350".into(), cells: vec![CatalogueCell::text("Detached, A_N > 350 m²"), CatalogueCell::number(lim::DETACHED_AN_GT_350, 2)] },
            CatalogueRow { id: "semi-or-end-an-le-350".into(), cells: vec![CatalogueCell::text("Semi / end terrace, A_N ≤ 350 m²"), CatalogueCell::number(lim::SEMI_OR_END_AN_LE_350, 2)] },
            CatalogueRow { id: "semi-or-end-an-gt-350".into(), cells: vec![CatalogueCell::text("Semi / end terrace, A_N > 350 m²"), CatalogueCell::number(lim::SEMI_OR_END_AN_GT_350, 2)] },
            CatalogueRow { id: "mid-terrace".into(), cells: vec![CatalogueCell::text("Mid terrace"), CatalogueCell::number(lim::MID_TERRACE, 2)] },
        ],
    }
}

fn reference_and_mean_u() -> CatalogueTable {
    use geg_anlage2_reference_u as u2;
    use geg_anlage3_mean_u as u3;
    CatalogueTable {
        id: "geg-anlage2-3-u",
        title_en: "Reference / mean U-values",
        title_de: "Referenz- / mittlere U-Werte",
        clause: ClauseId::new("GEG", "Anlage 2/3", "U"),
        columns: vec![
            CatalogueColumn { id: "kind", label_en: "Element", label_de: "Bauteil", unit: None },
            CatalogueColumn { id: "u_ref", label_en: "U_ref (residential)", label_de: "U_ref (Wohngebäude)", unit: Some("W/(m²·K)") },
            CatalogueColumn { id: "u_mean", label_en: "Ū (non-residential)", label_de: "Ū (Nichtwohngebäude)", unit: Some("W/(m²·K)") },
        ],
        rows: vec![
            CatalogueRow { id: "wall".into(), cells: vec![CatalogueCell::text("Wall / Außenwand"), CatalogueCell::number(u2::WALL, 2), CatalogueCell::number(u3::WALL, 2)] },
            CatalogueRow { id: "roof".into(), cells: vec![CatalogueCell::text("Roof / Dach"), CatalogueCell::number(u2::ROOF, 2), CatalogueCell::number(u3::ROOF, 2)] },
            CatalogueRow { id: "floor".into(), cells: vec![CatalogueCell::text("Floor / Boden"), CatalogueCell::number(u2::FLOOR, 2), CatalogueCell::number(u3::FLOOR, 2)] },
            CatalogueRow { id: "window".into(), cells: vec![CatalogueCell::text("Window / Fenster"), CatalogueCell::number(u2::WINDOW, 1), CatalogueCell::number(u3::WINDOW, 1)] },
            CatalogueRow { id: "door".into(), cells: vec![CatalogueCell::text("Door / Tür"), CatalogueCell::number(u2::DOOR, 1), CatalogueCell::number(u3::DOOR, 1)] },
        ],
    }
}

fn lighting_power_density() -> CatalogueTable {
    use din_v_18599_4_lighting_power_density as p;
    CatalogueTable {
        id: "din18599-4-lpd",
        title_en: "Lighting power density p_LX",
        title_de: "spezifische Beleuchtungsleistung p_LX",
        clause: ClauseId::new("DIN V 18599", "4", "p_LX"),
        columns: vec![
            CatalogueColumn { id: "profile", label_en: "Usage profile", label_de: "Nutzungsprofil", unit: None },
            CatalogueColumn { id: "lpd", label_en: "p_LX max", label_de: "p_LX max", unit: Some("W/m²") },
        ],
        rows: vec![
            CatalogueRow { id: "residential".into(), cells: vec![CatalogueCell::text("Residential / Wohnen"), CatalogueCell::number(p::RESIDENTIAL, 0)] },
            CatalogueRow { id: "office".into(), cells: vec![CatalogueCell::text("Office / Büro"), CatalogueCell::number(p::OFFICE, 0)] },
            CatalogueRow { id: "school".into(), cells: vec![CatalogueCell::text("School / Schule"), CatalogueCell::number(p::SCHOOL, 0)] },
        ],
    }
}

fn usage_profile_hours_dhw() -> CatalogueTable {
    CatalogueTable {
        id: "din18599-10-hours-dhw",
        title_en: "Usage-profile hours and DHW",
        title_de: "Nutzungsprofil-Stunden und TWW",
        clause: ClauseId::new("DIN V 18599", "10", "profiles"),
        columns: vec![
            CatalogueColumn { id: "profile", label_en: "Usage profile", label_de: "Nutzungsprofil", unit: None },
            CatalogueColumn { id: "fan", label_en: "Fan hours", label_de: "Lüfterstunden", unit: Some("h/a") },
            CatalogueColumn { id: "light", label_en: "Lighting hours", label_de: "Beleuchtungsstunden", unit: Some("h/a") },
            CatalogueColumn { id: "dhw", label_en: "DHW q_w", label_de: "TWW q_w", unit: Some("kWh/(Person·a)") },
        ],
        rows: vec![
            CatalogueRow {
                id: "wfh".into(),
                cells: vec![
                    CatalogueCell::text("WFH / Wohnen"),
                    CatalogueCell::number(din_v_18599_10_fan_hours::RESIDENTIAL_WFH, 0),
                    CatalogueCell::number(din_v_18599_10_lighting_hours::WFH, 0),
                    CatalogueCell::number(din_v_18599_10_dhw_specific::RESIDENTIAL, 0),
                ],
            },
            CatalogueRow {
                id: "office".into(),
                cells: vec![
                    CatalogueCell::text("Office / Büro"),
                    CatalogueCell::number(din_v_18599_10_fan_hours::OFFICE, 0),
                    CatalogueCell::number(din_v_18599_10_lighting_hours::OFFICE, 0),
                    CatalogueCell::number(din_v_18599_10_dhw_specific::OFFICE, 0),
                ],
            },
            CatalogueRow {
                id: "school".into(),
                cells: vec![
                    CatalogueCell::text("School / Schule"),
                    CatalogueCell::number(din_v_18599_10_fan_hours::SCHOOL, 0),
                    CatalogueCell::number(din_v_18599_10_lighting_hours::SCHOOL, 0),
                    CatalogueCell::number(din_v_18599_10_dhw_specific::SCHOOL, 0),
                ],
            },
        ],
    }
}

fn tabular_qp_specific() -> CatalogueTable {
    use din_v_18599_12_tabelle5_qp_specific as q;
    CatalogueTable {
        id: "din18599-12-qp-tab",
        title_en: "Tabular specific primary energy q_p,tab",
        title_de: "tabellarische Primärenergiekennwerte q_p,tab",
        clause: ClauseId::new("DIN V 18599", "12", "Table 5"),
        columns: vec![
            CatalogueColumn { id: "profile", label_en: "Usage profile", label_de: "Nutzungsprofil", unit: None },
            CatalogueColumn { id: "qp", label_en: "q_p,tab", label_de: "q_p,tab", unit: Some("kWh/(m²·a)") },
        ],
        rows: vec![
            CatalogueRow { id: "residential".into(), cells: vec![CatalogueCell::text("Residential / Wohnen"), CatalogueCell::number(q::RESIDENTIAL, 0)] },
            CatalogueRow { id: "office".into(), cells: vec![CatalogueCell::text("Office / Büro"), CatalogueCell::number(q::OFFICE, 0)] },
            CatalogueRow { id: "school".into(), cells: vec![CatalogueCell::text("School / Schule"), CatalogueCell::number(q::SCHOOL, 0)] },
        ],
    }
}

fn potsdam_monthly_climate() -> CatalogueTable {
    let climate = MonthlyClimate::potsdam_reference();
    CatalogueTable {
        id: "din18599-10-potsdam-climate",
        title_en: "Potsdam reference climate",
        title_de: "Potsdam-Referenzklima",
        clause: ClauseId::new("DIN V 18599", "10", "TRY"),
        columns: vec![
            CatalogueColumn { id: "month", label_en: "Month", label_de: "Monat", unit: None },
            CatalogueColumn { id: "theta", label_en: "θ_e", label_de: "θ_e", unit: Some("°C") },
            CatalogueColumn { id: "g", label_en: "G_h", label_de: "G_h", unit: Some("W/m²") },
        ],
        rows: (0..12)
            .map(|m| CatalogueRow {
                id: format!("{}", m + 1),
                cells: vec![
                    CatalogueCell::text(format!("{}", m + 1)),
                    CatalogueCell::number(climate.theta_e_c[m], 1),
                    CatalogueCell::number(climate.g_h_w_m2[m], 0),
                ],
            })
            .collect(),
    }
}
//#endregion 🔖️Tables

//#region 🔖️Render
/// 📚️ Lists every `ExampleSource` plus normative reference tables with windowed catalogue rows.
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
