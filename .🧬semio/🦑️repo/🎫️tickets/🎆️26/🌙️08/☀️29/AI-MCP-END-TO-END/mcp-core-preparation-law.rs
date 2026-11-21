use semio_plugin_interpreter::{CoreError, CoreModule, OwnedSemioArtifact};
use std::{io::Write, sync::Arc};

#[test]
fn real_core_preparation_settles_the_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(env!("SEMIO_PREPARATION_FIXTURE"))).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let bytes = case["component"].as_str().unwrap().as_bytes().chunks_exact(2)
            .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        let module = Arc::new(CoreModule::parse(&bytes).unwrap());
        let outcome = module.prepare_actor_image(case["fuel"].as_u64().unwrap(), 512 << 20, |_| case["continue"].as_bool().unwrap());
        assert_eq!(matches!(&outcome, Ok(Some(_))), case["ready"].as_bool().unwrap(), "{}", case["name"]);
        if let Some(expected) = case["hostImport"].as_str() {
            assert!(matches!(outcome, Err(CoreError::Host(message)) if message.ends_with(expected)));
        }
    }
}

#[test]
fn real_core_preparation_honors_initial_and_growing_storage() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(env!("SEMIO_PREPARATION_FIXTURE"))).unwrap();
    for case in fixture["storageCases"].as_array().unwrap() {
        let bytes = case["component"].as_str().unwrap().as_bytes().chunks_exact(2)
            .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        let module = Arc::new(CoreModule::parse(&bytes).unwrap());
        let limit = case["maximumStorageBytes"].as_u64().unwrap() as usize;
        assert!(matches!(module.prepare_actor_image(16, limit, |_| true), Ok(Some(_))), "{}", case["name"]);
        assert!(module.prepare_actor_image(16, 0, |_| true).is_err(), "{}", case["name"]);
    }
}

#[test]
#[ignore = "requires the exact prepared component"]
fn exact_component_reaches_import_free_preparation() {
    let source = std::env::var_os("SEMIO_PREPARATION_COMPONENT").expect("exact component path");
    let bytes = std::fs::read(&source).expect("read the exact component");
    let artifact = OwnedSemioArtifact::parse(&bytes).expect("parse the exact owned component");
    let began = std::time::Instant::now();
    let mut report = 0;
    let mut spent = 0;
    let prepared = artifact.prepare_actor(32_000_000_000, |fuel| {
        spent = fuel;
        if fuel >= report {
            eprintln!("[DEBUG] preparation fuel={fuel} elapsed_ms={}", began.elapsed().as_millis());
            report = fuel + 25_000_000;
        }
        began.elapsed() < std::time::Duration::from_secs(1_800)
    }).expect("the declared constructor completed without host imports");
    let prepared = prepared.expect("the exact component must opt into preparation");
    if let Some(path) = std::env::var_os("SEMIO_PREPARATION_IMAGE") {
        let digest = semio_framework_hash::Sha256::digest(&bytes);
        let mut output = std::fs::OpenOptions::new().write(true).create_new(true).open(&path).expect("create owned prepared image");
        prepared.write_image(digest, 512 << 20, |bytes| output.write_all(bytes)
            .map_err(|error| CoreError::State(error.to_string()))).expect("write sealed preparation");
        output.sync_all().expect("sync prepared image");
    }
    println!("[DEBUG] prepared {:?}, fuel={spent}, elapsed_ms={}", source, began.elapsed().as_millis());
}
