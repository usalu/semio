//! 🔁️ Sequence persistent routes: reorganize, node-graph editing and the run order, driven step by step.

use neural_engine::Value as NeuralValue;
use std::collections::{BTreeMap, VecDeque};
use neural_engine::{Dictionary, RegistryRetirement, SharedRegistry, ValueRetirement};
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
use imperative_engine::{Executor, Path, RunResult, Step, imperative_module_registry};
use semio_framework_tool_machine::{NodeDragEmit, NodeGraphEditRow, node_drag_emit};
use crate::{SEQUENCE_DOCUMENT_SCHEMA, SequenceSnapshot, SequenceStep, SequenceWorkingScene};
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use crate::editor::sequence::modes::edit::windows::main::config::SequenceMainWindowConfig;
use crate::mutations::SequenceMutation;
use crate::editor::sequence::modes::edit::windows::script::transient::SequenceScriptWindowTransient;
use semio_framework_pack_json::{Value, json};
use crate::editor::sequence::edit_rules;
use crate::editor::sequence::modes::edit::windows::{main, script};
use crate::editor::sequence::commands::node_graph::node_graph_edit;
use crate::editor::sequence::editor_vocabulary::{SEQUENCE_PLAY_APP_ID, sequence_child_leaves_emit, sequence_fault, sequence_working_scene_from_children};
use crate::editor::sequence::retained::{SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_STORE_MAXIMUM_BYTES, SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS, SequenceCommand, SequenceRetainedSceneOwner, sequence_bounded_child_emit_bytes};
use crate::editor::sequence::app::SequencePlayApp;

//#region 🧵️PersistentRemainingRoutes
pub const SEQUENCE_PERSISTENT_MAXIMUM_UNITS: usize = 66_049;
pub const SEQUENCE_PERSISTENT_TOOL_IDS: &[&str] = &["reorganize", "nodeGraphEdit", "run"];
const SEQUENCE_PERSISTENT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "run", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

enum SequencePersistentAdvance {
    Progress(&'static str, &'static [u8]),
    Complete(Vec<SemioFlowMutation>),
    CompleteRun(String),
}

#[derive(Default)]
struct SequenceReorganizeState {
    initialized: usize,
    pass: usize,
    edge: usize,
    emit: usize,
    depths: Vec<usize>,
    moves: Vec<(usize, f64, f64)>,
}

impl SequenceReorganizeState {
    fn advance(&mut self, scene: &SequenceWorkingScene, config: &SequenceMainWindowConfig) -> Result<SequencePersistentAdvance, Fault> {
        let node_count = scene.steps.len();
        let edge_count = scene.edges.len();
        if node_count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || edge_count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-reorganize-capacity"));
        }
        if self.initialized < node_count {
            self.depths.push(0);
            self.initialized += 1;
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-nodes", b"{\"en\":\"Indexing layout node\",\"de\":\"Layoutknoten wird indiziert\"}"));
        }
        if self.pass < node_count && edge_count != 0 {
            let edge = &scene.edges[self.edge];
            let from = scene.steps.iter().position(|step| step.id == edge.from);
            let to = scene.steps.iter().position(|step| step.id == edge.to);
            if let (Some(from), Some(to)) = (from, to) {
                self.depths[to] = self.depths[to].max(self.depths[from].saturating_add(1).min(node_count));
            }
            self.edge += 1;
            if self.edge == edge_count {
                self.edge = 0;
                self.pass += 1;
            }
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-edges", b"{\"en\":\"Relaxing layout edge\",\"de\":\"Layoutkante wird verarbeitet\"}"));
        }
        if self.emit < node_count {
            let index = self.emit;
            let step = &scene.steps[index];
            let primary = self.depths[index] as f64 * 280.0;
            let secondary = self.depths[..index].iter().filter(|depth| **depth == self.depths[index]).count() as f64 * 160.0;
            let (x, y) = if config.orientation == "topBottom" { (secondary, primary) } else { (primary, secondary) };
            if step.x != x || step.y != y {
                self.moves.push((index, x, y));
            }
            self.emit += 1;
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-publish-plan", b"{\"en\":\"Planning node position\",\"de\":\"Knotenposition wird geplant\"}"));
        }
        let mut edit = edit_rules::SceneEdit::new(scene.clone());
        let positions: Vec<(String, f64, f64)> = self.moves.drain(..).map(|(index, x, y)| (scene.steps[index].id.clone(), x, y)).collect();
        edit.move_to(&positions);
        Ok(SequencePersistentAdvance::Complete(edit.leaves))
    }

    pub fn release_one(&mut self) -> bool {
        self.depths.pop().is_some() || self.moves.pop().is_some()
    }

    pub fn empty(&self) -> bool {
        self.depths.is_empty() && self.moves.is_empty()
    }
}

#[derive(Clone, Copy, Default)]
enum SequenceNodeGraphStage {
    #[default]
    Parse,
    Apply,
    DeleteDiscover,
    DeleteApply,
    Complete,
}

