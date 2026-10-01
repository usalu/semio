use super::*;
use crate::mutations::drag_frames::DragFrames;
use crate::mutations::rotate_frames::RotateFrames;
use crate::mutations::scale_frames::ScaleFrames;
use semio_framework_plugin::HistoryView;

fn views() -> (LayoutSnapshot, NoConfig) {
    (crate::standards::v1::subsets::any::schema::default_document(), NoConfig::default())
}

fn translate(ids: &[&str], dx: f64, dy: f64) -> TranslateSelection {
    TranslateSelection { ids: ids.iter().map(|id| id.to_string()).collect(), dx, dy, phase: None, reason: None }
}

fn centre(document: &LayoutSnapshot, id: &str) -> (f64, f64) {
    crate::mutations::layout_frame_centre(document.pages[0].frames.iter().find(|frame| frame.id() == id).expect("demo frame").bounds())
}

/// ✋️ A one-shot translate yields ONE parametric `drag-frames` leaf over the literal ids and the offset — no absolute
/// positions, no coalesce key; without command authority it carries no transaction.
#[test]
fn translate_selection_yields_one_drag_frames_leaf() {
    let (document, config) = views();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&translate(&["frame-1", "frame-1"], 5.0, -2.0), &doc, &cfg).expect("translate");
    assert_eq!(emit.artifact_mutations, vec![LayoutMutation::DragFrames(DragFrames { page_id: "page-1".into(), targets: vec!["frame-1".into()], dx: 5.0, dy: -2.0 })], "one leaf, its targets deduplicated");
    assert_eq!((emit.coalesce_key, emit.transaction), (None, None));
}

/// 🔃️ A turn and a scaling record the centroid of the frame centres as their pivot, so the leaf replays on any base.
#[test]
fn rotate_and_scale_record_the_centroid_pivot() {
    let (document, config) = views();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let ((ax, ay), (bx, by)) = (centre(&document, "frame-1"), centre(&document, "frame-text-1"));
    let pivot = ((ax + bx) / 2.0, (ay + by) / 2.0);
    let turned = rotate(&RotateSelection { ids: vec!["frame-1".into(), "frame-text-1".into()], angle: 0.5, phase: None, reason: None }, &doc, &cfg).expect("rotate");
    assert_eq!(turned.artifact_mutations, vec![LayoutMutation::RotateFrames(RotateFrames { page_id: "page-1".into(), targets: vec!["frame-1".into(), "frame-text-1".into()], pivot_x: pivot.0, pivot_y: pivot.1, angle: 0.5 })]);
    let scaled = scale(&ScaleSelection { ids: vec!["frame-1".into()], sx: 2.0, sy: 0.5, phase: None, reason: None }, &doc, &cfg).expect("scale");
    assert_eq!(scaled.artifact_mutations, vec![LayoutMutation::ScaleFrames(ScaleFrames { page_id: "page-1".into(), targets: vec!["frame-1".into()], pivot_x: ax, pivot_y: ay, sx: 2.0, sy: 0.5 })]);
}

/// 🫥️ The identity, a zero factor and a stray commit or abort leave zero trace; an unknown phase is refused untouched.
#[test]
fn identity_and_stray_phases_leave_zero_trace() {
    let (document, config) = views();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    assert!(handle(&translate(&["frame-1"], 0.0, 0.0), &doc, &cfg).expect("identity").artifact_mutations.is_empty());
    assert!(scale(&ScaleSelection { ids: vec!["frame-1".into()], sx: 0.0, sy: 1.0, phase: None, reason: None }, &doc, &cfg).expect("zero factor").artifact_mutations.is_empty());
    for phase in ["commit", "abort", "hover"] {
        let payload = TranslateSelection { phase: Some(phase.into()), ..translate(&["frame-1"], 3.0, 0.0) };
        assert!(handle(&payload, &doc, &cfg).expect(phase).artifact_mutations.is_empty(), "{phase} without an open gesture leaves zero trace");
    }
}

/// 🔒️ A request whose every target is locked (the frame or its layer) is refused at the tool with zero trace; a mixed
/// request is yielded whole and its leaf skips the locked rest as `mutation.partial`.
#[test]
fn a_locked_frame_and_a_locked_layer_stay_put() {
    let (mut document, config) = views();
    match document.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").unwrap() {
        crate::Frame::Rect { locked, .. } => *locked = Some(true),
        _ => panic!("rect"),
    }
    let history = HistoryView::empty();
    let cfg = ConfigView { snapshot: &config, window: None };
    {
        let doc = ArtifactView::new(&document, &history);
        assert!(handle(&translate(&["frame-1"], 5.0, 0.0), &doc, &cfg).unwrap().artifact_mutations.is_empty());
        assert!(rotate(&RotateSelection { ids: vec!["frame-1".into()], angle: 1.0, phase: None, reason: None }, &doc, &cfg).unwrap().artifact_mutations.is_empty());
        assert!(scale(&ScaleSelection { ids: vec!["frame-1".into()], sx: 2.0, sy: 2.0, phase: None, reason: None }, &doc, &cfg).unwrap().artifact_mutations.is_empty());
        let mixed = handle(&translate(&["frame-1", "frame-text-1"], 3.0, 0.0), &doc, &cfg).unwrap();
        assert_eq!(mixed.artifact_mutations.len(), 1, "a mixed request is yielded whole");
        let outcome = protocol::Mutation::<LayoutSnapshot>::diff(&mixed.artifact_mutations[0], &document);
        assert_eq!(outcome.messages().iter().map(|message| (message.code.0.as_str(), message.target.clone())).collect::<Vec<_>>(), vec![("mutation.partial", vec!["frame-1".to_string()])]);
    }
    document.pages[0].layers[0].locked = true;
    match document.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").unwrap() {
        crate::Frame::Rect { locked, .. } => *locked = None,
        _ => panic!("rect"),
    }
    let doc = ArtifactView::new(&document, &history);
    assert!(handle(&translate(&["frame-text-1"], 3.0, 0.0), &doc, &cfg).unwrap().artifact_mutations.is_empty());
}
