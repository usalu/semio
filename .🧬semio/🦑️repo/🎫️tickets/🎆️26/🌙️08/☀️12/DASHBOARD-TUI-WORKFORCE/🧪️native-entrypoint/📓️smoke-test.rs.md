
/// 🚨️ A selected plugin without an application cannot report a successful native smoke boot.
#[test]
fn native_smoke_without_a_selected_application_returns_failure() {
    let _owner=RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error|error.into_inner());
    let contract:serde_json::Value=serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("smoke contract");
    let fixture=&contract["emptySmoke"];
    let registry:serde_json::Value=serde_json::from_str(include_str!("../../../../🔌️plugin/📇️registry/🧬️schema/🔣️.json")).expect("current descriptor contract");
    assert_eq!(fixture["descriptor"]["executionProtocol"]["appChannelVersion"],registry["$defs"]["CatalogDescriptorV1"]["properties"]["executionProtocol"]["properties"]["appChannelVersion"]["const"],"the smoke input uses the current host channel");
    let descriptor:semio_framework::manifest::PackageDescriptor=serde_json::from_value(fixture["descriptor"].clone()).expect("independent descriptor oracle");
    assert!(descriptor.manifest.apps.is_empty());
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join(format!("empty-native-smoke-{}",std::process::id()));
    std::fs::create_dir_all(&root).expect("test runtime root");
    std::fs::write(root.join("descriptor.json"),serde_json::to_vec(&descriptor).expect("oracle descriptor bytes")).expect("descriptor input");
    std::fs::write(root.join("component.wasm"),[0,97,115,109,13,0,1,0]).expect("empty component input");
    std::fs::write(root.join("🔣️runtime.json"),serde_json::to_vec(&fixture["runtime"]).expect("runtime bytes")).expect("runtime input");
    let actual=crate::native_entrypoint::drive_native_entrypoint(run_smoke_with_axes("empty-smoke",root.clone(),Vec::new(),(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native)));
    assert_eq!(actual,fixture["exitCode"].as_i64().expect("smoke status") as i32);
    std::fs::remove_dir_all(root).expect("remove test runtime");
}
