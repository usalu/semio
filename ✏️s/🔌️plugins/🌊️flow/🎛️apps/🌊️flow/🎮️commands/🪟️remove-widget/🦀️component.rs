//! 🪟️ 🧩️ Flow play app commands command — `remove-widget`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::host_operations;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct RemoveWidget {
    pub widget_id: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the removed widget's id used to also
/// get dropped from the selection here; selection is framework-owned `InteractionState` now — no
/// `SetSelection` config mutation needed, the framework auto-prunes the deleted id out of `graph`'s
/// selection via `interaction_topology` on the next dispatch.
pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let target_id = &payload.widget_id;
    let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| host.remove_widget(target_id).is_ok());
    Ok(Emit::mutations(operations))
}
