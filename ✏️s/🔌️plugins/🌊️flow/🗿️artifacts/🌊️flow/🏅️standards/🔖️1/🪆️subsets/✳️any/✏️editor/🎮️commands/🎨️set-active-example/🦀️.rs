//! 🎨️ Flow play app command — `set-active-example`.

use crate::examples::demo;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎨️ The navbar example id. An empty id is the picker's cleared row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Replaces the composed scene with the published `demo` or the default graph and publishes it on the content
/// child — the coordinate every reader resolves, so the example is exactly what the window, the next verb and a reload
/// see. Nothing is published when the scene already is that example.
pub fn set_active_example_edit(payload: &SetActiveExample, composed: &FlowSnapshot) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let target = if payload.example_id.is_empty() {
        FlowSnapshot::default()
    } else if payload.example_id == demo::ID {
        demo::snapshot_from_text(demo::PRIMARY_TEXT).map_err(|error| Fault::from(error.to_string()))?
    } else {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.example-unknown"), format!("setActiveExample has no example \"{}\"", payload.example_id)));
    };
    let scene = crate::flow_working_scene(&target);
    Ok(crate::editor::flow::flow_scene_replacement(composed, &scene.widgets, &scene.synapses, &scene.layout))
}

/// 🎨️ Loads the published `demo` document, or restores the default snapshot when the id is empty.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    set_active_example_edit(payload, &crate::flow_composed_snapshot(doc.snapshot, &doc.children)?)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
