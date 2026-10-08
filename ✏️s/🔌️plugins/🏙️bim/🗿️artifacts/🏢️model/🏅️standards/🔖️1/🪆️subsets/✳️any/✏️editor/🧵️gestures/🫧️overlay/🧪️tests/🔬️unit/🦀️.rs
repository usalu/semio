use super::*;
use crate::editor::bim::gestures::session::{Mark, Preview};
use crate::editor::bim::gestures::snap::{SnapHit, SnapKind};

fn role_of(record: &DslValue) -> String {
    semio_framework_pack_json::to_json_string(record)
}

#[semio_framework_async_macros::async_test]
async fn marks_become_overlay_records_mirrored_into_the_windows_space() {
    let preview = Preview::of(vec![Mark::path(&[[0.0, 1.0], [2.0, 3.0]], false, Style::Ghost), Mark::dot([1.0, 1.0], Style::Handle), Mark::label([0.5, 0.5], "2.00 m")]);
    let records = records(&preview, 0.01);
    assert_eq!(records.len(), 3);
    let text = records.iter().map(role_of).collect::<Vec<_>>();
    assert!(text.iter().all(|json| json.contains("overlay")), "every record paints over the plan");
    assert!(text[0].contains("-3"), "y is mirrored: model y 3 is window y -3");
    assert!(text[2].contains("2.00 m"));
}

#[semio_framework_async_macros::async_test]
async fn every_snap_kind_has_its_own_marker_and_free_has_none() {
    let marker = |kind: SnapKind| records(&Preview::of(vec![Mark::snap(&SnapHit { point: [1.0, 1.0], kind, source: String::new() })]), 0.01).len();
    for kind in [SnapKind::Endpoint, SnapKind::Midpoint, SnapKind::Intersection, SnapKind::Grid, SnapKind::Edge, SnapKind::Orthogonal] {
        assert_eq!(marker(kind), 1, "{kind:?}");
    }
    assert_eq!(marker(SnapKind::Free), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_degenerate_mark_paints_nothing_and_an_empty_preview_no_records() {
    assert!(records(&Preview::default(), 0.01).is_empty());
    assert!(records(&Preview::of(vec![Mark::path(&[[0.0, 0.0]], false, Style::Ghost)]), 0.01).is_empty());}
