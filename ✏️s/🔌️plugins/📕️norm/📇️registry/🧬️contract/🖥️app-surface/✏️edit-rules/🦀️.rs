//! ✏️ Edit rules of the norm app surface: the edit kinds, selector keys, edit rules and the set, insert, remove and remedy handlers. See [`super`].

use super::*;

//#region 🔖️ValuePath
/// 🧭 One value-tree edit intent addressed by a camelCase path — what the generic `setField` / `insertItem` / `removeItem` /
/// `applyRemedy` commands raise. A family resolves it to concrete kind mutations; nothing here compares or rewrites a snapshot.
#[derive(Clone, Debug, PartialEq)]
pub enum NormEdit {
    SetField { path: String, value: semio_framework_value::DslValue },
    InsertItem { path: String, index: usize, value: Option<semio_framework_value::DslValue> },
    RemoveItem { path: String, index: usize },
}

/// 🔑 How one path selector (`[index]` / `[id=…]`) is spelled in a leaf payload: its position, its row id, or not at all when the kind implies it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectorKey {
    Index(&'static str),
    Id(&'static str),
    Implied,
}

/// ✏️ One settable leaf: the `path` template (`walls[].lengthM`), the semantic `kind` it raises and where its selectors and value go in the payload.
#[derive(Clone, Copy, Debug)]
pub struct SetFieldRule {
    pub path: &'static str,
    pub kind: &'static str,
    pub selectors: &'static [SelectorKey],
    pub value: &'static str,
}

/// ➕ One insertable list: the `path` template of the array (`walls`, `walls[].openings`) and the kind that inserts a row into it.
#[derive(Clone, Copy, Debug)]
pub struct InsertItemRule {
    pub path: &'static str,
    pub kind: &'static str,
    pub selectors: &'static [SelectorKey],
    pub index: &'static str,
    pub item: &'static str,
}

/// ➖ One removable list: the `path` template of the array and the kind that removes the row at an index.
#[derive(Clone, Copy, Debug)]
pub struct RemoveItemRule {
    pub path: &'static str,
    pub kind: &'static str,
    pub selectors: &'static [SelectorKey],
    pub index: &'static str,
}

/// 📚 A family's declarative edit vocabulary: which paths raise which concrete kind.
#[derive(Clone, Copy, Debug)]
pub struct NormEditRules {
    pub set_field: &'static [SetFieldRule],
    pub insert_item: &'static [InsertItemRule],
    pub remove_item: &'static [RemoveItemRule],
}

struct Addressed {
    template: String,
    selected: Vec<(usize, Option<String>)>,
}

fn address(root: &semio_framework_value::DslValue, segments: &[PathSegment], path: &str) -> Result<Addressed, String> {
    let mut cursor = root;
    let mut template = String::new();
    let mut selected = Vec::new();
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                let semio_framework_value::DslValue::Object(entries) = cursor else {
                    return Err(format!("expected object before field '{name}' in path '{path}'"));
                };
                let Some((_, value)) = entries.iter().find(|(key, _)| key == name) else {
                    return Err(format!("missing field '{name}' in path '{path}'"));
                };
                if !template.is_empty() {
                    template.push('.');
                }
                template.push_str(name);
                cursor = value;
            }
            PathSegment::Index(_) | PathSegment::Id(_) => {
                let semio_framework_value::DslValue::Array(items) = cursor else {
                    return Err(format!("expected array before list selector in path '{path}'"));
                };
                let index = resolve_list_index(items, segment, path)?;
                selected.push((index, object_id_field(&items[index]).map(str::to_string)));
                template.push_str("[]");
                cursor = &items[index];
            }
        }
    }
    Ok(Addressed { template, selected })
}

fn selector_entries(selectors: &[SelectorKey], selected: &[(usize, Option<String>)], path: &str) -> Result<Vec<(String, semio_framework_value::DslValue)>, String> {
    if selectors.len() != selected.len() {
        return Err(format!("path '{path}' selects {} list rows, its edit rule addresses {}", selected.len(), selectors.len()));
    }
    selectors
        .iter()
        .zip(selected)
        .filter_map(|(key, (index, id))| match key {
            SelectorKey::Index(name) => Some(Ok(((*name).to_string(), semio_framework_value::DslValue::uint(*index as u64)))),
            SelectorKey::Id(name) => Some(id.clone().map(|id| ((*name).to_string(), semio_framework_value::DslValue::String(id))).ok_or_else(|| format!("path '{path}' selects a row without an id"))),
            SelectorKey::Implied => None,
        })
        .collect()
}

