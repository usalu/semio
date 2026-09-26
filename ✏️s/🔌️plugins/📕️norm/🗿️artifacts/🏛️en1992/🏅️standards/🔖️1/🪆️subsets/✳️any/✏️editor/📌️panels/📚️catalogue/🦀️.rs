//! 📚️ EN 1992 play app panel — examples plus Table 3.1 concrete and B500 reinforcing steel.

use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

use crate::{ConcreteGrade, ReinforcementGrade};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1992.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 📚️Materials
/// 🪨 Full concrete catalogue C12/15–C100/115 (EN 1992-1-1 Table 3.1).
pub fn concrete_catalogue() -> Vec<ConcreteGrade> {
    const CLASSES: &[(i32, &str)] = &[
        (12, "C12/15"),
        (16, "C16/20"),
        (20, "C20/25"),
        (25, "C25/30"),
        (30, "C30/37"),
        (35, "C35/45"),
        (40, "C40/50"),
        (45, "C45/55"),
        (50, "C50/60"),
        (55, "C55/67"),
        (60, "C60/75"),
        (70, "C70/85"),
        (80, "C80/95"),
        (90, "C90/105"),
        (100, "C100/115"),
    ];
    CLASSES
        .iter()
        .map(|(fck, name)| ConcreteGrade::from_f_ck(format!("c{fck}"), *name, (*fck as f64) * 1.0e6))
        .collect()
}

/// 🔩 Reinforcing steel catalogue B500A / B500B.
pub fn reinforcement_catalogue() -> Vec<ReinforcementGrade> {
    vec![ReinforcementGrade::b500a("b500a"), ReinforcementGrade::b500b("b500b")]
}

/// 📚 Markdown materials table for reports / debug.
pub fn catalogue_markdown() -> String {
    let mut out = String::from("# EN 1992 materials catalogue\n\n## Concrete (Table 3.1)\n\n| Class | f_ck [MPa] | f_cm | f_ctm | E_cm [GPa] | ε_cu2 |\n|---|---|---|---|---|---|\n");
    for g in concrete_catalogue() {
        out.push_str(&format!(
            "| {} | {:.0} | {:.0} | {:.2} | {:.1} | {:.2}‰ |\n",
            g.name,
            g.f_ck / 1e6,
            g.f_cm() / 1e6,
            g.f_ctm() / 1e6,
            g.e_cm() / 1e9,
            g.eps_cu2() * 1e3
        ));
    }
    out.push_str("\n## Reinforcing steel\n\n| Grade | f_yk [MPa] | k | ε_uk |\n|---|---|---|---|\n");
    for s in reinforcement_catalogue() {
        out.push_str(&format!(
            "| {} | {:.0} | {:.2} | {:.1}% |\n",
            s.name,
            s.f_yk / 1e6,
            s.k,
            s.eps_uk * 100.0
        ));
    }
    out
}
//#endregion 📚️Materials


/// 📚️ Normative materials tables (EN 1992-1-1 Table 3.1 + B500).
pub fn reference_tables() -> Vec<crate::app_surface::CatalogueTable> {
    use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
    use crate::document::ClauseId;
    let concrete_cols = vec![
        CatalogueColumn { id: "class", label_en: "Class", label_de: "Klasse", unit: None },
        CatalogueColumn { id: "fck", label_en: "f_ck", label_de: "f_ck", unit: Some("MPa") },
        CatalogueColumn { id: "fcm", label_en: "f_cm", label_de: "f_cm", unit: Some("MPa") },
        CatalogueColumn { id: "fctm", label_en: "f_ctm", label_de: "f_ctm", unit: Some("MPa") },
        CatalogueColumn { id: "ecm", label_en: "E_cm", label_de: "E_cm", unit: Some("GPa") },
    ];
    let concrete_rows = concrete_catalogue()
        .into_iter()
        .map(|g| CatalogueRow {
            id: g.id.clone(),
            cells: vec![
                CatalogueCell::text(&g.name),
                CatalogueCell::number(g.f_ck / 1.0e6, 0),
                CatalogueCell::number(g.f_cm() / 1.0e6, 0),
                CatalogueCell::number(g.f_ctm() / 1.0e6, 2),
                CatalogueCell::number(g.e_cm() / 1.0e9, 1),
            ],
        })
        .collect();
    let steel_cols = vec![
        CatalogueColumn { id: "grade", label_en: "Grade", label_de: "Sorte", unit: None },
        CatalogueColumn { id: "fyk", label_en: "f_yk", label_de: "f_yk", unit: Some("MPa") },
        CatalogueColumn { id: "k", label_en: "k", label_de: "k", unit: None },
        CatalogueColumn { id: "epsuk", label_en: "ε_uk", label_de: "ε_uk", unit: Some("%") },
    ];
    let steel_rows = reinforcement_catalogue()
        .into_iter()
        .map(|s| CatalogueRow {
            id: s.id.clone(),
            cells: vec![
                CatalogueCell::text(&s.name),
                CatalogueCell::number(s.f_yk / 1.0e6, 0),
                CatalogueCell::number(s.k, 2),
                CatalogueCell::number(s.eps_uk * 100.0, 1),
            ],
        })
        .collect();
    vec![
        CatalogueTable {
            id: "table-3-1-concrete",
            title_en: "Table 3.1 — Concrete strength classes",
            title_de: "Tabelle 3.1 — Betonfestigkeitsklassen",
            clause: ClauseId::new("EN 1992-1-1", "1-1", "3.1"),
            columns: concrete_cols,
            rows: concrete_rows,
        },
        CatalogueTable {
            id: "reinforcing-steel",
            title_en: "Reinforcing steel",
            title_de: "Betonstahl",
            clause: ClauseId::new("EN 1992-1-1", "1-1", "3.2"),
            columns: steel_cols,
            rows: steel_rows,
        },
    ]
}

//#region 🔖️Render
pub fn render(
    examples: Vec<semio_framework_plugin::ExampleSource>,
    locale: semio_framework_plugin::Locale,
    controller_id: &'static str,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_catalogue(&examples, &reference_tables(), locale, controller_id, &semio_framework_plugin::TreeWindows::unhosted())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
