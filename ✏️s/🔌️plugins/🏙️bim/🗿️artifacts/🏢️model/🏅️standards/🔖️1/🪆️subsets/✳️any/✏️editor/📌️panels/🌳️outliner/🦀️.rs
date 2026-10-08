//! 🌳️ BIM outliner: the model as one virtualised tree, bound to the framework `elements` interaction domain. Site, building and storey (by level) nest as the snapshot's references
//! say, each storey holds one group per element kind, and openings sit under the wall that hosts them. Every container is a windowed node that stamps its full extent and builds only the
//! slice the host scrolled to, so a model of thousands of elements costs the first paint one slice. Rows are the element ids, so a click selects the same element the plan and the world do.

use crate::editor::bim::entities::{kind_of, ordered_storeys, EntityKind, ENTITIES};
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{bim_action, tree_item_with_icon, ui_label, ui_text, ui_value_map, ui_value_text, BIM_EDITOR_CONTROLLER_ID};
use crate::editor::bim::terminology::BimLabels;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::Buildable;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::HasBase;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const BODY_KEY: &str = "bim.edit.outliner";
const ROOT: &str = "bim-outliner";
const SECTION: &str = "bim-outliner.model";
/// ➕️ The kinds the panel offers an add row for when their create mutation exists, in tree order.
/// 📂️ A group of at most this many elements opens on first paint; a larger one waits for the host to open it and then streams its slice.
const OPEN_GROUP_LIMIT: usize = 12;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.panel_outliner.as_str(), BimLabels::NATIVE_DE.panel_outliner.as_str()),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
/// 🌳️ One row of the outliner's single logical roster.
enum Row<'a> {
    Add(&'static EntityKind),
    Empty,
    Site(&'a str),
}

fn roster<'a>(snapshot: &'a ModelSnapshot) -> Vec<Row<'a>> {
    let mut rows: Vec<Row<'a>> = snapshot.sites.keys().map(|id| Row::Site(id.as_str())).collect();
    if snapshot.sites.is_empty() && snapshot.buildings.is_empty() {
        rows.push(Row::Empty);
    }
    rows.extend(ENTITIES.iter().filter(|row| !row.library && row.create.is_some()).map(Row::Add));
    rows
}

fn item(row_id: &str, name: &str, icon: &str, granularity: Option<&str>, description: Option<&str>) -> UiAssemblyResult<semio_framework_ui_contract::TreeItemBuilder> {
    let mut item = semio_framework_ui_contract::tree_item(ui_label(name)?).try_id(row_id).map_err(|_| crate::editor::bim::kit::ui_capacity_error())?.icon(ui_text(icon)?);
    if let Some(granularity) = granularity {
        item = item.granularity(ui_text(granularity)?);
    }
    if let Some(description) = description {
        item = item.description(ui_text(description)?);
    }
    Ok(item)
}

fn leaf(builder: semio_framework_ui_contract::TreeItemBuilder) -> UiAssemblyResult<BuiltNode> {
    builder.default_open(false).try_build().map_err(|_| crate::editor::bim::kit::ui_capacity_error())
}

fn entity_item(snapshot: &ModelSnapshot, row: &EntityKind, id: &str, description: Option<&str>) -> UiAssemblyResult<semio_framework_ui_contract::TreeItemBuilder> {
    item(id, &(row.name)(snapshot, id).unwrap_or_else(|| id.to_string()), row.icon, Some(row.kind), description)
}

fn element_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, inference: &ModelInference, row: &'static EntityKind, id: &str) -> UiAssemblyResult<BuiltNode> {
    let length = inference.wall_layout.get(id).map(|layout| format!("{:.2} m", layout.length));
    let builder = entity_item(snapshot, row, id, length.as_deref())?;
    let hosted: Vec<&String> = snapshot.openings.iter().filter(|(_, opening)| opening.host == id).map(|(opening, _)| opening).collect();
    let opening = kind_of("opening").unwrap_or(row);
    if hosted.is_empty() {
        return leaf(builder);
    }
    semio_framework_plugin::tree_window_indexed_item(windows, builder, id, false, hosted.len(), |index| leaf(entity_item(snapshot, opening, hosted[index], None)?))
}

