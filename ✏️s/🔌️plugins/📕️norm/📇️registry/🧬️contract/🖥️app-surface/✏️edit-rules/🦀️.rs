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
    MoveItem { path: String, from: usize, to: usize },
}

/// 🔑 How one path selector (`[index]` / `[id=…]`) is spelled in a leaf payload: its position, its row id, or not at all when the kind implies it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectorKey {
    Index(&'static str),
    Id(&'static str),
    Implied,
}

/// ✏️ One settable leaf: the `path` template (`walls[].lengthM`), the semantic `kind` it raises and where its selectors and value go in the payload; an empty `value` declares the leaf's clear action, raised by a `null` write.
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

/// ↕️ One reorderable list: the `path` template of the array and the kind that moves the row at `from` to `to` (both positions in the list before the move), with the parent selectors of the payload.
#[derive(Clone, Copy, Debug)]
pub struct MoveItemRule {
    pub path: &'static str,
    pub kind: &'static str,
    pub selectors: &'static [SelectorKey],
    pub from: &'static str,
    pub to: &'static str,
}

/// 🆔 One removable list whose remove kind addresses the row by its id, never by its position: the `path` template of the array, the
/// parent selectors of the payload and the payload field that carries the removed row's id.
#[derive(Clone, Copy, Debug)]
pub struct IdRemoval {
    pub path: &'static str,
    pub kind: &'static str,
    pub selectors: &'static [SelectorKey],
    pub id: &'static str,
}

/// 📚 A family's declarative edit vocabulary: which paths raise which concrete kind.
#[derive(Clone, Copy, Debug)]
pub struct NormEditRules {
    pub set_field: &'static [SetFieldRule],
    pub insert_item: &'static [InsertItemRule],
    pub remove_item: &'static [RemoveItemRule],
    pub move_item: &'static [MoveItemRule],
}

pub struct Addressed {
    pub template: String,
    pub selected: Vec<(usize, Option<String>)>,
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

/// 🌱 Addresses a path whose last list row is not there yet (`cases[id=C1].numbers[id=h].value` before the input `h` is set): the missing row keeps its id as selector, the kind that sets the leaf creates the row.
pub fn address_pending(root: &semio_framework_value::DslValue, segments: &[PathSegment]) -> Option<Addressed> {
    let mut cursor = Some(root);
    let mut template = String::new();
    let mut selected = Vec::new();
    let mut pending = false;
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                if let Some(semio_framework_value::DslValue::Object(entries)) = cursor {
                    cursor = entries.iter().find(|(key, _)| key == name).map(|(_, value)| value);
                    if cursor.is_none() && !pending {
                        return None;
                    }
                } else if !pending {
                    return None;
                }
                if !template.is_empty() {
                    template.push('.');
                }
                template.push_str(name);
            }
            PathSegment::Index(_) | PathSegment::Id(_) => {
                if pending {
                    return None;
                }
                let semio_framework_value::DslValue::Array(items) = cursor? else { return None };
                match resolve_list_index(items, segment, "") {
                    Ok(index) => {
                        selected.push((index, object_id_field(&items[index]).map(str::to_string)));
                        cursor = Some(&items[index]);
                    }
                    Err(_) => match segment {
                        PathSegment::Id(id) if !items.iter().any(|item| object_id_field(item) == Some(id.as_str())) => {
                            selected.push((items.len(), Some(id.clone())));
                            cursor = None;
                            pending = true;
                        }
                        _ => return None,
                    },
                }
                template.push_str("[]");
            }
        }
    }
    pending.then_some(Addressed { template, selected })
}

fn row_id_at(root: &semio_framework_value::DslValue, segments: &[PathSegment], element: usize, path: &str) -> Result<semio_framework_value::DslValue, String> {
    let mut cursor = root;
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                let semio_framework_value::DslValue::Object(entries) = cursor else {
                    return Err(format!("expected object before field '{name}' in path '{path}'"));
                };
                cursor = &entries.iter().find(|(key, _)| key == name).ok_or_else(|| format!("missing field '{name}' in path '{path}'"))?.1;
            }
            PathSegment::Index(_) | PathSegment::Id(_) => {
                let semio_framework_value::DslValue::Array(items) = cursor else {
                    return Err(format!("expected array before list selector in path '{path}'"));
                };
                cursor = &items[resolve_list_index(items, segment, path)?];
            }
        }
    }
    let semio_framework_value::DslValue::Array(items) = cursor else {
        return Err(format!("'{path}' is not a list"));
    };
    let row = items.get(element).ok_or_else(|| format!("'{path}' has no row {element}"))?;
    object_id_field(row).map(|id| semio_framework_value::DslValue::String(id.to_string())).ok_or_else(|| format!("row {element} of '{path}' has no id"))
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

