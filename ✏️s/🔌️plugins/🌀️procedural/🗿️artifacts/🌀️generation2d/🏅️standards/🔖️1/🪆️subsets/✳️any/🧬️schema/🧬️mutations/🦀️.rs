//! 🧬️ Generation2d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload: eight live in the
//! `🧬️mutations/<slug>/` triad leaves wired by `🦀️.rs` (their directory/module names are
//! leftovers of the generic slots they were repurposed from — see this ticket's wave2 report for
//! the glue.rs rename that would align them), the rest — those with no pre-wired slot — live inline
//! below as `mod <slug> { 🦠️mutation / 🔺️diff / ↩️inverse }` regions, same shape, same file.
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<Generation2dSnapshot>` and
//! `impl protocol::SemanticMutation<Generation2dSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::{widget_id, Generation2dSnapshot};
use protocol::Mutation;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
#[cfg(test)]
use semio_framework_artifact_playbook_playbook::FormGeneration;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};
//#region 🔖️Addressing
/// 🌡️ Resolves a widget's stable id to its BASE-state index in the fixture's widget list.
pub fn widget_index(host_snapshot: &FlowHostSnapshot, id: &str) -> Option<usize> {
    host_snapshot.widgets.iter().position(|widget| widget_id(widget) == id)
}

/// 🌡️ Resolves a synapse's stable id to its BASE-state index in the fixture's synapse list.
pub fn synapse_index(host_snapshot: &FlowHostSnapshot, id: &str) -> Option<usize> {
    host_snapshot.synapses.iter().position(|synapse| synapse.id == id)
}
//#endregion 🔖️Addressing

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = Generation2dSnapshot, diff = Generation2dDiff, schema = "generation.2d")]
pub enum Generation2dMutation {
    CreateWidget(super::create_widget::CreateWidget),
    ReplaceWidget(super::replace_widget::ReplaceWidget),
    DeleteWidget(super::delete_widget::DeleteWidget),
    ConnectSynapse(super::connect_synapse::ConnectSynapse),
    ReplaceSynapse(super::replace_synapse::ReplaceSynapse),
    DisconnectSynapse(super::disconnect_synapse::DisconnectSynapse),
    MoveWidget(super::move_widget::MoveWidget),
    ClearWidgetLayout(super::clear_widget_layout::ClearWidgetLayout),
    UpdateCamera(super::update_camera::UpdateCamera),
    ChangeSchema(super::change_schema::ChangeSchema),
    CreateGeneration(super::create_generation::CreateGeneration),
    DeleteGeneration(super::delete_generation::DeleteGeneration),
    RenameGeneration(super::rename_generation::RenameGeneration),
    ChangeGenerationValue(super::change_generation_value::ChangeGenerationValue),
    ChangeSliderValue(super::change_slider_value::ChangeSliderValue),
    MoveNodes(super::move_nodes::MoveNodes),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Generation2dMutation`] variant, in declaration order — the exact
/// vocabulary the `procedural-2d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🌀️mutate-procedural-2d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-widget",
    "replace-widget",
    "delete-widget",
    "connect-synapse",
    "replace-synapse",
    "disconnect-synapse",
    "move-widget",
    "clear-widget-layout",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
    "change-slider-value",
    "move-nodes",
];
//#endregion 🏷️Kinds

//#region 🔖️GestureLeaves
/// 🖊️ One number as the history labels print it: English with a decimal point, German with a decimal comma.
pub(crate) fn generation2d_label_number(value: f64) -> (String, String) {
    let english = format!("{}", (value * 1_000.0).round() / 1_000.0);
    let german = english.replace('.', ",");
    (english, german)
}

/// 🧱️ The payload-intrinsic target law every relative gesture leaf states in its schema: at least one id, each once.
pub(crate) fn generation2d_targets_invariant(targets: &[String]) -> Result<(), &'static str> {
    if targets.is_empty() || targets.iter().any(String::is_empty) {
        return Err("a gesture leaf names at least one non-empty target");
    }
    if targets.iter().enumerate().any(|(at, id)| targets[..at].contains(id)) {
        return Err("a gesture leaf names each target once");
    }
    Ok(())
}

