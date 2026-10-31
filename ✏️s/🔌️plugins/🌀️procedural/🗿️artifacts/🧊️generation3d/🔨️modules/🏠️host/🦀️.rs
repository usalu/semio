//! 🏠️ Generation3d publication, retained replay and store ownership.
#![allow(clippy::result_large_err)]
#![allow(unused_imports)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
use semio_s_artifact_procedural_3d_mut_generations::change_generation_value::ChangeGenerationValue;
use semio_s_artifact_procedural_3d_mut_document::change_schema::ChangeSchema;
use semio_s_artifact_procedural_3d_mut_synapses::connect_synapse::ConnectSynapse;
use semio_s_artifact_procedural_3d_mut_generations::create_generation::CreateGeneration;
use semio_s_artifact_procedural_3d_mut_widgets::create_widget::CreateWidget;
use semio_s_artifact_procedural_3d_mut_generations::delete_generation::DeleteGeneration;
use semio_s_artifact_procedural_3d_mut_widgets::delete_widget::DeleteWidget;
use semio_s_artifact_procedural_3d_mut_widgets::delete_widget_position::DeleteWidgetPosition;
use semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse;
use semio_s_artifact_procedural_3d_mut_widgets::move_widget::MoveWidget;
use semio_s_artifact_procedural_3d_mut_generations::rename_generation::RenameGeneration;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_mut_document::update_camera::UpdateCamera;
use semio_s_artifact_procedural_3d_mut_synapses::update_synapse::UpdateSynapse;
use semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget;
use semio_s_artifact_procedural_3d_io_text::snapshot::camera_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::camera_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::form_generation_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::form_generation_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::layout_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::layout_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::synapse_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::synapse_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::widget_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::widget_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::CameraJsonDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::FormGenerationDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::SynapseSpecDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::WidgetDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::WidgetLayoutDsl;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_mut_widgets::change_slider_value::ChangeSliderValue;
use semio_s_artifact_procedural_3d_mut_transforms::drag_transforms::DragTransforms;
use semio_s_artifact_procedural_3d_mut_transforms::rotate_transforms::RotateTransforms;
use semio_s_artifact_procedural_3d_mut_transforms::scale_transforms::ScaleTransforms;
use semio_s_artifact_procedural_3d_mut_widgets::move_nodes::MoveNodes;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::ChangeWidgetInput;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue;
use semio_s_artifact_procedural_3d_mut_generations::change_generation_preview::ChangeGenerationPreview;
use semio_s_artifact_procedural_3d_mut_generations::select_generation::SelectGeneration;
use protocol::OpBinary;
use store::ErasedSnapshotRetirement;
//#region 🔖️RetainedMountedIngress
pub const GENERATION3D_OWNER_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
pub const GENERATION3D_RETAINED_STACK_CAPACITY: usize = 64;
pub const GENERATION3D_MAXIMUM_DOMAIN_ITEMS: usize = 8_192;
pub const GENERATION3D_MAXIMUM_DOMAIN_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
pub const GENERATION3D_MUTATION_VARIANT_COUNT: usize = 22;

