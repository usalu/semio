use semio_plugin_testing_acceptance::{__ArtifactEditor, __EditorApp, assert_history_edits_end_to_end, block_on_acceptance};
use semio_s_artifact_puzzle_5d_puzzle5d::editor::puzzle5d::Puzzle5dPlayApp;
use semio_s_artifact_puzzle_5d_testkit::puzzle5d_app_manifest_for_tests;

#[test]
fn create_part_anchor_option_history() {
    let fixtures = std::path::Path::new("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🌱create-part");
    let summary = block_on_acceptance(assert_history_edits_end_to_end::<__EditorApp<Puzzle5dPlayApp>, <Puzzle5dPlayApp as __ArtifactEditor>::Members>(
        "Puzzle 5D CreatePart",
        puzzle5d_app_manifest_for_tests,
        fixtures,
        &[],
    ));
    println!("{summary}");
    assert!(summary.contains("option proven by create-part"), "{summary}");
}

#[test]
fn history_edits_end_to_end() {
    let fixtures = std::path::Path::new("../../../../..");
    let examples = <Puzzle5dPlayApp as __ArtifactEditor>::examples();
    let summary = block_on_acceptance(assert_history_edits_end_to_end::<__EditorApp<Puzzle5dPlayApp>, <Puzzle5dPlayApp as __ArtifactEditor>::Members>(
        "puzzle",
        puzzle5d_app_manifest_for_tests,
        fixtures,
        &examples,
    ));
    println!("[history-edit-acceptance] {summary}");
}
