//! 🕸️ 🎯️ Flow play app commands command — `spotlight-commit`.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::editor::flow::commands::node_graph_edit::FlowNodeGraphEditOp;

//#region 🔖️SharedDispatch
/// 🔦️ A Spotlight commit is a `nodeGraphEdit` batch: the same rows, the same child edit.
pub fn node_graph_edit_result(doc: &ArtifactView<'_, FlowSnapshot>, config: &FlowMainWindowConfig, session: &FlowEvalSession, operations: &[FlowNodeGraphEditOp]) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    crate::editor::flow::commands::node_graph_edit::node_graph_edit_result(doc, config, session, operations)
}
//#endregion 🔖️SharedDispatch

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct SpotlightCommit {
    #[dsl(statements)]
    pub operations: Vec<FlowNodeGraphEditOp>,
}

/// 🔦️ The command body: the batch's child edit against the main window's config.
pub fn handle(payload: &SpotlightCommit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    node_graph_edit_result(doc, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, &payload.operations)
}