/// 🧭️ Fixed ownership grammar for every Generation3d retained domain and lifecycle owner.
/// The 3d-only DeleteWidgetPosition entry is deliberately explicit: a 2d mutation catalog cannot satisfy this table.
pub const GENERATION3D_RETAINED_OWNER_CATALOG: &[&str] = &[
    "snapshot.host_snapshot.schema",
    "snapshot.host_snapshot.camera.x",
    "snapshot.host_snapshot.camera.y",
    "snapshot.host_snapshot.camera.zoom",
    "snapshot.host_snapshot.widgets.length",
    "snapshot.host_snapshot.widgets.item.neuron",
    "snapshot.host_snapshot.widgets.item.input-slider",
    "snapshot.host_snapshot.widgets.item.input-note",
    "snapshot.host_snapshot.widgets.item.input-image",
    "snapshot.host_snapshot.widgets.item.variable",
    "snapshot.host_snapshot.widgets.item.output-preview",
    "snapshot.host_snapshot.widgets.item.output-action",
    "snapshot.host_snapshot.widgets.item.output-export",
    "snapshot.host_snapshot.widgets.item.cluster",
    "snapshot.host_snapshot.widgets.item.strings",
    "snapshot.host_snapshot.widgets.item.dictionary.entries",
    "snapshot.host_snapshot.widgets.item.tree",
    "snapshot.host_snapshot.widgets.item.flow",
    "snapshot.host_snapshot.synapses.length",
    "snapshot.host_snapshot.synapses.item.id",
    "snapshot.host_snapshot.synapses.item.from",
    "snapshot.host_snapshot.synapses.item.to",
    "snapshot.host_snapshot.synapses.item.from-port",
    "snapshot.host_snapshot.synapses.item.to-port",
    "snapshot.host_snapshot.layout.length",
    "snapshot.host_snapshot.layout.item.id",
    "snapshot.host_snapshot.layout.item.x",
    "snapshot.host_snapshot.layout.item.y",
    "snapshot.generation.generations.length",
    "snapshot.generation.generations.item.id",
    "snapshot.generation.generations.item.name",
    "snapshot.generation.generations.item.values.length",
    "snapshot.generation.generations.item.values.key",
    "snapshot.generation.generations.item.values.scalar",
    "snapshot.generation.selected-generation-id",
    "snapshot.generation.preview-text",
    "mutations.create-widget",
    "mutations.update-widget",
    "mutations.delete-widget",
    "mutations.connect-synapse",
    "mutations.update-synapse",
    "mutations.disconnect-synapse",
    "mutations.move-widget",
    "mutations.delete-widget-position.3d-only",
    "mutations.update-camera",
    "mutations.change-schema",
    "mutations.create-generation",
    "mutations.delete-generation",
    "mutations.rename-generation",
    "mutations.change-generation-value",
    "history.edit.id",
    "history.edit.actor",
    "history.edit.forward",
    "history.edit.inverse",
    "history.edit.mutation-meta",
    "history.cursor.applied",
    "history.cursor.redo",
    "history.cursor.checkpoint",
    "conflict.rejected-fresh",
    "child.widget.cluster.tree",
    "child.widget.cluster.flow",
    "control.cancel",
    "control.retry",
    "control.close",
    "output.progress",
    "output.checkpoint",
    "output.preview",
    "output.terminal",
];

pub const GENERATION3D_RETAINED_MUTATION_OWNERS: [&str; GENERATION3D_MUTATION_VARIANT_COUNT] = [
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

pub const GENERATION3D_RETAINED_SCHEMA_DISCRIMINATOR: [u8; 4] = *b"P3D3";
pub const GENERATION3D_FORBIDDEN_2D_DISCRIMINATOR: [u8; 4] = *b"P2D2";

pub fn generation3d_retained_catalog_is_complete() -> bool {
    GENERATION3D_RETAINED_MUTATION_OWNERS == semio_s_artifact_procedural_3d_mutations::KINDS
        && GENERATION3D_RETAINED_OWNER_CATALOG.contains(&"mutations.delete-widget-position.3d-only")
        && !GENERATION3D_RETAINED_OWNER_CATALOG.iter().any(|owner| owner.contains("process2d"))
}

#[derive(semio_framework_value::RetireOwned)]
pub enum Generation3dReplayDisplaced {
    Widget(semio_framework_artifact_flow_flow::Widget),
    Widgets(Vec<semio_framework_artifact_flow_flow::Widget>),
    Layouts(semio_framework_artifact_flow_flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>),
    Synapse(semio_framework_artifact_flow_flow::SynapseSpec),
    Layout(semio_framework_artifact_flow_flow::WidgetLayout),
    Camera(semio_framework_artifact_flow_flow::CameraJson),
    Text(String),
    Generation(semio_framework_artifact_playbook_playbook::FormGeneration),
    Json(semio_framework_value::DslValue),
}

/// 🧊️ Retires one displaced replay owner at a cold boundary by paying every currency its own close quote names.
#[cfg(any(test, feature = "testkit"))]
fn generation3d_retire_displaced_cold(value: Generation3dReplayDisplaced) {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<Generation3dReplayDisplaced>(), maximum_release_bytes: 0, maximum_depth: 2 };
    let Ok((mut owner, _)) = semio_framework_value::retirement::admit_owned_retirement(value, birth).result else { panic!("cold displaced replay owner refused its own birth") };
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().expect("cold displaced copy demand").max(GENERATION3D_OWNER_BYTES);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).expect("cold displaced capacity demand"), maximum_release_bytes: owner.next_release_byte_demand().expect("cold displaced release demand"), maximum_depth: owner.next_depth_demand().expect("cold displaced depth demand").max(1) };
        owner.close_step(grant).expect("cold displaced replay close");
    }
}

