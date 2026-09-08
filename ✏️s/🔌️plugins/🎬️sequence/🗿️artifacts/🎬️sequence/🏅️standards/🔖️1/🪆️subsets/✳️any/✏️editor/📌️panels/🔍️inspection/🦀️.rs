//! 🔍️ Sequence play app panel — inspection: the selected step's kind and params.

use crate::{SequenceFixture, SequenceStep};
use crate::editor::sequence::terminology::SequenceLabels;
use crate::editor::sequence::ui_label;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult, UiFixedList, PanelTreeBuilder, PluginAssemblyError, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const SEQUENCE_PLAY_BODY_INSPECTOR: &str = "sequence.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(SEQUENCE_PLAY_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &SequenceFixture, selected: &[String], labels: &SequenceLabels) -> UiAssemblyResult<BuiltNode> {
    let steps: Vec<&SequenceStep> = selected.iter().filter_map(|id| fixture.steps.iter().find(|step| &step.id == id)).collect();
    let mut children = UiFixedList::default();
    let (section_id, heading) = if let Some(step) = steps.first() {
        let mut fields = UiFixedList::<(&str, String)>::default();
        if steps.len() == 1 {
            fields.try_push((labels.id.as_str(), step.id.clone())).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector id admission failed"))?;
        }
        fields.try_push((labels.kind.as_str(), step.kind.clone())).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector kind admission failed"))?;
        fields.try_push((labels.params.as_str(), dsl::os_pack::to_json_string(&step.params))).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector parameters admission failed"))?;
        for (index, (label, value)) in fields.into_iter().enumerate() {
            let field = ui::text(ui_label(format!("{label}: {value}"))?).try_id(format!("sequence-play-inspector.field.{index}"))
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector field id admission failed"))?.try_build()
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector field admission failed"))?;
            children.try_push(field).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector child admission failed"))?;
        }
        ("sequence-play-inspector.step", labels.step.as_str())
    } else {
        let text = if selected.is_empty() { labels.select_prompt.as_str() } else { labels.step_not_found.as_str() };
        let child = ui::text(ui_label(text)?).try_id("sequence-play-inspector.message")
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector message id admission failed"))?.try_build()
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector message admission failed"))?;
        children.try_push(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector message child admission failed"))?;
        (if selected.is_empty() { "sequence-play-inspector.empty" } else { "sequence-play-inspector.missing" }, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)
    };
    PanelTreeBuilder::new("sequence-play-inspector")?.section(section_id, Some(ui_label(heading)?), true, children)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sequence::config::SequenceConfig;
    use crate::editor::sequence::terminology::sequence_play_labels;
    use crate::editor::sequence::testkit::{new_app, render as render_body};
    

    #[semio_framework_async_macros::async_test]
    async fn inspection_shows_prompt_when_nothing_selected() {
        let mut app = new_app().await;
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_INSPECTOR).await.contains("Select a step"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactApp::render` carries no
    /// `InteractionView` (only `handle`/`copy_fragment`/`cut_operations` gained one — see this
    /// ticket's `w3b-summary.md`), so the live app can never feed this panel a real selection today —
    /// `SequencePlayApp::render` always calls this with an empty slice, a documented framework gap
    /// (the same one `space`'s node-graph canvas rendering and context menu carry). This exercises
    /// `render`'s own selected-step branch directly instead of through the app's dispatch/render loop.
    #[semio_framework_async_macros::async_test]
    async fn inspection_shows_selected_step_kind() {
        let app = new_app().await;
        let fixture = app.snapshot().expect("projection").to_fixture();
        let labels = sequence_play_labels(&SequenceConfig::default());
        let node = render(&fixture, &["step-1".to_string()], labels).expect("inspector");
        let node = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire inspector");
        assert!(serde_json::to_string(&node).unwrap().contains("state.set"));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semantic_contract {
    use super::*;

    fn project(node: BuiltNode) -> serde_json::Value {
        let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
        serde_json::from_str(&text).expect("independent UI oracle")
    }

    #[test]
    fn sequence_semantic_panels_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️panels.json")).expect("neutral panels");
        let document = neural_engine::ColdOwner::new(crate::default_snapshot());
        let fixture = neural_engine::ColdOwner::new(document.to_fixture());
        for row in vectors["cases"].as_array().expect("locales") {
            let config = crate::editor::sequence::config::SequenceConfig { locale: row["locale"].as_str().expect("locale").into(), ..Default::default() };
            let labels = crate::editor::sequence::terminology::sequence_play_labels(&config);
            let document = project(crate::editor::sequence::panels::document::render(&fixture, labels).expect("document tree"));
            assert_eq!(document["children"][0]["component"]["label"], row["steps"]);
            assert_eq!(document["children"][1]["component"]["label"], row["edges"]);
            let catalogue = project(crate::editor::sequence::panels::catalogue::render(&fixture, labels).expect("catalogue"));
            assert_eq!(catalogue["children"][0]["children"][0]["component"]["label"], row["firstAction"]);
            let inspector = project(render(&fixture, &["step-2".into()], labels).expect("selected inspector"));
            assert_eq!(inspector["children"][0]["component"]["label"], row["selectedHeading"]);
            assert_eq!(inspector["children"][0]["children"][1]["component"]["value"], row["selectedLine"]);
            for (selection, expected) in [(Vec::<String>::new(), &row["prompt"]), (vec!["missing".into()], &row["missing"])] {
                let inspector = project(render(&fixture, &selection, labels).expect("inspector message"));
                assert_eq!(&inspector["children"][0]["children"][0]["component"]["value"], expected);
            }
        }
    }
}
