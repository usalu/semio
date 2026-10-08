//! 🧬️ Generation3d artifact schema — every field of the artifact with its state class.

#[path = "🧭️transforms/🦀️.rs"]
pub mod transforms;


use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshot;
use crate::widget_id;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_host_snapshot as apply_generation_values_to_host_snapshot_json;

use ::semio_framework_schema::ArtifactSchema;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::{flow_host_with_session, FlowEvalSession, FlowHost};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Generation3dArtifact
/// 🧬️ Generation3dArtifact facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation3d")]
pub struct Generation3dArtifact {
    #[state(artifact)]
    pub host_snapshot: FlowHostSnapshot,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation3dArtifact

//#region 🔖️PreviewCamera
/// 📷️ 3D preview viewport camera (schema twin of the app config record).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dPreviewCamera {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub fov: f64,
}

impl Default for Generation3dPreviewCamera {
    fn default() -> Self {
        Self { position_x: 4.0, position_y: -4.0, position_z: 3.0, target_x: 0.0, target_y: 0.0, target_z: 0.0, fov: 45.0 }
    }
}
//#endregion 🔖️PreviewCamera

impl Default for Generation3dArtifact {
    fn default() -> Self {
        Self { host_snapshot: FlowHostSnapshot::default(), generation: GenerationPlayState::default().into() }
    }
}

