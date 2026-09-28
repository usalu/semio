//! 🎯️ Stable references to evaluated mesh instances and their editable components.

use std::collections::BTreeSet;

pub const DOMAIN: &str = "geometry";

pub fn edits_components(view: Option<&semio_framework_plugin::ViewModel>, granularity: Option<&str>) -> bool {
    let Some(view) = view else { return false };
    let kind = view.window_id.as_deref().and_then(|id| view.window_instances.iter().find(|window| window.id == id)).map(|window| window.window_kind_id.as_str()).or(view.active_window_kind_id.as_deref());
    kind == Some(super::edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW) && matches!(granularity, Some("vertex" | "edge" | "face"))
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentTarget<'a> {
    pub instance: &'a str,
    pub widget: &'a str,
    pub channel: &'a str,
    pub index: usize,
    pub granularity: &'a str,
    pub component: u32,
}

fn unsigned(text: &str) -> Option<u32> {
    if text.is_empty() || text.len() > 1 && text.starts_with('0') || !text.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
    text.parse().ok()
}

impl<'a> ComponentTarget<'a> {
    pub fn parse(id: &'a str) -> Option<Self> {
        let (prefix, component) = id.rsplit_once('.')?;
        let (instance, granularity) = prefix.rsplit_once('.')?;
        if !matches!(granularity, "vertex" | "edge" | "face") { return None; }
        let (port, index) = instance.rsplit_once('#')?;
        let (widget, channel) = port.split_once('@')?;
        if widget.is_empty() || channel.is_empty() || widget.contains('#') || channel.contains(['@', '#']) { return None; }
        Some(Self { instance, widget, channel, index: unsigned(index)? as usize, granularity, component: unsigned(component)? })
    }
}

pub fn component_group(ids: &[String]) -> Result<(ComponentTarget<'_>, Vec<u32>), String> {
    let first = ids.first().and_then(|id| ComponentTarget::parse(id)).ok_or("Select mesh faces, edges, or vertices in the preview")?;
    let mut components = BTreeSet::new();
    for id in ids {
        let target = ComponentTarget::parse(id).ok_or("The component selection contains an invalid target")?;
        if target.instance != first.instance || target.granularity != first.granularity { return Err("Select components of one mesh at the same granularity".into()); }
        components.insert(target.component);
    }
    Ok((first, components.into_iter().collect()))
}

fn mesh_component_vertices(mesh: &dsl::json::Value, mode: &str, ids: &[u32]) -> Option<BTreeSet<usize>> {
    if ids.is_empty() || ids.len() > 600_000 { return None; }
    let positions = mesh.get("vertices")?.as_array()?;
    let faces = mesh.get("faces")?.as_array()?;
    let mut vertices = BTreeSet::new();
    match mode {
        "vertex" => vertices.extend(ids.iter().map(|&id| id as usize)),
        "face" => {
            for &id in ids { for value in faces.get(id as usize)?.as_array()? { vertices.insert(value.as_u64()? as usize); } }
        }
        "edge" => {
            let selected = ids.iter().copied().collect::<BTreeSet<_>>();
            let mut halfedge = 0u32;
            let mut found = 0usize;
            for face in faces {
                let face = face.as_array()?;
                for (index, vertex) in face.iter().enumerate() {
                    if selected.contains(&halfedge) {
                        vertices.insert(vertex.as_u64()? as usize);
                        vertices.insert(face[(index + 1) % face.len()].as_u64()? as usize);
                        found += 1;
                    }
                    halfedge = halfedge.checked_add(1)?;
                }
            }
            if found != selected.len() { return None; }
        }
        _ => return None,
    }
    if vertices.is_empty() { return None; }
    for &id in &vertices {
        let point = positions.get(id)?.as_array()?;
        if point.len() != 3 { return None; }
        for axis in 0..3 {
            let coordinate = point.get(axis)?.as_f64()?;
            if !coordinate.is_finite() || coordinate.abs() > f32::MAX as f64 { return None; }
        }
    }
    Some(vertices)
}