/// 🧾 The concrete kind an edit denotes and its payload entries: the parent selectors the path resolved plus the new value, row id or position.
pub type EditPayload = (&'static str, Vec<(String, semio_framework_value::DslValue)>);

impl NormEditRules {
    /// 🧹 The rule that sets the leaf at `template`: a rule with an empty `value` is the leaf's clear action (its kind carries no value) and answers a `null` write, every other rule answers a value write.
    fn set_rule(&self, template: &str, clearing: bool) -> Option<&SetFieldRule> {
        let at = || self.set_field.iter().filter(move |rule| rule.path == template);
        at().find(|rule| rule.value.is_empty() == clearing).or_else(|| at().find(|rule| !rule.value.is_empty()))
    }

    /// 🧾 Resolves `edit` against the value `tree` into its [`EditPayload`], the one place that reads path templates, so top-level and nested
    /// rows share a mechanism: a path `parent[id=P].child[id=C].leaf` raises the rule whose template is `parent[].child[].leaf`, and the
    /// rule's `selectors` name where `P` and `C` land in the payload. A `removeItem` on a path of `id_removals` is addressed by the
    /// row's id (parent selectors plus the id field) instead of its position.
    pub fn payload(&self, id_removals: &[IdRemoval], tree: &semio_framework_value::DslValue, edit: &NormEdit) -> Result<EditPayload, String> {
        match edit {
            NormEdit::SetField { path, value } => {
                let segments = parse_path(path)?;
                let clearing = matches!(value, semio_framework_value::DslValue::Null);
                let (addressed, rule) = match address(tree, &segments, path) {
                    Ok(addressed) => {
                        let rule = self.set_rule(&addressed.template, clearing).ok_or_else(|| format!("no kind sets '{}' ({path})", addressed.template))?;
                        (addressed, rule)
                    }
                    Err(error) => {
                        let pending = address_pending(tree, &segments).ok_or_else(|| error.clone())?;
                        let rule = self.set_rule(&pending.template, clearing).ok_or(error)?;
                        (pending, rule)
                    }
                };
                let mut entries = selector_entries(rule.selectors, &addressed.selected, path)?;
                if !rule.value.is_empty() {
                    entries.push((rule.value.to_string(), value.clone()));
                }
                Ok((rule.kind, entries))
            }
            NormEdit::InsertItem { path, index, value } => {
                let addressed = address(tree, &parse_path(path)?, path)?;
                let rule = self.insert_item.iter().find(|rule| rule.path == addressed.template).ok_or_else(|| format!("no kind inserts into '{}' ({path})", addressed.template))?;
                let mut entries = selector_entries(rule.selectors, &addressed.selected, path)?;
                entries.push((rule.index.to_string(), semio_framework_value::DslValue::uint(*index as u64)));
                entries.push((rule.item.to_string(), value.clone().ok_or_else(|| format!("insertItem '{path}' needs a value"))?));
                Ok((rule.kind, entries))
            }
            NormEdit::MoveItem { path, from, to } => {
                let collection = address(tree, &parse_path(path)?, path)?;
                let rule = self.move_item.iter().find(|rule| rule.path == collection.template).ok_or_else(|| format!("no kind reorders '{}' ({path})", collection.template))?;
                let mut entries = selector_entries(rule.selectors, &collection.selected, path)?;
                entries.push((rule.from.to_string(), semio_framework_value::DslValue::uint(*from as u64)));
                entries.push((rule.to.to_string(), semio_framework_value::DslValue::uint(*to as u64)));
                Ok((rule.kind, entries))
            }
            NormEdit::RemoveItem { path, index } => {
                let segments = parse_path(path)?;
                let (collection_segments, element) = match segments.split_last() {
                    Some((PathSegment::Index(_) | PathSegment::Id(_), parent)) => {
                        let full = address(tree, &segments, path)?;
                        (parent, full.selected.last().map(|(position, _)| *position).ok_or_else(|| format!("removeItem '{path}' selects no list row"))?)
                    }
                    _ => (segments.as_slice(), *index),
                };
                let collection = address(tree, collection_segments, path)?;
                if let Some(rule) = id_removals.iter().find(|rule| rule.path == collection.template) {
                    let mut entries = selector_entries(rule.selectors, &collection.selected, path)?;
                    entries.push((rule.id.to_string(), row_id_at(tree, collection_segments, element, path)?));
                    return Ok((rule.kind, entries));
                }
                let rule = self.remove_item.iter().find(|rule| rule.path == collection.template).ok_or_else(|| format!("no kind removes from '{}' ({path})", collection.template))?;
                let mut entries = selector_entries(rule.selectors, &collection.selected, path)?;
                entries.push((rule.index.to_string(), semio_framework_value::DslValue::uint(element as u64)));
                Ok((rule.kind, entries))
            }
        }
    }

    /// 🎯 Resolves `edit` against `document` into the concrete kind mutations it denotes: the path names the leaf, the rule names the
    /// kind, the payload is the selectors the path resolved plus the new value — no snapshot is rewritten or compared.
    pub fn resolve<D, M>(&self, document: &D, edit: &NormEdit) -> Result<Vec<M>, String>
    where
        D: semio_framework_value::ToValue,
        M: protocol::Mutation<D>,
    {
        self.resolve_with_id_removals(&[], document, edit)
    }

    /// 🆔 Resolves like [`NormEditRules::resolve`], except that a `removeItem` on a path of `id_removals` raises the kind that removes the
    /// row by its id, nested rows included (`members[].actions` carries the `memberId` selector).
    pub fn resolve_with_id_removals<D, M>(&self, id_removals: &[IdRemoval], document: &D, edit: &NormEdit) -> Result<Vec<M>, String>
    where
        D: semio_framework_value::ToValue,
        M: protocol::Mutation<D>,
    {
        let tree = semio_framework_value::ToValue::to_value(document);
        let (kind, entries) = self.payload(id_removals, &tree, edit)?;
        M::from_payload_value(kind, semio_framework_value::DslValue::object(entries)).map(|mutation| vec![mutation]).map_err(|error| error.to_string())
    }
}

/// ↕️ The edit that moves the row `id` of the list at `path` to position `to` (clamped to the list), the intent of a drag-to-reorder in the outliner or the properties list; an unknown id or an unchanged position is an error so nothing is committed.
pub fn move_item_edit(tree: &semio_framework_value::DslValue, path: &str, id: &str, to: usize) -> Result<NormEdit, String> {
    let rows = get_value_at_path(tree, path)?.as_array().ok_or_else(|| format!("{path} is not a list"))?;
    let from = rows.iter().position(|row| object_id_field(row) == Some(id)).ok_or_else(|| format!("unknown id '{id}' in {path}"))?;
    let to = to.min(rows.len().saturating_sub(1));
    if from == to {
        return Err(format!("'{id}' is already at position {to} of {path}"));
    }
    Ok(NormEdit::MoveItem { path: path.to_string(), from, to })
}

/// ↕️ The edit of a drop: the row `id` takes the position of the row `over` it was dropped on (both positions are read before the move, so dropping on a later row moves down and on an earlier row moves up).
pub fn move_item_over_edit(tree: &semio_framework_value::DslValue, path: &str, id: &str, over: &str) -> Result<NormEdit, String> {
    let rows = get_value_at_path(tree, path)?.as_array().ok_or_else(|| format!("{path} is not a list"))?;
    let target = rows.iter().position(|row| object_id_field(row) == Some(over)).ok_or_else(|| format!("unknown id '{over}' in {path}"))?;
    move_item_edit(tree, path, id, target)
}

/// ↕️ The edit of a move-up (`delta` -1) or move-down (`delta` +1) step: the keyboard, touch and screen-reader alternative to the drag; a step off either end is an error so nothing is committed.
pub fn move_item_step_edit(tree: &semio_framework_value::DslValue, path: &str, id: &str, delta: isize) -> Result<NormEdit, String> {
    let rows = get_value_at_path(tree, path)?.as_array().ok_or_else(|| format!("{path} is not a list"))?;
    let from = rows.iter().position(|row| object_id_field(row) == Some(id)).ok_or_else(|| format!("unknown id '{id}' in {path}"))?;
    let to = from.checked_add_signed(delta).filter(|to| *to < rows.len()).ok_or_else(|| format!("'{id}' cannot move {delta} from position {from} of {path}"))?;
    move_item_edit(tree, path, id, to)
}

fn unresolved(error: String) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("norm.edit-unresolved"), error)
}

