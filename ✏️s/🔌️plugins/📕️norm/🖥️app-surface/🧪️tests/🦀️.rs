//! 🧪️ Every public norm editor/viewer wrapper renders its declared language-neutral surface inventory.

use semio_framework_plugin::{AppDefinition, Locale, PanelTabDefinition, PluginApp, Terminology, ViewModel};
use semio_framework_plugin::testkit::project_and_retire_fixture_tree;
use std::collections::BTreeSet;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fixture {
    contract_id: String,
    rows: Vec<Surface>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Surface {
    variant: String,
    role: String,
    app_id: String,
    body_keys: Vec<String>,
}

fn panel_bodies(panels: &[PanelTabDefinition], keys: &mut BTreeSet<String>) {
    for panel in panels {
        if let Some(body) = &panel.body_key {
            keys.insert(body.clone());
        }
        panel_bodies(&panel.children, keys);
    }
}

async fn check_surface<A: PluginApp>(mut app: A, definition: &AppDefinition, fixture: &Surface) -> usize {
    assert_eq!(definition.id, fixture.app_id);
    assert_eq!(definition.role.as_str(), fixture.role);
    assert_eq!(app.app_id().await, fixture.app_id);
    assert_eq!(app.document_schema().await, format!("semio.norm.{}/v1", fixture.variant));
    assert_eq!(definition.io.document_schema, app.document_schema().await);
    assert_eq!(definition.dialect.artifact_kind, format!("s.norm.{}", fixture.variant));
    let mut keys: BTreeSet<String> = definition.window_kinds.iter().map(|window| window.body_key.clone()).collect();
    panel_bodies(&definition.panel_tabs, &mut keys);
    assert_eq!(keys, fixture.body_keys.iter().cloned().collect());
    let view = ViewModel { locale: Locale::En, terminology: Terminology::Native, ..ViewModel::default() };
    for key in &fixture.body_keys {
        let tree = app.render(key, None, &view).await.unwrap_or_else(|error| panic!("{} {key}: {error:?}", fixture.app_id));
        let projection = project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement");
        assert!(!projection.contains("Unknown body"), "{} {key}", fixture.app_id);
    }
    let unknown = app.render("unregistered.norm.surface", None, &view).await.unwrap();
    assert!(project_and_retire_fixture_tree(unknown).expect("unknown fixture observation and retirement").contains("Unknown body"));
    assert!(app.render(&"x".repeat(70_000), None, &view).await.is_err());
    eprintln!("[DEBUG] Norm public surface {}: {} declared bodies, unknown fallback, oversized rejection", fixture.app_id, fixture.body_keys.len());
    fixture.body_keys.len()
}

#[semio_framework_async_macros::async_test]
async fn norm_public_surfaces_render_all_declared_bodies() {
    let fixture: Fixture = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    assert_eq!(fixture.contract_id, "semio.norm.surface-render/v1");
    assert_eq!(fixture.rows.len(), 30);
    let plugin = semio_s_plugin_norm::plugin().expect("the real norm plugin must assemble its complete surface registry");
    assert_eq!(plugin.manifest.apps.iter().map(|app| app.id.clone()).collect::<BTreeSet<_>>(), fixture.rows.iter().map(|row| row.app_id.clone()).collect());
    assert_eq!(plugin.artifact_definitions().definitions().map(|definition| definition.identity().as_str().to_owned()).collect::<BTreeSet<_>>(), fixture.rows.iter().map(|row| format!("s.norm.{}", row.variant)).collect());
    let mut visited = BTreeSet::new();
    let mut rendered = 0;
    for row in &fixture.rows {
        assert!(visited.insert(row.app_id.clone()));
        let definition = plugin.manifest.apps.iter().find(|app| app.id == row.app_id).unwrap();
        let app = plugin.create_app(&row.app_id).expect("each manifest app must have a registered factory");
        rendered += check_surface(app, definition, row).await;
    }
    assert_eq!(visited.len(), 30);
    assert_eq!(rendered, 120);
    assert!(plugin.create_app("s.norm.unknown@1/*#editor").is_none());
    eprintln!("[DEBUG] Norm public surface fixture: 30 registered factories, 120 declared bodies, 30 unknown fallbacks, 30 oversized rejections, serde tree oracle");
}