impl Generation3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Generation3dSnapshot {
        Generation3dSnapshot { host_snapshot: self.host_snapshot.clone(), generation: self.generation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Generation3dSnapshot) -> Self {
        Self { host_snapshot: snapshot.host_snapshot, generation: snapshot.generation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Generation3dSnapshot) {
        self.host_snapshot = snapshot.host_snapshot;
        std::mem::replace(&mut self.generation, snapshot.generation).retire_cold();
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.generation3d` — twenty-two handcrafted mutation leaves.
pub fn generation3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.procedural.generation3d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🕸️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🧬️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// pure helpers over document types (`FlowHostSnapshot`/`DagHostSnapshot`/`FlowHost`), not app-referencing (the
/// Config-referencing preview/mesh-export helpers that used to sit alongside these stayed in
/// `crate::editor::generation3d` instead — see that file's own `PreviewPipeline`/`MeshBridge` regions).
pub const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
pub const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
pub const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";
pub const PROCEDURAL_EXAMPLE_BOX_FILLET: &str = "box-fillet-preview";
pub const PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE: &str = "sphere-box-fuse";
pub const PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE: &str = "face-sweep-extrude";
pub const PROCEDURAL_EXAMPLE_RECTANGLE_WIRE: &str = "rectangle-wire-preview";
pub const PROCEDURAL_EXAMPLE_MESH_WORKBENCH: &str = "mesh-workbench";
pub const PROCEDURAL_EXAMPLE_BOX_SHELL: &str = "box-shell-preview";



/// 📄️ The artifact's `Default` projection. NOT empty: `FlowHostSnapshot::default()`
/// (`🧰️framework/…/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs`) is the three-widget
/// `slider → add → preview` demo graph, so this is the DEFAULT document, not the empty one — the
/// name it carried until ticket 26/09/09/PROCEDURAL-3D-END-TO-END said otherwise and made every
/// "empty document" law read against a populated graph.
pub fn default_generation3d_snapshot() -> Generation3dSnapshot {
    Generation3dSnapshot::default()
}

/// 🕳️ The genuinely EMPTY projection: no widgets, no synapses, no positions, no generations — the
/// identity element every totality law is written against.
pub fn empty_generation3d_snapshot() -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.host_snapshot.widgets.clear();
    snapshot.host_snapshot.synapses.clear();
    snapshot
}

/// 🧾️ Whether `example_id` names a bundled procedural-3d example host_snapshot.
pub fn is_generation3d_example_id(example_id: &str) -> bool {
    matches!(
        example_id,
        PROCEDURAL_EXAMPLE_HEX_COLUMN
            | "demo"
            | PROCEDURAL_EXAMPLE_RECT_EXTRUDE
            | PROCEDURAL_EXAMPLE_SPHERE_TORUS
            | PROCEDURAL_EXAMPLE_BOX_FILLET
            | PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE
            | PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE
            | PROCEDURAL_EXAMPLE_RECTANGLE_WIRE
            | PROCEDURAL_EXAMPLE_MESH_WORKBENCH
            | PROCEDURAL_EXAMPLE_BOX_SHELL
    )
}







/// 🎯️ The roster entry `selected_id` names — the ONE lookup every generate-mode surface resolves its
/// "current generation" through.
///
/// 🐛️ Why not `playbook::selected_generation(state)`: that reads `GenerationPlayState.selected_generation_id`,
/// which the ARTIFACT only ever writes from `CreateGeneration`/`DeleteGeneration` replay — `selectGeneration`
/// emits no artifact mutation at all, only `Generation3dConfigMutation::SetSelectedGeneration`. The
/// evaluation path already resolved the selection off the CONFIG (`flow_eval_tick::evaluate`,
/// `generation_command_result`) while the three generate-mode window bodies still resolved it off the
/// artifact, so clicking a generation row moved the evaluation and moved nothing a user could see
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #7). The config is the single authority now, and this
/// function is where it is read.
pub fn generation_by_id<'a>(generation: &'a GenerationPlayState, selected_id: Option<&str>) -> Option<&'a semio_framework_artifact_playbook_playbook::FormGeneration> {
    let selected_id = selected_id?;
    generation.generations.iter().find(|entry| entry.id == selected_id)
}

pub fn generation_host_snapshot_for(host_snapshot: &FlowHostSnapshot, generation: &GenerationPlayState, selected_id: Option<&str>) -> FlowHostSnapshot {
    let Some(selected) = generation_by_id(generation, selected_id) else {
        return host_snapshot.clone();
    };
    let mut patched = host_snapshot.clone();
    for widget in &mut patched.widgets {
        let Some(value) = selected.values.get(widget_id(widget)) else {
            continue;
        };
        match widget {
            Widget::InputSlider { value: current, .. } => {
                if let Some(number) = value.as_f64() {
                    *current = number;
                }
            }
            Widget::InputNote { text, .. } => {
                if let Some(next) = value.as_str() {
                    *text = next.to_string();
                }
            }
            Widget::InputImage { src, .. } => {
                if let Some(next) = value.as_str() {
                    *src = next.to_string();
                }
            }
            Widget::Variable { name, .. } => {
                if let Some(next) = value.as_str() {
                    *name = next.to_string();
                }
            }
            _ => {}
        }
    }
    patched
}

/// 🏠️ Runs `body` against a catalogue-seeded host built from `fixture`, then retires that host.
///
/// A `FlowHost` owns a cloned `FlowHostSnapshot`, whose `layout: OrderedMap<WidgetLayout>` rejects a bare
/// drop (`ordered-map root must be explicitly retired before drop`,
/// `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), so a host is CLOSED through
/// [`FlowHost::retire_cold`], never dropped. This scope is the ONLY way generation3d builds one —
/// there is no `host_from_host_snapshot(…) -> FlowHost` to leak.
#[cfg(feature = "component-app-assembly")]
pub fn with_host<R>(host_snapshot: &FlowHostSnapshot, body: impl FnOnce(&mut FlowHost) -> R) -> R {
    FlowHost::with_host_snapshot(host_snapshot, |host| {
        host.set_neuron_kind_info_map(semio_framework_os_flow::flow_neuron_kind_info_map());
        body(host)
    })
}

/// 🏠️ [`with_host`]'s session-backed twin: the host shares `session`'s neural cache and converged
/// evaluation baseline, and is retired the same way. `body` receives the session back alongside the
/// host because every real caller needs it mutably (`sync`/`tick`), which a captured `&mut` could
/// not provide while the scope itself holds the session borrow.
#[cfg(feature = "component-app-assembly")]
pub fn with_host_session<R>(host_snapshot: &FlowHostSnapshot, session: &mut FlowEvalSession, body: impl FnOnce(&mut FlowHost, &mut FlowEvalSession) -> R) -> R {
    let mut host = flow_host_with_session(host_snapshot, session);
    let result = body(&mut host, session);
    host.retire_cold();
    result
}

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
#[cfg(feature = "component-app-assembly")]
pub struct GraphEditor<'a> {
    host: &'a mut FlowHost,
    leaves: Vec<crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation>,
}

#[cfg(feature = "component-app-assembly")]
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
    pub fn finish(mut self) -> Vec<crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation> {
        std::mem::take(&mut self.leaves)
    }

