use crate::{mesh_child_handle, LowpolyObject, LowpolyPaintLayer, LowpolySnapshot, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};

/// 🧪️ A small (deliberately NOT the real `LOWPOLY_PAINT_TEXTURE_SIZE`-scoped canvas) but
/// structurally representative fixture: two objects, one carrying a mesh handle and a paint
/// layer, the other bare -- exercising every field kind `LowpolySnapshot` actually has.
fn fixture() -> LowpolySnapshot {
    let obj1 = LowpolyObject {
        id: "obj-1".into(),
        name: "First Object".into(),
        transform: LowpolyTransform { position: [1.0, 2.0, 3.0], rotation: [0.0, 90.0, 0.0], scale: [1.0, 1.0, 1.0] },
        smooth_shading: false,
        mesh: Some(mesh_child_handle("obj-1", "{\"half-edge\":\"fixture\"}")),
        paint_layers: vec![LowpolyPaintLayer { name: "Base".into(), visible: true, opacity: 0.5, blend_mode: "normal".into(), pixels: vec![10, 20, 30, 40, 50, 60, 70, 80] }],
    };
    let obj2 = LowpolyObject { id: "obj-2".into(), name: "Second Object".into(), transform: LowpolyTransform::default(), smooth_shading: true, mesh: None, paint_layers: Vec::new() };
    LowpolySnapshot { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: vec![obj1, obj2] }
}

#[semio_framework_async_macros::async_test]
async fn txt_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).expect("txt export");
    let recovered = crate::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes).expect("txt import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn json_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).expect("json export");
    let recovered = crate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).expect("json import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn obj_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).expect("obj export");
    let recovered = crate::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes).expect("obj import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn png_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).expect("png export");
    let recovered = crate::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes).expect("png import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn ply_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).expect("ply export");
    let recovered = crate::io::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes).expect("ply import");
    assert_eq!(snapshot, recovered);
}

/// 🚫️ stl/gltf/dwg/las are honest stubs (real mesh geometry unavailable at this layer, see
/// those leaves' own doc comments) -- proves they fail LOUDLY, never silently miscompile as a
/// pack-envelope-mismatch lie.
#[semio_framework_async_macros::async_test]
async fn unimplemented_geometry_formats_error_honestly_instead_of_lying() {
    let snapshot = fixture();
    assert!(crate::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).is_err());
    assert!(crate::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).is_err());
    assert!(crate::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).is_err());
    assert!(crate::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).is_err());
}

/// 🚫️ Import-direction counterpart: stl/gltf/dwg/las deserializers are honest stubs too
/// (creating a resolvable mesh child artifact from parsed geometry needs a store/session
/// handle, not available to a synchronous `&[u8] -> LowpolySnapshot` function) -- arbitrary
/// input bytes must error, never silently fabricate a geometry-less `LowpolySnapshot`.
#[semio_framework_async_macros::async_test]
async fn unimplemented_geometry_import_formats_error_honestly_instead_of_lying() {
    let bytes = b"not a real payload, contents are irrelevant -- these stubs ignore input entirely";
    assert!(crate::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(bytes).is_err());
    assert!(crate::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(bytes).is_err());
    assert!(crate::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(bytes).is_err());
    assert!(crate::io::import::deserializers::artifacts::las::v1_0::any::deserialize_bytes(bytes).is_err());
}
