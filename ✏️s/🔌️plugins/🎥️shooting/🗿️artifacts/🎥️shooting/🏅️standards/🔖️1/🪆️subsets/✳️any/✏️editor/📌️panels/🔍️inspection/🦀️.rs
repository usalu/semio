//! 🔍️ Shooting inspector — edits the selected or active shot through semantic field controls.

use crate::artifacts::shooting::{ShootingShot, ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::ShootingLabels;
use crate::editor::shooting::{shooting_action, ui_capacity_error, ui_children, ui_label, ui_node, ui_text, ui_value_list, ui_value_map, ui_value_text};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{column, field, input, section, text, BuiltNode, HasBase, InputKind, Trigger};

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_INSPECTION: &str = "shooting.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(SHOOTING_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn shot_field(shot: &ShootingShot, name: &'static str, label: &str, value: &str, kind: Option<InputKind>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let id = format!("shooting-play-inspector.shot.{name}");
    let child = if let Some(kind) = kind {
        let args = ui_value_map([("field", ui_value_text(name)?), ("shotIds", ui_value_list([ui_value_text(&shot.id)?])?)])?;
        let (action, args) = shooting_action("patchShots", Some(args))?;
        let builder = input(kind).value(ui_text(value)?);
        let builder = match args {
            Some(args) => builder.try_on_with(Trigger::Change, action, args),
            None => builder.try_on(Trigger::Change, action),
        }.map_err(|_| ui_capacity_error())?;
        ui_node(builder, &format!("{id}.input"))?
    } else {
        ui_node(text(ui_label(value)?), &format!("{id}.value"))?
    };
    ui_node(ui_children(field(ui_label(label)?), [child])?, &id)
}

fn shot_inspector_group(shot: &ShootingShot, labels: &ShootingLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let fields = [
        shot_field(shot, "label", labels.field_label.as_str(), &shot.label, Some(InputKind::Text))?,
        shot_field(shot, "format", labels.field_format.as_str(), &shot.format, None)?,
        shot_field(shot, "shape", labels.field_shape.as_str(), &shot.shape, None)?,
        shot_field(shot, "width", labels.field_width.as_str(), &shot.width.to_string(), Some(InputKind::Number))?,
        shot_field(shot, "height", labels.field_height.as_str(), &shot.height.to_string(), Some(InputKind::Number))?,
    ];
    ui_node(ui_children(section(ui_label(labels.shot.as_str())?).default_open(true), fields)?, "shooting-play-inspector.shot")
}

/// 🔍️ Resolves shot configuration selection, then the document's active shot, then a localized summary.
pub fn render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig, labels: &ShootingLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let selected = cfg.selected_shot_ids.first().and_then(|id| snapshot.shots.iter().find(|shot| &shot.id == id));
    let group = if let Some(shot) = selected.or_else(|| crate::artifacts::shooting::schema::active_shot(snapshot)) {
        shot_inspector_group(shot, labels)?
    } else {
        let summary = [
            ui_node(text(ui_label(SHOOTING_DOCUMENT_SCHEMA)?), "shooting-play-inspector.schema")?,
            ui_node(text(ui_label(&format!("{}: {}", labels.shots.as_str(), snapshot.shots.len()))?), "shooting-play-inspector.shots")?,
            ui_node(text(ui_label(&format!("{}: {}", labels.assets.as_str(), snapshot.assets.len()))?), "shooting-play-inspector.assets")?,
        ];
        ui_node(ui_children(section(ui_label(labels.inspection_title.as_str())?).default_open(true), summary)?, "shooting-play-inspector.empty")?
    };
    ui_node(ui_children(column(), [group])?, "shooting-play-inspector")
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{render as render_body, shooting_app};

    #[semio_framework_async_macros::async_test]
    async fn inspector_falls_back_to_the_active_shot() {
        let mut app = shooting_app().await;
        let json = render_body(&mut app, SHOOTING_PLAY_BODY_INSPECTION).await;
        assert!(json.contains("Shot"));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semantic_contract {
    use super::*;

    fn project(node: BuiltNode) -> serde_json::Value {
        let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire inspected semantic tree");
        serde_json::from_str(&text).expect("independent semantic JSON oracle")
    }

    #[test]
    fn shooting_semantic_panels_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️panels.json")).expect("neutral UI vectors");
        let mut snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let cfg = ShootingConfig::default();
        for row in vectors["cases"].as_array().expect("locales") {
            let labels = semio_framework_plugin::resolve_labels_for_locale::<ShootingLabels>(row["locale"].as_str().expect("locale"));
            let node = render(&snapshot, &cfg, labels).expect("shot inspector");
            let fields = &node.children[0].children;
            let bindings: Vec<serde_json::Value> = [0, 3, 4].into_iter().map(|i| serde_json::to_value(&fields[i].children[0].bindings[0]).expect("independent binding oracle")).collect();
            let tree = project(node);
            assert_eq!(tree["children"][0]["component"]["label"], row["shot"]);
            let fields = tree["children"][0]["children"].as_array().expect("shot fields");
            assert_eq!(serde_json::Value::Array(fields.iter().map(|field| field["component"]["label"].clone()).collect()), row["fields"]);
            for (index, binding) in bindings.iter().enumerate() {
                assert_eq!(binding["args"]["field"], vectors["editableFields"][index]);
                assert_eq!(binding["args"]["shotIds"][0], snapshot.active_shot_id);
                assert_eq!(binding["trigger"], "change");
            }
        }
        for node in [
            crate::editor::shooting::modes::edit::windows::scene::render(&snapshot, &cfg).expect("editor scene"),
            crate::viewer::shooting::modes::view::windows::scene::render(&snapshot).expect("viewer scene"),
        ] {
            let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("3D surface") };
            let scene: semio_framework_plugin::World3dScene = semio_framework_ui_scene::decode(props).expect("packed scene");
            let frame: serde_json::Value = serde_json::from_str(scene.frame_json.as_deref().expect("active frame")).expect("independent frame oracle");
            assert_eq!(frame, vectors["frame"]);
            project(node);
        }
        let node = crate::editor::shooting::modes::edit::windows::icon::render(&snapshot, &cfg).expect("icon scene");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("icon surface") };
        let scene: semio_framework_plugin::IconRenderScene = semio_framework_ui_scene::decode(props).expect("packed icon");
        let request: serde_json::Value = serde_json::from_str(&scene.request_json).expect("independent icon request oracle");
        assert_eq!(request["assetUrl"], vectors["iconAssetUrl"]);
        project(node);
        snapshot.shots.clear();
        snapshot.active_shot_id.clear();
        for row in vectors["cases"].as_array().expect("locales") {
            let labels = semio_framework_plugin::resolve_labels_for_locale::<ShootingLabels>(row["locale"].as_str().expect("locale"));
            let tree = project(render(&snapshot, &cfg, labels).expect("empty inspector"));
            assert_eq!(tree["children"][0]["component"]["label"], row["summary"]);
        }
    }
}
