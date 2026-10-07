//! 🥽️ Selected mesh components become an editable operation in the generator graph: ONE edit that splices the typed
//! operator in with its DEFAULT params and then sets every input the user chose as an absolute `change-widget-input` leaf
//! (design §19), so a history edit changes "extrude distance = 0.1", never a whole operator record.

use crate::editor::generation3d::{config::{Generation3dConfig, Generation3dConfigMutation}, selection::{component_group, selected_analytic_labels, DOMAIN}};
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{change_widget_input, WidgetInputValue};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, InteractionWrite};
use semio_framework_tool_machine::{authoring_clock, Scrub, ScrubInput, ToolStep};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit-mesh-selection")]
#[value(rename_all = "camelCase")]
pub struct EditMeshSelection {
    pub operation: String,
    pub amount: f64,
    pub cuts: u32,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub width: f64,
    pub segments: u32,
    pub merge_mode: String,
    pub tolerance: f64,
    pub radius: f64,
    pub grid: f64,
    pub center: [f64; 3],
}

impl Default for EditMeshSelection {
    fn default() -> Self { Self { operation: "extrude".into(), amount: 0.1, cuts: 1, dx: 0.0, dy: 0.0, dz: 0.0, width: 0.1, segments: 1, merge_mode: "center".into(), tolerance: 0.0001, radius: 1.0, grid: 1.0, center: [0.0; 3] } }
}

impl EditMeshSelection {
    pub fn granularity(&self) -> &'static str { match self.operation.as_str() { "moveVertices" | "dissolveVertices" | "mergeVertices" | "moveProportional" | "snapVertices" => "vertex", "loopCut" | "bevel" | "dissolveEdges" | "filletEdges" | "chamferEdges" => "edge", _ => "face" } }
}

/// 🎛️ The inputs a mesh edit sets on its operator, validated: the selected components (as the operator's text list) and
/// the operation's own value — the offset, the cut count, or the extrude distance / inset amount.
pub fn inputs(payload: &EditMeshSelection, ids: &[String]) -> Result<Vec<(&'static str, WidgetInputValue)>, String> {
    let (target, components) = component_group(ids)?;
    if target.granularity != payload.granularity() || target.analytic.is_none() && target.index != 0 { return Err("Select matching components of a single mesh".into()); }
    let operation = payload.operation.as_str();
    if !matches!(operation, "extrude" | "inset" | "subdivide" | "flip" | "deleteFaces" | "moveVertices" | "loopCut" | "bevel" | "dissolveEdges" | "dissolveVertices" | "mergeVertices" | "moveProportional" | "snapVertices" | "filletEdges" | "chamferEdges" | "shell") { return Err("Unknown mesh operation".into()); }
    if [payload.amount, payload.dx, payload.dy, payload.dz, payload.width, payload.tolerance, payload.radius, payload.grid, payload.center[0], payload.center[1], payload.center[2]].iter().any(|value| !value.is_finite()) { return Err("Mesh edit values must be finite".into()); }
    if matches!(operation, "inset" | "chamferEdges" | "shell") && payload.amount <= 0.0 { return Err("Inset amount must be positive".into()); }
    if !(1..=256).contains(&payload.cuts) { return Err("Cuts must be an integer from 1 to 256".into()); }
    if payload.width <= 0.0 || payload.radius <= 0.0 || payload.grid <= 0.0 { return Err("Width, radius, and grid must be positive".into()); }
    if !(1..=64).contains(&payload.segments) { return Err("Bevel segments must be an integer from 1 to 64".into()); }
    if payload.tolerance < 0.0 || !matches!(payload.merge_mode.as_str(), "first" | "center" | "distance") { return Err("Merge mode must be first, center, or distance with nonnegative tolerance".into()); }
    if matches!(operation, "filletEdges" | "chamferEdges" | "shell") {
        let labels = selected_analytic_labels(ids)?;
        let selection = WidgetInputValue::Text(semio_framework_pack_json::to_string(&semio_framework_pack_json::array(labels.into_iter().map(semio_framework_pack_json::Value::String))));
        let (channel, parameter, value) = match operation { "filletEdges" => ("edgeLabels", "radius", payload.radius), "chamferEdges" => ("edgeLabels", "distance", payload.amount), _ => ("faceLabels", "thickness", payload.amount) };
        return Ok(vec![(channel, selection), (parameter, WidgetInputValue::Number(value))]);
    }
    if target.analytic.is_some() { return Err("Analytic components require a B-Rep operation".into()); }
    let field = match operation { "moveVertices" => "vertices", "loopCut" | "bevel" | "dissolveEdges" => "edges", "dissolveVertices" | "mergeVertices" | "moveProportional" | "snapVertices" => "selection", _ => "faces" };
    let mut inputs = vec![(field, WidgetInputValue::Text(semio_framework_pack_json::to_string(&semio_framework_pack_json::array(components.into_iter().map(semio_framework_pack_json::Value::from)))))];
    match operation {
        "moveVertices" | "moveProportional" => inputs.push(("offset", WidgetInputValue::Vector([payload.dx, payload.dy, payload.dz]))),
        "loopCut" => inputs.push(("cuts", WidgetInputValue::Number(f64::from(payload.cuts)))),
        "extrude" => inputs.push(("distance", WidgetInputValue::Number(payload.amount))),
        "inset" => inputs.push(("amount", WidgetInputValue::Number(payload.amount))),
        _ => {}
    }
    match operation {
        "bevel" => inputs.extend([("amount", WidgetInputValue::Number(payload.width)), ("segments", WidgetInputValue::Number(f64::from(payload.segments)))]),
        "mergeVertices" => inputs.extend([("mode", WidgetInputValue::Text(payload.merge_mode.clone())), ("tolerance", WidgetInputValue::Number(payload.tolerance))]),
        "moveProportional" => inputs.extend([("center", WidgetInputValue::Vector(payload.center)), ("radius", WidgetInputValue::Number(payload.radius))]),
        "snapVertices" => inputs.push(("grid", WidgetInputValue::Number(payload.grid))),
        _ => {}
    }
    Ok(inputs)
}

