use super::*;
use crate::empty_paint_pixels;

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_unit_box_object() {
    let projection = default_snapshot();
    assert_eq!(projection.schema, crate::LOWPOLY_DOCUMENT_SCHEMA);
    assert_eq!(projection.objects.len(), 1);
    assert_eq!(projection.objects[0].id, "obj-1");
    assert_eq!(projection.objects[0].name, "Unit Box");
    assert_eq!(projection.objects[0].paint_layers.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn default_unit_box_mesh_parses_and_has_faces() {
    let projection = default_snapshot();
    let workspace = default_mesh_workspace();
    let mesh_json = workspace.get(&projection.objects[0].id).expect("workspace entry for default object");
    let mesh = HalfedgeMesh::from_json(mesh_json).expect("default mesh");
    assert!(mesh.face_count() >= 6, "unit box should expose six faces");
    assert!(mesh.vertex_count() >= 8, "unit box should expose eight vertices");
}

#[semio_framework_async_macros::async_test]
async fn projection_round_trips_paint_pixels_through_base64_json() {
    let mut projection = default_snapshot();
    projection.objects[0].paint_layers[0].pixels[0] = 7;
    projection.objects[0].paint_layers[0].pixels[1] = 9;
    let json = serde_json::to_string(&Into::<serde_json::Value>::into(dsl::ToValue::to_value(&projection))).unwrap();
    let restored: crate::LowpolySnapshot = dsl::FromValue::from_value(dsl::DslValue::from(serde_json::from_str::<serde_json::Value>(&json).unwrap())).unwrap();
    assert_eq!(restored, projection);
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_apply_and_inverse_round_trip() {
    // 🩹 Was `protocol::ArtifactEngine`-based (that trait doesn't exist anywhere in the
    // codebase — a stale reference predating 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL — and
    // `LowpolyEngine` never exposed a `snapshot()`/apply-mutation API either) and constructed
    // the since-removed `LowpolyMutation::ObjectsPatch` bag variant. Rewritten against the real
    // `protocol::Mutation` diff/apply/inverse contract and the new `rename-object` mutation.
    use crate::mutations::rename_object;
    use crate::LowpolyMutation;
    use protocol::{Mutation, MutationDiff};
    let base = default_snapshot();
    let object_id = base.objects[0].id.clone();
    let mutation = LowpolyMutation::RenameObject(rename_object::RenameObject { id: object_id, new_name: "Renamed".into() });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(after.objects[0].name, "Renamed");
    let inverse = mutation.inverse(&base);
    let mut state = after;
    for step in &inverse {
        state = step.diff(&base).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(state.objects[0].name, "Unit Box");
}

#[semio_framework_async_macros::async_test]
async fn pixel_runs_from_diff_captures_only_changed_bytes() {
    let mut before = vec![0u8; 16];
    let mut after = before.clone();
    after[4] = 9;
    after[5] = 9;
    after[10] = 3;
    let runs = pixel_runs_from_diff(&before, &after);
    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].0, 4);
    assert_eq!(runs[0].1, vec![9, 9]);
    assert_eq!(runs[1].0, 10);
    assert_eq!(runs[1].1, vec![3]);
    before[4] = 9;
    before[5] = 9;
    before[10] = 3;
    assert!(pixel_runs_from_diff(&before, &after).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn composite_layer_pixels_skips_invisible_layers() {
    let mut layer = LowpolyPaintLayer::new("Hidden");
    layer.visible = false;
    layer.pixels = vec![255, 0, 0, 255];
    let out = composite_layer_pixels(&[layer]);
    assert_eq!(&out[0..4], &[0, 0, 0, 0]);
}

#[semio_framework_async_macros::async_test]
async fn composite_layer_pixels_blends_partial_opacity_over_transparent_base() {
    let mut layer = LowpolyPaintLayer::new("Half");
    layer.opacity = 0.5;
    layer.pixels = vec![200, 100, 50, 255];
    let out = composite_layer_pixels(&[layer]);
    assert_eq!(&out[0..4], &[200, 100, 50, 128]);
}

#[semio_framework_async_macros::async_test]
async fn composite_layer_pixels_blends_stacked_opaque_and_translucent_layers() {
    let base = LowpolyPaintLayer { name: "Base".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), pixels: vec![255, 0, 0, 255] };
    let top = LowpolyPaintLayer { name: "Top".into(), visible: true, opacity: 0.5, blend_mode: "normal".into(), pixels: vec![0, 0, 255, 255] };
    let out = composite_layer_pixels(&[base, top]);
    assert_eq!(&out[0..4], &[128, 0, 128, 255]);
}

#[semio_framework_async_macros::async_test]
async fn stamp_brush_eraser_reduces_alpha_at_center() {
    let mut pixels = empty_paint_pixels();
    stamp_brush(&mut pixels, 0.5, 0.5, 4.0, [0, 0, 0, 0], 1.0, 1.0, true);
    let size = LOWPOLY_PAINT_TEXTURE_SIZE;
    let center = (size / 2 * size + size / 2) * 4;
    assert!(pixels[center + 3] < 255);
}

#[semio_framework_async_macros::async_test]
async fn flood_fill_only_affects_contiguous_matching_region() {
    let mut pixels = empty_paint_pixels();
    let size = LOWPOLY_PAINT_TEXTURE_SIZE;
    for y in 0..10 {
        for x in 0..10 {
            let offset = (y * size + x) * 4;
            pixels[offset..offset + 4].copy_from_slice(&[0, 255, 0, 255]);
        }
    }
    flood_fill(&mut pixels, 0.99, 0.01, [255, 0, 0, 255]);
    assert_eq!(&pixels[0..4], &[0, 255, 0, 255]);
    let far_offset = (500 * size + 500) * 4;
    assert_eq!(&pixels[far_offset..far_offset + 4], &[255, 0, 0, 255]);
}
