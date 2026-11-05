//! 📤️ Public export dispatch publishes exact downloads without changing document or history.
use super::{context, effects::collect};
use semio_framework::kernel::Effect;
use semio_framework_plugin::{PluginApp, artifact_app_laws};
use semio_s_artifact_lowpoly_lowpoly_commands::media::import_mesh_bytes;
use semio_framework_3d::mesh::{FaceId, VertexId};
use serde_json::Value;

#[semio_framework_async_macros::async_test]
async fn public_mesh_exports_publish_exact_downloads_and_preserve_document_and_history() {
    let fixture: Value = serde_json::from_str(include_str!("export.json")).unwrap();
    let mut app = context::app_with_registry().await;
    let original_id = app.snapshot().unwrap().objects[0].id.clone();
    context::act(&mut app, "addPrimitive", serde_json::json!({"kind": fixture["setup"]["primitive"]})).await;
    let export_id = app.snapshot().unwrap().objects.last().unwrap().id.clone();
    let original_row = semio_s_artifact_lowpoly_lowpoly_view::document_object_row_id(&original_id);
    context::select(&mut app, &[("object", &original_row)]).await;
    context::act(&mut app, "deleteSelection", serde_json::json!({})).await;
    context::act(&mut app, "patchObject", serde_json::json!({"objectId": export_id, "field": "name", "value": fixture["setup"]["name"]})).await;
    let before = app.snapshot().unwrap();
    assert_eq!(before.objects.len(), 1);
    assert_eq!(before.objects[0].name, fixture["setup"]["name"].as_str().unwrap());
    let history = app.0.history_snapshot().await.unwrap();
    let edits = context::committed_edits(&mut app).await;
    let mut expected: Vec<[f64;3]> = serde_json::from_value(fixture["expected"]["positions"].clone()).unwrap();
    expected.sort_by(|a,b| a.partial_cmp(b).unwrap());
    for format in fixture["formats"].as_array().unwrap() {
        let arguments = semio_framework_value::DslValue::object([("format".to_string(), semio_framework_value::DslValue::String(format["format"].as_str().unwrap().to_string()))]);
        artifact_app_laws::dispatch_fixture_action(&mut app.0, "exportMesh", Some(arguments), &context::action_meta(), &mut artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
        let effects = collect(&mut app).await;
        assert_eq!(effects.len(), 1, "export publishes one host download");
        let Effect::DownloadMediaExport {filename, mime_type, data, encoding} = &effects[0] else { panic!("expected exact DownloadMediaExport"); };
        let metadata = serde_json::json!({"filename":filename,"mimeType":mime_type,"encoding":encoding});
        assert_eq!(metadata, serde_json::json!({"filename":format["filename"],"mimeType":format["mimeType"],"encoding":format["encoding"]}));
        assert!(!data.is_empty());
        let decoded = import_mesh_bytes(filename, data.as_bytes()).expect("independent owning-format decoder accepts download");
        assert_eq!(decoded.objects.len(), fixture["expected"]["objects"].as_u64().unwrap() as usize);
        let mesh = decoded.objects[0].mesh_state.as_ref().unwrap().clone().into_mesh().unwrap();
        assert_eq!(mesh.face_count(), format["faces"].as_u64().unwrap() as usize);
        let mut positions: Vec<_> = (0..mesh.vertex_count()).map(|index| {
            let point=mesh.vertex_position(VertexId(index as u32)).unwrap();
            [f64::from(point.x()),f64::from(point.y()),f64::from(point.z())]
        }).collect();
        positions.sort_by(|a,b| a.partial_cmp(b).unwrap()); positions.dedup();
        assert_eq!(positions, expected);
        let mut area=0.0;
        for index in 0..mesh.face_count() {
            let vertices=mesh.face_vertex_ids(FaceId(index as u32)).unwrap();
            let origin=mesh.vertex_position(vertices[0]).unwrap();
            for triangle in 1..vertices.len()-1 {
                let a=mesh.vertex_position(vertices[triangle]).unwrap();
                let b=mesh.vertex_position(vertices[triangle+1]).unwrap();
                let u=[f64::from(a.x()-origin.x()),f64::from(a.y()-origin.y()),f64::from(a.z()-origin.z())];
                let v=[f64::from(b.x()-origin.x()),f64::from(b.y()-origin.y()),f64::from(b.z()-origin.z())];
                let cross=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]];
                area+=0.5*cross.iter().map(|axis|axis*axis).sum::<f64>().sqrt();
            }
        }
        assert!((area-fixture["expected"]["area"].as_f64().unwrap()).abs()<1e-6);
        assert_eq!(app.snapshot().unwrap(), before);
        assert_eq!(context::committed_edits(&mut app).await, edits);
        assert_eq!(app.0.history_snapshot().await.unwrap(), history);
    }
}
