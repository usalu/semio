//! 🛍️ BIM library: the materials and the seven type families (wall, slab, roof, column, beam, window, door) of the project, one virtualised section per family, bound to the framework
//! `library` interaction domain so a pick shows the type in the properties panel. A family gets an add row as soon as its `create-*` mutation exists in the entity table.

use crate::editor::bim::entities::{EntityKind, ENTITIES};
use crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN;
use crate::editor::bim::kit::{bim_action, tree_item_with_icon, ui_capacity_error, ui_label, ui_text, ui_value_map, ui_value_text, BIM_EDITOR_CONTROLLER_ID};
use crate::editor::bim::terminology::BimLabels;
use crate::ModelSnapshot;
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

//#region 🔖️Constants
pub const BODY_KEY: &str = "bim.edit.library";
const ROOT: &str = "bim-library";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: BimLabels::localized(|labels| labels.panel_library),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
/// 🛍️ One row of a family's roster.
enum Row {
    Add(&'static EntityKind),
    Preset(&'static str),
    Entry(String),
}

fn roster(snapshot: &ModelSnapshot, family: &'static EntityKind) -> Vec<Row> {
    let mut rows: Vec<Row> = family.create.iter().map(|_| Row::Add(family)).collect();
    if family.kind == "schedule" {
        rows.extend(crate::editor::bim::modes::edit::windows::schedule::edit::presets().iter().copied().map(Row::Preset));
    }
    rows.extend((family.ids)(snapshot).into_iter().map(Row::Entry));
    rows
}

fn entry_row(snapshot: &ModelSnapshot, family: &EntityKind, id: &str) -> UiAssemblyResult<BuiltNode> {
    semio_framework_ui_contract::tree_item(ui_label(&(family.name)(snapshot, id).unwrap_or_else(|| id.to_string()))?)
        .try_id(id)
        .map_err(|_| ui_capacity_error())?
        .icon(ui_text(family.icon)?)
        .granularity(ui_text(family.kind)?)
        .default_open(false)
        .try_build()
        .map_err(|_| ui_capacity_error())
}

fn add_row(family: &EntityKind, labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("kind", ui_value_text(family.kind)?), ("parent", ui_value_text("")?), ("name", ui_value_text("")?)])?;
    tree_item_with_icon(format!("{ROOT}.add.{}", family.kind), Label::data(BimLabels::named(labels.action_add_named, (family.label)(labels).as_str())), "plus", bim_action("createEntity", Some(args)))
}
fn preset_row(key: &str, labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    let name = crate::editor::bim::modes::edit::windows::schedule::preset_label(labels, key);
    let args = ui_value_map([("kind", ui_value_text("schedule")?), ("parent", ui_value_text(key)?), ("name", ui_value_text(&name)?)])?;
    tree_item_with_icon(format!("{ROOT}.preset.{key}"), Label::data(BimLabels::named(labels.act_new_named, &name)), "plus", bim_action("createEntity", Some(args)))
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🛍️ Renders the library.
pub fn render(snapshot: &ModelSnapshot, labels: &BimLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = PanelTreeBuilder::new(ROOT)?;
    for family in ENTITIES.iter().filter(|row| row.library) {
        let rows = roster(snapshot, family);
        builder = builder.window_section(windows, &format!("{ROOT}.{}", family.kind), Some(ui_label((family.group)(labels).as_str())?), true, &rows, |row| match row {
            Row::Add(family) => add_row(family, labels),
            Row::Preset(key) => preset_row(key, labels),
            Row::Entry(id) => entry_row(snapshot, family, id),
        })?;
    }
    builder.interaction_domain(BIM_EDITOR_CONTROLLER_ID, BIM_LIBRARY_DOMAIN)?.build()
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
