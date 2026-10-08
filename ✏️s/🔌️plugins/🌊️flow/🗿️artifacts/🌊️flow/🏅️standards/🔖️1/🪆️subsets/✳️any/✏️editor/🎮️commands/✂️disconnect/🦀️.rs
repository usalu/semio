//! 🔗️ 🔗️ Flow play app commands command — `disconnect`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::edit_rules::ContentEdit;
use crate::editor::flow::{flow_composed_content, flow_content_leaves_emit};
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct Disconnect {
    pub synapse_id: String,
}

pub fn handle(payload: &Disconnect, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let composed = crate::flow_composed_snapshot(doc.snapshot, &doc.children)?;
    let mut edit = ContentEdit::new(flow_composed_content(&composed)?);
    if !edit.remove_edge(&payload.synapse_id) {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("disconnect found no synapse \"{}\"", payload.synapse_id)));
    }
    Ok(flow_content_leaves_emit(&composed.content.child_id, edit.leaves))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
