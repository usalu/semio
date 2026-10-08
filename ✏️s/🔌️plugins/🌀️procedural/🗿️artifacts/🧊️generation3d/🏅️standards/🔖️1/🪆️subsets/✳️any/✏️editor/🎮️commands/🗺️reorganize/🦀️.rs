//! 🕸️ 🕸️ Generation3d play app commands command — `reorganize`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::standards::v1::subsets::any::schema::{with_host, GraphEditor};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

/// 🗺️ Lays the graph out again: one `move-widget` per widget the layered layout moves, as ONE edit.
pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let host_snapshot = &doc.snapshot.host_snapshot;
    with_host(host_snapshot, |host| {
        let mut editor = GraphEditor::new(host);
        if editor.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
            Ok(Emit::mutations(editor.finish()))
        } else {
            Ok(Emit::default())
        }
    })
}
