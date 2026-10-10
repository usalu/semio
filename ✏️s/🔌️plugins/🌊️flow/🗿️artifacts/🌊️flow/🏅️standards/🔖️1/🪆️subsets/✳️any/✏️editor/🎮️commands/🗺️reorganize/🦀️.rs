//! 🔄️ 🔄️ Flow play app commands command — `reorganize`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::edit_rules::ContentEdit;
use crate::editor::flow::{flow_composed_content, flow_content_leaves_emit, host_from_snapshot};
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Reorganize
/// 🔄️ The reorganize document operations, extracted so the extension action can reuse them without
/// round-tripping through the command enum.
/// 🗺️ Lays the composed scene out left-to-right and publishes the new layout on the content child — nothing when the
/// layout is already the host's own arrangement.
pub fn reorganize_edit(composed: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let mut host = host_from_snapshot(composed, config, session);
    let mut progress = |_| true;
    let mut control = semio_framework_os_infinite::board::schema::layout::LayoutControl::new(100_000_000, &mut progress);
    let laid_out = host.reorganize(&semio_framework_os_infinite::board::schema::layout::DagLayoutOptions::default(), &mut control).is_ok();
    let positions: Vec<(String, f64, f64)> = if laid_out { host.host_snapshot.widgets.iter().filter_map(|widget| { let id = crate::schema::widget_id(widget); host.host_snapshot.layout.get(id).map(|entry| (id.to_string(), entry.x, entry.y)) }).collect() } else { Vec::new() };
    host.retire_cold();
    let mut edit = ContentEdit::new(flow_composed_content(composed)?);
    for (id, x, y) in &positions {
        edit.set_position(id, *x, *y);
    }
    Ok(flow_content_leaves_emit(&composed.content.child_id, edit.leaves))
}
//#endregion 🔖️Reorganize

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    reorganize_edit(&crate::flow_composed_snapshot(doc.snapshot, &doc.children)?, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
