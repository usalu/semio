use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, PluginRuntime};

#[test]
fn vcs_guest_descriptor_has_one_canonical_native_openable_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🪪️native-openable-identity/🧬️v1/🔣️.json")).expect("neutral identity fixture");
    let authority = &fixture["authority"];
    let bundle = semio_s_plugin_vcs::plugin().expect("schema-owned VCS plugin");
    assert_eq!(bundle.manifest.plugin_id, authority["pluginId"].as_str().unwrap());
    assert_eq!(semio_s_plugin_vcs::artifacts::vcs::artifact_kind().id, authority["artifactKind"].as_str().unwrap());
    assert_eq!(semio_s_plugin_vcs::artifacts::vcs::VCS_DOCUMENT_SCHEMA, authority["artifactSchema"].as_str().unwrap());
    assert_eq!(semio_s_plugin_vcs::artifacts::vcs::VCS_DIALECT.artifact_kind, authority["dialectKind"].as_str().unwrap());
    let codec = semio_s_plugin_vcs::artifacts::vcs::standards::v1::subsets::any::io::io().native.codec;
    assert_eq!(codec.schema, authority["artifactSchema"].as_str().unwrap());
    assert_eq!(codec.extension, "vcs");
    let record = <semio_s_plugin_vcs::artifacts::vcs::VcsSnapshot as semio_framework_os_kernel::ArtifactPack>::record_spec().expect("VCS schema-owned record");
    assert_eq!(codec.pack_schema_hash, semio_framework_os_kernel::os_pack::schema_hash(&record));
    assert_ne!(codec.pack_schema_hash, [0; 32]);
    let runtime = PluginRuntime::<semio_s_plugin_vcs::VcsApps>::new();
    install_plugin_bundle_result(&runtime, Ok(bundle));
    let bytes = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(&runtime));
    let wire = semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).expect("guest descriptor bytes");
    let descriptor: semio_framework::PackageDescriptor = semio_framework::from_dsl_value(wire).expect("strict guest descriptor");
    assert_eq!(descriptor.package_id, authority["packageId"].as_str().unwrap());
    assert_eq!(descriptor.manifest.plugin_id, authority["pluginId"].as_str().unwrap());
    assert_eq!(descriptor.execution, semio_framework::ExecutionMode::Isolated);
    for (key, role) in [("viewerApp", semio_framework::AppRole::Viewer), ("editorApp", semio_framework::AppRole::Editor)] {
        let apps = descriptor.manifest.apps.iter().filter(|app| app.id == authority[key].as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].role, role);
        assert_eq!(apps[0].dialect.artifact_kind, authority["dialectKind"].as_str().unwrap());
        assert_eq!(apps[0].id, semio_framework::surface_app_id(&apps[0].dialect, role));
        if key == "viewerApp" {
            assert_eq!(apps[0].window_kinds.iter().filter(|window| window.id == authority["viewerWindowKindId"].as_str().unwrap()).count(), 1);
        }
    }
    assert!(descriptor.activation_events.iter().any(|event| matches!(event, semio_framework_plugin::kernel::ActivationEvent::OnArtifactKind { kind } if kind == authority["artifactKind"].as_str().unwrap())));
    assert!(!format!("{descriptor:?}").contains("vcs.document"));
}
