//! 📚️ EN 1997 play app panel — examples plus DIN 1054 / EN 1997-1 reference tables.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::artifact_schema::{part_1, resolve_params, resolve_upl_params, DesignApproach};
use crate::document::{AnnexChoice, ClauseId};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1997.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

/// 📚️ Normative reference tables (partial factors, UPL/HYD, Annex D bearing factors).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![partial_factors_table(), upl_hyd_factors_table(), bearing_n_factors_table()]
}

fn partial_factors_table() -> CatalogueTable {
    let specs: &[(&str, &str, &str, DesignApproach, AnnexChoice, &str)] = &[
        ("de-da2-bsp", "DE DA2* BS-P", "DE DA2* BS-P", DesignApproach::Da2, AnnexChoice::De, "bsP"),
        ("de-da2-bst", "DE DA2* BS-T", "DE DA2* BS-T", DesignApproach::Da2, AnnexChoice::De, "bsT"),
        ("de-da2-bsa", "DE DA2* BS-A", "DE DA2* BS-A", DesignApproach::Da2, AnnexChoice::De, "bsA"),
        ("en-da2-bsp", "EN DA2 BS-P", "EN DA2 BS-P", DesignApproach::Da2, AnnexChoice::En, "bsP"),
        ("de-geo3-bsp", "DE GEO-3 BS-P", "DE GEO-3 BS-P", DesignApproach::Geo3, AnnexChoice::De, "bsP"),
        ("en-da1str-bsp", "EN DA1-C1 BS-P", "EN DA1-C1 BS-P", DesignApproach::Da1Str, AnnexChoice::En, "bsP"),
    ];
    CatalogueTable {
        id: "din1054-partial-factors",
        title_en: "Partial factors γ_G, γ_Q, γ_R,v, γ_R,h",
        title_de: "Teilsicherheitsbeiwerte γ_G, γ_Q, γ_R,v, γ_R,h",
        clause: ClauseId::new("DIN 1054", "A", "A.2"),
        columns: vec![
            CatalogueColumn { id: "row", label_en: "Approach · annex · situation", label_de: "Ansatz · Anhang · Situation", unit: None },
            CatalogueColumn { id: "gamma_g", label_en: "γ_G", label_de: "γ_G", unit: None },
            CatalogueColumn { id: "gamma_q", label_en: "γ_Q", label_de: "γ_Q", unit: None },
            CatalogueColumn { id: "gamma_r_v", label_en: "γ_R,v", label_de: "γ_R,v", unit: None },
            CatalogueColumn { id: "gamma_r_h", label_en: "γ_R,h", label_de: "γ_R,h", unit: None },
        ],
        rows: specs
            .iter()
            .map(|(id, en, de, approach, annex, sit)| {
                let p = resolve_params(*approach, *annex, sit);
                CatalogueRow {
                    id: (*id).into(),
                    cells: vec![
                        CatalogueCell::text(format!("{en} / {de}")),
                        CatalogueCell::number(p.gamma_g, 2),
                        CatalogueCell::number(p.gamma_q, 2),
                        CatalogueCell::number(p.gamma_r_v, 2),
                        CatalogueCell::number(p.gamma_r_h, 2),
                    ],
                }
            })
            .collect(),
    }
}

fn upl_hyd_factors_table() -> CatalogueTable {
    let specs: &[(&str, &str, &str, AnnexChoice, &str)] = &[
        ("de-bsp", "DE BS-P", "DE BS-P", AnnexChoice::De, "bsP"),
        ("de-bst", "DE BS-T", "DE BS-T", AnnexChoice::De, "bsT"),
        ("de-bsa", "DE BS-A", "DE BS-A", AnnexChoice::De, "bsA"),
        ("en-bsp", "EN BS-P", "EN BS-P", AnnexChoice::En, "bsP"),
        ("en-bst", "EN BS-T", "EN BS-T", AnnexChoice::En, "bsT"),
        ("en-bsa", "EN BS-A", "EN BS-A", AnnexChoice::En, "bsA"),
    ];
    CatalogueTable {
        id: "upl-hyd-partial-factors",
        title_en: "UPL / HYD partial factors",
        title_de: "UPL- / HYD-Teilsicherheitsbeiwerte",
        clause: ClauseId::new("EN 1997-1", "10", "10.2"),
        columns: vec![
            CatalogueColumn { id: "row", label_en: "Annex · situation", label_de: "Anhang · Situation", unit: None },
            CatalogueColumn { id: "gamma_g_stb", label_en: "γ_G,stb", label_de: "γ_G,stb", unit: None },
            CatalogueColumn { id: "gamma_g_dst", label_en: "γ_G,dst", label_de: "γ_G,dst", unit: None },
            CatalogueColumn { id: "gamma_q_dst", label_en: "γ_Q,dst", label_de: "γ_Q,dst", unit: None },
            CatalogueColumn { id: "gamma_hyd", label_en: "γ_H", label_de: "γ_H", unit: None },
        ],
        rows: specs
            .iter()
            .map(|(id, en, de, annex, sit)| {
                let p = resolve_upl_params(*annex, sit);
                CatalogueRow {
                    id: (*id).into(),
                    cells: vec![
                        CatalogueCell::text(format!("{en} / {de}")),
                        CatalogueCell::number(p.gamma_g_stb, 2),
                        CatalogueCell::number(p.gamma_g_dst, 2),
                        CatalogueCell::number(p.gamma_q_dst, 2),
                        CatalogueCell::number(p.gamma_hyd, 2),
                    ],
                }
            })
            .collect(),
    }
}

fn bearing_n_factors_table() -> CatalogueTable {
    let phis = [20.0_f64, 25.0, 30.0, 35.0, 40.0];
    CatalogueTable {
        id: "annex-d-bearing-factors",
        title_en: "Bearing capacity factors N_q, N_c, N_γ",
        title_de: "Tragfähigkeitsbeiwerte N_q, N_c, N_γ",
        clause: ClauseId::new("EN 1997-1", "6", "D"),
        columns: vec![
            CatalogueColumn { id: "phi", label_en: "φ′", label_de: "φ′", unit: Some("°") },
            CatalogueColumn { id: "n_q", label_en: "N_q", label_de: "N_q", unit: None },
            CatalogueColumn { id: "n_c", label_en: "N_c", label_de: "N_c", unit: None },
            CatalogueColumn { id: "n_gamma", label_en: "N_γ", label_de: "N_γ", unit: None },
        ],
        rows: phis
            .iter()
            .map(|phi| CatalogueRow {
                id: format!("phi-{phi}"),
                cells: vec![
                    CatalogueCell::number(*phi, 0),
                    CatalogueCell::number(part_1::bearing_factor_n_q(*phi), 2),
                    CatalogueCell::number(part_1::bearing_factor_n_c(*phi), 2),
                    CatalogueCell::number(part_1::bearing_factor_n_gamma(*phi), 2),
                ],
            })
            .collect(),
    }
}

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
