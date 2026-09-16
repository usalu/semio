use super::*;
use crate::editor::fem2d::modes::edit::windows::results::config::Fem2dResultsAnimation;
use crate::editor::fem2d::terminology::Fem2dLabels;

fn labels() -> &'static Fem2dLabels {
    semio_framework_plugin::resolve_labels::<Fem2dLabels>(&semio_framework_plugin::ViewModel::default())
}

fn panel_json(window: &Fem2dResultsWindowConfig) -> String {
    let doc = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let node = render(&doc, window, "results-left", labels()).expect("results panel admission");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("fixture projection")
}

/// 📊️ LAW: every section of the panel renders, and every control binds the command that owns the
/// field it changes — a transport whose buttons name no action is a dead panel.
#[semio_framework_async_macros::async_test]
async fn results_panel_binds_every_transport_control_to_its_owning_command() {
    let json = panel_json(&Fem2dResultsWindowConfig::default());
    for id in ["fem2d-play-results.display", "fem2d-play-results.playback", "fem2d-play-results.analysis", "fem2d-play-results.phase", "fem2d-play-results.speed", "fem2d-play-results.transport.play"] {
        assert!(json.contains(id), "missing {id}: {json}");
    }
    for action in [DISPLAY_ACTION, PLAYBACK_ACTION, ANALYSIS_ACTION] {
        assert!(json.contains(action), "missing action {action}: {json}");
    }
    for field in ["sourceId", "mode", "modeIndex", "phase", "phaseStep", "speed", "loopMode", "waveform", "playing", "modalCount", "bucklingCount", "deformationScale"] {
        assert!(json.contains(field), "missing field binding {field}: {json}");
    }
}

/// 🪟️ LAW: every control is tagged with the results window it speaks for, so a split layout cannot
/// retune the pane the user is not looking at.
#[semio_framework_async_macros::async_test]
async fn results_panel_tags_every_control_with_its_window() {
    let json = panel_json(&Fem2dResultsWindowConfig::default());
    assert!(json.contains("windowId"), "{json}");
    assert!(json.contains("results-left"), "{json}");
}

/// ⏯️ LAW: the play button flips to a pause button while the window runs.
#[semio_framework_async_macros::async_test]
async fn results_panel_play_button_switches_to_pause_while_running() {
    let stopped = panel_json(&Fem2dResultsWindowConfig::default());
    assert!(stopped.contains(labels().play.as_str()), "{stopped}");
    let running = Fem2dResultsWindowConfig { animation: Fem2dResultsAnimation { playing: true, ..Fem2dResultsAnimation::default() }, ..Fem2dResultsWindowConfig::default() };
    let running = panel_json(&running);
    assert!(running.contains(labels().pause.as_str()), "{running}");
}

/// 📚️ LAW: the source select offers every load case AND every combination of the live document.
#[semio_framework_async_macros::async_test]
async fn results_panel_source_select_offers_cases_and_combinations() {
    let doc = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let json = panel_json(&Fem2dResultsWindowConfig::default());
    for case in &doc.load_cases {
        assert!(json.contains(case.id.as_str()), "missing load case {}: {json}", case.id);
    }
    for combination in &doc.combinations {
        assert!(json.contains(combination.id.as_str()), "missing combination {}: {json}", combination.id);
    }
}
