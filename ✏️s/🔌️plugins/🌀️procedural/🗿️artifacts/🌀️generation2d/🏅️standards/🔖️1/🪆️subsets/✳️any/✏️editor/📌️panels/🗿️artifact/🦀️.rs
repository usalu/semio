//! 📄️ Generation2d play app panel — the document tree: widgets of the current fixture.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_terminology::Generation2dLabels;
use semio_s_artifact_procedural_2d_model::GENERATION2D_PLAY_APP_ID;
use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_plugin::tree_item;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL;

//#region 🔖️Constants
pub const GENERATION2D_PLAY_BODY_ARTIFACT: &str = "generation2d.play.artifact";
pub const GENERATION2D_PLAY_DOCUMENT_WIDGETS: &str = "procedural2d-play-document.widgets";
/// 🕹️ The `graph` domain this tree picks into, and the granularity a widget row is a target at.
const GENERATION2D_INTERACTION_DOMAIN: &str = "graph";
const GENERATION2D_GRANULARITY_NODE: &str = "node";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(GENERATION2D_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ Item ids are the RAW widget id (no namespace prefix) — they must equal the `graph` interaction
/// domain's target ids one-for-one, which is what lets the row declare a `granularity` and pick
/// through the tree's single `interactionSelect` binding instead of carrying an argument map of its
/// own (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, windowed 26/09/16). `_config` is
/// unused (kept for call-site symmetry with `inspection`).
pub fn render(document: &Generation2dSnapshot, _config: &Generation2dConfig, labels: &Generation2dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    PanelTreeBuilder::new("procedural2d-play-document")?
        .window_section_or_placeholder(
            windows,
            GENERATION2D_PLAY_DOCUMENT_WIDGETS,
            Some(semio_s_procedural_core_semantic_ui::ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?),
            true,
            &document.host_snapshot.widgets,
            |widget| {
                let mut node = tree_item(widget_id(widget), widget_id(widget).to_string())?;
                if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
                    props.granularity = Some(semio_s_procedural_core_semantic_ui::ui_text(GENERATION2D_GRANULARITY_NODE)?);
                }
                Ok(node)
            },
            semio_s_procedural_core_semantic_ui::ui_label(labels.none.as_str())?,
        )?
        .interaction_domain(GENERATION2D_PLAY_APP_ID, GENERATION2D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
//#endregion 🧪️Tests
