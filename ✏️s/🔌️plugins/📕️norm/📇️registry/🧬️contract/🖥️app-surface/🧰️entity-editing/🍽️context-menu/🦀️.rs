//! 🍽️ Context menu: the menu a secondary click, the context-menu key or `Shift+F10` opens on a 3D pick, an outliner row or a checks row. The shell finds the trigger (domain-neutral, in the input layer) and asks the app for the rows; this module answers with the entity commands of the shortcuts (`duplicateEntities`, `copyEntities`, `pasteEntities`, `deleteEntities`), the bulk edit of a multi-selection, select all and focus check. Pure: the document tree, what was hit and the published context in, [`ContextMenuItemSpec`] rows in EN or DE out.
//!
//! 🔗️ Seam: `ArtifactEditor::context_menu`; the rows dispatch the very tools the chords and the long-press menu dispatch ([`host_entity_call`]), so no second route exists.

use crate::*;
use crate::accessibility::focus_order::{shortcut_action_id, Region, ShortcutAction};
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_value::DslValue;

/// 🌳️ The hit domain the shell gives a row of any tree; twin of the framework constant `CONTEXT_MENU_TREE_HIT_DOMAIN`.
pub const TREE_HIT_DOMAIN: &str = "tree";

const OUTLINER_ROW: &str = "norm-outliner.";
const CHECK_ROW: &str = "norm-checks-check-";

/// 🎯️ What the secondary click or the key landed on: the hits as `(domain, id)`, the ids of the interaction selection and the id of the window instance, all as the shell reports them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MenuProbe {
    pub hits: Vec<(String, String)>,
    pub selected: Vec<String>,
    pub window: String,
}

/// 🎯️ The probe of a framework context-menu request: the hits of the surface and the ids of every selection group.
pub fn request_probe(request: &semio_framework_plugin::ContextMenuRequest) -> MenuProbe {
    let window = request.window_instance_id.clone().unwrap_or_default();
    let Some(surface) = request.surface.as_ref() else { return MenuProbe { window, ..MenuProbe::default() } };
    MenuProbe { hits: surface.hits.iter().map(|hit| (hit.domain.clone(), hit.id.clone())).collect(), selected: surface.selection.iter().flat_map(|group| group.ids.iter().cloned()).collect(), window }
}

fn digits_prefix(text: &str) -> Option<usize> {
    let end = text.find(|c: char| !c.is_ascii_digit()).unwrap_or(text.len());
    text[..end].parse().ok()
}

/// ✅️ The index of the check a checks-window row id names (`norm-checks-check-3` and its detail rows `norm-checks-check-3.status`).
pub fn check_row_index(id: &str) -> Option<usize> {
    id.split_once(CHECK_ROW).and_then(|(_, rest)| digits_prefix(rest))
}

fn outliner_row_path(id: &str) -> Option<&str> {
    id.split_once(OUTLINER_ROW).and_then(|(_, rest)| rest.strip_suffix(".select"))
}

fn candidates(domain: &str, id: &str) -> Vec<String> {
    if domain == TREE_HIT_DOMAIN {
        return outliner_row_path(id).map(str::to_owned).into_iter().collect();
    }
    let decoded = crate::protocol::wire::decode_scoped_identity(id).ok().and_then(|segments| segments.last().map(|last| (*last).to_owned()));
    std::iter::once(id.to_owned()).chain(decoded).collect()
}

/// 🎯️ The rows the menu acts on: the rows hit, widened to the whole selection when the hit row is part of it (a secondary click inside a selection keeps the selection), else the selection of the context, else the interaction selection of the surface.
pub fn menu_target(tree: &DslValue, probe: &MenuProbe, context: &HostContext) -> Option<(String, Vec<String>)> {
    let hit = probe.hits.iter().find_map(|(domain, id)| selected_rows(tree, &candidates(domain, id)));
    match hit {
        Some((collection, ids)) if collection == context.collection && ids.iter().all(|id| context.ids.contains(id)) => Some((collection, context.ids.clone())),
        Some(rows) => Some(rows),
        None if !context.ids.is_empty() => Some((context.collection.clone(), context.ids.clone())),
        None => selected_rows(tree, &probe.selected),
    }
}

fn pick(german: bool, en: &str, de: &str) -> String {
    (if german { de } else { en }).to_owned()
}

fn row(id: &str, label: String, shortcut: Option<&str>) -> ContextMenuItemSpec {
    ContextMenuItemSpec { id: format!("norm.context.{id}"), label: Some(label), shortcut: shortcut.map(str::to_owned), ..ContextMenuItemSpec::default() }
}

fn separator(id: &str) -> ContextMenuItemSpec {
    ContextMenuItemSpec { id: format!("norm.context.{id}"), separator: Some(true), ..ContextMenuItemSpec::default() }
}

fn runs(mut spec: ContextMenuItemSpec, action: &str, args: DslValue) -> ContextMenuItemSpec {
    spec.action = Some(action.to_owned());
    spec.args = Some(args);
    spec
}

