
/// 🧑‍🏭️ Native application boot completes retained I/O before any presentation host exists.
#[test]
fn native_app_worker_completes_io_without_a_presentation_host() {
    let contract:serde_json::Value=serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("worker I/O contract");
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path=root.join(format!("worker-io-{}.json",std::process::id()));
    std::fs::write(&path,serde_json::to_vec(&contract["document"]).expect("oracle input")).expect("test document");
    let oracle:serde_json::Value=serde_json::from_slice(&std::fs::read(&path).expect("independent file read")).expect("independent JSON parser");
    let (sent,received)=std::sync::mpsc::channel();
    let request_path=path.clone();
    spawn_app_task(async move {
        let value=run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage {path:request_path,offset:0,max_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES}).await.expect("worker page read");
        let semio_framework_os_services::NativeIoValue::Page {mut bytes,eof}=value else {panic!("wrong I/O result")};
        assert!(eof);
        let actual:serde_json::Value=serde_json::from_slice(bytes.page(0).expect("page bytes")).expect("renderer JSON");
        while !matches!(bytes.close_step(1,semio_framework_job::JOB_PAYLOAD_PAGE_BYTES),semio_framework_job::JobPayloadCloseStep::Complete) {}
        sent.send(actual).expect("worker result");
    });
    let actual=received.recv_timeout(std::time::Duration::from_secs(5)).expect("application worker must advance I/O without a presentation callback");
    assert_eq!(actual,oracle);
    std::fs::remove_file(path).expect("remove test output");
}
