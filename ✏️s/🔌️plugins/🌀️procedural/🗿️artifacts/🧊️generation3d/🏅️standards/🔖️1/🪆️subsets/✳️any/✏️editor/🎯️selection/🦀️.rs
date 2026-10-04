//! 🎯️ Stable references to evaluated mesh instances and their editable components.

use std::collections::BTreeSet;

pub const DOMAIN: &str = "geometry";

pub fn edits_components(view: Option<&semio_framework_plugin::ViewModel>, granularity: Option<&str>) -> bool {
    let Some(view) = view else { return false };
    let kind = view.window_id.as_deref().and_then(|id| view.window_instances.iter().find(|window| window.id == id)).map(|window| window.window_kind_id.as_str()).or(view.active_window_kind_id.as_deref());
    kind == Some(super::edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW) && matches!(granularity, Some("vertex" | "edge" | "face"))
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnalyticComponent<'a> {
    pub handle: &'a str,
    pub label: &'a str,
    pub revision: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentTarget<'a> {
    pub instance: &'a str,
    pub widget: &'a str,
    pub channel: &'a str,
    pub index: usize,
    pub granularity: &'a str,
    pub component: u32,
    pub analytic: Option<AnalyticComponent<'a>>,
}

fn unsigned(text: &str) -> Option<u32> {
    if text.is_empty() || text.len() > 1 && text.starts_with('0') || !text.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
    text.parse().ok()
}

impl<'a> ComponentTarget<'a> {
    pub fn interaction_id(&self) -> String {
        let id = format!("{}.{}.{}", self.instance, self.granularity, self.component);
        self.analytic.as_ref().map_or_else(|| id.clone(), |source| format!("{id}~{}~{}~{}", source.handle, source.label, source.revision))
    }

    pub fn parse(id: &'a str) -> Option<Self> {
        let mut parts = id.split('~');
        let id = parts.next()?;
        let analytic = if let Some(handle) = parts.next() {
            let label = parts.next()?;
            let revision = parts.next()?;
            let hex = |value: &str| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
            if parts.next().is_some() || !hex(handle) || !hex(revision) || label.is_empty() || label.starts_with('0') || !label.bytes().all(|byte| byte.is_ascii_digit()) || label.parse::<u64>().is_err() { return None; }
            Some(AnalyticComponent { handle, label, revision })
        } else { None };
        let (prefix, component) = id.rsplit_once('.')?;
        let (instance, granularity) = prefix.rsplit_once('.')?;
        if !matches!(granularity, "vertex" | "edge" | "face") { return None; }
        let (port, index) = instance.rsplit_once('#')?;
        let (widget, channel) = port.split_once('@')?;
        if widget.is_empty() || channel.is_empty() || widget.contains('#') || channel.contains(['@', '#']) { return None; }
        Some(Self { instance, widget, channel, index: unsigned(index)? as usize, granularity, component: unsigned(component)?, analytic })
    }
}

pub fn component_group(ids: &[String]) -> Result<(ComponentTarget<'_>, Vec<u32>), String> {
    let first = ids.first().and_then(|id| ComponentTarget::parse(id)).ok_or("Select mesh faces, edges, or vertices in the preview")?;
    let mut components = BTreeSet::new();
    let mut labels = std::collections::BTreeMap::new();
    for id in ids {
        let target = ComponentTarget::parse(id).ok_or("The component selection contains an invalid target")?;
        if target.instance != first.instance || target.granularity != first.granularity || target.analytic.as_ref().map(|value| (value.handle,value.revision)) != first.analytic.as_ref().map(|value| (value.handle,value.revision)) { return Err("Select components of one evaluated geometry at the same granularity".into()); }
        if let Some(reference) = &target.analytic {
            if labels.insert(target.component,reference.label).is_some_and(|previous| previous != reference.label) { return Err("A renderer group has inconsistent component identity".into()); }
        }
        components.insert(target.component);
    }
    Ok((first, components.into_iter().collect()))
}

fn mesh_component_vertices(mesh: &semio_framework_pack_json::Value, mode: &str, ids: &[u32]) -> Option<BTreeSet<usize>> {
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
    if target.analytic.is_some() { return Err("Analytic components require a B-Rep operation".into()); }
    if target.index != 0 { return Err("Extract one mesh from the list before editing its components".into()); }
    if data.len() > 16_000_000 { return Err("The selected mesh exceeds the supported input size".into()); }
    let mesh = semio_framework_pack_json::parse(data, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| "The selected mesh is invalid")?;
    mesh_component_vertices(&mesh, target.granularity, &components).map(|vertices| vertices.into_iter().collect()).ok_or_else(|| "The selected mesh components no longer exist".into())
}

