//! 🔺️ Wires artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::diff::WiresDiff;
use crate::schema::WiresArtifact;
use crate::schema::{array_mut, entity_id};
use crate::WiresSnapshot;
use dsl::DslValue;
use protocol::MutationDiff;
use std::collections::BTreeMap;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️BoardOps
pub fn apply_board_step(
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
    /// 🧬️ Applies sparse document entries onto the artifact.
    pub fn apply_to_artifact(&self, artifact: &WiresArtifact) -> protocol::MutationApplyResult<WiresArtifact> {
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
            next
        })
    }
}

impl MutationDiff<WiresSnapshot> for WiresDiff {
    fn apply(&self, snapshot: &WiresSnapshot) -> protocol::MutationApplyResult<WiresSnapshot> {
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
    fn absorb(&mut self, other: Self) {
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
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
/// 🖼️ Whole-artifact replacement from a document snapshot.
pub fn diff_set_snapshot(snapshot: &WiresSnapshot) -> WiresDiff {
    WiresDiff { artifact: Some(Box::new(WiresArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

/// 🔺️ Mints and owns a fresh content-addressed handle from `board`'s `nodes`/`edges` and wraps it as a
/// single always-present-slot diff entry — the sole builder every board-mutating triad's `diff.rs`
/// goes through (`create-node`/`delete-node`/`move-node`/`resize-node`/`change-node-kind`/
/// `change-node-shape`/`edit-node-text`/`set-node-root` all call this via `board_after_*`/directly).
/// `board`'s `camera`/`meta`/`schema`/`wires` keys are intentionally ignored here — those live outside
/// the composed content on `WiresSnapshot.camera`/`.meta` and no triad in this plugin ever writes them.
pub fn diff_board_fixture(board: &DslValue) -> WiresDiff {
    let nodes = crate::schema::fixture_nodes(board).to_vec();
    let edges = crate::schema::fixture_edges(board).to_vec();
    WiresDiff { content: Some(crate::wires_content_child_with_owner(nodes, edges)), ..Default::default() }
}

pub fn diff_wires_fixture(wires: DslValue) -> WiresDiff {
    WiresDiff { wires_fixture: Some(wires), ..Default::default() }
}

pub fn diff_wires_and_board(wires: DslValue, board: &DslValue) -> WiresDiff {
    let mut diff = diff_board_fixture(board);
    diff.wires_fixture = Some(wires);
    diff
}

pub fn board_after_add_node(snapshot: &WiresSnapshot, node: &DslValue) -> DslValue {
    let mut board = crate::wires_working_board(snapshot);
    array_mut(&mut board, "nodes").push(node.clone());
    board
}

pub fn fixtures_after_add_edge(snapshot: &WiresSnapshot, edge: &DslValue, relationship: &DslValue) -> (DslValue, DslValue) {
    let mut wires = snapshot.wires_fixture.clone();
    let mut board = crate::wires_working_board(snapshot);
    apply_board_step(&mut wires, &mut board, None, None, None, Some((edge, relationship)), None);
    (wires, board)
}

pub fn board_after_remove_node(snapshot: &WiresSnapshot, node_id: &str) -> DslValue {
    let mut board = crate::wires_working_board(snapshot);
    apply_board_step(&mut DslValue::Null, &mut board, None, Some(node_id), None, None, None);
    board
}

pub fn board_after_patch_node(snapshot: &WiresSnapshot, node_id: &str, patch: &BTreeMap<String, DslValue>) -> DslValue {
    let mut board = crate::wires_working_board(snapshot);
    apply_board_step(&mut DslValue::Null, &mut board, None, None, Some((node_id, patch)), None, None);
    board
}

pub fn fixtures_after_remove_edge(snapshot: &WiresSnapshot, edge_id: &str) -> (DslValue, DslValue) {
    let mut wires = snapshot.wires_fixture.clone();
    let mut board = crate::wires_working_board(snapshot);
    apply_board_step(&mut wires, &mut board, None, None, None, None, Some(edge_id));
    (wires, board)
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