impl NormEditRules {
    /// 🎯 Resolves `edit` against `document` into the concrete kind mutations it denotes: the path names the leaf, the rule names the
    /// kind, the payload is the selectors the path resolved plus the new value — no snapshot is rewritten or compared.
    pub fn resolve<D, M>(&self, document: &D, edit: &NormEdit) -> Result<Vec<M>, String>
    where
        D: semio_framework_value::ToValue,
        M: protocol::Mutation<D>,
    {
        let tree = semio_framework_value::ToValue::to_value(document);
        let (kind, entries) = match edit {
            NormEdit::SetField { path, value } => {
                let addressed = address(&tree, &parse_path(path)?, path)?;
                let rule = self.set_field.iter().find(|rule| rule.path == addressed.template).ok_or_else(|| format!("no kind sets '{}' ({path})", addressed.template))?;
                let mut entries = selector_entries(rule.selectors, &addressed.selected, path)?;
                entries.push((rule.value.to_string(), value.clone()));
                (rule.kind, entries)
            }
            NormEdit::InsertItem { path, index, value } => {
                let addressed = address(&tree, &parse_path(path)?, path)?;
                let rule = self.insert_item.iter().find(|rule| rule.path == addressed.template).ok_or_else(|| format!("no kind inserts into '{}' ({path})", addressed.template))?;
                let mut entries = selector_entries(rule.selectors, &addressed.selected, path)?;
                entries.push((rule.index.to_string(), semio_framework_value::DslValue::uint(*index as u64)));
                entries.push((rule.item.to_string(), value.clone().ok_or_else(|| format!("insertItem '{path}' needs a value"))?));
                (rule.kind, entries)
            }
            NormEdit::RemoveItem { path, index } => {
                let segments = parse_path(path)?;
                let (collection, element) = match segments.split_last() {
                    Some((PathSegment::Index(_) | PathSegment::Id(_), parent)) => {
                        let parent_address = address(&tree, parent, path)?;
                        let full = address(&tree, &segments, path)?;
                        let element = full.selected.last().map(|(position, _)| *position).ok_or_else(|| format!("removeItem '{path}' selects no list row"))?;
                        (parent_address, element)
                    }
                    _ => (address(&tree, &segments, path)?, *index),
                };
                let rule = self.remove_item.iter().find(|rule| rule.path == collection.template).ok_or_else(|| format!("no kind removes from '{}' ({path})", collection.template))?;
                let mut entries = selector_entries(rule.selectors, &collection.selected, path)?;
                entries.push((rule.index.to_string(), semio_framework_value::DslValue::uint(element as u64)));
                (rule.kind, entries)
            }
        };
        M::from_payload_value(kind, semio_framework_value::DslValue::object(entries)).map(|mutation| vec![mutation]).map_err(|error| error.to_string())
    }
}

fn unresolved(error: String) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("norm.edit-unresolved"), error)
}

/// ✏️ Generic `setField` — `{path, value}` over the document value tree, resolved by the family to concrete kinds.
///
/// 📐️ **SI convention:** snapshot scalar quantity fields MUST store SI (m, N, Pa, …). `applyRemedy` writes
/// `Remedy.required.value` (already SI) straight into `remedy.target.path`. Family agents must not store mm/kN/MPa
/// in snapshot fields — display units are UI-only via [`format_quantity`] / [`NormFieldMeta::unit`].
pub fn handle_set_field<D, M, F>(document: &D, path: &str, value: semio_framework_value::DslValue, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve(document, &NormEdit::SetField { path: path.to_string(), value }).map_err(unresolved)?)
}

/// ➕ Generic `insertItem` — `{path, index, value?}`.
pub fn handle_insert_item<D, M, F>(document: &D, path: &str, index: usize, value: Option<semio_framework_value::DslValue>, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve(document, &NormEdit::InsertItem { path: path.to_string(), index, value }).map_err(unresolved)?)
}

/// ➖ Generic `removeItem` — `{path, index}`.
pub fn handle_remove_item<D, M, F>(document: &D, path: &str, index: usize, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve(document, &NormEdit::RemoveItem { path: path.to_string(), index }).map_err(unresolved)?)
}

