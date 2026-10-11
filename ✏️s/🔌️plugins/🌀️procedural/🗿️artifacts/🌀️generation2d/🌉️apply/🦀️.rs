//! 🌉️ Central apply entry points: the only code of this artifact that turns a mutation's diff into the next snapshot with `protocol::apply_diff`; the schema and mutation leaves only build diffs and inverses.
#![allow(unused_imports)]

use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::{widget_id, Generation2dSnapshot};
use protocol::Mutation;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
#[cfg(test)]
use semio_framework_artifact_playbook_playbook::FormGeneration;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};
use crate::standards::v1::subsets::any::schema::mutations::*;

pub(crate) const GENERATION2D_OWNER_BYTES: usize = 4_096;

/// 📐️ The ONE structural nesting bound this artifact declares. The retained ingress cursor's
/// `PackLimits::max_depth`, the retained owner's own frame stacks and the initializer's
/// `generation2d_copy_*` guards all read it, so the wire can never reject nesting the initializer
/// would happily copy. A `Widget::Neuron`'s `params` is a neural `Dictionary` whose entries are
/// themselves `Value::Dictionary`, and each such level costs several pack frames — a two-level
/// dictionary already spends more than a dozen (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) const GENERATION2D_RETAINED_STACK_CAPACITY: usize = 64;

pub(crate) const GENERATION2D_MAXIMUM_DOMAIN_ITEMS: usize = 8_192;

#[derive(semio_framework_value::RetireOwned)]
pub(crate) enum Generation2dReplayDisplaced {
    Widget(semio_framework_artifact_flow_flow::Widget),
    Layouts(semio_framework_artifact_flow_flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>),
    Synapse(semio_framework_artifact_flow_flow::SynapseSpec),
    Layout(semio_framework_artifact_flow_flow::WidgetLayout),
    Camera(semio_framework_artifact_flow_flow::CameraJson),
    Text(String),
    Generation(semio_framework_artifact_playbook_playbook::FormGeneration),
    Json(semio_framework_value::DslValue),
}

/// 🧊️ Retires one displaced replay owner at a cold boundary by paying every currency its own close quote names.
pub(crate) fn generation2d_retire_displaced_cold(value: Generation2dReplayDisplaced) {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<Generation2dReplayDisplaced>(), maximum_release_bytes: 0, maximum_depth: 2 };
    let Ok((mut owner, _)) = semio_framework_value::retirement::admit_owned_retirement(value, birth) else { panic!("cold displaced replay owner refused its own birth") };
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().expect("cold displaced copy demand").max(GENERATION2D_OWNER_BYTES);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).expect("cold displaced capacity demand"), maximum_release_bytes: owner.next_release_byte_demand().expect("cold displaced release demand"), maximum_depth: owner.next_depth_demand().expect("cold displaced depth demand").max(1) };
        owner.close_step(grant).expect("cold displaced replay close");
    }
}