/// 🪜️ Pays one Flow frontier turn out of the frontier's own quoted currencies — the mounted-pack session ladder this serves still counts items and bytes, so it cannot forward a caller grant.
pub fn generation3d_close_flow_frontier(flow: &mut semio_framework_artifact_flow_flow::retained::FlowRetirement) -> Result<(), semio_framework_value::ValueError> {
    let copy = flow.next_copy_byte_demand()?;
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: flow.next_capacity_byte_demand(copy)?, maximum_release_bytes: flow.next_release_byte_demand()?, maximum_depth: flow.next_depth_demand()?.max(1) };
    flow.step(grant).map(|_| ())
}

/// 🅿️ Parks one string list on an EMPTY Flow frontier; the callers only park once the previous owner is fully retired.
pub fn generation3d_park_flow_strings(flow: &mut semio_framework_artifact_flow_flow::retained::FlowRetirement, values: Vec<String>) {
    if flow.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Strings(values)).is_err() {
        unreachable!("an empty Flow frontier admits one owner");
    }
}

/// 🧮️ Replays a relative gesture into addressed operators and retains only displaced widget owners.
fn generation3d_replay_transforms(snapshot: &mut Generation3dSnapshot, targets: &[String], kinds: &[&str], identity: bool, compose: impl Fn(&semio_framework_artifact_flow_flow::neural::Dictionary) -> Option<Vec<(&'static str, semio_framework_artifact_flow_flow::neural::Value)>>) -> Result<Option<Generation3dReplayDisplaced>, &'static str> {
    use semio_s_artifact_procedural_3d_mut_widgets::generation3d_targets_invariant;
    use semio_s_artifact_procedural_3d_mut_widgets::generation3d_with_params;

    generation3d_targets_invariant(targets)?;
    if targets.len() > GENERATION3D_MAXIMUM_DOMAIN_ITEMS { return Err("generation3d-replay.target-limit"); }
    let mut updates = Vec::new();
    updates.try_reserve_exact(targets.len()).map_err(|_| "generation3d-replay.transform-preflight")?;
    for id in targets {
        let Some(index) = snapshot.host_snapshot.widgets.iter().position(|entry| semio_s_artifact_procedural_3d_model::widget_id(entry) == id) else { continue };
        let widget = &snapshot.host_snapshot.widgets[index];
        let semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, params, .. } = widget else { continue };
        if !kinds.contains(&neuron_kind.as_str()) { continue; }
        if let Some(next) = compose(params).map(|entries| generation3d_with_params(widget, entries)) { updates.push((index, next)); }
    }
    if updates.is_empty() { return Err("generation3d-replay.transform-target"); }
    if identity {
        for (_, widget) in updates { widget.retire_cold(); }
        return Ok(None);
    }
    let mut displaced = Vec::new();
    displaced.try_reserve_exact(updates.len()).map_err(|_| "generation3d-replay.displaced-preflight")?;
    for (index, next) in updates { displaced.push(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], next)); }
    Ok(Some(Generation3dReplayDisplaced::Widgets(displaced)))
}

