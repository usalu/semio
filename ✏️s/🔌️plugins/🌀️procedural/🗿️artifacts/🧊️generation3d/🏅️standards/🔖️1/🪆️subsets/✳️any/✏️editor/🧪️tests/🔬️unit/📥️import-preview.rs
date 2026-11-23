//! 📥️ Mounted preview effects produced by the actual retained import route.

use super::*;
use semio_framework_artifact_flow_flow::SynapseSpec;
use semio_plugin::artifact_app_laws as laws;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshotRead;

fn neuron(id: &str, kind: &str) -> Widget {
    Widget::Neuron { id: id.into(), neuron_kind: kind.into(), params: Default::default(),
        input_ports: vec![], output_ports: vec![], preview: true }
}

fn document(servable: bool) -> Generation3dSnapshotRead {
    let mut snapshot = Generation3dSnapshotRead::new(<Generation3dPlayApp as ArtifactEditor>::reference_birth().unwrap()().unwrap());
    snapshot.host_snapshot.widgets = if servable {
        vec![Widget::InputNote { id: "note".into(), text: "Incoming".into() }]
    } else {
        vec![neuron("unserved", "fixture.unserved.import")]
    };
    snapshot.host_snapshot.synapses.clear();
    std::mem::take(&mut snapshot.host_snapshot.layout).retire_cold();
    snapshot
}

#[semio_framework_async_macros::async_test]
async fn mounted_document_import_rearms_from_the_incoming_graph_after_load() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️import-preview.json")).unwrap();
    let textures: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let actor = "import-preview";
    for row in fixture["cases"].as_array().unwrap() {
        let old_servable = row["oldServable"].as_bool().unwrap();
        let mut old = document(old_servable);
        let payload = if let Some(incoming_servable) = row["incomingServable"].as_bool() {
            let incoming = document(incoming_servable);
            assert_eq!(flow_eval_tick::may_rearm(&incoming.host_snapshot), incoming_servable);
            let file = semio_s_artifact_procedural_3d_io::document_io::export_document(&incoming).unwrap();
            import_document::ImportDocument { name: file.filename, payload: file.data, widget_id: None, channel: None, texture_id: None }
        } else {
            let texture = &textures["textureImport"];
            let id = texture["target"]["textureId"].as_str().unwrap();
            let data = texture["payload"].as_str().unwrap();
            let original = textures["meshAssets"]["mesh"].to_string();
            let text = if row["artifactMutations"].as_bool().unwrap() {
                original
            } else {
                set_widget_input::import_mesh_texture(&original, id, data).unwrap()
            };
            old.host_snapshot.widgets = vec![Widget::InputNote { id: "mesh-source".into(), text }, neuron("mesh", "brep.mesh.construct")];
            old.host_snapshot.synapses.push(SynapseSpec {
                id: "mesh-data".into(), from: "mesh-source".into(), from_port: "text".into(), to: "mesh".into(), to_port: "data".into(),
            });
            import_document::ImportDocument { name: "texture.png".into(), payload: data.into(), widget_id: Some("mesh-source".into()), channel: Some("text".into()), texture_id: Some(id.into()) }
        };
        let current_servable = flow_eval_tick::may_rearm(&old.host_snapshot);
        if !row["incomingServable"].is_null() { assert_eq!(current_servable, old_servable); }
        let Effect::LoadDocument { pack, spr } = set_active_example::reset_generation3d_document_effect(&old) else { unreachable!() };
        let mut app = context::app_with_actor(protocol::ActorId(actor.into())).await;
        let files = store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() };
        laws::load_document(&mut app.0, &files, &mut laws::fixture_identity(), app.1.cx()).await.unwrap();
        app.bind_actor(actor).await;
        let mut meta = laws::meta(actor);
        if row["mounted"].as_bool().unwrap() {
            let view = semio_plugin::ViewModel {
                window_id: Some("import-preview-window".into()),
                window_instances: vec![semio_framework::ViewWindowInstance { id: "import-preview-window".into(), window_kind_id: "procedural-preview".into() }],
                ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
            };
            meta.view_state = Some(laws::receive_view_fixture(view, laws::OriginalWorkFixture::stock()));
        }
        let policy = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
        laws::dispatch_original_fixture_command(&mut app.0, &Generation3dCommand::ImportDocument(payload), &meta, policy, &mut laws::fixture_identity(), app.1.cx()).await.unwrap();
        let receipt = laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.unwrap();
        assert_eq!(receipt.completions, 1);
        let effects: Vec<_> = receipt.effects.iter().map(|effect| match effect {
            Effect::LoadDocument { .. } => "loadDocument",
            Effect::DispatchAction { action, args, .. } => {
                assert_eq!(args.as_ref().and_then(|args| args.get("toolId")).and_then(semio_framework_value::DslValue::as_str), Some("previewEval"));
                action.as_str()
            }
            other => panic!("unexpected import effect {other:?}"),
        }).collect();
        let expected: Vec<&str> = if row["incomingServable"].is_null() && !current_servable {
            Vec::new()
        } else {
            row["effects"].as_array().unwrap().iter().map(|effect| effect.as_str().unwrap()).collect()
        };
        assert_eq!(effects, expected, "{}", row["id"]);
    }
}
