use super::*;

const NATIVE_PREFERENCE_RETAINED_DOCUMENT_LAW: &str = include_str!("../🧫️fixtures/🔣️.json");

#[cfg(not(target_arch = "wasm32"))]
fn finish_document_write(
    documents: &mut semio_framework_os_services::RetainedFixedFileDocuments,
    token: semio_framework_os_services::FixedFileDocumentWriteToken,
    expected_steps: usize,
) {
    let mut steps = 0;
    loop {
        let outcome = documents.write_step(token).expect("the exact retained owner advances one bounded step");
        steps += 1;
        assert!(outcome.written_bytes <= semio_framework_os_services::STORAGE_FIXED_FILE_PAGE_BYTES);
        if outcome.ready_to_publish {
            break;
        }
    }
    assert_eq!(steps, expected_steps);
    documents.publish(token).expect("the complete exact owner publishes atomically");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_config_document_preserves_exact_owner_cancellation_supersession_and_atomic_publication() {
    let law: Value = serde_json::from_str(NATIVE_PREFERENCE_RETAINED_DOCUMENT_LAW).expect("native retained-document law parses");
    let step_bytes = law["authority"]["serviceStepBytes"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("service step bytes");
    let document_bytes = law["authority"]["configDocumentMaxBytes"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("config document bytes");
    let maximum_steps = law["authority"]["documentMaxWriteSteps"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("document steps");
    let field_bytes = law["authority"]["storedFieldMaxBytes"].as_u64().and_then(|value| usize::try_from(value).ok()).expect("stored field bytes");
    assert_eq!(step_bytes, semio_framework_os_services::STORAGE_FIXED_FILE_PAGE_BYTES);
    assert_eq!(document_bytes, semio_framework_os_services::STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES);
    assert_eq!(maximum_steps, semio_framework_os_services::STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS);
    assert_eq!(document_bytes, step_bytes * maximum_steps);
    assert_eq!(OS_SHELL_CONFIG_MAX_BYTES, document_bytes);
    assert_eq!(SHELL_CHROME_IO_FIELD_BYTES, field_bytes);

    let root = std::env::temp_dir().join(format!("semio-native-preference-document-{}-{:?}", std::process::id(), std::thread::current().id()));
    let path = root.join("semio.os.config.field");
    let mut documents = semio_framework_os_services::RetainedFixedFileDocuments::default();
    let prior = br#"{"version":1,"preferences":{},"namedLayouts":{},"dockLayouts":{"apps":{}},"dockUi":{"apps":{}},"windowPanes":{"apps":{}}}"#.to_vec();
    let prior_token = documents.begin_write(&path, 7, 1, prior.clone(), document_bytes).expect("the prior document is admitted");
    finish_document_write(&mut documents, prior_token, 1);
    assert_eq!(documents.read(&path, document_bytes).expect("the prior document is readable"), prior);

    let exact = vec![0x5au8; document_bytes];
    let exact_token = documents.begin_write(&path, 7, 2, exact.clone(), document_bytes).expect("the exact document is admitted");
    for _ in 0..maximum_steps {
        let outcome = documents.write_step(exact_token).expect("the exact owner advances");
        assert_eq!(outcome.written_bytes, step_bytes);
        assert_eq!(documents.read(&path, document_bytes).expect("an inactive write cannot hide the committed document"), prior);
    }
    documents.publish(exact_token).expect("four complete steps publish");
    assert_eq!(documents.read(&path, document_bytes).expect("the exact document is readable"), exact);
    assert_eq!(std::fs::read(&path).expect("the system-file oracle reads the same single value"), exact);

    let oversized = vec![0xa5u8; document_bytes + 1];
    assert!(documents.begin_write(&path, 7, 3, oversized, document_bytes).is_err(), "plus one refuses before a temporary owner exists");
    assert_eq!(documents.read(&path, document_bytes).expect("refusal preserves the committed document"), exact);

    let cancelled_bytes = vec![0xc3u8; document_bytes];
    let cancelled = documents.begin_write(&path, 7, 4, cancelled_bytes, document_bytes).expect("the cancellable document is admitted");
    for _ in 0..2 {
        documents.write_step(cancelled).expect("the exact cancellable owner advances");
    }
    documents.cancel(cancelled).expect("the exact owner cancels");
    assert!(documents.publish(cancelled).is_err(), "a cancelled owner cannot publish");
    while !documents.cancel_step(cancelled).expect("bounded cancellation advances") {}
    assert!(documents.write_step(cancelled).is_err(), "a terminal token stays stale");
    assert_eq!(documents.read(&path, document_bytes).expect("cancellation preserves the committed document"), exact);

    let stale = documents.begin_write(&path, 7, 5, vec![0xd4u8; document_bytes], document_bytes).expect("the first writer is admitted");
    documents.write_step(stale).expect("the first writer advances once");
    let replacement_bytes = vec![0xe5u8; document_bytes];
    let replacement = documents.begin_write(&path, 7, 6, replacement_bytes.clone(), document_bytes).expect("a newer generation supersedes the prior writer");
    assert!(documents.write_step(stale).is_err(), "a superseded token cannot advance");
    assert!(documents.publish(stale).is_err(), "a superseded token cannot publish");
    finish_document_write(&mut documents, replacement, maximum_steps);
    assert_eq!(documents.read(&path, document_bytes).expect("only the replacement is visible"), replacement_bytes);

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn native_config_uses_the_document_lane_and_flat_fields_keep_the_fixed_page_lane() {
    let shell = wgpu_shell_source();
    let read = shell.split("fn native_pref_read_page").nth(1).expect("native read route");
    let read = &read[..read.find("fn native_pref_write_page").expect("native write route follows")];
    assert!(read.contains("key == OS_SHELL_CONFIG_STORAGE_KEY") && read.contains("storage_worker_read_fixed_file_document"));
    assert!(read.contains("storage_worker_read_fixed_file_page") && read.contains("SHELL_CHROME_IO_FIELD_BYTES"));

    let write = shell.split("fn native_pref_write_page").nth(1).expect("native write route");
    let write = &write[..write.find("fn raw_prefs_get").expect("raw preference route follows")];
    assert!(write.contains("key == OS_SHELL_CONFIG_STORAGE_KEY") && write.contains("RetainedFixedFileDocuments"));
    assert!(write.contains("storage_worker_write_fixed_file_page") && write.contains("SHELL_CHROME_IO_FIELD_BYTES"));
    assert!(shell.contains("const OS_SHELL_CONFIG_MAX_BYTES: usize = 64 * 1024;"));
    assert!(shell.contains("const SHELL_CHROME_IO_FIELD_BYTES: usize = 4 * 1024;"));
}
