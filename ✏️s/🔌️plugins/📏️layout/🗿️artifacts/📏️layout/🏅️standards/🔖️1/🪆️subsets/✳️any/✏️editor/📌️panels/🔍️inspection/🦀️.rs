//! 🔍️ Layout play app panel — the inspector: a document summary (was field editors for the current
//! selection; see `render`'s doc comment for why that's gone).

use crate::{LayoutSnapshot, LAYOUT_DOCUMENT_SCHEMA};
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::terminology::LayoutLabels;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiAssemblyResult, BuiltNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use crate::editor::layout::ui_label;

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_INSPECTION: &str = "layout.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(LAYOUT_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to switch on
/// `config.selected_ids` to show one field-editor group per selected page/frame (name, bounds,
/// fill/stroke, story content, wrap mode, link path). Selection is now framework-owned
/// (`InteractionView`, threaded only into `handle`/`copy_fragment`/`cut_operations`) and
/// `ArtifactApp::render` never gained that parameter, so this panel has no live selection to render
/// against and always falls through to the document summary below — the same gap gis2d's and
/// puzzle3d's inspection panels flag (see this ticket's w3b-summary.md). Not fixed here (framework
/// file, out of this crate's remit).
pub fn render(doc: &LayoutSnapshot, config: &LayoutConfig, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let mut section = semio_framework_ui_contract::section(ui_label(labels.inspection.as_str())?).default_open(true).try_id("layout-play-inspector.empty").map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout inspector id admission failed"))?;
    for (index, value) in [
        format!("{}: {}", labels.schema.as_str(), LAYOUT_DOCUMENT_SCHEMA),
        format!("{}: {}", labels.name.as_str(), doc.name),
        format!("{}: {}", labels.pages.as_str(), doc.pages.len()),
        format!("{}: {}", labels.active_page.as_str(), config.active_page_id),
    ].into_iter().enumerate() {
        let child = semio_framework_ui_contract::text(ui_label(value)?).try_id(format!("layout-inspector.summary.{index}")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout summary key admission failed"))?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout summary text admission failed"))?;
        section = section.try_child(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout inspector child admission failed"))?;
    }
    section.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout inspector node admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{layout_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn the_inspector_always_summarises_the_document() {
        let mut app = layout_app().await;
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_INSPECTION).await;
        assert!(json.contains(LAYOUT_DOCUMENT_SCHEMA));
        assert!(json.contains("page-1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_INSPECTION));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semantic_contract {
    use super::*;
    #[test]
    fn layout_inspection_summary_matches_the_json_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️summary.json")).expect("neutral inspector vectors");
        let mut snapshot = crate::schema::default_document();
        snapshot.name = fixture["name"].as_str().expect("document name").into();
        snapshot.pages.clear();
        assert_eq!(snapshot.pages.len(), fixture["pageCount"].as_u64().expect("page count") as usize);
        for row in fixture["cases"].as_array().expect("locales") {
            let config = LayoutConfig { locale: row["locale"].as_str().expect("locale").into(), active_page_id: fixture["activePage"].as_str().expect("active page").into(), ..LayoutConfig::default() };
            let node = render(&snapshot, &config, crate::editor::layout::terminology::layout_labels(&config)).expect("semantic inspector");
            let projection = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project and retire inspector");
            let actual: serde_json::Value = serde_json::from_str(&projection).expect("independent semantic JSON oracle");
            assert_eq!(actual["component"]["label"], row["heading"]);
            let lines: Vec<_> = actual["children"].as_array().expect("summary lines").iter().map(|child| child["component"]["value"].clone()).collect();
            assert_eq!(lines, *row["lines"].as_array().expect("expected lines"));
        }
    }
}
