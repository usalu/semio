//! 🖐️ Touch menu window: the long-press menu of the viewport as a tree group (one item per [`TouchMenuEntry`]) and its accessibility nodes. A tap raises the shortcut action of the entry, so it routes exactly like the chord; Select all raises the framework `selectAll` verb over the topology the app publishes; Focus check selects the governing check of the selected entity like the context menu row. The menu closes on an outside tap, Escape or any command (see [`touch_menu_open`]). Mounted in the panels crate next to the host routes.
//!
//! 🔗️ Entries: [`touch_menu`]; routes: [`shortcut_route`]; nodes: [`PublishedNode`](crate::accessibility::published_nodes::PublishedNode).

use crate::*;
use crate::accessibility::published_nodes::{publish, PublishedNode};
use crate::accessibility::screen_reader::AriaNode;
use crate::viewport3d::config::Lang;
use crate::accessibility::focus_order::{shortcut_action_id, ShortcutAction};
use std::sync::{Mutex, PoisonError};

/// 🖐️ The window role the touch menu publishes its accessibility nodes under (`norm-<variant>-touch-menu`).
pub const TOUCH_MENU_ROLE: &str = "menu";

fn entry_text(entry: &TouchMenuEntry, locale: Locale) -> String {
    chrome(&entry.label.en, &entry.label.de, locale)
}

/// 🖐️ The long-press menu of the viewport as a tree group: an enabled entry is an action item of its shortcut action id, except Select all, which raises the framework's own `selectAll` verb with the norm scope ([`select_all_args`]: all entity kinds; the app's `interaction_topology` names every selectable entity, so the scene size is unbounded); a disabled one is a plain item that says why nothing happens.
pub fn render_touch_menu(entries: &[TouchMenuEntry], controller_id: &'static str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let select_all = shortcut_action_id(&ShortcutAction::SelectAll);
    let mut rows = UiFixedList::default();
    for entry in entries {
        let id = format!("norm-touch-menu.{}", entry.action_id);
        let label = norm_ui_label(entry_text(entry, locale))?;
        let row = if let (Some(index), true) = (entry.check, entry.enabled) {
            let args = action_args_map(vec![("index", UiValue::Number(index as f64))])?;
            tree_item_with_action(id, label, None, norm_action(controller_id, "setSelectedCheckIndex", Some(args))?)?
        } else if entry.enabled {
            let verb = if entry.action_id == select_all { semio_framework::SELECT_ALL_ACTION_ID } else { entry.action_id.as_str() };
            let args = if verb == semio_framework::SELECT_ALL_ACTION_ID { Some(action_args_map(select_all_args().iter().map(|(key, value)| Ok((key.as_str(), ui_value_text(value)?))).collect::<UiAssemblyResult<Vec<_>>>()?)?) } else { None };
            tree_item_with_action(id, label, None, norm_action(controller_id, verb, args)?)?
        } else {
            tree_item_desc(id, label, Some(chrome("Not available", "Nicht verfügbar", locale)))?
        };
        rows.try_push(row).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    tree_group("norm-touch-menu".to_string(), norm_ui_label(chrome("Edit", "Bearbeiten", locale))?, true, rows)
}

/// 🗣️ The accessibility nodes of the menu: a `menu` with one `menuitem` per entry in order, each with its position in the set; a disabled entry says in its description that it is not available.
pub fn touch_menu_nodes(entries: &[TouchMenuEntry], lang: Lang) -> Vec<PublishedNode> {
    let german = matches!(lang, Lang::De);
    let text = |en: &str, de: &str| if german { de.to_owned() } else { en.to_owned() };
    let menu = AriaNode { role: "menu", name: text("Edit", "Bearbeiten"), ..AriaNode::default() };
    let items = entries.iter().enumerate().map(|(index, entry)| PublishedNode {
        key: entry.action_id.clone(),
        node: AriaNode {
            role: "menuitem",
            name: if german { entry.label.de.clone() } else { entry.label.en.clone() },
            description: (!entry.enabled).then(|| text("Not available", "Nicht verfügbar")),
            position: Some((index + 1, entries.len())),
            ..AriaNode::default()
        },
        depth: 1,
    });
    std::iter::once(PublishedNode { key: "touch-menu".to_owned(), node: menu, depth: 0 }).chain(items).collect()
}

/// 📣 Publishes the nodes of the menu of the artifact `domain` for the shell's `accessible_nodes` hook.
pub fn publish_touch_menu_access(domain: &str, entries: &[TouchMenuEntry], lang: Lang) {
    publish(TOUCH_MENU_ROLE, domain, touch_menu_nodes(entries, lang));
}