#[derive(Default)]
struct SequenceNodeGraphState {
    stage: SequenceNodeGraphStage,
    operations: Vec<NodeGraphEditRow>,
    operation: usize,
    edit: Option<edit_rules::SceneEdit>,
    gesture: Option<String>,
    delete_frontier: VecDeque<String>,
    delete_current: Option<String>,
    delete_scan: usize,
    delete_discovered: Vec<String>,
    discarded_steps: VecDeque<SequenceStep>,
    retirement: ValueRetirement,
}

impl SequenceNodeGraphState {
    /// ✋️ Publishes the batch's child leaves (design §12, §13.3): a batch that moved steps is a drag, committed as ONE
    /// composed-child tool transaction through the ONE node-drag machine of `🛠️tool-machine` from the admission's authoring
    /// seed (a view without command authority publishes the leaves plainly); every other batch is one child edit.
    fn publish(&mut self, snapshot: &SequenceSnapshot, leaves: Vec<SemioFlowMutation>, authoring_seed: &str) -> Emit<SequenceMutation, NoConfigMutation> {
        match self.gesture.take() {
            Some(gesture) => match node_drag_emit(SEQUENCE_PLAY_APP_ID, node_graph_edit::NODE_GRAPH_EDIT_VERB, authoring_seed, &gesture, leaves) {
                NodeDragEmit::Nothing => Emit::default(),
                drag => Emit { ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Emit::node_drag_child::<SemioFlowSnapshot, _>(drag, "content", &snapshot.content.child_id) },
            },
            None => sequence_child_leaves_emit(snapshot, leaves),
        }
    }

