use super::*;

#[test]
fn identity_matrix_is_neutral() {
    for &t in &Transform2d::ALL {
        assert_eq!(t.semio_compose_rs(Transform2d::Identity), t);
        assert_eq!(Transform2d::Identity.semio_compose_rs(t), t);
    }
}

#[test]
fn inverse_composes_to_identity() {
    for &t in &Transform2d::ALL {
        assert_eq!(t.semio_compose_rs(t.inverse()), Transform2d::Identity);
        assert_eq!(t.inverse().semio_compose_rs(t), Transform2d::Identity);
    }
}

#[test]
fn four_quarter_rotations_is_identity() {
    let mut t = Transform2d::Identity;
    for _ in 0..4 {
        t = t.semio_compose_rs(Transform2d::Rot90);
    }
    assert_eq!(t, Transform2d::Identity);
}

#[test]
fn two_flips_is_identity() {
    for &t in &[Transform2d::FlipH, Transform2d::FlipV, Transform2d::FlipDiag, Transform2d::FlipAntiDiag] {
        assert_eq!(t.semio_compose_rs(t), Transform2d::Identity);
    }
}

#[test]
fn group_closure_every_composition_stays_in_d4() {
    for &a in &Transform2d::ALL {
        for &b in &Transform2d::ALL {
            let c = a.semio_compose_rs(b);
            assert!(Transform2d::ALL.contains(&c));
        }
    }
}

#[test]
fn apply_offset_matches_apply_window_orientation() {
    // Rotating the offset (1,0) ("east") by Rot90 should match where the tile that was at the
    // window's east edge ends up after rotating the window itself.
    let w = 3usize;
    let h = 3usize;
    let tiles: Vec<TileId> = (0..9).map(TileId).collect();
    let (nw, nh, rotated) = Transform2d::Rot90.apply_window(w, h, &tiles);
    assert_eq!((nw, nh), (h, w));
    // The east-offset direction (1,0) rotates to (0,1) under Rot90.
    assert_eq!(Transform2d::Rot90.apply_offset((1, 0)), (0, 1));
    // Sanity: rotated window is a permutation of the same 9 tiles.
    let mut sorted = rotated;
    sorted.sort();
    let mut expected: Vec<TileId> = tiles;
    expected.sort();
    assert_eq!(sorted, expected);
}

#[test]
fn apply_window_round_trips_through_inverse() {
    let w = 3usize;
    let h = 2usize;
    let tiles: Vec<TileId> = (0..6).map(TileId).collect();
    for &t in &Transform2d::ALL {
        let (mw, mh, mid) = t.apply_window(w, h, &tiles);
        let (rw, rh, back) = t.inverse().apply_window(mw, mh, &mid);
        assert_eq!((rw, rh), (w, h), "transform {t:?} did not round-trip dimensions");
        assert_eq!(back, tiles, "transform {t:?} did not round-trip content");
    }
}

#[test]
fn d4_group_has_eight_elements() {
    assert_eq!(SymmetryGroup2d::D4.elements().len(), 8);
    assert_eq!(SymmetryGroup2d::C4.elements().len(), 4);
    assert_eq!(SymmetryGroup2d::D2.elements().len(), 4);
    assert_eq!(SymmetryGroup2d::None.elements().len(), 1);
}

#[test]
fn cube_rotation_group_has_exactly_24_elements() {
    let rots = SymmetryGroup3d::Rot24.elements();
    assert_eq!(rots.len(), 24);
    assert!(rots.iter().all(|t| t.determinant() == 1), "every proper rotation must have determinant +1");
}

#[test]
fn cube_full_symmetry_group_has_exactly_48_elements() {
    let full = SymmetryGroup3d::Full48.elements();
    assert_eq!(full.len(), 48);
    let proper = full.iter().filter(|t| t.determinant() == 1).count();
    let improper = full.iter().filter(|t| t.determinant() == -1).count();
    assert_eq!(proper, 24);
    assert_eq!(improper, 24);
}

#[test]
fn cube_rotations_are_closed_under_composition() {
    let rots = SymmetryGroup3d::Rot24.elements();
    for &a in &rots {
        for &b in &rots {
            let c = a.semio_compose_rs(b);
            assert!(rots.contains(&c), "composition left the rotation group");
        }
    }
}

#[test]
fn cube_rotation_inverse_composes_to_identity() {
    let rots = SymmetryGroup3d::Rot24.elements();
    let id = Transform3d::identity();
    for &t in &rots {
        assert_eq!(t.semio_compose_rs(t.inverse()), id);
        assert_eq!(t.inverse().semio_compose_rs(t), id);
    }
}

#[test]
fn cube_offset_transform_preserves_unit_offset_length() {
    let rots = SymmetryGroup3d::Rot24.elements();
    for &t in &rots {
        for &axis in &[(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            let (x, y, z) = t.apply_offset(axis);
            assert_eq!(x.abs() + y.abs() + z.abs(), 1, "a rotation must map a face offset to another face offset");
        }
    }
}

#[test]
fn z_rot4_is_four_distinct_quarter_turns_returning_to_identity() {
    let elements = SymmetryGroup3d::ZRot4.elements();
    assert_eq!(elements.len(), 4);
    assert_eq!(elements[0], Transform3d::identity());
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(elements[i], elements[j], "ZRot4 elements must be pairwise distinct");
        }
    }
    // A fifth quarter-turn from the last element returns to identity.
    let z90 = elements[1];
    assert_eq!(elements[3].semio_compose_rs(z90), Transform3d::identity());
}

#[test]
fn custom_half_turn_groups_match_neutral_offset_vectors() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔀️topology-contracts/🔣️.json")).unwrap();
    let input2d: (i32, i32) = serde_json::from_value(oracle["halfTurn2d"]["input"].clone()).unwrap();
    let planar = SymmetryGroup2d::Custom(vec![Transform2d::Identity, Transform2d::Rot180]).elements();
    let output2d: Vec<_> = planar.iter().map(|t| t.apply_offset(input2d)).collect();
    assert_eq!(serde_json::to_value(output2d).unwrap(), oracle["halfTurn2d"]["outputs"]);
    let input3d: (i32, i32, i32) = serde_json::from_value(oracle["halfTurn3d"]["input"].clone()).unwrap();
    let unchanged: Vec<_> = SymmetryGroup3d::None.elements().iter().map(|t| t.apply_offset(input3d)).collect();
    assert_eq!(unchanged, vec![input3d]);
    let half_turn = SymmetryGroup3d::ZRot4.elements()[2];
    let spatial = SymmetryGroup3d::Custom(vec![Transform3d::identity(), half_turn]).elements();
    let output3d: Vec<_> = spatial.iter().map(|t| t.apply_offset(input3d)).collect();
    assert_eq!(serde_json::to_value(output3d).unwrap(), oracle["halfTurn3d"]["outputs"]);
    for &a in &planar {
        for &b in &planar {
            assert!(planar.contains(&a.semio_compose_rs(b)));
        }
    }
    for &a in &spatial {
        for &b in &spatial {
            assert!(spatial.contains(&a.semio_compose_rs(b)));
        }
    }
}
