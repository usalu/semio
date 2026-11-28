//! 🔁️ The FEM 2D results settle on the latest revision after every kind of edit: the twin of the FEM 3D settle laws. The mounted results jobs are driven the way the host drives them (spawn, cancel and step between renders), and each law bounds the number of host rounds a settle may take.

use crate::context::{dispatch, fem2d_app, history_verb, render, view, Fem2dApp, FEM2D_TEST_INSTANCE};
use semio_plugin::reactor::jobs::registry_driver::RegistryJobDriver;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::{add_node, add_region, set_active_example};
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results as results_window;
use std::collections::HashMap;

/// 🧰️ The host job registry: a job that ended keeps its entry, so a later `CancelJob` for it reaches a terminal job exactly like the host's.
type Drivers = HashMap<u64, (RegistryJobDriver, bool)>;

static SPAWNS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🧠️ The host rounds a settle may take after the last edit; the slab and the demo settle in a few hundred.
const SETTLE_ROUNDS: usize = 600;

async fn pump(app: &mut Fem2dApp, drivers: &mut Drivers, rounds: usize) -> String {
    pump_with(app, drivers, rounds, false).await
}

/// 🫥️ The poll the host runs while no results window is in the roster (another pane has focus, a panel swap): an empty roster.
async fn pump_with(app: &mut Fem2dApp, drivers: &mut Drivers, rounds: usize, blank_roster: bool) -> String {
    for _ in 0..rounds {
        let _ = render(app, results_window::BODY_KEY);
        let results_view = if blank_roster { semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) } else { view(results_window::WINDOW_KIND_ID) };
        let grant = app.1.cx().retained_grant();
        semio_plugin::artifact_app_laws::fixture_renew_turn(app.1.cx(), semio_plugin::artifact_app_laws::OriginalFixtureCaller::turn_fuel(), grant).expect("renew turn");
        let effects = semio_plugin::PluginApp::pending_effects(&mut app.0, Some(&results_view), app.1.cx()).await.expect("pending effects").unwrap_or_default();
        for effect in effects {
            match effect {
                semio_plugin::Effect::SpawnJob { job, kind, input, .. } => { let seq = SPAWNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed); eprintln!("[DEBUG] spawn #{seq} job={job:x} live={}", running(drivers)); assert!(seq < 80, "[DEBUG] solve respawn churn: {seq} spawns"); let mut driver = RegistryJobDriver::generous(job); driver.start(&kind, input, None).expect("job admission"); drivers.insert(job, (driver, false)); }
                semio_plugin::Effect::CancelJob { job } => { eprintln!("[DEBUG] cancel job={job:x} known={}", drivers.contains_key(&job)); if let Some((driver, _)) = drivers.get_mut(&job) { driver.cancel().expect("job cancel"); } }
                _ => {}
            }
        }
        for (driver, ended) in drivers.values_mut() { if !*ended && !matches!(driver.step().expect("job step"), semio_plugin::reactor::jobs::JobStep::Running(_)) { *ended = true; } }
        let _ = semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, FEM2D_TEST_INSTANCE, semio_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    }
    render(app, results_window::BODY_KEY)
}

fn settled(json: &str) -> bool {
    json.contains("Converged") && !json.contains("Pending") && !json.contains("Previous Results")
}

fn running(drivers: &Drivers) -> usize {
    drivers.values().filter(|(_, ended)| !ended).count()
}

fn drain(drivers: &mut Drivers) {
    for (driver, ended) in drivers.values_mut() { if !*ended { let _ = driver.cancel(); } }
    for _ in 0..4096 {
        for (driver, ended) in drivers.values_mut() { if !*ended && !matches!(driver.step(), Ok(semio_plugin::reactor::jobs::JobStep::Running(_))) { *ended = true; } }
        if running(drivers) == 0 { break; }
    }
}

async fn assert_settles(app: &mut Fem2dApp, drivers: &mut Drivers, why: &str) {
    let json = pump(app, drivers, SETTLE_ROUNDS).await;
    let ok = settled(&json) && running(drivers) == 0;
    let left = running(drivers);
    drain(drivers);
    assert!(ok, "{why}: results settle within {SETTLE_ROUNDS} rounds (jobs left {left}): {}", json.chars().take(600).collect::<String>());
}

