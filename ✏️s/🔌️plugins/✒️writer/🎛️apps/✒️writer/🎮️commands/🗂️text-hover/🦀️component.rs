//! 🗂️ 🗂️ Writer play app commands command — `text-hover`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::artifacts::writer::schema::{jack_ast_node_by_id, jack_ast_node_for_selection, parse_jack_ast};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "text-hover")]
pub struct TextHover {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

pub fn handle(payload: &TextHover, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let offset = match (payload.start, payload.end) {
        (Some(s), Some(e)) => Some(s + e.saturating_sub(s) / 2),
        _ => None,
    };
    if offset != config.editor_hover_offset {
        Ok(Emit::config(vec![WriterConfigMutation::SetEditorHoverOffset { offset }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
    } else {
        Ok(Emit::default())
    }
}