/// ✏️ Generic `setField` — `{path, value}` over the document value tree, resolved by the family to concrete kinds.
///
/// 📐️ **SI convention:** snapshot scalar quantity fields MUST store SI (m, N, Pa, …). `applyRemedy` writes
/// `Remedy.required.value` (already SI) straight into `remedy.target.path`. Family agents must not store mm/kN/MPa
/// in snapshot fields — display units are UI-only via [`format_quantity`] / [`FieldSpec::unit`].
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

/// 🩹 What a remedy does to the document: write a value at a leaf path, remove the row its target path selects, or insert a typed row at the end of the list its target path names.
#[derive(Clone, Debug, PartialEq)]
pub enum RemedyEdit {
    Set { path: String, value: semio_framework_value::DslValue },
    Remove { path: String },
    Insert { path: String, index: usize, value: semio_framework_value::DslValue },
}

/// 🩹 Resolves a remedy on `report` into the path it writes and the value it writes there (numeric SI or OneOf option string), reading
/// the current leaf type from `tree` without touching it; a row-removing remedy is no value write and fails here (see [`remedy_resolution`]).
pub fn remedy_edit(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &semio_framework_value::DslValue) -> Result<(String, semio_framework_value::DslValue), Fault> {
    match remedy_resolution(report, check_id, remedy_index, option_index, tree)? {
        RemedyEdit::Set { path, value } => Ok((path, value)),
        RemedyEdit::Remove { path } => Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.remedy-removes-row"), format!("remedy {remedy_index} on '{check_id}' removes the row '{path}' and writes no value"))),
        RemedyEdit::Insert { path, .. } => Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.remedy-inserts-row"), format!("remedy {remedy_index} on '{check_id}' inserts a row into '{path}' and writes no value"))),
    }
}

