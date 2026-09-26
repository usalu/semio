//! 📚️ VDI 3805 play app panel — examples plus Blatt code-list catalogue tables.

use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::document::ClauseId;

pub const BODY_CATALOGUE: &str = "norm.vdi3805.play.catalogue";

pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}

/// 📋 Blatt-sourced connection types (VDI 3805-1 §6.2 / product-sheet Anschlussart code lists).
fn connection_type_table() -> CatalogueTable {
    CatalogueTable {
        id: "connection-types",
        title_en: "Connection types",
        title_de: "Anschlussarten",
        clause: ClauseId::new("VDI 3805", "1", "6.2"),
        columns: vec![
            CatalogueColumn { id: "code", label_en: "Code", label_de: "Code", unit: None },
            CatalogueColumn { id: "en", label_en: "Meaning", label_de: "Bedeutung", unit: None },
            CatalogueColumn { id: "de", label_en: "DE", label_de: "DE", unit: None },
        ],
        rows: vec![
            CatalogueRow { id: "flange".into(), cells: vec![CatalogueCell::text("flange"), CatalogueCell::text("Flanged"), CatalogueCell::text("Flansch")] },
            CatalogueRow { id: "thread".into(), cells: vec![CatalogueCell::text("thread"), CatalogueCell::text("Threaded"), CatalogueCell::text("Gewinde")] },
            CatalogueRow { id: "weld".into(), cells: vec![CatalogueCell::text("weld"), CatalogueCell::text("Welded"), CatalogueCell::text("Schweißanschluss")] },
            CatalogueRow { id: "press".into(), cells: vec![CatalogueCell::text("press"), CatalogueCell::text("Press fit"), CatalogueCell::text("Pressverbindung")] },
        ],
    }
}

/// 📋 Pressure classes PN (VDI 3805 product sheets / DIN EN 1092 pressure series).
fn pressure_class_table() -> CatalogueTable {
    CatalogueTable {
        id: "pressure-classes",
        title_en: "Pressure classes",
        title_de: "Druckstufen",
        clause: ClauseId::new("VDI 3805", "1", "6.3"),
        columns: vec![
            CatalogueColumn { id: "code", label_en: "PN", label_de: "PN", unit: None },
            CatalogueColumn { id: "bar", label_en: "Nominal pressure", label_de: "Nenndruck", unit: Some("bar") },
        ],
        rows: [6, 10, 16, 25]
            .into_iter()
            .map(|pn| CatalogueRow {
                id: format!("PN{pn}"),
                cells: vec![CatalogueCell::text(format!("PN{pn}")), CatalogueCell::number(pn as f64, 0)],
            })
            .collect(),
    }
}

/// 📋 Filter classes for Blatt 19 (ISO 16890 / EN 779 legacy mapping used by VDI 3805-19).
fn filter_class_table() -> CatalogueTable {
    CatalogueTable {
        id: "filter-classes",
        title_en: "Filter classes (Blatt 19)",
        title_de: "Filterklassen (Blatt 19)",
        clause: ClauseId::new("VDI 3805", "19", "4.1"),
        columns: vec![
            CatalogueColumn { id: "code", label_en: "Class", label_de: "Klasse", unit: None },
            CatalogueColumn { id: "en", label_en: "Meaning", label_de: "Bedeutung", unit: None },
        ],
        rows: ["G4", "M5", "M6", "F7", "F8", "F9", "ePM1", "ePM2_5", "ePM10"]
            .into_iter()
            .map(|c| CatalogueRow { id: c.into(), cells: vec![CatalogueCell::text(c), CatalogueCell::text(c)] })
            .collect(),
    }
}

/// 📚️ Normative reference tables shown beside examples.
pub fn catalogue_tables() -> Vec<CatalogueTable> {
    vec![connection_type_table(), pressure_class_table(), filter_class_table()]
}

/// 🏷️ Alias used by the shared catalogue unit-test template.
pub fn reference_tables() -> Vec<CatalogueTable> {
    catalogue_tables()
}

pub fn render(
    examples: Vec<semio_framework_plugin::ExampleSource>,
    locale: semio_framework_plugin::Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let tables = catalogue_tables();
    crate::app_surface::render_catalogue(&examples, &tables, locale, controller_id, windows)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
