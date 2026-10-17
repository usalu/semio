//! 📝️ Document editors of the norm app surface: the object, array and value editors and the summary. See [`super`].

use super::*;

//#region 🔖️Render
/// 📜 Arrays and object maps at or above this length rely on host windowing (same path for both).
pub const NORM_LIST_VIRTUALIZE_THRESHOLD: usize = 64;

/// 🪪 Stable `TreeWindows` node key for a document path (`root` when empty).
pub fn inputs_section_id(path: &str) -> String {
    if path.is_empty() {
        "norm-inputs-root".into()
    } else {
        format!("norm-inputs-{path}")
    }
}

fn collection_header(label: &str, count: usize) -> String {
    format!("{label} ({count})")
}

fn render_object_editor(
    path: &str,
    entries: &[(String, semio_framework_value::DslValue)],
    locale: Locale,
    controller_id: &'static str,
    meta_fn: Option<NormFieldMetaFn>,
    windows: &TreeWindows<'_>,
    depth: usize,
    label: String,
) -> UiAssemblyResult<BuiltNode> {
    let section_id = inputs_section_id(path);
    let default_open = path.is_empty();
    let section_label = if path.is_empty() { chrome("Document", "Dokument", locale) } else { label };
    let pairs: Vec<(&String, &semio_framework_value::DslValue)> = entries.iter().map(|(key, child)| (key, child)).collect();
    tree_window_section_or_placeholder(
        windows,
        &section_id,
        norm_ui_label(collection_header(&section_label, pairs.len()))?,
        default_open,
        &pairs,
        |(key, child)| {
            let child_path = if path.is_empty() { (*key).clone() } else { format!("{path}.{}", *key) };
            render_value_editor(&child_path, child, locale, controller_id, meta_fn, windows, depth + 1)
        },
        norm_ui_label(chrome("Empty object", "Leeres Objekt", locale))?,
    )
}

fn render_array_editor(
    path: &str,
    items: &[semio_framework_value::DslValue],
    locale: Locale,
    controller_id: &'static str,
    meta_fn: Option<NormFieldMetaFn>,
    windows: &TreeWindows<'_>,
    depth: usize,
    label: String,
) -> UiAssemblyResult<BuiltNode> {
    let section_id = inputs_section_id(path);
    let default_open = false;
    let indexed: Vec<(usize, &semio_framework_value::DslValue)> = items.iter().enumerate().collect();
    let list = tree_window_section_or_placeholder(
        windows,
        &section_id,
        norm_ui_label(collection_header(&label, indexed.len()))?,
        default_open,
        &indexed,
        |(index, child)| {
            let child_path = list_element_path(path, child, *index);
            let mut row_children = UiFixedList::default();
            row_children
                .try_push(render_value_editor(&child_path, child, locale, controller_id, meta_fn, windows, depth + 1)?)
                .map_err(|_| ui_error("ui.fixed-capacity"))?;
            let remove_args = action_args_map(vec![("path", ui_value_text(&child_path)?), ("index", UiValue::Number(0.0))])?;
            row_children
                .try_push(tree_item_with_action(
                    format!("norm-inputs.{path}.remove-{index}"),
                    norm_ui_label(chrome("Remove", "Entfernen", locale))?,
                    None,
                    norm_action(controller_id, "removeItem", Some(remove_args))?,
                )?)
                .map_err(|_| ui_error("ui.fixed-capacity"))?;
            tree_group(
                format!("norm-inputs.{path}.row-{index}"),
                norm_ui_label(list_element_selector(child, *index))?,
                false,
                row_children,
            )
        },
        norm_ui_label(chrome("Empty list", "Leere Liste", locale))?,
    )?;
    if !windows.is_open(&section_id, default_open) {
        return Ok(list);
    }
    let insert_args = action_args_map(vec![("path", ui_value_text(path)?), ("index", UiValue::Number(items.len() as f64))])?;
    let add = tree_item_with_action(
        format!("norm-inputs.{path}.add"),
        norm_ui_label(chrome("Add item", "Eintrag hinzufügen", locale))?,
        None,
        norm_action(controller_id, "insertItem", Some(insert_args))?,
    )?;
    let mut children = UiFixedList::default();
    children.try_push(list).map_err(|_| ui_error("ui.fixed-capacity"))?;
    children.try_push(add).map_err(|_| ui_error("ui.fixed-capacity"))?;
    tree_group(format!("norm-inputs.{path}.editor"), norm_ui_label(label)?, true, children)
}

