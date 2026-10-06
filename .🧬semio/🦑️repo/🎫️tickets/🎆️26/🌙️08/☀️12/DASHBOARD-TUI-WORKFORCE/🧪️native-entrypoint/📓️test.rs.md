
/// 🚪️ A process entry point completes and retires mounted page I/O without a window pump.
#[test]
fn headless_entrypoint_drives_retained_io_and_matches_the_json_oracle() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("headless contract");
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path = root.join(format!("headless-entrypoint-{}.json",std::process::id()));
    std::fs::write(&path,serde_json::to_vec(&contract["document"]).expect("oracle input")).expect("test document");
    let oracle:serde_json::Value=serde_json::from_slice(&std::fs::read(&path).expect("independent file read")).expect("independent JSON parser");
    let actual=crate::native_entrypoint::drive_native_entrypoint(async {
        let value=run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage {path:path.clone(),offset:0,max_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES}).await.expect("headless page read");
        let semio_framework_os_services::NativeIoValue::Page {mut bytes,eof}=value else {panic!("wrong I/O result")};
        assert!(eof);
        let parsed:serde_json::Value=serde_json::from_slice(bytes.page(0).expect("page bytes")).expect("renderer JSON");
        while !matches!(bytes.close_step(1,semio_framework_job::JOB_PAYLOAD_PAGE_BYTES),semio_framework_job::JobPayloadCloseStep::Complete) {}
        parsed
    });
    assert_eq!(actual,oracle);
    assert_eq!(RENDERER_IO_SLOTS.iter().filter(|slot|matches!(slot.state.load(Ordering::Acquire),RENDERER_IO_LIVE|RENDERER_IO_CHECKED_OUT)).count(),contract["terminalIoOwners"].as_u64().expect("terminal owners") as usize);
    std::fs::remove_file(path).expect("remove test output");
}
