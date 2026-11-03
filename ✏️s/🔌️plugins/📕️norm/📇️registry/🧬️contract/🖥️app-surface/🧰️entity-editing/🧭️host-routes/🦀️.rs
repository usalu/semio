//! 🧭️ Host routes: the one table that tells a host what a shortcut chord or a long-press menu entry does: a history step of the store (undo, redo), a view step, an entity command or a selection change. Mounted in the panels crate, below the editor, because the shell hook routes keys through it.
//!
//! 🔗️ Seam: `semio_framework::PluginShellHooks::handle_key`; the entity commands: `copyEntities`, `duplicateEntities`, `pasteEntities`, `deleteEntities`.

use crate::*;
use crate::accessibility::focus_order::{shortcut_action_id, shortcuts, Region, ShortcutAction};
use crate::viewport3d::picking::{entity_address, granularity};
use semio_framework::ShellKeyOutcome;
use semio_framework_value::DslValue;
use semio_s_plugin_norm_structural_scene::LocalizedCopy;
use std::sync::{Mutex, PoisonError};

/// 📜️ The step of the document history a key asks for; the event-sourced inverse lives in the store (`ArtifactCommand::Undo` and `Redo`), the editor only names the step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryStep {
    Undo,
    Redo,
}

/// ↩️ The history step of a shortcut action, `None` for every other action.
pub fn history_step(action: &ShortcutAction) -> Option<HistoryStep> {
    match route_of(action) {
        HostRoute::History(step) => Some(step),
        HostRoute::ViewAct(_) | HostRoute::Entity(_) | HostRoute::SelectAll | HostRoute::FocusWindow(_) | HostRoute::Local => None,
    }
}

/// 🧭️ Where a shortcut action goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostRoute {
    History(HistoryStep),
    ViewAct(&'static str),
    Entity(&'static str),
    SelectAll,
    FocusWindow(Region),
    Local,
}

fn route_of(action: &ShortcutAction) -> HostRoute {
    match action {
        ShortcutAction::Undo => HostRoute::History(HistoryStep::Undo),
        ShortcutAction::Redo => HostRoute::History(HistoryStep::Redo),
        ShortcutAction::Copy => HostRoute::Entity("copyEntities"),
        ShortcutAction::Duplicate => HostRoute::Entity("duplicateEntities"),
        ShortcutAction::Paste => HostRoute::Entity("pasteEntities"),
        ShortcutAction::Delete => HostRoute::Entity("deleteEntities"),
        ShortcutAction::SelectAll => HostRoute::SelectAll,
        ShortcutAction::NextCheck | ShortcutAction::PreviousCheck | ShortcutAction::NextGoverningCheck | ShortcutAction::PreviousGoverningCheck | ShortcutAction::NextCombination | ShortcutAction::PreviousCombination => {
            crate::shortcut_view_act(&shortcut_action_id(action)).map_or(HostRoute::Local, |(act, _)| HostRoute::ViewAct(act))
        }
        ShortcutAction::Focus(region) => HostRoute::FocusWindow(*region),
        ShortcutAction::CycleRegion { .. }
        | ShortcutAction::CycleSelection { .. }
        | ShortcutAction::EnterHandles
        | ShortcutAction::OrbitLeft
        | ShortcutAction::OrbitRight
        | ShortcutAction::OrbitUp
        | ShortcutAction::OrbitDown
        | ShortcutAction::PanLeft
        | ShortcutAction::PanRight
        | ShortcutAction::PanUp
        | ShortcutAction::PanDown
        | ShortcutAction::ZoomIn
        | ShortcutAction::ZoomOut
        | ShortcutAction::ResetView
        | ShortcutAction::NudgeHandle { .. }
        | ShortcutAction::CycleHandle { .. }
        | ShortcutAction::CycleGripInEntity { .. }
        | ShortcutAction::ConfirmHandle
        | ShortcutAction::CancelHandle
        | ShortcutAction::ToggleSceneAsList
        | ShortcutAction::SearchCatalogue
        | ShortcutAction::ClearSelection
        | ShortcutAction::Help => HostRoute::Local,
    }
}

/// 🧭️ The route of a shortcut action id (`norm.shortcut.undo`): undo and redo belong to the store (the shell's reserved edit chords run `ArtifactCommand::Undo` and `Redo`), the stepping keys go to `viewportAct`, copy, duplicate and paste to the entity tools, the focus keys to the framework `focusWindow` verb, everything else stays with the focused window.
pub fn shortcut_route(action_id: &str) -> HostRoute {
    shortcuts().into_iter().find(|shortcut| shortcut_action_id(&shortcut.action) == action_id).map_or(HostRoute::Local, |shortcut| route_of(&shortcut.action))
}

//#region 🧭️HostContext
/// 🧭️ What a key press needs and the shell does not pass along: the selected rows of the entity collection and the clip of the last copy. Ephemeral and local to this user; the windows publish the selection when they render, the copy command stores the clip.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HostContext {
    pub collection: String,
    pub ids: Vec<String>,
    pub clip_json: Option<String>,
}