/// 🩹 Resolves a remedy on `report` into the path it writes and the value it writes there (numeric SI or OneOf option string), reading
/// the current leaf type from `tree` without touching it.
pub fn remedy_edit(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &semio_framework_value::DslValue) -> Result<(String, semio_framework_value::DslValue), Fault> {
    let check = report.checks.iter().find(|check| check.id == check_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-check"), format!("check '{check_id}' not in report")))?;
    let remedy = check.remedies.get(remedy_index).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-remedy"), format!("remedy {remedy_index} missing on '{check_id}'")))?;
    if !remedy.applicable {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-not-applicable"), format!("remedy {remedy_index} on '{check_id}' is not applicable")));
    }
    let path = remedy.target.path.clone();
    if path.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-empty-path"), format!("remedy {remedy_index} on '{check_id}' has empty target path")));
    }
    let current = get_value_at_path(tree, &path).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))?;
    let value = match remedy.bound {
        RemedyBound::OneOf => {
            let option = remedy
                .options
                .get(option_index)
                .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-option"), format!("option {option_index} missing on OneOf remedy {remedy_index} for '{check_id}'")))?;
            match current {
                semio_framework_value::DslValue::Bool(_) => semio_framework_value::DslValue::Bool(option.eq_ignore_ascii_case("true") || option == "1"),
                semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(_)) => {
                    let parsed = option.parse::<u64>().map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-option-type"), format!("OneOf option '{option}' is not a u64 for '{path}'")))?;
                    semio_framework_value::DslValue::uint(parsed)
                }
                semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(_)) => {
                    let parsed = option.parse::<i64>().map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-option-type"), format!("OneOf option '{option}' is not an i64 for '{path}'")))?;
                    semio_framework_value::DslValue::int(parsed)
                }
                semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(_)) => {
                    let parsed = option.parse::<f64>().map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-option-type"), format!("OneOf option '{option}' is not a float for '{path}'")))?;
                    semio_framework_value::DslValue::float(parsed)
                }
                _ => semio_framework_value::DslValue::String(option.clone()),
            }
        }
        _ => match current {
            semio_framework_value::DslValue::Bool(_) => semio_framework_value::DslValue::Bool(remedy.required.value >= 0.5),
            semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(_)) => {
                if !remedy.required.value.is_finite() || remedy.required.value < 0.0 {
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-uint"), format!("required {} is not a u64 for '{path}'", remedy.required.value)));
                }
                semio_framework_value::DslValue::uint(remedy.required.value.round() as u64)
            }
            semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(_)) => {
                if !remedy.required.value.is_finite() {
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-int"), format!("required {} is not an i64 for '{path}'", remedy.required.value)));
                }
                semio_framework_value::DslValue::int(remedy.required.value.round() as i64)
            }
            semio_framework_value::DslValue::String(_) => semio_framework_value::DslValue::String(format!("{}", remedy.required.value)),
            _ => semio_framework_value::DslValue::float(remedy.required.value),
        },
    };
    Ok((path, value))
}

/// 🩹 Writes a resolved remedy into a value tree — the remedy laws' perturbation helper; the mutation path resolves the same edit
/// to a concrete kind instead.
pub fn apply_remedy_edit(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &mut semio_framework_value::DslValue) -> Result<(), Fault> {
    let (path, value) = remedy_edit(report, check_id, remedy_index, option_index, tree)?;
    set_value_at_path(tree, &path, value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))
}

/// 🩹 Generic `applyRemedy` — re-evaluates, finds `checkId`, writes SI `required` (or OneOf option) into `remedy.target.path`.
pub fn handle_apply_remedy<F, Map>(document: &F::Document, check_id: &str, remedy_index: usize, resolve: Map) -> Result<Emit<F::Mutation, NoConfigMutation>, Fault>
where
    F: NormFamily,
    F::Document: Clone + semio_framework_value::ToValue,
    Map: FnOnce(&F::Document, &NormEdit) -> Result<Vec<F::Mutation>, String>,
{
    handle_apply_remedy_with_option::<F, Map>(document, check_id, remedy_index, 0, resolve)
}

/// 🩹 `applyRemedy` with an explicit OneOf `option_index` (ignored for numeric bounds).
pub fn handle_apply_remedy_with_option<F, Map>(document: &F::Document, check_id: &str, remedy_index: usize, option_index: usize, resolve: Map) -> Result<Emit<F::Mutation, NoConfigMutation>, Fault>
where
    F: NormFamily,
    F::Document: Clone + semio_framework_value::ToValue,
    Map: FnOnce(&F::Document, &NormEdit) -> Result<Vec<F::Mutation>, String>,
{
    let report = cached_report_for::<F>(document).unwrap_or_else(|| F::evaluate(document));
    let tree = semio_framework_value::ToValue::to_value(document);
    let (path, value) = remedy_edit(&report, check_id, remedy_index, option_index, &tree)?;
    commit_snapshot_fields(resolve(document, &NormEdit::SetField { path, value }).map_err(unresolved)?)
}
//#endregion 🔖️ValuePath
