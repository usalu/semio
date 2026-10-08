//! 🪟️ 🧩️ Flow play app commands command — `remove-widget`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::edit_rules::ContentEdit;
use crate::editor::flow::{flow_composed_content, flow_content_leaves_emit};
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct RemoveWidget {
    pub widget_id: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the removed widget's id used to also
/// get dropped from the selection here; selection is framework-owned `InteractionState` now — no
/// `SetSelection` config mutation needed, the framework auto-prunes the deleted id out of `graph`'s
/// selection via `interaction_topology` on the next dispatch.
pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let composed = crate::flow_composed_snapshot(doc.snapshot, &doc.children)?;
    let mut edit = ContentEdit::new(flow_composed_content(&composed)?);
    if !edit.remove(std::slice::from_ref(&payload.widget_id), &[]) {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("removeWidget found no widget \"{}\"", payload.widget_id)));
    }
    Ok(flow_content_leaves_emit(&composed.content.child_id, edit.leaves))
}