    fn layout_leaf(&mut self, id: &str) {
        use crate::standards::v1::subsets::any::schema::mutations::{move_widget::MoveWidget, Generation3dMutation};
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
        use crate::standards::v1::subsets::any::schema::mutations::{update_widget::UpdateWidget, Generation3dMutation};
        if let Some(widget) = self.host.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id) {
            self.leaves.push(Generation3dMutation::UpdateWidget(UpdateWidget { widget: widget.clone() }));
        }
    }

    fn synapse_leaves(&mut self, before: &[semio_framework_artifact_flow_flow::SynapseSpec]) {
        use crate::standards::v1::subsets::any::schema::mutations::{connect_synapse::ConnectSynapse, disconnect_synapse::DisconnectSynapse, update_synapse::UpdateSynapse, Generation3dMutation};
        let after = &self.host.host_snapshot.synapses;
        let mut leaves: Vec<Generation3dMutation> = before.iter().filter(|old| !after.iter().any(|new| new.id == old.id)).map(|old| Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: old.id.clone() })).collect();
        leaves.extend(after.iter().filter(|new| before.iter().any(|old| old.id == new.id && old != *new)).map(|new| Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: new.clone() })));
        leaves.extend(after.iter().enumerate().filter(|(_, new)| !before.iter().any(|old| old.id == new.id)).map(|(index, new)| Generation3dMutation::ConnectSynapse(ConnectSynapse { index, synapse: new.clone() })));
        self.leaves.extend(leaves);
    }

    /// 🧩️ Creates the widget `descriptor_json` describes at `(x, y)`: a `create-widget` at the end of the list and the
    /// `move-widget` that places it. Answers the new widget's id.
    pub fn add_widget(&mut self, descriptor_json: &str, x: f64, y: f64) -> Result<String, String> {
        use crate::standards::v1::subsets::any::schema::mutations::{create_widget::CreateWidget, Generation3dMutation};
        let index = self.host.host_snapshot.widgets.len();
        let id = self.host.add_widget(descriptor_json, x, y).map_err(|error| error.to_string())?;
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
        let leaves = crate::standards::v1::subsets::any::schema::mutations::generation3d_widget_removal(&self.host.host_snapshot, id);
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
        use crate::standards::v1::subsets::any::schema::mutations::{disconnect_synapse::DisconnectSynapse, Generation3dMutation};
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
    pub fn reorganize(&mut self, options_json: &str) -> Result<(), String> {
        let before = self.host.host_snapshot.layout.clone();
        let reorganized = self.host.reorganize(options_json).map_err(|error| error.to_string());
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
#[cfg(feature = "component-app-assembly")]
impl Drop for GraphEditor<'_> {
    fn drop(&mut self) {
        for leaf in self.leaves.drain(..) {
            leaf.retire_cold();
        }
    }
}
//#endregion 🧭️GraphEditor

//#region 🧪️Tests
#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "🧪️tests/🧭️graph-editor/🦀️.rs"]
mod graph_editor_tests;
//#endregion 🧪️Tests

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

#[cfg(feature = "component-app-assembly")]
pub fn dag_host_snapshot_to_workflow(host_snapshot: &semio_framework_artifact_infinite_dag::DagHostSnapshot) -> (Vec<semio_framework_ui::wgpu::NodeGraphNodeRecord>, Vec<semio_framework_ui::wgpu::NodeGraphEdgeRecord>) {
    let nodes: Vec<semio_framework_ui::wgpu::NodeGraphNodeRecord> = host_snapshot
        .nodes
        .iter()
        .map(|node| semio_framework_ui::wgpu::NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| semio_framework_ui::wgpu::NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| semio_framework_ui::wgpu::NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<semio_framework_ui::wgpu::NodeGraphEdgeRecord> = host_snapshot
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            semio_framework_ui::wgpu::NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}




//#endregion 🔖️DocumentHelpers

//#region 🔖️GumballTransforms




#[cfg(feature = "component-app-assembly")]
pub fn gumball_widget_json(host: &FlowHost, widget_id_str: &str) -> Option<semio_framework_value::DslValue> {
    host.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == widget_id_str).map(semio_framework_value::ToValue::to_value)
}

/// 🚫️ Why a gumball refuses its selection — each a NAMED fault code `generation3d.gumball.*` that a shell localizes; the
/// English [`GumballRefusal::detail`] is only the developer detail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GumballRefusal {
    UnknownOperation,
    NoShapeSource,
    KindUnavailable(String),
    NoShapeOutput,
    ListOutput,
    IdentifierOccupied,
    TransformUnavailable(String),
    MeshMissing,
    NotIndexedMesh,
    SelectionChanged,
    ComponentSelection(String),
    HostEdit(String),
}

