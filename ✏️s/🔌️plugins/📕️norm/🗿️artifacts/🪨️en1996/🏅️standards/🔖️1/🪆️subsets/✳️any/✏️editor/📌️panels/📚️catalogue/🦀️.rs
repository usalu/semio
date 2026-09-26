//! 📚️ EN 1996 play app panel — examples plus DE/EN masonry reference tables.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::artifact_schema::{fire_min_thickness_m, fk_factors, psi0_imposed, F_VLT_OVER_FB_DE};
use crate::document::{AnnexChoice, ClauseId};
use crate::{MasonryClass, MortarType, UnitGroup, UnitMaterial};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1996.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

fn column(id: &'static str, label_en: &'static str, label_de: &'static str, unit: Option<&'static str>) -> CatalogueColumn {
    CatalogueColumn { id, label_en, label_de, unit }
}

/// 📚️ Normative reference tables sourced from the same helpers `evaluate()` reads.
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![
        gamma_m_de_table(),
        fk_factors_table(),
        fire_min_thickness_rei90_table(),
        psi0_imposed_table(),
        f_vk_de_cap_table(),
    ]
}

fn gamma_m_de_table() -> CatalogueTable {
    let classes = [
        MasonryClass::Class1,
        MasonryClass::Class2,
        MasonryClass::Class3,
        MasonryClass::Class4,
        MasonryClass::Class5,
    ];
    CatalogueTable {
        id: "de-gamma-m-by-class",
        title_en: "Partial factor γ_M by masonry class (DE NA)",
        title_de: "Teilsicherheitsbeiwert γ_M nach Mauerwerksklasse (DE NA)",
        clause: ClauseId::new("DIN EN 1996-1-1/NA", "§2", "NA.2.4.3"),
        columns: vec![
            column("class", "Masonry class", "Mauerwerksklasse", None),
            column("gamma_persistent", "γ_M persistent", "γ_M ständig", None),
            column("gamma_accidental", "γ_M accidental", "γ_M außergewöhnlich", None),
        ],
        rows: classes
            .into_iter()
            .map(|class| {
                let label = format!("{class:?}");
                CatalogueRow {
                    id: label.clone(),
                    cells: vec![
                        CatalogueCell::text(label),
                        CatalogueCell::number(class.gamma_m_de(false), 1),
                        CatalogueCell::number(class.gamma_m_de(true), 1),
                    ],
                }
            })
            .collect(),
    }
}

fn fk_factors_table() -> CatalogueTable {
    let specs: &[(&str, &str, &str, AnnexChoice, UnitMaterial, UnitGroup, MortarType)] = &[
        ("de-clay-g1-gp", "DE clay Group 1 GP", "DE Ziegel Gruppe 1 NM", AnnexChoice::De, UnitMaterial::Clay, UnitGroup::Group1, MortarType::GeneralPurpose),
        ("de-clay-g2-gp", "DE clay Group 2 GP", "DE Ziegel Gruppe 2 NM", AnnexChoice::De, UnitMaterial::Clay, UnitGroup::Group2, MortarType::GeneralPurpose),
        ("de-aac-g1-gp", "DE AAC Group 1 GP", "DE Porenbeton Gruppe 1 NM", AnnexChoice::De, UnitMaterial::Aerated, UnitGroup::Group1, MortarType::GeneralPurpose),
        ("en-clay-g1-gp", "EN clay Group 1 GP", "EN Ziegel Gruppe 1 NM", AnnexChoice::En, UnitMaterial::Clay, UnitGroup::Group1, MortarType::GeneralPurpose),
        ("en-clay-g1-tl", "EN clay Group 1 thin", "EN Ziegel Gruppe 1 Dünnbett", AnnexChoice::En, UnitMaterial::Clay, UnitGroup::Group1, MortarType::ThinLayer),
        ("de-clay-g1-tl", "DE clay Group 1 thin", "DE Ziegel Gruppe 1 Dünnbett", AnnexChoice::De, UnitMaterial::Clay, UnitGroup::Group1, MortarType::ThinLayer),
    ];
    CatalogueTable {
        id: "fk-factors-de-en",
        title_en: "Characteristic strength factors K, α, β for f_k",
        title_de: "Charakteristische Festigkeitsfaktoren K, α, β für f_k",
        clause: ClauseId::new("EN 1996-1-1", "§3.6.1.2", "3.6.1.2"),
        columns: vec![
            column("row", "Annex · material · group · mortar", "Anhang · Material · Gruppe · Mörtel", None),
            column("k", "K", "K", None),
            column("alpha", "α", "α", None),
            column("beta", "β", "β", None),
        ],
        rows: specs
            .iter()
            .map(|(id, en, de, annex, material, group, mortar)| {
                let f = fk_factors(*annex, *material, *group, *mortar);
                CatalogueRow {
                    id: (*id).into(),
                    cells: vec![
                        CatalogueCell::text(format!("{en} / {de}")),
                        CatalogueCell::number(f.k, 2),
                        CatalogueCell::number(f.alpha, 2),
                        CatalogueCell::number(f.beta, 2),
                    ],
                }
            })
            .collect(),
    }
}

