//! 📚️ DIN V 18599 play app panel — examples plus GEG / DIN V 18599 reference tables.

use semio_s_plugin_norm_app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use semio_s_artifact_norm_din18599_checks::{annex2_reference_values, annex3_limit, Annex3EnvelopeClass,din_v_18599_10_dhw_specific, din_v_18599_10_fan_hours, din_v_18599_10_lighting_hours, din_v_18599_4_lighting_power_density, geg_anlage1_reference_u, GEG_ANLAGE1_DELTA_U_WB_REF, GEG_SECTION_16_HT_PRIME_RATIO, geg_anlage4_primary_energy_factors};
use semio_s_plugin_norm_compliance::ClauseId;
use semio_s_artifact_norm_din18599_model::{Adjacency, ElementKind, MonthlyClimate};
use semio_ui_locale::LocalizedLabel;
use semio_plugin::PanelGroup;
use semio_plugin::PanelTabDefinition;
use semio_plugin::TreeWindows;
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.din18599.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    semio_s_plugin_norm_app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 🔖️Tables
/// 📚️ Normative reference tables beside examples (GEG § 16 and Anlagen 1 and 4, DIN V 18599-4/-10, Potsdam TRY).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![
        primary_energy_factors(),
        ht_prime_limits(),
        nonresidential_u_limits(),
        residential_reference_u(),
        nonresidential_reference_u(),
        lighting_power_density(),
        usage_profile_hours_dhw(),
        residential_hot_water(),
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
        rows: f::ROWS.iter().map(|row| CatalogueRow {
            id: row.carrier.into(),
            cells: vec![
                CatalogueCell::text(format!("{} / {}", row.en, row.de)),
                row.factor.map_or_else(|| CatalogueCell::text("Method B / Verfahren B"), |value| CatalogueCell::number(value, 1)),
            ],
        }).collect(),
    }
}

fn nonresidential_reference_u() -> CatalogueTable {
    use ElementKind::*;
    let entries = [
        (Wall, Adjacency::Outdoor, "Exterior wall", "Außenwand"),
        (CurtainWall, Adjacency::Outdoor, "Curtain wall", "Vorhangfassade"),
        (Floor, Adjacency::Ground, "Ground slab", "Bodenplatte"),
        (Roof, Adjacency::Outdoor, "Roof", "Dach"),
        (RoofGlazing, Adjacency::Outdoor, "Roof glazing", "Glasdach"),
        (RooflightStrip, Adjacency::Outdoor, "Rooflight strip", "Lichtband"),
        (RoofDome, Adjacency::Outdoor, "Rooflight dome", "Lichtkuppel"),
        (Window, Adjacency::Outdoor, "Facade window", "Fassadenfenster"),
        (RoofWindow, Adjacency::Outdoor, "Roof window", "Dachflächenfenster"),
        (Door, Adjacency::Outdoor, "Exterior door", "Außentür"),
    ];
    CatalogueTable {
        id: "geg-anlage2-reference-envelope",
        title_en: "Non-residential reference envelope",
        title_de: "Referenzhülle für Nichtwohngebäude",
        clause: ClauseId::new("GModG", "Anlage 2", "1.1–1.11"),
        columns: vec![
            CatalogueColumn { id: "component", label_en: "Component", label_de: "Bauteil", unit: None },
            CatalogueColumn { id: "warm", label_en: "U at ≥ 19 °C", label_de: "U bei ≥ 19 °C", unit: Some("W/(m²·K)") },
            CatalogueColumn { id: "reduced", label_en: "U at 12 to < 19 °C", label_de: "U bei 12 bis < 19 °C", unit: Some("W/(m²·K)") },
            CatalogueColumn { id: "solar", label_en: "Solar g at ≥ 19 °C", label_de: "g-Wert bei ≥ 19 °C", unit: None },
            CatalogueColumn { id: "visible", label_en: "Light transmittance at ≥ 19 °C", label_de: "Lichttransmission bei ≥ 19 °C", unit: None },
        ],
        rows: entries.into_iter().filter_map(|(kind, adjacency, en, de)| {
            let normal = annex2_reference_values(kind, adjacency, 19.0)?;
            let reduced = annex2_reference_values(kind, adjacency, 12.0)?;
            let transmittance = |value: Option<f64>| value.map_or_else(|| CatalogueCell::text("—"), |value| CatalogueCell::number(value, 2));
            Some(CatalogueRow {
                id: normal.clause.into(),
                cells: vec![
                    CatalogueCell::text(format!("{en} / {de}")),
                    CatalogueCell::number(normal.u_value_w_m2k, 2),
                    CatalogueCell::number(reduced.u_value_w_m2k, 2),
                    transmittance(normal.solar_transmittance),
                    transmittance(normal.visible_transmittance),
                ],
            })
        }).collect(),
    }
}