/// 🥽️ Resolves an admissible single-mesh selection against its evaluated indexed topology.
pub fn selected_mesh_vertices(ids: &[String], data: &str) -> Result<Vec<usize>, String> {
    let (target, components) = component_group(ids)?;
    if target.index != 0 { return Err("Extract one mesh from the list before editing its components".into()); }
    if data.len() > 16_000_000 { return Err("The selected mesh exceeds the supported input size".into()); }
    let mesh = dsl::json::parse(data).map_err(|_| "The selected mesh is invalid")?;
    mesh_component_vertices(&mesh, target.granularity, &components).map(|vertices| vertices.into_iter().collect()).ok_or_else(|| "The selected mesh components no longer exist".into())
}

/// 📍️ Resolves the gumball center from indexed topology, including coincident but distinct vertices.
pub fn component_pivot(data: &str, mode: &str, ids: &[u32]) -> Option<[f64; 3]> {
    if data.len() > 16_000_000 { return None; }
    let mesh = dsl::json::parse(data).ok()?;
    let vertices = mesh_component_vertices(&mesh, mode, ids)?;
    let positions = mesh.get("vertices")?.as_array()?;
    let mut sum = [0.0; 3];
    for &id in &vertices {
        let point = positions.get(id)?.as_array()?;
        for axis in 0..3 { sum[axis] += point[axis].as_f64()? as f32 as f64; }
    }
    Some(sum.map(|value| (value / vertices.len() as f64) as f32 as f64))
}

/// 🚦️ Reads only current cached geometry; pending component transforms inherit their source topology.
pub fn validate_cached_components(snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &mut semio_framework_os_flow::FlowEvalSession, ids: &[String]) -> Result<(), String> {
    let (target, _) = component_group(ids)?;
    if target.index != 0 { return Err("Extract one mesh from the list before editing its components".into()); }
    crate::standards::v1::subsets::any::schema::with_host_session(snapshot, session, |host, _| {
        let pending = host.evaluate_step(semio_framework_os_flow::neural::EvalStepBudget::PROBE).into_iter().collect::<BTreeSet<_>>();
        let mut widget = target.widget;
        let mut channel = target.channel;
        let mut visited = BTreeSet::new();
        while visited.insert(widget) {
            if !pending.contains(widget) {
                if let Some(mesh) = host.outputs.get(widget).and_then(|output| output.get(channel)).and_then(|value| value.as_dictionary()).filter(|mesh| mesh.schema() == Some("mesh")) {
                    let data = mesh.get("data").and_then(|value| value.as_atom()).and_then(|value| value.as_str()).ok_or("The selected mesh has no indexed topology")?;
                    selected_mesh_vertices(ids, data)?;
                    return Ok(());
                }
            }
            let Some(semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. }) = snapshot.widgets.iter().find(|item| crate::widget_id(item) == widget) else { break };
            if channel != "meshOut" || !matches!(neuron_kind.as_str(), "brep.mesh.translateComponents" | "brep.mesh.rotateComponents" | "brep.mesh.scaleComponents") { break; }
            let mut wires = snapshot.synapses.iter().filter(|wire| wire.to == widget && wire.to_port == "mesh");
            let Some(source) = wires.next() else { break };
            if wires.next().is_some() { break; }
            widget = &source.from;
            channel = &source.from_port;
        }
        Err("The selected mesh is still being evaluated; wait for its preview before editing its components".into())
    })
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComponentSelection {
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Option<String>,
}

impl ComponentSelection {
    pub fn active(&self) -> bool { matches!(self.granularity.as_str(), "vertex" | "edge" | "face") }
    pub fn from_interaction(interaction: &semio_framework_plugin::app::InteractionView<'_>) -> Self {
        Self {
            granularity: interaction.active_granularity(DOMAIN).unwrap_or("object").to_string(),
            selected: interaction.selection(DOMAIN).ids.clone(),
            hovered: interaction.hover(DOMAIN, "pointer").ids.first().cloned(),
        }
    }

