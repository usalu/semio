//! 🧭️ Atomic graph transforms and retained selection for viewport gestures.

use crate::editor::generation3d::config::Generation3dConfigMutation;
use crate::standards::v1::subsets::any::schema::{commit_host_snapshot, ensure_gumball_node, with_host, mutations::text::Generation3dMutation};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_os_flow::FlowHost;
use semio_framework_plugin::{Emit, Fault, InteractionWrite};
use crate::editor::generation3d::selection::{component_group, ComponentTarget, DOMAIN};
use semio_framework_artifact_flow_flow::Widget;

pub fn selection_ids(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() { fallback.to_vec() } else { ids.to_vec() }
}

/// 🧷️ Accepts a pinned gesture instance only while the same component set continues through its transforms.
pub fn validate_component_gesture(snapshot: &FlowHostSnapshot, instances: &[String], selected: &[String]) -> Result<(), Fault> {
    if instances.is_empty() || selected.is_empty() { return Ok(()); }
    let (target, components) = component_group(selected).map_err(Fault::from)?;
    if instances.len() != 1 { return Err(Fault::from("Transform components of one mesh at a time")); }
    let mut widget_id = target.widget;
    let mut channel = target.channel;
    let mut visited = std::collections::BTreeSet::new();
    while visited.insert(widget_id) {
        if instances[0] == widget_id || instances[0] == format!("{widget_id}@{channel}#{}", target.index) { return Ok(()); }
        let Some(Widget::Neuron { neuron_kind, params, .. }) = snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == widget_id) else { break };
        if !matches!(neuron_kind.as_str(), "brep.mesh.translateComponents" | "brep.mesh.rotateComponents" | "brep.mesh.scaleComponents") || channel != "meshOut" { break; }
        let text = |key| params.get(key).and_then(|value| value.as_dictionary()).and_then(|value| value.get("value")).and_then(|value| value.as_atom()).and_then(|value| value.as_str());
        if text("mode") != Some(target.granularity) || text("selection").and_then(|value| serde_json::from_str::<Vec<u32>>(value).ok()).as_ref() != Some(&components) { break; }
        let mut inputs = snapshot.synapses.iter().filter(|wire| wire.to == widget_id && wire.to_port == "mesh");
        let Some(source) = inputs.next() else { break };
        if inputs.next().is_some() { break; }
        widget_id = &source.from;
        channel = &source.from_port;
    }
    Err(Fault::from("The component selection changed during the transform"))
}

/// 🔀️ Publishes one complete graph edit after every selected shape succeeds.
pub fn apply(
    snapshot: &FlowHostSnapshot,
    ids: &[String],
    operation: &str,
    transform: impl Fn(&mut FlowHost, &str) -> Result<(), String>,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    if ids.is_empty() { return Ok(Emit::default()); }
    if ids.iter().any(|id| ComponentTarget::parse(id).is_some()) {
        return with_host(snapshot, |host| {
            let (id, mode, components) = ensure_component_node(host, ids, operation).map_err(Fault::from)?;
            transform(host, &id).map_err(Fault::from)?;
            Ok(Emit {
                artifact_mutations: commit_host_snapshot(snapshot, &host.host_snapshot),
                interaction_writes: vec![InteractionWrite::replace("graph", "node", [id.clone()]), InteractionWrite::replace(DOMAIN, &mode, components.iter().map(|component| format!("{id}@meshOut#0.{mode}.{component}")))],
                coalesce_key: Some(format!("gumball-{id}-{mode}-{components:?}")),
                ..Default::default()
            })
        });
    }
    with_host(snapshot, |host| {
        let mut selected = std::collections::BTreeSet::new();
        for id in ids {
            let next = ensure_gumball_node(host, id, operation).map_err(Fault::from)?;
            if selected.insert(next.clone()) { transform(host, &next).map_err(Fault::from)?; }
        }
        Ok(Emit {
            artifact_mutations: commit_host_snapshot(snapshot, &host.host_snapshot),
            interaction_writes: vec![InteractionWrite::replace("graph", "node", selected)],
            coalesce_key: Some(format!("gumball-{operation}")),
            ..Default::default()
        })
    })
}

/// 🎯️ Splices an adjustable component transform and reuses it only for its own component set.
pub fn ensure_component_node(host: &mut FlowHost, ids: &[String], operation: &str) -> Result<(String, String, Vec<u32>), String> {
    let (target, components) = component_group(ids)?;
    if !matches!(operation, "translate" | "rotate" | "scale") { return Err("Unknown component transform".into()); }
    if target.index != 0 { return Err("Extract one mesh from the list before editing its components".into()); }
    let kind = host.host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::Neuron { id, neuron_kind, .. } if id == target.widget => Some(neuron_kind),
        _ => None,
    }).ok_or("The selected mesh no longer exists")?;
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let source = infos.get(kind).ok_or("The selected mesh operator is unavailable")?;
    if !source.outputs.iter().any(|port| port.name == target.channel && !port.cardinality.is_collection() && port.value_types.iter().any(|kind| kind == "mesh")) {
        return Err("Select a single indexed mesh output; convert B-Rep geometry to a mesh first".into());
    }
    let next_kind = format!("brep.mesh.{operation}Components");
    let selection = serde_json::to_string(&components).map_err(|error| error.to_string())?;
    let current = crate::standards::v1::subsets::any::schema::gumball_widget_json(host, target.widget);
    let params = current.as_ref().and_then(|value| value.get("params"));
    let parameter = |name| params.and_then(|value| value.get(name)).and_then(|value| value.get("value")).and_then(dsl::DslValue::as_str);
    let inputs = host.host_snapshot.synapses.iter().filter(|wire| wire.to == target.widget).collect::<Vec<_>>();
    if kind == &next_kind && parameter("mode") == Some(target.granularity) && parameter("selection") == Some(selection.as_str()) && (operation == "translate" || parameter("pivot") == Some("selection")) && inputs.len() == 1 && inputs[0].to_port == "mesh" {
        return Ok((target.widget.into(), target.granularity.into(), components));
    }
    let output = infos.get(&next_kind).and_then(|info| info.outputs.first()).ok_or("The component transform is unavailable")?;
    let base = format!("{}__{operation}Components", target.widget);
    let mut id = base.clone();
    let mut suffix = 2;
    while host.host_snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == id) { id = format!("{base}_{suffix}"); suffix += 1; }
    let (x, y) = host.host_snapshot.layout.get(target.widget).map_or((0.0, 0.0), |layout| (layout.x, layout.y));
    host.add_widget(&serde_json::json!({"kind":"neuron","id":id,"neuronKind":next_kind}).to_string(), x + 220.0, y).map_err(|error| error.to_string())?;
    let params = serde_json::json!({"mode":{"$schema":"text","value":target.granularity},"selection":{"$schema":"text","value":selection},"pivot":{"$schema":"text","value":"selection"}});
    host.set_neuron_params(&id, &params.to_string()).map_err(|error| error.to_string())?;
    host.insert_between(target.widget, target.channel, &id, "mesh", &output.name).map_err(|error| error.to_string())?;
    for widget in &mut host.host_snapshot.widgets {
        if let Widget::Neuron { id: widget_id, preview, .. } = widget {
            if widget_id == &id { *preview = true; }
            if widget_id == target.widget { *preview = false; }
        }
    }
    Ok((id, target.granularity.into(), components))
}