/// 📍️ Resolves the gumball center from indexed topology, including coincident but distinct vertices.
pub fn component_pivot(data: &str, mode: &str, ids: &[u32]) -> Option<[f64; 3]> {
    if data.len() > 16_000_000 { return None; }
    let mesh = semio_framework_pack_json::parse(data, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
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
pub fn validate_cached_components(snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &mut semio_framework_os_flow::FlowEvalSession, ids: &[String], revision: Option<&[u8; 32]>) -> Result<(), String> {
    let (target, _) = component_group(ids)?;
    if target.analytic.is_none() && target.index != 0 { return Err("Extract one mesh from the list before editing its components".into()); }
    crate::standards::v1::subsets::any::schema::with_host_session(snapshot, session, |host, session| {
        let pending = host.evaluate_step(semio_framework_os_flow::neural::EvalStepBudget::PROBE,&|_|true).into_iter().collect::<BTreeSet<_>>();
        if let Some(source) = &target.analytic {
            if pending.contains(target.widget) { return Err("The selected geometry is still being evaluated".into()); }
            let value = host.outputs.get(target.widget).and_then(|output| output.get(target.channel)).and_then(|value| value.as_dictionary()).ok_or("The selected geometry no longer exists")?;
            let value = if value.schema() == Some("list") { value.get(&target.index.to_string()).and_then(|value| value.as_dictionary()).ok_or("The selected geometry list item no longer exists")? } else if target.index == 0 { value } else { return Err("The selected geometry list item no longer exists".into()); };
            let handle = value.get("handle").and_then(|value| value.as_atom()).and_then(|value| value.as_str()).ok_or("The selected geometry has no analytic source")?;
            if handle != source.handle { return Err("The selected geometry has changed; select its current components".into()); }
            let pack = session.preview_mesh_pack(handle).and_then(super::decode_preview_mesh_pack).ok_or("The selected geometry preview is not current")?;
            let revision = revision.ok_or("The selected analytic edit has no authoring revision")?.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
            validate_analytic_source(ids, &revision, handle, &pack.component_references)?;
            return Ok(());
        }
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

fn projected_target<'a>(mut target: ComponentTarget<'a>, instances: &semio_framework_pack_json::Value, meshes: &semio_framework_pack_json::Value) -> Option<ComponentTarget<'a>> {
    let instance = instances.as_array()?.iter().find(|item| item.get("id").and_then(|id| id.as_str()) == Some(target.instance))?;
    if let Some(source) = &target.analytic {
        let current = instance.get("componentSource")?;
        if current.get("handle")?.as_str()? != source.handle || current.get("revision")?.as_str()? != source.revision { return None; }
        let mesh_id = instance.get("meshId")?.as_str()?;
        let references = meshes.as_array()?.iter().find(|mesh| mesh.get("id").and_then(|id| id.as_str()) == Some(mesh_id))?.get("data")?.get("componentReferences")?.get(target.granularity)?.as_array()?;
        let mut matches = references.iter().enumerate().filter(|(_, value)| value.as_str() == Some(source.label));
        target.component = u32::try_from(matches.next()?.0).ok()?;
        if matches.next().is_some() { return None; }
    }
    Some(target)
        
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

    pub fn project(&self, instances: &semio_framework_pack_json::Value, meshes: &semio_framework_pack_json::Value, value: &mut semio_framework_pack_json::Object) {
        if !self.active() { return; }
        let visible = |target| projected_target(target, instances, meshes);
        let selected = self.selected.iter().filter_map(|id| ComponentTarget::parse(id)).filter(|target| target.granularity == self.granularity).filter_map(visible).collect::<Vec<_>>();
        let hovered = self.hovered.as_deref().and_then(ComponentTarget::parse).filter(|target| target.granularity == self.granularity).and_then(visible);
        let active = selected.first().map(|target| target.instance).or_else(|| hovered.as_ref().map(|target| target.instance));
        value.insert("selectionMode", semio_framework_pack_json::Value::String(self.granularity.clone()));
        value.insert("granularity", semio_framework_pack_json::Value::String(self.granularity.clone()));
        value.insert("gumballActive", semio_framework_pack_json::Value::Bool(false));
        value.insert("showEdges", semio_framework_pack_json::Value::Bool(true));
        value.insert("activeObjectId", active.map_or(semio_framework_pack_json::Value::Null, |id| semio_framework_pack_json::Value::String(id.into())));
        value.insert("ids", semio_framework_pack_json::Value::Array(selected.iter().map(|target| target.instance).collect::<BTreeSet<_>>().into_iter().map(|id| semio_framework_pack_json::Value::String(id.into())).collect()));
        value.insert("componentIds", semio_framework_pack_json::Value::Array(selected.iter().filter(|target| Some(target.instance) == active).map(|target| target.component).collect::<BTreeSet<_>>().into_iter().map(semio_framework_pack_json::Value::from).collect()));
        value.insert("gumballSelectionIds", semio_framework_pack_json::Value::Array(selected.iter().map(|target| target.interaction_id()).collect::<BTreeSet<_>>().into_iter().map(semio_framework_pack_json::Value::String).collect()));
        let mut targets = semio_framework_pack_json::Object::new();
        for mode in ["mesh", "vertex", "edge", "face"] { targets.insert(mode, semio_framework_pack_json::Value::Bool(mode == self.granularity)); }
        value.insert("targets", semio_framework_pack_json::Value::Object(targets));
        if let Some(target) = hovered {
            let mut hover = semio_framework_pack_json::Object::new();
            hover.insert("objectId", semio_framework_pack_json::Value::String(target.instance.into()));
            hover.insert("mode", semio_framework_pack_json::Value::String(target.granularity.into()));
            hover.insert("id", semio_framework_pack_json::Value::from(target.component));
            value.insert("hoveredComponent", semio_framework_pack_json::Value::Object(hover));
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
    let analytic = component_group(&selection.selected).is_ok_and(|(target, _)| target.analytic.is_some());
    let edits: &[(&str, &str, &str)] = if analytic { match selection.granularity.as_str() {
        "edge" => &[("filletEdges", "Fillet 0.1", "0,1 verrunden"), ("chamferEdges", "Chamfer 0.1", "0,1 fasen")],
        "face" => &[("shell", "Shell 0.1", "0,1 aushöhlen")],
        _ => &[],
    } } else { match selection.granularity.as_str() {
        "face" => &[("extrude", "Extrude 0.1", "0,1 extrudieren"), ("inset", "Inset 0.1", "0,1 einziehen"), ("subdivide", "Subdivide", "Unterteilen"), ("flip", "Flip", "Umkehren"), ("deleteFaces", "Delete Faces", "Flächen löschen")],
        "edge" => &[("loopCut", "Cut Loop", "Schleife schneiden"), ("bevel", "Bevel 0.1", "0,1 abschrägen"), ("dissolveEdges", "Dissolve Edges", "Kanten auflösen")],
        "vertex" => &[("moveVertices", "Move Vertices", "Eckpunkte verschieben"), ("dissolveVertices", "Dissolve Vertices", "Eckpunkte auflösen"), ("mergeVertices", "Merge at Center", "Im Mittelpunkt zusammenführen"), ("moveProportional", "Move Proportionally", "Proportional verschieben"), ("snapVertices", "Snap to Grid", "Am Raster ausrichten")],
        _ => &[],
    } };
    let actions = if component_group(&selection.selected).is_ok() {
        edits.iter().map(|&(operation, en, de)| semio_framework_plugin::WindowEngagementPossible {
                id: format!("procedural.{}-{operation}", if analytic { "brep" } else { "mesh" }), label: if is_de { de } else { en }.into(),
                detail: Some(if is_de { "Danach im Inspektor einstellbar" } else { "Adjustable in the inspector afterwards" }.into()),
                action: Some(super::generation3d_action("editMeshSelection", Some(semio_framework_plugin::dsl_value!({"operation":operation,"amount":0.1,"cuts":1,"dx":0.0,"dy":0.0,"dz":0.0,"width":0.1,"segments":1,"mergeMode":"center","tolerance":0.0001,"radius":if analytic { 0.1 } else { 1.0 },"grid":1.0,"center":[0.0,0.0,0.0]})))),
            }).collect()
    } else { Vec::new() };
    let status = if selection.active() {
        Some(vec![semio_framework_plugin::WindowEngagementStatus {
            id: "procedural.component-selection".into(),
            text: if is_de { format!("{} ausgewählt · Geometrieauswahl bearbeiten: Strg/⌘+Umschalt+M", selection.selected.len()) } else { format!("{} selected · Edit Geometry Selection: Ctrl/⌘+Shift+M", selection.selected.len()) },
        }])
    } else { None };
    semio_framework_plugin::WindowEngagement { session_active: Some(true), options: Some(options), input: None, control: None, controls: None, status, possible_engagements: Some(actions) }
}

/// 🏷️ Exact labels are scoped to one painted source and authoring revision.
pub fn selected_analytic_labels(ids: &[String]) -> Result<Vec<String>, String> {
    component_group(ids)?;
    let mut labels = std::collections::BTreeMap::new();
    for id in ids {
        let target = ComponentTarget::parse(id).ok_or("The component selection contains an invalid target")?;
        let reference = target.analytic.filter(|_| target.granularity != "vertex").ok_or("Select analytic faces or edges")?;
        labels.insert(reference.label.parse::<u64>().map_err(|_| "The component label is invalid")?,reference.label.to_string());
    }
    Ok(labels.into_values().collect())
}

/// 🚦️ Admits exact component labels only against the current evaluated source.
pub fn validate_analytic_source(ids: &[String], revision: &str, handle: &str, references: &std::collections::BTreeMap<String, Vec<String>>) -> Result<Vec<String>, String> {
    let labels = selected_analytic_labels(ids)?;
    let target = ComponentTarget::parse(&ids[0]).ok_or("The component selection contains an invalid target")?;
    let source = target.analytic.ok_or("Select analytic faces or edges")?;
    if source.revision != revision || source.handle != handle { return Err("The selected geometry has changed; select its current components".into()); }
    let mut counts = std::collections::BTreeMap::new();
    for label in references.get(target.granularity).into_iter().flatten() { *counts.entry(label.as_str()).or_insert(0usize) += 1; }
    if labels.iter().any(|label| counts.get(label.as_str()) != Some(&1)) { return Err("The selected components no longer exist or are ambiguous".into()); }
    Ok(labels)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
