//! 📚️ Catalogue and inspection of the norm app surface: the catalogue table and the inspection render. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::spec_trace::{Realisation, SpecTraceTable};

//#region 🔖️Render
/// 📚️ One column in a normative reference [`CatalogueTable`].
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogueColumn {
    pub id: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub unit: Option<&'static str>,
}

/// 📚️ Typed cell value for a catalogue table row.
#[derive(Clone, Debug, PartialEq)]
pub enum CatalogueCell {
    Text(String),
    Localized { en: String, de: String },
    Number { value: f64, decimals: u8 },
    Empty,
}

impl CatalogueCell {
    /// 🏷️ Localized display string for a cell (numbers use fixed decimals; empty → em dash).
    pub fn display(&self, locale: Locale) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Localized { en, de } => chrome(en, de, locale),
            Self::Number { value, decimals } => format!("{value:.prec$}", prec = *decimals as usize),
            Self::Empty => chrome("—", "—", locale),
        }
    }

    /// 🔤 Text cell from a static label.
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    /// 🗣️ Text cell with one string per shell locale.
    pub fn localized(en: impl Into<String>, de: impl Into<String>) -> Self {
        Self::Localized { en: en.into(), de: de.into() }
    }

    /// 🔢 Number cell with fixed decimal places.
    pub fn number(value: f64, decimals: u8) -> Self {
        Self::Number { value, decimals }
    }
}

/// 📚️ One data row in a normative reference table.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogueRow {
    pub id: String,
    pub cells: Vec<CatalogueCell>,
}

/// 📚️ Schema-first normative reference table shown beside examples in the catalogue panel.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogueTable {
    pub id: &'static str,
    pub title_en: &'static str,
    pub title_de: &'static str,
    pub clause: ClauseId,
    pub columns: Vec<CatalogueColumn>,
    pub rows: Vec<CatalogueRow>,
}

/// 🧭️ The spec trace of an artifact as the uniform reference table of its catalogue panel: spec id, clause, entity, realisation kind, realisation and description.
pub fn spec_trace_table(trace: &SpecTraceTable) -> CatalogueTable {
    let kind = |realisation: &Realisation| match realisation {
        Realisation::Check(_) => CatalogueCell::localized("Check", "Prüfung"),
        Realisation::Data(_) => CatalogueCell::localized("Data", "Daten"),
        Realisation::DesignCase(_) => CatalogueCell::localized("Design case", "Bemessungsfall"),
        Realisation::NotModelled(_) => CatalogueCell::localized("Not modelled", "Nicht modelliert"),
    };
    CatalogueTable {
        id: "spec-trace",
        title_en: "Spec traceability (spec id, clause, realisation)",
        title_de: "Spezifikations-Rückverfolgung (Spezifikations-Id, Abschnitt, Realisierung)",
        clause: ClauseId::new(trace.standard, "catalogue", "traceability"),
        columns: vec![
            CatalogueColumn { id: "id", label_en: "Spec id", label_de: "Spezifikations-Id", unit: None },
            CatalogueColumn { id: "clause", label_en: "Clause", label_de: "Abschnitt", unit: None },
            CatalogueColumn { id: "entity", label_en: "Entity", label_de: "Entität", unit: None },
            CatalogueColumn { id: "kind", label_en: "Realisation", label_de: "Realisierung", unit: None },
            CatalogueColumn { id: "detail", label_en: "Check ids, data, case entry or reason", label_de: "Prüf-Ids, Daten, Fall-Eintrag oder Grund", unit: None },
            CatalogueColumn { id: "description", label_en: "Description", label_de: "Beschreibung", unit: None },
        ],
        rows: trace
            .items
            .iter()
            .map(|item| CatalogueRow {
                id: item.id.into(),
                cells: vec![
                    CatalogueCell::text(item.id),
                    CatalogueCell::localized(item.clause, semio_s_plugin_norm_compliance::designator(item.clause, &Locale::De)),
                    if item.entity.is_empty() { CatalogueCell::Empty } else { CatalogueCell::text(item.entity) },
                    kind(&item.realisation),
                    CatalogueCell::localized(item.realisation.describe(&Locale::En), item.realisation.describe(&Locale::De)),
                    CatalogueCell::localized(item.description.en, item.description.de),
                ],
            })
            .collect(),
    }
}

fn catalogue_column_header(column: &CatalogueColumn, locale: Locale) -> String {
    let label = chrome(column.label_en, column.label_de, locale);
    match column.unit {
        Some(unit) => format!("{label} [{unit}]"),
        None => label,
    }
}

