use super::*;
use crate::schema::default_snapshot;

#[test]
fn transient_schema_pack_and_typed_scratch_round_trip_exactly() {
    let transient = LowpolyTransient::default();
    let pack = transient.encode_pack();
    assert_eq!(LowpolyTransient::decode_pack(&pack).expect("transient pack"), transient);
    let mut scratch = LowpolyScratch::from_transient(&transient, LowpolySelection::default()).expect("typed transient");
    scratch.begin_stroke_drag();
    let next = scratch.transient_snapshot().expect("typed transient snapshot");
    let restored = LowpolyScratch::from_transient(&next, LowpolySelection::default()).expect("typed transient restore");
    assert!(restored.stroke_drag_active());
    assert_eq!(restored.mesh_workspace_map(), scratch.mesh_workspace_map());
}

#[test]
fn gesture_lifecycle_transitions_share_the_immutable_mesh_root() {
    let transient = LowpolyTransient::with_test_workspace_bytes(LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4);
    let paint = transient.begin_stroke_drag();
    let transform = transient.begin_transform_drag();
    let reset = transient.reset_gestures();
    assert!(Arc::ptr_eq(&transient.state.mesh_workspace, &paint.state.mesh_workspace));
    assert!(Arc::ptr_eq(&transient.state.mesh_workspace, &transform.state.mesh_workspace));
    assert!(Arc::ptr_eq(&transient.state.mesh_workspace, &reset.state.mesh_workspace));
    assert!(paint.state.stroke_drag_active && transform.state.transform_drag_active);
    assert!(!reset.state.stroke_drag_active && !reset.state.transform_drag_active);
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_none_without_an_active_transform_drag() {
    let scratch = LowpolyScratch::default();
    assert!(scratch.gesture_preview().is_none(), "no live gumball drag, nothing to preview");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_reflects_the_live_gumball_drag_and_clears_on_commit() {
    let mut scratch = LowpolyScratch::default();
    let projection = default_snapshot();
    let config = LowpolyConfig::default();
    scratch.set_transform_drag_active(true);

    let tick_a = scratch.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(0.5, 0.0, 0.0)), "translate");
    assert!(tick_a.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
    let (key, seq_after_a, payload_a) = scratch.gesture_preview().expect("a live gumball drag is previewable");
    assert_eq!(key, "gesture:transform");
    let value_a: serde_json::Value = serde_json::from_slice(&payload_a).expect("payload is valid json");
    assert_eq!(value_a["objectId"], serde_json::json!(projection.objects[0].id));
    assert_ne!(value_a["patch"], Into::<serde_json::Value>::into(dsl::ToValue::to_value(&LowpolyObjectPatch::default())), "the patch anchored to the drag-start snapshot must reflect the first tick");

    let tick_b = scratch.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(0.25, 0.0, 0.0)), "translate");
    assert!(tick_b.artifact_mutations.is_empty());
    let (_, seq_after_b, payload_b) = scratch.gesture_preview().expect("still live mid-drag");
    assert!(seq_after_b > seq_after_a, "seq is monotone per tick, for staleness detection on the receiving end");
    assert_ne!(payload_a, payload_b, "the base-anchored patch accumulates both ticks, not just the latest one");

    let end = scratch.commit_transform();
    assert_eq!(end.artifact_mutations.len(), 1, "the whole drag commits as exactly one real operation");
    assert!(scratch.gesture_preview().is_none(), "the drag ended: nothing left to preview, and the commit above already carried the real operation");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_a_pure_read_never_mutating_the_transform_session() {
    let mut scratch = LowpolyScratch::default();
    let projection = default_snapshot();
    let config = LowpolyConfig::default();
    scratch.set_transform_drag_active(true);
    scratch.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(1.0, 0.0, 0.0)), "translate");
    let object_id = scratch.transform.as_ref().unwrap().object_id.clone();
    let mesh_before = scratch.transform.as_ref().unwrap().doc.mesh_workspace().get(&object_id).cloned();
    let _ = scratch.gesture_preview();
    let _ = scratch.gesture_preview();
    assert_eq!(scratch.transform.as_ref().unwrap().doc.mesh_workspace().get(&object_id).cloned(), mesh_before, "gesture_preview must never mutate the live transform scratch it reads");
}
