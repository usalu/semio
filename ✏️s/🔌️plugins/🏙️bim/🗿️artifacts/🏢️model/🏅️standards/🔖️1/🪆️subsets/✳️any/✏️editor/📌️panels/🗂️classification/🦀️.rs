//! 🗂️ BIM classification browser: every classification system of the library as a tree of its entry table (an entry indented by its depth), a search box per system and one action row per entry that assigns its code to the
//! selected elements or types. A search selects the matching entries in the framework `library` domain (command `searchClassification`); while some entries of a system are selected the tree shows them with their ancestors
//! instead of the whole table, and an empty search shows the table again. A check marks an entry every selected holder carries. Everything is reachable from the keyboard: labelled inputs and rows, not gestures.

use crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN;
use crate::editor::bim::kit::{bim_action, tree_item_with_icon, ui_label, ui_value_list, ui_value_map, ui_value_text, BIM_EDITOR_CONTROLLER_ID};
use crate::editor::bim::panels::properties::input_row;
use crate::editor::bim::terminology::BimLabels;
use crate::{ClassificationSystem, ModelSnapshot};
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::BTreeSet;

//#region 🔖️Constants
pub const BODY_KEY: &str = "bim.edit.classification";
const ROOT: &str = "bim-classification";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: BimLabels::localized(|labels| labels.panel_classification),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
/// 🗂️ One row of the section of a system.
pub enum Row<'a> {
    Search,
    Hits(usize),
    Entry { depth: usize, code: &'a str, title: &'a str, assigned: bool },
}

/// 🔍️ The codes of the table rows a search selected in the `library` domain, empty when nothing of the system is selected.
pub fn found<'a>(system: &str, library: &'a [String]) -> BTreeSet<&'a str> {
    let prefix = format!("{system}:");
    library.iter().filter_map(|id| id.strip_prefix(prefix.as_str())).collect()
}

/// 🌳️ The rows of the table that are shown: the whole table in tree order, or the matches of a search with their ancestors; each row says whether every holder in `holders` carries it.
pub fn entries<'a>(id: &str, system: &'a ClassificationSystem, matches: &BTreeSet<&str>, holders: &[&String], snapshot: &ModelSnapshot) -> Vec<Row<'a>> {
    let shown: BTreeSet<&str> = if matches.is_empty() { system.entries.iter().map(|entry| entry.code.as_str()).collect() } else { matches.iter().flat_map(|code| system.lineage(code)).map(|entry| entry.code.as_str()).collect() };
    let carries = |code: &str| !holders.is_empty() && holders.iter().all(|holder| snapshot.classifications.get(holder.as_str()).and_then(|set| set.get(id)).is_some_and(|assigned| assigned == code));
    system.tree().into_iter().filter(|(_, entry)| shown.contains(entry.code.as_str())).map(|(depth, entry)| Row::Entry { depth, code: &entry.code, title: &entry.title, assigned: carries(&entry.code) }).collect()
}

fn entry_row(system: &str, depth: usize, code: &str, title: &str, assigned: bool, labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("system", ui_value_text(system)?), ("code", ui_value_text(code)?), ("ids", ui_value_list(Vec::new())?)])?;
    let text = format!("{}{code}  {title}", "· ".repeat(depth));
    let name = BimLabels::named(labels.classification_assign, &text);
    tree_item_with_icon(format!("{ROOT}.{system}.entry.{code}"), Label::data(name), if assigned { "check" } else { "tag" }, bim_action("setClassification", Some(args)))
}

fn search_row(system: &str, labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    input_row(&format!("{ROOT}.{system}.search"), labels.classification_search.as_str(), "", Some(labels.classification_search.as_str()), ui_value_map([("system", ui_value_text(system)?)]).and_then(|args| bim_action("searchClassification", Some(args))))
}

fn hits_row(system: &str, count: usize, labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    tree_item_desc(format!("{ROOT}.{system}.hits"), ui_label(labels.classification_hits.as_str())?, Some(count.to_string()))
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🗂️ Renders the classification browser: one windowed section per system, `elements` being the selected elements (the holders an assignment reaches) and `library` the selection of the library domain (the matches of a search).
pub fn render(snapshot: &ModelSnapshot, labels: &BimLabels, elements: &[String], library: &[String], windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = PanelTreeBuilder::new(ROOT)?;
    let holders: Vec<&String> = elements.iter().filter(|id| crate::mutations::elements::holds_data(snapshot, id)).collect();
    for (id, system) in &snapshot.classification_systems {
        let matches = found(id, library);
        let mut rows: Vec<Row<'_>> = vec![Row::Search];
        if !matches.is_empty() {
            rows.push(Row::Hits(matches.len()));
        }
        rows.extend(entries(id, system, &matches, &holders, snapshot));
        let title = format!("{} {}", system.name, system.edition).trim().to_string();
        builder = builder.window_section(windows, &format!("{ROOT}.{id}"), Some(ui_label(&title)?), true, &rows, |row| match row {
            Row::Search => search_row(id, labels),
            Row::Hits(count) => hits_row(id, *count, labels),
            Row::Entry { depth, code, title, assigned } => entry_row(id, *depth, code, title, *assigned, labels),
        })?;
    }
    if snapshot.classification_systems.is_empty() {
        builder = builder.section(format!("{ROOT}.empty"), Some(ui_label(labels.classification_none.as_str())?), true, semio_framework_plugin::ui_node_list(Vec::<UiAssemblyResult<BuiltNode>>::new())?)?;
    }
    builder.interaction_domain(BIM_EDITOR_CONTROLLER_ID, BIM_LIBRARY_DOMAIN)?.build()
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
