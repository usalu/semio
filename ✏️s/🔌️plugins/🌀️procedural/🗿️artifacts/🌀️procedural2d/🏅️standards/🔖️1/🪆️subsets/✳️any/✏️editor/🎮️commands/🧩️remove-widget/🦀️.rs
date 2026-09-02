//! 🧩️ 🧩️ Procedural2d play app commands command — `remove-widget`.

use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use crate::artifacts::procedural2d::schema::host_operations;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-widget")]
pub struct RemoveWidget {
    pub widget_id: String,
}

/// 🕹️ No longer prunes selection itself — the framework auto-prunes `graph`'s selection after any
/// document mutation that deletes a selected id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
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