    pub fn project(&self, instances: &dsl::json::Value, value: &mut dsl::json::Object) {
        if !self.active() { return; }
        let visible = |id: &str| instances.as_array().is_some_and(|items| items.iter().any(|item| item.get("id").and_then(|id| id.as_str()) == Some(id)));
        let selected = self.selected.iter().filter_map(|id| ComponentTarget::parse(id)).filter(|target| target.granularity == self.granularity && visible(target.instance)).collect::<Vec<_>>();
        let hovered = self.hovered.as_deref().and_then(ComponentTarget::parse).filter(|target| target.granularity == self.granularity && visible(target.instance));
        let active = selected.first().map(|target| target.instance).or_else(|| hovered.as_ref().map(|target| target.instance));
        value.insert("selectionMode", dsl::json::Value::String(self.granularity.clone()));
        value.insert("granularity", dsl::json::Value::String(self.granularity.clone()));
        value.insert("gumballActive", dsl::json::Value::Bool(false));
        value.insert("showEdges", dsl::json::Value::Bool(true));
        value.insert("activeObjectId", active.map_or(dsl::json::Value::Null, |id| dsl::json::Value::String(id.into())));
        value.insert("ids", dsl::json::Value::Array(selected.iter().map(|target| target.instance).collect::<BTreeSet<_>>().into_iter().map(|id| dsl::json::Value::String(id.into())).collect()));
        value.insert("componentIds", dsl::json::Value::Array(selected.iter().filter(|target| Some(target.instance) == active).map(|target| target.component).collect::<BTreeSet<_>>().into_iter().map(dsl::json::Value::from).collect()));
        let mut targets = dsl::json::Object::new();
        for mode in ["mesh", "vertex", "edge", "face"] { targets.insert(mode, dsl::json::Value::Bool(mode == self.granularity)); }
        value.insert("targets", dsl::json::Value::Object(targets));
        if let Some(target) = hovered {
            let mut hover = dsl::json::Object::new();
            hover.insert("objectId", dsl::json::Value::String(target.instance.into()));
            hover.insert("mode", dsl::json::Value::String(target.granularity.into()));
            hover.insert("id", dsl::json::Value::from(target.component));
            value.insert("hoveredComponent", dsl::json::Value::Object(hover));
        }
    }
}

pub fn engagement(selection: &ComponentSelection, is_de: bool) -> semio_framework_plugin::WindowEngagement {
    let options = [("object", "box", "Objects", "Objekte"), ("vertex", "circle", "Vertices", "Eckpunkte"), ("edge", "minus", "Edges", "Kanten"), ("face", "square", "Faces", "Flächen")]
        .into_iter().map(|(mode, icon, en, de)| semio_framework_plugin::WindowEngagementOption {
            id: format!("procedural.select-{mode}"),
            label: Some(if is_de { de } else { en }.into()),
            icon_id: Some(icon.into()),
            pressed: Some(if mode == "object" { !selection.active() } else { selection.granularity == mode }),
            disabled: None,
            action: Some(super::generation3d_action("setInteractionGranularity", Some(semio_framework_plugin::dsl_value!({"domainId": DOMAIN, "granularityId": mode})))),
        }).collect();
    let edits: &[(&str, &str, &str)] = match selection.granularity.as_str() {
        "face" => &[("extrude", "Extrude 0.1", "0,1 extrudieren"), ("inset", "Inset 0.1", "0,1 einziehen"), ("subdivide", "Subdivide", "Unterteilen"), ("flip", "Flip", "Umkehren"), ("deleteFaces", "Delete Faces", "Flächen löschen")],
        "edge" => &[("loopCut", "Cut Loop", "Schleife schneiden")],
        _ => &[],
    };
    let actions = if component_group(&selection.selected).is_ok() {
        edits.iter().map(|&(operation, en, de)| semio_framework_plugin::WindowEngagementPossible {
                id: format!("procedural.mesh-{operation}"), label: if is_de { de } else { en }.into(),
                detail: Some(if is_de { "Danach im Inspektor einstellbar" } else { "Adjustable in the inspector afterwards" }.into()),
                action: Some(super::generation3d_action("editMeshSelection", Some(semio_framework_plugin::dsl_value!({"operation":operation,"amount":0.1,"cuts":1,"dx":0.0,"dy":0.0,"dz":0.0})))),
            }).collect()
    } else { Vec::new() };
    let status = if selection.active() {
        Some(vec![semio_framework_plugin::WindowEngagementStatus {
            id: "procedural.component-selection".into(),
            text: if is_de { format!("{} ausgewählt · Netzauswahl bearbeiten: Strg/⌘+Umschalt+M", selection.selected.len()) } else { format!("{} selected · Edit Mesh Selection: Ctrl/⌘+Shift+M", selection.selected.len()) },
        }])
    } else { None };
    semio_framework_plugin::WindowEngagement { session_active: Some(true), options: Some(options), input: None, control: None, controls: None, status, possible_engagements: Some(actions) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