/// 🔁️ Direct semantic replay table. It consumes the retained mutation and writes only the
/// addressed field or collection; no VCS diff/apply or whole-snapshot replacement is reachable.
pub(crate) fn generation2d_apply_initialization_mutation(snapshot: &mut Generation2dSnapshot, mutation: &Generation2dMutation) -> Result<Option<Generation2dReplayDisplaced>, &'static str> {
    let retired = match mutation {
        Generation2dMutation::CreateWidget(payload) => {
            if snapshot.host_snapshot.widgets.iter().any(|entry| crate::widget_id(entry) == crate::widget_id(&payload.widget)) {
                return Err("generation2d-replay.widget-duplicate");
            }
            let index = payload.index.min(snapshot.host_snapshot.widgets.len());
            snapshot.host_snapshot.widgets.insert(index, generation2d_copy_widget(&payload.widget)?);
            None
        }
        Generation2dMutation::ReplaceWidget(payload) => {
            let id = crate::widget_id(&payload.widget);
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == id).ok_or("generation2d-replay.widget-missing")?;
            Some(Generation2dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], generation2d_copy_widget(&payload.widget)?)))
        }
        Generation2dMutation::DeleteWidget(payload) => {
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == payload.id).ok_or("generation2d-replay.widget-missing")?;
            Some(Generation2dReplayDisplaced::Widget(snapshot.host_snapshot.widgets.remove(index)))
        }
        Generation2dMutation::ConnectSynapse(payload) => {
            if snapshot.host_snapshot.synapses.iter().any(|entry| entry.id == payload.synapse.id) {
                return Err("generation2d-replay.synapse-duplicate");
            }
            let index = payload.index.min(snapshot.host_snapshot.synapses.len());
            snapshot.host_snapshot.synapses.insert(index, generation2d_copy_synapse(&payload.synapse)?);
            None
        }
        Generation2dMutation::ReplaceSynapse(payload) => {
            let index = snapshot.host_snapshot.synapses.iter().position(|entry| entry.id == payload.synapse.id).ok_or("generation2d-replay.synapse-missing")?;
            Some(Generation2dReplayDisplaced::Synapse(std::mem::replace(&mut snapshot.host_snapshot.synapses[index], generation2d_copy_synapse(&payload.synapse)?)))
        }
        Generation2dMutation::DisconnectSynapse(payload) => {
            let index = snapshot.host_snapshot.synapses.iter().position(|entry| entry.id == payload.id).ok_or("generation2d-replay.synapse-missing")?;
            Some(Generation2dReplayDisplaced::Synapse(snapshot.host_snapshot.synapses.remove(index)))
        }
        Generation2dMutation::MoveWidget(payload) => {
            if !payload.layout.x.is_finite() || !payload.layout.y.is_finite() {
                return Err("generation2d-replay.layout-nonfinite");
            }
            snapshot
                .host_snapshot
                .layout
                .insert(generation2d_copy_string(&payload.id)?, semio_framework_artifact_flow_flow::WidgetLayout { x: payload.layout.x, y: payload.layout.y })
                .map(Generation2dReplayDisplaced::Layout)
        }
        Generation2dMutation::ClearWidgetLayout(payload) => snapshot.host_snapshot.layout.remove(&payload.id).map(Generation2dReplayDisplaced::Layout),
        Generation2dMutation::UpdateCamera(payload) => {
            if !payload.camera.x.is_finite() || !payload.camera.y.is_finite() || !payload.camera.zoom.is_finite() {
                return Err("generation2d-replay.camera-nonfinite");
            }
            Some(Generation2dReplayDisplaced::Camera(std::mem::replace(&mut snapshot.host_snapshot.camera, semio_framework_artifact_flow_flow::CameraJson { x: payload.camera.x, y: payload.camera.y, zoom: payload.camera.zoom })))
        }
        Generation2dMutation::ChangeSchema(payload) => Some(Generation2dReplayDisplaced::Text(std::mem::replace(&mut snapshot.host_snapshot.schema, generation2d_copy_string(&payload.schema)?))),
        Generation2dMutation::CreateGeneration(payload) => {
            if snapshot.generation.generations.iter().any(|entry| entry.id == payload.generation.id) {
                return Err("generation2d-replay.generation-duplicate");
            }
            let mut selected = String::new();
            selected.try_reserve_exact(payload.generation.id.len()).map_err(|_| "generation2d-replay.selected-generation-preflight")?;
            for character in payload.generation.id.chars() {
                selected.push(character);
            }
            {
                let generations = &mut snapshot.generation.cold_builder_mut().expect("unique cold generation owner").generations;
                let at = payload.index.unwrap_or(generations.len()).min(generations.len());
                generations.insert(at, generation2d_copy_generation(&payload.generation)?);
            }
            snapshot.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some(selected);
            None
        }
        Generation2dMutation::DeleteGeneration(payload) => {
            let index = snapshot.generation.generations.iter().position(|entry| entry.id == payload.id).ok_or("generation2d-replay.generation-missing")?;
            let removed = snapshot.generation.cold_builder_mut()?.generations.remove(index);
            if snapshot.generation.selected_generation_id.as_deref() == Some(payload.id.as_str()) {
                let mut selected = None;
                if let Some(first) = snapshot.generation.generations.first() {
                    let mut id = String::new();
                    id.try_reserve_exact(first.id.len()).map_err(|_| "generation2d-replay.selected-generation-preflight")?;
                    for character in first.id.chars() {
                        id.push(character);
                    }
                    selected = Some(id);
                }
                snapshot.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = selected;
            }
            Some(Generation2dReplayDisplaced::Generation(removed))
        }
        Generation2dMutation::RenameGeneration(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation2d-replay.generation-missing")?;
            Some(Generation2dReplayDisplaced::Text(std::mem::replace(&mut entry.name, generation2d_copy_string(&payload.name)?)))
        }
        Generation2dMutation::SelectGeneration(payload) => {
            if payload.generation_id.as_ref().is_some_and(|id| !snapshot.generation.generations.iter().any(|entry| &entry.id == id)) {
                return Err("generation2d-replay.generation-missing");
            }
            let selected = match &payload.generation_id {
                Some(id) => Some(generation2d_copy_string(id)?),
                None => None,
            };
            snapshot.generation.cold_builder_mut()?.selected_generation_id = selected;
            None
        }
        Generation2dMutation::ChangeGenerationValue(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation2d-replay.generation-missing")?;
            let copied = generation2d_copy_json(&payload.value, 0)?;
            entry.values.insert(generation2d_copy_string(&payload.question_id)?, copied).map(Generation2dReplayDisplaced::Json)
        }
        Generation2dMutation::ChangeSliderValue(payload) => {
            if !payload.value.is_finite() {
                return Err("generation2d-replay.slider-nonfinite");
            }
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == payload.id).ok_or("generation2d-replay.widget-missing")?;
            let mut next = generation2d_copy_widget(&snapshot.host_snapshot.widgets[index])?;
            if !semio_framework_artifact_flow_flow::set_widget_slider_value(&mut next, payload.value) {
                next.retire_cold();
                return Err("generation2d-replay.slider-target");
            }
            Some(Generation2dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], next)))
        }
        Generation2dMutation::MoveNodes(payload) => {
            crate::standards::v1::subsets::any::schema::mutations::generation2d_targets_invariant(&payload.ids)?;
            if !payload.dx.is_finite() || !payload.dy.is_finite() || payload.ids.len() > GENERATION2D_MAXIMUM_DOMAIN_ITEMS {
                return Err("generation2d-replay.nodes-invariant");
            }
            let mut updates = Vec::new();
            updates.try_reserve_exact(payload.ids.len()).map_err(|_| "generation2d-replay.layout-preflight")?;
            for id in &payload.ids {
                if !snapshot.host_snapshot.widgets.iter().any(|entry| crate::widget_id(entry) == id) {
                    continue;
                }
                if let Some(layout) = snapshot.host_snapshot.layout.get(id) {
                    let next = semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x + payload.dx, y: layout.y + payload.dy };
                    if !next.x.is_finite() || !next.y.is_finite() {
                        return Err("generation2d-replay.layout-nonfinite");
                    }
                    updates.push((generation2d_copy_string(id)?, next));
                }
            }
            if updates.is_empty() {
                return Err("generation2d-replay.nodes-target");
            }
            if (payload.dx, payload.dy) == (0.0, 0.0) {
                None
            } else {
                let displaced = snapshot.host_snapshot.layout.clone();
                for (id, next) in updates {
                    snapshot.host_snapshot.layout.insert(id, next);
                }
                Some(Generation2dReplayDisplaced::Layouts(displaced))
            }
        }
    };
    Ok(retired)
}

