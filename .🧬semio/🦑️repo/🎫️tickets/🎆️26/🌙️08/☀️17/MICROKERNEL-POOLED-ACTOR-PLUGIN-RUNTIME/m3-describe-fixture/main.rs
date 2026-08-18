//! 🧪️ M3-flow descriptor proof harness — NOT part of the repo, scratch-only, ticket folder.
//! Same pattern as M0-stdio's own harness (`../m0-describe-fixture/main.rs`): calls the exact
//! same `describe::describe_plugin()` the real wasm `describe()` export would invoke, using
//! `semio-s-plugin-flow`'s public `plugin()` fn as a plain library dependency (no `--tests`, so
//! its `#[cfg(test)]` code — which carries pre-existing, out-of-scope breakage, see
//! `terra-m3-all-targets.txt` — is never compiled). Reproduces the emitter's exact two-pass hash
//! convention (`🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`) using the REAL built wasm's
//! REAL SHA-256, passed in on the command line (this binary never touches the wasm file itself).
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as dsl;

use semio_framework::PackageDescriptor;
use sha2::{Digest, Sha256};

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let mut out = String::with_capacity(64);
    for byte in hasher.finalize() {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn main() {
    let mut args = std::env::args().skip(1);
    let out_dir = args.next().expect("usage: m3_describe <out_dir> <wasm_sha256_hex>");
    let wasm_sha256 = args.next().expect("usage: m3_describe <out_dir> <wasm_sha256_hex>");

    semio_framework_plugin::plugin_runtime::install_plugin_bundle_result(semio_s_plugin_flow::plugin());
    let descriptor_bytes = semio_framework_plugin::describe::describe_plugin();

    let decoded = store::pack_rt::decode_wire_value(&descriptor_bytes).expect("decoding describe() output as a pack");
    let mut descriptor: PackageDescriptor = dsl::from_dsl_value(decoded).expect("decoding describe() output as a PackageDescriptor");

    descriptor.hashes.wasm_sha256 = wasm_sha256.clone();
    descriptor.hashes.core_wasm_sha256 = wasm_sha256;
    descriptor.hashes.descriptor_sha256 = String::new();
    let prehash_value = dsl::to_dsl_value(&descriptor).expect("encoding descriptor for hashing");
    let prehash_bytes = store::pack_rt::encode_wire_value(&prehash_value);
    descriptor.hashes.descriptor_sha256 = sha256_hex(&prehash_bytes);

    let final_value = dsl::to_dsl_value(&descriptor).expect("encoding final descriptor");
    let final_bytes = store::pack_rt::encode_wire_value(&final_value);
    let final_json = serde_json::to_string_pretty(&descriptor).expect("encoding descriptor as JSON");

    let out_dir = std::path::PathBuf::from(out_dir);
    std::fs::create_dir_all(&out_dir).expect("creating out dir");
    std::fs::write(out_dir.join("🛂️descriptor.semio"), &final_bytes).expect("writing 🛂️descriptor.semio");
    std::fs::write(out_dir.join("🔣️descriptor.json"), format!("{final_json}\n")).expect("writing 🔣️descriptor.json");

    println!(
        "described flow (role={:?}) -> {}/🛂️descriptor.semio + 🔣️descriptor.json (wasm_sha256={}, activationEvents={}, capabilityRequests={}, execution={:?})",
        descriptor.role,
        out_dir.display(),
        descriptor.hashes.wasm_sha256,
        descriptor.activation_events.len(),
        descriptor.capability_requests.len(),
        descriptor.execution
    );
}