impl GumballRefusal {
    /// 🏷️ The named fault code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnknownOperation => "generation3d.gumball.unknown-operation",
            Self::NoShapeSource => "generation3d.gumball.no-shape-source",
            Self::KindUnavailable(_) => "generation3d.gumball.kind-unavailable",
            Self::NoShapeOutput => "generation3d.gumball.no-shape-output",
            Self::ListOutput => "generation3d.gumball.list-output",
            Self::IdentifierOccupied => "generation3d.gumball.identifier-occupied",
            Self::TransformUnavailable(_) => "generation3d.gumball.transform-unavailable",
            Self::MeshMissing => "generation3d.gumball.mesh-missing",
            Self::NotIndexedMesh => "generation3d.gumball.not-indexed-mesh",
            Self::SelectionChanged => "generation3d.gumball.selection-changed",
            Self::ComponentSelection(_) => "generation3d.gumball.component-selection",
            Self::HostEdit(_) => "generation3d.gumball.host-edit",
        }
    }

    /// 🧑‍💻️ The English developer detail of the refusal.
    pub fn detail(&self) -> String {
        match self {
            Self::UnknownOperation => "unknown transform operation".into(),
            Self::NoShapeSource => "select a shape-producing widget".into(),
            Self::KindUnavailable(kind) => format!("widget kind {kind} is unavailable"),
            Self::NoShapeOutput => "the selected output does not contain a shape".into(),
            Self::ListOutput => "extract a single shape from the list before transforming it".into(),
            Self::IdentifierOccupied => "the generated transform identifier is already occupied".into(),
            Self::TransformUnavailable(kind) => format!("transform {kind} is unavailable"),
            Self::MeshMissing => "the selected mesh no longer exists".into(),
            Self::NotIndexedMesh => "select a single indexed mesh output; convert B-Rep geometry to a mesh first".into(),
            Self::SelectionChanged => "the component selection changed during the transform".into(),
            Self::ComponentSelection(detail) | Self::HostEdit(detail) => detail.clone(),
        }
    }
}


/// 🎚️ The literals a gumball composes from on an operator it inserts: a translate's zero offset, a rotate's zero turn about
/// +z, a scale's unit factors — the inserted record holds its kind's declared defaults (design §20.9), which need not be the
/// identity, so the gesture's relative leaf only composes correctly after these land.
pub fn gumball_identity(operation: &str) -> Vec<(&'static str, crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue)> {
    use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue;
    match operation {
        "translate" => vec![("offset", WidgetInputValue::Vector([0.0; 3]))],
        "rotate" => vec![("axis", WidgetInputValue::Vector([0.0, 0.0, 1.0])), ("angle", WidgetInputValue::Number(0.0))],
        "scale" => vec![("factor", WidgetInputValue::Vector([1.0; 3]))],
        _ => Vec::new(),
    }
}

/// 🧾️ The `change-widget-input` leaves an inserted operator's record needs to hold `wanted` (design §19.4): one per wanted
/// channel the record holds with another literal, typed like the record's own literal (a point stays a point); a channel
/// the record does not hold is not one of its inputs and is skipped.
pub fn record_input_leaves(record: &Widget, wanted: &[(&str, crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue)]) -> Vec<crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{change_widget_input, WidgetInputValue};
    let Widget::Neuron { id, params, .. } = record else { return Vec::new() };
    wanted
        .iter()
        .filter_map(|(channel, value)| {
            let stored = WidgetInputValue::of_literal(&semio_framework_value::ToValue::to_value(params.get(channel)?))?;
            let typed = match (value, &stored) {
                (WidgetInputValue::Point(axes) | WidgetInputValue::Vector(axes), WidgetInputValue::Point(_)) => WidgetInputValue::Point(*axes),
                (WidgetInputValue::Point(axes) | WidgetInputValue::Vector(axes), WidgetInputValue::Vector(_)) => WidgetInputValue::Vector(*axes),
                (value, stored) if value.schema() == stored.schema() => value.clone(),
                _ => return None,
            };
            (typed != stored).then(|| change_widget_input(id.clone(), *channel, typed))
        })
        .collect()
}
//#endregion 🔖️GumballTransforms

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_artifact_flow_flow::CameraJson;
pub use semio_framework_artifact_flow_flow::FlowHostSnapshot;
pub use semio_framework_artifact_flow_flow::Widget;
pub use semio_framework_artifact_playbook_playbook::GenerationPlayState;
//#endregion 🔁️Re-exports
