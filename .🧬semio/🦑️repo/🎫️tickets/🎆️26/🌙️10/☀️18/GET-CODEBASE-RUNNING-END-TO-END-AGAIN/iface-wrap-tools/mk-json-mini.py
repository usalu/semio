#!/usr/bin/env python3
"""Builds /private/tmp/iface-wrap/json-mini: the staged wire derives, the live manifest rows, the staged bind_inference_document and the appended tests compiled against the real pack-json/value crates."""
import json, os, re, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
M = "/private/tmp/iface-wrap/json-mini"
manifest = json.load(open(os.path.join(HERE, "staged", "manifest.json"), encoding="utf8"))
def staged(suffix):
    for row in manifest["files"]:
        if row["path"].endswith(suffix):
            return open(os.path.join(HERE, "staged", row["steps"][-1]["after"]), encoding="utf8").read()
    raise SystemExit("not staged: " + suffix)
mcp = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
wire = staged("🧬️schema/📡️artifact-inference-wire/🦀️.rs")
frames = staged("🏠️workspace/🧩️artifact-app-frames/🦀️.rs")
tests = open(os.path.join(HERE, "src/json/tests-append.rs"), encoding="utf8").read()
live_manifest = open(os.path.join(ROOT, "🧰️framework/🔨️modules/🛂️manifest/🧩️contribution/🦀️.rs"), encoding="utf8").read()
shutil.rmtree(M + "/wire", ignore_errors=True)
os.makedirs(M + "/wire/📡️artifact-inference-wire", exist_ok=True)
os.makedirs(M + "/wire/✅️approval-request", exist_ok=True)
shutil.copy(os.path.join(ROOT, mcp, "🧬️schema/✅️approval-request/🔣️.json"), M + "/wire/✅️approval-request/🔣️.json")
open(M + "/wire/📡️artifact-inference-wire/🦀️.rs", "w", encoding="utf8").write(wire)
def take(text, start, end_marker):
    i = text.index(start)
    j = text.index(end_marker, i)
    return text[i:j]
rows = take(live_manifest, "/// 💡️ One inference a plugin contributes onto an artifact kind", "/// 🗂️ Everything one plugin contributes onto one artifact kind")
const = "pub const INFERENCE_ARTIFACT_PACK_BASE64: &str = \"artifact-pack-base64\";\n"
i = frames.index("pub fn bind_inference_document(")
j = frames.index("\n}\n", i) + 3
bind = frames[i:j]
lib = """#![allow(warnings)]
pub mod semio_framework {
    use serde::{Deserialize, Serialize};
""" + rows + """}
pub mod semio_framework_os_mcp_schema {
    pub mod schema {
        use schemars::JsonSchema;
        use serde::{Deserialize, Serialize};
        #[path = "%s/wire/📡️artifact-inference-wire/🦀️.rs"]
        mod wire;
        pub use wire::*;
    }
}
#[derive(Debug)]
pub struct Fault { pub code: String, pub message: String }
#[derive(Default, Clone)]
pub struct ArtifactDocumentBinding { pub pack: Vec<u8>, pub spr: Vec<u8> }
pub struct InferCommand { pub canonical_payload: Vec<u8>, pub artifact_document: Option<ArtifactDocumentBinding> }
pub fn encode_base64(bytes: &[u8]) -> String {
    const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = (chunk[0] as u32) << 16 | (*chunk.get(1).unwrap_or(&0) as u32) << 8 | *chunk.get(2).unwrap_or(&0) as u32;
        for k in 0..4 { if k <= chunk.len() { out.push(A[(n >> (18 - 6 * k) & 63) as usize] as char) } else { out.push('=') } }
    }
    out
}
""" % M + bind + """
fn bound_inference_row(required: bool, encoding: &str) -> semio_framework::ContributedInferenceMetadata {
    semio_framework::ContributedInferenceMetadata { owner: "wfc".into(), artifact_kind: "s.wfc.bitmap".into(), artifact_schema: "s.wfc.bitmap".into(), artifact_schema_version: 1, inference_schema: "s.wfc.bitmap.solve".into(), inference_schema_version: 1, algorithm_version: 1, policy_version: 1, contributor: "wfc".into(), depends_on: Vec::new(),
        payload: Some(semio_framework::InferencePayloadContract { payload_schema_id: "p".into(), input_schema: "{}".into(), output_schema: "{}".into(), progress_unit: "cells".into(), artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".into(), encoding: encoding.into(), required }), commit: None }) }
}
fn bound_inference_command(document: Option<ArtifactDocumentBinding>, payload: &[u8]) -> InferCommand { InferCommand { canonical_payload: payload.to_vec(), artifact_document: document } }
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::*;
""" + tests + """
}
"""
open(M + "/src/lib.rs", "w", encoding="utf8").write(lib)
print("json-mini/src/lib.rs", len(lib))