fn nonresidential_u_limits() -> CatalogueTable {
    CatalogueTable {
        id: "geg-anlage3-mean-u-limits",
        title_en: "Non-residential mean U limits",
        title_de: "Grenzwerte der mittleren U-Werte für Nichtwohngebäude",
        clause: ClauseId::new("GModG", "Anlage 3", "1–4"),
        columns: vec![
            CatalogueColumn { id: "class", label_en: "Exterior component class", label_de: "Art des Außenbauteils", unit: None },
            CatalogueColumn { id: "warm", label_en: "Setpoint ≥ 19 °C", label_de: "Solltemperatur ≥ 19 °C", unit: Some("W/(m²·K)") },
            CatalogueColumn { id: "cool", label_en: "Setpoint 12 to < 19 °C", label_de: "Solltemperatur 12 bis < 19 °C", unit: Some("W/(m²·K)") },
        ],
        rows: Annex3EnvelopeClass::ALL.into_iter().map(|class| {
            let (en, de) = class.labels();
            CatalogueRow {
                id: class.clause().into(),
                cells: vec![
                    CatalogueCell::text(format!("{en} / {de}")),
                    CatalogueCell::number(annex3_limit(class, 19.0).unwrap_or_default(), 2),
                    CatalogueCell::number(annex3_limit(class, 12.0).unwrap_or_default(), 2),
                ],
            }
        }).collect(),
    }
}

fn ht_prime_limits() -> CatalogueTable {
    CatalogueTable {
        id: "geg-section16-ht-prime",
        title_en: "Specific transmission loss limit",
        title_de: "Grenzwert des spezifischen Transmissionswärmeverlusts",
        clause: ClauseId::new("GEG", "§ 16", "Anlage 1"),
        columns: vec![
            CatalogueColumn { id: "case", label_en: "Building case", label_de: "Gebäudefall", unit: None },
            CatalogueColumn { id: "ratio", label_en: "Maximum ratio to reference", label_de: "Maximalverhältnis zur Referenz", unit: None },
        ],
        rows: vec![
            CatalogueRow { id: "residential-same-geometry".into(), cells: vec![CatalogueCell::text("Residential, same envelope geometry / Wohngebäude, gleiche Hüllgeometrie"), CatalogueCell::number(GEG_SECTION_16_HT_PRIME_RATIO, 1)] },
        ],
    }
}

fn residential_reference_u() -> CatalogueTable {
    use geg_anlage1_reference_u as u;
    CatalogueTable {
        id: "geg-anlage1-residential-envelope",
        title_en: "Residential reference envelope",
        title_de: "Referenzhülle für Wohngebäude",
        clause: ClauseId::new("GEG", "Anlage 1", "1.1–1.4, 1.7–2"),
        columns: vec![
            CatalogueColumn { id: "kind", label_en: "Envelope input", label_de: "Eingabe zur Gebäudehülle", unit: None },
            CatalogueColumn { id: "u_ref", label_en: "Reference value", label_de: "Referenzwert", unit: Some("W/(m²·K)") },
        ],
        rows: vec![
            CatalogueRow { id: "wall-floor-outdoor".into(), cells: vec![CatalogueCell::text("Wall or floor to outdoors / Wand oder Decke gegen Außenluft"), CatalogueCell::number(u::WALL_AND_FLOOR_TO_OUTDOORS, 2)] },
            CatalogueRow { id: "wall-floor-ground-unheated".into(), cells: vec![CatalogueCell::text("Wall or floor to ground or unheated space / Wand oder Boden gegen Erdreich oder unbeheizten Raum"), CatalogueCell::number(u::WALL_AND_FLOOR_TO_GROUND_OR_UNHEATED, 2)] },
            CatalogueRow { id: "roof".into(), cells: vec![CatalogueCell::text("Roof / Dach"), CatalogueCell::number(u::ROOF, 2)] },
            CatalogueRow { id: "window".into(), cells: vec![CatalogueCell::text("Window / Fenster"), CatalogueCell::number(u::WINDOW, 1)] },
            CatalogueRow { id: "door".into(), cells: vec![CatalogueCell::text("Door / Tür"), CatalogueCell::number(u::DOOR, 1)] },
            CatalogueRow { id: "thermal-bridges".into(), cells: vec![CatalogueCell::text("Thermal-bridge surcharge / Wärmebrückenzuschlag ΔU_WB"), CatalogueCell::number(GEG_ANLAGE1_DELTA_U_WB_REF, 2)] },
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

fn residential_hot_water() -> CatalogueTable {
    CatalogueTable {
        id: "din18599-10-residential-dhw",
        title_en: "Residential useful hot-water demand",
        title_de: "Trinkwarmwasser-Nutzenergie im Wohngebäude",
        clause: ClauseId::new("DIN V 18599", "10", "Tabelle 4"),
        columns: vec![
            CatalogueColumn { id: "area", label_en: "Mean dwelling area", label_de: "Mittlere Wohnungsgröße", unit: Some("m²") },
            CatalogueColumn { id: "specific", label_en: "Useful hot-water demand", label_de: "Trinkwarmwasser-Nutzenergie", unit: Some("kWh/(m²·a)") },
        ],
        rows: [40_u32, 70, 140, 160, 200].into_iter().map(|area| {
            let area_m2 = f64::from(area);
            let demand = semio_s_artifact_norm_din18599_checks::residential_dhw_useful_kwh_a(area_m2, 1).unwrap_or_default();
            CatalogueRow {
                id: area.to_string(),
                cells: vec![CatalogueCell::number(area_m2, 0), CatalogueCell::number(demand / area_m2, 1)],
            }
        }).collect(),
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
    examples: Vec<semio_plugin::ExampleSource>,
    locale: semio_ui_locale::Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    semio_s_plugin_norm_app_surface::render_catalogue(&examples, &reference_tables(), locale, controller_id, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
//#endregion 🧪️Tests
