//! ⚡️ Generation3d artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Generation3dSnapshot`'s shape per `📓️derivation-rules.md`: an id-keyed widget
//! collection (`create`/`update`/`delete-widget`), a relationship/edge collection of synapses
//! (`connect`/`update`/`disconnect-synapse`), a per-widget position map (`move-widget` /
//! `delete-widget-position`), two document-level scalars (`update-camera`, `change-schema`), an
//! id-keyed generation collection bridged from `semio_framework_artifact_playbook_playbook::GenerationMutation`
//! (`create`/`delete`/`rename-generation`, `change-generation-value`) with its two document-level scalars
//! (`select-generation`, `change-generation-preview`), and six gesture intents (`change-slider-value`,
//! `drag`/`rotate`/`scale-transforms`, `move-nodes`, `change-widget-input`). Every variant wraps exactly
//! one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Generation3dSnapshot, Generation3dMutation>`; `#[derive(dsl::Mutations)]`
//! below generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! Every leaf folder's stem equals its `semanticKind` and every leaf is wired by [`Leaves`](self) below, in enum
//! order — no other file path-includes a leaf.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::widget_id;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_os_flow::FlowHost;

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the generation3d document, derived per
/// `📓️derivation-rules.md` from `Generation3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, dsl::Mutations, semio_framework_value::RetireOwned)]
#[mutations(snapshot = Generation3dSnapshot, diff = Generation3dDiff, schema = "generation.3d", reference = Generation3dSnapshot, reference_diff = Generation3dDiff)]
pub enum Generation3dMutation {
    CreateWidget(semio_s_artifact_procedural_3d_mut_widgets::create_widget::CreateWidget),
    UpdateWidget(semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget),
    DeleteWidget(semio_s_artifact_procedural_3d_mut_widgets::delete_widget::DeleteWidget),
    ConnectSynapse(semio_s_artifact_procedural_3d_mut_synapses::connect_synapse::ConnectSynapse),
    UpdateSynapse(semio_s_artifact_procedural_3d_mut_synapses::update_synapse::UpdateSynapse),
    DisconnectSynapse(semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse),
    MoveWidget(semio_s_artifact_procedural_3d_mut_widgets::move_widget::MoveWidget),
    DeleteWidgetPosition(semio_s_artifact_procedural_3d_mut_widgets::delete_widget_position::DeleteWidgetPosition),
    UpdateCamera(semio_s_artifact_procedural_3d_mut_document::update_camera::UpdateCamera),
    ChangeSchema(semio_s_artifact_procedural_3d_mut_document::change_schema::ChangeSchema),
    CreateGeneration(semio_s_artifact_procedural_3d_mut_generations::create_generation::CreateGeneration),
    DeleteGeneration(semio_s_artifact_procedural_3d_mut_generations::delete_generation::DeleteGeneration),
    RenameGeneration(semio_s_artifact_procedural_3d_mut_generations::rename_generation::RenameGeneration),
    ChangeGenerationValue(semio_s_artifact_procedural_3d_mut_generations::change_generation_value::ChangeGenerationValue),
    ChangeSliderValue(semio_s_artifact_procedural_3d_mut_widgets::change_slider_value::ChangeSliderValue),
    DragTransforms(semio_s_artifact_procedural_3d_mut_transforms::drag_transforms::DragTransforms),
    RotateTransforms(semio_s_artifact_procedural_3d_mut_transforms::rotate_transforms::RotateTransforms),
    ScaleTransforms(semio_s_artifact_procedural_3d_mut_transforms::scale_transforms::ScaleTransforms),
    MoveNodes(semio_s_artifact_procedural_3d_mut_widgets::move_nodes::MoveNodes),
    ChangeWidgetInput(semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::ChangeWidgetInput),
    SelectGeneration(semio_s_artifact_procedural_3d_mut_generations::select_generation::SelectGeneration),
    ChangeGenerationPreview(semio_s_artifact_procedural_3d_mut_generations::change_generation_preview::ChangeGenerationPreview),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Generation3dMutation`] variant, in declaration order — the exact
/// vocabulary the `procedural-3d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧊️mutate-procedural-3d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-widget",
    "update-widget",
    "delete-widget",
    "connect-synapse",
    "update-synapse",
    "disconnect-synapse",
    "move-widget",
    "delete-widget-position",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
    "change-slider-value",
    "drag-transforms",
    "rotate-transforms",
    "scale-transforms",
    "move-nodes",
    "change-widget-input",
    "select-generation",
    "change-generation-preview",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️GenerationBridge
/// 🌉️ Bridges one `semio_framework_artifact_playbook_playbook::GenerationMutation` (the framework's own generation-editing
/// vocabulary — `Add`/`Remove`/`Rename`/`UpdateValues`) onto this facet's semantic
/// `Generation3dMutation` variants, so app-layer callers that already hold a `GenerationMutation`
/// (from `semio_framework_artifact_playbook_playbook::generation_operations`) need only swap the mapping function at the call
/// site, not learn this facet's internal triad-leaf module paths.
pub fn generation_mutation_to_generation3d(operation: GenerationMutation) -> Generation3dMutation {
    match operation {
        GenerationMutation::Add { generation } => Generation3dMutation::CreateGeneration(semio_s_artifact_procedural_3d_mut_generations::create_generation::CreateGeneration { generation, index: None }),
        GenerationMutation::Remove { id } => Generation3dMutation::DeleteGeneration(semio_s_artifact_procedural_3d_mut_generations::delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation3dMutation::RenameGeneration(semio_s_artifact_procedural_3d_mut_generations::rename_generation::RenameGeneration { id, new_name: name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation3dMutation::ChangeGenerationValue(semio_s_artifact_procedural_3d_mut_generations::change_generation_value::ChangeGenerationValue { id, question_id, new_value: value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️Cascades
/// 🗑️ The leaves that remove widget `id` with everything that hangs on it, in replay order: the `disconnect-synapse` of every
/// wire naming it, the `delete-widget-position` of its layout entry (before the widget goes — the position leaf addresses a
/// live widget) and the `delete-widget`. `delete-widget` itself does not cascade; this is where the cascade is spelled. No
/// leaves when the document holds no such widget.
pub fn generation3d_widget_removal(host_snapshot: &FlowHostSnapshot, id: &str) -> Vec<Generation3dMutation> {
    if widget_index(host_snapshot, id).is_none() {
        return Vec::new();
    }
    let mut leaves: Vec<Generation3dMutation> = host_snapshot.synapses.iter().filter(|synapse| synapse.from == id || synapse.to == id).map(|synapse| Generation3dMutation::DisconnectSynapse(semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse { id: synapse.id.clone() })).collect();
    if host_snapshot.layout.contains_key(id) {
        leaves.push(Generation3dMutation::DeleteWidgetPosition(semio_s_artifact_procedural_3d_mut_widgets::delete_widget_position::DeleteWidgetPosition { id: id.to_string() }));
    }
    leaves.push(Generation3dMutation::DeleteWidget(semio_s_artifact_procedural_3d_mut_widgets::delete_widget::DeleteWidget { id: id.to_string() }));
    leaves
}

/// 🗑️ The leaves that delete a selection of wires and widgets: every selected wire is cut first, then every selected widget
/// goes with the wires it still holds; an id the document does not hold is skipped and no wire is cut twice.
pub fn generation3d_selection_removal(host_snapshot: &FlowHostSnapshot, selected: &[String]) -> Vec<Generation3dMutation> {
    let mut cut: Vec<String> = Vec::new();
    let mut leaves = Vec::new();
    for id in selected {
        if synapse_index(host_snapshot, id).is_some() && !cut.contains(id) {
            cut.push(id.clone());
            leaves.push(Generation3dMutation::DisconnectSynapse(semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse { id: id.clone() }));
        }
    }
    let mut removed: Vec<&String> = Vec::new();
    for id in selected {
        if removed.contains(&id) {
            continue;
        }
        removed.push(id);
        for leaf in generation3d_widget_removal(host_snapshot, id) {
            if let Generation3dMutation::DisconnectSynapse(wire) = &leaf {
                if cut.contains(&wire.id) {
                    continue;
                }
                cut.push(wire.id.clone());
            }
            leaves.push(leaf);
        }
    }
    leaves
}
//#endregion 🔖️Cascades

pub type Generation3dEnvelope = ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>;
pub type Generation3dStore = ArtifactStore<Generation3dSnapshot, Generation3dMutation>;

//#region 🧊️Retirement
impl Generation3dMutation {
    /// 🧊️ Explicit cold-only disposal of one owned mutation. `create-widget`/`update-widget` carry a
    /// whole `Widget`, and a `Neuron`/`OutputPreview`/`Cluster` widget owns a `neural::Dictionary`
    /// (and an `OrderedSet`/`Tree`) that FAIL-CLOSE on a bare drop
    /// (`🧠️neural/⚙️engine/🦀️.rs`'s `Drop`), so a dropped mutation aborts the process. Every other
    /// variant is strings, numbers and plain `DslValue`s, which drop freely
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn retire_cold(self) {
        match self {
            Self::CreateWidget(semio_s_artifact_procedural_3d_mut_widgets::create_widget::CreateWidget { widget, .. }) | Self::UpdateWidget(semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget { widget }) => widget.retire_cold(),
            _ => {}
        }
    }
}
//#endregion 🧊️Retirement

pub fn inverse_generation3d_mutation(projection: &Generation3dSnapshot, mutation: &Generation3dMutation) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
    protocol::Mutation::inverse(mutation, projection)?

    })
}
//#endregion 🔖️Apply

//#region 🧭️GraphEditor
/// 🧭️ The explicit-intent recorder of every graph edit a command authors: each verb applies one `FlowHost` authoring
/// operation — which validates it, mints its ids and measures its nodes — and records the declared mutation leaves
/// that operation IS, cascades included, in the order they replay. No whole-snapshot diff stands between the host and
/// the leaves: `remove_widget` records the `disconnect-synapse` and `delete-widget-position` leaves of the wires and
/// the layout entry it takes with it, `connect_ports` records the `disconnect-synapse` of the wire it displaces, and a
/// port insertion records the `update-widget` and the `update-synapse` leaves of the wires it renumbers.
///
/// 🧩️ The host is the validator and the measurer (its port, type and cycle rules, its widget factory, its layout
/// algorithm); the leaves are the document edit. Replaying [`GraphEditor::finish`]'s leaves on the snapshot the host was
/// opened on reaches the host's own snapshot (`graph_editor_replay_reaches_the_host_snapshot`).
pub struct GraphEditor<'a> {
    host: &'a mut FlowHost,
    leaves: Vec<crate::Generation3dMutation>,
}

impl<'a> GraphEditor<'a> {
    /// 🧭️ Opens a recorder over `host`.
    pub fn new(host: &'a mut FlowHost) -> Self {
        Self { host, leaves: Vec::new() }
    }

    /// 👁️ The host's current graph, for the reads a command decides on.
    pub fn snapshot(&self) -> &FlowHostSnapshot {
        &self.host.host_snapshot
    }

    /// 👁️ The host itself, read-only, for the queries that need more than the graph (a widget's value form).
    pub fn host(&self) -> &FlowHost {
        self.host
    }

    /// 📐️ The rendered rectangles `[x, y, width, height]` of every node except `except` — what an automatic placement avoids.
    pub fn occupied(&self, except: &str) -> Vec<[f64; 4]> {
        self.host.dag.host_snapshot.nodes.iter().filter(|node| node.id != except).map(|node| [node.x, node.y, node.width, node.height]).collect()
    }

    /// 📐️ The rendered size `[width, height]` of node `id`.
    pub fn size_of(&self, id: &str) -> Option<[f64; 2]> {
        self.host.dag.host_snapshot.nodes.iter().find(|node| node.id == id).map(|node| [node.width, node.height])
    }

    /// 🧾️ The leaves recorded, in replay order. The caller owns them and retires them cold.
    pub fn finish(mut self) -> Vec<crate::Generation3dMutation> {
        std::mem::take(&mut self.leaves)
    }

    fn layout_leaf(&mut self, id: &str) {
        use semio_s_artifact_procedural_3d_mut_widgets::move_widget::MoveWidget;
        use crate::Generation3dMutation;
        let Some(layout) = self.host.host_snapshot.layout.get(id).cloned() else { return };
        if let Some(Generation3dMutation::MoveWidget(last)) = self.leaves.last_mut() {
            if last.id == id {
                last.layout = layout;
                return;
            }
        }
        self.leaves.push(Generation3dMutation::MoveWidget(MoveWidget { id: id.to_string(), layout }));
    }

    fn widget_leaf(&mut self, id: &str) {
        use semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget;
        use crate::Generation3dMutation;
        if let Some(widget) = self.host.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id) {
            self.leaves.push(Generation3dMutation::UpdateWidget(UpdateWidget { widget: widget.clone() }));
        }
    }

    fn synapse_leaves(&mut self, before: &[semio_framework_artifact_flow_flow::SynapseSpec]) {
        use semio_s_artifact_procedural_3d_mut_synapses::connect_synapse::ConnectSynapse;
        use semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse;
        use semio_s_artifact_procedural_3d_mut_synapses::update_synapse::UpdateSynapse;
        use crate::Generation3dMutation;
        let after = &self.host.host_snapshot.synapses;
        let mut leaves: Vec<Generation3dMutation> = before.iter().filter(|old| !after.iter().any(|new| new.id == old.id)).map(|old| Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: old.id.clone() })).collect();
        leaves.extend(after.iter().filter(|new| before.iter().any(|old| old.id == new.id && old != *new)).map(|new| Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: new.clone() })));
        leaves.extend(after.iter().enumerate().filter(|(_, new)| !before.iter().any(|old| old.id == new.id)).map(|(index, new)| Generation3dMutation::ConnectSynapse(ConnectSynapse { index, synapse: new.clone() })));
        self.leaves.extend(leaves);
    }

    /// 🧩️ Creates the admitted widget at `(x, y)` and records its placement.
    pub fn add_widget(&mut self, descriptor: &semio_framework_artifact_flow_flow::WidgetDescriptor, x: f64, y: f64) -> Result<String, String> {
        use semio_s_artifact_procedural_3d_mut_widgets::create_widget::CreateWidget;
        use crate::Generation3dMutation;
        let index = self.host.host_snapshot.widgets.len();
        let id = self.host.add_widget_descriptor(descriptor, x, y).map_err(|error| error.to_string())?;
        let widget = self.host.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id).cloned().ok_or_else(|| format!("created widget {id} is missing"))?;
        self.leaves.push(Generation3dMutation::CreateWidget(CreateWidget { index, widget }));
        self.layout_leaf(&id);
        Ok(id)
    }

    /// 📍️ Places widget `id` at `(x, y)`: one `move-widget`.
    pub fn move_widget(&mut self, id: &str, x: f64, y: f64) -> Result<(), String> {
        self.host.move_widget(id, x, y).map_err(|error| error.to_string())?;
        self.layout_leaf(id);
        Ok(())
    }

    /// ➖️ Removes widget `id` with everything that hangs on it, as `generation3d_widget_removal` spells the cascade.
    pub fn remove_widget(&mut self, id: &str) -> Result<(), String> {
        let leaves = crate::generation3d_widget_removal(&self.host.host_snapshot, id);
        self.host.remove_widget(id).map_err(|error| error.to_string())?;
        self.leaves.extend(leaves);
        Ok(())
    }

    /// 🔗️ Wires an output to an input: the `disconnect-synapse` of the wire the input drops (an input takes one wire) and
    /// the `connect-synapse` at the end of the list. Answers the new wire's id.
    pub fn connect_ports(&mut self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, String> {
        let before = self.host.host_snapshot.synapses.clone();
        let id = self.host.connect_ports(from_id, from_port, to_id, to_port).map_err(|error| error.to_string())?;
        self.synapse_leaves(&before);
        Ok(id)
    }

    /// ✂️ Cuts wire `id`: one `disconnect-synapse`.
    pub fn disconnect(&mut self, id: &str) -> Result<(), String> {
        use semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse;
        use crate::Generation3dMutation;
        self.host.disconnect(id).map_err(|error| error.to_string())?;
        self.leaves.push(Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: id.to_string() }));
        Ok(())
    }

    /// 🔀️ Splices `mid_id` between `anchor_id` and the consumers of its `anchor_out_port`: the `update-synapse` of every
    /// consumer wire it re-sources and the `connect-synapse` of the wire that feeds it.
    pub fn insert_between(&mut self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), String> {
        let before = self.host.host_snapshot.synapses.clone();
        self.host.insert_between(anchor_id, anchor_out_port, mid_id, mid_in_port, mid_out_port).map_err(|error| error.to_string())?;
        self.synapse_leaves(&before);
        Ok(())
    }

    /// ➕️ Opens an input port of variadic operator `id` at `index`: the `update-widget` of its new port list and the
    /// `update-synapse` of every wire the insertion renumbers.
    pub fn add_input_port(&mut self, id: &str, index: usize) -> Result<(), String> {
        let before = self.host.host_snapshot.synapses.clone();
        self.host.add_input_port(id, index).map_err(|error| error.to_string())?;
        self.widget_leaf(id);
        self.synapse_leaves(&before);
        Ok(())
    }

    /// ➕️ Opens an output port of variadic operator `id` at `index`; see [`GraphEditor::add_input_port`].
    pub fn add_output_port(&mut self, id: &str, index: usize) -> Result<(), String> {
        let before = self.host.host_snapshot.synapses.clone();
        self.host.add_output_port(id, index).map_err(|error| error.to_string())?;
        self.widget_leaf(id);
        self.synapse_leaves(&before);
        Ok(())
    }

    /// 👁️ Sets operator `id`'s preview flag: one `update-widget` when it flips. Answers whether it did.
    pub fn set_preview(&mut self, id: &str, shown: bool) -> bool {
        let mut flipped = false;
        for widget in &mut self.host.host_snapshot.widgets {
            if let Widget::Neuron { id: widget_id, preview, .. } = widget {
                if widget_id == id && *preview != shown {
                    *preview = shown;
                    flipped = true;
                }
            }
        }
        if flipped {
            self.widget_leaf(id);
        }
        flipped
    }

    /// 🗺️ Lays the graph out again with the host's layered layout: one `move-widget` per widget that moves.
    pub fn reorganize(&mut self, options: &semio_framework_os_infinite::board::schema::layout::DagLayoutOptions, control: &mut semio_framework_os_infinite::board::schema::layout::LayoutControl<'_>) -> Result<(), String> {
        let before = self.host.host_snapshot.layout.clone();
        let reorganized = self.host.reorganize(options, control).map_err(|error| error.to_string());
        let moved: Vec<String> = self.host.host_snapshot.layout.iter().filter(|(id, layout)| before.get(id) != Some(*layout)).map(|(id, _)| id.clone()).collect();
        let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
        retirement.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(before));
        retirement.retire_cold();
        reorganized?;
        for id in moved {
            self.layout_leaf(&id);
        }
        Ok(())
    }
}

/// 🧊️ A recorder dropped with leaves still in hand — a command refused after it recorded — retires them cold: a recorded
/// `create-widget` owns a widget whose neural dictionary rejects a bare drop.
impl Drop for GraphEditor<'_> {
    fn drop(&mut self) {
        for leaf in self.leaves.drain(..) {
            leaf.retire_cold();
        }
    }
}
//#endregion 🧭️GraphEditor

impl crate::GraphEditor<'_> {
    /// 📥️ Admits a widget descriptor at the editor text boundary before recording native intent.
    pub fn add_widget_json(&mut self, text: &str, x: f64, y: f64) -> Result<String, String> {
        let descriptor = semio_framework_pack_json::from_json_str::<semio_framework_artifact_flow_flow::WidgetDescriptor>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        self.add_widget(&descriptor, x, y)
    }
}

#[path = "../../../../../../🌉️apply/🦀️.rs"]
pub mod central_apply;
