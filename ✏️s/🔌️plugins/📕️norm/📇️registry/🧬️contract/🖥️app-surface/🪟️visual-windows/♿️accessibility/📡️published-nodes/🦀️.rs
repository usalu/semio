//! 📡️ Published nodes: the accessible nodes the window renderers just drew (check rows, scene entities and handles, diagrams with their table twins), kept per window role and artifact domain for the shell's `accessible_nodes` hook, in English or German as rendered.
//!
//! 🔗️ Nodes: [`screen_reader`](super::screen_reader); seam: `semio_framework::PluginShellHooks::accessible_nodes`; see `visual-editor-architecture.md` §8.2.

use super::screen_reader::{diagram_nodes, entity_node, handle_node, scene_list_node, AriaNode};
use crate::viewport3d::config::Lang;
use crate::diagrams2d::{marker_grip, marker_rest, marker_utilisation};
use semio_s_plugin_norm_structural_scene::projection_kit::base_id;
use semio_s_plugin_norm_structural_scene::{Diagram2d, Handle, HandleKind, StructuralScene};
use std::sync::{Mutex, PoisonError};

/// 📡️ One node of a window's accessibility tree: a key unique in the window, the node and its depth.
#[derive(Clone, Debug, PartialEq)]
pub struct PublishedNode {
    pub key: String,
    pub node: AriaNode,
    pub depth: usize,
}

type Slot = (String, Vec<PublishedNode>);

static STORE: Mutex<Vec<Slot>> = Mutex::new(Vec::new());

fn slot_key(role: &str, domain: &str) -> String {
    format!("{role}|{domain}")
}

/// 📣 Replaces the nodes of the window `role` (`viewport`, `checks`, `diagrams`) of the artifact `domain` (`norm.en1993`; empty for a window without one).
pub fn publish(role: &str, domain: &str, nodes: Vec<PublishedNode>) {
    let key = slot_key(role, domain);
    let mut store = STORE.lock().unwrap_or_else(PoisonError::into_inner);
    match store.iter_mut().find(|(known, _)| *known == key) {
        Some((_, slot)) => *slot = nodes,
        None => store.push((key, nodes)),
    }
}

/// 📡️ The nodes last published for a window, empty before its first paint.
pub fn published(role: &str, domain: &str) -> Vec<PublishedNode> {
    let key = slot_key(role, domain);
    STORE.lock().unwrap_or_else(PoisonError::into_inner).iter().find(|(known, _)| *known == key).map(|(_, nodes)| nodes.clone()).unwrap_or_default()
}

/// 🪟️ The window role and artifact domain of a shell window id: `norm-en1993-checks` is (`checks`, `norm.en1993`), `norm-en1993-touch-menu` is (`menu`, `norm.en1993`), `norm-viewport` is (`viewport`, ``); the handle overlay lives in the viewport; `None` for windows that publish nothing.
pub fn window_scope(window_id: &str) -> Option<(&'static str, String)> {
    let parts: Vec<&str> = window_id.split('-').collect();
    let role = match parts.last().copied()? {
        "viewport" | "handle" | "handles" => "viewport",
        "checks" => "checks",
        "diagrams" => "diagrams",
        "menu" => "menu",
        _ => return None,
    };
    let domain = if parts.len() >= 3 { format!("norm.{}", parts[1]) } else { String::new() };
    Some((role, domain))
}

/// 🆔️ The artifact domain of a window body key (`norm.en1993.play.diagrams` is `norm.en1993`).
pub fn domain_of_body(body_key: &str) -> String {
    body_key.split('.').take(2).collect::<Vec<_>>().join(".")
}

fn publish_all(items: impl IntoIterator<Item = (String, AriaNode, usize)>) -> Vec<PublishedNode> {
    items.into_iter().map(|(key, node, depth)| PublishedNode { key, node, depth }).collect()
}

/// 🕹️ The parameter a handle rests at: the anchor projected on the axis of an along-axis handle, the angle of an angular one, else the lower limit or zero; a screen reader announces it as the slider value.
pub fn rest_parameter(handle: &Handle) -> f64 {
    match &handle.kind {
        HandleKind::ScalarAlong { axis } => handle.anchor.dot(axis),
        HandleKind::Angular { .. } => handle.kind.angular_s(&handle.anchor).unwrap_or(0.0),
        _ => handle.limits.as_ref().map_or(0.0, |limits| limits.min),
    }
}

/// 🧊️ The viewport window: the scene list, one option per entity in scene order, and one slider per handle of the selection below it.
pub fn viewport_nodes(scene: &StructuralScene, selected: &[String], handles: &[Handle], lang: Lang) -> Vec<PublishedNode> {
    let size = scene.entities.len();
    let list = std::iter::once(("scene:list".to_owned(), scene_list_node(lang), 0));
    let options = scene.entities.iter().enumerate().map(|(index, e)| (format!("scene:{}", e.entity.id), entity_node(e, lang, index + 1, size, selected.contains(&e.entity.id)), 1));
    let governing = |handle: &Handle| {
        let owner = scene.entities.iter().find(|e| e.entity.id == handle.entity.id).or_else(|| scene.entities.iter().find(|e| base_id(&e.entity.id) == base_id(&handle.entity.id)));
        owner.and_then(|e| e.utilisation.as_ref()).map(|u| (u.ratio, u.status))
    };
    let sliders = handles.iter().map(|handle| (format!("handle:{}", handle.id), handle_node(handle, Some(rest_parameter(handle)), governing(handle), lang), 1));
    publish_all(list.chain(options).chain(sliders))
}

/// 📈️ The diagrams window: per diagram its image node and the table node of its twin, the image pointing at the table, then one slider per marker bound to a field (named by the marker label, valued in the unit of the field with the live utilisation of its bound check) as the keyboard twin of dragging it.
pub fn diagram_window_nodes(diagrams: &[Diagram2d], lang: Lang) -> Vec<PublishedNode> {
    let pairs = diagrams.iter().flat_map(|diagram| {
        let table_key = format!("diagram:{}:table", diagram.id);
        let (image, table) = diagram_nodes(diagram, lang, &table_key);
        [(format!("diagram:{}", diagram.id), image, 0), (table_key, table, 0)]
    });
    let sliders = diagrams.iter().flat_map(|diagram| {
        diagram.markers.iter().filter_map(move |marker| {
            let grip = marker_grip(diagram, marker)?;
            let node = handle_node(&grip, marker_rest(diagram, marker), marker_utilisation(marker), lang);
            Some((grip.id, node, 1))
        })
    });
    publish_all(pairs.chain(sliders))
}
