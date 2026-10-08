//! 📐️ Sequence edit rules: every editing gesture (add, remove, move, params, collapse, connect, disconnect, auto-layout, import) is resolved to the concrete intent leaves of the
//! composed Flow content child (`insert-node`, `remove-edge`, `remove-node`, `drag-nodes`, `set-node-param`, `insert-edge`) while it is applied to a working copy of the scene the
//! next step of the same gesture reads. The leaves are emitted by the rule that owns the gesture; no two scenes are ever differenced.

use crate::{SequenceEdge, SequenceStep, SequenceWorkingScene, SlotRef, StepParams};
use infinite_board_port_directed_dag::would_create_cycle;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{self as flow_mutations, SemioFlowMutation};

/// 📏️ How far two drag offsets may differ and still be one drag: a layout moves every step by its own offset, equal offsets share one leaf.
const DRAG_OFFSET_TOLERANCE: f64 = 1e-6;

/// 🧮️ The next unused `<prefix>-<serial>` id of the scene, above every step and edge serial and at least 101.
pub fn next_id(scene: &SequenceWorkingScene, prefix: &str) -> String {
    let serial = |prefix: &str, id: &str| id.strip_prefix(prefix).and_then(|rest| rest.parse::<u64>().ok());
    let step_max = scene.steps.iter().filter_map(|step| serial("step-", &step.id)).max().unwrap_or(0);
    let edge_max = scene.edges.iter().filter_map(|edge| serial("edge-", &edge.id)).max().unwrap_or(0);
    format!("{prefix}-{}", step_max.max(edge_max).max(100).saturating_add(1))
}

/// 🎛️ Whether `kind` owns slots whose members move, collapse and vanish with it.
pub fn is_control(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

/// 🎰️ The slot a control kind receives a dropped step in.
pub fn default_slot(kind: &str) -> &'static str {
    if kind == "control.if" {
        "then"
    } else {
        "body"
    }
}

/// 🗑️ Every step a removal of `roots` takes with it: the roots and, transitively, the members of every removed control's slots, in scene order.
pub fn removal_closure(scene: &SequenceWorkingScene, roots: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut selected: Vec<String> = Vec::new();
    let mut frontier: Vec<String> = roots.into_iter().filter(|root| scene.steps.iter().any(|step| &step.id == root)).collect();
    while let Some(id) = frontier.pop() {
        if selected.contains(&id) {
            continue;
        }
        if scene.steps.iter().any(|step| step.id == id && is_control(&step.kind)) {
            frontier.extend(scene.steps.iter().filter(|step| step.slot.as_ref().is_some_and(|slot| slot.owner == id)).map(|step| step.id.clone()));
        }
        selected.push(id);
    }
    scene.steps.iter().filter(|step| selected.contains(&step.id)).map(|step| step.id.clone()).collect()
}

/// ✏️ One gesture in progress: the working scene as the gesture has left it and the concrete leaves it has emitted so far.
#[derive(Clone, Debug, Default)]
pub struct SceneEdit {
    pub scene: SequenceWorkingScene,
    pub leaves: Vec<SemioFlowMutation>,
}

impl SceneEdit {
    /// 🏗️ Starts a gesture over `scene`.
    pub fn new(scene: SequenceWorkingScene) -> Self {
        Self { scene, leaves: Vec::new() }
    }

    /// ➕️ Inserts `step`: one `insert-node`.
    pub fn insert_step(&mut self, step: SequenceStep) {
        let node = crate::sequence_content_snapshot_from_working(std::slice::from_ref(&step), &[]).nodes.remove(0);
        self.leaves.push(SemioFlowMutation::InsertNode(flow_mutations::insert_node::InsertNode::new(node)));
        self.scene.steps.push(step);
    }

    /// ➕️ Inserts a new step of `kind` at `(x, y)` in `slot` under the next free id and returns that id.
    pub fn add_step(&mut self, kind: &str, x: f64, y: f64, slot: Option<SlotRef>) -> String {
        let id = next_id(&self.scene, "step");
        self.insert_step(SequenceStep { id: id.clone(), kind: kind.into(), params: StepParams::new(), x, y, slot, collapsed: false });
        id
    }

    /// ➕️ Inserts a new step of `kind` dropped at `(x, y)`: into the default slot of the picked control when it is expanded, free otherwise.
    pub fn add_step_dropped(&mut self, kind: &str, x: f64, y: f64, picked: Option<&str>) -> String {
        let slot = picked.and_then(|owner| self.scene.steps.iter().find(|step| step.id == owner && is_control(&step.kind) && !step.collapsed).map(|step| SlotRef { owner: owner.into(), name: default_slot(&step.kind).into() }));
        self.add_step(kind, x, y, slot)
    }

    /// ➖️ Removes the edge `id`: one `remove-edge`. `false` when absent.
    pub fn disconnect_edge(&mut self, id: &str) -> bool {
        let Some(index) = self.scene.edges.iter().position(|edge| edge.id == id) else { return false };
        self.scene.edges.remove(index);
        self.leaves.push(SemioFlowMutation::RemoveEdge(flow_mutations::remove_edge::RemoveEdge { id: id.into() }));
        true
    }

    /// ➖️ Removes every edge from `from_id` to `to_id`: one `remove-edge` each. `false` when none.
    pub fn disconnect(&mut self, from_id: &str, to_id: &str) -> bool {
        let ids: Vec<String> = self.scene.edges.iter().filter(|edge| edge.from == from_id && edge.to == to_id).map(|edge| edge.id.clone()).collect();
        ids.iter().fold(false, |any, id| self.disconnect_edge(id) | any)
    }

