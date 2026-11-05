//! 📚️ EN 1994 play app panel — selectable examples plus normative γ/ψ/stud tables.

use semio_s_plugin_norm_app_surface::{spec_trace_table, CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use semio_s_artifact_norm_en1994_checks::spec_trace::TABLE as SPEC_TRACE;
use semio_s_artifact_norm_en1994_checks::{part_1_1, part_en1990};
use semio_s_plugin_norm_compliance::{AnnexChoice, ClauseId};
use semio_s_artifact_norm_en1994_model::{CompositeBeam, Stud};
use semio_ui_locale::LocalizedLabel;
use semio_plugin::PanelGroup;
use semio_plugin::PanelTabDefinition;
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1994.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    semio_s_plugin_norm_app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

/// 📚️ Normative reference tables (γ_G/γ_Q, ψ factors, stud spacing limits) shared with `evaluate()`.
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![partial_factors_gamma(), psi_factors_table(), stud_spacing_limits_table(), spec_trace_table(&SPEC_TRACE)]
}

fn partial_factors_gamma() -> CatalogueTable {
    CatalogueTable {
        id: "en1990-partial-factors-gamma",
        title_en: "Partial factors γ_G and γ_Q",
        title_de: "Teilsicherheitsbeiwerte γ_G und γ_Q",
        clause: ClauseId::new("EN 1990", "A1.2", "Table A1.2(B)"),
        columns: vec![
            CatalogueColumn { id: "annex", label_en: "Annex", label_de: "Anhang", unit: None },
            CatalogueColumn { id: "gamma_g", label_en: "γ_G", label_de: "γ_G", unit: None },
            CatalogueColumn { id: "gamma_q", label_en: "γ_Q", label_de: "γ_Q", unit: None },
        ],
        rows: vec![
            CatalogueRow {
                id: "en".into(),
                cells: vec![
                    CatalogueCell::localized("EN recommended values", "EN-Empfehlungen"),
                    CatalogueCell::number(part_en1990::gamma_g(AnnexChoice::En, semio_s_plugin_norm_compliance::ReliabilityClass::RC2), 2),
                    CatalogueCell::number(part_en1990::gamma_q(AnnexChoice::En, semio_s_plugin_norm_compliance::ReliabilityClass::RC2), 2),
                ],
            },
            CatalogueRow {
                id: "de".into(),
                cells: vec![
                    CatalogueCell::localized("German NA", "Deutscher NA"),
                    CatalogueCell::number(part_en1990::gamma_g(AnnexChoice::De, semio_s_plugin_norm_compliance::ReliabilityClass::RC2), 2),
                    CatalogueCell::number(part_en1990::gamma_q(AnnexChoice::De, semio_s_plugin_norm_compliance::ReliabilityClass::RC2), 2),
                ],
            },
        ],
    }
}

fn psi_factors_table() -> CatalogueTable {
    let rows_spec: &[(&str, &str, &str, &str)] = &[
        ("permanent", "permanent", "Permanent", "Ständig"),
        ("imposed-B", "imposed", "Imposed B (office)", "Nutzlast B (Büro)"),
        ("imposed-C", "imposed", "Imposed C (assembly)", "Nutzlast C (Versammlung)"),
        ("snow", "snow", "Snow", "Schnee"),
        ("wind", "wind", "Wind", "Wind"),
    ];
    CatalogueTable {
        id: "en1990-psi-factors",
        title_en: "Combination factors ψ₀, ψ₁, ψ₂",
        title_de: "Kombinationsbeiwerte ψ₀, ψ₁, ψ₂",
        clause: ClauseId::new("EN 1990", "A1.1", "Table A1.1"),
        columns: vec![
            CatalogueColumn { id: "action", label_en: "Action", label_de: "Einwirkung", unit: None },
            CatalogueColumn { id: "psi0", label_en: "ψ₀", label_de: "ψ₀", unit: None },
            CatalogueColumn { id: "psi1", label_en: "ψ₁", label_de: "ψ₁", unit: None },
            CatalogueColumn { id: "psi2", label_en: "ψ₂", label_de: "ψ₂", unit: None },
        ],
        rows: rows_spec
            .iter()
            .map(|(id, kind, en, de)| {
                let cat = if *kind == "imposed" {
                    if id.ends_with('C') { "C" } else { "B" }
                } else {
                    ""
                };
                let (p0, p1, p2) = part_en1990::psi_factors(kind, cat);
                CatalogueRow {
                    id: (*id).into(),
                    cells: vec![
                        CatalogueCell::localized(en.to_string(), de.to_string()),
                        CatalogueCell::number(p0, 2),
                        CatalogueCell::number(p1, 2),
                        CatalogueCell::number(p2, 2),
                    ],
                }
            })
            .collect(),
    }
}

fn stud_spacing_limits_table() -> CatalogueTable {
    let beam = CompositeBeam::default_example();
    let (s_min, s_max) = part_1_1::stud_spacing_limits_m(&beam, Stud::typical("", "", 0.0).diameter_m);
    CatalogueTable {
        id: "en1994-stud-spacing-limits",
        title_en: "Stud spacing limits",
        title_de: "Grenzwerte Bolzenabstand",
        clause: ClauseId::new("EN 1994-1-1", "6.6.5.5", "6.6.5.5"),
        columns: vec![
            CatalogueColumn { id: "limit", label_en: "Limit", label_de: "Grenzwert", unit: None },
            CatalogueColumn { id: "value", label_en: "Value", label_de: "Wert", unit: Some("m") },
            CatalogueColumn { id: "rule", label_en: "Rule", label_de: "Regel", unit: None },
        ],
        rows: vec![
            CatalogueRow {
                id: "s-min".into(),
                cells: vec![
                    CatalogueCell::text("s_min"),
                    CatalogueCell::number(s_min, 4),
                    CatalogueCell::text(format!("{} × d", part_1_1::STUD_SPACING_MIN_DIAMETER_FACTOR)),
                ],
            },
            CatalogueRow {
                id: "s-max".into(),
                cells: vec![
                    CatalogueCell::text("s_max"),
                    CatalogueCell::number(s_max, 4),
                    CatalogueCell::text(format!("min({} × h_c, {} m)", part_1_1::STUD_SPACING_MAX_HC_FACTOR, part_1_1::STUD_SPACING_MAX_ABS_M)),
                ],
            },
        ],
    }
}

//#region 🔖️Render
pub fn render(examples: Vec<semio_plugin::ExampleSource>, locale: semio_ui_locale::Locale, controller_id: &'static str) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    let mut tables = reference_tables();
    tables.push(semio_s_plugin_norm_app_surface::case_catalogue_table("en1994", &<semio_s_artifact_norm_en1994_checks::case_catalogue::En1994Cases as semio_s_plugin_norm_compliance::design_case::CaseCatalogue>::entries(), "", locale));
    semio_s_plugin_norm_app_surface::render_catalogue(&examples, &tables, locale, controller_id, &semio_plugin::TreeWindows::unhosted())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
