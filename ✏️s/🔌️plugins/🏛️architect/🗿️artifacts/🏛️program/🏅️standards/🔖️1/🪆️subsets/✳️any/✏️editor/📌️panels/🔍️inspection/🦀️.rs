//! 🔍️ Architect inspection panel — the document-wide register summary.

use crate::artifacts::program::ProgramSnapshot;
use crate::editor::architect::config::{active_register, ArchitectConfig};
use crate::editor::architect::{ui_children, ui_label, ui_node};
use semio_framework_ui_contract::{column, field, section, text, BuiltNode};
use semio_framework_plugin::{
    LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const ARCHITECT_BODY_INSPECTION: &str = "architect.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(ARCHITECT_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
/// `InteractionView` (matches `gis2d`'s inspection panel precedent), so this panel can no longer
/// tell which entity is currently selected — it always shows the document-wide register summary
/// now; the per-selected-entity typed inspector branches (element/stakeholder/adjacency/
/// requirement/risk/generic, keyed off the deleted `cfg.selected_ids`) are gone with it.
pub fn render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let values = [
        ("schema", "Schema", program.schema.clone()),
        ("active-register", "Active Register", active_register(cfg).to_string()),
        ("elements", "Elements", program.elements.len().to_string()),
        ("stakeholders", "Stakeholders", program.stakeholders.len().to_string()),
        ("adjacencies", "Adjacencies", program.adjacencies.len().to_string()),
        ("requirements", "Requirements", program.requirements.len().to_string()),
        ("risks", "Risks", program.risks.len().to_string()),
    ];
    let mut fields = Vec::with_capacity(values.len());
    for (key, label, value) in values {
        let id = format!("architect-inspection.summary.{key}");
        let value = ui_node(text(ui_label(value)?), &format!("{id}.value"))?;
        fields.push(ui_node(ui_children(field(ui_label(label)?), [value])?, &id)?);
    }
    let summary = ui_node(ui_children(section(ui_label("ProgramSnapshot")?).default_open(true), fields)?, "architect-inspection.summary")?;
    ui_node(ui_children(column(), [summary])?, "architect-inspection")
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[semio_framework_async_macros::async_test]
    async fn the_tab_is_the_framework_inspection_tab_bound_to_this_apps_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_INSPECTION));
        assert!(matches!(definition.group, PanelGroup::Details));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_inspector_always_summarises_the_document_wide_register_counts() {
        let program = sample_plugin();
        let cfg = ArchitectConfig::default();
        let json = crate::editor::architect::testkit::project_render(render(&program, &cfg));
        assert!(json.contains("architect-inspection.summary.schema"));
        assert!(json.contains(&program.elements.len().to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_summary_reflects_the_active_register() {
        let program = sample_plugin();
        let cfg = ArchitectConfig { active_register: "risks".into(), ..ArchitectConfig::default() };
        let json = crate::editor::architect::testkit::project_render(render(&program, &cfg));
        assert!(json.contains("\"value\":\"risks\""));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semantic_contract {
    use super::*;

    fn project(node: BuiltNode) -> serde_json::Value {
        let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
        serde_json::from_str(&text).expect("independent tree JSON oracle")
    }

    #[test]
    fn architect_semantic_panels_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️panels.json")).expect("neutral semantic vectors");
        let program = crate::artifacts::program::sample_plugin();
        let cfg = ArchitectConfig::default();
        let tree = project(render(&program, &cfg).expect("inspector"));
        let fields = tree["children"][0]["children"].as_array().expect("summary fields");
        assert_eq!(serde_json::Value::Array(fields.iter().map(|field| field["component"]["label"].clone()).collect()), vectors["summaryFields"]);
        assert_eq!(fields[1]["children"][0]["component"]["value"], vectors["activeRegister"]);
        let node = crate::editor::architect::modes::edit::windows::adjacency::render(&program, &cfg).expect("adjacency");
        let binding: serde_json::Value = serde_json::to_value(&node.children[1].children[1].bindings[0]).expect("independent adjacency binding oracle");
        project(node);
        assert_eq!(binding["args"]["cycle"], true);
        assert_eq!(serde_json::Value::Array(binding["args"].as_object().expect("arguments").keys().cloned().map(serde_json::Value::String).collect()), vectors["adjacencyArgs"]);
        let tree = project(crate::editor::architect::panels::catalogue::render().expect("catalogue"));
        let pages = tree["children"].as_array().expect("catalogue sections").iter().skip(1).map(|section| section["children"].as_array().expect("register rows").len()).collect::<Vec<_>>();
        assert_eq!(serde_json::to_value(&pages).expect("independent page oracle"), vectors["registerPages"]);
        assert_eq!(pages.iter().sum::<usize>(), vectors["registerCount"].as_u64().expect("register count") as usize);
        let text = tree.to_string();
        for action in vectors["catalogueActions"].as_array().expect("actions") { assert!(text.contains(action.as_str().expect("id"))); }
        let tree = project(crate::viewer::architect::modes::view::windows::register::render(&program).expect("viewer register"));
        for total in vectors["viewerTotals"].as_array().expect("totals") { assert!(tree.to_string().contains(total.as_str().expect("total"))); }
        for node in [
            crate::editor::architect::modes::edit::windows::graph::render(&program, &cfg).expect("graph"),
            crate::editor::architect::modes::edit::windows::register::render(&program, &cfg).expect("register"),
            crate::editor::architect::modes::edit::windows::report::render(&cfg).expect("report placeholder"),
            crate::editor::architect::modes::edit::windows::trace::render(&program).expect("trace"),
            crate::editor::architect::panels::document::render(&program, &cfg).expect("document"),
        ] { project(node); }
    }
}