    fn advance(&mut self, command: &SequenceCommand, scene: &SequenceWorkingScene) -> Result<SequencePersistentAdvance, Fault> {
        match self.stage {
            SequenceNodeGraphStage::Parse => {
                let SequenceCommand::NodeGraphEdit(payload) = command else {
                    return Err(sequence_fault("app.command.tool-mismatch", "sequence-node-graph-route"));
                };
                if payload.operations_json.len() > SEQUENCE_RETAINED_RAW_BYTES {
                    return Err(sequence_fault("sequence.node-graph.malformed", "sequence-node-graph-bytes"));
                }
                let Ok(Value::Array(operations)) = json::parse(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
                    return Err(sequence_fault("sequence.node-graph.malformed", "sequence-node-graph-json"));
                };
                self.operations = operations.iter().map(|row| node_graph_edit::sequence_node_graph_row(&json::to_dsl_value(row))).collect::<Result<_, _>>()?;
                if self.operations.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                    return Err(sequence_fault("sequence.node-graph.malformed", "sequence-node-graph-items"));
                }
                self.edit = Some(edit_rules::SceneEdit::new(scene.clone()));
                self.stage = SequenceNodeGraphStage::Apply;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-parse", b"{\"en\":\"Decoded graph edit\",\"de\":\"Graphbearbeitung wurde dekodiert\"}"))
            }
            SequenceNodeGraphStage::Apply if self.operation < self.operations.len() => {
                let operation = &self.operations[self.operation];
                let edit = self.edit.as_mut().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-target"))?;
                match operation {
                    NodeGraphEditRow::Move(record) => {
                        if edit.drag(&record.node_ids, record.dx, record.dy) {
                            self.gesture.get_or_insert_with(|| record.gesture_id.clone());
                        }
                    }
                    NodeGraphEditRow::Connect { source_node_id: from, target_node_id: to, .. } => {
                        edit.connect(from, to, false);
                    }
                    NodeGraphEditRow::Disconnect { synapse_id } => {
                        edit.disconnect_edge(synapse_id);
                    }
                    NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
                        if node_ids.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                            return Err(sequence_fault("sequence.editor.capacity", "sequence-node-graph-delete-capacity"));
                        }
                        for synapse_id in synapse_ids {
                            edit.disconnect_edge(synapse_id);
                        }
                        self.delete_frontier = node_ids.iter().cloned().collect();
                        self.stage = SequenceNodeGraphStage::DeleteDiscover;
                        return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-delete", b"{\"en\":\"Preparing bounded graph removal\",\"de\":\"Begrenzte Graphentfernung wird vorbereitet\"}"));
                    }
                    NodeGraphEditRow::SetSlider { .. } | NodeGraphEditRow::InsertPort { .. } => return Err(sequence_fault("sequence.node-graph.unsupported", "sequence-node-graph-row-unsupported")),
                }
                self.operation += 1;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-operation", b"{\"en\":\"Applying graph operation\",\"de\":\"Graphoperation wird angewendet\"}"))
            }
            SequenceNodeGraphStage::DeleteDiscover => {
                if self.delete_current.is_none() {
                    if let Some(id) = self.delete_frontier.pop_front() {
                        if !self.delete_discovered.contains(&id) {
                            self.delete_discovered.push(id.clone());
                            self.delete_current = Some(id);
                            self.delete_scan = 0;
                        }
                        return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-root", "{\"en\":\"Traversing one selected graph root\",\"de\":\"Eine ausgewählte Graphwurzel wird durchlaufen\"}".as_bytes()));
                    }
                    self.stage = SequenceNodeGraphStage::DeleteApply;
                    self.delete_scan = 0;
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-apply", "{\"en\":\"Preparing selected graph removal\",\"de\":\"Ausgewählte Graphentfernung wird vorbereitet\"}".as_bytes()));
                }
                let target = &self.edit.as_ref().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-target"))?.scene;
                if self.delete_scan < target.steps.len() {
                    let step = &target.steps[self.delete_scan];
                    if step.slot.as_ref().is_some_and(|slot| self.delete_current.as_ref().is_some_and(|id| slot.owner == *id)) && !self.delete_discovered.contains(&step.id) && !self.delete_frontier.contains(&step.id) {
                        self.delete_frontier.push_back(step.id.clone());
                    }
                    self.delete_scan += 1;
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-child", b"{\"en\":\"Traversing one nested graph step\",\"de\":\"Ein verschachtelter Graphschritt wird durchlaufen\"}"));
                }
                self.delete_current = None;
                self.delete_scan = 0;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-next", "{\"en\":\"Advancing selected graph traversal\",\"de\":\"Ausgewählter Graphdurchlauf wird fortgesetzt\"}".as_bytes()))
            }
            SequenceNodeGraphStage::DeleteApply => {
                if let Some(id) = self.delete_discovered.pop() {
                    let edit = self.edit.as_mut().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-target"))?;
                    self.discarded_steps.extend(edit.remove_steps(std::slice::from_ref(&id)));
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-delete", "{\"en\":\"Removing one selected graph step\",\"de\":\"Ein ausgewählter Graphschritt wird entfernt\"}".as_bytes()));
                }
                self.operation += 1;
                self.stage = SequenceNodeGraphStage::Apply;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-complete", "{\"en\":\"Completed selected graph removal\",\"de\":\"Ausgewählte Graphentfernung wurde abgeschlossen\"}".as_bytes()))
            }
            SequenceNodeGraphStage::Apply => {
                self.stage = SequenceNodeGraphStage::Complete;
                self.advance(command, scene)
            }
            SequenceNodeGraphStage::Complete => {
                let edit = self.edit.as_mut().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-target"))?;
                let leaves = std::mem::take(&mut edit.leaves);
                if leaves.len() > SEQUENCE_PERSISTENT_MAXIMUM_UNITS {
                    return Err(sequence_fault("sequence.editor.capacity", "sequence-node-graph-output-items"));
                }
                Ok(SequencePersistentAdvance::Complete(leaves))
            }
        }
    }

    pub fn release_one(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        if grant.maximum_items == 0 {
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(step) = sequence_drain_retirement(&mut self.retirement, grant) {
            return step;
        }
        if self.operations.pop().is_some()
            || self.gesture.take().is_some()
            || self.delete_frontier.pop_front().is_some()
            || self.delete_current.take().is_some()
            || self.delete_discovered.pop().is_some()
        {
            return sequence_unit();
        }
        if let Some(step) = self.discarded_steps.front_mut() {
            let progress = sequence_push_params(&mut self.retirement, step, grant)?;
            self.discarded_steps.pop_front();
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress));
        }
        if let Some(edit) = self.edit.as_mut() {
            if let Some(step) = edit.scene.steps.last_mut() {
                let progress = sequence_push_params(&mut self.retirement, step, grant)?;
                edit.scene.steps.pop();
                return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress));
            }
            if edit.scene.edges.pop().is_some() || edit.leaves.pop().is_some() {
                return sequence_unit();
            }
        }
        if self.edit.take().is_some() {
            return sequence_unit();
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))
    }

    pub fn close_demand(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let admits = !self.discarded_steps.is_empty() || self.edit.as_ref().is_some_and(|edit| !edit.scene.steps.is_empty());
        let retains = !self.operations.is_empty() || self.gesture.is_some() || !self.delete_frontier.is_empty() || self.delete_current.is_some() || !self.delete_discovered.is_empty() || self.edit.is_some();
        sequence_owner_demand(&self.retirement, admits, retains)
    }
    pub fn empty(&self) -> bool {
        self.operations.is_empty()
            && self.gesture.is_none()
            && self.delete_frontier.is_empty()
            && self.delete_current.is_none()
            && self.delete_discovered.is_empty()
            && self.discarded_steps.is_empty()
            && self.retirement.terminal_is_empty()
            && self.edit.is_none()
    }
}

#[derive(Clone, Copy, Default)]
enum SequenceRunOrderStage {
    #[default]
    Steps,
    Edges,
    Heads,
    Choose,
    Walk,
    Remainder,
    Complete,
}

#[derive(Default)]
struct SequenceRunOrder {
    owner: Option<String>,
    name: Option<String>,
    stage: SequenceRunOrderStage,
    cursor: usize,
    scoped: Vec<usize>,
    incoming: Vec<(String, String)>,
    outgoing: Vec<(String, String)>,
    heads: Vec<usize>,
    ordered: Vec<usize>,
    current: Option<String>,
}

impl SequenceRunOrder {
    pub fn new(slot: Option<(&str, &str)>) -> Self {
        Self { owner: slot.map(|value| value.0.into()), name: slot.map(|value| value.1.into()), ..Self::default() }
    }