fn node(x: f64) -> Fem2dCommand {
    Fem2dCommand::AddNode(add_node::AddNode { x, y: 1.0 })
}

/// 🧠️ LAW: an edit or undo that supersedes a solve in flight leaves the results settled on the latest revision, never on a pending stage.
#[semio_framework_async_macros::async_test]
async fn superseded_solve_settles_on_the_latest_revision_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    assert_settles(&mut app, &mut drivers, "the initial solve").await;
    dispatch(&mut app, node(1.0)).await;
    let _ = pump(&mut app, &mut drivers, 30).await;
    dispatch(&mut app, node(4.0)).await;
    let _ = pump(&mut app, &mut drivers, 30).await;
    history_verb(&mut app, "undo").await;
    assert_settles(&mut app, &mut drivers, "undo over a superseded solve").await;
}

/// 🧠️ LAW: a sequence of edits, undo, redo and an added region, each superseding the solve in flight, leaves the results settled on the last revision within the round bound and no job running.
#[semio_framework_async_macros::async_test]
async fn edit_undo_redo_add_region_sequence_settles_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    let _ = pump(&mut app, &mut drivers, 40).await;
    for step in 0..12u32 {
        dispatch(&mut app, node(step as f64)).await;
        let _ = pump(&mut app, &mut drivers, 2).await;
    }
    for verb in ["undo", "undo", "redo", "undo", "redo", "redo"] {
        history_verb(&mut app, verb).await;
        let _ = pump(&mut app, &mut drivers, 1).await;
    }
    let material = app.snapshot_ref().materials.iter().next().expect("demo material").id.as_str().to_owned();
    dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 20.0, y: 0.0, width: 3.0, height: 3.0, material_id: material, thickness: None, mesh_size: None })).await;
    let _ = pump(&mut app, &mut drivers, 3).await;
    history_verb(&mut app, "undo").await;
    history_verb(&mut app, "redo").await;
    assert_settles(&mut app, &mut drivers, "the edit, undo, redo and add region sequence").await;
}

/// 🎟️ LAW: a long session of edits, undos and redos, each superseding the solve in flight, never fills the document, config, draft or interaction Store's snapshot read issuer: the reads they hold stay below the pump pressure mark and a fresh read is still admitted at the end.
#[semio_framework_async_macros::async_test]
async fn a_long_supersede_session_never_exhausts_the_snapshot_read_issuers_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    let held = |app: &Fem2dApp| [
        app.stores.store.returned_snapshot_read_count() + app.stores.store.outstanding_snapshot_read_count(),
        app.stores.config_store.returned_snapshot_read_count() + app.stores.config_store.outstanding_snapshot_read_count(),
        app.stores.draft_store.returned_snapshot_read_count() + app.stores.draft_store.outstanding_snapshot_read_count(),
        app.stores.interaction_store.returned_snapshot_read_count() + app.stores.interaction_store.outstanding_snapshot_read_count(),
    ];
    let _ = pump(&mut app, &mut drivers, 40).await;
    let mut peak = 0usize;
    for round in 0..16u32 {
        dispatch(&mut app, node(round as f64)).await;
        let _ = pump(&mut app, &mut drivers, 2).await;
        if round % 3 == 2 { history_verb(&mut app, "undo").await; let _ = pump(&mut app, &mut drivers, 1).await; }
        peak = peak.max(held(&app).into_iter().max().unwrap_or(0));
    }
    let _ = pump(&mut app, &mut drivers, 300).await;
    let settled = held(&app).into_iter().max().unwrap_or(0);
    drain(&mut drivers);
    assert!(peak < store::os_store::snapshot_read::SNAPSHOT_READ_RETURN_PRESSURE, "held snapshot reads reached {peak} while solves were superseded");
    assert!(settled <= 2, "a settled session holds {settled} reads");
    assert!(app.stores.store.snapshot_read().is_ok(), "the issuer still admits a fresh read after the long session");
}