pub(super) fn render_value_editor(
    path: &str,
    value: &semio_framework_value::DslValue,
    locale: Locale,
    controller_id: &'static str,
    meta_fn: Option<NormFieldMetaFn>,
    windows: &TreeWindows<'_>,
    depth: usize,
) -> UiAssemblyResult<BuiltNode> {
    if depth > 24 {
        return tree_item_desc(format!("norm-inputs.{path}"), norm_ui_label(path)?, Some("…".into()));
    }
    let meta = resolve_field_meta(path, meta_fn);
    let label = field_label(path, meta, locale);
    match value {
        semio_framework_value::DslValue::Object(entries) => render_object_editor(path, entries, locale, controller_id, meta_fn, windows, depth, label),
        semio_framework_value::DslValue::Array(items) => render_array_editor(path, items, locale, controller_id, meta_fn, windows, depth, label),
        semio_framework_value::DslValue::Bool(flag) => {
            let row_id = format!("norm-inputs.{path}");
            let options: Vec<(String, String)> = vec![("true".into(), chrome("True", "Wahr", locale)), ("false".into(), chrome("False", "Falsch", locale))];
            let mut control = ui::select(ui_text(if *flag { "true" } else { "false" })?).try_id(format!("{row_id}.select")).map_err(|_| ui_error("ui.node.id"))?;
            for (option, option_label) in options {
                control = control.try_item(ui_text(option)?, norm_ui_label(option_label)?).map_err(|_| ui_error("ui.select.item"))?;
            }
            let args = action_args_map(vec![("path", ui_value_text(path)?)])?;
            control_row(&row_id, &label, bind_change(control, controller_id, "setField", args)?.try_build().map_err(|_| ui_error("ui.node.build"))?)
        }
        semio_framework_value::DslValue::Number(number) => {
            let row_id = format!("norm-inputs.{path}");
            let display = if let Some(unit) = meta.and_then(|meta| meta.unit) {
                format!("{} {unit}", number.as_f64())
            } else {
                format!("{}", number.as_f64())
            };
            let control = ui::input(InputKind::Number).value(ui_text(display)?).try_id(format!("{row_id}.input")).map_err(|_| ui_error("ui.node.id"))?;
            let args = action_args_map(vec![("path", ui_value_text(path)?)])?;
            control_row(&row_id, &label, bind_change(control, controller_id, "setField", args)?.try_build().map_err(|_| ui_error("ui.node.build"))?)
        }
        semio_framework_value::DslValue::String(raw) => {
            let row_id = format!("norm-inputs.{path}");
            if let Some(choices) = meta.and_then(|meta| meta.choices) {
                let mut control = ui::select(ui_text(raw)?).try_id(format!("{row_id}.select")).map_err(|_| ui_error("ui.node.id"))?;
                for choice in choices.iter().take(32) {
                    let choice_label = chrome(choice.label_en, choice.label_de, locale);
                    control = control.try_item(ui_text(choice.value)?, norm_ui_label(choice_label)?).map_err(|_| ui_error("ui.select.item"))?;
                }
                let args = action_args_map(vec![("path", ui_value_text(path)?)])?;
                control_row(&row_id, &label, bind_change(control, controller_id, "setField", args)?.try_build().map_err(|_| ui_error("ui.node.build"))?)
            } else {
                let control = ui::input(InputKind::Text).value(ui_text(raw)?).commit(ui_text("blur")?).try_id(format!("{row_id}.input")).map_err(|_| ui_error("ui.node.id"))?;
                let args = action_args_map(vec![("path", ui_value_text(path)?)])?;
                control_row(&row_id, &label, bind_on(control, Trigger::Commit, controller_id, "setField", args)?.try_build().map_err(|_| ui_error("ui.node.build"))?)
            }
        }
        semio_framework_value::DslValue::Bytes(bytes) => tree_item_desc(format!("norm-inputs.{path}"), norm_ui_label(label)?, Some(format!("{} {}", bytes.len(), chrome("bytes", "Bytes", locale)))),
        semio_framework_value::DslValue::Null => tree_item_desc(format!("norm-inputs.{path}"), norm_ui_label(label)?, Some(chrome("empty", "leer", locale))),
    }
}

/// 📄️ Structured, schema-driven, localized property editor over the document's camelCase value tree.
pub fn render_document_editor<D: semio_framework_value::ToValue>(
    document: &D,
    locale: Locale,
    controller_id: &'static str,
    meta_fn: Option<NormFieldMetaFn>,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode> {
    let tree = semio_framework_value::ToValue::to_value(document);
    render_value_editor("", &tree, locale, controller_id, meta_fn, windows, 0)
}

/// 📄️ Legacy pretty JSON — kept for tests that still project raw text chunks.
pub fn render_document_json<D: semio_framework_value::ToValue>(document: &D) -> UiAssemblyResult<BuiltNode> {
    let json = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(document)));
    render_text_chunks(&json)
}

/// 🧾️ Localized summary headline from the report rollup.
pub fn render_summary<F: NormFamily>(host: &NormHost<F>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = host.report();
    let summary = &report.summary;
    let verdict = if summary.complies {
        chrome("complies", "erfüllt", locale)
    } else {
        chrome("does not comply", "nicht erfüllt", locale)
    };
    let parts = summary
        .parts
        .iter()
        .map(|part| {
            let mark = if part.complies { "✓" } else { "✗" };
            format!("{mark}{}", part.part)
        })
        .collect::<Vec<_>>()
        .join(", ");
    render_text(format!(
        "{} — {} {}: {verdict}: {} {}, {} {}, {} {} · {}={:.2} · {parts}",
        F::family_id().label(),
        summary.total,
        chrome("checks", "Nachweise", locale),
        summary.fail,
        chrome("failing", "nicht bestanden", locale),
        summary.warning,
        chrome("warnings", "Warnungen", locale),
        summary.pass,
        chrome("passing", "bestanden", locale),
        chrome("worst u", "max. Ausnutzung", locale),
        summary.worst_utilization
    ))
}
//#endregion 🔖️Render
