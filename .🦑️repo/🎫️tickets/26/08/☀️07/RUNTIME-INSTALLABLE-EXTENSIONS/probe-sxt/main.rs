//! Probe for Wave 1.A .sxt pack/unpack/verify.

#[path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs"]
mod os_semio;

#[path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs"]
mod os_extension;

fn main() {
    use os_extension::*;
    let manifest = ExtensionPackageManifest {
        extension_id: "flow.math".into(),
        label: "Flow Math".into(),
        version: "0.1.0".into(),
        extends: "flow".into(),
        capabilities: vec!["flow.operator".into()],
        contributions: serde_json::json!([{ "kind": "flowExtension", "id": "math.add" }]),
        package_format: EXTENSION_PACKAGE_FORMAT,
    };
    let component = b"\0asmfake".to_vec();
    let assets = vec![("readme.txt".into(), b"hello".to_vec())];
    let packed = pack(&manifest, &component, &assets).expect("pack");
    println!("[DEBUG] packed_len={}", packed.len());
    let verified = verify(&packed).expect("verify");
    assert_eq!(verified, manifest);
    let unpacked = unpack(&packed).expect("unpack");
    assert_eq!(unpacked.component_wasm, component);
    assert_eq!(unpacked.assets.get("readme.txt").map(Vec::as_slice), Some(b"hello".as_slice()));
    let hash = content_hash(&packed);
    println!("[DEBUG] content_hash={}", hash);
    let again = pack(
        &unpacked.manifest,
        &unpacked.component_wasm,
        &unpacked
            .assets
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>(),
    )
    .expect("repack");
    assert_eq!(packed, again);
    assert_eq!(hash, content_hash(&again));
    println!("[DEBUG] wave1a probe ok");
}
