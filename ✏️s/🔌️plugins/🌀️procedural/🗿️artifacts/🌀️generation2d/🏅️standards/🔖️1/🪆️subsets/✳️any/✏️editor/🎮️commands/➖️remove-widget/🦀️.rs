//! 🧩️ 🧩️ Generation2d play app commands command — `remove-widget`.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use semio_s_artifact_procedural_2d_mutations::host_remove_widget;
use semio_s_artifact_procedural_2d_model::with_host;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "remove-widget")]
pub struct RemoveWidget {
    pub widget_id: String,
}

/// 🕹️ No longer prunes selection itself — the framework auto-prunes `graph`'s selection after any
/// document mutation that deletes a selected id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.host_snapshot;
    let target_id = &payload.widget_id;
    let operations = with_host(fixture, |host| host_remove_widget(host, target_id)).unwrap_or_default();
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: operations, ..Default::default() })
}
