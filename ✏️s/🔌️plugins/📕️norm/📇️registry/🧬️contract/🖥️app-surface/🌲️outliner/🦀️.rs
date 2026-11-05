//! 🌲️ Entity outliner of the norm app surface: collections as groups, entities as rows with status glyph, selection, filter and add/remove. See [`super`].

include!("../📋️prelude/🦀️.rs");

use semio_s_plugin_norm_app_surface_foundation::*;
use semio_s_plugin_norm_app_surface_live_core::*;
pub use semio_s_plugin_norm_app_surface_interaction_topology::*;
use semio_framework_value::DslValue;

//#region 🌲️State
/// 🌲️ What the host feeds the outliner: selected entity ids or paths, the filter text, when the owning window can persist it the action the filter input commits to, the interaction domain whose selection a row activation writes (the viewport highlights the same entity) and the collections whose rows can be reordered (the family declares a `MoveItemRule` for them).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutlinerState {
    pub selected: Vec<String>,
    pub filter: String,
    pub filter_action: Option<&'static str>,
    pub interaction_domain: Option<&'static str>,
    pub reorderable: Vec<&'static str>,
}

impl OutlinerState {
    /// ↕️ The top-level collections the family can reorder: the lists of its `MoveItemRule`s.
    pub fn with_reorderable(mut self, rules: &NormEditRules) -> Self {
        self.reorderable = rules.move_item.iter().map(|rule| rule.path).filter(|path| !path.contains(|c: char| c == '.' || c == '[')).collect();
        self
    }
}

/// 🫳️ The MIME type of the payload a dragged outliner row carries; its JSON object (`{"id": "<row id>"}`) merges into the arguments of the drop action of the row it is dropped on.
pub const ROW_DRAG_MIME: &str = "application/x-semio-norm-row";

/// 🌲️ Everything one outliner render reads.
pub struct OutlinerContext<'a, 'w> {
    pub report: &'a CheckReport,
    pub locale: Locale,
    pub controller_id: &'static str,
    pub windows: &'a TreeWindows<'w>,
    pub state: &'a OutlinerState,
}

//#endregion 🌲️State

//#region 🌲️Model
fn is_selected(state: &OutlinerState, row: &EntityRow<'_>) -> bool {
    state.selected.iter().any(|selected| *selected == row.path || (!row.id.is_empty() && *selected == row.id))
}
//#endregion 🌲️Model

//#region 🌲️Render
fn row_label(context: &OutlinerContext<'_, '_>, row: &EntityRow<'_>) -> String {
    match entity_feedback(context.report, &row.id, &row.path) {
        Some(feedback) => format!("{} · {}", row.title, utilization_text(feedback.status, feedback.utilization)),
        None => row.title.clone(),
    }
}

fn drag_payload(id: &str) -> UiAssemblyResult<ui::UiFixedMap<ui::UiText>> {
    let json = semio_framework_pack_json::to_json_string(&DslValue::object(vec![("id".to_string(), DslValue::String(id.to_string()))]));
    let mut payload = ui::UiFixedMap::<ui::UiText>::default();
    payload.try_push(ui_text(ROW_DRAG_MIME)?, ui_text(json)?).map_err(|_| ui_error("ui.drag-data"))?;
    Ok(payload)
}

/// ↕️ The reorder affordances of one row: the drag payload and the drop binding (the row dropped on is the target) and, as the keyboard, touch and screen-reader alternative, a move-up and a move-down item; the first row has no up, the last no down.
fn reorder_items(context: &OutlinerContext<'_, '_>, row: &EntityRow<'_>, collection: &str, count: usize) -> UiAssemblyResult<Vec<BuiltNode>> {
    let locale = context.locale;
    let mut items = Vec::new();
    for (step, glyph, en, de) in [(-1.0, "↑", "Move up", "Nach oben"), (1.0, "↓", "Move down", "Nach unten")] {
        if (step < 0.0 && row.index == 0) || (step > 0.0 && row.index + 1 >= count) {
            continue;
        }
        let args = action_args_map(vec![("path", ui_value_text(collection)?), ("id", ui_value_text(&row.id)?), ("step", UiValue::Number(step))])?;
        items.push(tree_item_with_action(format!("norm-outliner.{}.move{}", row.path, if step < 0.0 { "-up" } else { "-down" }), norm_ui_label(format!("{glyph} {} — {}", chrome(en, de, locale), row.title))?, None, norm_action(context.controller_id, "moveEntity", Some(args))?)?);
    }
    Ok(items)
}

