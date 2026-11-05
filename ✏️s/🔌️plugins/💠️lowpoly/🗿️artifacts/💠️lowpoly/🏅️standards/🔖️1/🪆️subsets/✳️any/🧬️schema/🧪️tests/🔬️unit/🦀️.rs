extern crate semio_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
use semio_s_artifact_lowpoly_lowpoly_model::{standards::v1::subsets::any::io::text::snapshot::default_snapshot, standards::v1::subsets::any::io::text::snapshot::default_mesh_workspace};
use semio_s_artifact_lowpoly_lowpoly_model::*;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_PAINT_TEXTURE_SIZE;
use framework_schema::ArtifactSchema;
use semio_plugin::MeshData;
use semio_framework_3d::mesh::HalfedgeMesh;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::empty_paint_pixels;

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_concrete_forest_left_object() {
    let projection = default_snapshot();
    assert_eq!(projection.schema, semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_DOCUMENT_SCHEMA);
    assert_eq!(projection.objects.len(), 1);
    assert_eq!(projection.objects[0].id, "obj-1");
    assert_eq!(projection.objects[0].name, LOWPOLY_DEFAULT_EXAMPLE_LABEL);
    assert_eq!(projection.objects[0].paint_layers.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn default_concrete_forest_mesh_parses_and_has_faces() {
    let projection = default_snapshot();
    let workspace = default_mesh_workspace();
    let mesh_json = workspace.get(&projection.objects[0].id).expect("workspace entry for default object");
    let mesh = mesh_json.clone().into_mesh().expect("default mesh");
    assert!(mesh.face_count() > 6, "concrete forest should expose more than a unit box");
    assert!(mesh.vertex_count() > 8, "concrete forest should expose more than eight vertices");
}

#[semio_framework_async_macros::async_test]
async fn projection_round_trips_paint_pixels_through_base64_json() {
    let mut projection = default_snapshot();
    projection.objects[0].paint_layers[0].pixels = empty_paint_pixels();
    projection.objects[0].paint_layers[0].pixels[0] = 7;
    projection.objects[0].paint_layers[0].pixels[1] = 9;
    let json = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::lowpoly_json_encode(&projection);
    let restored: semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(serde_json::from_str::<serde_json::Value>(&json).unwrap())).unwrap();
    assert_eq!(restored, projection);
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_apply_and_inverse_round_trip() {
    use semio_s_artifact_lowpoly_lowpoly_mut_objects::rename_object;
    use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
    use protocol::{ColdMutation, Mutation};
    let base = default_snapshot();
    let object_id = base.objects[0].id.clone();
    let mutation = LowpolyMutation::RenameObject(rename_object::RenameObject { id: object_id, new_name: "Renamed".into() });
    let after = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("valid mutation diff");
    assert_eq!(after.objects[0].name, "Renamed");
    let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    let mut state = after;
    for step in &inverse {
        state = protocol::apply_diff_cold(step.diff(&state).diff(), &state).expect("valid mutation diff");
    }
    assert_eq!(state.objects[0].name, LOWPOLY_DEFAULT_EXAMPLE_LABEL);
}

#[semio_framework_async_macros::async_test]
async fn pixel_runs_from_diff_captures_only_changed_pixels() {
    let mut before = vec![0u8; 16];
    let mut after = before.clone();
    after[4] = 9;
    after[5] = 9;
    after[10] = 3;
    let runs = pixel_runs_from_diff(&before, &after);
    assert_eq!(runs, vec![(4, vec![9, 9, 0, 0, 0, 0, 3, 0])], "adjacent changed pixels merge into one run");
    before[4] = 9;
    before[5] = 9;
    let runs = pixel_runs_from_diff(&before, &after);
    assert_eq!(runs, vec![(8, vec![0, 0, 3, 0])], "an unchanged pixel splits the runs");
    assert!(pixel_runs_from_diff(&after, &after).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn composite_layer_pixels_skips_invisible_layers() {
    let mut layer = LowpolyPaintLayer::new("Hidden");
    layer.visible = false;
    layer.pixels = [255, 0, 0, 255].repeat(LOWPOLY_PAINT_TEXTURE_SIZE.pow(2));
    let out = composite_layer_pixels(&[layer]).expect("canonical layer composite");
    assert_eq!(out.len(), LOWPOLY_PAINT_TEXTURE_SIZE.pow(2) * 4);
    assert!(out.chunks_exact(4).all(|pixel| pixel == [0, 0, 0, 0]));
}

#[semio_framework_async_macros::async_test]
async fn composite_layer_pixels_blends_partial_opacity_over_transparent_base() {
    let mut layer = LowpolyPaintLayer::new("Half");
    layer.opacity = 0.5;
    layer.pixels = [200, 100, 50, 255].repeat(LOWPOLY_PAINT_TEXTURE_SIZE.pow(2));
    let out = composite_layer_pixels(&[layer]).expect("canonical layer composite");
    assert_eq!(out.len(), LOWPOLY_PAINT_TEXTURE_SIZE.pow(2) * 4);
    assert!(out.chunks_exact(4).all(|pixel| pixel == [200, 100, 50, 128]));
}

#[semio_framework_async_macros::async_test]
async fn composite_layer_pixels_blends_stacked_opaque_and_translucent_layers() {
    let base = LowpolyPaintLayer { name: "Base".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), pixels: [255, 0, 0, 255].repeat(LOWPOLY_PAINT_TEXTURE_SIZE.pow(2)) };
    let top = LowpolyPaintLayer { name: "Top".into(), visible: true, opacity: 0.5, blend_mode: "normal".into(), pixels: [0, 0, 255, 255].repeat(LOWPOLY_PAINT_TEXTURE_SIZE.pow(2)) };
    let out = composite_layer_pixels(&[base, top]).expect("canonical stacked composite");
    assert_eq!(out.len(), LOWPOLY_PAINT_TEXTURE_SIZE.pow(2) * 4);
    assert!(out.chunks_exact(4).all(|pixel| pixel == [128, 0, 128, 255]));
}

#[semio_framework_async_macros::async_test]
async fn stamp_brush_eraser_reduces_alpha_at_center() {
    let mut pixels = empty_paint_pixels();
    stamp_brush(&mut pixels, 0.5, 0.5, 4.0, [0, 0, 0, 0], 1.0, 1.0, true).expect("canonical brush canvas");
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
    flood_fill(&mut pixels, 0.99, 0.01, [255, 0, 0, 255]).expect("canonical fill canvas");
    assert_eq!(&pixels[0..4], &[0, 255, 0, 255]);
    let far_x = size - 10;
    let far_y = size - 10;
    let far_offset = (far_y * size + far_x) * 4;
    assert_eq!(&pixels[far_offset..far_offset + 4], &[255, 0, 0, 255]);
}

#[test]
fn canonical_texture_contract_accepts_only_empty_or_complete_layers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🎨️texture/🧪️tests/🔣️.json")).unwrap();
    assert_eq!(semio_s_artifact_lowpoly_lowpoly_model::paint_texture::PaintTextureError::CODE, fixture["refusal"].as_str().unwrap());
    for case in fixture["cases"].as_array().unwrap() {
        let bytes = case["bytes"].as_u64().unwrap() as usize;
        let mut layer = LowpolyPaintLayer::new("Contract / Vertrag");
        layer.pixels = vec![17; bytes];
        let before = layer.clone();
        let wire = semio_framework_value::ToValue::to_value(&layer);
        let cold = <LowpolyPaintLayer as semio_framework_value::FromValue>::from_value(wire.clone());
        let controlled = semio_framework_test_original_snapshot_owners::with_decoding(|owner| {
            <LowpolyPaintLayer as semio_framework_value::FromValue>::from_value_controlled(&wire, owner.native())
        });
        for decoded in [cold, controlled] {
            if case["valid"] == true {
                assert_eq!(decoded.expect("canonical cold and controlled layer decode"), layer);
            } else {
                let error = decoded.expect_err("invalid cold and controlled layer decode");
                assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::InvalidValue);
                assert!(error.to_string().contains(semio_s_artifact_lowpoly_lowpoly_model::paint_texture::PaintTextureError::CODE));
            }
        }
        <semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(wire);
        if case["valid"] == true {
            layer.validate_pixels().expect("canonical layer");
            let materialized = layer.materialized_pixels().expect("canonical materialization");
            assert_eq!(materialized.len(), semio_s_artifact_lowpoly_lowpoly_model::paint_texture::LOWPOLY_PAINT_TEXTURE_BYTES);
            assert!(materialized.iter().all(|byte| *byte == if bytes == 0 { 255 } else { 17 }));
        } else {
            assert_eq!(layer.validate_pixels().unwrap_err().actual_bytes, bytes, "{case}");
            assert_eq!(layer.materialized_pixels().unwrap_err().actual_bytes, bytes, "{case}");
            assert_eq!(composite_layer_pixels(&[layer.clone()]).unwrap_err().actual_bytes, bytes, "{case}");
            layer.visible = false;
            assert_eq!(composite_layer_pixels(&[layer.clone()]).unwrap_err().actual_bytes, bytes, "hidden malformed layers still refuse");
            layer.visible = before.visible;
        }
        assert_eq!(layer, before, "validation and materialization preserve the authored source");
    }
}

#[test]
fn malformed_raw_paint_canvases_refuse_without_crop_pad_or_partial_writes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🎨️texture/🧪️tests/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case| case["valid"] == false) {
        let bytes = case["bytes"].as_u64().unwrap() as usize;
        let mut pixels = vec![17; bytes];
        let before = pixels.clone();
        assert_eq!(stamp_brush(&mut pixels, 0.5, 0.5, 4.0, [1, 2, 3, 255], 1.0, 1.0, false).unwrap_err().actual_bytes, bytes);
        assert_eq!(pixels, before);
        assert_eq!(flood_fill(&mut pixels, 0.5, 0.5, [1, 2, 3, 255]).unwrap_err().actual_bytes, bytes);
        assert_eq!(pixels, before);
        assert_eq!(sample_pixel_from(&pixels, 0.99, 0.01).unwrap_err().actual_bytes, bytes);
        assert_eq!(pixels, before);
    }
}
