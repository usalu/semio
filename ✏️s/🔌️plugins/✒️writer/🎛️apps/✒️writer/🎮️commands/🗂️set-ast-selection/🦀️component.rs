//! 🗂️ 🗂️ Writer play app commands command — `set-ast-selection`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::artifacts::writer::schema::{jack_ast_node_by_id, jack_ast_node_for_selection, parse_jack_ast};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "ast-selection")]
pub struct SetAstSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetAstSelection, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let mut ops = vec![WriterConfigMutation::SetSelectedAstIds { ids: payload.ids.clone() }];
    if let Some(id) = payload.ids.first() {
        if document.language_id == "jack" {
            let root = parse_jack_ast(&writer_text(document));
            if let Some(node) = jack_ast_node_by_id(&root, id) {
                ops.push(WriterConfigMutation::SetEditorSelection { selection: Some(WriterEditorSelection { start: node.start, end: node.end }) });
            }
        }
    }
    ops.push(WriterConfigMutation::SetRevision { value: config.revision + 1 });
    Ok(Emit::config(ops))
}
