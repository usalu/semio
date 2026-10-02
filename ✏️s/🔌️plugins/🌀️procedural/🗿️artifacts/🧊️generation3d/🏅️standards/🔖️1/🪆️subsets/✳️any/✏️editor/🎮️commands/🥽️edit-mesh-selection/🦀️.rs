//! 🥽️ Selected mesh components become an editable operation in the generator graph: ONE edit that splices the typed
//! operator in with its DEFAULT params and then sets every input the user chose as an absolute `change-widget-input` leaf
//! (design §19), so a history edit changes "extrude distance = 0.1", never a whole operator record.

use crate::editor::generation3d::{config::{Generation3dConfig, Generation3dConfigMutation}, selection::{component_group, DOMAIN}};
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{change_widget_input, WidgetInputValue};
use crate::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host, mutations::text::Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
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

/// 🎛️ The inputs a mesh edit sets on its operator, validated: the selected components (as the operator's text list) and
/// the operation's own value — the offset, the cut count, or the extrude distance / inset amount.
pub fn inputs(payload: &EditMeshSelection, ids: &[String]) -> Result<Vec<(&'static str, WidgetInputValue)>, String> {
    let (_, components) = component_group(ids)?;
    let operation = payload.operation.as_str();
    if !matches!(operation, "extrude" | "inset" | "subdivide" | "flip" | "deleteFaces" | "moveVertices" | "loopCut") { return Err("Unknown mesh operation".into()); }
    if [payload.amount, payload.dx, payload.dy, payload.dz].iter().any(|value| !value.is_finite()) { return Err("Mesh edit values must be finite".into()); }
    if operation == "inset" && payload.amount <= 0.0 { return Err("Inset amount must be positive".into()); }
    if !(1..=256).contains(&payload.cuts) { return Err("Cuts must be an integer from 1 to 256".into()); }
    let field = match operation { "moveVertices" => "vertices", "loopCut" => "edges", _ => "faces" };
    let mut inputs = vec![(field, WidgetInputValue::Text(serde_json::to_string(&components).map_err(|error| error.to_string())?))];
    match operation {
        "moveVertices" => inputs.push(("offset", WidgetInputValue::Vector([payload.dx, payload.dy, payload.dz]))),
        "loopCut" => inputs.push(("cuts", WidgetInputValue::Number(f64::from(payload.cuts)))),
        "extrude" => inputs.push(("distance", WidgetInputValue::Number(payload.amount))),
        "inset" => inputs.push(("amount", WidgetInputValue::Number(payload.amount))),
        _ => {}
    }
    Ok(inputs)
}

/// 🧩️ The ONE edit a mesh operation is on `host_snapshot`: the splice rows that insert the typed operator with its
/// DEFAULT params into the selected output (consumers reconnected), then one `change-widget-input` per input the user
/// set — and the inserted operator's id.
pub(crate) fn mesh_operation_rows(host_snapshot: &FlowHostSnapshot, operation: &str, granularity: &str, ids: &[String], inputs: Vec<(&str, WidgetInputValue)>) -> Result<(String, Vec<Generation3dMutation>), String> {
    with_host(host_snapshot, |host| {
        let id = insert_mesh_operation(host, operation, granularity, ids)?;
        let mut rows = commit_host_snapshot(host_snapshot, &host.host_snapshot);
        rows.extend(inputs.into_iter().map(|(channel, input)| change_widget_input(id.clone(), channel, input)));
        Ok((id, rows))
    })
}

/// 🧩️ Splices one typed mesh operation, with its default params, into the selected output and reconnects its consumers.
fn insert_mesh_operation(host: &mut FlowHost, operation: &str, granularity: &str, ids: &[String]) -> Result<String, String> {
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
    host.insert_between(target.widget, target.channel, &id, "mesh", &output.name).map_err(|error| error.to_string())?;
    for widget in &mut host.host_snapshot.widgets {
        if let Widget::Neuron { id: widget_id, preview, .. } = widget {
            if widget_id == &id { *preview = true; }
            if widget_id == target.widget { *preview = false; }
        }
    }
    Ok(id)
}

/// 🧾️ The rows ONE mesh edit of the components `ids` is on `host_snapshot`, and the inserted operator's id.
pub fn edit_rows(payload: &EditMeshSelection, host_snapshot: &FlowHostSnapshot, ids: &[String]) -> Result<(String, Vec<Generation3dMutation>), String> {
    mesh_operation_rows(host_snapshot, &payload.operation, payload.granularity(), ids, inputs(payload, ids)?)
}

pub fn apply_selected(payload: &EditMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, ids: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let (id, rows) = edit_rows(payload, &doc.snapshot.host_snapshot, ids).map_err(Fault::from)?;
    Ok(Emit {
        artifact_mutations: rows,
        interaction_writes: vec![InteractionWrite::replace(DOMAIN, payload.granularity(), std::iter::empty::<String>()), InteractionWrite::replace("graph", "node", [id])],
        ..Default::default()
    })
}

pub fn handle(payload: &EditMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    apply_selected(payload, doc, &[])
}

pub fn delete_selected(doc: &ArtifactView<'_, Generation3dSnapshot>, ids: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    if ids.is_empty() { return Ok(Emit::default()); }
    apply_selected(&EditMeshSelection { operation: "deleteFaces".into(), amount: 0.0, cuts: 1, dx: 0.0, dy: 0.0, dz: 0.0 }, doc, ids)
}
