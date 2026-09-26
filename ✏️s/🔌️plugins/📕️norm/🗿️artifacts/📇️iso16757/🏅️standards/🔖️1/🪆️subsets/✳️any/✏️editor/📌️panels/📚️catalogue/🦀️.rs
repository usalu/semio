//! 📚️ ISO 16757 play app panel — examples plus Part 1/2/5 normative reference tables.

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable};
use crate::artifact_schema::{part_1, part_2, part_5};
use crate::document::ClauseId;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.iso16757.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 🔖️Tables
/// 📚️ Normative reference tables beside examples — same consts `evaluate()` reads.
pub fn reference_tables() -> Vec<CatalogueTable> {
    vec![
        edition_profile_table(),
        exchange_process_table(),
        lifecycle_status_table(),
        constraint_operator_table(),
        space_kind_table(),
        property_kind_table(),
        installation_clearance_table(),
    ]
}

fn code_list_columns() -> Vec<CatalogueColumn> {
    vec![
        CatalogueColumn { id: "code", label_en: "Code", label_de: "Code", unit: None },
        CatalogueColumn { id: "en", label_en: "English", label_de: "Englisch", unit: None },
        CatalogueColumn { id: "de", label_en: "German", label_de: "Deutsch", unit: None },
    ]
}

fn code_list_rows(rows: &[(&str, &str, &str)]) -> Vec<CatalogueRow> {
    rows.iter()
        .map(|(code, en, de)| CatalogueRow {
            id: (*code).into(),
            cells: vec![CatalogueCell::text(*code), CatalogueCell::text(*en), CatalogueCell::text(*de)],
        })
        .collect()
}

fn edition_profile_table() -> CatalogueTable {
    CatalogueTable {
        id: "iso16757-5-edition-profiles",
        title_en: "Edition profiles",
        title_de: "Editionsprofile",
        clause: ClauseId::new("ISO 16757", "5", "4.1"),
        columns: code_list_columns(),
        rows: code_list_rows(part_5::EDITION_PROFILE_ROWS),
    }
}

fn exchange_process_table() -> CatalogueTable {
    CatalogueTable {
        id: "iso16757-5-exchange-process",
        title_en: "Exchange process stages",
        title_de: "Austauschprozess-Stufen",
        clause: ClauseId::new("ISO 16757", "5", "6.1"),
        columns: code_list_columns(),
        rows: code_list_rows(part_5::EXCHANGE_PROCESS_ROWS),
    }
}

fn lifecycle_status_table() -> CatalogueTable {
    CatalogueTable {
        id: "iso16757-1-lifecycle-status",
        title_en: "Catalogue lifecycle statuses",
        title_de: "Katalog-Lebenszyklusstatus",
        clause: ClauseId::new("ISO 16757", "1", "5.1"),
        columns: code_list_columns(),
        rows: code_list_rows(part_1::LIFECYCLE_STATUS_ROWS),
    }
}

fn constraint_operator_table() -> CatalogueTable {
    CatalogueTable {
        id: "iso16757-1-constraint-operators",
        title_en: "Selection constraint operators",
        title_de: "Auswahl-Einschränkungsoperatoren",
        clause: ClauseId::new("ISO 16757", "1", "6.3"),
        columns: code_list_columns(),
        rows: code_list_rows(part_1::CONSTRAINT_OPERATOR_ROWS),
    }
}

fn space_kind_table() -> CatalogueTable {
    CatalogueTable {
        id: "iso16757-2-space-kinds",
        title_en: "Geometry space kinds",
        title_de: "Geometrie-Raumarten",
        clause: ClauseId::new("ISO 16757", "2", "5.3.5"),
        columns: code_list_columns(),
        rows: code_list_rows(part_2::SPACE_KIND_ROWS),
    }
}

fn property_kind_table() -> CatalogueTable {
    CatalogueTable {
        id: "iso16757-1-property-kinds",
        title_en: "Property definition kinds",
        title_de: "Eigenschaftsdefinitionsarten",
        clause: ClauseId::new("ISO 16757", "1", "5.3"),
        columns: code_list_columns(),
        rows: code_list_rows(part_1::PROPERTY_KIND_ROWS),
    }
}

fn installation_clearance_table() -> CatalogueTable {
    CatalogueTable {
        id: "iso16757-2-installation-clearance",
        title_en: "Installation clearance",
        title_de: "Einbau-Freiraum",
        clause: ClauseId::new("ISO 16757", "2", "7.1"),
        columns: vec![
            CatalogueColumn { id: "symbol", label_en: "Symbol", label_de: "Symbol", unit: None },
            CatalogueColumn { id: "value", label_en: "Clearance", label_de: "Freiraum", unit: Some("m") },
            CatalogueColumn { id: "en", label_en: "English", label_de: "Englisch", unit: None },
            CatalogueColumn { id: "de", label_en: "German", label_de: "Deutsch", unit: None },
        ],
        rows: vec![CatalogueRow {
            id: "clearance".into(),
            cells: vec![
                CatalogueCell::text("c_install"),
                CatalogueCell::number(part_2::INSTALL_CLEARANCE_M, 2),
                CatalogueCell::text("Minimum clearance around product solid"),
                CatalogueCell::text("Mindestfreiraum um den Produktkörper"),
            ],
        }],
    }
}
//#endregion 🔖️Tables

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
