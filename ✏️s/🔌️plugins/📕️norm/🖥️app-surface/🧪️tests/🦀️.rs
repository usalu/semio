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

//#region 🧵️RetainedCohort
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedFactoryRow {
    payload_schema: String,
    maximum_raw_bytes: usize,
    shared: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedContractRow {
    tool_id: String,
    lanes: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedRouteRow {
    id: String,
    publication_lanes: Vec<String>,
    admission: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedAppRow {
    variant: String,
    controller: String,
    document_schema: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedExpected {
    apps: usize,
    routes_per_app: usize,
    identities: usize,
    retained: usize,
    batch_only_pending_rewrite: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedFixture {
    factory: RetainedFactoryRow,
    publication_contracts: Vec<RetainedContractRow>,
    routes: Vec<RetainedRouteRow>,
    apps: Vec<RetainedAppRow>,
    expected: RetainedExpected,
}

macro_rules! assert_norm_pair {
    ($($module:ident => ($editor:ident, $viewer:ident)),+ $(,)?) => {
        $(
            semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<semio_s_plugin_norm::editor::$module::$editor, semio_s_plugin_norm::viewer::$module::$viewer>().await;
            semio_framework_plugin::testkit::assert_viewer_never_mutates::<semio_s_plugin_norm::viewer::$module::$viewer>().await;
        )+
    };
}

/// 🧵️ The language-neutral cohort law: all fifteen norm editors classify their three actions
/// `Migrated` and build against the one shared owned bounded factory. `create_app` is the real gate —
/// it runs the framework's proof↔factory join (`interactive-job.catalog-authority`) and the
/// publication-lane availability check, both of which fail closed on a mechanical classification flip.
#[semio_framework_async_macros::async_test]
async fn every_norm_editor_action_is_migrated_onto_the_shared_owned_factory() {
    let fixture: RetainedFixture = serde_json::from_str(include_str!("../../🧪️fixtures/🧫️retained-command-dispositions/🔣️.json")).unwrap();
    assert!(fixture.factory.shared);
    assert_eq!(fixture.factory.payload_schema, semio_s_plugin_norm::app_surface::NORM_RETAINED_PAYLOAD_SCHEMA);
    assert_eq!(fixture.factory.maximum_raw_bytes, semio_s_plugin_norm::app_surface::NORM_RETAINED_RAW_BYTES);
    assert_eq!(fixture.apps.len(), fixture.expected.apps);
    assert_eq!(fixture.routes.len(), fixture.expected.routes_per_app);
    assert_eq!(fixture.expected.identities, fixture.expected.apps * fixture.expected.routes_per_app);
    assert_eq!(fixture.expected.retained, fixture.expected.identities);
    assert_eq!(fixture.expected.batch_only_pending_rewrite, 0);
    assert_eq!(fixture.routes.iter().map(|route| route.id.as_str()).collect::<Vec<_>>(), semio_s_plugin_norm::app_surface::NORM_RETAINED_TOOL_IDS.to_vec());
    assert_eq!(fixture.publication_contracts.len(), semio_s_plugin_norm::app_surface::NORM_PUBLICATION_CONTRACTS.len());
    for (row, declared) in fixture.publication_contracts.iter().zip(semio_s_plugin_norm::app_surface::NORM_PUBLICATION_CONTRACTS) {
        assert_eq!(row.tool_id, declared.tool_id);
        assert_eq!(row.lanes, declared.lanes.iter().map(|lane| format!("{lane:?}")).collect::<Vec<_>>());
    }
    for (route, contract) in fixture.routes.iter().zip(&fixture.publication_contracts) {
        assert_eq!(route.admission, "migrated");
        assert_eq!(route.publication_lanes, contract.lanes);
    }

    let plugin = semio_s_plugin_norm::plugin().expect("the real norm plugin must assemble its complete surface registry");
    let mut identities = 0usize;
    for app in &fixture.apps {
        let definition = plugin.manifest.apps.iter().find(|entry| entry.id == app.controller).unwrap_or_else(|| panic!("{} is not a registered norm editor", app.controller));
        assert_eq!(definition.io.document_schema, app.document_schema);
        assert_eq!(definition.dialect.artifact_kind, format!("s.norm.{}", app.variant));
        for window in definition.window_kinds.iter() {
            for route in &fixture.routes {
                let action = window.actions.iter().find(|action| action.id == route.id).unwrap_or_else(|| panic!("{} window {} does not declare {}", app.controller, window.id, route.id));
                assert_eq!(action.semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{} {} must dispatch from the UI", app.controller, route.id);
            }
        }
        assert!(plugin.create_app(&app.controller).is_some(), "{} must build with its owned bounded factory registered", app.controller);
        identities += fixture.routes.len();
    }
    assert_eq!(identities, fixture.expected.identities);
    assert_norm_pair! {
        din4108 => (Din4108PlayApp, Din4108Viewer),
        din16798 => (Din16798PlayApp, Din16798Viewer),
        din18599 => (Din18599PlayApp, Din18599Viewer),
        en1990 => (En1990PlayApp, En1990Viewer),
        en1991 => (En1991PlayApp, En1991Viewer),
        en1992 => (En1992PlayApp, En1992Viewer),
        en1993 => (En1993PlayApp, En1993Viewer),
        en1994 => (En1994PlayApp, En1994Viewer),
        en1995 => (En1995PlayApp, En1995Viewer),
        en1996 => (En1996PlayApp, En1996Viewer),
        en1997 => (En1997PlayApp, En1997Viewer),
        en1998 => (En1998PlayApp, En1998Viewer),
        en1999 => (En1999PlayApp, En1999Viewer),
        iso16757 => (Iso16757PlayApp, Iso16757Viewer),
        vdi3805 => (Vdi3805PlayApp, Vdi3805Viewer),
    }
    eprintln!("[DEBUG] Norm retained cohort: {identities} migrated identities across {} editors on one shared owned factory", fixture.apps.len());
}
//#endregion 🧵️RetainedCohort