/// ➕ The typed row of an [`RemedyBound::Insert`] remedy: its first option parsed as a JSON object, every string value `"$required"` replaced by the remedy's computed `required` value, the row id checked against the list so a second application never duplicates it.
fn remedy_row(remedy: &Remedy, rows: &[semio_framework_value::DslValue]) -> Result<semio_framework_value::DslValue, String> {
    fn fill(value: &mut semio_framework_value::DslValue, required: f64) {
        match value {
            semio_framework_value::DslValue::String(text) if text == "$required" => *value = semio_framework_value::DslValue::float(required),
            semio_framework_value::DslValue::Array(items) => items.iter_mut().for_each(|item| fill(item, required)),
            semio_framework_value::DslValue::Object(entries) => entries.iter_mut().for_each(|(_, item)| fill(item, required)),
            _ => {}
        }
    }
    let json = remedy.options.first().ok_or("an insert remedy carries its row as the first option")?;
    let mut row = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    if !matches!(row, semio_framework_value::DslValue::Object(_)) {
        return Err("an insert remedy's row must be a JSON object".into());
    }
    fill(&mut row, remedy.required.value);
    match object_id_field(&row) {
        Some(id) if rows.iter().any(|existing| object_id_field(existing) == Some(id)) => Err(format!("a row '{id}' already exists")),
        _ => Ok(row),
    }
}

/// 🩹 Resolves a remedy into its [`RemedyEdit`]: [`RemedyBound::Remove`] selects the row of the target path (it must exist), [`RemedyBound::Insert`] appends its typed row to the list of the target path, every other bound writes a value like [`remedy_edit`].
pub fn remedy_resolution(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &semio_framework_value::DslValue) -> Result<RemedyEdit, Fault> {
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
        RemedyBound::Remove => {
            if !matches!(parse_path(&path).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))?.last(), Some(PathSegment::Index(_) | PathSegment::Id(_))) {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-remove-path"), format!("remove remedy {remedy_index} on '{check_id}' must target a list row, not '{path}'")));
            }
            return Ok(RemedyEdit::Remove { path });
        }
        RemedyBound::Insert => {
            let rows = current.as_array().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-insert-path"), format!("insert remedy {remedy_index} on '{check_id}' must target a list, not '{path}'")))?;
            let value = remedy_row(remedy, rows).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-insert-row"), format!("insert remedy {remedy_index} on '{check_id}': {error}")))?;
            return Ok(RemedyEdit::Insert { path, index: rows.len(), value });
        }
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
        RemedyBound::AtLeast | RemedyBound::AtMost | RemedyBound::Exactly => match current {
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
    Ok(RemedyEdit::Set { path, value })
}

