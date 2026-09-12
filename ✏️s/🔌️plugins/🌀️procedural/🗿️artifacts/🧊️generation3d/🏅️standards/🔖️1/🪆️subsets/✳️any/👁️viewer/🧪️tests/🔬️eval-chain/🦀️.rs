//! ⏱️ The read-only surface's half of the ONE addressed evaluation law.
//!
//! 🐛️ Regression guard for `📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1: the viewer used
//! to build a fresh `FlowEvalSession::new()` per command and `tick` it synchronously to completion
//! with ZERO `ExtensionInvocation` sites anywhere in the surface. The brep/math operators are
//! contributed by the host at runtime and are never linked into the guest, so that loop could only
//! ever fault and no brep-bearing document could tessellate in the viewer at all.

use super::*;
use crate::viewer::generation3d::testkit::{self, app};
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, ViewWindowInstance};

/// ⚖️ The ONE language-agnostic addressing fixture both surfaces answer, with a third-party twin at
/// `✏️editor/🧪️tests/🔬️tick-addressing/contract.ts`.
const TICK_ADDRESSING_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🪟️tick-addressing.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TickAddressingFixture {
    format: String,
    version: u8,
    window_kinds: std::collections::BTreeMap<String, String>,
    arming: Vec<ArmingCase>,
    dispatch: Vec<DispatchCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttachedWindow {
    id: String,
    kind: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArmingCase {
    id: String,
    surface: String,
    attached: Vec<AttachedWindow>,
    armed_window_ids: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct DispatchCase {
    id: String,
    surface: String,
    attached: Vec<AttachedWindow>,
    current_window_id: String,
    payload_window_id: String,
    payload_window_kind: String,
    admitted: bool,
}

fn fixture() -> TickAddressingFixture {
    let fixture: TickAddressingFixture = serde_json::from_str(TICK_ADDRESSING_FIXTURE_JSON).expect("tick addressing fixture");
    assert_eq!(fixture.format, "semio.generation3d.tick-addressing");
    assert_eq!(fixture.version, 1);
    fixture
}

fn roster(fixture: &TickAddressingFixture, attached: &[AttachedWindow]) -> ViewModel {
    let window_instances = attached
        .iter()
        .map(|window| ViewWindowInstance { id: window.id.clone(), window_kind_id: fixture.window_kinds.get(&window.kind).unwrap_or_else(|| panic!("fixture window kind {}", window.kind)).clone() })
        .collect();
    ViewModel { window_instances, ..Default::default() }
}

/// ⚖️ LAW: what the READ-ONLY surface's `pending_effects` may put on the wire for a given attached
/// roster. Every armed tick names a concrete preview window, and a roster with none arms NOTHING —
/// the same law the sibling surface answers, read off the same fixture rows.
#[semio_framework_async_macros::async_test]
async fn every_armed_tick_names_a_viewer_preview_window_that_is_actually_attached() {
    let _serial = testkit::lock();
    let fixture = fixture();
    let mut app = app().await;
    for case in fixture.arming.iter().filter(|case| case.surface == "viewer") {
        let view = roster(&fixture, &case.attached);
        let armed = testkit::armed_window_ids(&app.pending_effects(Some(&view)).await);
        assert_eq!(armed, case.armed_window_ids, "arming case {}", case.id);
        eprintln!("[DEBUG] viewer tick arming {}: attached={:?} armed={armed:?}", case.id, view.window_instances.iter().map(|window| window.id.as_str()).collect::<Vec<_>>());
    }
    let empty = testkit::armed_window_ids(&app.pending_effects(None).await);
    assert!(empty.is_empty(), "no roster at all must arm nothing, got {empty:?}");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: which of those addresses the REAL retained route admits, driven through
/// `PluginApp::handle_command` → wire decode → `ArtifactRetainedCommandPhase::Preflight` → work.
#[semio_framework_async_macros::async_test]
async fn only_a_viewer_preview_addressed_tick_passes_the_retained_preflight() {
    let _serial = testkit::lock();
    let fixture = fixture();
    for case in fixture.dispatch.iter().filter(|case| case.surface == "viewer") {
        let mut app = app().await;
        let view = roster(&fixture, &case.attached).for_window_instance(&case.current_window_id).expect("the current window is attached");
        let kind = fixture.window_kinds.get(&case.payload_window_kind).cloned().unwrap_or_default();
        let args = crate::preview_eval::window_args(&case.payload_window_id, &kind);
        let action_meta = ActionMeta { view_state: Some(view), ..semio_framework_plugin::testkit::meta("local") };
        let outcome = match testkit::dispatch_effect_command(&mut app, "flowEvalTick", Some(&args), &action_meta).await {
            Ok(()) => match semio_framework_plugin::testkit::settle_registered_typed_operation(&mut *app, action_meta.instance_id).await {
                Ok(receipt) => receipt.lanes.contains(&TypedOperationResultLane::Fault).then(|| format!("retained publication faulted: {:?}", receipt.lanes)),
                Err(fault) => Some(format!("{fault:?}")),
            },
            Err(fault) => Some(format!("{fault:?}")),
        };
        eprintln!("[DEBUG] viewer tick dispatch {}: payloadWindowId={:?} refusal={outcome:?}", case.id, case.payload_window_id);
        assert_eq!(outcome.is_none(), case.admitted, "dispatch case {}: {outcome:?}", case.id);
        semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
    }
}

/// ⚖️ LAW: an admitted viewer tick either declares extension work or re-arms the chain. It must
/// NEVER complete as a host-local synchronous tick with no continuation — that shape is exactly
/// what made the viewer unable to paint a brep-bearing document.
#[semio_framework_async_macros::async_test]
async fn a_viewer_tick_emits_extension_work_or_re_arms_but_never_settles_silently() {
    let _serial = testkit::lock();
    let mut app = app().await;
    let view = testkit::view_shell_view("view-preview");
    let action_meta = ActionMeta { view_state: Some(view.clone()), ..semio_framework_plugin::testkit::meta("local") };
    let args = crate::preview_eval::window_args("view-preview", preview::WINDOW_KIND_ID);
    testkit::dispatch_effect_command(&mut app, "flowEvalTick", Some(&args), &action_meta).await.expect("viewer preview tick");
    let tick = semio_framework_plugin::testkit::settle_registered_typed_operation(&mut *app, action_meta.instance_id).await.expect("retained publication");
    assert!(!tick.lanes.contains(&TypedOperationResultLane::Fault), "viewer preview tick faulted: {:?}", tick.lanes);
    let answered = crate::brep_extension::settle(&mut *app, action_meta.instance_id).await;
    let rearmed = testkit::armed_window_ids(&tick.effects);
    eprintln!("[DEBUG] viewer preview tick: rearmed={rearmed:?} answered={answered}");
    assert!(answered > 0 || rearmed.iter().any(|id| id == "view-preview"), "the viewer evaluation must emit ExtensionInvocation or re-arm flowEvalTick, not a dead sync tick");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the SERVED ORDER on the read-only surface. The shell opens the document and only THEN
/// pushes the contributions closure, so the first evaluation always runs against a registry that
/// cannot address the geometry kernel. Installing operators into a process-wide registry publishes
/// nothing, so the recovery has to be armed by the install itself — and it has to reach a PAINTED
/// preview, because a viewer whose only window shows an empty world is the defect this lane closed.
///
/// 🧪️ A `--lib` binary LINKS both extension packs, and a linked pack shadows the contributed stub —
/// so an empty contribution table alone still evaluates and would prove nothing.
/// [`crate::flow_operators::UnlinkedFlowExtensions`] therefore retires the linked installers for the
/// length of this law, which is the served guest's actual shape: nothing linked, everything
/// contributed. The evaluate hop then crosses the real extension wire, answered in-process by
/// `🔬️brep-extension` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted() {
    let _serial = testkit::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::take();
    semio_framework_os_flow::sync_host_flow_extension_contributions("[]".to_string()).expect("an empty closure is a legal registry state — it is the state every served boot starts in");
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_err(),
        "the geometry kernel must be unaddressable before the host pushes anything, or this law proves nothing"
    );

    let mut app = app().await;
    let view = testkit::view_shell_view("view-preview");
    testkit::drain_armed_flow_eval_ticks(&mut app, &view).await;
    let faulted = testkit::render_with_view(&mut app, preview::BODY_KEY, &view).await;
    assert_eq!(testkit::preview_mesh_count(&faulted), 0, "an unaddressable geometry kernel paints nothing");

    let contributions = crate::flow_operators::staged_flow_extension_contributions_json(&[]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    let mut install_effects = Vec::new();
    for (index, page) in pages.iter().enumerate() {
        let receipt = testkit::dispatch_with_view(
            &mut app,
            Generation3dViewCommand::SetContributions(set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
            view.clone(),
        )
        .await
        .expect("contributions page");
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "contributions page {index} faulted in the retained job ladder");
        let rearms = testkit::armed_window_ids(&receipt.effects);
        if index + 1 < pages.len() {
            assert!(rearms.is_empty(), "page {index} installs nothing yet, so it owes no re-arm");
        } else {
            assert_eq!(rearms, vec!["view-preview".to_string()], "the run's last page owes exactly one re-arm per attached preview window");
            install_effects = receipt.effects.clone();
        }
    }
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_ok(),
        "the pushed closure must make the geometry kernel addressable"
    );

    let ticks = testkit::drain_armed_flow_eval_ticks_from(&mut app, &view, &install_effects).await;
    assert!(ticks > 0, "the install's own effects must be the thing that restarts the read-only chain");
    let preview_body = testkit::render_with_view(&mut app, preview::BODY_KEY, &view).await;
    let meshes = testkit::preview_mesh_count(&preview_body);
    assert!(meshes >= 1, "the re-armed viewer chain must reach a painted preview, got {meshes}");
    println!("[STATS] viewer late install re-armed {ticks} ticks and painted meshes={meshes}");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}