    pub fn matches(&self, step: &SequenceStep) -> bool {
        match ((self.owner.as_deref(), self.name.as_deref()), step.slot.as_ref()) {
            ((None, None), None) => true,
            ((Some(owner), Some(name)), Some(slot)) => slot.owner == owner && slot.name == name,
            _ => false,
        }
    }

    fn advance(&mut self, scene: &SequenceWorkingScene) -> &'static str {
        match self.stage {
            SequenceRunOrderStage::Steps if self.cursor < scene.steps.len() => {
                if self.matches(&scene.steps[self.cursor]) {
                    self.scoped.push(self.cursor);
                }
                self.cursor += 1;
                "sequence-run-order-step"
            }
            SequenceRunOrderStage::Steps => {
                self.stage = SequenceRunOrderStage::Edges;
                self.cursor = 0;
                "sequence-run-order-edges"
            }
            SequenceRunOrderStage::Edges if self.cursor < scene.edges.len() => {
                let edge = &scene.edges[self.cursor];
                if let Some(entry) = self.incoming.iter_mut().find(|entry| entry.0 == edge.to) {
                    entry.1 = edge.from.clone();
                } else {
                    self.incoming.push((edge.to.clone(), edge.from.clone()));
                }
                if let Some(entry) = self.outgoing.iter_mut().find(|entry| entry.0 == edge.from) {
                    entry.1 = edge.to.clone();
                } else {
                    self.outgoing.push((edge.from.clone(), edge.to.clone()));
                }
                self.cursor += 1;
                "sequence-run-order-edge"
            }
            SequenceRunOrderStage::Edges => {
                self.stage = SequenceRunOrderStage::Heads;
                self.cursor = 0;
                "sequence-run-order-heads"
            }
            SequenceRunOrderStage::Heads if self.cursor < self.scoped.len() => {
                let index = self.scoped[self.cursor];
                if !self.incoming.iter().any(|entry| entry.0 == scene.steps[index].id) {
                    self.heads.push(index);
                }
                self.cursor += 1;
                "sequence-run-order-head"
            }
            SequenceRunOrderStage::Heads => {
                self.stage = SequenceRunOrderStage::Choose;
                "sequence-run-order-choose"
            }
            SequenceRunOrderStage::Choose => {
                self.current = if self.heads.len() == 1 {
                    Some(scene.steps[self.heads[0]].id.clone())
                } else if self.scoped.len() == 1 {
                    Some(scene.steps[self.scoped[0]].id.clone())
                } else {
                    None
                };
                self.stage = if self.current.is_some() { SequenceRunOrderStage::Walk } else { SequenceRunOrderStage::Remainder };
                "sequence-run-order-start"
            }
            SequenceRunOrderStage::Walk => {
                let Some(id) = self.current.take() else {
                    self.stage = SequenceRunOrderStage::Remainder;
                    return "sequence-run-order-remainder";
                };
                if let Some(index) = self.scoped.iter().copied().find(|index| scene.steps[*index].id == id && !self.ordered.contains(index)) {
                    self.ordered.push(index);
                }
                self.current = self.outgoing.iter().find(|entry| entry.0 == id).map(|entry| entry.1.clone());
                if self.current.as_ref().is_some_and(|next| self.ordered.iter().any(|index| scene.steps[*index].id == *next)) {
                    self.current = None;
                }
                "sequence-run-order-walk"
            }
            SequenceRunOrderStage::Remainder => {
                let next = self.scoped.iter().copied().filter(|index| !self.ordered.contains(index)).min_by(|left, right| scene.steps[*left].id.cmp(&scene.steps[*right].id));
                if let Some(index) = next {
                    self.ordered.push(index);
                } else {
                    self.stage = SequenceRunOrderStage::Complete;
                }
                "sequence-run-order-remainder"
            }
            SequenceRunOrderStage::Complete => "sequence-run-order-complete",
        }
    }

    pub fn complete(&self) -> bool {
        matches!(self.stage, SequenceRunOrderStage::Complete)
    }
    pub fn release_one(&mut self) -> bool {
        self.scoped.pop().is_some()
            || self.incoming.pop().is_some()
            || self.outgoing.pop().is_some()
            || self.heads.pop().is_some()
            || self.ordered.pop().is_some()
            || self.current.take().is_some()
            || self.owner.take().is_some()
            || self.name.take().is_some()
    }
}

struct SequenceRunFrame {
    order: SequenceRunOrder,
    cursor: usize,
    repeat_remaining: usize,
    repeat_total: usize,
    while_key: Option<String>,
    while_iterations: usize,
}

/// 🧊️ Rebinding a live `Dictionary` root by plain assignment drops the PREVIOUS one without
/// retiring it, which the neural engine refuses in the guest with `final Dictionary ownership must
/// be explicitly retired or owned by a cold boundary`. `imperative_engine` has carried this exact
/// helper since its own sweep; `SequenceRunState::advance`'s `self.scope = result.scope` runs on
/// every executed step of a `run` verb, so it is the hottest of the four rebinds here.
fn replace_scope_cold(scope: &mut Dictionary, next: Dictionary) {
    neural_engine::ColdRetire::retire_cold(std::mem::replace(scope, next));
}

