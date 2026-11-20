#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/📐️projection/🧵️linework/🦀️.rs"]
mod linework;

fn draw(segments: &[[[f32; 2]; 2]], merge: bool, radius: f32) -> String {
    let mut lines = linework::LineGraph::default();
    for segment in segments {
        lines.insert(segment.map(|[x, y]| [x, y, 0.0]));
    }
    let mut path = String::new();
    while let Some(part) = lines.next(merge, radius) {
        path.push_str(&part);
    }
    assert_eq!(lines.len(), 0);
    assert!(!lines.remove_one());
    path
}

#[test]
fn closed_triangle_merges_with_and_without_easing() {
    let lines = [[[0.0, 0.0], [10.0, 0.0]], [[10.0, 0.0], [0.0, 10.0]], [[0.0, 10.0], [0.0, 0.0]]];
    assert_eq!(draw(&lines, true, 0.0), "M0,0L0,10L10,0L0,0Z");
    let rounded = draw(&lines, true, 3.0);
    assert!(rounded.starts_with("M0,3L0,7Q0,10"));
    assert_eq!(rounded.matches('M').count(), 1);
    assert_eq!(rounded.matches('Q').count(), 3);
    assert!(rounded.ends_with("L3,0Q0,0 0,3Z"));
    let individual = draw(&lines, false, 3.0);
    assert_eq!(individual.matches('M').count(), 3);
    assert_eq!(individual.matches('Q').count(), 6);
}

#[test]
fn forks_stop_chains_and_reversed_duplicates_collapse() {
    let lines = [[[0.0, 0.0], [0.0, 10.0]], [[0.0, 10.0], [0.0, 20.0]], [[0.0, 10.0], [10.0, 10.0]], [[0.0, 20.0], [0.0, 10.0]]];
    assert_eq!(draw(&lines, true, 3.0), "M0,0L0,10M0,10L0,20M0,10L10,10");
}

#[test]
fn short_leg_clamps_corner_and_collinear_joints_remain_linear() {
    let corner = [[[0.0, 0.0], [0.0, 2.0]], [[0.0, 2.0], [10.0, 2.0]]];
    assert_eq!(draw(&corner, true, 3.0), "M0,0L0,1Q0,2 1,2L10,2");
    let straight = [[[0.0, 0.0], [0.0, 2.0]], [[0.0, 2.0], [0.0, 10.0]]];
    assert_eq!(draw(&straight, true, 3.0), "M0,0L0,2L0,10");
}
