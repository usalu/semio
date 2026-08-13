//! 🗂️ 🗂️ Writer play app commands command — `set-editor-selection`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::artifacts::writer::schema::{jack_ast_node_by_id, jack_ast_node_for_selection, parse_jack_ast};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️TextSelectShared
/// 🙈️ Shared body for `TextSelect`/`SetEditorSelection` — both stage a raw start/end range into
/// `editor_selection`, additionally resolving the covering jack AST node for jack documents.
fn text_select_operations(start: usize, end: usize, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Vec<WriterConfigMutation> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let mut ops = vec![WriterConfigMutation::SetEditorSelection { selection: Some(WriterEditorSelection { start, end }) }];
    let ids = if document.language_id == "jack" {
        let root = parse_jack_ast(&writer_text(document));
        jack_ast_node_for_selection(&root, start.min(end), start.max(end)).map(|node| vec![node.id.clone()]).unwrap_or_default()
    } else {
        Vec::new()
    };
    ops.push(WriterConfigMutation::SetSelectedAstIds { ids });
    ops.push(WriterConfigMutation::SetRevision { value: config.revision + 1 });
    ops
}
//#endregion 🔖️TextSelectShared

//#region 🔖️TextSelect
//#endregion 🔖️TextSelect

//#region 🔖️SetEditorSelection
//#endregion 🔖️SetEditorSelection

//#region 🔖️SelectAstNode
//#endregion 🔖️SelectAstNode

//#region 🔖️SetAstSelection
//#endregion 🔖️SetAstSelection

//#region 🔖️SetAstHover
//#endregion 🔖️SetAstHover

//#region 🔖️TextHover
//#endregion 🔖️TextHover

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "editor-selection")]
pub struct SetEditorSelection {
    pub start: usize,
    pub end: usize,
}

pub fn handle(payload: &SetEditorSelection, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    Ok(Emit::config(text_select_operations(payload.start, payload.end, doc, cfg)))
}
