use super::*;
use crate::{empty_cad_snapshot, sample_scene_fixture::sample_model_child};

#[semio_framework_async_macros::async_test]
async fn empty_scene_has_no_bounds() {
    let snapshot = empty_cad_snapshot();
    assert!(scene_bounds(&snapshot).is_none());
    assert_eq!(object_count(&snapshot), 0);
    assert_eq!(vertex_count(&snapshot), 0);
}

#[semio_framework_async_macros::async_test]
async fn object_count_reflects_occupied_model_slots() {
    let mut snapshot = empty_cad_snapshot();
    snapshot.shape_model = Some(sample_model_child("bounds-law-1"));
    assert_eq!(object_count(&snapshot), 1);
    snapshot.building_model = Some(sample_model_child("bounds-law-2"));
    assert_eq!(object_count(&snapshot), 2);
}