fn entity_row(context: &OutlinerContext<'_, '_>, row: &EntityRow<'_>, count: usize, meta_fn: Option<NormFieldMetaFn>, body: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let selected = is_selected(context.state, row);
    let collection = row.path.split('[').next().unwrap_or_default();
    let reorder = !row.id.is_empty() && context.state.reorderable.contains(&collection);
    let locale = context.locale;
    let address = context.state.interaction_domain.and_then(|_| row_address(row));
    let item = ui::tree_item(norm_ui_label(row_label(context, row))?).try_id(address.clone().unwrap_or_else(|| format!("norm-outliner.{}", row.path))).map_err(|_| ui_error("ui.node.id"))?.default_open(selected).selected(selected);
    let item = if address.is_some() { item.granularity(ui_text("entity")?) } else { item };
    let item = if reorder { bind_on(item.draggable(true).drag_data(drag_payload(&row.id)?), Trigger::Drop, context.controller_id, "moveEntity", action_args_map(vec![("path", ui_value_text(collection)?), ("over", ui_value_text(&row.id)?)])?)? } else { item };
    let mut item = item.try_child(body).map_err(|_| ui_error("ui.node.child"))?;
    if meta_fn.and_then(|lookup| lookup(&row.path)).is_some_and(|meta| meta.can_remove) {
    let remove_args = action_args_map(vec![("path", ui_value_text(&row.path)?), ("index", UiValue::Number(0.0))])?;
    let remove = tree_item_with_action(format!("norm-outliner.{}.remove", row.path), norm_ui_label(format!("{} — {}", chrome("Remove", "Entfernen", locale), row.title))?, None, norm_action(context.controller_id, "removeItem", Some(remove_args))?)?;
        item = item.try_child(remove).map_err(|_| ui_error("ui.node.child"))?;
    }
    if reorder {
        for moved in reorder_items(context, row, collection, count)? {
            item = item.try_child(moved).map_err(|_| ui_error("ui.node.child"))?;
        }
    }
    item.try_build().map_err(|_| ui_error("ui.node.build"))
}

fn filter_input(context: &OutlinerContext<'_, '_>, path: &str) -> UiAssemblyResult<Option<BuiltNode>> {
    let Some(action) = context.state.filter_action else { return Ok(None) };
    let input = ui::input(InputKind::Text).value(ui_text(&context.state.filter)?).commit(ui_text("blur")?).try_id(format!("norm-outliner.{path}.filter")).map_err(|_| ui_error("ui.node.id"))?;
    let input = bind_on(input, Trigger::Commit, context.controller_id, action, action_args_map(vec![("path", ui_value_text(path)?)])?)?;
    let input = input.try_label(chrome("Filter entities", "Entitäten filtern", context.locale)).map_err(|_| ui_error("ui.node.label"))?;
    Ok(Some(input.try_build().map_err(|_| ui_error("ui.node.build"))?))
}

/// 🌲️ A collection as a windowed group of entity rows plus an add action; `body` draws one entity's contents (its inspector).
pub fn render_collection(context: &OutlinerContext<'_, '_>, path: &str, label: &str, items: &[DslValue], default_open: bool, meta_fn: Option<NormFieldMetaFn>, mut body: impl FnMut(&EntityRow<'_>) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let locale = context.locale;
    let rows = collection_rows(path, items, &context.state.filter);
    let section_id = inputs_section_id(path);
    let count = if rows.len() == items.len() { format!("{}", items.len()) } else { format!("{}/{}", rows.len(), items.len()) };
    let list = tree_window_section_or_placeholder(
        context.windows,
        &section_id,
        norm_ui_label(format!("{label} ({count})"))?,
        default_open,
        &rows,
        |row| {
            let inner = body(row)?;
            entity_row(context, row, items.len(), meta_fn, inner)
        },
        norm_ui_label(chrome("Empty list", "Leere Liste", locale))?,
    )?;
    if !context.windows.is_open(&section_id, default_open) {
        return Ok(list);
    }
    let can_insert = meta_fn.and_then(|lookup| lookup(path)).is_some_and(|meta| meta.can_insert);
    let mut children = UiFixedList::default();
    if let Some(filter) = filter_input(context, path)? {
        children.try_push(filter).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    children.try_push(list).map_err(|_| ui_error("ui.fixed-capacity"))?;
    if can_insert {
    let insert_args = action_args_map(vec![("path", ui_value_text(path)?), ("index", UiValue::Number(items.len() as f64))])?;
    let add = tree_item_with_action(format!("norm-outliner.{path}.add"), norm_ui_label(chrome("Add item", "Eintrag hinzufügen", locale))?, None, norm_action(context.controller_id, "insertItem", Some(insert_args))?)?;
        children.try_push(add).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    tree_group(format!("norm-outliner.{path}.group"), norm_ui_label(label)?, true, children)
}
//#endregion 🌲️Render

#[path = "🪟️window/🦀️.rs"]
mod window;
pub use window::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
