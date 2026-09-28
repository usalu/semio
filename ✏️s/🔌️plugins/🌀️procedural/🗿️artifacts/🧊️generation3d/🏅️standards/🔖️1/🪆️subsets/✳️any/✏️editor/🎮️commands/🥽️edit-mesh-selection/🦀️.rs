//! 🥽️ Selected mesh components become an editable operation in the generator graph.

use crate::editor::generation3d::{config::{Generation3dConfig, Generation3dConfigMutation}, selection::{component_group, DOMAIN}};
use crate::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host, mutations::text::Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, InteractionWrite};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "edit-mesh-selection")]
pub struct EditMeshSelection {
    pub operation: String,
    pub amount: f64,
    pub cuts: u32,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl EditMeshSelection {
    fn granularity(&self) -> &'static str { match self.operation.as_str() { "moveVertices" => "vertex", "loopCut" => "edge", _ => "face" } }
}

pub fn insert_operation(host: &mut FlowHost, payload: &EditMeshSelection, ids: &[String]) -> Result<String, String> {
    let (_, components) = component_group(ids)?;
    let operation = payload.operation.as_str();
    if !matches!(operation, "extrude" | "inset" | "subdivide" | "flip" | "deleteFaces" | "moveVertices" | "loopCut") { return Err("Unknown mesh operation".into()); }
    if [payload.amount, payload.dx, payload.dy, payload.dz].iter().any(|value| !value.is_finite()) { return Err("Mesh edit values must be finite".into()); }
    if operation == "inset" && payload.amount <= 0.0 { return Err("Inset amount must be positive".into()); }
    if !(1..=256).contains(&payload.cuts) { return Err("Cuts must be an integer from 1 to 256".into()); }
    let field = match operation { "moveVertices" => "vertices", "loopCut" => "edges", _ => "faces" };
    let mut params = serde_json::json!({field:{"$schema":"text","value":serde_json::to_string(&components).map_err(|error| error.to_string())?}});
    if operation == "moveVertices" { params["offset"] = serde_json::json!({"$schema":"vector","x":payload.dx,"y":payload.dy,"z":payload.dz}); }
    if operation == "loopCut" { params["cuts"] = serde_json::json!({"$schema":"number","value":payload.cuts}); }
    if matches!(operation, "extrude" | "inset") { params[if operation == "extrude" { "distance" } else { "amount" }] = serde_json::json!({"$schema":"number","value":payload.amount}); }
    insert_mesh_operation(host, operation, payload.granularity(), ids, &params.to_string())
}

/// 🧩️ Splices one typed mesh operation into the selected output and reconnects its consumers.
pub(crate) fn insert_mesh_operation(host: &mut FlowHost, operation: &str, granularity: &str, ids: &[String], params_json: &str) -> Result<String, String> {
    let (target, _) = component_group(ids)?;
    if target.granularity != granularity { return Err("The operation requires matching face, edge, or vertex selection".into()); }
    if target.index != 0 { return Err("Extract one mesh from the list before editing its components".into()); }
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let kind = host.host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::Neuron { id, neuron_kind, .. } if id == target.widget => Some(neuron_kind), _ => None }).ok_or("The selected mesh no longer exists")?;
    let source = infos.get(kind).ok_or("The selected mesh operator is unavailable")?;
    if !source.outputs.iter().any(|port| port.name == target.channel && !port.cardinality.is_collection() && port.value_types.iter().any(|kind| kind == "mesh")) { return Err("Select a single indexed mesh output; convert B-Rep geometry to a mesh first".into()); }
    let next_kind = format!("brep.mesh.{operation}");
    let output = infos.get(&next_kind).and_then(|info| info.outputs.first()).ok_or("The mesh operation is unavailable")?;
    let base = format!("{}__{}", target.widget, operation);
    let mut id = base.clone();
    let mut suffix = 2;
    while host.host_snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == id) { id = format!("{base}_{suffix}"); suffix += 1; }
    let (x, y) = host.host_snapshot.layout.get(target.widget).map_or((0.0, 0.0), |layout| (layout.x, layout.y));
    host.add_widget(&serde_json::json!({"kind":"neuron","id":id,"neuronKind":next_kind}).to_string(), x + 220.0, y).map_err(|error| error.to_string())?;
    host.set_neuron_params(&id, params_json).map_err(|error| error.to_string())?;
    host.insert_between(target.widget, target.channel, &id, "mesh", &output.name).map_err(|error| error.to_string())?;
    for widget in &mut host.host_snapshot.widgets {
        if let Widget::Neuron { id: widget_id, preview, .. } = widget {
            if widget_id == &id { *preview = true; }
            if widget_id == target.widget { *preview = false; }
        }
    }
    Ok(id)
}

pub fn apply_selected(payload: &EditMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, ids: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    with_host(&doc.snapshot.host_snapshot, |host| {
        let id = insert_operation(host, payload, ids).map_err(Fault::from)?;
        Ok(Emit {
            artifact_mutations: commit_host_snapshot(&doc.snapshot.host_snapshot, &host.host_snapshot),
            interaction_writes: vec![InteractionWrite::replace(DOMAIN, payload.granularity(), std::iter::empty::<String>()), InteractionWrite::replace("graph", "node", [id])],
            ..Default::default()
        })
    })
}

pub fn handle(payload: &EditMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    apply_selected(payload, doc, &[])
}

pub fn delete_selected(doc: &ArtifactView<'_, Generation3dSnapshot>, ids: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    if ids.is_empty() { return Ok(Emit::default()); }
    apply_selected(&EditMeshSelection { operation: "deleteFaces".into(), amount: 0.0, cuts: 1, dx: 0.0, dy: 0.0, dz: 0.0 }, doc, ids)
}