#[derive(Default)]
struct SequenceRunState {
    initialized: bool,
    registry: Option<SharedRegistry>,
    registry_retirement: Option<RegistryRetirement>,
    pending: Vec<Dictionary>,
    retirement: ValueRetirement,
    scope: Dictionary,
    effects: Vec<imperative_engine::EffectLogEntry>,
    frames: Vec<SequenceRunFrame>,
}

fn sequence_run_string(params: &Dictionary, key: &str) -> String {
    params.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or_default().to_string()
}

fn sequence_run_number(params: &Dictionary, key: &str) -> usize {
    params.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).unwrap_or(0.0).max(0.0) as usize
}

fn sequence_run_scope_bool(scope: &Dictionary, key: &str) -> bool {
    scope.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_bool()).unwrap_or(false)
}

impl SequenceRunState {
    fn advance(&mut self, scene: &SequenceWorkingScene) -> Result<SequencePersistentAdvance, Fault> {
        if scene.steps.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || scene.edges.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
            return Err(sequence_fault("sequence.run.capacity", "sequence-run-scene-capacity"));
        }
        if !self.initialized {
            let (registry, retirement) = SharedRegistry::new(imperative_module_registry());
            self.registry = Some(registry);
            self.registry_retirement = Some(retirement);
            replace_scope_cold(&mut self.scope, Dictionary::new());
            self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(None), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: None, while_iterations: 0 });
            self.initialized = true;
            return Ok(SequencePersistentAdvance::Progress("sequence-run-initialize", "{\"en\":\"Preparing execution\",\"de\":\"Ausführung wird vorbereitet\"}".as_bytes()));
        }
        let Some(frame) = self.frames.last_mut() else {
            let result = RunResult { scope: self.scope.clone(), effects: self.effects.clone() };
            let json = semio_framework_pack_json::to_json_string(&result);
            if json.len() > SEQUENCE_STORE_MAXIMUM_BYTES {
                return Err(sequence_fault("sequence.run.capacity", "sequence-run-result-capacity"));
            }
            return Ok(SequencePersistentAdvance::CompleteRun(json));
        };
        if !frame.order.complete() {
            let stage = frame.order.advance(scene);
            return Ok(SequencePersistentAdvance::Progress(stage, b"{\"en\":\"Ordering Sequence graph\",\"de\":\"Sequenzgraph wird geordnet\"}"));
        }
        if frame.cursor >= frame.order.ordered.len() {
            if let Some(key) = frame.while_key.as_ref() {
                if sequence_run_scope_bool(&self.scope, key) {
                    if frame.while_iterations == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                        return Err(sequence_fault("sequence.run.capacity", "sequence-run-while-capacity"));
                    }
                    frame.cursor = 0;
                    frame.while_iterations += 1;
                    return Ok(SequencePersistentAdvance::Progress("sequence-run-while-cursor", b"{\"en\":\"Continuing bounded while body\",\"de\":\"Begrenzter Solange-Block wird fortgesetzt\"}"));
                }
            } else if frame.repeat_remaining > 1 {
                frame.repeat_remaining -= 1;
                frame.cursor = 0;
                let index = frame.repeat_total - frame.repeat_remaining;
                let next_scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(index as i64)));
                replace_scope_cold(&mut self.scope, next_scope);
                return Ok(SequencePersistentAdvance::Progress("sequence-run-repeat-cursor", b"{\"en\":\"Continuing bounded repeat body\",\"de\":\"Begrenzter Wiederholungsblock wird fortgesetzt\"}"));
            }
            self.frames.pop();
            return Ok(SequencePersistentAdvance::Progress("sequence-run-retire-frame", "{\"en\":\"Completed nested execution frame\",\"de\":\"Verschachtelter Ausführungsrahmen wurde abgeschlossen\"}".as_bytes()));
        }
        let index = frame.order.ordered[frame.cursor];
        frame.cursor += 1;
        let step = scene.steps.get(index).ok_or_else(|| sequence_fault("sequence.run.step-missing", "sequence-run-step-index"))?;
        match step.kind.as_str() {
            "control.if" => {
                let key = sequence_run_string(&step.params.0, "key");
                let slot = if sequence_run_scope_bool(&self.scope, &key) { "then" } else { "else" };
                let depth_fault = self.frames.len() >= 65;
                let required_effects = if depth_fault { 2 } else { 1 };
                if self.effects.len().saturating_add(required_effects) > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                    return Err(sequence_fault("sequence.run.capacity", "sequence-run-effect-capacity"));
                }
                let input = self.scope.merge(&step.params.0);
                self.effects.push(imperative_engine::EffectLogEntry {
                    step_id: step.id.clone(),
                    kind: step.kind.clone(),
                    input,
                    output: Some(Dictionary::new().insert("branch", NeuralValue::Atom(neural_engine::Atom::String(slot.into())))),
                    error: None,
                });
                if depth_fault {
                    self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                } else {
                    self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, slot))), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: None, while_iterations: 0 });
                }
            }
            "control.repeat" => {
                let count = sequence_run_number(&step.params.0, "count");
                if count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                    return Err(sequence_fault("sequence.run.capacity", "sequence-run-repeat-capacity"));
                }
                if self.frames.len() >= 65 {
                    if self.effects.len() == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                        return Err(sequence_fault("sequence.run.capacity", "sequence-run-effect-capacity"));
                    }
                    self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                } else if count != 0 {
                    let next_scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(0)));
                    replace_scope_cold(&mut self.scope, next_scope);
                    self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, "body"))), cursor: 0, repeat_remaining: count, repeat_total: count, while_key: None, while_iterations: 0 });
                }
            }
            "control.while" => {
                let key = sequence_run_string(&step.params.0, "key");
                if sequence_run_scope_bool(&self.scope, &key) {
                    if self.frames.len() >= 65 {
                        if self.effects.len() == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                            return Err(sequence_fault("sequence.run.capacity", "sequence-run-effect-capacity"));
                        }
                        self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                    } else {
                        self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, "body"))), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: Some(key), while_iterations: 1 });
                    }
                }
            }
            _ => {
                let registry = self.registry.as_ref().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-run-registry"))?;
                let result = Executor::new(registry).run(&Path { steps: vec![Step { id: step.id.clone(), kind: step.kind.clone(), params: step.params.0.clone(), bodies: BTreeMap::new() }] }, &self.scope);
                if self.effects.len().saturating_add(result.effects.len()) > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                    self.pending.push(result.scope);
                    for effect in result.effects {
                        self.pending.push(effect.input);
                        if let Some(output) = effect.output { self.pending.push(output); }
                    }
                    return Err(sequence_fault("sequence.run.capacity", "sequence-run-effect-capacity"));
                }
                let halt_frame = result.effects.iter().any(|effect| effect.error.is_some());
                replace_scope_cold(&mut self.scope, result.scope);
                self.effects.extend(result.effects);
                if halt_frame {
                    if let Some(frame) = self.frames.last_mut() {
                        frame.cursor = frame.order.ordered.len();
                    }
                }
            }
        }
        Ok(SequencePersistentAdvance::Progress("sequence-run-step", "{\"en\":\"Executed Sequence step\",\"de\":\"Sequenzschritt wurde ausgeführt\"}".as_bytes()))
    }

    pub fn release_one(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        if grant.maximum_items == 0 {
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(step) = sequence_drain_retirement(&mut self.retirement, grant) {
            return step;
        }
        if let Some(step) = sequence_push_pending(&mut self.retirement, &mut self.pending, grant) {
            return step;
        }
        if let Some(effect) = self.effects.pop() {
            self.pending.push(effect.input);
            if let Some(output) = effect.output {
                self.pending.push(output);
            }
            return sequence_unit();
        }
        if let Some(frame) = self.frames.last_mut() {
            if frame.order.release_one() {
                return sequence_unit();
            }
        }
        if self.frames.pop().is_some() {
            return sequence_unit();
        }
        if let Some(registry) = self.registry.take() {
            drop(registry);
            return sequence_unit();
        }
        if let Some(retirement) = self.registry_retirement.as_mut() {
            let step = retirement.close_step(grant)?;
            if retirement.terminal_is_empty() {
                self.registry_retirement = None;
            }
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress()));
        }
        if self.initialized {
            self.pending.push(std::mem::take(&mut self.scope));
            self.initialized = false;
            return sequence_unit();
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))
    }

    pub fn close_demand(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(retirement) = self.registry_retirement.as_ref().filter(|_| self.retirement.terminal_is_empty() && self.pending.is_empty() && self.effects.is_empty() && self.frames.is_empty() && self.registry.is_none()) {
            let copy_bytes = retirement.next_copy_byte_demand()?;
            return Ok(semio_framework_value::RetirementDemand { copy_bytes, capacity_bytes: retirement.next_capacity_byte_demand(copy_bytes)?, release_bytes: retirement.next_release_byte_demand()?, depth: retirement.next_depth_demand()?.max(1) });
        }
        let admits = !self.pending.is_empty();
        let retains = self.initialized || self.registry.is_some() || self.registry_retirement.is_some() || !self.effects.is_empty() || !self.frames.is_empty();
        sequence_owner_demand(&self.retirement, admits, retains)
    }
    pub fn empty(&self) -> bool {
        !self.initialized && self.registry.is_none() && self.registry_retirement.is_none() && self.pending.is_empty() && self.retirement.terminal_is_empty() && self.effects.is_empty() && self.frames.is_empty()
    }
}

