//! 🧬️ Generation2d artifact schema — every field of the artifact with its state class.

use crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshot;
use semio_framework_artifact_infinite_dag::DagHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_host_snapshot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::render_scene_json;

use ::semio_framework_schema::ArtifactSchema;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::{flow_host_with_session, flow_neuron_kind_info_map, FlowEvalSession, FlowHost};
#[cfg(feature = "component-app-assembly")]
use semio_framework_ui::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Generation2dArtifact
/// 🧬️ Generation2dArtifact facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, Default)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation2d")]
pub struct Generation2dArtifact {
    #[state(artifact)]
    pub host_snapshot: FlowHostSnapshot,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation2dArtifact


impl Generation2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Generation2dSnapshot {
        Generation2dSnapshot { host_snapshot: self.host_snapshot.clone(), generation: self.generation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Generation2dSnapshot) -> Self {
        Self { host_snapshot: snapshot.host_snapshot, generation: snapshot.generation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Generation2dSnapshot) {
        self.host_snapshot = snapshot.host_snapshot;
        std::mem::replace(&mut self.generation, snapshot.generation).retire_cold();
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.generation2d` — twenty handcrafted schema leaves.
pub fn generation2d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.procedural.generation2d",
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
/// pure helpers over document types (`FlowHostSnapshot`/`DagHostSnapshot`/eval `Value`), not app-referencing.
/// 🏠️ Runs `body` against a catalogue-seeded host built from `fixture`, then retires that host.
///
/// A `FlowHost` owns a cloned `FlowHostSnapshot`, whose `layout: OrderedMap<WidgetLayout>` rejects a bare
/// drop (`ordered-map root must be explicitly retired before drop`,
/// `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), so a host is CLOSED through
/// [`FlowHost::retire_cold`], never dropped.
#[cfg(feature = "component-app-assembly")]
pub fn with_host<R>(host_snapshot: &FlowHostSnapshot, body: impl FnOnce(&mut FlowHost) -> R) -> R {
    FlowHost::with_host_snapshot(host_snapshot, |host| {
        host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
        body(host)
    })
}

/// 🏠️ [`with_host`]'s session-backed twin: the host shares `session`'s neural cache and converged
/// evaluation baseline, and is retired the same way. `body` receives the session back alongside the
/// host because every real caller needs it mutably (`sync`/`tick`).
#[cfg(feature = "component-app-assembly")]
pub fn host_with_session(host_snapshot: &FlowHostSnapshot, session: &FlowEvalSession, grant: semio_framework_value::RetainedCloneGrant) -> Result<FlowHost, semio_framework_value::ValueError> {
    match flow_host_with_session(FlowHost::from_host_snapshot(host_snapshot.clone()), session, grant) {
        Ok((host, _)) => Ok(host),
        Err((error, host)) => {
            host.retire_cold();
            Err(error)
        }
    }
}

#[cfg(feature = "component-app-assembly")]
pub fn with_host_session<R>(host_snapshot: &FlowHostSnapshot, session: &mut FlowEvalSession, grant: semio_framework_value::RetainedCloneGrant, body: impl FnOnce(&mut FlowHost, &mut FlowEvalSession) -> R) -> Result<R, semio_framework_value::ValueError> {
    let mut host = host_with_session(host_snapshot, session, grant)?;
    let result = body(&mut host, session);
    host.retire_cold();
    Ok(result)
}

//#region 🔖️HostGestures
/// 🔌️ Connects two ports through the host and returns the concrete leaves the host names: a `disconnect-synapse` for every wire
/// the host displaces from the target port, then one `connect-synapse` of the minted wire at the end of the list.
#[cfg(feature = "component-app-assembly")]
pub fn host_connect_ports(host: &mut FlowHost, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<Vec<crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation>, String> {
    use crate::standards::v1::subsets::any::schema::mutations::{connect_synapse, disconnect_synapse};
    let displaced: Vec<String> = host.host_snapshot.synapses.iter().filter(|synapse| synapse.to == to_id && synapse.to_port == to_port).map(|synapse| synapse.id.clone()).collect();
    let id = host.connect_ports(from_id, from_port, to_id, to_port).map_err(|error| error.to_string())?;
    let minted = host.host_snapshot.synapses.iter().position(|synapse| synapse.id == id).ok_or_else(|| format!("the host minted no wire {id}"))?;
    let mut leaves: Vec<_> = displaced.into_iter().map(disconnect_synapse).collect();
    leaves.push(connect_synapse(minted, host.host_snapshot.synapses[minted].clone()));
    Ok(leaves)
}

/// ✂️ Cuts one wire through the host: the single `disconnect-synapse` leaf.
#[cfg(feature = "component-app-assembly")]
pub fn host_disconnect(host: &mut FlowHost, synapse_id: &str) -> Result<Vec<crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation>, String> {
    host.disconnect(synapse_id).map_err(|error| error.to_string())?;
    Ok(vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse(synapse_id.to_string())])
}

/// 🗑️ Removes a widget through the host: a `disconnect-synapse` per wire it carried, its `clear-widget-layout` when it was placed,
/// then its `delete-widget`.
#[cfg(feature = "component-app-assembly")]
pub fn host_remove_widget(host: &mut FlowHost, widget_id: &str) -> Result<Vec<crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation>, String> {
    use crate::standards::v1::subsets::any::schema::mutations::{clear_widget_layout, delete_widget, disconnect_synapse};
    let wires: Vec<String> = host.host_snapshot.synapses.iter().filter(|synapse| synapse.from == widget_id || synapse.to == widget_id).map(|synapse| synapse.id.clone()).collect();
    let placed = host.host_snapshot.layout.contains_key(widget_id);
    host.remove_widget(widget_id).map_err(|error| error.to_string())?;
    let mut leaves: Vec<_> = wires.into_iter().map(disconnect_synapse).collect();
    if placed {
        leaves.push(clear_widget_layout(widget_id.to_string()));
    }
    leaves.push(delete_widget(widget_id.to_string()));
    Ok(leaves)
}

/// ➕️ Adds a widget through the host: the `create-widget` of what the host built from `descriptor_json` at the end of the list,
/// then the `move-widget` of the position the gesture named.
#[cfg(feature = "component-app-assembly")]
pub fn host_add_widget(host: &mut FlowHost, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<Vec<crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation>, String> {
    use crate::standards::v1::subsets::any::schema::mutations::{create_widget, move_widget};
    let id = host.add_widget(descriptor_json, world_x, world_y).map_err(|error| error.to_string())?;
    let at = host.host_snapshot.widgets.iter().position(|widget| crate::widget_id(widget) == id).ok_or_else(|| format!("the host built no widget {id}"))?;
    let widget = host.host_snapshot.widgets[at].clone();
    Ok(vec![create_widget(at, widget), move_widget(id, semio_framework_artifact_flow_flow::WidgetLayout { x: world_x, y: world_y })])
}

/// 🔢️ Inserts a port through the host: the `replace-widget` of the widget the host rebuilt, and a `replace-synapse` for each of its wires the
/// rebuild re-addressed.
#[cfg(feature = "component-app-assembly")]
pub fn host_insert_port(host: &mut FlowHost, widget_id: &str, input: bool, index: usize) -> Result<Vec<crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation>, String> {
    use crate::standards::v1::subsets::any::schema::mutations::{replace_synapse, replace_widget};
    let wires: Vec<_> = host.host_snapshot.synapses.iter().filter(|synapse| synapse.from == widget_id || synapse.to == widget_id).cloned().collect();
    if input {
        host.add_input_port(widget_id, index).map_err(|error| error.to_string())?;
    } else {
        host.add_output_port(widget_id, index).map_err(|error| error.to_string())?;
    }
    let at = host.host_snapshot.widgets.iter().position(|widget| crate::widget_id(widget) == widget_id).ok_or_else(|| format!("the host lost widget {widget_id}"))?;
    let widget = host.host_snapshot.widgets[at].clone();
    let mut leaves = vec![replace_widget(widget)];
    for before in wires {
        if let Some(after) = host.host_snapshot.synapses.iter().find(|synapse| synapse.id == before.id).filter(|after| **after != before) {
            leaves.push(replace_synapse(after.clone()));
        }
    }
    Ok(leaves)
}

/// 🗺️ Reorganizes the graph through the host: one `move-widget` per widget whose placement the layout pass changed.
#[cfg(feature = "component-app-assembly")]
pub fn host_reorganize(host: &mut FlowHost, options_json: &str) -> Result<Vec<crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation>, String> {
    use crate::standards::v1::subsets::any::schema::mutations::move_widget;
    let before: Vec<(String, semio_framework_artifact_flow_flow::WidgetLayout)> = host.host_snapshot.layout.iter().map(|(id, layout)| (id.clone(), layout.clone())).collect();
    let options: semio_framework_os_infinite::board::schema::layout::DagLayoutOptions = semio_framework_pack_json::from_json_str(options_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut progress = |_| true;
    let mut control = semio_framework_os_infinite::board::schema::layout::LayoutControl::new(100_000_000, &mut progress);
    host.reorganize(&options, &mut control).map_err(|error| error.to_string())?;
    Ok(host
        .host_snapshot
        .layout
        .iter()
        .filter(|(id, layout)| before.iter().find(|(held, _)| held == *id).map(|(_, previous)| previous) != Some(*layout))
        .map(|(id, layout)| move_widget(id.clone(), layout.clone()))
        .collect())
}
//#endregion 🔖️HostGestures

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

#[cfg(feature = "component-app-assembly")]
pub fn dag_host_snapshot_to_workflow(host_snapshot: &DagHostSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = host_snapshot
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: port.id.clone(), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: port.id.clone(), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = host_snapshot
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}





















pub fn empty_generation2d_snapshot() -> Generation2dSnapshot {
    Generation2dSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_artifact_flow_flow::CameraJson;
pub use semio_framework_artifact_flow_flow::FlowHostSnapshot;
//#endregion 🔁️Re-exports