    /// ➖️ Removes the steps `ids` with the edges touching them (`remove-edge` each, then `remove-node` each) and returns the removed steps.
    pub fn remove_steps(&mut self, ids: &[String]) -> Vec<SequenceStep> {
        let touching: Vec<String> = self.scene.edges.iter().filter(|edge| ids.contains(&edge.from) || ids.contains(&edge.to)).map(|edge| edge.id.clone()).collect();
        for id in &touching {
            self.disconnect_edge(id);
        }
        let mut removed = Vec::new();
        for id in ids {
            if let Some(index) = self.scene.steps.iter().position(|step| &step.id == id) {
                removed.push(self.scene.steps.remove(index));
                self.leaves.push(SemioFlowMutation::RemoveNode(flow_mutations::remove_node::RemoveNode { id: id.clone() }));
            }
        }
        removed
    }

    /// ↔️ Moves the existing steps among `ids` by `(dx, dy)`: one `drag-nodes`. `false` when nothing moves.
    pub fn drag(&mut self, ids: &[String], dx: f64, dy: f64) -> bool {
        if (dx, dy) == (0.0, 0.0) || !dx.is_finite() || !dy.is_finite() {
            return false;
        }
        let targets: Vec<String> = self.scene.steps.iter().filter(|step| ids.contains(&step.id)).map(|step| step.id.clone()).collect();
        if targets.is_empty() {
            return false;
        }
        for step in self.scene.steps.iter_mut().filter(|step| targets.contains(&step.id)) {
            step.x += dx;
            step.y += dy;
        }
        self.leaves.push(SemioFlowMutation::DragNodes(flow_mutations::drag_nodes::DragNodes { targets, dx, dy }));
        true
    }

    /// 🧭️ Moves each step named in `positions` to its absolute position: one `drag-nodes` per distinct offset.
    pub fn move_to(&mut self, positions: &[(String, f64, f64)]) {
        let mut groups: Vec<(f64, f64, Vec<String>)> = Vec::new();
        for (id, x, y) in positions {
            let Some(step) = self.scene.steps.iter().find(|step| &step.id == id) else { continue };
            let (dx, dy) = (x - step.x, y - step.y);
            if (dx, dy) == (0.0, 0.0) || !dx.is_finite() || !dy.is_finite() {
                continue;
            }
            match groups.iter_mut().find(|(gx, gy, _)| (gx - dx).abs() <= DRAG_OFFSET_TOLERANCE && (gy - dy).abs() <= DRAG_OFFSET_TOLERANCE) {
                Some((_, _, ids)) => ids.push(id.clone()),
                None => groups.push((dx, dy, vec![id.clone()])),
            }
        }
        for (dx, dy, ids) in groups {
            self.drag(&ids, dx, dy);
        }
    }

    /// 🎛️ Sets the params of the step `id`: one `set-node-param` of the `params` entry. `Err(params)` hands the params back when the step is absent or already holds them.
    pub fn set_params(&mut self, id: &str, params: StepParams) -> Result<(), StepParams> {
        match self.scene.steps.iter_mut().find(|step| step.id == id) {
            Some(step) if step.params != params => {
                step.params = params;
                let value = semio_framework_pack_json::to_json_string(&step.params.0);
                self.leaves.push(SemioFlowMutation::SetNodeParam(flow_mutations::set_node_param::SetNodeParam { id: id.into(), key: "params".into(), value, at: None }));
                Ok(())
            }
            _ => Err(params),
        }
    }

    /// 🗂️ Toggles the collapsed flag of the control step `id`: one `set-node-param` of the `collapsed` entry. `false` when absent or not a control.
    pub fn toggle_collapsed(&mut self, id: &str) -> bool {
        let Some(step) = self.scene.steps.iter_mut().find(|step| step.id == id && is_control(&step.kind)) else { return false };
        step.collapsed = !step.collapsed;
        self.leaves.push(SemioFlowMutation::SetNodeParam(flow_mutations::set_node_param::SetNodeParam { id: id.into(), key: "collapsed".into(), value: step.collapsed.to_string(), at: None }));
        true
    }

    /// 🔗️ Connects `from_id` to `to_id` when both exist, differ, close no cycle and `from_id` has no outgoing edge: `remove-edge` for the edge entering `to_id` and one `insert-edge`.
    /// With `same_slot` the two steps must also share their slot scope.
    pub fn connect(&mut self, from_id: &str, to_id: &str, same_slot: bool) -> bool {
        let from = self.scene.steps.iter().find(|step| step.id == from_id);
        let to = self.scene.steps.iter().find(|step| step.id == to_id);
        let Some((from, to)) = from.zip(to) else { return false };
        let scope = |step: &SequenceStep| step.slot.as_ref().map(|slot| (slot.owner.clone(), slot.name.clone()));
        if from_id == to_id || (same_slot && scope(from) != scope(to)) {
            return false;
        }
        let existing: Vec<(String, String)> = self.scene.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) || self.scene.edges.iter().any(|edge| edge.from == from_id) {
            return false;
        }
        let entering: Vec<String> = self.scene.edges.iter().filter(|edge| edge.to == to_id).map(|edge| edge.id.clone()).collect();
        for id in &entering {
            self.disconnect_edge(id);
        }
        let edge = SequenceEdge { id: next_id(&self.scene, "edge"), from: from_id.into(), to: to_id.into() };
        let node = crate::sequence_content_snapshot_from_working(&[], std::slice::from_ref(&edge)).edges.remove(0);
        self.leaves.push(SemioFlowMutation::InsertEdge(flow_mutations::insert_edge::InsertEdge::new(node)));
        self.scene.edges.push(edge);
        true
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