/// 🩹 Writes a resolved remedy into a value tree — the remedy laws' perturbation helper; the mutation path resolves the same edit
/// to a concrete kind instead.
pub fn apply_remedy_edit(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &mut semio_framework_value::DslValue) -> Result<(), Fault> {
    let outcome = match remedy_resolution(report, check_id, remedy_index, option_index, tree)? {
        RemedyEdit::Set { path, value } => set_value_at_path(tree, &path, value),
        RemedyEdit::Remove { path } => remove_value_at_path(tree, &path, 0),
        RemedyEdit::Insert { path, index, value } => insert_value_at_path(tree, &path, index, Some(value)),
    };
    outcome.map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))
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
    let report = F::live_report(document);
    let tree = semio_framework_value::ToValue::to_value(document);
    let edit = match remedy_resolution(&report, check_id, remedy_index, option_index, &tree)? {
        RemedyEdit::Set { path, value } => NormEdit::SetField { path, value },
        RemedyEdit::Remove { path } => NormEdit::RemoveItem { path, index: 0 },
        RemedyEdit::Insert { path, index, value } => NormEdit::InsertItem { path, index, value: Some(value) },
    };
    commit_snapshot_fields(resolve(document, &edit).map_err(unresolved)?)
}
//#endregion 🔖️ValuePath

//#region 🫧️LiveEdits
/// 🧱️ Validates a write bundle: non-empty, within [`NORM_LIVE_MAX_WRITES`], no path twice, every number finite.
pub fn validate_writes(writes: &[FieldWrite]) -> Result<(), String> {
    if writes.is_empty() {
        return Err("setFields needs at least one write".into());
    }
    if writes.len() > NORM_LIVE_MAX_WRITES {
        return Err(format!("setFields carries {} writes, at most {NORM_LIVE_MAX_WRITES} are allowed", writes.len()));
    }
    for (at, write) in writes.iter().enumerate() {
        if writes[..at].iter().any(|earlier| earlier.path == write.path) {
            return Err(format!("path '{}' is written twice", write.path));
        }
        if matches!(&write.value, semio_framework_value::DslValue::Number(number) if !number.as_f64().is_finite()) {
            return Err(format!("write '{}' is not finite", write.path));
        }
    }
    Ok(())
}

/// 🔢️ Carries a numeric write over to the leaf's own number kind (counts stay integers); other kinds pass through.
pub fn coerce_like(current: &semio_framework_value::DslValue, value: &semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    use semio_framework_value::{DslValue, Number};
    match (current, value) {
        (DslValue::Number(Number::UInt(_)), DslValue::Number(number)) if number.as_f64() >= 0.0 => DslValue::uint(number.as_f64().round() as u64),
        (DslValue::Number(Number::Int(_)), DslValue::Number(number)) => DslValue::int(number.as_f64().round() as i64),
        (DslValue::Number(Number::Float(_)), DslValue::Number(number)) => DslValue::float(number.as_f64()),
        _ => value.clone(),
    }
}

/// 📦️ Resolves a validated write bundle into the concrete mutations of all its leaves, all-or-nothing: the first failing write fails the bundle.
pub fn resolve_set_fields<D, M, F>(document: &D, writes: &[FieldWrite], mut resolve: F) -> Result<Vec<M>, String>
where
    D: semio_framework_value::ToValue,
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    validate_writes(writes)?;
    let tree = semio_framework_value::ToValue::to_value(document);
    let mut mutations = Vec::new();
    for write in writes.iter().filter(|write| !is_view_write(write)) {
        let value = get_value_at_path(&tree, &write.path).map_or_else(|_| write.value.clone(), |current| coerce_like(current, &write.value));
        mutations.extend(resolve(document, &NormEdit::SetField { path: write.path.clone(), value })?);
    }
    Ok(mutations)
}

/// 📦️ Generic `setFields` — `{writes: [{path, value}]}` becomes ONE emit of all leaf mutations, i.e. one undo step for a drag release.
pub fn handle_set_fields<D, M, F>(document: &D, writes: &[FieldWrite], resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: semio_framework_value::ToValue,
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve_set_fields(document, writes, resolve).map_err(unresolved)?)
}

/// 👻️ `previewRemedy` — the ghost a remedy would write, resolved exactly like `applyRemedy` but kept in the transient lane.
pub fn preview_remedy(report: &CheckReport, tree: &semio_framework_value::DslValue, check_id: &str, remedy_index: usize, option_index: usize) -> Result<GhostPreview, Fault> {
    let (path, value) = remedy_edit(report, check_id, remedy_index, option_index, tree)?;
    Ok(GhostPreview { writes: vec![FieldWrite { path, value }], source: GhostSource::Remedy { check_id: check_id.to_string(), index: remedy_index as u32 } })
}
//#endregion 🫧️LiveEdits