/// 🧩️ The ONE edit a mesh operation is on `host_snapshot`: the splice rows that insert the typed operator with its
/// DEFAULT params into the selected output (consumers reconnected), then one `change-widget-input` per input the user
/// set — and the inserted operator's id.
pub(crate) fn mesh_operation_rows(host_snapshot: &FlowHostSnapshot, operation: &str, granularity: &str, ids: &[String], inputs: Vec<(&str, WidgetInputValue)>) -> Result<(String, Vec<Generation3dMutation>), String> {
    let (target, _) = component_group(ids)?;
    with_host(host_snapshot, |host| {
        if let Some(source) = &target.analytic {
            let (id, selector) = insert_brep_operation(host, operation, granularity, &target)?;
            let mut rows = commit_host_snapshot(host_snapshot, &host.host_snapshot);
            rows.push(change_widget_input(selector.clone(), "sourceHandle", WidgetInputValue::Text(source.handle.to_string())));
            rows.extend(inputs.into_iter().map(|(channel, input)| change_widget_input(if matches!(channel, "edgeLabels" | "faceLabels") { selector.clone() } else { id.clone() }, channel, input)));
            return Ok((id, rows));
        }
        let id = insert_mesh_operation(host, operation, granularity, ids)?;
        let mut rows = commit_host_snapshot(host_snapshot, &host.host_snapshot);
        rows.extend(inputs.into_iter().map(|(channel, input)| change_widget_input(id.clone(), channel, input)));
        Ok((id, rows))
    })
}

