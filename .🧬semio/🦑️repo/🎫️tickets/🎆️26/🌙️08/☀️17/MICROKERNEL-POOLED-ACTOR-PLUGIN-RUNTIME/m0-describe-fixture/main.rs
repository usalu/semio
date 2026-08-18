//! 🧪️ M0-stdio descriptor proof harness — NOT part of the repo, scratch-only, ticket folder.
//! Calls the exact same `describe::describe_plugin()` the real wasm `describe()` export invokes
//! (mirrors E2-builder-descriptor's own substitute for the note proof migration, `📓️terra-E2-
//! builder-descriptor-report.md` §6), because `semio-s-plugin-stdio`'s own `#[cfg(test)]` code
//! carries pre-existing, out-of-scope breakage (267 errors across ~19 unrelated artifact-format
//! test files) that blocks `cargo test -p semio-s-plugin-stdio --lib` from compiling at all — so a
//! `#[test]`-based harness INSIDE that crate is not an option. This binary instead depends on
//! `semio-s-plugin-stdio` as a plain library (no `--tests`, so its broken `#[cfg(test)]` modules are
//! never compiled) and reproduces the emitter's exact two-pass hash convention
//! (`🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`) using the REAL built wasm's REAL SHA-256,
//! passed in on the command line rather than computed here (this binary never touches the wasm file
//! itself, matching the emitter's own "the guest cannot know its own already-built wasm bytes" note).
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
    let out_dir = args.next().expect("usage: m0_describe <out_dir> <wasm_sha256_hex>");
    let wasm_sha256 = args.next().expect("usage: m0_describe <out_dir> <wasm_sha256_hex>");

    semio_framework_plugin::plugin_runtime::install_plugin_bundle_result(semio_s_plugin_stdio::plugin());
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
        "described stdio (role={:?}) -> {}/🛂️descriptor.semio + 🔣️descriptor.json (wasm_sha256={}, activationEvents={}, capabilityRequests={}, execution={:?})",
        descriptor.role,
        out_dir.display(),
        descriptor.hashes.wasm_sha256,
        descriptor.activation_events.len(),
        descriptor.capability_requests.len(),
        descriptor.execution
    );
}