//#region 🔖️RemedyLaw
fn remedy_severity(status: &CheckStatus) -> u8 {
    match status {
        CheckStatus::Fail => 2,
        CheckStatus::Warning => 1,
        _ => 0,
    }
}

/// ⚖️ The remedy law over one document: every remedy of every check of its report must (1) carry EN and DE text, (2) be an advisory (`applicable: false`, no options) or name an existing field, numeric for `AtLeast` / `AtMost` / `Exactly` and with at least one option for `OneOf`, (3) apply without a fault and yield a document that decodes (so an enum token, a count and a text field stay valid), (4) write a finite value inside the field's hard range (`limits(path)`, the field meta range; the schema types are enforced by the decode) and (5) lower the check's utilisation or its severity when the report is evaluated again (a `Remove` and the fix of an invalid-input check, id with `.input.`, may drop the check). Returns one finding per violation, empty when the law holds.
pub fn remedy_law_findings<S>(label: &str, document: &S, evaluate: impl Fn(&S) -> CheckReport, limits: impl Fn(&str) -> Option<(f64, f64)>) -> Vec<String>
where
    S: semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    let mut findings = Vec::new();
    let report = evaluate(document);
    let tree = semio_framework_value::ToValue::to_value(document);
    for check in &report.checks {
        for (index, remedy) in check.remedies.iter().enumerate() {
            let at = format!("{label}: {} remedy {index} on '{}'", check.id, remedy.target.path);
            if remedy.action.en.trim().is_empty() || remedy.action.de.trim().is_empty() {
                findings.push(format!("{at}: the action text lacks EN or DE"));
            }
            let advisory = remedy.bound == RemedyBound::OneOf && remedy.options.is_empty();
            if !remedy.applicable {
                if !advisory {
                    findings.push(format!("{at}: a remedy that is not applicable must be an advisory (OneOf without options)"));
                }
                continue;
            }
            if advisory {
                findings.push(format!("{at}: an applicable OneOf remedy has no options (apply fails with 'option 0 missing')"));
                continue;
            }
            match get_value_at_path(&tree, &remedy.target.path) {
                Err(error) => {
                    findings.push(format!("{at}: the target does not exist: {error}"));
                    continue;
                }
                Ok(current) => {
                    let numeric = matches!(current, semio_framework_value::DslValue::Number(_) | semio_framework_value::DslValue::Bool(_));
                    if matches!(remedy.bound, RemedyBound::AtLeast | RemedyBound::AtMost | RemedyBound::Exactly) && !(numeric && remedy.required.value.is_finite()) {
                        findings.push(format!("{at}: a numeric remedy ({}) targets a non-numeric field or carries a non-finite value", remedy.required.value));
                        continue;
                    }
                }
            }
            let options = if remedy.bound == RemedyBound::OneOf { remedy.options.len() } else { 1 };
            for option in 0..options {
                let mut changed = tree.clone();
                if let Err(fault) = apply_remedy_edit(&report, &check.id, index, option, &mut changed) {
                    findings.push(format!("{at}: applying option {option} fails: {fault:?}"));
                    continue;
                }
                if let (Some((min, max)), Ok(semio_framework_value::DslValue::Number(written))) = (limits(&remedy.target.path), get_value_at_path(&changed, &remedy.target.path)) {
                    let value = written.as_f64();
                    if !(value >= min && value <= max) {
                        findings.push(format!("{at}: the written value {value} lies outside the field range {min} to {max}"));
                    }
                }
                let fixed = match <S as semio_framework_value::FromValue>::from_value(changed) {
                    Ok(fixed) => fixed,
                    Err(error) => {
                        findings.push(format!("{at}: the document is invalid after applying option {option}: {error:?}"));
                        continue;
                    }
                };
                match evaluate(&fixed).checks.iter().find(|updated| updated.id == check.id) {
                    None if remedy.bound == RemedyBound::Remove || check.id.contains(".input.") => {}
                    None => findings.push(format!("{at}: the check vanished after applying option {option}")),
                    Some(updated) => {
                        let lowered = remedy_severity(&updated.status) < remedy_severity(&check.status) || updated.utilization < check.utilization;
                        if !lowered {
                            findings.push(format!("{at}: option {option} does not help: {:?} u {:.4} -> {:?} u {:.4}", check.status, check.utilization, updated.status, updated.utilization));
                        }
                    }
                }
            }
        }
    }
    findings
}
//#endregion 🔖️RemedyLaw