static HOST_CONTEXT: Mutex<HostContext> = Mutex::new(HostContext { collection: String::new(), ids: Vec::new(), clip_json: None });

/// 🧭️ The collection of the document that holds the selected rows, and their ids: `selected` carries ids and element paths (`walls[id=w1]`) as the outliner state does; the collection with the most selected rows wins, and a tie never goes to the placement rows (their ids equal the entity they move).
pub fn selected_rows(tree: &DslValue, selected: &[String]) -> Option<(String, Vec<String>)> {
    let DslValue::Object(fields) = tree else { return None };
    fields
        .iter()
        .filter_map(|(name, value)| {
            let rows = value.as_array()?;
            let ids: Vec<String> = rows
                .iter()
                .enumerate()
                .filter(|(index, row)| object_id_field(row).is_some_and(|id| selected.iter().any(|candidate| candidate == id || *candidate == list_element_path(name, row, *index))))
                .filter_map(|(_, row)| object_id_field(row).map(str::to_owned))
                .collect();
            (!ids.is_empty()).then(|| (name.clone(), ids))
        })
        .max_by_key(|(name, ids)| (ids.len(), name.as_str() != semio_s_plugin_norm_structural_scene::projection_kit::PLACEMENTS_FIELD))
}

/// 📣 Publishes the selection of the document the key handler acts on; an empty selection clears it. The clip stays.
pub fn publish_host_selection(tree: &DslValue, selected: &[String]) {
    let (collection, ids) = selected_rows(tree, selected).unwrap_or_default();
    let mut context = HOST_CONTEXT.lock().unwrap_or_else(PoisonError::into_inner);
    context.collection = collection;
    context.ids = ids;
}

/// 📋️ Keeps the clip of a copy for the next paste.
pub fn remember_clip(clip_json: String) {
    HOST_CONTEXT.lock().unwrap_or_else(PoisonError::into_inner).clip_json = Some(clip_json);
}

/// 🧭️ The published context.
pub fn host_context() -> HostContext {
    HOST_CONTEXT.lock().unwrap_or_else(PoisonError::into_inner).clone()
}

static HOST_FOCUS: Mutex<Option<usize>> = Mutex::new(None);

/// 📣 Publishes the check the Edit menu focuses: the index of the governing check of the one selected entity (`None` for no or several selected rows, or an entity no check covers), computed by the inspector, which holds the report.
pub fn publish_host_focus(check: Option<usize>) {
    *HOST_FOCUS.lock().unwrap_or_else(PoisonError::into_inner) = check;
}

/// 🎯️ The check the Edit menu focuses, as last published.
pub fn host_focus() -> Option<usize> {
    *HOST_FOCUS.lock().unwrap_or_else(PoisonError::into_inner)
}

/// 🧭️ The entity tool call a chord or a menu entry stands for against `context`: the tool id and its arguments, exactly what the tool accepts when called with arguments (`copyEntities`, `duplicateEntities` and `deleteEntities` with the selected collection and ids, `pasteEntities` with the clip). `None` for every action that is no entity route; an error when the context has nothing to act on.
pub fn host_entity_call(action_id: &str, context: &HostContext) -> Option<Result<(&'static str, DslValue), String>> {
    let HostRoute::Entity(tool) = shortcut_route(action_id) else { return None };
    Some(match tool {
        "pasteEntities" => context
            .clip_json
            .as_deref()
            .ok_or_else(|| "nothing was copied".to_owned())
            .and_then(|json| semio_framework_pack_json::from_json_str::<DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()))
            .map(|clip| (tool, DslValue::object(vec![("clip".to_owned(), clip)]))),
        _ if context.ids.is_empty() => Err("nothing is selected".to_owned()),
        _ => Ok((tool, DslValue::object(vec![("collection".to_owned(), DslValue::String(context.collection.clone())), ("ids".to_owned(), DslValue::Array(context.ids.iter().cloned().map(DslValue::String).collect()))]))),
    })
}
//#endregion 🧭️HostContext

//#region 🎯️SelectAll
static VISIBLE: Mutex<Vec<(String, Option<Vec<String>>)>> = Mutex::new(Vec::new());

