
use super::*;
use crate::mutations::{create_mesh, create_object, delete_mesh, delete_object, edit_paint_layer, insert_paint_layer, rename_object};
use crate::schema::default_snapshot;
use protocol::{OpBinary, OpText};

fn tiny_mesh_json() -> String {
    semio_framework_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("mesh json")
}

fn tiny_object(id: &str, name: &str) -> crate::LowpolyObject {
    let mesh_workspace = tiny_mesh_json();
    let mesh = crate::mesh_child_handle(id, &mesh_workspace);
    crate::LowpolyObject { id: id.into(), name: name.into(), transform: Default::default(), smooth_shading: false, mesh: Some(mesh), paint_layers: vec![crate::LowpolyPaintLayer::new("Base")] }
}

/// 🧪️ One representative value per variant — reused by the round-trip law test below.
fn demo_mutation_cases() -> Vec<LowpolyMutation> {
    let projection = default_snapshot();
    let object_id = projection.objects[0].id.clone();
    vec![
        LowpolyMutation::CreateObject(create_object::CreateObject { index: 1, object: tiny_object("obj-100", "Box") }),
        LowpolyMutation::DeleteObject(delete_object::DeleteObject { id: object_id.clone() }),
        LowpolyMutation::ReorderObjects(crate::mutations::reorder_objects::ReorderObjects { id: object_id.clone(), to_index: 0 }),
        LowpolyMutation::RenameObject(rename_object::RenameObject { id: object_id.clone(), new_name: "Renamed".into() }),
        LowpolyMutation::ChangeObjectSmoothShading(crate::mutations::change_object_smooth_shading::ChangeObjectSmoothShading { id: object_id.clone(), new_smooth_shading: true }),
        LowpolyMutation::MoveObject(crate::mutations::move_object::MoveObject { id: object_id.clone(), new_position: [1.0, 2.0, 3.0] }),
        LowpolyMutation::RotateObject(crate::mutations::rotate_object::RotateObject { id: object_id.clone(), new_rotation: [0.1, 0.2, 0.3] }),
        LowpolyMutation::ScaleObject(crate::mutations::scale_object::ScaleObject { id: object_id.clone(), new_scale: [2.0, 2.0, 2.0] }),
        LowpolyMutation::CreateMesh(create_mesh::CreateMesh {
            id: object_id.clone(),
            child_id: "mesh-fixture-01".into(),
            target: store::os_io::ArtifactRef { artifact_id: format!("{object_id}-mesh"), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() } },
            mesh_workspace: tiny_mesh_json(),
        }),
        LowpolyMutation::DeleteMesh(delete_mesh::DeleteMesh { id: object_id.clone() }),
        LowpolyMutation::InsertPaintLayer(insert_paint_layer::InsertPaintLayer { object_id: object_id.clone(), index: 1, layer: crate::LowpolyPaintLayer::new("Detail") }),
        LowpolyMutation::RemovePaintLayer(crate::mutations::remove_paint_layer::RemovePaintLayer { object_id: object_id.clone(), index: 0 }),
        LowpolyMutation::RenamePaintLayer(crate::mutations::rename_paint_layer::RenamePaintLayer { object_id: object_id.clone(), index: 0, new_name: "Top".into() }),
        LowpolyMutation::ChangePaintLayerVisible(crate::mutations::change_paint_layer_visible::ChangePaintLayerVisible { object_id: object_id.clone(), index: 0, new_visible: false }),
        LowpolyMutation::ChangePaintLayerOpacity(crate::mutations::change_paint_layer_opacity::ChangePaintLayerOpacity { object_id: object_id.clone(), index: 0, new_opacity: 0.5 }),
        LowpolyMutation::ChangePaintLayerBlendMode(crate::mutations::change_paint_layer_blend_mode::ChangePaintLayerBlendMode { object_id: object_id.clone(), index: 0, new_blend_mode: "multiply".into() }),
        LowpolyMutation::EditPaintLayer(edit_paint_layer::EditPaintLayer { object_id, layer_index: 0, runs: vec![PixelRun { offset: 12, bytes: vec![255, 0, 0, 255] }, PixelRun { offset: 400, bytes: vec![0, 255, 0, 255, 0, 0, 0, 128] }] }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = <LowpolyMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
        let decoded = <LowpolyMutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
    }
}

#[semio_framework_async_macros::async_test]
async fn op_text_parse_rejects_garbage() {
    let result = <LowpolyMutation as OpText>::parse_op("not json at all");
    assert!(result.is_err());
}
