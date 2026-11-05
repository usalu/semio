//! 🪟️ Outliner window body: the entity collections of a document as rows with status glyph, a select action that points checks, properties, model and diagrams at the entity, and add/remove. See [`super`].

use super::*;

/// 🌲️ One outlined collection: document path and bilingual label.
#[derive(Clone, Copy, Debug)]
pub struct OutlinedCollection {
    pub path: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
}

/// 🎯️ The index of the governing check of an entity, the applicable check of the highest utilisation; the outliner select action and the context menu focus action both point at it.
pub fn governing_check_index(report: &CheckReport, entity_id: &str) -> Option<usize> {
    report.checks.iter().enumerate().filter(|(_, check)| check.status != CheckStatus::NotApplicable && !entity_id.is_empty() && check.subject.entity_id == entity_id).max_by(|a, b| a.1.utilization.total_cmp(&b.1.utilization)).map(|(index, _)| index)
}

fn worst_check_index(report: &CheckReport, row: &EntityRow<'_>) -> Option<usize> {
    governing_check_index(report, &row.id)
}

/// 🌲️ The outliner window: one windowed group per non-empty collection, the entity `entities` first, then every other top-level list of entities labelled from the field metadata; selecting a row dispatches `setSelectedCheckIndex` for the entity's governing check, which every other window follows.
pub fn render_outliner_document(tree: &DslValue, entities: &[OutlinedCollection], meta_fn: Option<NormFieldMetaFn>, context: &OutlinerContext<'_, '_>) -> UiAssemblyResult<BuiltNode> {
    let locale = context.locale;
    let mut listed: Vec<(String, String)> = entities.iter().map(|collection| (collection.path.to_string(), chrome(collection.label_en, collection.label_de, locale))).collect();
    if let DslValue::Object(entries) = tree {
        for (key, value) in entries {
            let supporting = matches!(value, DslValue::Array(items) if !items.is_empty() && is_entity_collection(items)) && !entities.iter().any(|collection| collection.path == key);
            if supporting {
                listed.push((key.clone(), field_label(key, resolve_field_meta(key, meta_fn), locale)));
            }
        }
    }
    outline(tree, listed, meta_fn, context)
}

fn outline(tree: &DslValue, collections: impl IntoIterator<Item = (String, String)>, meta_fn: Option<NormFieldMetaFn>, context: &OutlinerContext<'_, '_>) -> UiAssemblyResult<BuiltNode> {
    let locale = context.locale;
    let mut groups = Vec::new();
    for (path, label) in collections {
        let Some(DslValue::Array(items)) = tree.get(&path) else { continue };
        groups.push(render_collection(context, &path, &label, items, true, meta_fn, |row| {
            let action = match worst_check_index(context.report, row) {
                Some(index) => Some(norm_action(context.controller_id, "setSelectedCheckIndex", Some(action_args_map(vec![("index", UiValue::Number(index as f64))])?))?),
                None => None,
            };
            let id = format!("norm-outliner.{}.select", row.path);
            let label = norm_ui_label(chrome("Select", "Auswählen", locale))?;
            match action {
                Some(action) => tree_item_with_action(id, label, Some(row.title.clone()), action),
                None => tree_item_desc(id, label, Some(row.title.clone())),
            }
        })?);
    }
    let rows = ui_node_list(groups.into_iter().map(Ok))?;
    let tree = PanelTreeBuilder::new("norm-outliner-tree")?.section("norm-outliner-root", Some(norm_ui_label(chrome("Outliner", "Gliederung", locale))?), true, rows)?;
    match context.state.interaction_domain {
        Some(domain) => tree.interaction_domain(context.controller_id, domain.to_string())?.build(),
        None => tree.build(),
    }
}