fn with_visible<R>(domain: &str, write: impl FnOnce(&mut Option<Vec<String>>) -> R) -> R {
    let mut store = VISIBLE.lock().unwrap_or_else(PoisonError::into_inner);
    let index = store.iter().position(|(slot, _)| slot == domain).unwrap_or_else(|| {
        store.push((domain.to_owned(), None));
        store.len() - 1
    });
    write(&mut store[index].1)
}

/// 📣 Publishes which rows of `tree` the outliner filter lets through, so the interaction topology (and with it the framework `selectAll` verb) respects it; an empty filter admits every entity (`None`).
pub fn publish_visible_rows(domain: &str, tree: &DslValue, filter: &str) {
    let visible = (!filter.trim().is_empty()).then(|| match tree {
        DslValue::Object(fields) => fields.iter().filter_map(|(name, value)| value.as_array().map(|rows| (name, rows))).flat_map(|(name, rows)| collection_rows(name, rows, filter)).map(|row| row.id).filter(|id| !id.is_empty()).collect::<Vec<String>>(),
        _ => Vec::new(),
    });
    with_visible(domain, |slot| *slot = visible);
}

/// 🔎️ The ids of the rows the outliner filter of `domain` admits, or `None` when no filter narrows the list.
pub fn visible_rows(domain: &str) -> Option<Vec<String>> {
    with_visible(domain, |slot| slot.clone())
}

/// 🎯️ The granularities Select all selects together: every entity kind of a scene (checks and load cases are lists, not scene entities).
pub fn select_all_granularities() -> String {
    use semio_s_plugin_norm_structural_scene::EntityKind::*;
    [Member, Plate, Shell, Solid, Fastener, Support, Load, Surface, Group].map(crate::viewport3d::picking::granularity).join(",")
}

/// 🎯️ The arguments of the framework `selectAll` verb for the norm interaction domain: all entity kinds at once.
pub fn select_all_args() -> Vec<(String, String)> {
    vec![("granularities".to_owned(), select_all_granularities())]
}

/// 🎯️ What the key handler answers to Select all in the window `scope` (`norm-en1992-viewport`): the framework `selectAll` verb with the norm scope, the one path the touch menu and the palette take too; `Ignored` when the window has no interaction domain.
pub fn select_all_outcome(scope: &str) -> ShellKeyOutcome {
    let has_domain = crate::accessibility::published_nodes::window_scope(scope).is_some_and(|(_, domain)| !domain.is_empty());
    if has_domain { ShellKeyOutcome::verb(semio_framework::SELECT_ALL_ACTION_ID, select_all_args()) } else { ShellKeyOutcome::Ignored }
}
//#endregion 🎯️SelectAll

/// 👆️ One entry of the long-press menu of the viewport on touch devices. `check` marks the focus entry: the index of the check a tap selects (`setSelectedCheckIndex`), the same row the context menu offers.
#[derive(Clone, Debug, PartialEq)]
pub struct TouchMenuEntry {
    pub action_id: String,
    pub label: LocalizedCopy,
    pub enabled: bool,
    pub check: Option<usize>,
}

/// 🎯️ The id of the focus-check entry of the Edit menu (the context menu's `norm.context.focus-check`).
pub const TOUCH_FOCUS_CHECK_ID: &str = "norm.touch.focus-check";

/// 👆️ The long-press menu, the touch twin of the context menu: Focus check first (enabled when `focus` names the governing check of the one selected entity), then Copy, Duplicate and Delete (they need a selection), Paste (needs a clip) and Select all (always possible); the action ids are the shortcut action ids, so a tap routes exactly like the chord.
pub fn touch_menu(selected: usize, has_clip: bool, focus: Option<usize>) -> Vec<TouchMenuEntry> {
    let entry = |action: ShortcutAction, en: &str, de: &str, enabled: bool| TouchMenuEntry { action_id: shortcut_action_id(&action), label: LocalizedCopy::new(en, de), enabled, check: None };
    let focus_check = TouchMenuEntry { action_id: TOUCH_FOCUS_CHECK_ID.to_owned(), label: LocalizedCopy::new("Focus check", "Prüfung fokussieren"), enabled: focus.is_some(), check: focus };
    vec![
        focus_check,
        entry(ShortcutAction::Copy, "Copy", "Kopieren", selected > 0),
        entry(ShortcutAction::Paste, "Paste", "Einfügen", has_clip),
        entry(ShortcutAction::Duplicate, "Duplicate", "Duplizieren", selected > 0),
        entry(ShortcutAction::Delete, "Delete", "Löschen", selected > 0),
        entry(ShortcutAction::SelectAll, "Select all", "Alle auswählen", true),
    ]
}
