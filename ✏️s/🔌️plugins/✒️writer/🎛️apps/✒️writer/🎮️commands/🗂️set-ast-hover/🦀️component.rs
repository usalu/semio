//! 🗂️ 🗂️ Writer play app commands command — `set-ast-hover`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::artifacts::writer::schema::{jack_ast_node_by_id, jack_ast_node_for_selection, parse_jack_ast};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
        use crate::apps::writer::testkit::app_with_jack;
    use crate::apps::writer::{WriterCommand, WRITER_PLAY_BODY_ARTIFACT, WRITER_PLAY_BODY_MAIN};
    use crate::artifacts::writer::schema::parse_jack_ast;
    use crate::artifacts::writer::writer_text;
    use semio_framework_plugin::{PluginApp, ViewModel};
    use serde_json::Value;

    #[test]
    fn set_ast_hover_updates_tree_highlight_and_scene_hover() {
        let mut app = app_with_jack();
        let root = parse_jack_ast(&writer_text(&app.snapshot().expect("projection")));
        let result = app.dispatch_typed(WriterCommand::SetAstHover(SetAstHover { id: Some(root.id.clone()) }), &semio_framework_plugin::testkit::meta("local")).expect("hover");
        assert!(result.mutations.is_empty());
        let tree_node = app.render(WRITER_PLAY_BODY_ARTIFACT, None, &ViewModel::default()).expect("render tree");
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
