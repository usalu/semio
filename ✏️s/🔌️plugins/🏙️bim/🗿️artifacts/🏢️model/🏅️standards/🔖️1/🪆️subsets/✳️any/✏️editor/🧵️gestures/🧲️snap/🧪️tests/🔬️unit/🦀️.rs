use super::*;
use crate::editor::bim::gestures::tests::fixture::room;
use crate::{GridLine, Point2};

fn request(tolerance: f64) -> SnapRequest<'static> {
    SnapRequest { storey: Some("st-ground"), tolerance, ..SnapRequest::default() }
}

#[semio_framework_async_macros::async_test]
async fn an_endpoint_beats_a_midpoint_beats_the_edge_and_nothing_in_reach_is_free() {
    let snapshot = room();
    let hit = snap(&snapshot, [0.05, 0.04], &request(0.1));
    assert_eq!((hit.point, hit.kind), ([0.0, 0.0], SnapKind::Endpoint));
    let hit = snap(&snapshot, [4.03, 0.02], &request(0.1));
    assert_eq!((hit.point, hit.kind, hit.source.as_str()), ([4.0, 0.0], SnapKind::Midpoint, "w-south"));
    let hit = snap(&snapshot, [2.0, 0.05], &request(0.1));
    assert_eq!((hit.point, hit.kind), ([2.0, 0.0], SnapKind::Edge));
    let hit = snap(&snapshot, [4.0, 3.0], &request(0.1));
    assert_eq!((hit.point, hit.kind), ([4.0, 3.0], SnapKind::Free));
}

#[semio_framework_async_macros::async_test]
async fn grid_lines_snap_along_themselves_and_at_their_intersections() {
    let mut snapshot = room();
    for (id, label, start, end) in [("g-a", "A", (-1.0, 3.0), (9.0, 3.0)), ("g-1", "1", (2.0, -1.0), (2.0, 7.0))] {
        snapshot.grids.insert(id.into(), GridLine { building: "bldg-1".into(), label: label.into(), start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 } });
    }
    let hit = snap(&snapshot, [2.05, 3.04], &request(0.1));
    assert_eq!((hit.point, hit.kind), ([2.0, 3.0], SnapKind::Intersection));
    let hit = snap(&snapshot, [5.0, 3.05], &request(0.1));
    assert_eq!((hit.point, hit.kind), ([5.0, 3.0], SnapKind::Grid));
    let hit = snap(&snapshot, [2.0, 3.0], &SnapRequest { storey: Some("st-first"), ..request(0.1) });
    assert_eq!(hit.kind, SnapKind::Intersection, "grid lines belong to the building, every storey sees them");
}

#[semio_framework_async_macros::async_test]
async fn a_direction_near_the_orthogonal_snaps_onto_it_and_shift_locks_it() {
    let snapshot = ModelSnapshot::default();
    let anchor = Some([1.0, 1.0]);
    let near = snap(&snapshot, [4.0, 1.04], &SnapRequest { anchor, tolerance: 0.1, ..SnapRequest::default() });
    assert_eq!(near.kind, SnapKind::Orthogonal);
    assert!((near.point[1] - 1.0).abs() < 1e-9);
    let far = snap(&snapshot, [4.0, 1.9], &SnapRequest { anchor, tolerance: 0.1, ..SnapRequest::default() });
    assert_eq!((far.kind, far.point), (SnapKind::Free, [4.0, 1.9]));
    let locked = snap(&snapshot, [4.0, 1.9], &SnapRequest { anchor, tolerance: 0.1, lock_orthogonal: true, ..SnapRequest::default() });
    assert_eq!(locked.kind, SnapKind::Orthogonal, "with shift the nearest 45 degree direction wins however far the pointer is");
    assert!((locked.point[1] - 1.0).abs() < 1e-9 && locked.point[0] > 1.0, "locked onto the horizontal through the anchor");
}

#[semio_framework_async_macros::async_test]
async fn excluded_elements_and_extra_corners_are_honoured() {
    let snapshot = room();
    let excluded = vec!["w-south".to_string(), "w-west".to_string()];
    let hit = snap(&snapshot, [0.02, 0.02], &SnapRequest { exclude: &excluded, ..request(0.1) });
    assert_ne!(hit.source, "w-south", "a wall never snaps to itself");
    let hit = snap(&ModelSnapshot::default(), [5.02, 5.0], &SnapRequest { extra: &[[5.0, 5.0]], tolerance: 0.1, ..SnapRequest::default() });
    assert_eq!((hit.point, hit.kind), ([5.0, 5.0], SnapKind::Endpoint));
}