/// 🩹️ The `mutation.partial` warning of the targets a relative leaf skipped, or nothing.
pub(crate) fn generation2d_partial(skipped: Vec<String>, total: usize, reason: &str) -> Option<protocol::MutationMessage> {
    (!skipped.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {total} target(s) skipped ({reason}): {}", skipped.len(), skipped.join(", "))).at(skipped))
}
//#endregion 🔖️GestureLeaves
//#endregion 🔖️Mutations

//#region 🧊️Retirement
impl Generation2dMutation {
    /// 🧊️ Explicit cold-only disposal of one owned mutation. `create-widget`/`replace-widget` carry a
    /// whole `Widget`, whose `neural::Dictionary` (and `OrderedSet`/`Tree`) fail-close on a bare drop,
    /// so a dropped mutation aborts the process. Every other variant drops freely.
    pub fn retire_cold(self) {
        match self {
            Self::CreateWidget(super::create_widget::CreateWidget { widget, .. }) | Self::ReplaceWidget(super::replace_widget::ReplaceWidget { widget }) => widget.retire_cold(),
            _ => {}
        }
    }
}
//#endregion 🧊️Retirement

//#region 🔖️GenerationBridge
/// 🌉️ Bridges one `semio_framework_artifact_playbook_playbook::GenerationMutation` (the framework's own generation-editing
/// vocabulary — `Add`/`Remove`/`Rename`/`UpdateValues`) onto this facet's semantic
/// `Generation2dMutation` variants, so app-layer callers that already hold a `GenerationMutation`
/// (from `semio_framework_artifact_playbook_playbook::generation_operations`) need only swap the mapping function at the call
/// site, not learn this facet's internal triad-leaf module paths. Twin of generation3d's
/// `generation_mutation_to_generation3d` — the two facets' generation payloads differ only in field
/// naming (`name`/`value` here, `new_name`/`new_value` there).
pub fn generation_mutation_to_generation2d(operation: GenerationMutation) -> Generation2dMutation {
    match operation {
        GenerationMutation::Add { generation } => Generation2dMutation::CreateGeneration(super::create_generation::CreateGeneration { generation, index: None }),
        GenerationMutation::Remove { id } => Generation2dMutation::DeleteGeneration(super::delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation2dMutation::RenameGeneration(super::rename_generation::RenameGeneration { id, name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation2dMutation::ChangeGenerationValue(super::change_generation_value::ChangeGenerationValue { id, question_id, value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️Builders
pub use super::change_generation_value::change_generation_value;
pub use super::change_schema::change_schema;
pub use super::change_slider_value::change_slider_value;
pub use super::clear_widget_layout::clear_widget_layout;
pub use super::connect_synapse::connect_synapse;
pub use super::create_generation::create_generation;
pub use super::create_widget::create_widget;
pub use super::delete_generation::delete_generation;
pub use super::delete_widget::delete_widget;
pub use super::disconnect_synapse::disconnect_synapse;
pub use super::move_nodes::move_nodes;
pub use super::move_widget::move_widget;
pub use super::rename_generation::rename_generation;
pub use super::replace_synapse::replace_synapse;
pub use super::replace_widget::replace_widget;
pub use super::update_camera::update_camera;
//#endregion 🔖️Builders

//#region 🔖️HostSnapshotOperations
/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable semantic operation set:
/// created/replaced/deleted widgets and synapses (keyed by id), moved/cleared layout entries, and
/// a changed fixture schema. The canvas camera is ephemeral view state (app config), never a
/// document operation.
pub fn generation2d_host_snapshot_operations(before: &FlowHostSnapshot, after: &FlowHostSnapshot) -> Vec<Generation2dMutation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(delete_widget(widget_id(widget).to_string()));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        match before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget)) {
            None => operations.push(create_widget(index, widget.clone())),
            Some(prior) if prior != widget => operations.push(replace_widget(widget.clone())),
            _ => {}
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(disconnect_synapse(synapse.id.clone()));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        match before.synapses.iter().find(|entry| entry.id == synapse.id) {
            None => operations.push(connect_synapse(index, synapse.clone())),
            Some(prior) if prior != synapse => operations.push(replace_synapse(synapse.clone())),
            _ => {}
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(clear_widget_layout(id.clone()));
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(move_widget(id.clone(), layout.clone()));
        }
    }
    if before.schema != after.schema {
        operations.push(change_schema(after.schema.clone()));
    }
    operations
}
//#endregion 🔖️FixtureOperations

pub type Generation2dEnvelope = ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>;
pub type Generation2dStore = ArtifactStore<Generation2dSnapshot, Generation2dMutation>;

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

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_generation2d_mutation(projection: &Generation2dSnapshot, mutation: &Generation2dMutation) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️gesture-leaves/🦀️.rs"]
mod gesture_leaves_tests;
//#endregion 🧪️Tests

pub(crate) const GENERATION2D_OWNER_BYTES: usize = 4_096;

/// 📐️ The ONE structural nesting bound this artifact declares. The retained ingress cursor's
/// `PackLimits::max_depth`, the retained owner's own frame stacks and the initializer's
/// `generation2d_copy_*` guards all read it, so the wire can never reject nesting the initializer
/// would happily copy. A `Widget::Neuron`'s `params` is a neural `Dictionary` whose entries are
/// themselves `Value::Dictionary`, and each such level costs several pack frames — a two-level
/// dictionary already spends more than a dozen (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) const GENERATION2D_RETAINED_STACK_CAPACITY: usize = 64;

pub(crate) const GENERATION2D_MAXIMUM_DOMAIN_ITEMS: usize = 8_192;

enum Generation2dReplayDisplaced {
    Widget(semio_framework_artifact_flow_flow::Widget),
    Layouts(semio_framework_artifact_flow_flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>),
    Synapse(semio_framework_artifact_flow_flow::SynapseSpec),
    Layout(std::sync::Arc<semio_framework_artifact_flow_flow::WidgetLayout>),
    Camera(semio_framework_artifact_flow_flow::CameraJson),
    Text(String),
    Generation(semio_framework_artifact_playbook_playbook::FormGeneration),
    Json(std::sync::Arc<semio_framework_value::DslValue>),
}

/// 🎟️ One bounded unit of the Flow frontier's reserve-then-close protocol.
///
/// [`semio_framework_artifact_flow_flow::retained::FlowRetirement`] keeps its continuation owners in
/// a `PagedList` whose pages must be admitted before they can hold anything, so its bare `close_step`
/// answers `Blocked` for as long as `next_allocation_bytes` still names a page. The erased
/// [`store::ErasedSnapshotRetirement`] contract every framework close ladder drives this codec through
/// carries one item and one page and has no demand channel, so this codec pays that reservation itself
/// through [`semio_framework_artifact_flow_flow::retained::FlowRetirement::close_page`].
pub(crate) fn generation2d_close_flow_frontier(flow: &mut semio_framework_artifact_flow_flow::retained::FlowRetirement, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
    if maximum_items == 0 || maximum_bytes == 0 {
        return Ok(store::SnapshotRetirementStep::Blocked);
    }
    let demand = flow.next_close_byte_demand().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, message))?;
    let step = flow.close_page(maximum_items, maximum_bytes.max(demand))?;
    Ok(match step {
        store::SnapshotRetirementStep::Pending { released_items, released_bytes } => store::SnapshotRetirementStep::Pending { released_items, released_bytes: released_bytes.min(maximum_bytes) },
        step => step,
    })
}

struct Generation2dReplayRetirement {
    value: std::mem::ManuallyDrop<Option<Generation2dReplayDisplaced>>,
    domain: semio_framework_artifact_flow_flow::retained::FlowRetirement,
    child: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl store::ErasedSnapshotRetirement for Generation2dReplayRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if let Some(child) = self.child.as_mut() {
            let step = child.close_step(maximum_items, maximum_bytes)?;
            if matches!(step, store::SnapshotRetirementStep::Complete) { self.child = None; return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
            return Ok(step);
        }
        if !self.domain.terminal_is_empty() {
            return generation2d_close_flow_frontier(&mut self.domain, maximum_items, maximum_bytes);
        }
        if maximum_items == 0 || maximum_bytes < GENERATION2D_OWNER_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            match value {
                Generation2dReplayDisplaced::Widget(value) => self.domain.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Widget(value)),
                Generation2dReplayDisplaced::Layouts(value) => self.domain.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(value)),
                Generation2dReplayDisplaced::Synapse(value) => self.domain.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Specs(vec![value])),
                Generation2dReplayDisplaced::Layout(value) => drop(value),
                Generation2dReplayDisplaced::Camera(value) => { let _ = value; },
                Generation2dReplayDisplaced::Text(value) => self.domain.text(value),
                Generation2dReplayDisplaced::Generation(value) => self.child = Some(Box::new(semio_framework_artifact_playbook_playbook::GenerationPlayRoot::from(semio_framework_artifact_playbook_playbook::GenerationPlayState { generations: vec![value], selected_generation_id: None, preview_text: None }).into_retirement())),
                Generation2dReplayDisplaced::Json(value) => self.child = Some(semio_framework_value::retirement::shared_lease_retirement(value)),
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: GENERATION2D_OWNER_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.child.is_none() && self.domain.terminal_is_empty()
    }
}

impl Drop for Generation2dReplayRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.value.is_none()), "Generation2d replay displacement reached Drop before terminal-empty close");
    }
}