pub(crate) fn generation2d_copy_string(source: &str) -> Result<String, &'static str> {
    let mut target = String::new();
    target.try_reserve_exact(source.len()).map_err(|_| "generation2d-initializer.string-preflight")?;
    for character in source.chars() {
        target.push(character);
    }
    Ok(target)
}

pub(crate) fn generation2d_copy_json(source: &semio_framework_value::DslValue, depth: usize) -> Result<semio_framework_value::DslValue, &'static str> {
    if depth >= GENERATION2D_RETAINED_STACK_CAPACITY {
        return Err("generation2d-initializer.json-depth");
    }
    Ok(match source {
        semio_framework_value::DslValue::Null => semio_framework_value::DslValue::Null,
        semio_framework_value::DslValue::Bool(value) => semio_framework_value::DslValue::Bool(*value),
        semio_framework_value::DslValue::Number(value) => semio_framework_value::DslValue::Number(*value),
        semio_framework_value::DslValue::String(value) => semio_framework_value::DslValue::String(generation2d_copy_string(value)?),
        semio_framework_value::DslValue::Bytes(values) => {
            let mut target = Vec::new();
            target.try_reserve_exact(values.len()).map_err(|_| "generation2d-initializer.json-bytes-preflight")?;
            target.extend_from_slice(values);
            semio_framework_value::DslValue::Bytes(target)
        }
        semio_framework_value::DslValue::Array(values) => {
            let mut target = Vec::new();
            target.try_reserve_exact(values.len()).map_err(|_| "generation2d-initializer.json-array-preflight")?;
            for value in values {
                target.push(generation2d_copy_json(value, depth + 1)?);
            }
            semio_framework_value::DslValue::Array(target)
        }
        semio_framework_value::DslValue::Object(values) => {
            let mut target = Vec::new();
            for (key, value) in values {
                target.push((generation2d_copy_string(key)?, generation2d_copy_json(value, depth + 1)?));
            }
            semio_framework_value::DslValue::Object(target)
        }
    })
}