fn group_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, storey: &str, row: &'static EntityKind, ids: &[String]) -> UiAssemblyResult<BuiltNode> {
    let group_id = format!("{storey}::{}", row.kind);
    let count = ids.len().to_string();
    let builder = item(&group_id, (row.group)(labels).as_str(), row.icon, None, Some(&count))?;
    semio_framework_plugin::tree_window_indexed_item(windows, builder, &group_id, ids.len() <= OPEN_GROUP_LIMIT, ids.len(), |index| element_row(windows, snapshot, inference, row, &ids[index]))
}

fn storey_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, id: &str) -> UiAssemblyResult<BuiltNode> {
    let storey = kind_of("storey").unwrap_or(&ENTITIES[0]);
    let elevation = inference.storey_levels.get(id).map(|level| format!("{:+.2} m", level.elevation));
    let builder = entity_item(snapshot, storey, id, elevation.as_deref())?;
    let groups: Vec<(&'static EntityKind, Vec<String>)> = ENTITIES
        .iter()
        .filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid" | "opening"))
        .map(|row| (row, (row.ids)(snapshot).into_iter().filter(|candidate| (row.parent)(snapshot, candidate).as_deref() == Some(id)).collect::<Vec<_>>()))
        .filter(|(_, ids)| !ids.is_empty())
        .collect();
    if groups.is_empty() {
        return leaf(builder);
    }
    semio_framework_plugin::tree_window_indexed_item(windows, builder, id, true, groups.len(), |index| group_row(windows, snapshot, inference, labels, id, groups[index].0, &groups[index].1))
}

fn building_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, id: &str) -> UiAssemblyResult<BuiltNode> {
    let building = kind_of("building").unwrap_or(&ENTITIES[0]);
    let grid = kind_of("grid").unwrap_or(building);
    let builder = entity_item(snapshot, building, id, None)?;
    let storeys = ordered_storeys(snapshot, id);
    let grids: Vec<&String> = snapshot.grids.iter().filter(|(_, line)| line.building == id).map(|(grid_id, _)| grid_id).collect();
    let total = storeys.len() + grids.len();
    if total == 0 {
        return leaf(builder);
    }
    semio_framework_plugin::tree_window_indexed_item(windows, builder, id, true, total, |index| match storeys.get(index) {
        Some(storey) => storey_row(windows, snapshot, inference, labels, storey),
        None => leaf(entity_item(snapshot, grid, grids[index - storeys.len()], None)?),
    })
}

fn site_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, id: &str) -> UiAssemblyResult<BuiltNode> {
    let site = kind_of("site").unwrap_or(&ENTITIES[0]);
    let builder = entity_item(snapshot, site, id, None)?;
    let buildings: Vec<&String> = snapshot.buildings.iter().filter(|(_, building)| building.site == id).map(|(building, _)| building).collect();
    if buildings.is_empty() {
        return leaf(builder);
    }
    semio_framework_plugin::tree_window_indexed_item(windows, builder, id, true, buildings.len(), |index| building_row(windows, snapshot, inference, labels, buildings[index]))
}

fn add_row(row: &EntityKind, labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("kind", ui_value_text(row.kind)?), ("parent", ui_value_text("")?), ("name", ui_value_text("")?)])?;
    tree_item_with_icon(format!("{ROOT}.add.{}", row.kind), Label::data(BimLabels::named(labels.action_add_named, (row.label)(labels).as_str())), "plus", bim_action("createEntity", Some(args)))
}

fn empty_row(labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    leaf(item(&format!("{ROOT}.empty"), labels.empty_outliner.as_str(), "info", None, None)?)
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🌳️ Renders the outliner.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let rows = roster(snapshot);
    let title = if snapshot.project.name.is_empty() { labels.panel_outliner.as_str() } else { snapshot.project.name.as_str() };
    PanelTreeBuilder::new(ROOT)?
        .window_section(windows, SECTION, Some(ui_label(title)?), true, &rows, |row| match row {
            Row::Add(kind) => add_row(kind, labels),
            Row::Empty => empty_row(labels),
            Row::Site(id) => site_row(windows, snapshot, inference, labels, id),
        })?
        .interaction_domain(BIM_EDITOR_CONTROLLER_ID, BIM_ELEMENT_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
