//! 🧩️ 🧩️ Generation2d play app commands command — `remove-widget`.

use crate::artifacts::generation2d::op::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use crate::artifacts::generation2d::schema::host_operations;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-widget")]
pub struct RemoveWidget {
    pub widget_id: String,
}

/// 🕹️ No longer prunes selection itself — the framework auto-prunes `graph`'s selection after any
/// document mutation that deletes a selected id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let target_id = &payload.widget_id;
    let operations = host_operations(fixture, |host| {
        let _ = host.remove_widget(target_id);
    });
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: operations, ..Default::default() })
}