/// ♻️ Quotes the next neural value-retirement turn as one indivisible demand.
fn sequence_retirement_demand(retirement: &ValueRetirement) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    let copy_bytes = retirement.next_copy_byte_demand()?;
    Ok(semio_framework_value::RetirementDemand { copy_bytes, capacity_bytes: retirement.next_capacity_byte_demand(copy_bytes)?, release_bytes: retirement.next_release_byte_demand()?, depth: retirement.next_depth_demand()?.max(1) })
}

/// 📏️ Quotes one parked-dictionary admission or one structural drop.
pub fn sequence_owner_demand(retirement: &ValueRetirement, admits_dictionary: bool, retains_structure: bool) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    if !retirement.terminal_is_empty() {
        return sequence_retirement_demand(retirement);
    }
    if admits_dictionary {
        return Ok(semio_framework_value::RetirementDemand { capacity_bytes: ValueRetirement::domain_frame_birth_bytes(), depth: 1, ..Default::default() });
    }
    Ok(if retains_structure { semio_framework_value::RetirementDemand { depth: 1, ..Default::default() } } else { Default::default() })
}

/// ♻️ Drains one turn of a non-empty neural value retirement; `None` once it is terminal-empty.
pub fn sequence_drain_retirement(retirement: &mut ValueRetirement, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Option<Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError>> {
    if retirement.terminal_is_empty() {
        return None;
    }
    Some(retirement.close_step(grant).map(|step| semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())))
}

