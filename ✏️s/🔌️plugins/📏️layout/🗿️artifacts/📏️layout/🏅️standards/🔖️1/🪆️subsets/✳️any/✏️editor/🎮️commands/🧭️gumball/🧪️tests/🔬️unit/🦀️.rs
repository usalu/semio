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

#[test]
fn a_locked_frame_and_a_locked_layer_stay_put() {
    let (mut document, config) = views();
    match document.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").unwrap() {
        crate::Frame::Rect { locked, .. } => *locked = Some(true),
        _ => panic!("rect"),
    }
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    assert!(handle(&TranslateSelection { ids: vec!["frame-1".into()], dx: 5.0, dy: 0.0 }, &doc, &cfg).unwrap().artifact_mutations.is_empty());
    assert!(rotate(&RotateSelection { ids: vec!["frame-1".into()], angle: 1.0 }, &doc, &cfg).unwrap().artifact_mutations.is_empty());
    assert!(scale(&ScaleSelection { ids: vec!["frame-1".into()], sx: 2.0, sy: 2.0 }, &doc, &cfg).unwrap().artifact_mutations.is_empty());
    let moved = handle(&TranslateSelection { ids: vec!["frame-text-1".into()], dx: 3.0, dy: 0.0 }, &doc, &cfg).unwrap();
    assert_eq!(moved.artifact_mutations.len(), 1);
    document.pages[0].layers[0].locked = true;
    match document.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").unwrap() {
        crate::Frame::Rect { locked, .. } => *locked = None,
        _ => panic!("rect"),
    }
    let doc = ArtifactView::new(&document, &history);
    assert!(handle(&TranslateSelection { ids: vec!["frame-text-1".into()], dx: 3.0, dy: 0.0 }, &doc, &cfg).unwrap().artifact_mutations.is_empty());
}
