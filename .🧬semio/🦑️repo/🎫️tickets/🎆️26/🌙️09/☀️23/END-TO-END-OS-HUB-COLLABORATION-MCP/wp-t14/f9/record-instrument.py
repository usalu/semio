#!/usr/bin/env python3
"""🧭️ OVERLAY-ONLY instrumentation for F9 carrier regeneration (never lands in the tree): after `content-id.py` applied
`store::content_id` in an overlay, this makes `content_id` append `prefix<TAB>new id<TAB>old candidates` to the file named
by `T14_F9_RECORD` (native only). The old candidates are every way the 29 former sites fed the same bytes into
`DefaultHasher` (`str`, `[u8]`, U+001F-joined `str` fields, a `str` head + `[u8]` tail, and the bytes re-serialized by `serde_json` — process3d's
`steps-flow` hashed `serde_json::to_string(&to_value(content))`, whose map keys serde_json sorts), so one run of an owner's tests
yields the exact old→new id map for every id the production code re-mints. process3d's `steps-flow` hashed
`serde_json::to_string(&to_value(content))` of the DslValue itself (float text differs from the first-party JSON the new id
hashes), so its minting site records the exact old id from the content. Refuses to run on the live tree.
usage: record-instrument.py --root <overlay>"""
import sys
from pathlib import Path

OLD = """pub fn content_id(prefix: &str, bytes: &[u8]) -> String {
    format!("{prefix}-{}", semio_framework_hash::hex_lower(&semio_framework_hash::Sha256::digest(bytes)[..8]))
}
"""
NEW = """pub fn content_id(prefix: &str, bytes: &[u8]) -> String {
    let id = format!("{prefix}-{}", semio_framework_hash::hex_lower(&semio_framework_hash::Sha256::digest(bytes)[..8]));
    #[cfg(not(target_arch = "wasm32"))]
    t14_f9_record(prefix, bytes, &id);
    id
}

#[cfg(not(target_arch = "wasm32"))]
fn t14_f9_record(prefix: &str, bytes: &[u8], id: &str) {
    use std::hash::{Hash, Hasher};
    use std::io::Write;
    let Ok(path) = std::env::var("T14_F9_RECORD") else { return };
    let finish = |feed: &dyn Fn(&mut std::collections::hash_map::DefaultHasher)| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        feed(&mut hasher);
        format!("{prefix}-{:016x}", hasher.finish())
    };
    let mut candidates = vec![finish(&|hasher| bytes.hash(hasher))];
    if let Ok(text) = std::str::from_utf8(bytes) {
        candidates.push(finish(&|hasher| text.hash(hasher)));
        candidates.push(finish(&|hasher| text.split('\\u{1f}').for_each(|part| part.hash(hasher))));
    }
    if let Some(at) = bytes.iter().position(|byte| *byte == 0x1f) {
        if let Ok(head) = std::str::from_utf8(&bytes[..at]) {
            let tail = &bytes[at + 1..];
            candidates.push(finish(&|hasher| {
                head.hash(hasher);
                tail.hash(hasher);
            }));
        }
    }
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(bytes) {
        if let Ok(sorted) = serde_json::to_string(&json) {
            candidates.push(finish(&|hasher| sorted.hash(hasher)));
        }
    }
    let line = format!("{prefix}\\t{id}\\t{}\\n", candidates.join(","));
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(line.as_bytes());
    }
}
"""
P3_OLD = """fn brep_child_handle_for_text(slug: &str, content: &str) -> store::ArtifactChild<SemioBrepSnapshot> {
    let child_id = store::content_id(&format!("{slug}-brep"), content.as_bytes());
"""
P3_NEW = """#[cfg(not(target_arch = "wasm32"))]
static T14_F9_NESTED: std::sync::Mutex<Vec<(String, String)>> = std::sync::Mutex::new(Vec::new());

#[cfg(not(target_arch = "wasm32"))]
fn t14_f9_record_exact(prefix: &str, id: &str, old: &str) {
    use std::io::Write;
    let Ok(path) = std::env::var("T14_F9_RECORD") else { return };
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(format!("{prefix}\\t{id}\\t{old}\\n").as_bytes());
    }
}

fn brep_child_handle_for_text(slug: &str, content: &str) -> store::ArtifactChild<SemioBrepSnapshot> {
    let child_id = store::content_id(&format!("{slug}-brep"), content.as_bytes());
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        content.hash(&mut hasher);
        let old = format!("{slug}-brep-{:016x}", hasher.finish());
        t14_f9_record_exact(&format!("{slug}-brep"), &child_id, &old);
        if let Ok(mut nested) = T14_F9_NESTED.lock() {
            nested.push((child_id.clone(), old));
        }
    }
"""
P3_FLOW_OLD = """    let child_id = store::content_id("steps-flow", dsl::json::to_json_string(content).as_bytes());
"""
P3_FLOW_NEW = P3_FLOW_OLD + """    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::hash::{Hash, Hasher};
        let mut text = serde_json::to_string(&semio_framework_os_kernel::ToValue::to_value(content)).unwrap_or_default();
        if let Ok(nested) = T14_F9_NESTED.lock() {
            for (new, old) in nested.iter() {
                text = text.replace(new.as_str(), old.as_str());
            }
        }
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        text.hash(&mut hasher);
        t14_f9_record_exact("steps-flow", &child_id, &format!("steps-flow-{:016x}", hasher.finish()));
    }
"""
if __name__ != "__main__":
    raise ImportError("record-instrument.py exposes OLD/NEW to runpy only")
ROOT = Path(sys.argv[sys.argv.index("--root") + 1])
if ROOT.resolve() == Path("/Users/ueli/Documents/semio").resolve():
    raise SystemExit("record-instrument.py is overlay-only")
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
text = STORE.read_text(encoding="utf-8")
PROCESS3D = ROOT / "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs"
p3 = PROCESS3D.read_text(encoding="utf-8")
if "T14_F9_NESTED" in p3:
    print("process3d already instrumented")
elif p3.count(P3_OLD) != 1 or p3.count(P3_FLOW_OLD) != 1:
    raise SystemExit("process3d brep/steps-flow anchors missing (apply content-id.py first)")
else:
    PROCESS3D.write_text(p3.replace(P3_OLD, P3_NEW).replace(P3_FLOW_OLD, P3_FLOW_NEW), encoding="utf-8")
    print("instrumented", PROCESS3D)
if "fn t14_f9_record" in text:
    print("already instrumented")
elif text.count(OLD) != 1:
    raise SystemExit(f"content_id anchor found {text.count(OLD)} times (apply content-id.py first)")
else:
    STORE.write_text(text.replace(OLD, NEW), encoding="utf-8")
    print("instrumented", STORE)
