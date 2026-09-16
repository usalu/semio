use super::*;
use crate::editor::shooting::unit_tests::context::{render as render_body, shooting_app};

#[semio_framework_async_macros::async_test]
async fn document_lists_shots_and_assets() {
    let mut app = shooting_app().await;
    let json = render_body(&mut app, SHOOTING_PLAY_BODY_ARTIFACT).await;
    assert!(json.contains("Overview Svg"));
    assert!(json.contains("Base"));
}

//#region 🪟️WindowLaws
use crate::{ShootingAsset, ShootingShot, ShootingSnapshot};
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

/// 🪟️ A document an order of magnitude past one viewport — the subject of every window law below.
fn oversized_snapshot(shots: usize, assets: usize) -> ShootingSnapshot {
    ShootingSnapshot {
        shots: (0..shots).map(|index| ShootingShot { id: format!("shot-{index}"), label: format!("Shot {index}"), width: 256, height: 256, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: None }).collect(),
        assets: (0..assets).map(|index| ShootingAsset { id: format!("asset-{index}"), name: format!("Asset {index}"), url: String::new(), format: "glb".into(), origin: [0.0; 3], orientation: None, scale: None }).collect(),
        ..Default::default()
    }
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(snapshot: &ShootingSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let node = render(snapshot, &ShootingLabels::NATIVE_EN, &TreeWindows::for_body(&view, SHOOTING_PLAY_BODY_ARTIFACT)).expect("render the shooting document tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the shooting document tree")
}

/// 🪟️ Law (a): every container stamps its FULL extent and materialises at most its slice — no `+N`.
#[test]
fn oversized_document_stamps_totals_and_never_a_continuation_row() {
    let snapshot = oversized_snapshot(200, 120);
    let json = window_body(&snapshot, Vec::new());
    assert!(json.contains("\"total\":200"), "shots section stamps its full extent: {json}");
    assert!(json.contains("\"total\":120"), "assets section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("shooting-shot:").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing.
#[test]
fn closed_section_stamps_total_and_materialises_no_children() {
    let snapshot = oversized_snapshot(200, 120);
    let json = window_body(&snapshot, vec![TreeWindowRequest { body_key: SHOOTING_PLAY_BODY_ARTIFACT.into(), node_key: "shooting-play-document.shots".into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":200"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("shooting-shot:"), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by raw entry id.
#[test]
fn host_window_materialises_exactly_its_slice() {
    let snapshot = oversized_snapshot(200, 120);
    let json = window_body(&snapshot, vec![TreeWindowRequest { body_key: SHOOTING_PLAY_BODY_ARTIFACT.into(), node_key: "shooting-play-document.shots".into(), open: Some(true), offset: 50, rows: 10 }]);
    assert!(json.contains("\"offset\":50"), "the section reports its offset: {json}");
    for index in 50..60 {
        assert!(json.contains(&format!("shooting-shot:shot-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("shooting-shot:shot-49\""), "the row before the window stays out: {json}");
    assert!(!json.contains("shooting-shot:shot-60\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d) for an UNBOUND tree: shooting mixes shot and asset ids in one tree under two namespaces,
/// so it binds no interaction domain (see the panel's own note) — every row keeps its own action instead.
#[test]
fn unbound_tree_keeps_per_row_actions_and_declares_no_domain() {
    let snapshot = oversized_snapshot(4, 2);
    let json = window_body(&snapshot, Vec::new());
    assert!(!json.contains("interactionDomain"), "shooting's mixed-namespace tree binds no domain: {json}");
    assert!(json.contains("setShotSelection"), "shot rows keep their own action: {json}");
    assert!(json.contains("interactionSelect"), "asset rows keep their hand-built domain dispatch: {json}");
    assert!(!json.contains("granularity"), "an unbound tree stamps no pick granularity: {json}");
}
//#endregion 🪟️WindowLaws
