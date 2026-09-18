//! 🔬️ Snapshot facet laws — the default document is a legal 1×1×1 grid, the addressing helpers agree
//! with the collections they index, and the non-uniform axis maths is exact.

use super::*;

#[test]
fn the_default_document_is_a_legal_single_cell_grid() {
    let snapshot = Grid3dSnapshot::default();
    assert_eq!(snapshot.schema, WFC_GRID3D_DOCUMENT_SCHEMA);
    assert_eq!((snapshot.width, snapshot.height, snapshot.depth), (1, 1, 1));
    assert_eq!(snapshot.cell_sizes_x.len(), snapshot.width as usize);
    assert!(cell_in_grid(&snapshot, 0, 0, 0));
    assert!(!cell_in_grid(&snapshot, 1, 0, 0));
}

#[test]
fn an_axis_offset_is_the_cumulative_sum_of_every_size_before_it() {
    let sizes = [1.0, 2.0, 0.5];
    assert_eq!(axis_offset(&sizes, 0), 0.0);
    assert_eq!(axis_offset(&sizes, 1), 1.0);
    assert_eq!(axis_offset(&sizes, 2), 3.0);
    assert_eq!(axis_offset(&sizes, 3), 3.5);
    assert_eq!(axis_size(&sizes, 1), 2.0);
    assert_eq!(axis_size(&sizes, 9), 1.0, "an index the array does not reach falls back to a unit cell");
}

#[test]
fn resizing_an_axis_truncates_or_extends_with_the_last_authored_size() {
    assert_eq!(resized_axis(&[1.0, 2.0, 3.0], 2), vec![1.0, 2.0]);
    assert_eq!(resized_axis(&[1.0, 2.0], 4), vec![1.0, 2.0, 2.0, 2.0]);
    assert_eq!(resized_axis(&[], 2), vec![1.0, 1.0]);
}

#[test]
fn every_direction_maps_to_its_own_stencil_slot_and_its_own_opposite() {
    let mut seen = Vec::new();
    for direction in Grid3dDirection::ALL {
        assert_eq!(direction.opposite().opposite(), direction);
        assert_ne!(direction.opposite(), direction);
        seen.push(direction.stencil_index());
    }
    seen.sort_unstable();
    assert_eq!(seen, vec![0, 1, 2, 3, 4, 5], "the six directions cover the six Face6 offsets exactly once");
}

#[test]
fn the_cell_key_is_the_canonical_sort_key_the_collections_use() {
    assert_eq!(cell_key(1, 2, 3), "1:2:3");
    let snapshot = crate::examples::blocks::snapshot();
    assert!(pinned_index(&snapshot, 0, 0, 0).is_some());
    assert!(pinned_index(&snapshot, 3, 2, 3).is_none());
    assert!(masked_index(&snapshot, 3, 2, 3).is_some());
    assert!(tile_index(&snapshot, "wall").is_some());
    assert!(tile_index(&snapshot, "nothing").is_none());
    assert!(rule_index(&snapshot, &snapshot.rules[0].id).is_some());
}

#[test]
fn tile_media_round_trips_through_its_own_value_bridge() {
    let media = Grid3dTileMedia::Mesh { mesh: Grid3dMesh { positions: vec![0.0, 0.0, 0.0], indices: vec![0], color: Some(Grid3dColor { r: 1, g: 2, b: 3, a: 4 }) } };
    let value = dsl::ToValue::to_value(&media);
    let decoded: Grid3dTileMedia = dsl::FromValue::from_value(value).expect("tile media decodes");
    assert_eq!(decoded, media);
    let child = Grid3dTileMedia::MeshChild { child: crate::mesh_child_handle("mesh-a") };
    let value = dsl::ToValue::to_value(&child);
    let decoded: Grid3dTileMedia = dsl::FromValue::from_value(value).expect("mesh child decodes");
    assert_eq!(decoded, child);
}

#[test]
fn every_direction_carries_its_screaming_wire_token() {
    for (direction, token) in [
        (Grid3dDirection::Left, "LEFT"),
        (Grid3dDirection::Right, "RIGHT"),
        (Grid3dDirection::Front, "FRONT"),
        (Grid3dDirection::Back, "BACK"),
        (Grid3dDirection::Bottom, "BOTTOM"),
        (Grid3dDirection::Top, "TOP"),
    ] {
        let value = dsl::ToValue::to_value(&direction);
        assert_eq!(dsl::json::to_json_string(&direction), format!("\"{token}\""), "the wire token must match every schema leaf");
        assert_eq!(direction.label(), token);
        let decoded: Grid3dDirection = dsl::FromValue::from_value(value).expect("the token decodes back");
        assert_eq!(decoded, direction);
    }
}

#[test]
fn every_axis_carries_its_own_lowercase_wire_token() {
    for (axis, token) in [(Grid3dAxis::X, "x"), (Grid3dAxis::Y, "y"), (Grid3dAxis::Z, "z")] {
        assert_eq!(dsl::json::to_json_string(&axis), format!("\"{token}\""));
        assert_eq!(axis.label(), token);
    }
}
