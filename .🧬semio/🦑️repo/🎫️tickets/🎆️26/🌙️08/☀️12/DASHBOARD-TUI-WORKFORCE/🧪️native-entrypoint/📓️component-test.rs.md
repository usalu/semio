
/// 📦️ Local development components use the producer bound and preserve their exact page bytes.
#[test]
fn native_component_reader_admits_local_dev_artifacts_above_catalog_network_bound() {
    use std::io::Write;
    let _owner=RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error|error.into_inner());
    let contract:serde_json::Value=serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("component input contract");
    assert_eq!(semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES,contract["networkComponentMaxBytes"].as_u64().expect("network bound"));
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path=root.join(format!("local-component-pages-{}.json",std::process::id()));
    let content=serde_json::to_vec(&contract["document"]).expect("oracle input");
    let length=contract["localComponentTestBytes"].as_u64().expect("local input length") as usize;
    {
        let mut file=std::fs::File::create(&path).expect("test component");
        file.write_all(&content).expect("document bytes");
        let padding=[b' ';65536];
        let mut remaining=length-content.len();
        while remaining!=0 {let count=remaining.min(padding.len());file.write_all(&padding[..count]).expect("bounded padding");remaining-=count;}
    }
    let oracle:serde_json::Value=serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(&path).expect("independent file read"))).expect("independent JSON parser");
    let actual=crate::native_entrypoint::drive_native_entrypoint(kernel_runtime::read_native_component(&path)).expect("local development component pages");
    assert_eq!(actual.len(),length);
    assert_eq!(&actual[..content.len()],content.as_slice());
    assert!(actual[content.len()..].iter().all(|byte|*byte==b' '));
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual).expect("native JSON pages"),oracle);
    drop(actual);
    let maximum=contract["localComponentMaxBytes"].as_u64().expect("local component ceiling");
    std::fs::File::create(&path).expect("oversized local input").set_len(maximum+1).expect("bounded sparse input");
    let refusal=crate::native_entrypoint::drive_native_entrypoint(kernel_runtime::read_native_component(&path)).expect_err("maximum plus one local byte must be refused");
    assert!(refusal.contains(&format!("exceeds {maximum} bytes")));
    std::fs::remove_file(path).expect("remove test output");
}