pub(crate) fn generation2d_retire_displaced(value: Generation2dReplayDisplaced) -> Box<dyn store::ErasedSnapshotRetirement> {
    Box::new(Generation2dReplayRetirement { value: std::mem::ManuallyDrop::new(Some(value)), domain: semio_framework_artifact_flow_flow::retained::FlowRetirement::default(), child: None })
}

/// 🔁️ Direct semantic replay table. It consumes the retained mutation and writes only the
/// addressed field or collection; no VCS diff/apply or whole-snapshot replacement is reachable.
pub(crate) fn generation2d_apply_initialization_mutation(snapshot: &mut Generation2dSnapshot, mutation: &Generation2dMutation) -> Result<Option<Box<dyn store::ErasedSnapshotRetirement>>, &'static str> {
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
            Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], generation2d_copy_widget(&payload.widget)?))))
        }
        Generation2dMutation::DeleteWidget(payload) => {
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == payload.id).ok_or("generation2d-replay.widget-missing")?;
            Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Widget(snapshot.host_snapshot.widgets.remove(index))))
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
            Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Synapse(std::mem::replace(&mut snapshot.host_snapshot.synapses[index], generation2d_copy_synapse(&payload.synapse)?))))
        }
        Generation2dMutation::DisconnectSynapse(payload) => {
            let index = snapshot.host_snapshot.synapses.iter().position(|entry| entry.id == payload.id).ok_or("generation2d-replay.synapse-missing")?;
            Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Synapse(snapshot.host_snapshot.synapses.remove(index))))
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
                .map(generation2d_retire_displaced)
        }
        Generation2dMutation::ClearWidgetLayout(payload) => snapshot.host_snapshot.layout.remove(&payload.id).map(Generation2dReplayDisplaced::Layout).map(generation2d_retire_displaced),
        Generation2dMutation::UpdateCamera(payload) => {
            if !payload.camera.x.is_finite() || !payload.camera.y.is_finite() || !payload.camera.zoom.is_finite() {
                return Err("generation2d-replay.camera-nonfinite");
            }
            Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Camera(std::mem::replace(&mut snapshot.host_snapshot.camera, semio_framework_artifact_flow_flow::CameraJson { x: payload.camera.x, y: payload.camera.y, zoom: payload.camera.zoom }))))
        }
        Generation2dMutation::ChangeSchema(payload) => Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Text(std::mem::replace(&mut snapshot.host_snapshot.schema, generation2d_copy_string(&payload.schema)?)))),
        Generation2dMutation::CreateGeneration(payload) => {
            if snapshot.generation.generations.iter().any(|entry| entry.id == payload.generation.id) {
                return Err("generation2d-replay.generation-duplicate");
            }
            let mut selected = String::new();
            selected.try_reserve_exact(payload.generation.id.len()).map_err(|_| "generation2d-replay.selected-generation-preflight")?;
            for character in payload.generation.id.chars() {
                selected.push(character);
            }
            snapshot.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(generation2d_copy_generation(&payload.generation)?);
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
            Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Generation(removed)))
        }
        Generation2dMutation::RenameGeneration(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation2d-replay.generation-missing")?;
            Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Text(std::mem::replace(&mut entry.name, generation2d_copy_string(&payload.name)?))))
        }
        Generation2dMutation::ChangeGenerationValue(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation2d-replay.generation-missing")?;
            let copied = generation2d_copy_json(&payload.value, 0)?;
            entry.values.insert(generation2d_copy_string(&payload.question_id)?, copied).map(Generation2dReplayDisplaced::Json).map(generation2d_retire_displaced)
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
            Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], next))))
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
                Some(generation2d_retire_displaced(Generation2dReplayDisplaced::Layouts(displaced)))
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
        if let Some(mut retirement) = generation2d_apply_initialization_mutation(snapshot, mutation).expect("P2 production fixture retained replay") {
            for _ in 0..GENERATION2D_MAXIMUM_DOMAIN_ITEMS {
                match retirement.close_step(1, GENERATION2D_OWNER_BYTES).expect("P2 production fixture displacement close") {
                    store::SnapshotRetirementStep::Complete => {
                        assert!(retirement.terminal_is_empty());
                        break;
                    }
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= GENERATION2D_OWNER_BYTES);
                    }
                    store::SnapshotRetirementStep::Blocked => {}
                }
            }
            assert!(retirement.terminal_is_empty());
        }
    }
}
