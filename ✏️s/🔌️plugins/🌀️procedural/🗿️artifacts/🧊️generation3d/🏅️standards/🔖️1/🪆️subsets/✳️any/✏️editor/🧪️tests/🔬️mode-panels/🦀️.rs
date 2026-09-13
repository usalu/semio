//! 📌️ Panel publication is MODE-INDEPENDENT.
//!
//! 🧾️ The Artifact / Catalogue / Inspection tabs are framework-injected panels, not windows of a mode
//! layout (`📓️audit-window-inventory-2026-09-12.md` §1.2/§3.2), so every play mode must publish the
//! same three bodies. The wgpu shell boots straight into `generate` while its focused pane is still
//! the edit-mode `procedural-main`, so the panel projection carried a `focusedWindowId` the generate
//! roster does not list — and `WindowConfigOwnerRegistry::capture` faulted on it BEFORE
//! `generation3d_render_body` ever matched a body key, taking all three app panels down with
//! `wgpu-ui.surface-not-published:framework.panel.*` while the framework-rendered History panel (which
//! returns earlier) published fine (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app_with_registry};
use semio_framework_plugin::{PluginApp, ViewModel, ViewWindowInstance};

//#region 📌️Fixture
const MODE_PANEL_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/📌️mode-panel-publication.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModePanelFixture {
    format: String,
    version: u8,
    panels: Vec<ModePanelRow>,
    cases: Vec<ModePanelCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModePanelRow {
    tab: String,
    body_key: String,
    root: String,
    contains: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModePanelCase {
    id: String,
    mode: String,
    windows: Vec<ModePanelWindow>,
    focused_window: String,
    focused_window_in_roster: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModePanelWindow {
    id: String,
    kind: String,
}

fn mode_panel_fixture() -> ModePanelFixture {
    let fixture: ModePanelFixture = serde_json::from_str(MODE_PANEL_FIXTURE_JSON).expect("mode panel publication fixture");
    assert_eq!(fixture.format, "semio.generation3d.mode-panel-publication");
    assert_eq!(fixture.version, 1);
    fixture
}

/// 🪟️ The shell roster one case declares, projected exactly the way a mounted PANEL surface is
/// (`SurfaceRole::Panel` → `ViewModel::for_panel`) — never hand-built, so the law exercises the real
/// projection the host mounts panels with.
fn panel_view(case: &ModePanelCase) -> ViewModel {
    let roster = case.windows.iter().map(|window| ViewWindowInstance { id: window.id.clone(), window_kind_id: window.kind.clone() }).collect();
    ViewModel { active_mode_id: Some(case.mode.clone()), focused_window_id: Some(case.focused_window.clone()), window_instances: roster, ..Default::default() }
        .for_panel()
}
//#endregion 📌️Fixture

//#region ⚖️Laws
/// ⚖️ LAW: every declared panel publishes a body in EVERY mode, whatever the shell's focused pane is.
/// A focused pane the active mode's roster does not list is an ordinary mode-switch race, never a
/// reason for a panel to publish nothing.
#[semio_framework_async_macros::async_test]
async fn every_panel_publishes_its_body_in_generate_mode_as_well_as_edit() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = mode_panel_fixture();
    let mut app = app_with_registry().await;
    for case in &fixture.cases {
        let view = panel_view(case);
        assert_eq!(view.window_id, None, "case {} is a panel projection, not a window one", case.id);
        for panel in &fixture.panels {
            let rendered = app.render(&panel.body_key, None, &view).await;
            let tree = match rendered {
                Ok(tree) => tree,
                Err(fault) => panic!("case {} published no body for {} ({}): {fault:?}", case.id, panel.tab, panel.body_key),
            };
            let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("panel body json");
            eprintln!("[DEBUG] mode-panel case={} panel={} bytes={}", case.id, panel.tab, json.len());
            assert!(json.contains(&panel.root), "case {} published {} without its own root {}", case.id, panel.tab, panel.root);
            for marker in &panel.contains {
                assert!(json.contains(marker), "case {} published {} without {marker}", case.id, panel.tab);
            }
        }
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: ONE shared panel projection — the body a panel publishes is byte-identical across modes.
/// A mode-specific panel body would mean a second projection to keep in step, which is exactly the
/// duplication this app must not grow.
#[semio_framework_async_macros::async_test]
async fn a_panel_publishes_the_same_body_in_every_mode() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = mode_panel_fixture();
    let mut app = app_with_registry().await;
    for panel in &fixture.panels {
        let mut baseline: Option<(&str, String)> = None;
        for case in &fixture.cases {
            let view = panel_view(case);
            let tree = app.render(&panel.body_key, None, &view).await.unwrap_or_else(|fault| panic!("case {} published no body for {}: {fault:?}", case.id, panel.tab));
            let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("panel body json");
            match &baseline {
                None => baseline = Some((case.id.as_str(), json)),
                Some((first, expected)) => assert_eq!(&json, expected, "{} published a different body in case {} than in case {first}", panel.tab, case.id),
            }
        }
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the three panels are declared ONCE on the app, with the framework tab ids and the body keys
/// the fixture names — no mode declares a panel of its own, and no mode layout carries one.
#[test]
fn the_app_declares_one_shared_panel_per_framework_tab_and_no_mode_declares_its_own() {
    let fixture = mode_panel_fixture();
    let definition = create_generation3d_app();
    for panel in &fixture.panels {
        let declared: Vec<_> = definition.panel_tabs.iter().filter(|tab| tab.kind == semio_framework_plugin::PanelTabKind::App(panel.tab.clone())).collect();
        assert_eq!(declared.len(), 1, "{} must be declared exactly once on the app", panel.tab);
        assert_eq!(declared[0].body_key.as_deref(), Some(panel.body_key.as_str()), "{} must carry its fixture body key", panel.tab);
    }
    for mode in [modes::edit::definition(), modes::generate::definition()] {
        let json = serde_json::to_string(&mode).expect("mode definition json");
        for panel in &fixture.panels {
            assert!(!json.contains(&panel.body_key), "mode {} must not carry the {} body — panels are app-scoped", mode.id, panel.tab);
        }
    }
    let generate_layout = serde_json::to_string(&modes::generate::layout()).expect("generate layout json");
    for panel in &fixture.panels {
        assert!(!generate_layout.contains(&panel.body_key), "the generate layout must not carry the {} body", panel.tab);
    }
}
//#endregion ⚖️Laws
