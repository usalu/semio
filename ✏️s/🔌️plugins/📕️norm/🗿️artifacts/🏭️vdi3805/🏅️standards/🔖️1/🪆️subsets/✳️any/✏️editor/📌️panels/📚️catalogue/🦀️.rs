//! 📚️ VDI 3805 play app panel — examples plus Blatt code-list catalogue tables.

use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::document::ClauseId;
use crate::{CONNECTION_TYPE_ROWS, FILTER_CLASS_ROWS, PRESSURE_CLASS_ROWS, SHEET_NUMERIC_BOUNDS_19_AIRFLOW, SHEET_NUMERIC_BOUNDS_53_COP, SHEET_NUMERIC_BOUNDS_DN, SHEET_NUMERIC_BOUNDS_VOLUME_M3, SHEET_NUMERIC_BOUNDS_AXIAL_FORCE_N, SHEET_NUMERIC_BOUNDS_16_PRESSURE_DROP_PA};

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
        rows: CONNECTION_TYPE_ROWS
            .iter()
            .map(|(code, en, de)| CatalogueRow {
                id: (*code).into(),
                cells: vec![CatalogueCell::text(*code), CatalogueCell::text(*en), CatalogueCell::text(*de)],
            })
            .collect(),
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
        rows: PRESSURE_CLASS_ROWS
            .iter()
            .map(|(code, bar)| CatalogueRow {
                id: (*code).into(),
                cells: vec![CatalogueCell::text(*code), CatalogueCell::number(*bar, 0)],
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
            CatalogueColumn { id: "de", label_en: "DE", label_de: "DE", unit: None },
        ],
        rows: FILTER_CLASS_ROWS
            .iter()
            .map(|(code, en, de)| CatalogueRow {
                id: (*code).into(),
                cells: vec![CatalogueCell::text(*code), CatalogueCell::text(*en), CatalogueCell::text(*de)],
            })
            .collect(),
    }
}

/// 📋 Heat-pump COP domain (VDI 3805-53 §4.2) — same numbers evaluate() reads.
fn heat_pump_cop_table() -> CatalogueTable {
    let (min_v, max_v) = SHEET_NUMERIC_BOUNDS_53_COP;
    CatalogueTable {
        id: "heat-pump-cop",
        title_en: "Heat-pump COP domain",
        title_de: "Wärmepumpen-COP-Bereich",
        clause: ClauseId::new("VDI 3805", "53", "4.2"),
        columns: vec![
            CatalogueColumn { id: "bound", label_en: "Bound", label_de: "Schranke", unit: None },
            CatalogueColumn { id: "value", label_en: "COP", label_de: "COP", unit: None },
        ],
        rows: vec![
            CatalogueRow { id: "min".into(), cells: vec![CatalogueCell::text("min"), CatalogueCell::number(min_v, 1)] },
            CatalogueRow { id: "max".into(), cells: vec![CatalogueCell::text("max"), CatalogueCell::number(max_v, 1)] },
        ],
    }
}


/// 📋 Airflow domain (VDI 3805-16/19) — same numbers evaluate() reads.
fn airflow_domain_table() -> CatalogueTable {
    let (min_v, max_v) = SHEET_NUMERIC_BOUNDS_19_AIRFLOW;
    CatalogueTable {
        id: "airflow-domain",
        title_en: "Airflow domain",
        title_de: "Volumenstrom-Bereich",
        clause: ClauseId::new("VDI 3805", "19", "4.2"),
        columns: vec![
            CatalogueColumn { id: "bound", label_en: "Bound", label_de: "Schranke", unit: None },
            CatalogueColumn { id: "value", label_en: "Airflow", label_de: "Volumenstrom", unit: Some("m³/s") },
        ],
        rows: vec![
            CatalogueRow { id: "min".into(), cells: vec![CatalogueCell::text("min"), CatalogueCell::number(min_v, 5)] },
            CatalogueRow { id: "max".into(), cells: vec![CatalogueCell::text("max"), CatalogueCell::number(max_v, 1)] },
        ],
    }
}

fn bound_table(id: &'static str, title_en: &'static str, title_de: &'static str, part: &'static str, clause: &'static str, unit: Option<&'static str>, bounds: (f64, f64), prec: u8) -> CatalogueTable {
    let (min_v, max_v) = bounds;
    CatalogueTable {
        id,
        title_en,
        title_de,
        clause: ClauseId::new("VDI 3805", part, clause),
        columns: vec![
            CatalogueColumn { id: "bound", label_en: "Bound", label_de: "Schranke", unit: None },
            CatalogueColumn { id: "value", label_en: "Value", label_de: "Wert", unit },
        ],
        rows: vec![
            CatalogueRow { id: "min".into(), cells: vec![CatalogueCell::text("min"), CatalogueCell::number(min_v, prec)] },
            CatalogueRow { id: "max".into(), cells: vec![CatalogueCell::text("max"), CatalogueCell::number(max_v, prec)] },
        ],
    }
}

/// 📚️ Normative reference tables shown beside examples.
pub fn catalogue_tables() -> Vec<CatalogueTable> {
    vec![
        connection_type_table(),
        pressure_class_table(),
        filter_class_table(),
        heat_pump_cop_table(),
        airflow_domain_table(),
        bound_table("dn-domain", "DN domain", "DN-Bereich", "1", "6.1", None, SHEET_NUMERIC_BOUNDS_DN, 0),
        bound_table("volume-domain", "Volume domain", "Volumen-Bereich", "7", "4.2", Some("m³"), SHEET_NUMERIC_BOUNDS_VOLUME_M3, 3),
        bound_table("axial-force-domain", "Axial force domain", "Axialkraft-Bereich", "60", "4.2", Some("N"), SHEET_NUMERIC_BOUNDS_AXIAL_FORCE_N, 0),
        bound_table("pressure-drop-domain", "Pressure drop domain", "Druckverlust-Bereich", "16", "4.2", Some("Pa"), SHEET_NUMERIC_BOUNDS_16_PRESSURE_DROP_PA, 0),
    ]
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
    crate::app_surface::render_catalogue(&examples, &reference_tables(), locale, controller_id, windows)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
