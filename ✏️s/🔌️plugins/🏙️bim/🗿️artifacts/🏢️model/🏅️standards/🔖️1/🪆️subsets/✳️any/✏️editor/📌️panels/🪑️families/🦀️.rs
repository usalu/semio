//! 🪑️ BIM family browser: the families a component can be an instance of (no profile), by category, bound to the framework `library` interaction domain so a pick selects the family. A search box selects the families a text
//! matches (command `searchFamilies`); while a family is selected a details section names its category, parameters, solids, size and volume (read off the `families` inference, the same values the family preview draws)
//! and offers the rows that place it with the component tool or open it in the family editor. Everything is a labelled input or row, reachable from the keyboard.

use crate::editor::bim::entities::components::{placeable, wall_mounted};
use crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN;
use crate::editor::bim::kit::{bim_action, tree_item_with_icon, ui_capacity_error, ui_label, ui_text, ui_value_map, ui_value_text, BIM_EDITOR_CONTROLLER_ID};
use crate::editor::bim::modes::edit::windows::family::vocabulary::category_label;
use crate::editor::bim::panels::properties::input_row;
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::inferences::families::FamilyValue;
use crate::{FamilyCategory, ModelInference, ModelSnapshot};
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::ui_node_list;
use semio_framework_plugin::Buildable;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::HasBase;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::LabelText;

//#region 🔖️Constants
pub const BODY_KEY: &str = "bim.edit.families";
const ROOT: &str = "bim-families";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: BimLabels::localized(|labels| labels.panel_family_browser),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
/// 🪑️ One row of a category section.
enum Row<'a> {
    Search,
    Hits(usize),
    Entry(&'a str),
}

/// 📦️ The size of the visible solids of a family value as width (x), depth (y) and height (z) in metres; none when nothing is visible.
pub fn size_of(value: &FamilyValue) -> Option<[f64; 3]> {
    let bounds = value.visible().map(|(_, mesh)| mesh.bounds).reduce(|a, b| {
        let mut joined = a;
        joined.min.x = a.min.x.min(b.min.x);
        joined.min.y = a.min.y.min(b.min.y);
        joined.min.z = a.min.z.min(b.min.z);
        joined.max.x = a.max.x.max(b.max.x);
        joined.max.y = a.max.y.max(b.max.y);
        joined.max.z = a.max.z.max(b.max.z);
        joined
    })?;
    Some([bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z])
}

/// 🎯️ The placeable families the library selection holds, in the order of the browser.
pub fn selected(snapshot: &ModelSnapshot, library: &[String]) -> Vec<String> {
    placeable(snapshot).into_iter().map(|(id, _)| id).filter(|id| library.contains(id)).collect()
}

fn entry_row(snapshot: &ModelSnapshot, id: &str) -> UiAssemblyResult<BuiltNode> {
    let name = snapshot.families.get(id).map_or_else(|| id.to_string(), |family| family.name.clone());
    semio_framework_ui_contract::tree_item(ui_label(&name)?).try_id(id).map_err(|_| ui_capacity_error())?.icon(ui_text("shapes")?).granularity(ui_text("family")?).default_open(false).try_build().map_err(|_| ui_capacity_error())
}

fn action_row(kind: &str, family: &str, name: &str, label: LabelText, icon: &str, action: &str) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("family", ui_value_text(family)?)])?;
    tree_item_with_icon(format!("{ROOT}.{kind}.{family}"), Label::data(BimLabels::named(label, name)), icon, bim_action(action, Some(args)))
}

fn detail_rows(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let Some(family) = snapshot.families.get(id) else { return Vec::new() };
    let value = inference.families.get(id);
    let size = value.and_then(size_of).map(|[width, depth, height]| format!("{width:.3} × {depth:.3} × {height:.3} m"));
    let mut rows = vec![
        ui_label(labels.field_category.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.detail.category"), label, Some(category_label(labels, family.category)))),
        ui_label(labels.fam_section_parameters.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.detail.parameters"), label, Some(snapshot.family_parameters.values().filter(|row| row.family == id).count().to_string()))),
        ui_label(labels.fam_section_solids.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.detail.solids"), label, Some(snapshot.family_solids.values().filter(|row| row.family == id).count().to_string()))),
    ];
    if let Some(size) = size {
        rows.push(ui_label(labels.browser_size.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.detail.size"), label, Some(size))));
    }
    if let Some(value) = value {
        rows.push(ui_label(labels.field_volume.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.detail.volume"), label, Some(format!("{:.6}", value.volume())))));
        rows.push(ui_label(labels.fam_field_issues.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.detail.issues"), label, Some(value.issues.len().to_string()))));
    }
    rows.push(action_row("place", id, &family.name, labels.browser_place_named, "armchair", "placeComponent"));
    rows.push(action_row("open", id, &family.name, labels.browser_edit_named, "shapes", "openFamily"));
    rows
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🪑️ Renders the browser: the search box, the details of the selected family and one windowed section per category of placeable families; `library` is the selection of the library domain.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, library: &[String], windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = PanelTreeBuilder::new(ROOT)?;
    let list = placeable(snapshot);
    if list.is_empty() {
        builder = builder.section(format!("{ROOT}.empty"), Some(ui_label(labels.browser_none.as_str())?), true, ui_node_list(Vec::<UiAssemblyResult<BuiltNode>>::new())?)?;
        return builder.interaction_domain(BIM_EDITOR_CONTROLLER_ID, BIM_LIBRARY_DOMAIN)?.build();
    }
    let chosen = selected(snapshot, library);
    if let Some(first) = chosen.first() {
        builder = builder.section(format!("{ROOT}.detail"), Some(ui_label(labels.browser_selected.as_str())?), true, ui_node_list(detail_rows(snapshot, inference, first, labels))?)?;
    }
    let mut categories: Vec<FamilyCategory> = list.iter().map(|(_, category)| *category).collect();
    categories.dedup();
    for (position, category) in categories.into_iter().enumerate() {
        let mut rows: Vec<Row<'_>> = Vec::new();
        if position == 0 {
            rows.push(Row::Search);
            if !chosen.is_empty() {
                rows.push(Row::Hits(chosen.len()));
            }
        }
        rows.extend(list.iter().filter(|(_, known)| *known == category).map(|(id, _)| Row::Entry(id.as_str())));
        let title = format!("{}{}", category_label(labels, category), if wall_mounted(category) { " ⌂" } else { "" });
        builder = builder.window_section(windows, &format!("{ROOT}.{category:?}"), Some(ui_label(&title)?), true, &rows, |row| match row {
            Row::Search => input_row(&format!("{ROOT}.search"), labels.browser_search.as_str(), "", Some(labels.browser_search.as_str()), ui_value_map([("category", ui_value_text("")?)]).and_then(|args| bim_action("searchFamilies", Some(args)))),
            Row::Hits(count) => ui_label(labels.browser_hits.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.hits"), label, Some(count.to_string()))),
            Row::Entry(id) => entry_row(snapshot, id),
        })?;
    }
    builder.interaction_domain(BIM_EDITOR_CONTROLLER_ID, BIM_LIBRARY_DOMAIN)?.build()
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
