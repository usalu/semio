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
pub fn handle(_payload: &Reorganize, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Err(Fault::from("generation3d.reorganize.original-control-required"))
}

/// 🎛️ Borrows the retained command's original fuel, deadline and cancellation for native layout.
pub fn apply(snapshot: &Generation3dSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let maximum = cx.fuel_remaining();
    let mut completed = 0;
    let mut progress = |progress: semio_framework_os_infinite::board::schema::layout::LayoutProgress| {
        if cx.is_cancelled() || cx.deadline_exceeded() { return false; }
        cx.consume_fuel(progress.completed.saturating_sub(completed));
        completed = progress.completed;
        true
    };
    let mut control = semio_framework_os_infinite::board::schema::layout::LayoutControl::new(maximum, &mut progress);
    let options = semio_framework_os_infinite::board::schema::layout::DagLayoutOptions::default();
    with_host(&snapshot.host_snapshot, |host| {
        let mut editor = GraphEditor::new(host);
        editor.reorganize(&options, &mut control).map_err(Fault::from)?;
        Ok(Emit::mutations(editor.finish()))
    })
}
