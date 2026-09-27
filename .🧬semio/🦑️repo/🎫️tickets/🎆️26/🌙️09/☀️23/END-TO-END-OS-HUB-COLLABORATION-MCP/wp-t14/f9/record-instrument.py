#!/usr/bin/env python3
"""🧭️ OVERLAY-ONLY instrumentation for F9 carrier regeneration (never lands in the tree): after `content-id.py` applied
`store::content_id` in an overlay, this makes `content_id` append `prefix<TAB>new id<TAB>old candidates` to the file named
by `T14_F9_RECORD` (native only). The old candidates are every way the 29 former sites fed the same bytes into
`DefaultHasher` (`str`, `[u8]`, U+001F-joined `str` fields, a `str` head + `[u8]` tail, and the bytes re-serialized by `serde_json` — process3d's
`steps-flow` hashed `serde_json::to_string(&to_value(content))`, whose map keys serde_json sorts), so one run of an owner's tests
yields the exact old→new id map for every id the production code re-mints. Refuses to run on the live tree.
usage: record-instrument.py --root <overlay>"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1])
if ROOT.resolve() == Path("/Users/ueli/Documents/semio").resolve():
    raise SystemExit("record-instrument.py is overlay-only")
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
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
text = STORE.read_text(encoding="utf-8")
if "fn t14_f9_record" in text:
    print("already instrumented")
elif text.count(OLD) != 1:
    raise SystemExit(f"content_id anchor found {text.count(OLD)} times (apply content-id.py first)")
else:
    STORE.write_text(text.replace(OLD, NEW), encoding="utf-8")
    print("instrumented", STORE)
