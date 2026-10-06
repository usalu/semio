use super::*;

#[semio_framework_async_macros::async_test]
async fn a_whole_artifact_diff_wins_over_every_content_diff() {
    let base = FlowSnapshot::default();
    let mut replacement = base.clone();
    replacement.schema = "flow.replaced".into();
    let mut diff = diff_replace_content(Vec::new(), Vec::new(), Default::default());
    diff.absorb(diff_set_snapshot(&replacement));
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), replacement);
}
