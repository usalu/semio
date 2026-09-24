use super::*;
use semio_framework_plugin::HistoryView;

fn views() -> (LayoutSnapshot, NoConfig) {
    (crate::standards::v1::subsets::any::schema::default_document(), NoConfig::default())
}

#[test]
fn translate_selection_moves_the_frame_by_the_delta() {
    let (document, config) = views();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&TranslateSelection { ids: vec!["frame-1".into()], dx: 5.0, dy: -2.0 }, &doc, &cfg).expect("translate");
    let LayoutMutation::MoveFrame(moved) = &emit.artifact_mutations[0] else { panic!("move") };
    assert_eq!((moved.new_x, moved.new_y), (15.0, 8.0));
}

#[test]
fn rotate_selection_adds_the_angle() {
    let (document, config) = views();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = rotate(&RotateSelection { ids: vec!["frame-1".into()], angle: 0.5 }, &doc, &cfg).expect("rotate");
    let LayoutMutation::RotateFrame(rotated) = &emit.artifact_mutations[0] else { panic!("rotate") };
    assert_eq!(rotated.new_rotation, 0.5);
}

#[test]
fn scale_selection_resizes_and_keeps_a_minimum() {
    let (document, config) = views();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = scale(&ScaleSelection { ids: vec!["frame-1".into()], sx: 2.0, sy: 0.0 }, &doc, &cfg).expect("scale");
    let LayoutMutation::MoveFrame(moved) = &emit.artifact_mutations[0] else { panic!("move") };
    let LayoutMutation::ResizeFrame(resized) = &emit.artifact_mutations[1] else { panic!("resize") };
    assert_eq!((resized.new_width, resized.new_height), (80.0, 1.0));
    assert_eq!((moved.new_x, moved.new_y), (-10.0, 29.5));
}
