#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::vcs::VcsSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::vcs::standards::v1::subsets::any::schema::inferences::VcsInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::vcs::standards::v1::subsets::any::schema::inferences::VcsInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::vcs::standards::v1::subsets::any::schema::inferences::VcsInference::infer(&crate::artifacts::vcs::VcsSnapshot::default()),
        crate::artifacts::vcs::standards::v1::subsets::any::schema::inferences::VcsInference::default(),
    );
}

#[test]
fn summary_counts_the_demo_fixtures_tags_and_notes() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::vcs::VcsSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::vcs::standards::v1::subsets::any::schema::inferences::VcsInference::infer(&snapshot);
    assert_eq!(inference.summary.tag_count, 2);
    assert!(inference.summary.has_notes);
}
//#endregion 🧪️InferenceLaws