pub(crate) fn generation2d_copy_neural_value(source: &semio_framework_artifact_flow_flow::neural::Value, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Value, &'static str> {
    if depth >= GENERATION2D_RETAINED_STACK_CAPACITY {
        return Err("generation2d-initializer.neural-depth");
    }
    Ok(match source {
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null) => semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null),
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(generation2d_copy_string(value)?))
        }
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(value) => semio_framework_artifact_flow_flow::neural::Value::Dictionary(generation2d_copy_dictionary(value, depth + 1)?),
    })
}

pub(crate) fn generation2d_copy_dictionary(source: &semio_framework_artifact_flow_flow::neural::Dictionary, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Dictionary, &'static str> {
    let mut target = semio_framework_artifact_flow_flow::neural::Dictionary::new();
    for key in source.keys() {
        let value = source.get(key).ok_or("generation2d-initializer.dictionary-owner")?;
        target = target.insert(generation2d_copy_string(key)?, generation2d_copy_neural_value(value, depth + 1)?);
    }
    Ok(target)
}

pub(crate) fn generation2d_copy_tree(source: &semio_framework_artifact_flow_flow::neural::Tree, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Tree, &'static str> {
    if depth >= GENERATION2D_RETAINED_STACK_CAPACITY {
        return Err("generation2d-initializer.tree-depth");
    }
    let mut neurons = Vec::new();
    neurons.try_reserve_exact(source.neurons.len()).map_err(|_| "generation2d-initializer.neurons-preflight")?;
    for neuron in &source.neurons {
        neurons.push(semio_framework_artifact_flow_flow::neural::Neuron {
            id: generation2d_copy_string(&neuron.id)?,
            kind: generation2d_copy_string(&neuron.kind)?,
            params: generation2d_copy_dictionary(&neuron.params, depth + 1)?,
            tree: match neuron.tree.as_deref() {
                Some(tree) => Some(Box::new(generation2d_copy_tree(tree, depth + 1)?)),
                None => None,
            },
        });
    }
    let mut synapses = Vec::new();
    synapses.try_reserve_exact(source.synapses.len()).map_err(|_| "generation2d-initializer.tree-synapses-preflight")?;
    for synapse in &source.synapses {
        synapses.push(semio_framework_artifact_flow_flow::neural::Synapse {
            id: generation2d_copy_string(&synapse.id)?,
            from: generation2d_copy_string(&synapse.from)?,
            to: generation2d_copy_string(&synapse.to)?,
            from_port: generation2d_copy_string(&synapse.from_port)?,
            to_port: generation2d_copy_string(&synapse.to_port)?,
        });
    }
    Ok(semio_framework_artifact_flow_flow::neural::Tree { neurons, synapses })
}

