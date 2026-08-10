//! 🗂️ Writer play app commands — text/AST selection and hover. All config-only View commands.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::artifacts::writer::engine::{jack_ast_node_by_id, jack_ast_node_for_selection, parse_jack_ast};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
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
        let root = parse_jack_ast(&document.text);
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
pub mod text_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "text-select")]
    pub struct TextSelect {
        pub start: usize,
        pub end: usize,
    }

    pub fn handle(payload: &TextSelect, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(Emit::config(text_select_operations(payload.start, payload.end, doc, cfg)))
    }
}
//#endregion 🔖️TextSelect

//#region 🔖️SetEditorSelection
pub mod set_editor_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "editor-selection")]
    pub struct SetEditorSelection {
        pub start: usize,
        pub end: usize,
    }

    pub fn handle(payload: &SetEditorSelection, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(Emit::config(text_select_operations(payload.start, payload.end, doc, cfg)))
    }
}
//#endregion 🔖️SetEditorSelection

//#region 🔖️SelectAstNode
pub mod select_ast_node {
    use super::*;

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
}
//#endregion 🔖️SelectAstNode

//#region 🔖️SetAstSelection
pub mod set_ast_selection {
    use super::*;

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
                let root = parse_jack_ast(&document.text);
                if let Some(node) = jack_ast_node_by_id(&root, id) {
                    ops.push(WriterConfigMutation::SetEditorSelection { selection: Some(WriterEditorSelection { start: node.start, end: node.end }) });
                }
            }
        }
        ops.push(WriterConfigMutation::SetRevision { value: config.revision + 1 });
        Ok(Emit::config(ops))
    }
}
//#endregion 🔖️SetAstSelection

//#region 🔖️SetAstHover
pub mod set_ast_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "ast-hover")]
    pub struct SetAstHover {
        pub id: Option<String>,
    }

    pub fn handle(payload: &SetAstHover, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        let config = cfg.snapshot;
        if payload.id != config.tree_hovered_ast_id {
            Ok(Emit::config(vec![WriterConfigMutation::SetTreeHoveredAstId { id: payload.id.clone() }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetAstHover

//#region 🔖️TextHover
pub mod text_hover {
    use super::*;

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
}
//#endregion 🔖️TextHover

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_ast_hover;
    use crate::apps::writer::testkit::app_with_jack;
    use crate::apps::writer::{WriterCommand, WRITER_PLAY_BODY_DOCUMENT, WRITER_PLAY_BODY_MAIN};
    use crate::artifacts::writer::engine::parse_jack_ast;
    use semio_framework_plugin::{PluginApp, ViewModel};
    use serde_json::Value;

    #[test]
    fn set_ast_hover_updates_tree_highlight_and_scene_hover() {
        let mut app = app_with_jack();
        let root = parse_jack_ast(&app.snapshot().expect("projection").text);
        let result = app.dispatch_typed(WriterCommand::SetAstHover(set_ast_hover::SetAstHover { id: Some(root.id.clone()) }), &semio_framework_plugin::testkit::meta("local")).expect("hover");
        assert!(result.mutations.is_empty());
        let tree_node = app.render(WRITER_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).expect("render tree");
        let tree_json = serde_json::to_string(&tree_node).unwrap();
        assert!(tree_json.contains(&root.id));
        let scene_node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewModel::default()).expect("render scene");
        let scene_value = serde_json::to_value(&scene_node).unwrap();
        let hover_json = scene_value["textEditor"]["hoverJson"].as_str().expect("hoverJson string");
        let hover_range: Value = serde_json::from_str(hover_json).unwrap();
        assert_eq!(hover_range["start"].as_u64(), Some(root.start as u64));
        assert_eq!(hover_range["end"].as_u64(), Some(root.end as u64));
    }
}
//#endregion 🧪️Tests