/// 🔁️ Direct semantic replay table. It consumes the retained mutation and writes only the
/// addressed field or collection; no VCS diff/apply or whole-snapshot replacement is reachable.
pub fn generation3d_apply_initialization_mutation(snapshot: &mut Generation3dSnapshot, mutation: &Generation3dMutation) -> Result<Option<Generation3dReplayDisplaced>, &'static str> {
    let retired = match mutation {
        Generation3dMutation::CreateWidget(payload) => {
            if snapshot.host_snapshot.widgets.iter().any(|entry| semio_s_artifact_procedural_3d_model::widget_id(entry) == semio_s_artifact_procedural_3d_model::widget_id(&payload.widget)) {
                return Err("generation3d-replay.widget-duplicate");
            }
            let index = payload.index.min(snapshot.host_snapshot.widgets.len());
            snapshot.host_snapshot.widgets.insert(index, generation3d_copy_widget(&payload.widget)?);
            None
        }
        Generation3dMutation::UpdateWidget(payload) => {
            let id = semio_s_artifact_procedural_3d_model::widget_id(&payload.widget);
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| semio_s_artifact_procedural_3d_model::widget_id(entry) == id).ok_or("generation3d-replay.widget-missing")?;
            for (index, synapse) in semio_s_artifact_procedural_3d_mut_widgets::update_widget::variable_synapses(&snapshot.host_snapshot.widgets[index], &payload.widget, &snapshot.host_snapshot.synapses) { snapshot.host_snapshot.synapses[index] = synapse; }
            Some(Generation3dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], generation3d_copy_widget(&payload.widget)?)))
        }
        Generation3dMutation::DeleteWidget(payload) => {
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| semio_s_artifact_procedural_3d_model::widget_id(entry) == payload.id).ok_or("generation3d-replay.widget-missing")?;
            Some(Generation3dReplayDisplaced::Widget(snapshot.host_snapshot.widgets.remove(index)))
        }
        Generation3dMutation::ConnectSynapse(payload) => {
            if snapshot.host_snapshot.synapses.iter().any(|entry| entry.id == payload.synapse.id) {
                return Err("generation3d-replay.synapse-duplicate");
            }
            let index = payload.index.min(snapshot.host_snapshot.synapses.len());
            snapshot.host_snapshot.synapses.insert(index, generation3d_copy_synapse(&payload.synapse)?);
            None
        }
        Generation3dMutation::UpdateSynapse(payload) => {
            let index = snapshot.host_snapshot.synapses.iter().position(|entry| entry.id == payload.synapse.id).ok_or("generation3d-replay.synapse-missing")?;
            Some(Generation3dReplayDisplaced::Synapse(std::mem::replace(&mut snapshot.host_snapshot.synapses[index], generation3d_copy_synapse(&payload.synapse)?)))
        }
        Generation3dMutation::DisconnectSynapse(payload) => {
            let index = snapshot.host_snapshot.synapses.iter().position(|entry| entry.id == payload.id).ok_or("generation3d-replay.synapse-missing")?;
            Some(Generation3dReplayDisplaced::Synapse(snapshot.host_snapshot.synapses.remove(index)))
        }
        Generation3dMutation::MoveWidget(payload) => {
            if !payload.layout.x.is_finite() || !payload.layout.y.is_finite() {
                return Err("generation3d-replay.layout-nonfinite");
            }
            snapshot
                .host_snapshot
                .layout
                .insert(generation3d_copy_string(&payload.id)?, semio_framework_artifact_flow_flow::WidgetLayout { x: payload.layout.x, y: payload.layout.y })
                .map(Generation3dReplayDisplaced::Layout)
                
        }
        Generation3dMutation::DeleteWidgetPosition(payload) => snapshot.host_snapshot.layout.remove(&payload.id).map(Generation3dReplayDisplaced::Layout),
        Generation3dMutation::UpdateCamera(payload) => {
            if !payload.camera.x.is_finite() || !payload.camera.y.is_finite() || !payload.camera.zoom.is_finite() {
                return Err("generation3d-replay.camera-nonfinite");
            }
            Some(Generation3dReplayDisplaced::Camera(std::mem::replace(&mut snapshot.host_snapshot.camera, semio_framework_artifact_flow_flow::CameraJson { x: payload.camera.x, y: payload.camera.y, zoom: payload.camera.zoom })))
        }
        Generation3dMutation::ChangeSchema(payload) => Some(Generation3dReplayDisplaced::Text(std::mem::replace(&mut snapshot.host_snapshot.schema, generation3d_copy_string(&payload.new_schema)?))),
        Generation3dMutation::CreateGeneration(payload) => {
            let generation = snapshot.generation.cold_builder_mut()?;
            if generation.generations.iter().any(|entry| entry.id == payload.generation.id) {
                return Err("generation3d-replay.generation-duplicate");
            }
            let mut selected = String::new();
            selected.try_reserve_exact(payload.generation.id.len()).map_err(|_| "generation3d-replay.selected-generation-preflight")?;
            for character in payload.generation.id.chars() {
                selected.push(character);
            }
            let at = payload.index.unwrap_or(generation.generations.len()).min(generation.generations.len());
            generation.generations.insert(at, generation3d_copy_generation(&payload.generation)?);
            generation.selected_generation_id = Some(selected);
            None
        }
        Generation3dMutation::DeleteGeneration(payload) => {
            let generation = snapshot.generation.cold_builder_mut()?;
            let index = generation.generations.iter().position(|entry| entry.id == payload.id).ok_or("generation3d-replay.generation-missing")?;
            let removed = generation.generations.remove(index);
            if generation.selected_generation_id.as_deref() == Some(payload.id.as_str()) {
                let mut selected = None;
                if let Some(first) = generation.generations.first() {
                    let mut id = String::new();
                    id.try_reserve_exact(first.id.len()).map_err(|_| "generation3d-replay.selected-generation-preflight")?;
                    for character in first.id.chars() {
                        id.push(character);
                    }
                    selected = Some(id);
                }
                generation.selected_generation_id = selected;
            }
            Some(Generation3dReplayDisplaced::Generation(removed))
        }
        Generation3dMutation::RenameGeneration(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation3d-replay.generation-missing")?;
            Some(Generation3dReplayDisplaced::Text(std::mem::replace(&mut entry.name, generation3d_copy_string(&payload.new_name)?)))
        }
        Generation3dMutation::ChangeGenerationValue(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation3d-replay.generation-missing")?;
            entry.values.insert(generation3d_copy_string(&payload.question_id)?, generation3d_copy_json(&payload.new_value, 0)?).map(Generation3dReplayDisplaced::Json)
        }
        Generation3dMutation::ChangeSliderValue(payload) => {
            if !payload.value.is_finite() { return Err("generation3d-replay.slider-nonfinite"); }
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| semio_s_artifact_procedural_3d_model::widget_id(entry) == payload.id).ok_or("generation3d-replay.widget-missing")?;
            let mut next = generation3d_copy_widget(&snapshot.host_snapshot.widgets[index])?;
            if !semio_framework_artifact_flow_flow::set_widget_slider_value(&mut next, payload.value) { next.retire_cold(); return Err("generation3d-replay.slider-target"); }
            Some(Generation3dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], next)))
        }
        Generation3dMutation::DragTransforms(payload) => {
            use semio_s_artifact_procedural_3d_mut_transforms::generation3d_param_vector;
            use semio_s_artifact_procedural_3d_mut_transforms::generation3d_vector_value;
            use semio_s_artifact_procedural_3d_mut_transforms::GENERATION3D_TRANSLATE_KINDS;

            let delta = [payload.dx, payload.dy, payload.dz];
            if delta.iter().any(|value| !value.is_finite()) { return Err("generation3d-replay.drag-nonfinite"); }
            generation3d_replay_transforms(snapshot, &payload.targets, &GENERATION3D_TRANSLATE_KINDS, delta == [0.0; 3], |params| {
                let current = generation3d_param_vector(params, "offset", [0.0; 3]);
                let next: [f64; 3] = std::array::from_fn(|axis| current[axis] + delta[axis]);
                next.iter().all(|value| value.is_finite()).then(|| vec![("offset", generation3d_vector_value("vector", next))])
            })?
        }
        Generation3dMutation::RotateTransforms(payload) => {
            use semio_s_artifact_procedural_3d_mut_transforms::generation3d_param_vector;
            use semio_s_artifact_procedural_3d_mut_transforms::generation3d_param_number;
            use semio_s_artifact_procedural_3d_mut_transforms::generation3d_vector_value;
            use semio_s_artifact_procedural_3d_mut_transforms::generation3d_number_value;
            use semio_s_artifact_procedural_3d_mut_transforms::GENERATION3D_ROTATE_KINDS;

            use semio_s_artifact_procedural_3d_model::transforms::AxisAngle;
            let axis = [payload.ax, payload.ay, payload.az];
            if axis == [0.0; 3] || axis.iter().chain([&payload.angle]).any(|value| !value.is_finite()) { return Err("generation3d-replay.rotation-invariant"); }
            let delta = AxisAngle { axis, angle: payload.angle };
            generation3d_replay_transforms(snapshot, &payload.targets, &GENERATION3D_ROTATE_KINDS, payload.angle == 0.0, |params| {
                let current = AxisAngle { axis: generation3d_param_vector(params, "axis", [0.0, 0.0, 1.0]), angle: generation3d_param_number(params, "angle", 0.0) };
                let next = current.then(delta).ok()?;
                Some(vec![("axis", generation3d_vector_value("vector", next.axis)), ("angle", generation3d_number_value(next.angle))])
            })?
        }
        Generation3dMutation::ScaleTransforms(payload) => {
            use semio_s_artifact_procedural_3d_mut_transforms::generation3d_param_vector;
            use semio_s_artifact_procedural_3d_mut_transforms::generation3d_vector_value;
            use semio_s_artifact_procedural_3d_mut_transforms::GENERATION3D_SCALE_KINDS;

            use semio_s_artifact_procedural_3d_model::transforms::compose_scale;
            let factors = [payload.sx, payload.sy, payload.sz];
            if factors.iter().any(|value| !value.is_finite() || *value <= 0.0) { return Err("generation3d-replay.scale-invariant"); }
            generation3d_replay_transforms(snapshot, &payload.targets, &GENERATION3D_SCALE_KINDS, factors == [1.0; 3], |params| {
                let next = compose_scale(generation3d_param_vector(params, "factor", [1.0; 3]), factors).ok()?;
                Some(vec![("factor", generation3d_vector_value("vector", next)), ("center", generation3d_vector_value("point", [0.0; 3]))])
            })?
        }
        Generation3dMutation::MoveNodes(payload) => {
            semio_s_artifact_procedural_3d_mut_widgets::generation3d_targets_invariant(&payload.ids)?;
            if !payload.dx.is_finite() || !payload.dy.is_finite() || payload.ids.len() > GENERATION3D_MAXIMUM_DOMAIN_ITEMS { return Err("generation3d-replay.nodes-invariant"); }
            let mut updates = Vec::new();
            updates.try_reserve_exact(payload.ids.len()).map_err(|_| "generation3d-replay.layout-preflight")?;
            for id in &payload.ids {
                if !snapshot.host_snapshot.widgets.iter().any(|entry| semio_s_artifact_procedural_3d_model::widget_id(entry) == id) { continue; }
                if let Some(layout) = snapshot.host_snapshot.layout.get(id) {
                    let next = semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x + payload.dx, y: layout.y + payload.dy };
                    if !next.x.is_finite() || !next.y.is_finite() { return Err("generation3d-replay.layout-nonfinite"); }
                    updates.push((generation3d_copy_string(id)?, next));
                }
            }
            if updates.is_empty() { return Err("generation3d-replay.nodes-target"); }
            if (payload.dx, payload.dy) == (0.0, 0.0) { None } else {
                let displaced = snapshot.host_snapshot.layout.clone();
                for (id, next) in updates { snapshot.host_snapshot.layout.insert(id, next); }
                Some(Generation3dReplayDisplaced::Layouts(displaced))
            }
        }
        Generation3dMutation::SelectGeneration(payload) => {
            if let Some(id) = &payload.generation_id {
                if !snapshot.generation.generations.iter().any(|entry| &entry.id == id) { return Err("generation3d-replay.generation-missing"); }
            }
            let next = payload.generation_id.as_deref().map(generation3d_copy_string).transpose()?;
            std::mem::replace(&mut snapshot.generation.cold_builder_mut()?.selected_generation_id, next).map(Generation3dReplayDisplaced::Text)
        }
        Generation3dMutation::ChangeGenerationPreview(payload) => {
            let next = payload.text.as_deref().map(generation3d_copy_string).transpose()?;
            std::mem::replace(&mut snapshot.generation.cold_builder_mut()?.preview_text, next).map(Generation3dReplayDisplaced::Text)
        }
        Generation3dMutation::ChangeWidgetInput(payload) => {
            if !payload.admissible() { return Err("generation3d-replay.input-invariant"); }
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| semio_s_artifact_procedural_3d_model::widget_id(entry) == payload.id).ok_or("generation3d-replay.widget-missing")?;
            let wired = snapshot.host_snapshot.synapses.iter().any(|synapse| synapse.to == payload.id && synapse.to_port == payload.channel);
            match payload.landing(&snapshot.host_snapshot.widgets[index], wired).map_err(|_| "generation3d-replay.input-target")? {
                None => None,
                Some(next) => Some(Generation3dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], next))),
            }
        }
    };
    Ok(retired)
}
pub fn generation3d_copy_string(source: &str) -> Result<String, &'static str> {
    let mut target = String::new();
    target.try_reserve_exact(source.len()).map_err(|_| "generation3d-initializer.string-preflight")?;
    for character in source.chars() {
        target.push(character);
    }
    Ok(target)
}
fn generation3d_copy_json(source: &semio_framework_value::DslValue, depth: usize) -> Result<semio_framework_value::DslValue, &'static str> {
    if depth >= GENERATION3D_RETAINED_STACK_CAPACITY {
        return Err("generation3d-initializer.json-depth");
    }
    Ok(match source {
        semio_framework_value::DslValue::Null => semio_framework_value::DslValue::Null,
        semio_framework_value::DslValue::Bool(value) => semio_framework_value::DslValue::Bool(*value),
        semio_framework_value::DslValue::Number(value) => semio_framework_value::DslValue::Number(*value),
        semio_framework_value::DslValue::String(value) => semio_framework_value::DslValue::String(generation3d_copy_string(value)?),
        semio_framework_value::DslValue::Bytes(values) => {
            let mut target = Vec::new();
            target.try_reserve_exact(values.len()).map_err(|_| "generation3d-initializer.json-bytes-preflight")?;
            target.extend_from_slice(values);
            semio_framework_value::DslValue::Bytes(target)
        }
        semio_framework_value::DslValue::Array(values) => {
            let mut target = Vec::new();
            target.try_reserve_exact(values.len()).map_err(|_| "generation3d-initializer.json-array-preflight")?;
            for value in values {
                target.push(generation3d_copy_json(value, depth + 1)?);
            }
            semio_framework_value::DslValue::Array(target)
        }
        semio_framework_value::DslValue::Object(values) => {
            let mut target = Vec::new();
            for (key, value) in values {
                target.push((generation3d_copy_string(key)?, generation3d_copy_json(value, depth + 1)?));
            }
            semio_framework_value::DslValue::Object(target)
        }
    })
}
pub fn generation3d_copy_widget(source: &semio_framework_artifact_flow_flow::Widget) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
    Ok(match source {
        semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            let mut inputs = Vec::new();
            inputs.try_reserve_exact(input_ports.len()).map_err(|_| "generation3d-initializer.inputs-preflight")?;
            for value in input_ports {
                inputs.push(generation3d_copy_string(value)?);
            }
            let mut outputs = Vec::new();
            outputs.try_reserve_exact(output_ports.len()).map_err(|_| "generation3d-initializer.outputs-preflight")?;
            for value in output_ports {
                outputs.push(generation3d_copy_string(value)?);
            }
            semio_framework_artifact_flow_flow::Widget::Neuron {
                id: generation3d_copy_string(id)?,
                neuron_kind: generation3d_copy_string(neuron_kind)?,
                params: generation3d_copy_dictionary(params, 0)?,
                input_ports: inputs,
                output_ports: outputs,
                preview: *preview,
            }
        }
        semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, value, min, max, step } => {
            semio_framework_artifact_flow_flow::Widget::InputSlider { id: generation3d_copy_string(id)?, label: generation3d_copy_string(label)?, value: *value, min: *min, max: *max, step: *step }
        }
        semio_framework_artifact_flow_flow::Widget::InputNote { id, text } => semio_framework_artifact_flow_flow::Widget::InputNote { id: generation3d_copy_string(id)?, text: generation3d_copy_string(text)? },
        semio_framework_artifact_flow_flow::Widget::InputImage { id, src } => semio_framework_artifact_flow_flow::Widget::InputImage { id: generation3d_copy_string(id)?, src: generation3d_copy_string(src)? },
        semio_framework_artifact_flow_flow::Widget::Variable { id, name, schema } => {
            semio_framework_artifact_flow_flow::Widget::Variable { id: generation3d_copy_string(id)?, name: generation3d_copy_string(name)?, schema: generation3d_copy_string(schema)? }
        }
        semio_framework_artifact_flow_flow::Widget::OutputPreview { id, preview, expanded } => {
            let mut next_expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
            for value in expanded {
                next_expanded.insert(generation3d_copy_string(value)?);
            }
            semio_framework_artifact_flow_flow::Widget::OutputPreview { id: generation3d_copy_string(id)?, preview: generation3d_copy_dictionary(preview, 0)?, expanded: next_expanded }
        }
        semio_framework_artifact_flow_flow::Widget::OutputAction { id, action } => semio_framework_artifact_flow_flow::Widget::OutputAction { id: generation3d_copy_string(id)?, action: generation3d_copy_string(action)? },
        semio_framework_artifact_flow_flow::Widget::OutputExport { id, format } => semio_framework_artifact_flow_flow::Widget::OutputExport { id: generation3d_copy_string(id)?, format: generation3d_copy_string(format)? },
        semio_framework_artifact_flow_flow::Widget::Cluster { id, name, tree, flow } => {
            semio_framework_artifact_flow_flow::Widget::Cluster { id: generation3d_copy_string(id)?, name: generation3d_copy_string(name)?, tree: generation3d_copy_tree(tree, 0)?, flow: generation3d_copy_flow_ui(flow)? }
        }
    })
}
pub fn generation3d_copy_synapse(source: &semio_framework_artifact_flow_flow::SynapseSpec) -> Result<semio_framework_artifact_flow_flow::SynapseSpec, &'static str> {
    Ok(semio_framework_artifact_flow_flow::SynapseSpec {
        id: generation3d_copy_string(&source.id)?,
        from: generation3d_copy_string(&source.from)?,
        to: generation3d_copy_string(&source.to)?,
        from_port: generation3d_copy_string(&source.from_port)?,
        to_port: generation3d_copy_string(&source.to_port)?,
    })
}
pub fn generation3d_copy_generation(source: &semio_framework_artifact_playbook_playbook::FormGeneration) -> Result<semio_framework_artifact_playbook_playbook::FormGeneration, &'static str> {
    Ok(semio_framework_artifact_playbook_playbook::FormGeneration { id: generation3d_copy_string(&source.id)?, name: generation3d_copy_string(&source.name)?, values: source.values.clone() })
}
//#endregion 🔖️RetainedMountedIngress