/// 🧠️ LAW: switching the active example while a solve is in flight cancels it and settles the results on the new example.
#[semio_framework_async_macros::async_test]
async fn example_switch_during_a_running_solve_settles_on_the_new_example_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    let _ = pump(&mut app, &mut drivers, 3).await;
    dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "empty".into() })).await;
    let _ = pump(&mut app, &mut drivers, 10).await;
    dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() })).await;
    assert_settles(&mut app, &mut drivers, "the example switch").await;
}

fn count(app: &Fem2dApp) -> usize {
    app.snapshot_ref().nodes.len()
}

/// ↩️ LAW: with the results settled after every step like the browser (solve, undo, solve, redo, solve, undo twice), each history verb moves the document by exactly one edit, redo is never lost to the re-solve of the undo, and the results settle on the final revision.
#[semio_framework_async_macros::async_test]
async fn undo_redo_between_settled_solves_keeps_counts_and_settles_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    assert_settles(&mut app, &mut drivers, "the initial solve").await;
    let base = count(&app);
    dispatch(&mut app, node(1.0)).await;
    dispatch(&mut app, node(2.0)).await;
    assert_settles(&mut app, &mut drivers, "after both edits").await;
    assert_eq!(count(&app), base + 2);
    history_verb(&mut app, "undo").await;
    assert_eq!(count(&app), base + 1, "undo");
    assert_settles(&mut app, &mut drivers, "after the first undo").await;
    history_verb(&mut app, "redo").await;
    assert_eq!(count(&app), base + 2, "redo after the undo's re-solve settled");
    assert_settles(&mut app, &mut drivers, "after the redo").await;
    history_verb(&mut app, "undo").await;
    assert_eq!(count(&app), base + 1, "second undo");
    assert_settles(&mut app, &mut drivers, "after the second undo").await;
    history_verb(&mut app, "undo").await;
    assert_eq!(count(&app), base, "third undo");
    assert_settles(&mut app, &mut drivers, "after the third undo").await;
    history_verb(&mut app, "redo").await;
    history_verb(&mut app, "redo").await;
    assert_eq!(count(&app), base + 2, "both redos");
    assert_settles(&mut app, &mut drivers, "after both redos").await;
}

/// ↩️ LAW: a redo issued while the undo's re-solve is still in flight (no settle between) lands and the results settle on it.
#[semio_framework_async_macros::async_test]
async fn redo_during_the_undo_re_solve_lands_and_settles_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    assert_settles(&mut app, &mut drivers, "the initial solve").await;
    let base = count(&app);
    dispatch(&mut app, node(1.0)).await;
    assert_settles(&mut app, &mut drivers, "after the edit").await;
    history_verb(&mut app, "undo").await;
    let _ = pump(&mut app, &mut drivers, 20).await;
    history_verb(&mut app, "redo").await;
    assert_eq!(count(&app), base + 1, "redo during the re-solve");
    assert_settles(&mut app, &mut drivers, "after the redo").await;
}

/// 🫥️ LAW: a poll with no results window in the roster (the results pane hidden for a few host rounds, as the History panel or a palette run does) after an undo or redo never leaves the results cancelled on the current revision: once the window is back the solve respawns and settles.
#[semio_framework_async_macros::async_test]
async fn a_hidden_results_window_during_undo_and_redo_still_settles_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    assert_settles(&mut app, &mut drivers, "the initial solve").await;
    let base = count(&app);
    dispatch(&mut app, node(1.0)).await;
    assert_settles(&mut app, &mut drivers, "after the edit").await;
    history_verb(&mut app, "undo").await;
    let _ = pump_with(&mut app, &mut drivers, 6, true).await;
    assert_settles(&mut app, &mut drivers, "after the undo with the window hidden").await;
    history_verb(&mut app, "redo").await;
    let _ = pump_with(&mut app, &mut drivers, 6, true).await;
    assert_eq!(count(&app), base + 1);
    assert_settles(&mut app, &mut drivers, "after the redo with the window hidden").await;
}

