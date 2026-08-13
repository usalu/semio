//! 🗂️ 🗂️ Writer play app commands command — `select-ast-node`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::artifacts::writer::schema::{jack_ast_node_by_id, jack_ast_node_for_selection, parse_jack_ast};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "select-ast-node")]
pub struct SelectAstNode {
    pub id: String,
    pub start: usize,
    pub end: usize,
}

pub fn handle(payload: &SelectAstNode, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let ids = if payload.id.is_empty() { Vec::new() } else { vec![payload.id.clone()] };
    Ok(Emit::config(vec![
        WriterConfigMutation::SetSelectedAstIds { ids },
        WriterConfigMutation::SetEditorSelection { selection: Some(WriterEditorSelection { start: payload.start, end: payload.end }) },
        WriterConfigMutation::SetRevision { value: config.revision + 1 },
    ]))
}