#[cfg(any(test, feature = "testkit"))]
pub fn generation3d_all_retained_mutation_fixtures_for_test(corpus: &str) -> Vec<Generation3dMutation> {
    let synapse = semio_framework_artifact_flow_flow::SynapseSpec { id: "retained-synapse".into(), from: "retained-a".into(), to: "retained-b".into(), from_port: "out".into(), to_port: "in".into() };
    let mut values: semio_framework_artifact_playbook_playbook::PlaybookValues = semio_framework_artifact_playbook_playbook::PlaybookValues::new();
    values.insert(
        "nested".into(),
        semio_framework_value::DslValue::object([("array".to_string(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::Bool(true), semio_framework_value::DslValue::Null, semio_framework_value::DslValue::float(3.5)])), ("text".to_string(), semio_framework_value::DslValue::String("retained".to_string()))]),
    );
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("retained".into()))),
        ),
    );
    let mut mutations = vec![
        Generation3dMutation::CreateWidget(CreateWidget {
            index: 0,
            widget: semio_framework_artifact_flow_flow::Widget::Neuron { id: "retained-a".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true },
        }),
        Generation3dMutation::UpdateWidget(UpdateWidget { widget: semio_framework_artifact_flow_flow::Widget::Cluster { id: "retained-a".into(), name: "Updated".into(), tree: Default::default(), flow: Default::default() } }),
        Generation3dMutation::DeleteWidget(DeleteWidget { id: "retained-a".into() }),
        Generation3dMutation::ConnectSynapse(ConnectSynapse { index: 0, synapse: generation3d_copy_synapse(&synapse).expect("P3 synapse fixture copy") }),
        Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: semio_framework_artifact_flow_flow::SynapseSpec { to_port: "alternate".into(), ..synapse } }),
        Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "retained-synapse".into() }),
        Generation3dMutation::MoveWidget(MoveWidget { id: "retained-a".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 11.0, y: -7.0 } }),
        Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "retained-a".into() }),
        Generation3dMutation::UpdateCamera(UpdateCamera { camera: semio_framework_artifact_flow_flow::CameraJson { x: 3.0, y: 4.0, zoom: 1.5 } }),
        Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.host_snapshot.retained".into() }),
        Generation3dMutation::CreateGeneration(CreateGeneration { generation: semio_framework_artifact_playbook_playbook::FormGeneration { id: "retained-generation".into(), name: "Retained Generation".into(), values }, index: Some(0) }),
        Generation3dMutation::DeleteGeneration(DeleteGeneration { id: "retained-generation".into() }),
        Generation3dMutation::RenameGeneration(RenameGeneration { id: "retained-generation".into(), new_name: "Renamed Generation".into() }),
        Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue {
            id: "retained-generation".into(),
            question_id: "deep-answer".into(),
            new_value: semio_framework_value::DslValue::object([("object".to_string(), semio_framework_value::DslValue::object([("array".to_string(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(1.0), semio_framework_value::DslValue::Bool(false), semio_framework_value::DslValue::String("value".to_string())]))]))]),
        }),
        Generation3dMutation::SelectGeneration(SelectGeneration { generation_id: Some("retained-generation".into()) }),
        Generation3dMutation::SelectGeneration(SelectGeneration { generation_id: None }),
        Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text: Some("Retained preview \u{1F600}".into()) }),
        Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text: None }),
    ];
    let corpus: serde_json::Value = serde_json::from_str(corpus).expect("semantic wire corpus");
    for case in corpus["cases"].as_array().expect("semantic wire cases") {
        mutations.push(<Generation3dMutation as semio_framework_value::FromValue>::from_value(case["mutation"].clone().into()).expect("authored semantic mutation"));
    }
    mutations
}

#[cfg(any(test, feature = "testkit"))]
pub fn generation3d_apply_retained_mutations_for_test(snapshot: &mut Generation3dSnapshot, mutations: &[Generation3dMutation]) {
    for mutation in mutations {
        if let Some(displaced) = generation3d_apply_initialization_mutation(snapshot, mutation).expect("P3 production fixture retained replay") {
            generation3d_retire_displaced_cold(displaced);
        }
    }
}
