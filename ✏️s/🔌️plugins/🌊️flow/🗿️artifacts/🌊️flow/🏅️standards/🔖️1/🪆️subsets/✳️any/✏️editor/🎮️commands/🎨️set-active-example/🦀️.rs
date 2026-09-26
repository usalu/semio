//! 🎨️ Flow play app command — `set-active-example`.

use crate::examples::demo;
use crate::schema::mutations::snapshot_operations;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎨️ The navbar example id. An empty id is the picker's cleared row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ The document operations that turn `current` into the published `demo` or the default snapshot.
pub fn set_active_example_operations(payload: &SetActiveExample, current: &FlowSnapshot) -> Result<Vec<FlowMutation>, Fault> {
    let target = if payload.example_id.is_empty() {
        FlowSnapshot::default()
    } else if payload.example_id == demo::ID {
        <FlowSnapshot as store::ArtifactDsl>::parse_dsl(demo::PRIMARY_TEXT).map_err(|error| Fault::from(error.to_string()))?
    } else {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.example-unknown"), format!("setActiveExample has no example \"{}\"", payload.example_id)));
    };
    Ok(example_snapshot_operations(current, &target))
}

/// ✏️ Diffs two flow documents the way the live host bridge does — widget deletes already cascade
/// their synapses, so parallel `DisconnectWidgets` rows for those edges are dropped.
fn example_snapshot_operations(before: &FlowSnapshot, after: &FlowSnapshot) -> Vec<FlowMutation> {
    let operations = snapshot_operations(before, after);
    let removed_widgets: Vec<&str> = operations
        .iter()
        .filter_map(|operation| match operation {
            FlowMutation::DeleteWidget(payload) => Some(payload.id.as_str()),
            _ => None,
        })
        .collect();
    if removed_widgets.is_empty() {
        return operations;
    }
    let before_scene = before.to_host_snapshot();
    let cascaded_synapses: Vec<String> = before_scene
        .synapses
        .iter()
        .filter(|synapse| removed_widgets.iter().any(|id| *id == synapse.from || *id == synapse.to))
        .map(|synapse| synapse.id.clone())
        .collect();
    before_scene.retire_cold();
    operations
        .into_iter()
        .filter(|operation| match operation {
            FlowMutation::DisconnectWidgets(payload) => !cascaded_synapses.iter().any(|id| id == &payload.id),
            _ => true,
        })
        .collect()
}

/// 🎨️ Loads the published `demo` document, or restores the default snapshot when the id is empty.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(set_active_example_operations(payload, doc.snapshot)?))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