/// 🧩️ Inserts a source-scoped topology selector and feature through existing graph mutations.
fn insert_brep_operation(host: &mut FlowHost, operation: &str, granularity: &str, target: &crate::editor::generation3d::selection::ComponentTarget<'_>) -> Result<(String, String), String> {
    if target.granularity != granularity || !matches!(operation, "filletEdges" | "chamferEdges" | "shell") { return Err("The B-Rep operation requires matching edge or face selection".into()); }
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let source_kind = host.host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::Neuron { id, neuron_kind, .. } if id == target.widget => Some(neuron_kind), _ => None }).ok_or("The selected geometry no longer exists")?;
    let source = infos.get(source_kind).and_then(|info| info.outputs.iter().find(|port| port.name == target.channel)).ok_or("The selected geometry output is unavailable")?;
    let collection = source.cardinality.is_collection() || source.value_types.iter().any(|kind| kind == "list") && !source.value_types.iter().any(|kind| kind == "geometry");
    if !source.value_types.iter().any(|kind| matches!(kind.as_str(), "geometry" | "list")) || !collection && target.index != 0 { return Err("Select components of one current B-Rep geometry output".into()); }
    let feature_kind = format!("brep.solid.{operation}");
    let output = infos.get(&feature_kind).and_then(|info| info.outputs.first()).ok_or("The B-Rep operation is unavailable")?;
    let source_output = infos.get("brep.brep").and_then(|info| info.outputs.iter().find(|port| port.value_types == ["geometry"])).ok_or("The topology source output is unavailable")?;
    let consumers: Vec<_> = host.host_snapshot.synapses.iter().filter(|wire| wire.from == target.widget && wire.from_port == target.channel).map(|wire| (wire.to.clone(), wire.to_port.clone())).collect();
    let (x, y) = host.host_snapshot.layout.get(target.widget).map_or((0.0, 0.0), |layout| (layout.x, layout.y));
    let selector = add_edit_widget(host, &format!("{}__selected", target.widget), "brep.brep", x + 220.0, y)?;
    let feature = add_edit_widget(host, &format!("{}__{operation}", target.widget), &feature_kind, x + 440.0, y)?;
    host.connect_ports(target.widget, target.channel, &selector, "brep").map_err(|error| error.to_string())?;
    host.connect_ports(&selector, &source_output.name, &feature, "geometry").map_err(|error| error.to_string())?;
    let (selection, input) = if granularity == "edge" { ("selectedEdges", "edges") } else { ("selectedFaces", "openFaces") };
    host.connect_ports(&selector, selection, &feature, input).map_err(|error| error.to_string())?;
    let replacement = if collection {
        let id = add_edit_widget(host, &format!("{}__replace", target.widget), "list.set", x + 660.0, y)?;
        host.connect_ports(target.widget, target.channel, &id, "list").map_err(|error| error.to_string())?;
        host.connect_ports(&selector, "sourceIndex", &id, "index").map_err(|error| error.to_string())?;
        host.connect_ports(&feature, &output.name, &id, "value").map_err(|error| error.to_string())?;
        let channel = infos.get("list.set").and_then(|info| info.outputs.first()).ok_or("The collection replacement operation is unavailable")?.name.clone();
        (id, channel)
    } else { (feature.clone(), output.name.clone()) };
    for (consumer, input) in consumers { host.connect_ports(&replacement.0, &replacement.1, &consumer, &input).map_err(|error| error.to_string())?; }
    for widget in &mut host.host_snapshot.widgets {
        if let Widget::Neuron { id, preview, .. } = widget {
            if id == &replacement.0 { *preview = true; }
            if id == target.widget { *preview = false; }
        }
    }
    Ok((feature, selector))
}

/// 🏷️ Creates a uniquely named edit widget through the existing host authoring operation.
fn add_edit_widget(host: &mut FlowHost, base: &str, kind: &str, x: f64, y: f64) -> Result<String, String> {
    let mut id = base.to_string();
    let mut suffix = 2;
    while host.host_snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == id) { id = format!("{base}_{suffix}"); suffix += 1; }
    host.add_widget(&serde_json::json!({"kind":"neuron","id":id,"neuronKind":kind}).to_string(), x, y).map_err(|error| error.to_string())?;
    Ok(id)
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

/// 🛠️ ONE mesh edit as ONE tool transaction of `<app>#<verb>` (design §19.1; its intent is the `create-widget` that inserts
/// the operator, so the row reads "Insert …"): the rows commit one-shot through the framework's scrub machine at rest, the
/// ref minted from the admission's authoring seed; a view without command authority publishes them plainly.
pub(crate) fn mesh_edit_emit(verb: &str, doc: &ArtifactView<'_, Generation3dSnapshot>, rows: Vec<Generation3dMutation>) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let seed = doc.operation().map(|operation| operation.authoring_seed.clone()).unwrap_or_default();
    let tool = format!("{}#{verb}", crate::editor::generation3d::GENERATION3D_EDITOR_APP_ID);
    match Scrub::start(tool, protocol::ActorId(seed.clone()), "").send(ScrubInput::Commit { gesture: verb.to_string(), leaves: rows }, authoring_clock(0)) {
        Ok(ToolStep::Committed(transaction, rows)) if !seed.is_empty() => Emit::commit_transaction(transaction, rows),
        Ok(ToolStep::Committed(_, rows)) => Emit::mutations(rows),
        _ => Emit::default(),
    }
}

pub fn apply_selected(verb: &str, payload: &EditMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, ids: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let (id, rows) = edit_rows(payload, &doc.snapshot.host_snapshot, ids).map_err(Fault::from)?;
    Ok(Emit { interaction_writes: vec![InteractionWrite::replace(DOMAIN, payload.granularity(), std::iter::empty::<String>()), InteractionWrite::replace("graph", "node", [id])], ..mesh_edit_emit(verb, doc, rows) })
}

pub fn handle(payload: &EditMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    apply_selected("editMeshSelection", payload, doc, &[])
}

pub fn delete_selected(doc: &ArtifactView<'_, Generation3dSnapshot>, ids: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    if ids.is_empty() { return Ok(Emit::default()); }
    apply_selected("deleteSelection", &EditMeshSelection { operation: "deleteFaces".into(), amount: 0.0, ..Default::default() }, doc, ids)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