/// 🅿️ Admits one parked dictionary under the grant and keeps it parked when refused.
pub fn sequence_push_pending(retirement: &mut ValueRetirement, pending: &mut Vec<Dictionary>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Option<Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError>> {
    let dictionary = pending.pop()?;
    Some(match retirement.push_dictionary(dictionary, grant) {
        Ok(progress) => Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)),
        Err((error, dictionary)) => {
            pending.push(dictionary);
            Err(error)
        }
    })
}

/// 🧳️ Admits one step's parameter dictionary and puts it back when refused.
pub fn sequence_push_params(retirement: &mut ValueRetirement, step: &mut SequenceStep, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, semio_framework_value::ValueError> {
    let dictionary = std::mem::take(&mut step.params.0);
    retirement.push_dictionary(dictionary, grant).map_err(|(error, dictionary)| {
        step.params.0 = dictionary;
        error
    })
}

/// 🧱️ One structural item released without a physical backing grant.
pub fn sequence_unit() -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
    Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
}

/// 🗂️ Maps one release turn onto the interactive close contract.
pub fn sequence_close_step(step: Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_value::ValueError> {
    Ok(match step {
        Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)) => semio_framework_job::InteractiveJobCloseStep::Pending { progress },
        Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)) => semio_framework_job::InteractiveJobCloseStep::Complete { progress },
        Err(error) => return Err(error),
    })
}

enum SequencePersistentWorkspace {
    Reorganize(SequenceReorganizeState),
    NodeGraph(SequenceNodeGraphState),
    Run(SequenceRunState),
}
impl SequencePersistentWorkspace {
    pub fn new(tool_id: &str) -> Self {
        match tool_id {
            "reorganize" => Self::Reorganize(SequenceReorganizeState::default()),
            "nodeGraphEdit" => Self::NodeGraph(SequenceNodeGraphState::default()),
            _ => Self::Run(SequenceRunState::default()),
        }
    }
    fn advance(&mut self, command: &SequenceCommand, scene: &SequenceWorkingScene, config: &SequenceMainWindowConfig) -> Result<SequencePersistentAdvance, Fault> {
        match self {
            Self::Reorganize(state) => state.advance(scene, config),
            Self::NodeGraph(state) => state.advance(command, scene),
            Self::Run(state) => state.advance(scene),
        }
    }
    pub fn release_one(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        match self {
            Self::Reorganize(state) => if state.release_one() { sequence_unit() } else { Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())) },
            Self::NodeGraph(state) => state.release_one(grant),
            Self::Run(state) => state.release_one(grant),
        }
    }

    pub fn close_demand(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        match self {
            Self::Reorganize(state) => Ok(if state.empty() { Default::default() } else { semio_framework_value::RetirementDemand { depth: 1, ..Default::default() } }),
            Self::NodeGraph(state) => state.close_demand(),
            Self::Run(state) => state.close_demand(),
        }
    }
    pub fn empty(&self) -> bool {
        match self {
            Self::Reorganize(state) => state.empty(),
            Self::NodeGraph(state) => state.empty(),
            Self::Run(state) => state.empty(),
        }
    }
}