pub(crate) fn generation2d_copy_flow_ui(source: &semio_framework_artifact_flow_flow::FlowGui) -> Result<semio_framework_artifact_flow_flow::FlowGui, &'static str> {
    let mut nodes = semio_framework_artifact_flow_flow::OrderedMap::new();
    for (id, node) in &source.nodes {
        let chrome = match &node.chrome {
            semio_framework_artifact_flow_flow::NodeChrome::Plain { preview } => semio_framework_artifact_flow_flow::NodeChrome::Plain { preview: *preview },
            semio_framework_artifact_flow_flow::NodeChrome::Slider { label, min, max, step, value } => {
                semio_framework_artifact_flow_flow::NodeChrome::Slider { label: generation2d_copy_string(label)?, min: *min, max: *max, step: *step, value: *value }
            }
            semio_framework_artifact_flow_flow::NodeChrome::Note { text } => semio_framework_artifact_flow_flow::NodeChrome::Note { text: generation2d_copy_string(text)? },
            semio_framework_artifact_flow_flow::NodeChrome::Image { src } => semio_framework_artifact_flow_flow::NodeChrome::Image { src: generation2d_copy_string(src)? },
            semio_framework_artifact_flow_flow::NodeChrome::Variable { name, schema } => semio_framework_artifact_flow_flow::NodeChrome::Variable { name: generation2d_copy_string(name)?, schema: generation2d_copy_string(schema)? },
        };
        nodes.insert(generation2d_copy_string(id)?, semio_framework_artifact_flow_flow::FlowNodeGui { layout: semio_framework_artifact_flow_flow::WidgetLayout { x: node.layout.x, y: node.layout.y }, chrome });
    }
    let mut previews = Vec::new();
    previews.try_reserve_exact(source.previews.len()).map_err(|_| "generation2d-initializer.previews-preflight")?;
    for preview in &source.previews {
        let source = match &preview.source {
            Some(source) => Some(semio_framework_artifact_flow_flow::FlowChannelRef { neuron: generation2d_copy_string(&source.neuron)?, channel: generation2d_copy_string(&source.channel)? }),
            None => None,
        };
        let mut expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
        for value in &preview.expanded {
            expanded.insert(generation2d_copy_string(value)?);
        }
        previews.push(semio_framework_artifact_flow_flow::FlowPreviewGui {
            id: generation2d_copy_string(&preview.id)?,
            source,
            mode: generation2d_copy_string(&preview.mode)?,
            preview: generation2d_copy_dictionary(&preview.preview, 0)?,
            expanded,
            layout: preview.layout.as_ref().map(|layout| semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x, y: layout.y }),
        });
    }
    Ok(semio_framework_artifact_flow_flow::FlowUi { camera: semio_framework_artifact_flow_flow::CameraJson { x: source.camera.x, y: source.camera.y, zoom: source.camera.zoom }, nodes, previews })
}

