//! 🗃️ Bulk inspector: the properties window of a multi-selection. Every field any selected row has is one line with its typed control; a field the rows agree on shows the common value, a field they disagree on shows "Mixed" and an empty control. A control raises one `setFields` bundle that writes the field of every selected row (one undo step). See [`super`].
//!
//! 🔗️ Rows: [`bulk_rows`]; the write: [`bulk_writes_arg`]; see `visual-editor-architecture.md` §6.

use super::*;

fn scalar(value: &DslValue) -> bool {
    match value {
        DslValue::Bool(_) | DslValue::Number(_) | DslValue::String(_) => true,
        DslValue::Null | DslValue::Bytes(_) | DslValue::Array(_) | DslValue::Object(_) => false,
    }
}

/// 🗃️ The inspector of the rows `ids` of `collection`: a header with the count, then one line per scalar field the selected rows have. `tree` is the document value tree; `system` the unit system of the unit picker.
pub fn render_bulk_inspector(tree: &DslValue, collection: &str, ids: &[String], locale: Locale, controller_id: &'static str, meta_fn: Option<NormFieldMetaFn>, system: UnitSystem) -> UiAssemblyResult<BuiltNode> {
    let german = matches!(locale, Locale::De);
    let Some(rows) = get_value_at_path(tree, collection).ok().and_then(DslValue::as_array) else { return Err(ui_error("ui.bulk.collection")) };
    let first = rows.iter().find(|row| object_id_field(row).is_some_and(|id| ids.iter().any(|wanted| wanted == id)));
    let mut lines = UiFixedList::default();
    let header = format!("{} {}", ids.len(), chrome("selected", "ausgewählt", locale));
    lines.try_push(tree_item_desc(format!("norm-bulk.{collection}.count"), norm_ui_label(header)?, Some(chrome("Changes apply to every selected row as one undoable edit.", "Änderungen gelten für jede ausgewählte Zeile als eine rückgängig machbare Änderung.", locale)))?).map_err(|_| ui_error("ui.fixed-capacity"))?;
    for line in bulk_rows(tree, collection, ids, german) {
        let Some(sample) = first.and_then(|row| row.get(&line.field)).filter(|value| scalar(value)) else { continue };
        let path = format!("{collection}[id={}].{}", first.and_then(object_id_field).unwrap_or_default(), line.field);
        let row_id = format!("norm-bulk.{collection}.{}", line.field);
        let current = if line.mixed { DslValue::Null } else { sample.clone() };
        let mut view = field_view(&path, sample, tree, locale, meta_fn, system);
        view.read_only |= ids.iter().any(|id| resolve_field_meta(&format!("{collection}[id={id}].{}", line.field), meta_fn).is_none_or(|spec| spec.read_only));
        view.bulk = Some(BulkTarget { collection: collection.to_string(), ids: ids.to_vec(), field: line.field.clone() });
        let mut item = ui::tree_item(norm_ui_label(view.labelled())?).try_id(&row_id).map_err(|_| ui_error("ui.node.id"))?;
        item = item.description(ui_text(if line.mixed { mixed_label(german).to_string() } else { line.text.clone() })?);
        for control in field_controls(&view, &current, &row_id, controller_id, locale)? {
            item = item.try_child(control).map_err(|_| ui_error("ui.node.child"))?;
        }
        lines.try_push(item.try_build().map_err(|_| ui_error("ui.node.build"))?).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    tree_group(format!("norm-bulk.{collection}"), norm_ui_label(chrome("Selection", "Auswahl", locale))?, true, lines)
}

/// 🗃️ The inspector for the selection the host published: the bulk inspector when more than one row of one collection is selected, `None` otherwise (the single-row inspector stays).
pub fn render_selection_inspector(tree: &DslValue, selected: &[String], locale: Locale, controller_id: &'static str, meta_fn: Option<NormFieldMetaFn>, system: UnitSystem) -> UiAssemblyResult<Option<BuiltNode>> {
    match selected_rows(tree, selected) {
        Some((collection, ids)) if ids.len() > 1 => render_bulk_inspector(tree, &collection, &ids, locale, controller_id, meta_fn, system).map(Some),
        Some(_) | None => Ok(None),
    }
}