/// 🧭️ Runs one history verb the way the host interleaves it: the verb is admitted from the model window and its reserved job commits, the mounted results jobs then take host rounds (a solve spawns, steps and publishes between the commit and the verb's own publication), and only then the verb's typed publication settles. Answers the debug text of a refusal.
async fn history_verb_interleaved(app: &mut Fem2dApp, drivers: &mut Drivers, verb: &str, rounds: usize) -> Result<(), String> {
    use semio_plugin::artifact_app_laws::{dispatch_fixture_action, fixture_identity, fixture_mounted_policy, meta, settle_framework_reserved_admission, settle_registered_typed_operation};
    let mut action = meta("local");
    action.view_state = Some(view(semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID).into());
    let mut identity = fixture_identity();
    let admitted = dispatch_fixture_action(&mut app.0, verb, None, &action, &mut identity, app.1.cx(), fixture_mounted_policy().close).await.map_err(|fault| format!("admission {fault:?}"))?;
    settle_framework_reserved_admission(&mut app.0, admitted, app.1.cx()).await.map_err(|fault| format!("reserved commit {fault:?}"))?;
    let _ = pump(app, drivers, rounds).await;
    settle_registered_typed_operation(&mut app.0, FEM2D_TEST_INSTANCE, fixture_mounted_policy(), &mut identity, app.1.cx()).await.map(|_| ()).map_err(|fault| format!("publication {fault:?}"))
}

/// ↩️ LAW: a history verb whose publication runs after the results jobs took host rounds (a re-solve spawned by the verb's own commit) is never refused as stale, so redo stays available after an undo and the counts follow.
#[semio_framework_async_macros::async_test]
async fn history_verbs_interleaved_with_the_re_solve_are_never_refused_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    assert_settles(&mut app, &mut drivers, "the initial solve").await;
    let base = count(&app);
    dispatch(&mut app, node(1.0)).await;
    assert_settles(&mut app, &mut drivers, "after the edit").await;
    for (round, rounds) in [1usize, 3, 12].into_iter().enumerate() {
        history_verb_interleaved(&mut app, &mut drivers, "undo", rounds).await.unwrap_or_else(|refusal| panic!("undo {round}: {refusal}"));
        assert_eq!(count(&app), base, "undo {round}");
        history_verb_interleaved(&mut app, &mut drivers, "redo", rounds).await.unwrap_or_else(|refusal| panic!("redo {round}: {refusal}"));
        assert_eq!(count(&app), base + 1, "redo {round}");
    }
    assert_settles(&mut app, &mut drivers, "after the interleaved rounds").await;
}

/// 🧹️ LAW: an app whose first solve settled retires cleanly (the receipted close releases every born root and no job payload is abandoned).
#[semio_framework_async_macros::async_test]
async fn a_settled_first_solve_closes_cleanly_2d() {
    let mut app = fem2d_app();
    let mut drivers = HashMap::new();
    assert_settles(&mut app, &mut drivers, "the initial solve").await;
}

/// ↩️ LAW: an undo and a redo pressed while the edit's own solve is still in flight (the matrix and a fast user: add, Ctrl+Z, Ctrl+Shift+Z with no wait) each move the document by exactly that edit, and the results settle on the final revision with no job left running.
#[semio_framework_async_macros::async_test]
async fn undo_and_redo_pressed_over_the_edits_running_solve_settle_2d() {
    for rounds in [30usize] {
        let mut app = fem2d_app();
        let mut drivers = HashMap::new();
        assert_settles(&mut app, &mut drivers, "the initial solve").await;
        let base = count(&app);
        dispatch(&mut app, node(1.0)).await;
        let _ = pump(&mut app, &mut drivers, rounds).await;
        history_verb(&mut app, "undo").await;
        assert_eq!(count(&app), base, "undo after {rounds} rounds");
        let _ = pump(&mut app, &mut drivers, rounds).await;
        history_verb(&mut app, "redo").await;
        assert_eq!(count(&app), base + 1, "redo after {rounds} rounds");
        assert_settles(&mut app, &mut drivers, &format!("after {rounds} rounds")).await;
    }
}
