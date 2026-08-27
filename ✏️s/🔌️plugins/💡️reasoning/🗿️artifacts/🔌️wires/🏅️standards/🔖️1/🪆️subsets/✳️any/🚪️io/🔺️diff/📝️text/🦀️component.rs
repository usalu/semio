//! 🔺️ Wires artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::wires::schema::diff::WiresDiff;
use crate::artifacts::wires::schema::WiresArtifact;
use crate::artifacts::wires::schema::{array_mut, entity_id};
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use protocol::MutationDiff;
use std::collections::BTreeMap;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️BoardOps
pub async fn apply_board_step(
    wires: &mut DslValue,
    board: &mut DslValue,
    add_node: Option<&DslValue>,
    remove_node_id: Option<&str>,
    patch_node: Option<(&str, &BTreeMap<String, DslValue>)>,
    add_edge: Option<(&DslValue, &DslValue)>,
    remove_edge_id: Option<&str>,
) {
    if let Some(node) = add_node {
        array_mut(board, "nodes").push(node.clone());
    }
    if let Some(node_id) = remove_node_id {
        array_mut(board, "nodes").retain(|node| entity_id(node, "id") != Some(node_id));
    }
    if let Some((node_id, patch)) = patch_node {
        if let Some(DslValue::Object(entries)) = array_mut(board, "nodes").iter_mut().find(|node| entity_id(node, "id") == Some(node_id)) {
            for (key, value) in patch {
                if let Some((_, slot)) = entries.iter_mut().find(|(entry_key, _)| entry_key == key) {
                    *slot = value.clone();
                } else {
                    entries.push((key.clone(), value.clone()));
                }
            }
        }
    }
    if let Some((edge, relationship)) = add_edge {
        array_mut(board, "edges").push(edge.clone());
        if !relationship.is_null() {
            array_mut(wires, "relationships").push(relationship.clone());
        }
    }
    if let Some(edge_id) = remove_edge_id {
        array_mut(board, "edges").retain(|edge| entity_id(edge, "id") != Some(edge_id));
        array_mut(wires, "relationships").retain(|relationship| entity_id(relationship, "edgeId") != Some(edge_id));
    }
}
//#endregion 🔖️BoardOps

//#region 🔖️Apply
impl WiresDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &WiresArtifact) -> protocol::MutationApplyResult<WiresArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(wires) = &self.wires_fixture {
                next.wires_fixture = wires.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            if let Some(camera) = &self.camera {
                next.camera = camera.clone();
            }
            if let Some(meta) = &self.meta {
                next.meta = meta.clone();
            }
            if let Some(value) = &self.drag_node_id {
                next.drag_node_id = value.clone();
            }
            if let Some(value) = self.drag_last_x {
                next.drag_last_x = value;
            }
            if let Some(value) = self.drag_last_y {
                next.drag_last_y = value;
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<WiresSnapshot> for WiresDiff {
    async fn apply(&self, snapshot: &WiresSnapshot) -> protocol::MutationApplyResult<WiresSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(wires) = &self.wires_fixture {
                next.wires_fixture = wires.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            if let Some(camera) = &self.camera {
                next.camera = camera.clone();
            }
            if let Some(meta) = &self.meta {
                next.meta = meta.clone();
            }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(wires_fixture);
        take!(content);
        take!(camera);
        take!(meta);
        take!(drag_node_id);
        take!(drag_last_x);
        take!(drag_last_y);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
/// 🖼️ Whole-artifact replacement from a snapshot (UI fields defaulted).
pub async fn diff_set_snapshot(snapshot: &WiresSnapshot) -> WiresDiff {
    WiresDiff { artifact: Some(Box::new(WiresArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

/// 🔺️ Mints and owns a fresh content-addressed handle from `board`'s `nodes`/`edges` and wraps it as a
/// single always-present-slot diff entry — the sole builder every board-mutating triad's `diff.rs`
/// goes through (`create-node`/`delete-node`/`move-node`/`resize-node`/`change-node-kind`/
/// `change-node-shape`/`edit-node-text`/`set-node-root` all call this via `board_after_*`/directly).
/// `board`'s `camera`/`meta`/`schema`/`wires` keys are intentionally ignored here — those live outside
/// the composed content on `WiresSnapshot.camera`/`.meta` and no triad in this plugin ever writes them.
pub async fn diff_board_fixture(board: DslValue) -> WiresDiff {
    let nodes = crate::artifacts::wires::schema::fixture_nodes(&board).to_vec();
    let edges = crate::artifacts::wires::schema::fixture_edges(&board).to_vec();
    WiresDiff { content: Some(crate::artifacts::wires::wires_content_child_with_owner(nodes, edges)), ..Default::default() }
}

pub async fn diff_wires_fixture(wires: DslValue) -> WiresDiff {
    WiresDiff { wires_fixture: Some(wires), ..Default::default() }
}

pub async fn diff_wires_and_board(wires: DslValue, board: DslValue) -> WiresDiff {
    let mut diff = diff_board_fixture(board);
    diff.wires_fixture = Some(wires);
    diff
}

pub async fn board_after_add_node(snapshot: &WiresSnapshot, node: &DslValue) -> DslValue {
    let mut board = crate::artifacts::wires::wires_working_board(snapshot);
    array_mut(&mut board, "nodes").push(node.clone());
    board
}

pub async fn fixtures_after_add_edge(snapshot: &WiresSnapshot, edge: &DslValue, relationship: &DslValue) -> (DslValue, DslValue) {
    let mut wires = snapshot.wires_fixture.clone();
    let mut board = crate::artifacts::wires::wires_working_board(snapshot);
    apply_board_step(&mut wires, &mut board, None, None, None, Some((edge, relationship)), None);
    (wires, board)
}

pub async fn board_after_remove_node(snapshot: &WiresSnapshot, node_id: &str) -> DslValue {
    let mut board = crate::artifacts::wires::wires_working_board(snapshot);
    apply_board_step(&mut DslValue::Null, &mut board, None, Some(node_id), None, None, None);
    board
}

pub async fn board_after_patch_node(snapshot: &WiresSnapshot, node_id: &str, patch: &BTreeMap<String, DslValue>) -> DslValue {
    let mut board = crate::artifacts::wires::wires_working_board(snapshot);
    apply_board_step(&mut DslValue::Null, &mut board, None, None, Some((node_id, patch)), None, None);
    board
}

pub async fn fixtures_after_remove_edge(snapshot: &WiresSnapshot, edge_id: &str) -> (DslValue, DslValue) {
    let mut wires = snapshot.wires_fixture.clone();
    let mut board = crate::artifacts::wires::wires_working_board(snapshot);
    apply_board_step(&mut wires, &mut board, None, None, None, None, Some(edge_id));
    (wires, board)
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::empty_wires_snapshot;
    use serde_json::json;

    async fn node(id: &str, text: &str) -> DslValue {
        dsl::to_dsl_value(&json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })).unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_adds_node_via_board_fixture_delta() {
        let snapshot = empty_wires_snapshot();
        let diff = diff_board_fixture(board_after_add_node(&snapshot, &node("node-1", "Alpha")));
        let after = diff.apply(&snapshot).expect("valid mutation diff");
        assert_eq!(crate::artifacts::wires::wires_working_board(&after).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_replace_wins() {
        let mut diff = diff_board_fixture(board_after_add_node(&empty_wires_snapshot(), &node("node-1", "Alpha")));
        let replacement = empty_wires_snapshot();
        diff.absorb(diff_set_snapshot(&replacement));
        assert!(diff.content.is_none());
        assert!(diff.artifact.is_some());
    }
}
//#endregion 🧪️Tests