pub(crate) fn generation2d_copy_widget(source: &semio_framework_artifact_flow_flow::Widget) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
    Ok(match source {
        semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            let mut inputs = Vec::new();
            inputs.try_reserve_exact(input_ports.len()).map_err(|_| "generation2d-initializer.inputs-preflight")?;
            for value in input_ports {
                inputs.push(generation2d_copy_string(value)?);
            }
            let mut outputs = Vec::new();
            outputs.try_reserve_exact(output_ports.len()).map_err(|_| "generation2d-initializer.outputs-preflight")?;
            for value in output_ports {
                outputs.push(generation2d_copy_string(value)?);
            }
            semio_framework_artifact_flow_flow::Widget::Neuron {
                id: generation2d_copy_string(id)?,
                neuron_kind: generation2d_copy_string(neuron_kind)?,
                params: generation2d_copy_dictionary(params, 0)?,
                input_ports: inputs,
                output_ports: outputs,
                preview: *preview,
            }
        }
        semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, value, min, max, step } => {
            semio_framework_artifact_flow_flow::Widget::InputSlider { id: generation2d_copy_string(id)?, label: generation2d_copy_string(label)?, value: *value, min: *min, max: *max, step: *step }
        }
        semio_framework_artifact_flow_flow::Widget::InputNote { id, text } => semio_framework_artifact_flow_flow::Widget::InputNote { id: generation2d_copy_string(id)?, text: generation2d_copy_string(text)? },
        semio_framework_artifact_flow_flow::Widget::InputImage { id, src } => semio_framework_artifact_flow_flow::Widget::InputImage { id: generation2d_copy_string(id)?, src: generation2d_copy_string(src)? },
        semio_framework_artifact_flow_flow::Widget::Variable { id, name, schema } => {
            semio_framework_artifact_flow_flow::Widget::Variable { id: generation2d_copy_string(id)?, name: generation2d_copy_string(name)?, schema: generation2d_copy_string(schema)? }
        }
        semio_framework_artifact_flow_flow::Widget::OutputPreview { id, preview, expanded } => {
            let mut next_expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
            for value in expanded {
                next_expanded.insert(generation2d_copy_string(value)?);
            }
            semio_framework_artifact_flow_flow::Widget::OutputPreview { id: generation2d_copy_string(id)?, preview: generation2d_copy_dictionary(preview, 0)?, expanded: next_expanded }
        }
        semio_framework_artifact_flow_flow::Widget::OutputAction { id, action } => semio_framework_artifact_flow_flow::Widget::OutputAction { id: generation2d_copy_string(id)?, action: generation2d_copy_string(action)? },
        semio_framework_artifact_flow_flow::Widget::OutputExport { id, format } => semio_framework_artifact_flow_flow::Widget::OutputExport { id: generation2d_copy_string(id)?, format: generation2d_copy_string(format)? },
        semio_framework_artifact_flow_flow::Widget::Cluster { id, name, tree, flow } => {
            semio_framework_artifact_flow_flow::Widget::Cluster { id: generation2d_copy_string(id)?, name: generation2d_copy_string(name)?, tree: generation2d_copy_tree(tree, 0)?, flow: generation2d_copy_flow_ui(flow)? }
        }
    })
}

pub(crate) fn generation2d_copy_synapse(source: &semio_framework_artifact_flow_flow::SynapseSpec) -> Result<semio_framework_artifact_flow_flow::SynapseSpec, &'static str> {
    Ok(semio_framework_artifact_flow_flow::SynapseSpec {
        id: generation2d_copy_string(&source.id)?,
        from: generation2d_copy_string(&source.from)?,
        to: generation2d_copy_string(&source.to)?,
        from_port: generation2d_copy_string(&source.from_port)?,
        to_port: generation2d_copy_string(&source.to_port)?,
    })
}

pub(crate) fn generation2d_copy_generation(source: &semio_framework_artifact_playbook_playbook::FormGeneration) -> Result<semio_framework_artifact_playbook_playbook::FormGeneration, &'static str> {
    Ok(semio_framework_artifact_playbook_playbook::FormGeneration { id: generation2d_copy_string(&source.id)?, name: generation2d_copy_string(&source.name)?, values: source.values.clone() })
}