fn entity_row(spec: ContextMenuItemSpec, action: ShortcutAction, context: &HostContext, reason: String) -> ContextMenuItemSpec {
    match host_entity_call(&shortcut_action_id(&action), context) {
        Some(Ok((tool, args))) => runs(spec, tool, args),
        _ => spec.disabled_because(reason),
    }
}

fn focus_row(german: bool, index: Option<usize>) -> ContextMenuItemSpec {
    let spec = row("focus-check", pick(german, "Focus check", "Prüfung fokussieren"), None);
    match index {
        Some(index) => runs(spec, "setSelectedCheckIndex", DslValue::object(vec![("index".to_owned(), DslValue::uint(index as u64))])),
        None => spec.disabled_because(pick(german, "No check covers this entity", "Keine Prüfung betrifft diese Entität")),
    }
}

/// 🗃️ The bulk edit row of a multi-selection: it raises the framework `focusWindow` verb for the Properties window kind (opened when closed, raised, focused, announced), which shows the shared fields of exactly this selection (a field the rows disagree on reads Mixed, a write goes to every selected row as one `setFields` bundle); the label counts the selected rows and the fields they share, and a selection without a shared editable field says so.
fn bulk_edit_row(tree: &DslValue, collection: &str, ids: &[String], window: &str, german: bool) -> ContextMenuItemSpec {
    let lines = bulk_rows(tree, collection, ids, german);
    let mixed = lines.iter().filter(|line| line.mixed).count();
    let label = if german { format!("Gemeinsam bearbeiten ({} ausgewählt, {} Felder, {} gemischt)", ids.len(), lines.len(), mixed) } else { format!("Bulk edit ({} selected, {} fields, {} mixed)", ids.len(), lines.len(), mixed) };
    let spec = row("bulk-edit", label, Some("alt+3"));
    if lines.is_empty() {
        return spec.disabled_because(pick(german, "The selected entities share no editable field", "Die ausgewählten Entitäten teilen kein bearbeitbares Feld"));
    }
    match crate::window_kind_of_region(window, Region::Properties) {
        Some(kind) => runs(spec, semio_framework::FOCUS_WINDOW_ACTION_ID, DslValue::object(vec![(semio_framework::FOCUS_WINDOW_ARG_KIND.to_owned(), DslValue::String(kind))])),
        None => spec.disabled_because(pick(german, "The Properties window is not available here", "Das Eigenschaften-Fenster ist hier nicht verfügbar")),
    }
}

/// 🍽️ The rows of the context menu. `focus` answers the index of the governing check of an entity id; `german` picks the language (no default language: the caller states it). Over empty space outside the viewport the app has nothing to add and answers no rows, so the shell offers the window's own actions. Order: focus check, then duplicate, copy, paste, delete, bulk edit (2+ rows), and select all last; rows that cannot run stay in place, disabled with the reason announced.
pub fn context_menu_items(tree: &DslValue, probe: &MenuProbe, context: &HostContext, focus: &dyn Fn(&str) -> Option<usize>, german: bool) -> Vec<ContextMenuItemSpec> {
    let target = menu_target(tree, probe, context);
    let check_hit = probe.hits.iter().find_map(|(_, id)| check_row_index(id));
    if target.is_none() && check_hit.is_none() && !probe.window.contains("viewport") {
        return Vec::new();
    }
    let mut items = Vec::new();
    match (&target, check_hit) {
        (_, Some(index)) => items.push(focus_row(german, Some(index))),
        (Some((_, ids)), None) if ids.len() == 1 => items.push(focus_row(german, focus(ids[0].as_str()))),
        _ => {}
    }
    if !items.is_empty() {
        items.push(separator("separator-focus"));
    }
    let (collection, ids) = target.clone().unwrap_or_default();
    let acting = HostContext { collection: collection.clone(), ids: ids.clone(), clip_json: context.clip_json.clone() };
    let nothing = pick(german, "Select an entity first", "Zuerst eine Entität auswählen");
    items.push(entity_row(row("duplicate", pick(german, "Duplicate", "Duplizieren"), Some("mod+d")), ShortcutAction::Duplicate, &acting, nothing.clone()));
    items.push(entity_row(row("copy", pick(german, "Copy", "Kopieren"), Some("mod+c")), ShortcutAction::Copy, &acting, nothing.clone()));
    items.push(entity_row(row("paste", pick(german, "Paste", "Einfügen"), Some("mod+v")), ShortcutAction::Paste, &acting, pick(german, "Nothing was copied", "Es wurde nichts kopiert")));
    let mut delete = row("delete", pick(german, "Delete", "Löschen"), Some("Delete"));
    delete.destructive = Some(true);
    items.push(entity_row(delete, ShortcutAction::Delete, &acting, nothing));
    if ids.len() > 1 {
        items.push(bulk_edit_row(tree, &collection, &ids, &probe.window, german));
    }
    items.push(separator("separator-select"));
    items.push(runs(row("select-all", pick(german, "Select all", "Alle auswählen"), Some("mod+a")), semio_framework::SELECT_ALL_ACTION_ID, DslValue::object(select_all_args().into_iter().map(|(key, value)| (key, DslValue::String(value))))));
    items
}
