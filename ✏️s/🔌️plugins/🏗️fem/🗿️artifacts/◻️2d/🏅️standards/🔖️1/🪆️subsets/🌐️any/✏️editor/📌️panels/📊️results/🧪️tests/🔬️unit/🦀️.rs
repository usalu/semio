use super::*;
use crate::editor::fem2d::modes::edit::windows::results::config::Fem2dResultsAnimation;
use crate::editor::fem2d::terminology::Fem2dLabels;

fn labels() -> &'static Fem2dLabels {
    semio_framework_plugin::resolve_labels::<Fem2dLabels>(&semio_framework_plugin::ViewModel::default())
}

fn panel_json(window: &Fem2dResultsWindowConfig) -> String {
    let doc = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let node = render(&doc, Some(window), "results-left", labels()).expect("results panel admission");
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

/// 🪟️ LAW: the tag every control carries is the SAME id `addressed_window_id` resolves it to — a
/// panel that tags one pane while the command writes another is the split-layout bug this closes.
#[semio_framework_async_macros::async_test]
async fn results_panel_tag_resolves_to_the_partition_the_command_writes() {
    let config = semio_framework_plugin::NoConfig::default();
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
    let split = semio_framework_plugin::ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: "results-left".into(), window_kind_id: crate::editor::fem2d::modes::edit::windows::results::WINDOW_KIND_ID.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-right".into(), window_kind_id: crate::editor::fem2d::modes::edit::windows::results::WINDOW_KIND_ID.into() },
        ],
        focused_window_id: Some("results-left".into()),
        ..Default::default()
    };
    let tag = crate::editor::fem2d::results_window_instance_id(&split).expect("a panel projection resolves a results window");
    assert_eq!(tag, "results-left", "the panel tags the focused results pane");
    let resolved = crate::editor::fem2d::modes::edit::windows::results::config::addressed_window_id(&cfg, &split, Some(&tag)).expect("the tag addresses an open results window");
    assert_eq!(resolved, tag);
    let json = panel_json(&Fem2dResultsWindowConfig::default());
    assert!(json.contains("results-left"), "every control carries the tag: {json}");
}

/// 🪟️ LAW: a panel projection that captured another pane still renders a working transport — a bare
/// play/pause toggle naming only the window — and says which pane to focus for live readouts.
#[semio_framework_async_macros::async_test]
async fn results_panel_without_a_captured_window_offers_a_toggle_and_a_focus_hint() {
    let doc = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let node = render(&doc, None, "results-left", labels()).expect("results panel admission");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("fixture projection");
    assert!(json.contains(labels().focus_results_hint.as_str()), "{json}");
    assert!(json.contains(&format!("{} / {}", labels().play.as_str(), labels().pause.as_str())), "{json}");
    assert!(json.contains("fem2d-play-results.transport.play"), "{json}");
}