/// 🧊️ Cold-only disposal of a detached mutation — `CreateWidget`/`ReplaceWidget` carry an owned
/// `Widget` whose `Dictionary`/`Tree`/`OrderedSet` payloads reject a bare drop
/// (`🧠️neural/⚙️engine/🦀️.rs`'s `Dictionary::drop`, `🌱️value/🗂️ordered/🦀️.rs`'s roots). Every other
/// variant is plain owned text/floats and closes on drop.
pub(crate) fn generation2d_retire_mutation_cold(mutation: Generation2dMutation) {
    match mutation {
        Generation2dMutation::CreateWidget(payload) => payload.widget.retire_cold(),
        Generation2dMutation::ReplaceWidget(payload) => payload.widget.retire_cold(),
        Generation2dMutation::DeleteWidget(_)
        | Generation2dMutation::ConnectSynapse(_)
        | Generation2dMutation::ReplaceSynapse(_)
        | Generation2dMutation::DisconnectSynapse(_)
        | Generation2dMutation::MoveWidget(_)
        | Generation2dMutation::ClearWidgetLayout(_)
        | Generation2dMutation::UpdateCamera(_)
        | Generation2dMutation::ChangeSchema(_)
        | Generation2dMutation::CreateGeneration(_)
        | Generation2dMutation::DeleteGeneration(_)
        | Generation2dMutation::RenameGeneration(_)
        | Generation2dMutation::SelectGeneration(_)
        | Generation2dMutation::ChangeGenerationValue(_)
        | Generation2dMutation::ChangeSliderValue(_)
        | Generation2dMutation::MoveNodes(_) => {}
    }
}

/// 🧊️ The plural twin of [`generation2d_retire_mutation_cold`].
#[cfg(test)]
pub(crate) fn generation2d_retire_mutations_cold(mutations: Vec<Generation2dMutation>) {
    for mutation in mutations {
        generation2d_retire_mutation_cold(mutation);
    }
}

#[cfg(test)]
pub(crate) fn generation2d_apply_retained_mutations_for_test(snapshot: &mut Generation2dSnapshot, mutations: &[Generation2dMutation]) {
    for mutation in mutations {
        if let Some(displaced) = generation2d_apply_initialization_mutation(snapshot, mutation).expect("P2 production fixture retained replay") {
            generation2d_retire_displaced_cold(displaced);
        }
    }
}


/// 🧬️ Applies a mutation to a projection — generic over every variant, so it never needs edits
/// when the semantic vocabulary grows. A refused diff (Error/Fatal) is never applied as its empty delta: the refusal
/// travels as the outcome's own messages, codes and levels unchanged, and an apply-time rejection joins them as the
/// `Fatal` `mutation.apply.*` message `MutationOutcome::apply_to` would persist.
pub fn apply_generation2d_mutation(projection: &mut Generation2dSnapshot, mutation: &Generation2dMutation) -> Result<(), Vec<protocol::MutationMessage>> {
    let (delta, messages) = protocol::Mutation::diff(mutation, &*projection).into_parts();
    if messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        delta.retire_cold();
        return Err(messages);
    }
    let applied = protocol::apply_diff(&delta, &*projection);
    delta.retire_cold();
    match applied {
        Ok(next) => {
            std::mem::replace(projection, next).retire_cold();
            Ok(())
        }
        Err(error) => Err(messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect()),
    }
}

/// 🎚️ The slider fields after `target` lands on it (the range widened exactly like the canvas knob), computed on a probe copy; `None` when no range can hold the value.
pub(crate) fn generation2d_slider_landing(value: f64, min: f64, max: f64, step: f64, target: f64) -> Option<(f64, f64, f64, f64)> {
    let mut landing = semio_framework_artifact_flow_flow::Widget::InputSlider { id: String::new(), label: String::new(), value, min, max, step };
    if !semio_framework_artifact_flow_flow::set_widget_slider_value(&mut landing, target) {
        return None;
    }
    match landing {
        semio_framework_artifact_flow_flow::Widget::InputSlider { value, min, max, step, .. } => Some((value, min, max, step)),
        _ => None,
    }
}
