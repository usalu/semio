//! 🧭️ Norm edit vocabulary, typed mutation boundaries and selector-aware payload resolution.

extern crate semio_os_kernel as protocol;
use semio_s_plugin_norm_app_surface_foundation::{get_value_at_path, object_id_field, parse_path, resolve_list_index, set_value_at_path, PathSegment};

#[path = "🎒️payload-arguments/🦀️.rs"]
mod payload_arguments;
pub use payload_arguments::{SetFieldArgument, SetFieldArgumentSource};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

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
    pub set_field_arguments: &'static [SetFieldArgument],
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

fn segment_path(segments: &[PathSegment]) -> String {
    let mut path = String::new();
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                if !path.is_empty() { path.push('.'); }
                path.push_str(name);
            }
            PathSegment::Index(index) => path.push_str(&format!("[{index}]")),
            PathSegment::Id(id) => path.push_str(&format!("[id={id}]")),
        }
    }
    path
}

/// 🧭️ A value setter's concrete typed boundary and its descendant path.
#[derive(Clone, Debug)]
pub struct SetterBinding<'a> {
    pub rule: &'a SetFieldRule,
    pub boundary: String,
    pub relative: String,
}

fn segment_template(segments: &[PathSegment]) -> String {
    let mut template = String::new();
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                if !template.is_empty() { template.push('.'); }
                template.push_str(name);
            }
            PathSegment::Index(_) | PathSegment::Id(_) => template.push_str("[]"),
        }
    }
    template
}

impl NormEditRules {
    /// 📥️ The exact insertion rule of a selector-normalized collection.
    pub fn insertion_binding(&self, path: &str) -> Option<&InsertItemRule> {
        let segments = parse_path(path).ok()?;
        let template = segment_template(&segments);
        self.insert_item.iter().find(|rule| rule.path == template)
    }

    /// ➕️ Whether a collection declares its own insertion mutation.
    pub fn can_insert_item(&self, path: &str) -> bool {
        self.insertion_binding(path).is_some()
    }

    /// ➖️ Whether a collection or selected row declares positional or explicit identity removal.
    pub fn can_remove_item<'a>(&self, path: &str, id_paths: impl IntoIterator<Item = &'a str>) -> bool {
        let Ok(mut segments) = parse_path(path) else { return false };
        if matches!(segments.last(), Some(PathSegment::Index(_) | PathSegment::Id(_))) { segments.pop(); }
        let template = segment_template(&segments);
        self.remove_item.iter().any(|rule| rule.path == template) || id_paths.into_iter().any(|path| path == template)
    }

    /// 🪆️ The nearest non-clear typed setter, preserving the concrete selector boundary.
    pub fn setter_binding(&self, path: &str) -> Option<SetterBinding<'_>> {
        let segments = parse_path(path).ok()?;
        for end in (1..=segments.len()).rev() {
            let template = segment_template(&segments[..end]);
            if let Some(rule) = self.set_field.iter().find(|rule| !rule.value.is_empty() && rule.path == template) {
                return Some(SetterBinding { rule, boundary: segment_path(&segments[..end]), relative: segment_path(&segments[end..]) });
            }
        }
        None
    }


    /// 🖊️ Whether an existing field lies within a declared typed setter; document addressing and value admission remain authoritative at commit.
    pub fn can_set_field(&self, path: &str) -> bool {
        let Ok(segments) = parse_path(path) else { return false };
        let mut template = String::new();
        for segment in segments {
            match segment {
                PathSegment::Field(name) => {
                    if !template.is_empty() { template.push('.'); }
                    template.push_str(&name);
                }
                PathSegment::Index(_) | PathSegment::Id(_) => template.push_str("[]"),
            }
        }
        self.set_field.iter().any(|rule| !rule.value.is_empty() && template.strip_prefix(rule.path).is_some_and(|tail| tail.is_empty() || tail.starts_with('.') || tail.starts_with("[]")))
    }

    /// 🧹 The rule that sets the leaf at `template`: a rule with an empty `value` is the leaf's clear action (its kind carries no value) and answers a `null` write, every other rule answers a value write.
    fn set_rule(&self, template: &str, clearing: bool) -> Option<&SetFieldRule> {
        let at = || self.set_field.iter().filter(move |rule| rule.path == template);
        at().find(|rule| rule.value.is_empty() == clearing).or_else(|| at().find(|rule| !rule.value.is_empty()))
    }

    fn parent_set_payload(&self, tree: &semio_framework_value::DslValue, segments: &[PathSegment], path: &str, value: &semio_framework_value::DslValue) -> Result<Option<EditPayload>, String> {
        for end in (1..segments.len()).rev() {
            let addressed = address(tree, &segments[..end], path)?;
            let Some(rule) = self.set_rule(&addressed.template, false) else { continue };
            let mut replacement = get_value_at_path(tree, &segment_path(&segments[..end]))?.clone();
            set_value_at_path(&mut replacement, &segment_path(&segments[end..]), value.clone())?;
            let mut entries = selector_entries(rule.selectors, &addressed.selected, path)?;
            entries.push((rule.value.to_string(), replacement));
            self.append_set_field_arguments(rule, tree, &segments[..end], path, &mut entries)?;
            return Ok(Some((rule.kind, entries)));
        }
        Ok(None)
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
                        let Some(rule) = self.set_rule(&addressed.template, clearing) else {
                            return self.parent_set_payload(tree, &segments, path, value)?.ok_or_else(|| format!("no kind sets '{}' ({path})", addressed.template));
                        };
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
                self.append_set_field_arguments(rule, tree, &segments, path, &mut entries)?;
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

//#endregion 🔖️ValuePath
