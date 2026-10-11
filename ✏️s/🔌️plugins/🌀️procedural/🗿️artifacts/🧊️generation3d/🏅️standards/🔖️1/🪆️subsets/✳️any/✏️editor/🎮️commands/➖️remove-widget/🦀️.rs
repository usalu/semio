//! 🧩️ 🧩️ Generation3d play app commands command — `remove-widget`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_widget_removal, Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "remove-widget")]
pub struct RemoveWidget {
    pub widget_id: String,
}

/// 🕹️ Removes the widget with its cascade — the `disconnect-synapse` of every wire naming it, the `delete-widget-position` of
/// its layout entry and the `delete-widget` — as ONE edit; a widget the document does not hold authors nothing. No longer prunes selection itself — the framework auto-prunes `graph`'s selection after any
/// document mutation that deletes a selected id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit { artifact_mutations: generation3d_widget_removal(&doc.snapshot.host_snapshot, &payload.widget_id), ..Default::default() })
}