pub struct SequencePersistentWork {
    pub tool_id: &'static str,
    pub workspace_identity: u64,
    pub progress: usize,
    pub replay_target: Option<usize>,
    pub scene_owner: SequenceRetainedSceneOwner,
    workspace: SequencePersistentWorkspace,
    pub completed: bool,
    pub closing: bool,
}
impl SequencePersistentWork {
    pub fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self {
            tool_id,
            workspace_identity: identity,
            progress: 0,
            replay_target: None,
            scene_owner: SequenceRetainedSceneOwner::default(),
            workspace: SequencePersistentWorkspace::new(tool_id),
            completed: false,
            closing: false,
        }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequencePersistentWork {
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }
    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }
    fn extent(
        &self,
        _command: &SequenceCommand,
        snapshot: &SequenceSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        let scene = sequence_working_scene_from_children(snapshot, context?.children.as_ref()).ok()?;
        (scene.steps.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS && scene.edges.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS).then_some(SEQUENCE_PERSISTENT_MAXIMUM_UNITS)
    }
    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context, operation } = *input;
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.progress >= SEQUENCE_PERSISTENT_MAXIMUM_UNITS || command.command_id() != self.tool_id {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-persistent-progress-capacity"));
        }
        let context = context.ok_or_else(|| sequence_fault("sequence.window.unavailable", "sequence-window-context-required"))?;
        let config = main::config::from_snapshot(context.window_config.as_ref());
        self.scene_owner.capture(snapshot, context.children.as_ref())?;
        let scene = self.scene_owner.scene()?;
        match self.workspace.advance(command, scene, &config)? {
            SequencePersistentAdvance::Progress(stage, preview) => {
                self.progress += 1;
                if let Some(target) = self.replay_target {
                    if self.progress == target {
                        self.replay_target = None;
                    }
                    Ok(ArtifactCommandWorkStep::Replay { stage, preview })
                } else {
                    Ok(ArtifactCommandWorkStep::Progress { stage, preview })
                }
            }
            SequencePersistentAdvance::Complete(leaves) => {
                if self.replay_target.is_some() {
                    return Err(sequence_fault("sequence.resume.invalid", "sequence-persistent-replay-overrun"));
                }
                let emit = match &mut self.workspace {
                    SequencePersistentWorkspace::NodeGraph(state) => state.publish(snapshot, leaves, &operation.authoring_seed),
                    _ => sequence_child_leaves_emit(snapshot, leaves),
                };
                let exact_child = emit.child_emits.first().is_none_or(|child| child.slot == "content" && child.child_id == snapshot.content.child_id);
                let exact_lane = emit.config_mutations.is_empty() && emit.draft_mutations.is_empty() && emit.artifact_mutations.is_empty() && emit.child_emits.len() <= 1 && exact_child;
                if !exact_lane {
                    return Err(sequence_fault("sequence.publication.lane", "sequence-persistent-publication-lane"));
                }
                sequence_bounded_child_emit_bytes(&emit.child_emits, SEQUENCE_STORE_MAXIMUM_BYTES).map_err(|_| sequence_fault("sequence.editor.capacity", "sequence-persistent-output-bytes"))?;
                self.completed = true;
                Ok(ArtifactCommandWorkStep::Complete(emit))
            }
            SequencePersistentAdvance::CompleteRun(json) => {
                if self.replay_target.is_some() || self.tool_id != "run" {
                    return Err(sequence_fault("sequence.resume.invalid", "sequence-run-replay-overrun"));
                }
                let view = context.view_state.as_ref().ok_or_else(|| sequence_fault("sequence.window.unavailable", "sequence-script-window-view-required"))?;
                let transient = SequenceScriptWindowTransient { last_run_json: json };
                self.completed = true;
                Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                    emit: Emit::default(),
                    ephemeral: semio_framework_plugin::EphemeralEmit {
                        presence: Vec::new(),
                        transient: Vec::new(),
                        window_transient: vec![script::transient::addressed(view, transient)?],
                    },
                })
            }
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        let mut target = [0_u8; 24];
        self.write_checkpoint(&mut target).ok()?;
        target.get(index).copied()
    }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRP1" || checkpoint[4..8] != [0, 0, 0, 0] {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-invalid"));
        }
        let progress = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-cursor"))?)).map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-identity"))?);
        if identity != self.workspace_identity || progress > SEQUENCE_PERSISTENT_MAXIMUM_UNITS {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-owner"));
        }
        if !self.workspace.empty() || !self.scene_owner.empty() {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-persistent-restore-live-workspace"));
        }
        self.progress = 0;
        self.replay_target = (progress != 0).then_some(progress);
        self.workspace = SequencePersistentWorkspace::new(self.tool_id);
        self.completed = false;
        Ok(())
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            if !self.closing {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
            }
            if grant.maximum_items == 0 {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
            }
            self.replay_target = None;
            if !self.workspace.empty() {
                return sequence_close_step(self.workspace.release_one(grant));
            }
            sequence_close_step(self.scene_owner.release_one(grant))
        })
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demand()?.depth) }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none() && self.workspace.empty() && self.scene_owner.empty()
    }
}

impl SequencePersistentWork {
    pub fn close_demand(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.workspace.empty() { self.workspace.close_demand() } else { self.scene_owner.close_demand() }
    }
}

pub struct SequencePersistentJobFactory {
    pub keys: Vec<semio_framework::ToolFactoryKey>,
}
impl SequencePersistentJobFactory {
    pub fn new(controller: &str) -> Self {
        Self { keys: SEQUENCE_PERSISTENT_TOOL_IDS.iter().map(|id| semio_framework::ToolFactoryKey::new(controller, *id)).collect() }
    }
}
impl semio_framework::ToolJobFactory for SequencePersistentJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        "sequence.play/persistent-command.v1"
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_PERSISTENT_MAXIMUM_UNITS, 1, SEQUENCE_STORE_MAXIMUM_BYTES, 7_500, 1, 1)
    }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: SEQUENCE_RETAINED_RAW_BYTES, checkpoint_resume: true);
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequencePersistentJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_PERSISTENT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_PERSISTENT_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️PersistentRemainingRoutes