fn catalogue_row_description(table: &CatalogueTable, row: &CatalogueRow, locale: Locale) -> String {
    table
        .columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            let cell = row.cells.get(index).unwrap_or(&CatalogueCell::Empty);
            format!("{}={}", catalogue_column_header(column, locale), cell.display(locale))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

fn render_catalogue_examples(examples: &[ExampleSource], locale: Locale, controller_id: &'static str) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut items = UiFixedList::default();
    if examples.is_empty() {
        items
            .try_push(tree_item_desc("norm-catalogue.empty", norm_ui_label(chrome("Examples", "Beispiele", locale))?, Some(chrome("No examples", "Keine Beispiele", locale)))?)
            .map_err(|_| ui_error("ui.fixed-capacity"))?;
    } else {
        for example in examples {
            let args = action_args_map(vec![("exampleId", ui_value_text(example.id())?)])?;
            items
                .try_push(tree_item_with_action(
                    format!("norm-catalogue.{}", example.id()),
                    norm_ui_label(example.label().resolve(Terminology::Native, locale))?,
                    None,
                    norm_action(controller_id, "setActiveExample", Some(args))?,
                )?)
                .map_err(|_| ui_error("ui.fixed-capacity"))?;
        }
    }
    Ok(items)
}

/// 📚️ Catalogue panel: example picker plus optional normative reference tables (windowed).
///
/// ```ignore
/// render_catalogue(&examples, &tables, locale, CONTROLLER_ID, &TreeWindows::for_body(view, BODY_CATALOGUE))
/// ```
pub fn render_catalogue(
    examples: &[ExampleSource],
    tables: &[CatalogueTable],
    locale: Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode> {
    let example_items = render_catalogue_examples(examples, locale, controller_id)?;
    let mut builder = PanelTreeBuilder::new("norm-catalogue")?.section(
        "norm-catalogue.examples",
        Some(norm_ui_label(chrome("Examples", "Beispiele", locale))?),
        true,
        example_items,
    )?;
    for table in tables {
        let section_id = format!("norm-catalogue.table-{}", table.id);
        let title = chrome(table.title_en, table.title_de, locale);
        let header = format!("{title} · {} · ({})", table.clause.text(&locale), table.rows.len());
        let column_band = table.columns.iter().map(|column| catalogue_column_header(column, locale)).collect::<Vec<_>>().join(" | ");
        let rows: Vec<&CatalogueRow> = table.rows.iter().collect();
        builder = builder.window_section_or_placeholder(
            windows,
            &section_id,
            Some(norm_ui_label(header)?),
            false,
            &rows,
            |row| {
                tree_item_desc(
                    format!("norm-catalogue.table-{}.{}", table.id, row.id),
                    norm_ui_label(catalogue_row_description(table, row, locale))?,
                    Some(column_band.clone()),
                )
            },
            norm_ui_label(chrome("Empty table", "Leere Tabelle", locale))?,
        )?;
    }
    builder.build()
}

/// 🔍️ Full check card for the inspection panel.
pub fn render_inspection(report: &CheckReport, selected_check_index: Option<u32>, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    let checks = &report.checks;
    let index = selected_check_index.map(|value| value as usize).filter(|index| *index < checks.len()).unwrap_or(0);
    let Some(check) = checks.get(index) else {
        let items = ui_node_list([tree_item_desc("norm-inspection.empty", norm_ui_label(chrome("Check", "Nachweis", locale))?, Some(chrome("No checks", "Keine Nachweise", locale)))])?;
        return PanelTreeBuilder::new("norm-inspection")?.section("norm-inspection.summary", Some(norm_ui_label(chrome("Inspection", "Inspektion", locale))?), true, items)?.build();
    };
    let protocol_locale = protocol_locale(locale);
    let spec = ChecksWindowSpec { controller_id, ..ChecksWindowSpec::default() };
    let mut rows = vec![
        tree_item_desc("norm-inspection.check.id", norm_ui_label(chrome("Id", "Kennung", locale))?, Some(check.id.clone()))?,
        tree_item_desc("norm-inspection.check.part", norm_ui_label(chrome("Part", "Teil", locale))?, Some(check.part.clone()))?,
        tree_item_desc("norm-inspection.check.subject", norm_ui_label(chrome("Subject", "Gegenstand", locale))?, Some(check.subject.label.resolve(&protocol_locale).to_string()))?,
    ];
    if !check.subject.path.is_empty() {
        rows.push(tree_item_desc("norm-inspection.check.subject-path", norm_ui_label(chrome("Subject path", "Gegenstandspfad", locale))?, Some(check.subject.path.clone()))?);
    }
    rows.push(tree_item_desc("norm-inspection.check.title", norm_ui_label(chrome("Title", "Titel", locale))?, Some(check.title.resolve(&protocol_locale).to_string()))?);
    rows.extend(check_detail_rows("norm-inspection.check", check, locale, &spec)?);
    rows.push(tree_item_desc("norm-inspection.check.annex", norm_ui_label(chrome("Annex", "Anhang", locale))?, Some(check.annex.label().into()))?);
    let items = ui_node_list(rows.into_iter().map(Ok))?;
    let tree = PanelTreeBuilder::new("norm-inspection")?
        .section("norm-inspection.check", Some(norm_ui_label(format!("{} {}", chrome("Check", "Nachweis", locale), index + 1))?), true, items)?
        .build()?;
    ui::column().try_children([utilisation_progress_node(check.status, check.utilization, locale)?, tree]).map_err(|_| ui_error("ui.node.children"))?.try_build().map_err(|_| ui_error("ui.node.build"))
}

/// ❓️ The unknown-body-key fallback every norm app's `render` ends with.
pub fn render_unknown_body(body_key: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    render_text(format!("{}: {body_key}", chrome("Unknown body", "Unbekannter Inhalt", locale)))
}
//#endregion 🔖️Render
