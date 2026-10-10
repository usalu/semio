use super::*;

fn face(space: &str, adjacent: &str, part: Part, element: &str, centre: [f64; 3], area: f64) -> Face {
    Face { space: space.into(), adjacent: adjacent.into(), part, element: element.into(), centre, area }
}

#[test]
fn the_two_sides_of_a_wall_pair_and_the_closest_pieces_pair_first() {
    let faces = vec![
        face("a", "b", Part::Wall, "w", [0.0, 1.0, 1.5], 6.0),
        face("b", "a", Part::Wall, "w", [0.2, 3.0, 1.5], 3.0),
        face("b", "a", Part::Wall, "w", [0.2, 1.0, 1.5], 6.0),
        face("b", "a", Part::Wall, "other", [0.2, 1.0, 1.5], 6.0),
    ];
    assert_eq!(pair(&faces), vec![(0, 2)]);
}

#[test]
fn a_floor_pairs_with_the_ceiling_of_the_same_outline_below() {
    let faces = vec![face("up", "down", Part::Floor, "s1/floor", [2.0, 1.5, 3.0], 12.0), face("down", "up", Part::Ceiling, "s0/ceiling", [2.0, 1.5, 3.0], 12.0), face("down", "up", Part::Floor, "s0/floor", [2.0, 1.5, 0.0], 12.0)];
    assert_eq!(pair(&faces), vec![(0, 1)]);
}

#[test]
fn faces_of_other_neighbours_or_other_outlines_stay_alone() {
    let faces = vec![face("a", "b", Part::Floor, "x", [0.0, 0.0, 3.0], 10.0), face("b", "a", Part::Ceiling, "y", [0.5, 0.0, 3.0], 10.0), face("c", "a", Part::Ceiling, "y", [0.0, 0.0, 3.0], 10.0)];
    assert!(pair(&faces).is_empty());
    assert_eq!(centre_of(&[[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 2.0, 0.0], [0.0, 2.0, 0.0]]), [1.0, 1.0, 0.0]);
}
