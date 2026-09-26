//! 📚️ EN 1990 play app panel — examples plus DE NA / EN ψ tables and importance factors.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::artifact_schema::{psi_for_category, NaDe, NaEn};
use crate::document::{ClauseId, NationalAnnex};
use crate::ImportanceClass;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

pub const BODY_CATALOGUE: &str = "norm.en1990.play.catalogue";

pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}

/// 📚️ Normative reference tables beside examples (EN 1990 A1.1 / DIN EN 1990/NA ψ + EN 1998-1 γ_I).
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![psi_table_en(), psi_table_de(), importance_gamma_i()]
}

fn psi_columns() -> Vec<CatalogueColumn> {
    vec![
        CatalogueColumn { id: "category", label_en: "Category", label_de: "Kategorie", unit: None },
        CatalogueColumn { id: "psi0", label_en: "ψ₀", label_de: "ψ₀", unit: None },
        CatalogueColumn { id: "psi1", label_en: "ψ₁", label_de: "ψ₁", unit: None },
        CatalogueColumn { id: "psi2", label_en: "ψ₂", label_de: "ψ₂", unit: None },
    ]
}

fn psi_row(key: &str, label_en: &str, label_de: &str, annex: &impl NationalAnnex) -> CatalogueRow {
    let row = psi_for_category(annex, key);
    CatalogueRow {
        id: key.into(),
        cells: vec![
            CatalogueCell::text(format!("{label_en} / {label_de}")),
            CatalogueCell::number(row.psi_0, 2),
            CatalogueCell::number(row.psi_1, 2),
            CatalogueCell::number(row.psi_2, 2),
        ],
    }
}

fn psi_table_en() -> CatalogueTable {
    let annex = NaEn;
    CatalogueTable {
        id: "table-a1-1-psi-en",
        title_en: "Combination factors ψ (EN)",
        title_de: "Kombinationsbeiwerte ψ (EN)",
        clause: ClauseId::new("EN 1990", "A1.1", "Table A1.1"),
        columns: psi_columns(),
        rows: vec![
            psi_row("residential", "Residential", "Wohnen", &annex),
            psi_row("office", "Office", "Büro", &annex),
            psi_row("congregation", "Congregation", "Versammlung", &annex),
            psi_row("retail", "Retail", "Verkauf", &annex),
            psi_row("storage", "Storage", "Lager", &annex),
            psi_row("snow", "Snow", "Schnee", &annex),
            psi_row("wind", "Wind", "Wind", &annex),
            psi_row("temperature", "Temperature", "Temperatur", &annex),
        ],
    }
}

fn psi_table_de() -> CatalogueTable {
    let annex = NaDe;
    CatalogueTable {
        id: "table-na-a1-1-psi-de",
        title_en: "Combination factors ψ (DE NA)",
        title_de: "Kombinationsbeiwerte ψ (DE NA)",
        clause: ClauseId::new("DIN EN 1990/NA", "NA.A.1.1", "Table NA.A.1.1"),
        columns: psi_columns(),
        rows: vec![
            psi_row("residential", "Residential", "Wohnen", &annex),
            psi_row("office", "Office", "Büro", &annex),
            psi_row("congregation", "Congregation", "Versammlung", &annex),
            psi_row("retail", "Retail", "Verkauf", &annex),
            psi_row("storage", "Storage", "Lager", &annex),
            psi_row("snow", "Snow ≤1000 m", "Schnee ≤1000 m", &annex),
            psi_row("snow_high", "Snow >1000 m", "Schnee >1000 m", &annex),
            psi_row("wind", "Wind", "Wind", &annex),
            psi_row("temperature", "Temperature", "Temperatur", &annex),
        ],
    }
}

fn importance_gamma_i() -> CatalogueTable {
    CatalogueTable {
        id: "importance-gamma-i",
        title_en: "Importance factor γ_I",
        title_de: "Bedeutungsbeiwert γ_I",
        clause: ClauseId::new("EN 1998-1", "4.2.5", "Table 4.3"),
        columns: vec![
            CatalogueColumn { id: "class", label_en: "Class", label_de: "Kategorie", unit: None },
            CatalogueColumn { id: "gamma", label_en: "γ_I", label_de: "γ_I", unit: None },
        ],
        rows: [ImportanceClass::I, ImportanceClass::II, ImportanceClass::III, ImportanceClass::IV]
            .into_iter()
            .map(|class| {
                let label = format!("{class:?}");
                CatalogueRow {
                    id: label.clone(),
                    cells: vec![CatalogueCell::text(label), CatalogueCell::number(class.gamma_i(), 1)],
                }
            })
            .collect(),
    }
}

pub fn render(
    examples: Vec<semio_framework_plugin::ExampleSource>,
    locale: semio_framework_plugin::Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let tables = reference_tables();
    crate::app_surface::render_catalogue(&examples, &tables, locale, controller_id, windows)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