fn fire_min_thickness_rei90_table() -> CatalogueTable {
    let specs: &[(&str, &str, &str, UnitMaterial)] = &[
        ("rei90-clay", "Clay REI 90", "Ziegel REI 90", UnitMaterial::Clay),
        ("rei90-aerated", "Aerated REI 90", "Porenbeton REI 90", UnitMaterial::Aerated),
        ("rei90-calcium", "Calcium silicate REI 90", "Kalksandstein REI 90", UnitMaterial::CalciumSilicate),
        ("rei90-concrete", "Concrete REI 90", "Betonstein REI 90", UnitMaterial::Concrete),
    ];
    CatalogueTable {
        id: "fire-min-thickness-rei90",
        title_en: "Minimum wall thickness for REI 90 (α = 1.0)",
        title_de: "Mindestwanddicke für REI 90 (α = 1,0)",
        clause: ClauseId::new("EN 1996-1-2", "§4", "Table N.B"),
        columns: vec![
            column("row", "Unit material", "Steinmaterial", None),
            column("t_min", "t_min", "t_min", Some("m")),
        ],
        rows: specs
            .iter()
            .map(|(id, en, de, material)| {
                let t = fire_min_thickness_m(90, *material);
                CatalogueRow {
                    id: (*id).into(),
                    cells: vec![
                        CatalogueCell::text(format!("{en} / {de}")),
                        CatalogueCell::number(t, 3),
                    ],
                }
            })
            .collect(),
    }
}

fn psi0_imposed_table() -> CatalogueTable {
    let specs: &[(&str, &str, &str)] = &[
        ("A", "Category A residential", "Kategorie A Wohnen"),
        ("B", "Category B office", "Kategorie B Büro"),
        ("C", "Category C congregation", "Kategorie C Versammlung"),
        ("D", "Category D retail", "Kategorie D Verkauf"),
        ("E", "Category E storage", "Kategorie E Lager"),
        ("H", "Category H roofs", "Kategorie H Dächer"),
    ];
    CatalogueTable {
        id: "psi0-imposed-categories",
        title_en: "Combination factor ψ₀ for imposed categories",
        title_de: "Kombinationsbeiwert ψ₀ für Nutzungskategorien",
        clause: ClauseId::new("EN 1990", "A1.1", "Table A1.1"),
        columns: vec![
            column("category", "Category", "Kategorie", None),
            column("psi0", "ψ₀", "ψ₀", None),
        ],
        rows: specs
            .iter()
            .map(|(id, en, de)| CatalogueRow {
                id: (*id).into(),
                cells: vec![
                    CatalogueCell::text(format!("{en} / {de}")),
                    CatalogueCell::number(psi0_imposed(id), 1),
                ],
            })
            .collect(),
    }
}

fn f_vk_de_cap_table() -> CatalogueTable {
    CatalogueTable {
        id: "f-vk-de-cap",
        title_en: "DE-NA shear cap ratio f_vlt / f_b",
        title_de: "DE-NA Schubdeckelverhältnis f_vlt / f_b",
        clause: ClauseId::new("DIN EN 1996-1-1/NA", "§6.2", "NA.6.2"),
        columns: vec![
            column("row", "Limit", "Grenze", None),
            column("ratio", "f_vlt / f_b", "f_vlt / f_b", None),
        ],
        rows: vec![CatalogueRow {
            id: "f-vlt-over-fb".into(),
            cells: vec![
                CatalogueCell::text("DE-NA upper limit / DE-NA Obergrenze"),
                CatalogueCell::number(F_VLT_OVER_FB_DE, 3),
            ],
        }],
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
