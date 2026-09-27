use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

#[test]
fn empty_folder_opens_persist_dialog() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    for folder_path in [None, Some("  ".to_owned())] {
        let emit = handle(&PersistLocally { space_id: "draft-1".into(), folder_path }, &doc, &cfg).expect("handle");
        assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::OpenDialog { dialog_id, .. } if dialog_id == "persistLocally")));
        assert!(emit.artifact_mutations.is_empty());
    }
}

/// 🚫️ The direct lane never writes: a folder-bound persist runs only as the retained job, and the job's validate stage
/// refuses by name before anything is written.
#[test]
fn a_folder_persist_runs_only_as_the_retained_job_and_validates_by_name() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let direct = handle(&PersistLocally { space_id: "draft-1".into(), folder_path: Some("/tmp/sh2-persist".into()) }, &doc, &cfg);
    assert!(matches!(direct, Err(fault) if fault.code.0.as_str() == "s.home.persist-locally.requires-retained-job"));
    for (space_id, folder_path, code) in [
        ("", "/tmp/sh2-persist", "s.home.persist-locally.studio-invalid"),
        ("draft-1", " ", "s.home.persist-locally.path-invalid"),
        ("sh2-no-such-draft", "/tmp/sh2-persist", "s.home.persist-locally.unknown-studio"),
    ] {
        assert!(matches!(validate(space_id, folder_path), Err(fault) if fault.code.0.as_str() == code), "{code}");
    }
}
