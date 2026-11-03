//! ✏️ Case edits: the editor actions of design cases as value-tree edits. The catalogue browser adds a seeded case (`insertItem cases`), the properties window sets or clears one unit-aware input (`setField cases[id=ID].numbers[id=KEY].value`, `null` clears), the case list duplicates and removes. Every edit is a [`NormEdit`] the artifact resolves to `insert-case`, `change-case-number|text` or `remove-case`; the live session re-evaluates the case on each. See [`super`] and `../../⚖️compliance/🧰️design-case`.
use super::*;
use semio_framework_plugin::UiListBuilder;
use semio_framework_value::DslValue;
use semio_s_plugin_norm_compliance::design_case::{CaseRow, DesignCase};

//#region 🔖️Ids
/// 🏷️ The lower-case slug of `text`: alphanumerics kept, every other run becomes one dash.
pub fn case_slug(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// 🆔 A case id for `stem` that no row has yet: the slug itself, then `-2`, `-3`, ….
pub fn next_case_id(rows: &[CaseRow], stem: &str) -> String {
    let stem = case_slug(stem);
    if !rows.iter().any(|row| row.id == stem) {
        return stem;
    }
    (2..).map(|n| format!("{stem}-{n}")).find(|id| !rows.iter().any(|row| &row.id == id)).unwrap_or_default()
}
//#endregion 🔖️Ids

//#region 🔖️CaseEdits
fn cases_edit(index: usize, case: &CaseRow) -> NormEdit {
    NormEdit::InsertItem { path: "cases".into(), index, value: Some(semio_framework_value::ToValue::to_value(case)) }
}

/// ➕️ The browser's add-case action: a new case of catalogue entry `entry_id` seeded with the example values of its declared inputs, inserted at `index` (clamped).
pub fn insert_case_edit<C: CaseCatalogue>(rows: &[CaseRow], entry_id: &str, index: usize) -> Result<NormEdit, String> {
    let entry = design_case::entry::<C>(entry_id).ok_or_else(|| format!("unknown catalogue entry {entry_id}"))?;
    let case = CaseRow::from(&design_case::seed::<C>(&entry, &next_case_id(rows, &format!("case-{}", entry.id))));
    Ok(cases_edit(index.min(rows.len()), &case))
}

/// 📑 The duplicate action: a copy of the case at `index` with a fresh id and marked labels, inserted right after it.
pub fn duplicate_case_edit(rows: &[CaseRow], index: usize) -> Result<NormEdit, String> {
    let source = rows.get(index).ok_or_else(|| format!("case index {index} out of range"))?;
    let copy = CaseRow { id: next_case_id(rows, &source.id), label_en: format!("{} (copy)", source.label_en), label_de: format!("{} (Kopie)", source.label_de), ..source.clone() };
    Ok(cases_edit(index + 1, &copy))
}

/// ➖️ The remove action for the case at `index`.
pub fn remove_case_edit(rows: &[CaseRow], index: usize) -> Result<NormEdit, String> {
    if index >= rows.len() {
        return Err(format!("case index {index} out of range"));
    }
    Ok(NormEdit::RemoveItem { path: "cases".into(), index })
}

/// 🔢️ Sets the number input `key` of case `case_id` (stored unit), creating the row when the input was empty.
pub fn set_number_edit(case_id: &str, key: &str, value: f64) -> NormEdit {
    NormEdit::SetField { path: design_case::input_path(case_id, key, true), value: DslValue::float(value) }
}

/// 🔤 Sets the text or choice input `key` of case `case_id`.
pub fn set_text_edit(case_id: &str, key: &str, value: &str) -> NormEdit {
    NormEdit::SetField { path: design_case::input_path(case_id, key, false), value: DslValue::String(value.into()) }
}

/// 🧹 Clears the input `key` of case `case_id`: the `null` set the artifact turns into the removal of the input row.
pub fn clear_input_edit(case_id: &str, key: &str, number: bool) -> NormEdit {
    NormEdit::SetField { path: design_case::input_path(case_id, key, number), value: DslValue::Null }
}
//#endregion 🔖️CaseEdits

//#region 🔖️InputText
/// 🚫 Why the text typed into an input row is not accepted.
#[derive(Clone, Debug, PartialEq)]
pub enum CaseInputError {
    NotANumber(String),
    OutOfRange { value: f64, min: f64, max: f64 },
    NotAChoice(String),
}
impl CaseInputError {
    pub fn message(&self, locale: Locale) -> String {
        match self {
            Self::NotANumber(text) => chrome(&format!("'{text}' is not a number with a known unit."), &format!("'{text}' ist keine Zahl mit bekannter Einheit."), locale),
            Self::OutOfRange { value, min, max } => chrome(&format!("{value} is outside {min} … {max}."), &format!("{value} liegt außerhalb {min} … {max}."), locale),
            Self::NotAChoice(text) => chrome(&format!("'{text}' is not an option."), &format!("'{text}' ist keine Option."), locale),
        }
    }
}

/// ✍️ The edit for text typed into the input `row`: empty text clears the input, a number is parsed with the unit it names (bare numbers take `display`), converted to the stored unit and checked against the range, a choice must be an option, text is kept.
pub fn input_row_edit(row: &CaseInputRow, text: &str, display: UnitId) -> Result<NormEdit, CaseInputError> {
    let text = text.trim();
    let Some((key, number)) = case_input_key(&row.path) else { return Err(CaseInputError::NotANumber(text.into())) };
    let case_id = row.path.strip_prefix("cases[id=").and_then(|rest| rest.split_once(']')).map_or("", |(id, _)| id);
    if text.is_empty() {
        return Ok(clear_input_edit(case_id, key, number));
    }
    match row.spec.kind {
        InputKind::Number | InputKind::Vector | InputKind::Series => {
            let value = row.field.parse(text, display).ok().filter(|value| value.is_finite()).ok_or_else(|| CaseInputError::NotANumber(text.into()))?;
            match row.spec.range.filter(|_| !row.field.accepts(value)) {
                Some((min, max)) => Err(CaseInputError::OutOfRange { value, min, max }),
                None => Ok(set_number_edit(case_id, key, value)),
            }
        }
        InputKind::Choice if !row.spec.choices.iter().any(|choice| choice.value == text) => Err(CaseInputError::NotAChoice(text.into())),
        InputKind::Choice | InputKind::Text => Ok(set_text_edit(case_id, key, text)),
    }
}

/// 🖨️ The current value of the input `row` as text in `display` (empty when unset).
pub fn input_row_text(row: &CaseInputRow, display: UnitId, locale: Locale) -> String {
    match (row.number, &row.text) {
        (Some(value), _) => row.field.format(value, display, locale).unwrap_or_default(),
        (None, Some(text)) => text.clone(),
        _ => String::new(),
    }
}
//#endregion 🔖️InputText

//#region 🔖️Projection
fn empty_row(key: &str, value: DslValue) -> DslValue {
    DslValue::object(vec![("id".to_string(), DslValue::String(key.into())), ("value".to_string(), value)])
}

fn with_empty_inputs(case: &DslValue, rows: &[CaseInputRow]) -> DslValue {
    let DslValue::Object(entries) = case else { return case.clone() };
    let missing = |number: bool| -> Vec<DslValue> {
        rows.iter()
            .filter(|row| row.declared && row.spec.is_number() == number && if number { row.number.is_none() } else { row.text.is_none() })
            .filter_map(|row| case_input_key(&row.path).map(|(key, _)| empty_row(key, if number { DslValue::Null } else { DslValue::String(String::new()) })))
            .collect()
    };
    DslValue::object(entries.iter().map(|(name, value)| {
        let number = name == "numbers";
        match (value, number || name == "texts") {
            (DslValue::Array(items), true) => (name.clone(), DslValue::Array(items.iter().cloned().chain(missing(number)).collect())),
            _ => (name.clone(), value.clone()),
        }
    }))
}

/// 🪟️ The value tree the properties window shows: every case lists the inputs its catalogue entry declares, the unset ones as empty rows (`null` number, empty text) so they can be typed in; the document itself is not touched.
pub fn case_input_tree<C: CaseCatalogue>(tree: &DslValue) -> DslValue {
    let DslValue::Object(entries) = tree else { return tree.clone() };
    DslValue::object(entries.iter().map(|(name, value)| match (name.as_str(), value) {
        ("cases", DslValue::Array(cases)) => {
            let projected = cases
                .iter()
                .map(|case| match <CaseRow as semio_framework_value::FromValue>::from_value(case.clone()) {
                    Ok(row) => with_empty_inputs(case, &case_input_rows::<C>(&DesignCase::from(&row))),
                    Err(_) => case.clone(),
                })
                .collect();
            (name.clone(), DslValue::Array(projected))
        }
        _ => (name.clone(), value.clone()),
    }))
}

/// 🧾️ The inputs window of a norm app with design cases: [`render_inspector_projected`] over [`case_input_tree`].
pub fn render_inspector_with_cases<F: NormFamily, C: CaseCatalogue>(host: &NormHost<F>, locale: Locale, controller_id: &'static str, meta_fn: Option<NormFieldMetaFn>, windows: &TreeWindows<'_>, options: &InspectorOptions) -> UiAssemblyResult<BuiltNode> {
    render_inspector_projected(host, locale, controller_id, meta_fn, windows, options, |tree| case_input_tree::<C>(&tree))
}
//#endregion 🔖️Projection

//#region 🔖️Browser
fn ui_value_of(value: &DslValue) -> UiAssemblyResult<UiValue> {
    Ok(match value {
        DslValue::Null | DslValue::Bytes(_) => UiValue::Null,
        DslValue::Bool(flag) => UiValue::Bool(*flag),
        DslValue::Number(number) => UiValue::Number(number.as_f64()),
        DslValue::String(text) => ui_value_text(text)?,
        DslValue::Array(items) => {
            let mut list = UiListBuilder::try_new().ok_or_else(|| ui_error("ui.value.list"))?;
            for item in items {
                list.push(ui_value_of(item)?).map_err(|_| ui_error("ui.value.list.item"))?;
            }
            UiValue::List(list.finish())
        }
        DslValue::Object(entries) => {
            let mut map = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
            for (key, item) in entries {
                map.push(key.as_str().into(), ui_value_of(item)?).map_err(|_| ui_error("ui.value.map.entry"))?;
            }
            UiValue::Map(map.finish())
        }
    })
}

fn edit_args(edit: &NormEdit) -> UiAssemblyResult<UiValue> {
    match edit {
        NormEdit::InsertItem { path, index, value } => {
            let mut entries = vec![("path", ui_value_text(path)?), ("index", UiValue::Number(*index as f64))];
            if let Some(value) = value {
                entries.push(("value", ui_value_of(value)?));
            }
            action_args_map(entries)
        }
        NormEdit::RemoveItem { path, index } => action_args_map(vec![("path", ui_value_text(path)?), ("index", UiValue::Number(*index as f64))]),
        NormEdit::SetField { path, value } => action_args_map(vec![("path", ui_value_text(path)?), ("value", ui_value_of(value)?)]),
        NormEdit::MoveItem { path, from, to } => action_args_map(vec![("path", ui_value_text(path)?), ("from", UiValue::Number(*from as f64)), ("to", UiValue::Number(*to as f64))]),
    }
}

fn edit_action(controller_id: &'static str, edit: &NormEdit) -> UiAssemblyResult<(semio_framework_plugin::ActionId, Option<UiValue>)> {
    let action = match edit {
        NormEdit::InsertItem { .. } => "insertItem",
        NormEdit::RemoveItem { .. } => "removeItem",
        NormEdit::SetField { .. } => "setField",
        NormEdit::MoveItem { .. } => return Err(ui_error("ui.edit.move-item-has-no-action")),
    };
    norm_action(controller_id, action, Some(edit_args(edit)?))
}

/// 📚️ The case browser panel: the catalogue entries matching `query` each with an add-case action, then the document's cases each with duplicate and remove actions; every action is the `insertItem` or `removeItem` command of the artifact.
pub fn render_case_browser<C: CaseCatalogue>(rows: &[CaseRow], query: &str, locale: Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let entries = design_case::search(&C::entries(), query);
    let listed: Vec<&CaseEntry> = entries.iter().collect();
    let cases: Vec<(usize, &CaseRow)> = rows.iter().enumerate().collect();
    let add_header = chrome("Add case from the catalogue", "Fall aus dem Katalog hinzufügen", locale);
    let case_header = chrome("Cases", "Fälle", locale);
    PanelTreeBuilder::new("norm-cases")?
        .window_section_or_placeholder(
            windows,
            "norm-cases.catalogue",
            Some(norm_ui_label(format!("{add_header} ({})", listed.len()))?),
            true,
            &listed,
            |entry| {
                let edit = insert_case_edit::<C>(rows, entry.id, rows.len()).map_err(|_| ui_error("norm.case-insert"))?;
                tree_item_with_action(format!("norm-cases.add.{}", entry.id), norm_ui_label(format!("{} · {} · {}", entry.id, semio_s_plugin_norm_compliance::designator(entry.clause, &locale), entry.title(&locale)))?, None, edit_action(controller_id, &edit)?)
            },
            norm_ui_label(chrome("No matching entry", "Kein passender Eintrag", locale))?,
        )?
        .window_section_or_placeholder(
            windows,
            "norm-cases.list",
            Some(norm_ui_label(format!("{case_header} ({})", cases.len()))?),
            true,
            &cases,
            |(index, row)| {
                let label = chrome(&row.label_en, &row.label_de, locale);
                let duplicate = duplicate_case_edit(rows, *index).map_err(|_| ui_error("norm.case-duplicate"))?;
                let remove = remove_case_edit(rows, *index).map_err(|_| ui_error("norm.case-remove"))?;
                let mut item = ui::tree_item(norm_ui_label(format!("{} · {label}", row.entry_id))?).try_id(format!("norm-cases.case.{}", row.id)).map_err(|_| ui_error("ui.node.id"))?;
                item = item.try_child(tree_item_with_action(format!("norm-cases.case.{}.duplicate", row.id), norm_ui_label(chrome("Duplicate", "Duplizieren", locale))?, None, edit_action(controller_id, &duplicate)?)?).map_err(|_| ui_error("ui.node.child"))?;
                item = item.try_child(tree_item_with_action(format!("norm-cases.case.{}.remove", row.id), norm_ui_label(chrome("Remove", "Entfernen", locale))?, None, edit_action(controller_id, &remove)?)?).map_err(|_| ui_error("ui.node.child"))?;
                item.try_build().map_err(|_| ui_error("ui.node.build"))
            },
            norm_ui_label(chrome("No cases yet", "Noch keine Fälle", locale))?,
        )?
        .build()
}

/// 📚️ The catalogue panel `catalogue` with the case browser below it (one column).
pub fn with_case_browser<C: CaseCatalogue>(catalogue: BuiltNode, rows: &[CaseRow], locale: Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let browser = render_case_browser::<C>(rows, "", locale, controller_id, windows)?;
    ui::column().try_children([catalogue, browser]).map_err(|_| ui_error("ui.node.children"))?.try_build().map_err(|_| ui_error("ui.node.build"))
}
//#endregion 🔖️Browser

#[path = "🧪️laws/🦀️.rs"]
pub mod case_edit_laws;
pub use case_edit_laws::assert_case_edit_paths;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
